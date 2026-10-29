//! Executor lifecycle: local authority and durable effects precede network receipts.
use super::{
    checkpoint::{self, Plan, RepoPaths, Trigger},
    config::Bootstrap,
    journal::{Binding, Journal},
    process::{Finish, Processes},
    transport::{
        backoff, channel_backoff, maintain_lease, transient, Channel, ChannelEnd, ChannelEvent,
        Delivery, HttpIdentity, Opened, Submitted, Transport,
    },
};
use anyhow::{bail, Context, Result};
use futures_util::{stream::FuturesUnordered, StreamExt};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sikaru_sdk::api::*;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{sync::watch, time::Instant};

#[derive(Default)]
pub struct RunOptions {
    pub interactive: bool,
    pub approval_wait: Duration,
    pub timeout: Option<Duration>,
    pub stop: Option<watch::Receiver<bool>>,
    pub admission: Option<watch::Receiver<bool>>,
    /// Hidden `--no-workspace-checkpoints`: no private staging or content uploads.
    pub skip_checkpoints: bool,
    /// Controller state inside the workspace that checkpoints leave out.
    pub private_dirs: Vec<PathBuf>,
}
pub async fn serve_with_options(
    b: Bootstrap,
    base_url: String,
    http: HttpIdentity,
    options: RunOptions,
) -> Result<Value> {
    let binding = Binding::from_bootstrap(&b)?;
    let instance = format!("{:032x}", rand::random::<u128>());
    let transport = Transport::new(&b, base_url, http, binding.clone())?;
    let startup = Instant::now() + Duration::from_secs(180);
    let current = transport.status(startup).await?;
    transport.validate(&current)?;
    let mut journal = Journal::open_verified(&b.state_dir, binding, instance, true)?;
    let mut processes = Processes::new(&journal, Duration::from_secs(b.command_timeout_seconds));
    // Local and before readiness: no lease exists while the repository opens.
    let repository = checkpoint_repository(&b, &options).await;
    let preparation = prepare(&transport, &mut journal, &processes, startup).await;
    let (outcome, cleanup) = match preparation {
        Ok(deadline) => {
            progress(
                options.interactive,
                "Workspace ready.",
                json!({"event":"executor_ready","attachment_id":b.attachment_id,"session_id":b.session_id}),
            );
            run(
                transport.clone(),
                &mut journal,
                &mut processes,
                deadline,
                options,
                repository,
            )
            .await
        }
        Err(_) => (
            Err(anyhow::anyhow!(
                "recovery_required: executor could not establish original authority"
            )),
            processes.cleanup(&mut journal),
        ),
    };
    finish(outcome, cleanup, &transport, &mut journal).await
}
async fn checkpoint_repository(b: &Bootstrap, options: &RunOptions) -> Option<RepoPaths> {
    if options.skip_checkpoints {
        progress(
            options.interactive,
            "Workspace checkpoints are off for this run.",
            json!({"event":"checkpoint_skipped","reason":"disabled"}),
        );
        return None;
    }
    let paths = RepoPaths::new(&b.workspace, &b.state_dir, &options.private_dirs);
    match checkpoint::prepare(&paths).await {
        Ok(()) => Some(paths),
        Err(_) => {
            progress(
                options.interactive,
                "Workspace checkpoints are unavailable for this run.",
                json!({"event":"checkpoint_failed","trigger":null,"reason":"repository_unavailable"}),
            );
            None
        }
    }
}
async fn prepare(
    transport: &Transport,
    journal: &mut Journal,
    processes: &Processes,
    deadline: Instant,
) -> Result<Instant> {
    let a = transport.status(deadline).await?;
    transport.validate(&a)?;
    transport.connect(&journal.instance, deadline).await?;
    reconcile(transport, journal, processes, deadline).await?;
    journal.verify()?;
    transport.ready(&journal.instance, deadline).await
}
async fn reconcile(
    transport: &Transport,
    journal: &mut Journal,
    processes: &Processes,
    deadline: Instant,
) -> Result<()> {
    let page = transport.poll(deadline).await?;
    transport.validate(&page.attachment)?;
    let uncertain = uncertain_operations(&page, journal)?;
    replay_receipts(transport, journal, &page, deadline).await?;
    let observations = page
        .live_handles
        .iter()
        .map(|h| processes.observation(&h.handle_id))
        .collect::<Vec<_>>();
    let lost = observations
        .iter()
        .any(|o| o.status == ProcessObservationStatus::Lost);
    let body = ReconcileInput {
        executor_instance_id: journal.instance.clone(),
        journal_id: journal.binding.journal_id.clone(),
        workspace_provenance: journal.binding.workspace_provenance.clone(),
        receipts: Some(vec![]),
        processes: Some(observations),
        uncertain_operation_ids: Some(uncertain.clone()),
    };
    let response = transport.reconcile(&body, deadline).await?;
    transport.validate(&response.attachment)?;
    if lost || !uncertain.is_empty() {
        bail!("recovery_required: uncertain effects or lost process ownership");
    }
    Ok(())
}
fn uncertain_operations(page: &WorkPage, journal: &Journal) -> Result<Vec<String>> {
    let mut uncertain = Vec::new();
    for issued in &page.issued_operations {
        let key = format!("{}/{}", issued.run_id, issued.tool_call_id);
        match journal.entries.get(&key).and_then(|e| e.receipt.as_ref()) {
            None => uncertain.push(issued.tool_call_id.clone()),
            Some(receipt) => {
                if receipt["request_digest"] != issued.request_digest {
                    bail!("issued operation identity changed");
                }
            }
        }
    }
    Ok(uncertain)
}
/// Replay unacknowledged receipts, and every receipt for an operation the service issued: one a
/// warm caller acknowledged but never consumed has since parked and needs its ledger result.
async fn replay_receipts(
    transport: &Transport,
    journal: &mut Journal,
    page: &WorkPage,
    deadline: Instant,
) -> Result<()> {
    let issued = page
        .issued_operations
        .iter()
        .map(|op| format!("{}/{}", op.run_id, op.tool_call_id))
        .collect::<std::collections::HashSet<_>>();
    let pending = journal
        .entries
        .iter()
        .filter(|(k, e)| !e.acknowledged || issued.contains(*k))
        .filter_map(|(k, e)| e.receipt.clone().map(|r| (k.clone(), r)))
        .collect::<Vec<_>>();
    for (key, value) in pending {
        let receipt: ReceiptInput = serde_json::from_value(value)?;
        transport.submit(&receipt, deadline).await?;
        journal.ack(&key)?;
    }
    Ok(())
}
/// Serve the run, then capture the workspace while the lease is still renewed. Returns the
/// run's outcome and the local process cleanup, which precedes the capture.
async fn run(
    transport: Arc<Transport>,
    journal: &mut Journal,
    processes: &mut Processes,
    deadline: Instant,
    mut options: RunOptions,
    repository: Option<RepoPaths>,
) -> (Result<Value>, Result<()>) {
    let (lease, receiver) = watch::channel(deadline);
    let heartbeat = maintain_lease(transport.clone(), lease);
    tokio::pin!(heartbeat);
    let completed = Mutex::new(None::<String>);
    let admission = options.admission.take();
    let approval_wait = options.approval_wait;
    let interactive = options.interactive;
    let mut lease_held = true;
    let outcome = tokio::select! {
        biased;
        _=termination_signal()=>Ok(json!({"status":"cancelled","reason":"signal","execution":null})),
        _=stop_requested(&mut options.stop)=>Ok(json!({"status":"cancelled","reason":"controller_stopped","execution":null})),
        _=run_deadline(options.timeout)=>Ok(json!({"status":"cancelled","reason":"deadline","execution":null})),
        result=&mut heartbeat=>{
            lease_held = false;
            match result { Err(error) => Err(error), Ok(()) => Err(anyhow::anyhow!("lease_expired")) }
        },
        result=async {await_admission(admission).await?;work_loop(&transport,journal,processes,receiver.clone(),approval_wait,&completed).await}=>result,
    };
    // Owned processes stop before the workspace is captured.
    let cleanup = processes.cleanup(journal);
    let Some(repo) = repository else {
        return (outcome, cleanup);
    };
    let plan = Plan {
        trigger: Trigger::from_outcome(&outcome, journal.has_uncertain_effects()),
        run_id: completed.into_inner().unwrap_or_else(|p| p.into_inner()),
        repo,
        interactive,
    };
    let status = outcome.as_ref().map_or("recovery_required", |v| {
        v["status"].as_str().unwrap_or("failed")
    });
    progress(
        interactive,
        "Run finished; saving the workspace…",
        json!({"event":"run_finished","status":status,"run_id":plan.run_id}),
    );
    let local_clean = cleanup.is_ok();
    let initially_held = lease_held;
    // Reconciliation and capture share the heartbeat polling loop.
    let capture = async {
        if plan.trigger == Trigger::Turn && initially_held && local_clean {
            let deadline = *receiver.borrow();
            let _ = reconcile(&transport, journal, processes, deadline).await;
        }
        checkpoint::checkpoint(&transport, &plan, async |deadline| {
            if !local_clean {
                bail!("cleanup_unconfirmed");
            }
            reconcile(&transport, journal, processes, deadline).await
        })
        .await;
    };
    tokio::pin!(capture);
    loop {
        tokio::select! {
            _ = &mut capture => break,
            _ = &mut heartbeat, if lease_held => lease_held = false,
        }
    }
    (outcome, cleanup)
}
async fn work_loop(
    transport: &Transport,
    journal: &mut Journal,
    processes: &mut Processes,
    lease: watch::Receiver<Instant>,
    approval_wait: Duration,
    completed: &Mutex<Option<String>>,
) -> Result<Value> {
    let mut work = Work {
        transport,
        journal,
        processes,
        lease,
        approval_wait,
        approval_deadline: None,
        completed,
        inflight: FuturesUnordered::new(),
        running: HashMap::new(),
        queued: Vec::new(),
        refusals: HashMap::new(),
    };
    let mut link = ChannelLink::default();
    loop {
        let Some(page) = work.poll().await? else {
            continue;
        };
        let channel = link.admits(&page);
        match work.serve_page(page, None).await? {
            Served::Done(result) => return Ok(result),
            _ if channel => work.channel_session(&mut link).await?,
            Served::Wait(delay, approval) => work.idle(delay, approval).await?,
            Served::Busy => {}
        }
    }
}
/// Operations running at once; more wait in hand until one finishes.
const MAX_IN_FLIGHT: usize = 4;
/// Consecutive refusals of one operation's receipt, while the service keeps issuing it, that
/// end the run.
const MAX_REFUSALS: u32 = 3;
/// A running operation's wait; `None` when the lease expired first.
type Running =
    std::pin::Pin<Box<dyn std::future::Future<Output = (OperationView, Option<Finish>)> + Send>>;
/// The executor's live state; both transports serve pages through it. Only this state touches
/// the journal and processes, so operation effects and receipts apply one at a time while the
/// operations' waits run concurrently.
struct Work<'a> {
    transport: &'a Transport,
    journal: &'a mut Journal,
    processes: &'a mut Processes,
    lease: watch::Receiver<Instant>,
    approval_wait: Duration,
    approval_deadline: Option<Instant>,
    completed: &'a Mutex<Option<String>>,
    inflight: FuturesUnordered<Running>,
    /// Keys of running operations, and whether a repeat delivery joined them.
    running: HashMap<String, bool>,
    queued: Vec<OperationView>,
    refusals: HashMap<String, u32>,
}
enum Served {
    Done(Value),
    Wait(Duration, Option<Instant>),
    Busy,
}
enum Wake {
    Channel(ChannelEvent),
    Finished((OperationView, Option<Finish>)),
    Tick,
}
impl Work<'_> {
    async fn serve_page(
        &mut self,
        page: WorkPage,
        channel: Option<&mut Channel>,
    ) -> Result<Served> {
        self.transport.validate(&page.attachment)?;
        if let Some(run) = page.execution.as_ref().filter(|run| run.terminal) {
            *self.completed.lock().unwrap_or_else(|p| p.into_inner()) = Some(run.run_id.clone());
        }

        if let Some(result) =
            completion_after_wait(&page, self.approval_wait, &mut self.approval_deadline)
        {
            self.drain().await?;
            return Ok(Served::Done(result));
        }
        self.dispatch_page(page, channel).await
    }
    async fn dispatch_page(
        &mut self,
        page: WorkPage,
        mut channel: Option<&mut Channel>,
    ) -> Result<Served> {
        let delay = Duration::from_secs(page.poll_after_seconds.unwrap_or(1).clamp(1, 5) as u64);
        if page.operations.is_empty() {
            let waiting_approval = page
                .execution
                .as_ref()
                .is_some_and(|run| run.approval_required);
            let approval = self.approval_deadline.filter(|_| waiting_approval);
            return Ok(Served::Wait(delay, approval));
        }
        // Every operation in hand runs: it was issued to this executor.
        let (mut answered, mut refused) = (false, false);
        for op in page.operations {
            match self.dispatch(op, channel.as_deref_mut()).await? {
                Some(Submitted::Recorded) => answered = true,
                Some(Submitted::Refused) => refused = true,
                None => {}
            }
        }
        // Only an answered page moves the turn on at once. Running operations wake the idle
        // wait when they finish, and the service may keep issuing an operation whose receipt it
        // refuses: poll at the idle pace rather than spin on either.
        Ok(match answered && !refused {
            true => Served::Busy,
            false => Served::Wait(delay, None),
        })
    }
    /// Start an operation, join it while it runs, or answer it from the journal. `None` unless
    /// a receipt was delivered.
    async fn dispatch(
        &mut self,
        op: OperationView,
        channel: Option<&mut Channel>,
    ) -> Result<Option<Submitted>> {
        ensure_lease(&self.lease)?;
        validate_operation(&op, &self.journal.binding)?;
        let key = operation_key(&op);
        let Some(op) = self.admit(op, &key) else {
            return Ok(None);
        };
        let Some(receipt) = self.journal.intent(&key, serde_json::to_value(&op)?)? else {
            self.start(op, key)?;
            return Ok(None);
        };
        let receipt = serde_json::from_value(receipt)?;
        self.deliver(&key, &receipt, channel).await.map(Some)
    }
    /// The operation, when it may start now. A repeat of a running operation joins it, and one
    /// beyond the bound waits in hand for a free slot.
    fn admit(&mut self, op: OperationView, key: &str) -> Option<OperationView> {
        if let Some(joined) = self.running.get_mut(key) {
            *joined = true;
            return None;
        }
        if self.running.len() < MAX_IN_FLIGHT {
            return Some(op);
        }
        if !self
            .queued
            .iter()
            .any(|queued| operation_key(queued) == key)
        {
            self.queued.push(op);
        }
        None
    }
    fn start(&mut self, op: OperationView, key: String) -> Result<()> {
        let method = serde_json::to_value(&op.method)?;
        let method = method.as_str().context("invalid method")?;
        let pending = self
            .processes
            .begin(method, &op.arguments, self.journal, Some(&key));
        let lease = self.lease.clone();
        self.running.insert(key, false);
        self.inflight.push(Box::pin(async move {
            let finish = tokio::select! {finish=pending=>Some(finish),_=lease_expiry(lease)=>None};
            (op, finish)
        }));
        Ok(())
    }
    /// Record a finished operation's receipt and deliver it, once more if a repeat joined it,
    /// then start operations waiting for a free slot.
    async fn complete(
        &mut self,
        (op, finish): (OperationView, Option<Finish>),
        mut channel: Option<&mut Channel>,
    ) -> Result<Submitted> {
        let key = operation_key(&op);
        let receipt = self.record(&op, &key, finish)?;
        let joined = self.running.remove(&key).unwrap_or(false);
        let mut submitted = self.deliver(&key, &receipt, channel.as_deref_mut()).await?;
        // A refused receipt is refused again: only a recorded one answers the joined repeat too.
        if joined && matches!(submitted, Submitted::Recorded) {
            submitted = self.deliver(&key, &receipt, channel.as_deref_mut()).await?;
        }
        while self.running.len() < MAX_IN_FLIGHT && !self.queued.is_empty() {
            let op = self.queued.remove(0);
            self.dispatch(op, channel.as_deref_mut()).await?;
        }
        Ok(submitted)
    }
    fn record(
        &mut self,
        op: &OperationView,
        key: &str,
        finish: Option<Finish>,
    ) -> Result<ReceiptInput> {
        let Some(finish) = finish else {
            bail!("lease_expired");
        };
        let result = finish(self.processes, self.journal);
        if result.is_err() && self.processes.owns_operation(key) {
            bail!("recovery_required: process observation failed after spawn");
        }
        let receipt = make_receipt(op, result)?;
        self.journal.receipt(key, serde_json::to_value(&receipt)?)?;
        Ok(receipt)
    }
    async fn deliver(
        &mut self,
        key: &str,
        receipt: &ReceiptInput,
        channel: Option<&mut Channel>,
    ) -> Result<Submitted> {
        let accepted = match channel {
            Some(channel) => {
                let deadline = *self.lease.borrow();
                channel.deliver(receipt, deadline).await == Delivery::Accepted
            }
            None => false,
        };
        let submitted = match accepted {
            true => self.journal.ack(key).map(|_| Submitted::Recorded)?,
            false => {
                deliver_receipt(self.transport, self.journal, &self.lease, key, receipt).await?
            }
        };
        self.settle(key, &submitted)?;
        Ok(submitted)
    }
    /// A receipt the service keeps refusing while it keeps issuing the operation never clears.
    fn settle(&mut self, key: &str, submitted: &Submitted) -> Result<()> {
        if matches!(submitted, Submitted::Recorded) {
            self.refusals.remove(key);
            return Ok(());
        }
        let refusals = self.refusals.entry(key.to_owned()).or_default();
        *refusals += 1;
        if *refusals >= MAX_REFUSALS {
            bail!("receipt_delivery_failed: immutable receipt retained");
        }
        Ok(())
    }
    /// Finish every running operation, journaling and delivering its receipt.
    async fn drain(&mut self) -> Result<()> {
        while let Some(finished) = self.inflight.next().await {
            self.complete(finished, None).await?;
        }
        Ok(())
    }
    /// Wait out the idle delay. A running operation's recorded receipt ends it early: the turn
    /// may have moved on.
    async fn idle(&mut self, delay: Duration, approval_deadline: Option<Instant>) -> Result<()> {
        let wake = Instant::now() + delay;
        let wake = approval_deadline.map_or(wake, |deadline| deadline.min(wake));
        loop {
            let finished = tokio::select! {
                _ = tokio::time::sleep_until(wake) => return Ok(()),
                Some(finished) = self.inflight.next(), if !self.inflight.is_empty() => finished,
            };
            if matches!(self.complete(finished, None).await?, Submitted::Recorded) {
                return Ok(());
            }
        }
    }
    /// The next work page, or `None` once a failed poll has re-established authority.
    async fn poll(&mut self) -> Result<Option<WorkPage>> {
        ensure_lease(&self.lease)?;
        self.processes.maintenance(self.journal)?;
        let deadline = *self.lease.borrow();
        match self.transport.poll(deadline).await {
            Ok(page) => Ok(Some(page)),
            Err(_) => self.recover().await.map(|_| None),
        }
    }
    /// Reconnect after a lost connection, once running operations have their receipts.
    /// Transient failures retry with backoff until the lease expires; any other failure ends
    /// the run.
    async fn recover(&mut self) -> Result<()> {
        self.drain().await?;
        let mut attempt = 0;
        loop {
            match reconnect(self.transport, self.journal, self.processes, &self.lease).await {
                Err(error) if transient(&error) => pause(&self.lease, &mut attempt).await?,
                outcome => return outcome,
            }
        }
    }
    /// Serve running-turn pages pushed over the channel; any other page returns to polling.
    async fn channel_session(&mut self, link: &mut ChannelLink) -> Result<()> {
        let (end, connected) = match self.transport.open_channel().await {
            Opened::Channel(mut channel) => {
                let end = self.serve_channel(&mut channel, link).await;
                channel.close().await;
                (end?, true)
            }
            Opened::Ended(ChannelEnd::Poll) => {
                // The handshake selects polling although the page did not: poll at the idle pace.
                return self.idle(Duration::from_secs(1), None).await;
            }
            Opened::Ended(end) => (end, false),
        };
        if connected && end == ChannelEnd::Lost {
            // As after any lost connection: reconcile and replay receipts before reconnecting.
            self.recover().await?;
        }
        if end != ChannelEnd::Poll {
            link.failed();
        }
        Ok(())
    }
    async fn serve_channel(
        &mut self,
        channel: &mut Channel,
        link: &mut ChannelLink,
    ) -> Result<ChannelEnd> {
        let mut tick = tokio::time::interval(Duration::from_secs(1));
        loop {
            let page = match self.next_wake(channel, &mut tick).await? {
                Wake::Channel(ChannelEvent::Page(page)) => page,
                Wake::Channel(ChannelEvent::Ended(end)) => return Ok(end),
                Wake::Finished(finished) => {
                    self.complete(finished, Some(channel)).await?;
                    continue;
                }
                _ => continue,
            };
            if let Some(end) = self.serve_pushed(*page, channel, link).await? {
                return Ok(end);
            }
        }
    }
    /// The next channel event or finished operation; the tick re-checks the lease and processes.
    async fn next_wake(
        &mut self,
        channel: &mut Channel,
        tick: &mut tokio::time::Interval,
    ) -> Result<Wake> {
        ensure_lease(&self.lease)?;
        self.processes.maintenance(self.journal)?;
        Ok(tokio::select! {
            event = channel.next() => Wake::Channel(event),
            Some(finished) = self.inflight.next(), if !self.inflight.is_empty() => Wake::Finished(finished),
            _ = tick.tick() => Wake::Tick,
        })
    }
    /// Serve one pushed page; how the channel ended, if serving it ended the session.
    async fn serve_pushed(
        &mut self,
        page: WorkPage,
        channel: &mut Channel,
        link: &mut ChannelLink,
    ) -> Result<Option<ChannelEnd>> {
        if !channel_page(&page) {
            return Ok(Some(ChannelEnd::Poll));
        }
        self.transport.validate(&page.attachment)?;
        link.progressed();
        self.dispatch_page(page, Some(channel)).await?;
        Ok(channel.ended())
    }
}
fn operation_key(op: &OperationView) -> String {
    format!("{}/{}", op.run_id, op.tool_call_id)
}
/// A running turn with nothing but operations: approvals and
/// terminal or stopping states are served by the HTTP route.
fn channel_page(page: &WorkPage) -> bool {
    page.transport == Some(WorkPageTransport::Channel) && completion(page).is_none()
}
/// Channel attempts for this run. After a failure the run keeps polling and retries the
/// channel on a growing backoff; serving a page resets it.
#[derive(Default)]
struct ChannelLink {
    failures: u32,
    retry_at: Option<Instant>,
}
impl ChannelLink {
    fn admits(&self, page: &WorkPage) -> bool {
        channel_page(page) && self.retry_at.is_none_or(|at| Instant::now() >= at)
    }
    fn progressed(&mut self) {
        if self.failures > 0 {
            eprintln!(
                "{}",
                json!({"event":"executor_transport","transport":"channel","reason":"channel_recovered"})
            );
        }
        self.failures = 0;
        self.retry_at = None;
    }
    fn failed(&mut self) {
        self.failures = self.failures.saturating_add(1);
        self.retry_at = Some(Instant::now() + channel_backoff(self.failures));
        if self.failures == 1 {
            eprintln!(
                "{}",
                json!({"event":"executor_transport","transport":"poll","reason":"channel_unavailable"})
            );
        }
    }
}

fn completion(page: &WorkPage) -> Option<Value> {
    match page.attachment.status {
        AttachmentViewStatus::Stopping => {
            return Some(json!({"status":"cancelled","execution":page.execution}))
        }
        AttachmentViewStatus::RecoveryRequired => {
            return Some(json!({"status":"recovery_required","execution":page.execution}))
        }
        _ => {}
    }
    let execution = page.execution.as_ref()?;
    if execution.approval_required {
        return Some(json!({"status":"approval_required","execution":execution}));
    }
    if execution.terminal {
        return Some(json!({"status":execution.status,"execution":execution}));
    }
    None
}
async fn reconnect(
    transport: &Transport,
    journal: &mut Journal,
    processes: &Processes,
    lease: &watch::Receiver<Instant>,
) -> Result<()> {
    ensure_lease(lease)?;
    let deadline = *lease.borrow();
    transport.connect(&journal.instance, deadline).await?;
    let deadline = *lease.borrow();
    reconcile(transport, journal, processes, deadline).await?;
    // Ready renews; keep the older local lease until heartbeat independently renews.
    let deadline = *lease.borrow();
    transport.ready(&journal.instance, deadline).await?;
    Ok(())
}
async fn deliver_receipt(
    transport: &Transport,
    journal: &mut Journal,
    lease: &watch::Receiver<Instant>,
    key: &str,
    receipt: &ReceiptInput,
) -> Result<Submitted> {
    // Transient failures retry until the lease expires; a refusal settles delivery. The journal
    // keeps the receipt either way, so a repeated operation is answered from it.
    let mut attempt = 0;
    loop {
        let deadline = *lease.borrow();
        match transport.submit(receipt, deadline).await {
            Ok(submitted) => return journal.ack(key).map(|_| submitted),
            Err(error) if transient(&error) => pause(lease, &mut attempt).await?,
            Err(_) => bail!("receipt_delivery_failed: immutable receipt retained"),
        }
    }
}
/// Wait out a transient failure before retrying, never past the lease.
async fn pause(lease: &watch::Receiver<Instant>, attempt: &mut u32) -> Result<()> {
    // Copy the deadline out: a borrow held across the sleep would block lease renewal.
    let wake = (Instant::now() + backoff(attempt)).min(*lease.borrow());
    tokio::time::sleep_until(wake).await;
    ensure_lease(lease)
}
fn make_receipt(op: &OperationView, result: Result<Value>) -> Result<ReceiptInput> {
    let (status, payload) = match result {
        Ok(v) => (ReceiptInputStatus::Completed, v),
        Err(_) => (
            ReceiptInputStatus::Failed,
            json!({"error":"native task IO failed"}),
        ),
    };
    Ok(ReceiptInput {
        run_id: op.run_id.clone(),
        tool_call_id: op.tool_call_id.clone(),
        tool_provider_id: op.tool_provider_id.clone(),
        capability_name: Some(ReceiptInputCapabilityName::ComputeExecute),
        request_digest: op.request_digest.clone(),
        idempotency_key: format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&[&op.run_id, &op.tool_call_id])?)
        ),
        status,
        payload: serde_json::from_value(payload)?,
    })
}
fn validate_operation(op: &OperationView, b: &Binding) -> Result<()> {
    if op.owner_epoch != b.owner_epoch || op.workspace_generation != b.workspace_generation {
        bail!("stale operation authority");
    }
    if op.capability_name != OperationViewCapabilityName::ComputeExecute
        || op.request_digest.is_empty()
        || op.run_id.is_empty()
        || op.tool_call_id.is_empty()
    {
        bail!("invalid admitted operation identity");
    }
    Ok(())
}
fn ensure_lease(lease: &watch::Receiver<Instant>) -> Result<()> {
    if Instant::now() >= *lease.borrow() {
        bail!("lease_expired");
    }
    Ok(())
}
async fn lease_expiry(mut lease: watch::Receiver<Instant>) {
    loop {
        let deadline = *lease.borrow_and_update();
        tokio::select! {_=tokio::time::sleep_until(deadline)=>return,result=lease.changed()=>{if result.is_err(){return;}}}
    }
}
async fn finish(
    outcome: Result<Value>,
    cleanup: Result<()>,
    transport: &Transport,
    journal: &mut Journal,
) -> Result<Value> {
    if let Err(error) = &cleanup {
        eprintln!("native process cleanup failed: {error}");
    }
    let local_clean = cleanup.is_ok();
    let mut result = outcome.unwrap_or_else(
        |error| json!({"status":"recovery_required","reason":error.to_string(),"execution":null}),
    );
    let cancellation = result["status"] == "cancelled";
    let cancel_ack = if cancellation {
        Some(transport.stop().await)
    } else {
        None
    };
    if journal.has_uncertain_effects() {
        result["status"] = json!("recovery_required");
        result["reason"] = json!("interrupted_operation_requires_reconciliation");
    }
    let remote_clean = transport.cleanup(local_clean).await;
    if local_clean && remote_clean && !journal.has_uncertain_effects() {
        journal.mark_clean()?;
    }
    Ok(cleanup_outcome(
        result,
        local_clean,
        remote_clean,
        cancel_ack,
    ))
}
fn cleanup_outcome(
    mut result: Value,
    local_clean: bool,
    remote_clean: bool,
    cancel_ack: Option<bool>,
) -> Value {
    result["cleanup"] = json!(if local_clean && remote_clean {
        "confirmed"
    } else {
        "unconfirmed"
    });
    if !local_clean || !remote_clean {
        result["status"] = json!("recovery_required");
    }
    result["cancel_acknowledged"] = json!(cancel_ack);
    result["usage"] = json!({"available":false});
    result
}
pub(crate) async fn termination_signal() {
    let mut term = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("SIGTERM handler");
    tokio::select! {_=tokio::signal::ctrl_c()=>{},_=term.recv()=>{}}
}

fn completion_after_wait(
    page: &WorkPage,
    wait: Duration,
    deadline: &mut Option<Instant>,
) -> Option<Value> {
    let result = completion(page)?;
    if result["status"] != "approval_required" {
        return Some(result);
    }
    let end = deadline.get_or_insert_with(|| Instant::now() + wait);
    (Instant::now() >= *end).then_some(result)
}
async fn stop_requested(stop: &mut Option<watch::Receiver<bool>>) {
    match stop {
        Some(receiver) => {
            while !*receiver.borrow_and_update() {
                if receiver.changed().await.is_err() {
                    return;
                }
            }
        }
        None => std::future::pending::<()>().await,
    }
}
async fn run_deadline(timeout: Option<Duration>) {
    match timeout {
        Some(duration) => tokio::time::sleep(duration).await,
        None => std::future::pending::<()>().await,
    }
}

async fn await_admission(admission: Option<watch::Receiver<bool>>) -> Result<()> {
    if let Some(mut receiver) = admission {
        while !*receiver.borrow_and_update() {
            receiver
                .changed()
                .await
                .context("controller admission channel closed")?;
        }
    }
    Ok(())
}

/// Keep automation events machine-readable while giving terminal users progress.
pub fn progress(interactive: bool, message: &str, event: Value) {
    if interactive {
        eprintln!("{message}");
    } else {
        eprintln!("{event}");
    }
}
