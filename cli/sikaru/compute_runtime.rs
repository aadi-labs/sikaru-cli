//! Executor lifecycle: local authority and durable effects precede network receipts.
use super::{
    checkpoint::{self, Plan, RemoteSlot, RepoPaths, Trigger},
    config::Bootstrap,
    journal::{Binding, Journal},
    process::Processes,
    transport::{
        channel_backoff, maintain_lease, Channel, ChannelEnd, ChannelEvent, Delivery, HttpIdentity,
        Opened, Transport,
    },
};
use anyhow::{bail, Context, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sikaru_sdk::api::*;
use std::{
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
    /// Hidden `--no-workspace-checkpoints`: no private repository, remote or push.
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
    replay_receipts(transport, journal, deadline).await?;
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
async fn replay_receipts(
    transport: &Transport,
    journal: &mut Journal,
    deadline: Instant,
) -> Result<()> {
    let pending = journal
        .entries
        .iter()
        .filter(|(_, e)| !e.acknowledged)
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
    let slot = RemoteSlot::default();
    let checkpoints = repository.is_some();
    let keeper = async {
        if checkpoints {
            checkpoint::keep_remote(&transport, &slot, receiver.clone()).await;
        }
        std::future::pending::<()>().await
    };
    tokio::pin!(keeper);
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
        _=&mut keeper=>unreachable!("the remote keeper never returns"),
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
        checkpoint::checkpoint(&transport, &plan, &slot, async |deadline| {
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
    };
    let mut link = ChannelLink::default();
    loop {
        ensure_lease(&work.lease)?;
        work.processes.maintenance(work.journal)?;
        let deadline = *work.lease.borrow();
        let page = match work.transport.poll(deadline).await {
            Ok(page) => page,
            Err(_) => {
                work.reconnect().await?;
                continue;
            }
        };
        let channel = link.admits(&page);
        match work.serve_page(page, None).await? {
            Served::Done(result) => return Ok(result),
            _ if channel => {
                if let Some(result) = work.channel_session(&mut link).await? {
                    return Ok(result);
                }
            }
            Served::Wait(delay, approval) => idle_wait(delay, approval).await,
            Served::Busy => {}
        }
    }
}
/// The executor's live state; both transports serve pages through it.
struct Work<'a> {
    transport: &'a Transport,
    journal: &'a mut Journal,
    processes: &'a mut Processes,
    lease: watch::Receiver<Instant>,
    approval_wait: Duration,
    approval_deadline: Option<Instant>,
    completed: &'a Mutex<Option<String>>,
}
enum Served {
    Done(Value),
    Wait(Duration, Option<Instant>),
    Busy,
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
            return Ok(Served::Done(result));
        }
        self.dispatch_page(page, channel).await
    }
    async fn dispatch_page(
        &mut self,
        page: WorkPage,
        mut channel: Option<&mut Channel>,
    ) -> Result<Served> {
        if page.operations.is_empty() {
            let delay =
                Duration::from_secs(page.poll_after_seconds.unwrap_or(1).clamp(1, 5) as u64);
            let waiting_approval = page
                .execution
                .as_ref()
                .is_some_and(|run| run.approval_required);
            let approval = self.approval_deadline.filter(|_| waiting_approval);
            return Ok(Served::Wait(delay, approval));
        }
        // Every operation in hand runs: it was issued to this executor.
        for op in page.operations {
            self.dispatch(op, channel.as_deref_mut()).await?;
        }
        Ok(Served::Busy)
    }
    async fn dispatch(&mut self, op: OperationView, channel: Option<&mut Channel>) -> Result<()> {
        ensure_lease(&self.lease)?;
        validate_operation(&op, &self.journal.binding)?;
        let key = format!("{}/{}", op.run_id, op.tool_call_id);
        let receipt: ReceiptInput = match self.journal.intent(&key, serde_json::to_value(&op)?)? {
            Some(receipt) => serde_json::from_value(receipt)?,
            None => execute_operation(&op, &key, self.journal, self.processes, &self.lease).await?,
        };
        if let Some(channel) = channel {
            let deadline = *self.lease.borrow();
            if channel.deliver(&receipt, deadline).await == Delivery::Accepted {
                return self.journal.ack(&key);
            }
        }
        deliver_receipt(self.transport, self.journal, &self.lease, &key, &receipt).await
    }
    async fn reconnect(&mut self) -> Result<()> {
        reconnect(self.transport, self.journal, self.processes, &self.lease).await
    }
    /// Serve running-turn pages pushed over the channel; any other page returns to polling.
    async fn channel_session(&mut self, link: &mut ChannelLink) -> Result<Option<Value>> {
        let end = match self.transport.open_channel().await {
            Opened::Channel(mut channel) => {
                let end = self.serve_channel(&mut channel, link).await;
                channel.close().await;
                end?
            }
            Opened::Ended(ChannelEnd::Lost) => return link.retry_later().await.map(|_| None),
            Opened::Ended(ChannelEnd::Poll) => {
                // The handshake selects polling although the page did not: poll at the idle pace.
                idle_wait(Duration::from_secs(1), None).await;
                return Ok(None);
            }
            Opened::Ended(end) => end,
        };
        if end == ChannelEnd::Lost {
            // As after any lost connection: reconcile and replay receipts before reconnecting.
            self.reconnect().await?;
        }
        if end != ChannelEnd::Poll {
            link.retry_later().await?;
        }
        Ok(None)
    }
    async fn serve_channel(
        &mut self,
        channel: &mut Channel,
        link: &mut ChannelLink,
    ) -> Result<ChannelEnd> {
        let mut tick = tokio::time::interval(Duration::from_secs(1));
        loop {
            ensure_lease(&self.lease)?;
            self.processes.maintenance(self.journal)?;
            let event = tokio::select! {
                event = channel.next() => event,
                _ = tick.tick() => continue,
            };
            let page = match event {
                ChannelEvent::Page(page) => page,
                ChannelEvent::Ended(end) => return Ok(end),
                _ => continue,
            };
            if !channel_page(&page) {
                return Ok(ChannelEnd::Poll);
            }
            self.transport.validate(&page.attachment)?;
            link.progressed();
            self.dispatch_page(*page, Some(channel)).await?;
            if let Some(end) = channel.ended() {
                return Ok(end);
            }
        }
    }
}
/// A running turn with nothing but operations: approvals and
/// terminal or stopping states are served by the HTTP route.
fn channel_page(page: &WorkPage) -> bool {
    page.transport == Some(WorkPageTransport::Channel) && completion(page).is_none()
}
const CHANNEL_ATTEMPTS: u32 = 3;
/// Channel attempts for this run. After `CHANNEL_ATTEMPTS` consecutive failures
/// the run stays on polling, which needs no change to its binding.
#[derive(Default)]
struct ChannelLink {
    failures: u32,
    exhausted: bool,
}
impl ChannelLink {
    fn admits(&self, page: &WorkPage) -> bool {
        !self.exhausted && channel_page(page)
    }
    fn progressed(&mut self) {
        self.failures = 0;
    }
    async fn retry_later(&mut self) -> Result<()> {
        self.failures += 1;
        if self.failures >= CHANNEL_ATTEMPTS {
            self.exhausted = true;
            eprintln!(
                "{}",
                json!({"event":"executor_transport","transport":"poll","reason":"channel_unavailable"})
            );
            return Ok(());
        }
        tokio::time::sleep(channel_backoff(self.failures)).await;
        Ok(())
    }
}

async fn idle_wait(delay: Duration, approval_deadline: Option<Instant>) {
    let wake = Instant::now() + delay;
    tokio::time::sleep_until(approval_deadline.map_or(wake, |deadline| deadline.min(wake))).await;
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
async fn execute_operation(
    op: &OperationView,
    key: &str,
    journal: &mut Journal,
    processes: &mut Processes,
    lease: &watch::Receiver<Instant>,
) -> Result<ReceiptInput> {
    let method = serde_json::to_value(&op.method)?;
    let effect = processes.execute_owned(
        method.as_str().context("invalid method")?,
        &op.arguments,
        journal,
        Some(key),
    );
    let result = tokio::select! {result=effect=>result,_=lease_expiry(lease.clone())=>bail!("lease_expired")};
    if result.is_err() && processes.owns_operation(key) {
        bail!("recovery_required: process observation failed after spawn");
    }
    let receipt = make_receipt(op, result)?;
    journal.receipt(key, serde_json::to_value(&receipt)?)?;
    Ok(receipt)
}
async fn deliver_receipt(
    transport: &Transport,
    journal: &mut Journal,
    lease: &watch::Receiver<Instant>,
    key: &str,
    receipt: &ReceiptInput,
) -> Result<()> {
    for _ in 0..3 {
        ensure_lease(lease)?;
        let deadline = *lease.borrow();
        if transport.submit(receipt, deadline).await.is_ok() {
            journal.ack(key)?;
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    bail!("receipt_delivery_failed: immutable receipt retained")
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
