//! Workspace checkpoints: one commit per push on the session branch, taken when a run ends,
//! while the lease is still renewed and before cleanup. Failures are reported on stderr and
//! never change the run's result.
use super::{
    checkpoint_repo::{Cancellation, Chain, Pack, PrivateRepo, SkipReason, Skipped, Staged},
    git_http::{GitHttpError, PushOutcome, Remote},
    runtime::progress,
    transport::{backoff, transient, RecordRefusal, Transport, TransportFailure},
};
use anyhow::{bail, Result};
use git2::Oid;
use serde_json::{json, Value};
use sikaru_sdk::api::*;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex, OnceLock,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{
    signal::unix::{signal, Signal, SignalKind},
    sync::watch,
    time::Instant,
};

/// Longest wait for a checkpoint when a run ends. One push is capped at `max_push_bytes`
/// (100 MB): about 100 s at 1 MB/s, plus the mint and record calls. It stays far below the
/// 15-minute remote token and the executor credential lifetime.
pub const EXIT_BOUND: Duration = Duration::from_secs(120);
/// A signal-triggered checkpoint, or one a signal interrupts, gets at most this long.
/// Container runtimes commonly allow 10 s between SIGTERM and SIGKILL; this leaves time for
/// cleanup and the result.
pub const SIGNAL_GRACE: Duration = Duration::from_secs(5);
/// Mint a new remote when its token has less than this left. Longer than `EXIT_BOUND`,
/// so no push starts on a token that can expire before the push is abandoned.
const TOKEN_MARGIN: Duration = Duration::from_secs(180);
const MAX_MINTS: u32 = 4;
/// The keeper never re-mints sooner than this after a mint, whatever lifetime the service grants.
const MIN_REMINT: Duration = Duration::from_secs(10);
/// Reparent attempts when another writer moved the branch; never a forced push.
const STALE_ATTEMPTS: u32 = 3;
/// Share of `max_push_bytes` one pack may use; the rest covers trees, commits and framing.
const PACK_BUDGET_PERCENT: u64 = 90;

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
/// Create the private repository (capturing the workspace's HEAD once) before readiness.
pub async fn prepare(paths: &RepoPaths) -> Result<()> {
    blocking(paths.clone(), |_| Ok(())).await
}
async fn blocking<T: Send + 'static>(
    paths: RepoPaths,
    work: impl FnOnce(&PrivateRepo) -> Result<T> + Send + 'static,
) -> Result<T> {
    // A filesystem call inside libgit2 may not be interruptible. Runtime shutdown
    // must not join it after the checkpoint's timeout, as spawn_blocking would.
    // Normal traversals and pack generation observe cancellation at their callbacks.
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
    #[error("stale_head")]
    StaleHead,
    #[error("push_rejected")]
    Rejected,
    #[error("push_too_large")]
    TooLarge,
    #[error("no_commit")]
    NoCommit,
}
/// A fixed reason for `checkpoint_failed`; error text can carry paths or remote details.
fn reason(error: &anyhow::Error) -> &'static str {
    if let Some(e) = error.downcast_ref::<CheckpointError>() {
        return match e {
            CheckpointError::Timeout => "timeout",
            CheckpointError::Signal => "signal",
            CheckpointError::StaleHead => "stale_head",
            CheckpointError::Rejected => "push_rejected",
            CheckpointError::TooLarge => "push_too_large",
            CheckpointError::NoCommit => "no_commit",
        };
    }
    if let Some(e) = error.downcast_ref::<RecordRefusal>() {
        return match e {
            RecordRefusal::NotOnBranch => "commit_not_on_branch",
            RecordRefusal::AlreadyCheckpointed => "turn_already_checkpointed",
            RecordRefusal::NotQuiescent => "workspace_not_quiescent",
        };
    }
    if let Some(e) = error.downcast_ref::<GitHttpError>() {
        return match e {
            GitHttpError::Unauthorized => "unauthorized",
            GitHttpError::Status(_) => "remote_unavailable",
            GitHttpError::Protocol => "remote_protocol",
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
    fn reporter(&self, objects: usize) -> impl Fn(u64, u64) + Send + Sync + 'static {
        let (interactive, last) = (self.interactive, Arc::new(AtomicU64::new(0)));
        move |sent, total| {
            let step = (total / 10).max(1);
            if interactive || (sent < total && sent < last.load(Ordering::Relaxed) + step) {
                return;
            }
            last.store(sent, Ordering::Relaxed);
            eprintln!(
                "{}",
                json!({"event":"checkpoint_progress","objects":objects,"bytes_sent":sent,"bytes_total":total})
            );
        }
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

/// The newest minted remote. The service mints only under a valid lease, so a checkpoint after
/// lease loss pushes with the token held here.
#[derive(Default)]
pub struct RemoteSlot(Mutex<Option<Minted>>);
impl RemoteSlot {
    fn current(&self) -> Option<Minted> {
        self.0.lock().unwrap_or_else(|p| p.into_inner()).clone()
    }
    fn store(&self, minted: Minted) {
        *self.0.lock().unwrap_or_else(|p| p.into_inner()) = Some(minted);
    }
    /// When the keeper mints next: `TOKEN_MARGIN` before expiry, but never before half the
    /// token's lifetime (at least `MIN_REMINT`) has passed, so a short-lived token cannot spin it.
    fn refresh_at(&self) -> Option<Instant> {
        self.current().map(|m| {
            let lifetime = m.expires.saturating_duration_since(m.minted_at);
            let margin = m.expires.checked_sub(TOKEN_MARGIN).unwrap_or(m.minted_at);
            margin.max(m.minted_at + (lifetime / 2).max(MIN_REMINT))
        })
    }
}
/// Keep a remote minted while the lease is valid. Runs beside the heartbeat for the whole run.
pub async fn keep_remote(
    transport: &Transport,
    slot: &RemoteSlot,
    lease: watch::Receiver<Instant>,
) {
    let mut attempt = 0;
    loop {
        // Copy the deadline out: a watch::Ref held across an await blocks lease renewal.
        let lease_end = *lease.borrow();
        if Instant::now() >= lease_end {
            return std::future::pending().await;
        }
        if let Some(at) = slot.refresh_at().filter(|at| Instant::now() < *at) {
            tokio::time::sleep_until(at.min(lease_end)).await;
            continue;
        }
        match transport.workspace_remote(lease_end).await {
            Ok(view) => {
                slot.store(Minted::from_view(view, 1));
                attempt = 0;
            }
            Err(error) if transient(&error) => tokio::time::sleep(backoff(&mut attempt)).await,
            // Not offered for this attachment: the checkpoint reports the failure itself.
            Err(_) => return std::future::pending().await,
        }
    }
}

/// Take one checkpoint, bounded, and report it. Never fails the caller.
pub async fn checkpoint(
    transport: &Transport,
    plan: &Plan,
    slot: &RemoteSlot,
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
    let work = tokio::time::timeout_at(
        deadline,
        attempt(transport, plan, slot, events, deadline, &mut reconcile),
    );
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
    slot: &RemoteSlot,
    events: Events,
    deadline: Instant,
    reconcile: &mut impl AsyncFnMut(Instant) -> Result<()>,
) -> Result<Pushed> {
    let mut remote = match slot.current() {
        Some(minted) => minted,
        None => Minted::mint(transport, deadline).await?,
    };
    let ignore = remote.view.ignore_defaults.clone();
    let private = plan.repo.private.clone();
    let staged = blocking(plan.repo.clone(), move |repo| {
        repo.write_exclude(&ignore, &private)?;
        repo.stage()
    })
    .await?;
    let pushed = push_until_current(transport, plan, &mut remote, staged, events, deadline).await?;
    slot.store(remote);
    // Only the final head is recorded: an intermediate commit of a split push is a partial
    // workspace, and recording it would let reads serve that partial tree.
    let body = WorkspaceCheckpointInput {
        commit_sha: pushed.commit_sha.clone(),
        run_id: plan.run_id.clone(),
        trigger: plan.trigger.wire(),
    };
    record(transport, &body, deadline, reconcile).await?;
    Ok(pushed)
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
/// Push the staged tree as a chain of single-commit pushes on the session branch. The first
/// checkpoint on an unborn branch is a root commit: the branch never carries task history.
async fn push_until_current(
    transport: &Transport,
    plan: &Plan,
    remote: &mut Minted,
    staged: Staged,
    events: Events,
    deadline: Instant,
) -> Result<Pushed> {
    let refname = branch_ref(&remote.view.branch);
    let limit = u64::try_from(remote.view.max_push_bytes).unwrap_or(0);
    let budget = limit.saturating_mul(PACK_BUDGET_PERCENT) / 100;
    let message = format!(
        "checkpoint: {} {}",
        plan.trigger.name(),
        plan.run_id.as_deref().unwrap_or("-")
    );
    let mut commits = 0;
    for _ in 0..=STALE_ATTEMPTS {
        remote.fresh(transport, deadline).await?;
        let head = read_head(transport, remote, &refname, deadline).await?;
        let (target, text) = (staged.tree, message.clone());
        let chain = blocking(plan.repo.clone(), move |repo| {
            repo.chain(head, target, budget, &text)
        })
        .await?;
        let mut skipped = staged.skipped.clone();
        skipped.extend(chain.skipped.iter().cloned());
        let mut sender = ChainSender {
            transport,
            plan,
            remote,
            events,
            deadline,
            refname: &refname,
            limit,
        };
        let (finished, accepted) = sender.push(&chain, head).await?;
        commits += accepted;
        if finished {
            let last = chain
                .commits
                .last()
                .copied()
                .or(head)
                .ok_or(CheckpointError::NoCommit)?;
            return Ok(Pushed {
                commit_sha: last.to_string(),
                commits,
                skipped,
            });
        }
    }
    bail!(CheckpointError::StaleHead)
}

struct ChainSender<'a> {
    transport: &'a Transport,
    plan: &'a Plan,
    remote: &'a mut Minted,
    events: Events,
    deadline: Instant,
    refname: &'a str,
    limit: u64,
}
impl ChainSender<'_> {
    async fn push(&mut self, chain: &Chain, head: Option<Oid>) -> Result<(bool, usize)> {
        let mut old = head;
        for (i, commit) in chain.commits.iter().copied().enumerate() {
            let hide: Vec<Oid> = old.into_iter().collect();
            let pack =
                blocking(self.plan.repo.clone(), move |repo| repo.pack(commit, &hide)).await?;
            if pack.bytes.len() as u64 + 512 > self.limit {
                bail!(CheckpointError::TooLarge);
            }
            self.remote.fresh(self.transport, self.deadline).await?;
            let outcome = push_one(
                self.transport,
                self.remote,
                self.refname,
                old,
                commit,
                pack,
                self.events,
                self.deadline,
            )
            .await?;
            if outcome == PushOutcome::Refused {
                let current =
                    read_head(self.transport, self.remote, self.refname, self.deadline).await?;
                if current == old {
                    bail!(CheckpointError::Rejected);
                }
                return Ok((false, i));
            }
            let name = self.refname.to_owned();
            blocking(self.plan.repo.clone(), move |repo| {
                repo.set_pushed(&name, commit)
            })
            .await?;
            old = Some(commit);
        }
        Ok((true, chain.commits.len()))
    }
}
#[allow(clippy::too_many_arguments)]
async fn push_one(
    transport: &Transport,
    remote: &mut Minted,
    refname: &str,
    old: Option<Oid>,
    new: Oid,
    pack: Pack,
    events: Events,
    deadline: Instant,
) -> Result<PushOutcome> {
    let (objects, bytes) = (pack.objects, bytes::Bytes::from(pack.bytes));
    let (old, new) = (old.map(|o| o.to_string()), new.to_string());
    for attempt in 0..2 {
        let destination = remote.remote(&transport.http, refname);
        let push = destination.push(
            old.as_deref(),
            &new,
            bytes.clone(),
            events.reporter(objects),
        );
        match within(deadline, push).await {
            Err(error) if attempt == 0 && unauthorized(&error) => {
                remote.renew(transport, deadline).await?
            }
            other => return other,
        }
    }
    bail!(GitHttpError::Unauthorized)
}
/// The session branch head the remote advertises; None for an unborn branch.
async fn read_head(
    transport: &Transport,
    remote: &mut Minted,
    refname: &str,
    deadline: Instant,
) -> Result<Option<Oid>> {
    for attempt in 0..2 {
        match within(
            deadline,
            remote.remote(&transport.http, refname).advertisement(),
        )
        .await
        {
            Err(error) if attempt == 0 && unauthorized(&error) => {
                remote.renew(transport, deadline).await?
            }
            Err(error) => return Err(error),
            Ok(advertised) => {
                return Ok(advertised.head.as_deref().map(Oid::from_str).transpose()?);
            }
        }
    }
    bail!(GitHttpError::Unauthorized)
}
fn unauthorized(error: &anyhow::Error) -> bool {
    matches!(
        error.downcast_ref::<GitHttpError>(),
        Some(GitHttpError::Unauthorized)
    )
}
async fn within<T>(
    deadline: Instant,
    work: impl std::future::Future<Output = Result<T>>,
) -> Result<T> {
    tokio::time::timeout_at(deadline, work)
        .await
        .map_err(|_| anyhow::Error::new(CheckpointError::Timeout))?
}
fn branch_ref(branch: &str) -> String {
    if branch.starts_with("refs/") {
        branch.to_owned()
    } else {
        format!("refs/heads/{branch}")
    }
}

/// A minted remote and when its token stops working.
#[derive(Clone)]
struct Minted {
    view: WorkspaceRemoteView,
    minted_at: Instant,
    expires: Instant,
    mints: u32,
}
impl Minted {
    fn from_view(view: WorkspaceRemoteView, mints: u32) -> Self {
        Self {
            minted_at: Instant::now(),
            expires: expiry(rfc3339_seconds(&view.expires_at).unwrap_or(0)),
            view,
            mints,
        }
    }
    async fn mint(transport: &Transport, deadline: Instant) -> Result<Self> {
        let mut attempt = 0;
        loop {
            match transport.workspace_remote(deadline).await {
                Ok(view) => return Ok(Self::from_view(view, 1)),
                Err(error) if transient(&error) && attempt < 2 => {
                    tokio::time::sleep(backoff(&mut attempt)).await
                }
                Err(error) => return Err(error),
            }
        }
    }
    /// A new token. After lease loss the service refuses, and the caller keeps the held one.
    async fn renew(&mut self, transport: &Transport, deadline: Instant) -> Result<()> {
        if self.mints >= MAX_MINTS {
            bail!(GitHttpError::Unauthorized);
        }
        let next = Self::mint(transport, deadline).await?;
        *self = Self {
            mints: self.mints + 1,
            ..next
        };
        Ok(())
    }
    async fn fresh(&mut self, transport: &Transport, deadline: Instant) -> Result<()> {
        if Instant::now() + TOKEN_MARGIN >= self.expires && self.mints < MAX_MINTS {
            // Best effort: a refusal (for example after lease loss) leaves the held token in use.
            let _ = self.renew(transport, deadline).await;
        }
        Ok(())
    }
    fn remote<'a>(&'a self, http: &'a reqwest::Client, refname: &'a str) -> Remote<'a> {
        Remote {
            http,
            url: &self.view.remote_url,
            username: "x-token",
            token: &self.view.token,
            refname,
        }
    }
}
fn expiry(unix_seconds: i64) -> Instant {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64);
    Instant::now() + Duration::from_secs(unix_seconds.saturating_sub(now).max(0) as u64)
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

/// Parse the service's RFC3339 expiry without accepting malformed or truncated timestamps.
fn rfc3339_seconds(text: &str) -> Result<i64> {
    validate_timestamp_layout(text)?;
    Ok(date_seconds(text)? + time_seconds(text)? - timestamp_offset(&text[19..])?)
}
fn timestamp_field(text: &str, range: std::ops::Range<usize>) -> Result<i64> {
    let value = text.get(range).ok_or(CheckpointError::Timeout)?;
    if !value.bytes().all(|b| b.is_ascii_digit()) {
        bail!(CheckpointError::Timeout);
    }
    Ok(value.parse()?)
}
fn date_seconds(text: &str) -> Result<i64> {
    let (year, month, day) = (
        timestamp_field(text, 0..4)?,
        timestamp_field(text, 5..7)?,
        timestamp_field(text, 8..10)?,
    );
    validate_calendar(year, month, day)?;
    let (y, m) = if month <= 2 {
        (year - 1, month + 9)
    } else {
        (year, month - 3)
    };
    let days =
        365 * y + y.div_euclid(4) - y.div_euclid(100) + y.div_euclid(400) + (153 * m + 2) / 5 + day
            - 1
            - 719_468;
    Ok(days * 86_400)
}
fn time_seconds(text: &str) -> Result<i64> {
    let (hour, minute, second) = (
        timestamp_field(text, 11..13)?,
        timestamp_field(text, 14..16)?,
        timestamp_field(text, 17..19)?,
    );
    if hour > 23 || minute > 59 || second > 59 {
        bail!(CheckpointError::Timeout);
    }
    Ok(hour * 3600 + minute * 60 + second)
}
fn validate_timestamp_layout(text: &str) -> Result<()> {
    if !text.is_ascii() || text.len() < 20 {
        bail!(CheckpointError::Timeout);
    }
    for (index, value) in [(4, b'-'), (7, b'-'), (10, b'T'), (13, b':'), (16, b':')] {
        if text.as_bytes()[index] != value {
            bail!(CheckpointError::Timeout);
        }
    }
    Ok(())
}
fn validate_calendar(year: i64, month: i64, day: i64) -> Result<()> {
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = match month {
        2 => 28 + i64::from(leap),
        4 | 6 | 9 | 11 => 30,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => bail!(CheckpointError::Timeout),
    };
    if day < 1 || day > days {
        bail!(CheckpointError::Timeout);
    }
    Ok(())
}
fn timestamp_offset(tail: &str) -> Result<i64> {
    let zone = if let Some(fraction) = tail.strip_prefix('.') {
        let digits = fraction.bytes().take_while(u8::is_ascii_digit).count();
        if digits == 0 {
            bail!(CheckpointError::Timeout);
        }
        &fraction[digits..]
    } else {
        tail
    };
    if zone == "Z" {
        return Ok(0);
    }
    parse_offset(zone)
}
fn parse_offset(zone: &str) -> Result<i64> {
    if zone.len() != 6 || !matches!(zone.as_bytes()[0], b'+' | b'-') || zone.as_bytes()[3] != b':' {
        bail!(CheckpointError::Timeout);
    }
    let (hours, minutes) = (timestamp_field(zone, 1..3)?, timestamp_field(zone, 4..6)?);
    if hours > 23 || minutes > 59 {
        bail!(CheckpointError::Timeout);
    }
    let sign = if zone.starts_with('-') { -1 } else { 1 };
    Ok(sign * (hours * 3600 + minutes * 60))
}

#[cfg(test)]
mod expiry_tests {
    use super::*;
    #[test]
    fn expiry_accepts_utc_fractional_seconds_and_numeric_offsets() {
        for (text, expected) in [
            ("1970-01-02T00:00:00Z", 86_400),
            ("1970-01-01T01:00:00+01:00", 0),
            ("1970-01-01T00:00:00.250Z", 0),
            ("1970-01-01T00:00:00-01:30", 5_400),
            ("2000-02-29T00:00:00Z", 951_782_400),
        ] {
            assert_eq!(rfc3339_seconds(text).unwrap(), expected, "{text}");
        }
    }
    #[test]
    fn malformed_expiry_cannot_panic_or_create_a_long_lived_token() {
        for text in [
            "",
            "x",
            "🕐",
            "1970-01-01T00:00:00",
            "1970-01-01T00:00:00+",
            "1970-01-01T00:00:00.Z",
            "1970-13-01T00:00:00Z",
            "1970-02-30T00:00:00Z",
            "1970-01-01T24:00:00Z",
            "1970-01-01T00:00:00+24:00",
            "1970-01-01T00:00:00+01:-1",
        ] {
            assert!(rfc3339_seconds(text).is_err(), "{text}");
        }
    }
}
