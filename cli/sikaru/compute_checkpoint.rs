//! Content manifest checkpoints taken when a run ends,
//! while the lease is still renewed and before cleanup. Failures are reported on stderr and
//! never change the run's result.
use super::{
    checkpoint_repo::{Cancellation, PrivateRepo, SkipReason, Skipped},
    runtime::progress,
    transport::{backoff, transient, RecordRefusal, Transport, TransportFailure},
};
use anyhow::{bail, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sikaru_sdk::api::*;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
    time::Duration,
};
use tokio::{
    signal::unix::{signal, Signal, SignalKind},
    time::Instant,
};

/// Bound workspace capture and publication after a run.
pub const EXIT_BOUND: Duration = Duration::from_secs(120);
/// A signal-triggered checkpoint, or one a signal interrupts, gets at most this long.
/// Container runtimes commonly allow 10 s between SIGTERM and SIGKILL; this leaves time for
/// cleanup and the result.
pub const SIGNAL_GRACE: Duration = Duration::from_secs(5);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigger {
    Turn,
    Stop,
    Signal,
    LeaseLost,
    Interrupted,
}
impl Trigger {
    /// Why this checkpoint is taken, from the executor result.
    pub fn from_result(result: &Value) -> Self {
        let reason = result["reason"].as_str().unwrap_or_default();
        if reason == "signal" {
            return Self::Signal;
        }
        if reason.contains("lease_expired") {
            return Self::LeaseLost;
        }
        match result["status"].as_str().unwrap_or_default() {
            "recovery_required" => Self::Interrupted,
            _ if reason == "deadline" => Self::Interrupted,
            "cancelled" | "approval_required" => Self::Stop,
            _ => Self::Turn,
        }
    }
    /// The same decision for a run that ended in an error, or with uncertain effects.
    pub fn from_outcome(outcome: &Result<Value>, uncertain: bool) -> Self {
        let trigger = match outcome {
            Ok(result) => Self::from_result(result),
            Err(error) => {
                Self::from_result(&json!({"status":"recovery_required","reason":error.to_string()}))
            }
        };
        if uncertain && trigger == Self::Turn {
            Self::Interrupted
        } else {
            trigger
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Turn => "turn",
            Self::Stop => "stop",
            Self::Signal => "signal",
            Self::LeaseLost => "lease_lost",
            Self::Interrupted => "interrupted",
        }
    }
    fn wire(self) -> WorkspaceCheckpointInputTrigger {
        match self {
            Self::Turn => WorkspaceCheckpointInputTrigger::Turn,
            Self::Stop => WorkspaceCheckpointInputTrigger::Stop,
            Self::Signal => WorkspaceCheckpointInputTrigger::Signal,
            Self::LeaseLost => WorkspaceCheckpointInputTrigger::LeaseLost,
            Self::Interrupted => WorkspaceCheckpointInputTrigger::Interrupted,
        }
    }
}

#[derive(Clone)]
pub struct RepoPaths {
    pub git_dir: PathBuf,
    pub workspace: PathBuf,
    pub private: Vec<PathBuf>,
}
impl RepoPaths {
    pub fn new(workspace: &Path, state_dir: &Path, private_dirs: &[PathBuf]) -> Self {
        let mut private = vec![state_dir.to_owned()];
        private.extend_from_slice(private_dirs);
        Self {
            git_dir: state_dir.join("workspace.git"),
            workspace: workspace.to_owned(),
            private,
        }
    }
}
/// Create isolated workspace staging before readiness.
pub async fn prepare(paths: &RepoPaths) -> Result<()> {
    blocking(paths.clone(), |_| Ok(())).await
}
async fn blocking<T: Send + 'static>(
    paths: RepoPaths,
    work: impl FnOnce(&PrivateRepo) -> Result<T> + Send + 'static,
) -> Result<T> {
    // A filesystem call inside libgit2 may not be interruptible. Runtime shutdown
    // must not join it after the checkpoint's timeout, as spawn_blocking would.
    // Normal traversals observe cancellation at their callbacks.
    let cancellation = Cancellation::default();
    let _cancel_on_drop = CancelOnDrop(cancellation.clone());
    let (send, receive) = tokio::sync::oneshot::channel();
    std::thread::Builder::new()
        .name("workspace-checkpoint".into())
        .spawn(move || {
            let result = (|| {
                let _permit = WorkerPermit::acquire(&paths.git_dir)?;
                PrivateRepo::open_or_init_cancellable(
                    &paths.git_dir,
                    &paths.workspace,
                    cancellation,
                )
                .and_then(|repo| work(&repo))
            })();
            let _ = send.send(result);
        })?;
    receive
        .await
        .map_err(|_| anyhow::anyhow!("checkpoint worker stopped"))?
}

// A timed-out worker may still be leaving an uninterruptible filesystem call.
// Hold its reservation until it actually returns; later checkpoints fail best-effort
// rather than racing its private index or initialization directory.
static ACTIVE_WORKERS: OnceLock<Mutex<HashSet<PathBuf>>> = OnceLock::new();
struct WorkerPermit(PathBuf);
impl WorkerPermit {
    fn acquire(git_dir: &Path) -> Result<Self> {
        let absolute = std::path::absolute(git_dir)?;
        let parent = absolute
            .parent()
            .ok_or_else(|| anyhow::anyhow!("invalid checkpoint path"))?;
        // Resolve aliases consistently even on the first initialization. This runs
        // on the dedicated worker so slow filesystem metadata cannot block Tokio.
        std::fs::create_dir_all(parent)?;
        let key = parent.canonicalize()?.join(absolute.file_name().unwrap());
        let mut active = ACTIVE_WORKERS
            .get_or_init(Default::default)
            .lock()
            .unwrap_or_else(|p| p.into_inner());
        anyhow::ensure!(
            active.insert(key.clone()),
            "checkpoint worker still stopping"
        );
        Ok(Self(key))
    }
}
impl Drop for WorkerPermit {
    fn drop(&mut self) {
        ACTIVE_WORKERS
            .get_or_init(Default::default)
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .remove(&self.0);
    }
}

struct CancelOnDrop(Cancellation);
impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.0.cancel();
    }
}

pub struct Plan {
    pub trigger: Trigger,
    /// The last run that completed on this attachment, if any.
    pub run_id: Option<String>,
    pub repo: RepoPaths,
    pub interactive: bool,
}
#[derive(Debug, thiserror::Error)]
enum CheckpointError {
    #[error("timeout")]
    Timeout,
    #[error("signal")]
    Signal,
    #[error("content_mismatch")]
    ContentMismatch,
}
/// A fixed reason for `checkpoint_failed`; error text can carry paths or remote details.
fn reason(error: &anyhow::Error) -> &'static str {
    if let Some(e) = error.downcast_ref::<CheckpointError>() {
        return match e {
            CheckpointError::Timeout => "timeout",
            CheckpointError::Signal => "signal",
            CheckpointError::ContentMismatch => "content_mismatch",
        };
    }
    if let Some(e) = error.downcast_ref::<RecordRefusal>() {
        return match e {
            RecordRefusal::AlreadyCheckpointed => "turn_already_checkpointed",
            RecordRefusal::NotQuiescent => "workspace_not_quiescent",
        };
    }
    match error.downcast_ref::<TransportFailure>() {
        Some(TransportFailure::Transient) => "service_unavailable",
        Some(_) => "service_rejected",
        None if error.downcast_ref::<reqwest::Error>().is_some() => "remote_unavailable",
        None => "checkpoint_error",
    }
}

#[derive(Clone, Copy)]
struct Events {
    interactive: bool,
    trigger: Trigger,
}
impl Events {
    fn started(&self, run_id: &Option<String>) {
        progress(
            self.interactive,
            "Saving workspace checkpoint…",
            json!({"event":"checkpoint_started","trigger":self.trigger.name(),"run_id":run_id}),
        );
    }
    fn pushed(&self, pushed: &Pushed) {
        progress(
            self.interactive,
            "Workspace checkpoint saved.",
            json!({"event":"checkpoint_pushed",
            "trigger":self.trigger.name(),"commit_sha":pushed.commit_sha,"commits":pushed.commits,
            "skipped":skipped_summary(&pushed.skipped)}),
        );
    }
    fn failed(&self, reason: &str) {
        progress(
            self.interactive,
            "Workspace checkpoint was not saved; the run result is unchanged.",
            json!({"event":"checkpoint_failed","trigger":self.trigger.name(),"reason":reason}),
        );
    }
}
fn skipped_summary(skipped: &[Skipped]) -> Value {
    let count = |reason| skipped.iter().filter(|s| s.reason == reason).count();
    json!({"link_outside_workspace":count(SkipReason::LinkOutsideWorkspace),
        "nested_repository":count(SkipReason::NestedRepository),"too_large":count(SkipReason::TooLarge),
        "paths":skipped.iter().take(20).map(|s| &s.path).collect::<Vec<_>>()})
}

/// Termination signals delivered after the checkpoint starts.
struct Interrupt {
    term: Option<Signal>,
    int: Option<Signal>,
}
impl Interrupt {
    fn new() -> Self {
        Self {
            term: signal(SignalKind::terminate()).ok(),
            int: signal(SignalKind::interrupt()).ok(),
        }
    }
    async fn recv(&mut self) {
        async fn next(signal: &mut Option<Signal>) {
            match signal {
                Some(signal) => {
                    signal.recv().await;
                }
                None => std::future::pending::<()>().await,
            }
        }
        tokio::select! { _ = next(&mut self.term) => {}, _ = next(&mut self.int) => {} }
    }
}

/// Take one checkpoint, bounded, and report it. Never fails the caller.
pub async fn checkpoint(
    transport: &Transport,
    plan: &Plan,
    mut reconcile: impl AsyncFnMut(Instant) -> Result<()>,
) {
    let events = Events {
        interactive: plan.interactive,
        trigger: plan.trigger,
    };
    events.started(&plan.run_id);
    let bound = if plan.trigger == Trigger::Signal {
        SIGNAL_GRACE
    } else {
        EXIT_BOUND
    };
    let deadline = Instant::now() + bound;
    let mut interrupt = Interrupt::new();
    let work =
        tokio::time::timeout_at(deadline, attempt(transport, plan, deadline, &mut reconcile));
    tokio::pin!(work);
    let result = tokio::select! {
        result = &mut work => result,
        _ = interrupt.recv() => tokio::time::timeout(SIGNAL_GRACE, &mut work)
            .await
            .unwrap_or(Ok(Err(CheckpointError::Signal.into()))),
    };
    match result.unwrap_or_else(|_| Err(checkpoint_timeout(plan.trigger).into())) {
        Ok(pushed) => events.pushed(&pushed),
        Err(error) => events.failed(reason(&error)),
    }
}

fn checkpoint_timeout(trigger: Trigger) -> CheckpointError {
    if trigger == Trigger::Signal {
        CheckpointError::Signal
    } else {
        CheckpointError::Timeout
    }
}

struct Pushed {
    commit_sha: String,
    commits: usize,
    skipped: Vec<Skipped>,
}
async fn attempt(
    transport: &Transport,
    plan: &Plan,
    deadline: Instant,
    reconcile: &mut impl AsyncFnMut(Instant) -> Result<()>,
) -> Result<Pushed> {
    let policy = transport
        .workspace_policy(plan.run_id.clone(), deadline)
        .await?;
    let private = plan.repo.private.clone();
    let snapshot = blocking(plan.repo.clone(), move |repo| {
        repo.write_exclude(&policy.ignore, &private, &policy.credentials)?;
        repo.snapshot()
    })
    .await?;
    let total =
        snapshot.blobs.iter().map(|b| b.size as u64).sum::<u64>() + snapshot.manifest.len() as u64;
    let mut sent = 0;
    for blob in &snapshot.blobs {
        let id = blob.id;
        let bytes = blocking(plan.repo.clone(), move |repo| repo.blob(id)).await?;
        upload(transport, bytes, &blob.sha256, deadline).await?;
        sent += blob.size as u64;
        progress(
            plan.interactive,
            "Uploading workspace…",
            serde_json::json!({
            "event":"checkpoint_progress", "objects":snapshot.blobs.len(), "bytes_sent":sent, "bytes_total":total}),
        );
    }
    let digest = format!("{:x}", Sha256::digest(&snapshot.manifest));
    upload(transport, snapshot.manifest, &digest, deadline).await?;
    let body = WorkspaceCheckpointInput {
        manifest_digest: digest.clone(),
        run_id: plan.run_id.clone(),
        trigger: plan.trigger.wire(),
    };
    record(transport, &body, deadline, reconcile).await?;
    Ok(Pushed {
        commit_sha: digest,
        commits: 1,
        skipped: snapshot.skipped,
    })
}
async fn upload(
    transport: &Transport,
    bytes: Vec<u8>,
    digest: &str,
    deadline: Instant,
) -> Result<()> {
    let size = bytes.len();
    let response = transport.upload_workspace_content(bytes, deadline).await?;
    if response.sha256 != digest || response.size != size as i64 {
        bail!(CheckpointError::ContentMismatch);
    }
    Ok(())
}
async fn record(
    transport: &Transport,
    body: &WorkspaceCheckpointInput,
    deadline: Instant,
    reconcile: &mut impl AsyncFnMut(Instant) -> Result<()>,
) -> Result<()> {
    let mut attempt = 0;
    let mut reconciled = false;
    loop {
        match transport.record_checkpoint(body, deadline).await {
            Ok(_) => return Ok(()),
            Err(error)
                if !reconciled
                    && matches!(
                        error.downcast_ref::<RecordRefusal>(),
                        Some(RecordRefusal::NotQuiescent)
                    ) =>
            {
                reconciled = true;
                if reconcile(deadline).await.is_err() {
                    return Err(error);
                }
            }
            Err(error) if transient(&error) && attempt < 2 => {
                tokio::time::sleep(backoff(&mut attempt)).await
            }
            Err(error) => return Err(error),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Trigger;
    use serde_json::json;
    #[test]
    fn timed_out_local_work_does_not_hold_the_executable_open() {
        const CHILD: &str = "SIKARU_CHECKPOINT_SHUTDOWN_TEST";
        if std::env::var_os(CHILD).is_some() {
            let directory = tempfile::tempdir().unwrap();
            let paths =
                super::RepoPaths::new(directory.path(), &directory.path().join("state"), &[]);
            let runtime = tokio::runtime::Runtime::new().unwrap();
            runtime.block_on(async {
                let (started, ready) = tokio::sync::oneshot::channel();
                let work = super::blocking(paths, move |_| {
                    let _ = started.send(());
                    // Model an uninterruptible filesystem read inside libgit2. Timeout must
                    // not turn into a runtime-shutdown wait for this synchronous operation.
                    std::thread::sleep(std::time::Duration::from_secs(30));
                    Ok(())
                });
                tokio::pin!(work);
                tokio::select! {
                    result = &mut work => panic!("worker finished unexpectedly: {result:?}"),
                    _ = ready => {}
                }
                assert!(
                    tokio::time::timeout(std::time::Duration::from_millis(20), work)
                        .await
                        .is_err()
                );
            });
            drop(runtime);
            return;
        }
        let mut child = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "timed_out_local_work_does_not_hold_the_executable_open",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .unwrap();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(3);
        while std::time::Instant::now() < deadline {
            if let Some(status) = child.try_wait().unwrap() {
                assert!(status.success());
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        child.kill().unwrap();
        child.wait().unwrap();
        panic!("checkpoint timeout left the executable waiting for synchronous work");
    }
    #[tokio::test]
    async fn cancelled_worker_reserves_its_repository_until_it_stops() {
        let directory = tempfile::tempdir().unwrap();
        let paths = super::RepoPaths::new(directory.path(), &directory.path().join("state"), &[]);
        // Create state before the reservation so path aliases resolve consistently.
        std::fs::create_dir_all(directory.path().join("state")).unwrap();
        let (started, ready) = tokio::sync::oneshot::channel();
        let (stop, wait) = std::sync::mpsc::channel();
        let mut work = Box::pin(super::blocking(paths.clone(), move |_| {
            let _ = started.send(());
            wait.recv().unwrap();
            Ok(())
        }));
        tokio::select! {
            result = &mut work => panic!("worker finished unexpectedly: {result:?}"),
            _ = ready => {}
        }
        assert!(
            tokio::time::timeout(std::time::Duration::from_millis(20), &mut work)
                .await
                .is_err()
        );
        drop(work);
        assert!(
            super::blocking::<()>(paths.clone(), |_| panic!("overlapping repository write"))
                .await
                .is_err()
        );
        stop.send(()).unwrap();
        tokio::time::timeout(std::time::Duration::from_secs(1), async {
            loop {
                if super::blocking(paths.clone(), |_| Ok(())).await.is_ok() {
                    break;
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
    }
    #[test]
    fn triggers_follow_the_final_result() {
        for (result, trigger) in [
            (
                json!({"status":"completed","execution":{"run_id":"r"}}),
                Trigger::Turn,
            ),
            (json!({"status":"failed"}), Trigger::Turn),
            (
                json!({"status":"cancelled","reason":"signal"}),
                Trigger::Signal,
            ),
            (
                json!({"status":"recovery_required","reason":"signal"}),
                Trigger::Signal,
            ),
            (
                json!({"status":"cancelled","reason":"controller_stopped"}),
                Trigger::Stop,
            ),
            (json!({"status":"cancelled"}), Trigger::Stop),
            (json!({"status":"approval_required"}), Trigger::Stop),
            (
                json!({"status":"cancelled","reason":"deadline"}),
                Trigger::Interrupted,
            ),
            (
                json!({"status":"recovery_required","reason":"lease_expired"}),
                Trigger::LeaseLost,
            ),
            (
                json!({"status":"recovery_required","reason":"credential_or_lease_expired"}),
                Trigger::LeaseLost,
            ),
            (
                json!({"status":"recovery_required","reason":"interrupted_operation_requires_reconciliation"}),
                Trigger::Interrupted,
            ),
        ] {
            assert_eq!(Trigger::from_result(&result), trigger, "{result}");
        }
    }
    #[test]
    fn errors_and_uncertain_effects_are_not_turns() {
        assert_eq!(
            Trigger::from_outcome(&Err(anyhow::anyhow!("lease_expired")), false),
            Trigger::LeaseLost
        );
        assert_eq!(
            Trigger::from_outcome(&Err(anyhow::anyhow!("recovery_required: lost")), false),
            Trigger::Interrupted
        );
        assert_eq!(
            Trigger::from_outcome(&Ok(json!({"status":"completed"})), true),
            Trigger::Interrupted
        );
        assert_eq!(
            Trigger::from_outcome(&Ok(json!({"status":"completed"})), false),
            Trigger::Turn
        );
    }
}
