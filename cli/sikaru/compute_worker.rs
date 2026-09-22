//! Restricted worker launch authority; launcher owns sandbox lifetime and teardown proof.
use super::{
    launcher::{self, Handle},
    runtime::termination_signal,
    state::{self, call, State},
};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sikaru_sdk::api::*;
use std::{
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{sync::watch, task::JoinSet, time::Instant};
#[derive(Default)]
struct LaunchTasks {
    tasks: JoinSet<Value>,
    identities: std::collections::HashMap<tokio::task::Id, (String, String)>,
}
impl LaunchTasks {
    fn len(&self) -> usize {
        self.tasks.len()
    }
    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
    fn insert(
        &mut self,
        attachment: String,
        launch: String,
        future: impl std::future::Future<Output = Value> + Send + 'static,
    ) {
        let task = self.tasks.spawn(future);
        self.identities.insert(task.id(), (attachment, launch));
    }
    async fn join_next(&mut self) -> Option<Value> {
        let (id, mut result) = match self.tasks.join_next_with_id().await? {
            Ok((id, value)) => (id, value),
            Err(error) => (error.id(), state::failure("worker_task_failed")),
        };
        if let Some((attachment, launch)) = self.identities.remove(&id) {
            result["attachment_id"] = json!(attachment);
            result["launch_id"] = json!(launch);
        }
        Some(result)
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WorkerBootstrap {
    project_id: String,
    environment_id: String,
    token: String,
    state_dir: PathBuf,
}
#[derive(Serialize, Deserialize, Default)]
struct WorkerState {
    project: String,
    environment: String,
    entries: Vec<String>,
}
#[derive(Serialize, Deserialize)]
struct Launch {
    attachment: AttachmentView,
    key: String,
    claim: Option<ClaimView>,
    credential_id: Option<String>,
    launch_intent: bool,
    handle: Option<Handle>,
    done: bool,
    created: f64,
}
pub fn command() -> clap::Command {
    clap::Command::new("worker")
        .about("Launch customer sandboxes using a restricted worker credential")
        .arg(clap::Arg::new("bootstrap").long("bootstrap").required(true))
        .arg(
            clap::Arg::new("launcher")
                .long("launcher")
                .required(true)
                .help("Executable implementing launch/status/teardown JSON v1 over stdin"),
        )
        .arg(
            clap::Arg::new("concurrency")
                .long("concurrency")
                .default_value("1")
                .value_parser(clap::value_parser!(u64).range(1..=64)),
        )
        .arg(
            clap::Arg::new("once")
                .long("once")
                .action(clap::ArgAction::SetTrue)
                .help("Drain one queue page and reconcile owned launches"),
        )
}
pub async fn execute(
    m: &clap::ArgMatches,
    ctx: &fern_cli_sdk::openapi::AppContext,
) -> Result<Value> {
    let b: WorkerBootstrap =
        super::config::read_private_json(m.get_one::<String>("bootstrap").unwrap())?;
    let client = std::sync::Arc::new(
        sikaru_sdk::ApiClientBuilder::new(ctx.effective_base_url())
            .token(b.token.clone())
            .max_retries(0)
            .reqwest_client(ctx.http_config().build_client()?)
            .build()?,
    );
    let mut ledger = open_worker_state(&b)?;
    let launcher = PathBuf::from(m.get_one::<String>("launcher").unwrap()).canonicalize()?;
    let capacity = *m.get_one::<u64>("concurrency").unwrap() as usize;
    let restored = restore(&ledger, &b.state_dir, capacity)?;
    let (stop, receiver) = watch::channel(false);
    let mut tasks = LaunchTasks::default();
    for saved in restored {
        spawn(
            &mut tasks,
            client.clone(),
            launcher.clone(),
            saved,
            receiver.clone(),
            ctx.effective_base_url(),
        );
    }
    let mut results = Vec::new();
    let driving = drive(
        &client,
        &b.environment_id,
        &mut ledger,
        &b.state_dir,
        &launcher,
        &mut tasks,
        receiver,
        capacity,
        ctx.effective_base_url(),
        m.get_flag("once"),
        &mut results,
    );
    let credentials = super::transport::maintain_credentials(
        &client,
        &b.project_id,
        None,
        Instant::now() + Duration::from_secs(30),
        None,
    );
    let outcome = tokio::select! {
        result=driving=>result,
        result=credentials=>result.map(|_| false),
    };
    let _ = stop.send(true);
    while let Some(result) = tasks.join_next().await {
        results.push(result);
    }
    if outcome.is_err() {
        results.push(state::failure("worker_poll_or_renewal_failed"));
    }
    Ok(shutdown_result(outcome, results))
}
fn shutdown_result(outcome: Result<bool>, results: Vec<Value>) -> Value {
    let interrupted = matches!(outcome, Ok(true));
    let mut result = worker_result(results);
    if interrupted {
        if result["status"] == "completed" {
            result["status"] = json!("cancelled");
        }
        result["reason"] = json!("signal");
        result["cancel_acknowledged"] = json!(result["cleanup"] == "confirmed");
    }
    result
}
fn open_worker_state(b: &WorkerBootstrap) -> Result<State<WorkerState>> {
    let initial = if b.state_dir.join("workflow.jsonl").exists() {
        None
    } else {
        Some(WorkerState {
            project: b.project_id.clone(),
            environment: b.environment_id.clone(),
            entries: vec![],
        })
    };
    let ledger = State::open(&b.state_dir, initial)?;
    if ledger.value.project != b.project_id || ledger.value.environment != b.environment_id {
        bail!("worker state identity mismatch");
    }
    Ok(ledger)
}

fn restore(
    ledger: &State<WorkerState>,
    root: &std::path::Path,
    capacity: usize,
) -> Result<Vec<State<Launch>>> {
    let mut active = Vec::new();
    for key in &ledger.value.entries {
        let saved: State<Launch> = State::open(&root.join(key), None)?;
        if !saved.value.done {
            active.push(saved);
        }
    }
    if active.len() > capacity {
        bail!("existing owned launches exceed concurrency; restore with original capacity");
    }
    Ok(active)
}
fn worker_result(results: Vec<Value>) -> Value {
    let uncertain = results
        .iter()
        .any(|r| r["cleanup"] != "confirmed" || r["status"] == "recovery_required");
    let failed = results.iter().any(|r| r["status"] == "failed");
    let status = if uncertain {
        "recovery_required"
    } else if failed {
        "failed"
    } else {
        "completed"
    };
    json!({"status":status,"cleanup":if results.iter().all(|r|r["cleanup"]=="confirmed"){"confirmed"}else{"unconfirmed"},"launches":results,"usage":{"available":false},"cancel_acknowledged":null})
}
#[allow(clippy::too_many_arguments)]
async fn drive(
    c: &std::sync::Arc<ApiClient>,
    environment: &str,
    ledger: &mut State<WorkerState>,
    root: &std::path::Path,
    launcher: &std::path::Path,
    tasks: &mut LaunchTasks,
    receiver: watch::Receiver<bool>,
    capacity: usize,
    base_url: String,
    once_mode: bool,
    results: &mut Vec<Value>,
) -> Result<bool> {
    let mut once = false;
    let mut poll_at = Instant::now();
    let mut failures = 0;
    loop {
        if queue_due(tasks.len(), capacity, once, poll_at) {
            let admission = admit(
                c,
                environment,
                ledger,
                root,
                launcher,
                tasks,
                receiver.clone(),
                capacity,
                base_url.clone(),
            )
            .await;
            let (delay, admitted) = admission_delay(admission, &mut failures)?;
            poll_at = Instant::now() + delay;
            once = once_mode && admitted;
        }
        if once && tasks.is_empty() {
            return Ok(false);
        }
        tokio::select! {
            _=termination_signal()=>return Ok(true),

            result=tasks.join_next(),if !tasks.is_empty()=>results.push(result.unwrap()),
            _=tokio::time::sleep_until(poll_at),if tasks.len() < capacity && !once=>{},
        }
    }
}

fn admission_delay(result: Result<Duration>, failures: &mut u32) -> Result<(Duration, bool)> {
    match result {
        Ok(delay) => {
            *failures = 0;
            Ok((delay, true))
        }
        Err(error) if super::transport::transient(&error) => {
            Ok((super::transport::backoff(failures), false))
        }
        Err(error) => Err(error),
    }
}

fn queue_due(active: usize, capacity: usize, once: bool, poll_at: Instant) -> bool {
    active < capacity && !once && Instant::now() >= poll_at
}

fn spawn(
    tasks: &mut LaunchTasks,
    c: std::sync::Arc<ApiClient>,
    launcher: PathBuf,
    saved: State<Launch>,
    stop: watch::Receiver<bool>,
    base_url: String,
) {
    let attachment = saved.value.attachment.id.clone();
    let launch = saved.value.key.clone();
    eprintln!(
        "{}",
        json!({"event":"launch_observed","attachment_id":attachment,"launch_id":launch})
    );
    tasks.insert(attachment, launch, async move {
        own(&c, &launcher, saved, stop, &base_url)
            .await
            .unwrap_or_else(|_| state::failure("launch_or_teardown_unconfirmed"))
    });
}
#[allow(clippy::too_many_arguments)]
async fn admit(
    c: &std::sync::Arc<ApiClient>,
    environment: &str,
    ledger: &mut State<WorkerState>,
    root: &std::path::Path,
    launcher: &std::path::Path,
    tasks: &mut LaunchTasks,
    stop: watch::Receiver<bool>,
    capacity: usize,
    base_url: String,
) -> Result<Duration> {
    let page = call(c.compute_workers.poll(
        &ledger.value.project,
        environment,
        &ComputeWorkersPollQueryRequest {
            limit: Some((capacity - tasks.len()) as i64),
            wait_seconds: Some(0),
        },
        None,
    ))
    .await?;
    let remaining = capacity - tasks.len();
    for attachment in page.attachments.into_iter().take(remaining) {
        let key = attachment.id.clone();
        if ledger.value.entries.contains(&key) {
            continue;
        }
        if key.contains('/') || key == ".." {
            bail!("invalid attachment identity");
        }
        let path = root.join(&key);
        let initial = if path.join("workflow.jsonl").exists() {
            None
        } else {
            Some(Launch {
                attachment,
                key: state::identity(),
                claim: None,
                credential_id: None,
                launch_intent: false,
                handle: None,
                done: false,
                created: now(),
            })
        };
        let saved = State::open(&path, initial)?;
        ledger.value.entries.push(key);
        ledger.save()?;
        spawn(
            tasks,
            c.clone(),
            launcher.to_owned(),
            saved,
            stop.clone(),
            base_url.clone(),
        );
    }
    Ok(Duration::from_secs(
        page.poll_after_seconds.unwrap_or(1).clamp(1, 5) as u64,
    ))
}
fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}
async fn own(
    c: &ApiClient,
    launcher: &std::path::Path,
    mut s: State<Launch>,
    mut stop: watch::Receiver<bool>,
    base_url: &str,
) -> Result<Value> {
    if !claim(c, &mut s).await? {
        return Ok(
            json!({"status":"not_launched","reason":"claim_rejected","cleanup":"confirmed"}),
        );
    }
    let start_remaining = (180.0 - (now() - s.value.created)).max(0.0).min(180.0);
    let startup = Instant::now() + Duration::from_secs_f64(start_remaining);
    let outcome = if s.value.launch_intent {
        monitor(c, launcher, &mut s, &mut stop, startup).await
    } else {
        match launch(c, launcher, &mut s, startup, base_url).await {
            Ok(()) => monitor(c, launcher, &mut s, &mut stop, startup).await,
            Err(error) => Err(error),
        }
    };
    let result = teardown(c, launcher, &mut s).await;
    let mut value = result.unwrap_or_else(|_| state::failure("sandbox_teardown_unconfirmed"));
    if let Err(error) = outcome {
        let reason = error.to_string();
        value["status"] = json!(if reason == "startup_deadline_expired"
            || reason == "launcher_terminated_before_ready"
        {
            "failed"
        } else {
            "recovery_required"
        });
        value["reason"] = json!(reason);
    }
    value["attachment_id"] = json!(s.value.attachment.id);
    Ok(value)
}
async fn claim(c: &ApiClient, s: &mut State<Launch>) -> Result<bool> {
    if s.value.claim.is_some() {
        return Ok(true);
    }
    let a = &s.value.attachment;
    let response = tokio::time::timeout(
        Duration::from_secs(10),
        c.compute_attachments.claim(
            &a.project_id,
            &a.id,
            &ClaimInput {
                idempotency_key: s.value.key.clone(),
            },
            None,
        ),
    )
    .await;
    match response {
        Ok(Ok(claim)) => {
            s.value.claim = Some(claim);
            s.save()?;
            Ok(true)
        }
        Ok(Err(sikaru_sdk::ApiError::ConflictError { .. })) => {
            s.value.done = true;
            s.save()?;
            Ok(false)
        }
        _ => bail!("claim_response_unknown; original idempotency key retained"),
    }
}

fn input(s: &Launch, operation: &str) -> Value {
    json!({"version":1,"operation":operation,"launch_id":s.key,"project_id":s.attachment.project_id,"environment_id":s.attachment.environment_id,
        "session_id":s.attachment.session_id,"attachment_id":s.attachment.id,"claim":s.claim,"workspace_generation":s.attachment.workspace_generation,
        "journal_id":s.attachment.journal_id,"workspace_provenance":s.attachment.workspace_provenance,"handle":s.handle})
}
async fn launch(
    c: &ApiClient,
    launcher: &std::path::Path,
    s: &mut State<Launch>,
    deadline: Instant,
    base_url: &str,
) -> Result<()> {
    if Instant::now() >= deadline {
        bail!("startup_deadline_expired");
    }
    let a = &s.value.attachment;
    let claim = s.value.claim.as_ref().unwrap();
    let credential = call(c.compute_attachments.issue_credential(
        &a.project_id,
        &a.id,
        &ExecutorCredentialInput {
            owner_id: claim.owner_id.clone(),
            owner_epoch: claim.owner_epoch,
        },
        None,
    ))
    .await?;
    s.value.credential_id = Some(credential.credential_id.clone());
    s.value.launch_intent = true;
    s.save()?; // fsync before the only launch invocation
    let mut payload = input(&s.value, "launch");
    payload["credential"] = json!(credential);
    payload["base_url"] = json!(base_url);
    let response = launcher::invoke(
        launcher,
        payload,
        deadline.saturating_duration_since(Instant::now()),
    )
    .await;
    if let Ok(response) = response {
        record_response(s, response)?;
    }
    Ok(()) // lost ACK is reconciled by deterministic launch identity, never launched again
}
fn record_response(s: &mut State<Launch>, response: launcher::Response) -> Result<()> {
    if let Some(handle) = response.handle {
        if let Some(old) = &s.value.handle {
            if old != &handle {
                bail!("launcher handle changed");
            }
            return s.verify();
        }
        s.value.handle = Some(handle);
        return s.save();
    }
    s.verify()
}
async fn monitor(
    c: &ApiClient,
    launcher: &std::path::Path,
    s: &mut State<Launch>,
    stop: &mut watch::Receiver<bool>,
    startup: Instant,
) -> Result<()> {
    let mut ready = false;
    loop {
        if *stop.borrow() {
            return Ok(());
        }
        let a = &s.value.attachment;
        let remote = call(c.compute_attachments.status(&a.project_id, &a.id, None)).await?;
        if remote.status == AttachmentViewStatus::Ready {
            ready = true;
        }
        startup_valid(&remote, ready, startup)?;
        if terminal(&remote) {
            return Ok(());
        }
        let response =
            launcher::invoke(launcher, input(&s.value, "status"), Duration::from_secs(10)).await?;
        let finished = ["terminated", "not_launched"].contains(&response.status.as_str());
        record_response(s, response)?;
        if finished {
            return require_ready(ready);
        }
        tokio::select! {_=stop.changed()=>return Ok(()),_=tokio::time::sleep(Duration::from_millis(250))=>{}}
    }
}
fn terminal(a: &AttachmentView) -> bool {
    matches!(
        a.status,
        AttachmentViewStatus::Cleaned
            | AttachmentViewStatus::Stopping
            | AttachmentViewStatus::RecoveryRequired
            | AttachmentViewStatus::StartupExpired
            | AttachmentViewStatus::Abandoned
    )
}
async fn teardown(
    c: &ApiClient,
    launcher: &std::path::Path,
    s: &mut State<Launch>,
) -> Result<Value> {
    let response = launcher::invoke(
        launcher,
        input(&s.value, "teardown"),
        Duration::from_secs(30),
    )
    .await?;
    if response.status == "not_launched" && s.value.handle.is_some() {
        bail!("launcher contradicted prior launch evidence");
    }
    let evidence = teardown_evidence(&response)?;
    record_response(s, response)?;
    let a = &s.value.attachment;
    let claim = s.value.claim.as_ref().unwrap();
    let remote = call(c.compute_attachments.teardown(
        &a.project_id,
        &a.id,
        &TeardownInput {
            children_terminated: true,
            evidence,
            owner_epoch: claim.owner_epoch,
            owner_id: claim.owner_id.clone(),
            workspace_generation: a.workspace_generation.clone(),
        },
        None,
    ))
    .await?;
    let confirmed = remote.cleanup_status == AttachmentViewCleanupStatus::Confirmed;
    s.value.done = confirmed;
    s.save()?;
    Ok(
        json!({"status":if remote.status==AttachmentViewStatus::RecoveryRequired{"recovery_required"}else{"completed"},"cleanup":if confirmed{"confirmed"}else{"unconfirmed"},"cancel_acknowledged":null,"usage":{"available":false}}),
    )
}
fn teardown_evidence(response: &launcher::Response) -> Result<String> {
    let evidence = response
        .evidence
        .clone()
        .filter(|v| !v.is_empty())
        .ok_or_else(|| anyhow::anyhow!("missing teardown evidence"))?;
    if !["terminated", "not_launched"].contains(&response.status.as_str()) {
        bail!("sandbox still live or unknown");
    }
    if response.status == "terminated" && response.handle.is_none() {
        bail!("missing sandbox handle proof");
    }
    Ok(evidence)
}

fn require_ready(ready: bool) -> Result<()> {
    if !ready {
        bail!("launcher_terminated_before_ready");
    }
    Ok(())
}

fn startup_valid(remote: &AttachmentView, ready: bool, startup: Instant) -> Result<()> {
    if remote.status == AttachmentViewStatus::StartupExpired
        || (!ready && Instant::now() >= startup)
    {
        bail!("startup_deadline_expired");
    }
    Ok(())
}

#[cfg(test)]
mod task_identity_tests {
    use super::*;
    #[tokio::test]
    async fn cancelled_launch_task_retains_identity_for_reconciliation() {
        let mut launches = LaunchTasks::default();
        launches.insert("attachment".into(), "launch".into(), std::future::pending());
        launches.tasks.abort_all();
        let result = launches.join_next().await.unwrap();
        assert_eq!(result["attachment_id"], "attachment");
        assert_eq!(result["launch_id"], "launch");
        assert_eq!(result["status"], "recovery_required");
        assert_eq!(result["cleanup"], "unconfirmed");
    }
}
