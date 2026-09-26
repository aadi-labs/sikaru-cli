//! Unix owned process groups. Retain an unreaped leader until group cleanup.
use super::{config::control_variable, journal::Journal};
use anyhow::{bail, Context, Result};
use serde_json::{json, Value};
use std::os::{
    fd::{AsRawFd, FromRawFd},
    unix::{
        fs::{FileExt, OpenOptionsExt},
        process::CommandExt,
    },
};
use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::JoinHandle,
    time::{Duration, Instant},
};
use tokio::sync::watch;
const ARTIFACT_LIMIT: usize = 1024 * 1024;
const PAGE_LIMIT: usize = 24 * 1024; // JSON escaping of every byte still fits the receipt limit.
const MAX_CONDITIONS: usize = 64;
const NOTICE_TAIL_BYTES: u64 = 1024; // the default completion-notice tail
/// Re-check cadence for conditions without a local event source (path, port, HTTP).
const RECHECK: Duration = Duration::from_millis(100);
const MATCH_CHARS: usize = 1000;
/// Output kept from the previous scan, so a match may span two reads.
const CARRY_BYTES: usize = 4096;
#[derive(Debug)]
struct UnknownProcessHandle(String);
impl std::fmt::Display for UnknownProcessHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("unknown process handle")
    }
}
impl std::error::Error for UnknownProcessHandle {}

fn process_observation(result: Result<Value>) -> Result<Value> {
    match result {
        Err(error) if error.is::<UnknownProcessHandle>() => {
            let missing = error.downcast_ref::<UnknownProcessHandle>().unwrap();
            Ok(json!({"status":"error", "handle_id":missing.0, "error":{
                "code":"unknown_process_handle",
                "message":"Use the process id returned by bash.run or bash.start, not a tool-call id."}}))
        }
        other => other,
    }
}

pub struct Job {
    child: Child,
    group: Arc<Mutex<Option<i32>>>,
    artifact: File,
    path: PathBuf,
    reader: Option<JoinHandle<std::io::Result<()>>>,
    reader_stop: Arc<AtomicBool>,
    truncated: Arc<AtomicBool>,
    timed_out: Arc<AtomicBool>,
    deadline: Instant,
    state: Value,
    closed: bool,
    cleanup_failed: bool,
    /// Set once the leader exited and its output drained, or the job finished.
    exit: Arc<watch::Sender<bool>>,
    /// Bumped whenever output reaches the artifact; wakes log watchers.
    activity: Arc<watch::Sender<u64>>,
}
pub struct Processes {
    jobs: BTreeMap<String, Job>,
    terminal: BTreeMap<String, Value>,
    timeout: Duration,
}
impl Processes {
    pub fn new(journal: &Journal, timeout: Duration) -> Self {
        let terminal = journal
            .handles
            .iter()
            .map(|(id, state)| {
                let mut state = state.clone();
                if state["status"] == "running" {
                    state["status"] = json!("failed");
                    state["reason"] = json!("ownership_lost");
                }
                (id.clone(), state)
            })
            .collect();
        Self {
            jobs: BTreeMap::new(),
            terminal,
            timeout,
        }
    }
    pub fn maintenance(&mut self, journal: &mut Journal) -> Result<()> {
        for (id, job) in &mut self.jobs {
            if job.maintain()? {
                journal.handle(id, job.state.clone())?;
            }
        }
        Ok(())
    }
    #[cfg(test)]
    pub async fn execute(
        &mut self,
        method: &str,
        args: &HashMap<String, Value>,
        journal: &mut Journal,
    ) -> Result<Value> {
        self.execute_owned(method, args, journal, None).await
    }
    pub async fn execute_owned(
        &mut self,
        method: &str,
        args: &HashMap<String, Value>,
        journal: &mut Journal,
        operation_key: Option<&str>,
    ) -> Result<Value> {
        let result = match method {
            "bash.run" => self.run(args, journal, operation_key).await,
            "workspace.write_text" => write_text(args, journal.binding.anchor.path()),
            "bash.start" => self.start(args, journal, operation_key),
            "bash.read" => self.read(args, journal),
            "bash.wait" => self.wait(args, journal).await,
            "bash.wait_for" => self.wait_for(args, journal).await,
            "jobs.next_completed" => self.next_completed(args, journal).await,
            "bash.cancel" => self.cancel(args, journal),
            _ => bail!("unsupported compute operation"),
        };
        process_observation(result)
    }
    async fn run(
        &mut self,
        args: &HashMap<String, Value>,
        journal: &mut Journal,
        operation_key: Option<&str>,
    ) -> Result<Value> {
        allowed(args, &["command", "cwd", "env", "yield_seconds", "limit"])?;
        let seconds = args
            .get("yield_seconds")
            .map(|value| value.as_f64().context("yield_seconds must be numeric"))
            .transpose()?
            .unwrap_or(1.0);
        if !seconds.is_finite() || seconds < 0.0 {
            bail!("invalid yield_seconds");
        }
        let limit = integer(args, "limit", 8192)?.min(PAGE_LIMIT as u64);
        if limit == 0 {
            bail!("output limit must be positive");
        }
        let mut spawn_args = args.clone();
        spawn_args.remove("yield_seconds");
        spawn_args.remove("limit");
        let state = self.start(&spawn_args, journal, operation_key)?;
        let handle = state["id"].clone();
        let wait_args = HashMap::from([
            ("handle_id".into(), handle.clone()),
            ("timeout".into(), json!(seconds)),
        ]);
        self.wait(&wait_args, journal).await?;
        let read_args = HashMap::from([
            ("handle_id".into(), handle),
            ("offset".into(), json!(0)),
            ("limit".into(), json!(limit)),
        ]);
        self.read(&read_args, journal)
    }
    fn start(
        &mut self,
        args: &HashMap<String, Value>,
        journal: &mut Journal,
        operation_key: Option<&str>,
    ) -> Result<Value> {
        allowed(args, &["command", "cwd", "env"])?;
        let command = string(args, "command")?;
        if command.trim().is_empty() {
            bail!("empty command");
        }
        let cwd = resolve(
            journal.binding.anchor.path(),
            args.get("cwd")
                .map(|v| v.as_str().context("cwd must be a string"))
                .transpose()?
                .unwrap_or("."),
        );
        let env = task_env(args.get("env"))?;
        journal.verify()?;
        let id = format!("{:032x}", rand::random::<u128>());
        let (artifact, path) = journal.artifact(&id)?;
        let mut job = Job::spawn(&id, command, &cwd, env, artifact, path, self.timeout)?;
        if let Some(key) = operation_key {
            job.state["operation_key"] = json!(key);
        }
        let state = job.state.clone();
        // Store ownership before any fallible journal write. Drop cleans it on error.
        self.jobs.insert(id.clone(), job);
        journal.handle(&id, state.clone())?;
        Ok(state)
    }
    pub fn owns_operation(&self, key: &str) -> bool {
        self.jobs
            .values()
            .any(|job| job.state["operation_key"] == key)
    }
    fn state(&self, id: &str) -> Result<Value> {
        if let Some(job) = self.jobs.get(id) {
            return Ok(job.state.clone());
        }
        self.terminal
            .get(id)
            .cloned()
            .ok_or_else(|| UnknownProcessHandle(id.to_owned()).into())
    }
    fn read(&mut self, args: &HashMap<String, Value>, journal: &mut Journal) -> Result<Value> {
        allowed(args, &["handle_id", "offset", "limit"])?;
        self.maintenance(journal)?;
        let id = string(args, "handle_id")?;
        let state = self.state(id)?;
        let limit = integer(args, "limit", 16384)?.min(PAGE_LIMIT as u64);
        if limit == 0 {
            bail!("output limit must be positive");
        }
        let Some(job) = self.jobs.get_mut(id) else {
            let (artifact, path) = journal.existing_artifact(id)?;
            return read_page(&artifact, &path, args, state, limit);
        };
        read_page(&job.artifact, &job.path, args, state, limit)
    }
    async fn wait(
        &mut self,
        args: &HashMap<String, Value>,
        journal: &mut Journal,
    ) -> Result<Value> {
        allowed(args, &["handle_id", "timeout"])?;
        let id = string(args, "handle_id")?;
        let until = self.wait_deadline(args.get("timeout"))?;
        self.maintenance(journal)?;
        self.state(id)?;
        // One wait is served by the exit event itself, bounded by the deadline.
        if let Some(signal) = self.exit_signal(id) {
            exited(signal, until).await;
        }
        self.maintenance(journal)?;
        self.state(id)
    }
    /// The requested timeout, clamped by the command deadline.
    fn wait_deadline(&self, timeout: Option<&Value>) -> Result<Instant> {
        let seconds = timeout
            .map(|v| v.as_f64().context("timeout must be numeric"))
            .transpose()?
            .unwrap_or(self.timeout.as_secs_f64());
        if !seconds.is_finite() || seconds < 0.0 {
            bail!("invalid wait timeout");
        }
        Ok(Instant::now() + Duration::from_secs_f64(seconds.min(self.timeout.as_secs_f64())))
    }
    /// A running owned job's exit event; None once it is terminal or unknown.
    fn exit_signal(&self, id: &str) -> Option<watch::Receiver<bool>> {
        self.jobs
            .get(id)
            .filter(|job| !job.closed)
            .map(|job| job.exit.subscribe())
    }
    /// `bash.wait_for`: one call that returns when the first condition fires,
    /// when none can fire any more, or at its single deadline; never an error on the deadline.
    async fn wait_for(
        &mut self,
        args: &HashMap<String, Value>,
        journal: &mut Journal,
    ) -> Result<Value> {
        allowed(args, &["conditions", "timeout"])?;
        let raw = args
            .get("conditions")
            .and_then(Value::as_array)
            .filter(|items| !items.is_empty() && items.len() <= MAX_CONDITIONS)
            .context("conditions must be a nonempty list")?;
        let workspace = journal.binding.anchor.path().to_owned();
        let conditions = raw
            .iter()
            .map(|item| Condition::parse(item, &workspace))
            .collect::<Result<Vec<_>>>()?;
        let started = Instant::now();
        let until = self.wait_deadline(args.get("timeout"))?;
        self.maintenance(journal)?;
        let watchers = conditions
            .iter()
            .map(|c| self.watcher(c, journal, until))
            .collect::<Result<Vec<_>>>()?;
        let (outcomes, expired) = watch_until_fired(watchers, until).await;
        self.maintenance(journal)?;
        let entries = raw
            .iter()
            .zip(&conditions)
            .zip(outcomes)
            .map(|((item, condition), outcome)| self.entry(item, condition, outcome, expired))
            .collect::<Vec<_>>();
        Ok(json!({"status": wait_status(&entries),
            "elapsed_seconds": (started.elapsed().as_secs_f64() * 1000.0).round() / 1000.0,
            "conditions": entries}))
    }
    /// `jobs.next_completed`: the first of the named processes to finish, as a notice
    /// with the last `tail_bytes` of its output.
    async fn next_completed(
        &mut self,
        args: &HashMap<String, Value>,
        journal: &mut Journal,
    ) -> Result<Value> {
        allowed(args, &["handle_ids", "timeout", "tail_bytes"])?;
        let candidates = handle_ids(args.get("handle_ids"))?;
        let tail = integer(args, "tail_bytes", NOTICE_TAIL_BYTES)?;
        if tail == 0 {
            bail!("tail_bytes must be positive");
        }
        let until = self.wait_deadline(args.get("timeout"))?;
        self.maintenance(journal)?;
        let exits = candidates
            .iter()
            .map(|id| Condition::Exit(id.clone()))
            .collect::<Vec<_>>();
        let watchers = exits
            .iter()
            .map(|c| self.watcher(c, journal, until))
            .collect::<Result<Vec<_>>>()?;
        let (outcomes, _) = watch_until_fired(watchers, until).await;
        self.maintenance(journal)?;
        let first = outcomes
            .iter()
            .position(|o| o.as_ref().is_some_and(Outcome::fired));
        let Some(index) = first else {
            return Ok(json!({"completed": null, "pending": candidates}));
        };
        let notice = self.notice(&candidates[index], tail, journal)?;
        let pending = candidates
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != index)
            .map(|(_, id)| id)
            .collect::<Vec<_>>();
        Ok(json!({"completed": notice, "pending": pending}))
    }
    fn notice(&mut self, id: &str, tail_bytes: u64, journal: &mut Journal) -> Result<Value> {
        let tail = HashMap::from([
            ("handle_id".into(), json!(id)),
            ("limit".into(), json!(tail_bytes)),
        ]);
        let page = self.read(&tail, journal)?;
        Ok(
            json!({"id": id, "status": page["status"], "returncode": page["returncode"],
            "tail": page["output"], "omitted_before": page["omitted_before"]}),
        )
    }
    /// A 'static future watching one condition; it never borrows the registry.
    fn watcher(&self, condition: &Condition, journal: &Journal, until: Instant) -> Result<Watcher> {
        Ok(match condition {
            Condition::Exit(id) => self.exit_watcher(id),
            Condition::Log { handle, pattern } => self.log_watcher(handle, pattern, journal)?,
            Condition::Path { path, state } => Box::pin(watch_path(path.clone(), *state)),
            Condition::Port { host, port } => Box::pin(watch_port(host.clone(), *port)),
            Condition::Http { url, status } => Box::pin(watch_http(url.clone(), *status, until)),
        })
    }
    fn exit_watcher(&self, id: &str) -> Watcher {
        match (self.exit_signal(id), self.state(id).is_ok()) {
            (Some(mut signal), _) => Box::pin(async move {
                let _ = signal.wait_for(|done| *done).await.is_ok();
                Outcome::hit(Value::Null)
            }),
            (None, true) => Box::pin(std::future::ready(Outcome::hit(Value::Null))),
            (None, false) => Box::pin(std::future::ready(Outcome::reason("error"))),
        }
    }
    fn log_watcher(&self, id: &str, pattern: &str, journal: &Journal) -> Result<Watcher> {
        // Syntax this regex engine lacks (for example look-around) cannot be watched here.
        let Ok(pattern) = regex::bytes::Regex::new(pattern) else {
            return Ok(Box::pin(std::future::ready(Outcome::reason("unsupported"))));
        };
        if self.state(id).is_err() {
            return Ok(Box::pin(std::future::ready(Outcome::reason("error"))));
        }
        let (artifact, signals) = match self.jobs.get(id) {
            Some(job) => (
                job.artifact.try_clone()?,
                (!job.closed).then(|| (job.exit.subscribe(), job.activity.subscribe())),
            ),
            None => (journal.existing_artifact(id)?.0, None),
        };
        let scanner = LogScanner::new(artifact, pattern);
        Ok(Box::pin(watch_log(scanner, signals)))
    }
    fn entry(
        &self,
        item: &Value,
        condition: &Condition,
        outcome: Option<Outcome>,
        expired: bool,
    ) -> Value {
        let outcome = outcome.unwrap_or(Outcome::reason(if expired {
            "deadline"
        } else {
            "pending"
        }));
        let observed = match (condition, outcome.fired()) {
            (Condition::Exit(id), true) => {
                let state = self.state(id).unwrap_or(Value::Null);
                json!({"status": state["status"], "returncode": state["returncode"]})
            }
            _ => outcome.observed.clone().unwrap_or(Value::Null),
        };
        json!({"condition": item, "fired": outcome.fired(), "observed": observed, "reason": outcome.reason})
    }
    fn cancel(&mut self, args: &HashMap<String, Value>, journal: &mut Journal) -> Result<Value> {
        allowed(args, &["handle_id"])?;
        let id = string(args, "handle_id")?;
        if let Some(job) = self.jobs.get_mut(id) {
            job.finish(true, false)?;
            journal.handle(id, job.state.clone())?;
        }
        self.state(id)
    }
    pub fn cleanup(&mut self, journal: &mut Journal) -> Result<()> {
        let mut failed = false;
        for (id, job) in &mut self.jobs {
            let result = job
                .finish(true, false)
                .and_then(|_| journal.handle(id, job.state.clone()));
            if let Err(error) = result {
                eprintln!("native owned process cleanup: {error}");
                failed = true;
            }
        }
        if self
            .terminal
            .values()
            .any(|v| v["reason"] == "ownership_lost")
        {
            failed = true;
        }
        if failed {
            bail!("cleanup_unconfirmed: process ownership or durable cleanup failed");
        }
        Ok(())
    }
    pub fn observation(&self, id: &str) -> sikaru_sdk::api::ProcessObservation {
        let state = self
            .state(id)
            .unwrap_or(json!({"status":"failed","reason":"ownership_lost"}));
        let status = if state["reason"] == "ownership_lost" {
            "lost"
        } else {
            state["status"].as_str().unwrap_or("lost")
        };
        sikaru_sdk::api::ProcessObservation {
            handle_id: id.into(),
            status: serde_json::from_value(json!(status)).unwrap(),
            evidence: "native owned process registry".into(),
        }
    }
}
impl Job {
    fn spawn(
        id: &str,
        command: &str,
        cwd: &Path,
        env: BTreeMap<String, String>,
        artifact: File,
        path: PathBuf,
        timeout: Duration,
    ) -> Result<Self> {
        let (mut read, write) = output_pipe()?;
        let mut cmd = Command::new("/bin/bash");
        cmd.args(["-c", command])
            .current_dir(cwd)
            .env_clear()
            .envs(env)
            .stdin(Stdio::null())
            .stdout(Stdio::from(write.try_clone()?))
            .stderr(Stdio::from(write));
        unsafe {
            cmd.pre_exec(|| {
                if libc::setsid() < 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let mut sink = artifact.try_clone()?;
        let child = cmd.spawn().context("shell launch failed")?;
        let group = Arc::new(Mutex::new(Some(child.id() as i32)));
        let timed_out = Arc::new(AtomicBool::new(false));
        let exit = Arc::new(watch::channel(false).0);
        let activity = Arc::new(watch::channel(0u64).0);
        let supervisor = Supervisor {
            group: group.clone(),
            timed_out: timed_out.clone(),
            deadline: Instant::now() + timeout,
            exit: exit.clone(),
            activity: activity.clone(),
        };
        let deadline = Instant::now() + timeout;
        let stop = Arc::new(AtomicBool::new(false));
        let truncated = Arc::new(AtomicBool::new(false));
        let reader_stop = stop.clone();
        let overflow = truncated.clone();
        let mut job = Self {
            child,
            group,
            artifact,
            path,
            reader: None,
            reader_stop,
            truncated,
            timed_out,
            deadline,
            state: json!({"id":id,"status":"running","returncode":null,"truncated":false,"timed_out":false}),
            closed: false,
            cleanup_failed: false,
            exit,
            activity,
        };
        job.reader = Some(
            std::thread::Builder::new()
                .name("compute-output".into())
                .spawn(move || copy_output(&mut read, &mut sink, &stop, &overflow, &supervisor))?,
        );
        Ok(job)
    }
    fn maintain(&mut self) -> Result<bool> {
        if self.closed {
            return Ok(false);
        }
        let exited = leader_exited(self.child.id())?;
        let timeout =
            self.timed_out.load(Ordering::Acquire) || (Instant::now() >= self.deadline && !exited);
        if timeout || self.truncated.load(Ordering::Acquire) || exited {
            self.finish(false, timeout)?;
            return Ok(true);
        }
        Ok(false)
    }
    fn finish(&mut self, cancelled: bool, timed_out: bool) -> Result<()> {
        if self.cleanup_failed {
            bail!("prior cleanup remains unconfirmed");
        }
        if self.closed {
            return Ok(());
        }
        // Synchronize with the independent output/deadline supervisor before reaping.
        let mut group = self
            .group
            .lock()
            .map_err(|_| anyhow::anyhow!("process ownership lock failed"))?;
        let pid = group.context("process ownership was already released")?;
        signal_owned_group(pid)?;
        confirm_group_exit(pid)?;
        *group = None;
        drop(group);
        let status = self.child.wait()?;
        self.closed = true;
        self.cleanup_failed = true;
        self.close_output()?;
        self.cleanup_failed = false;
        self.record_exit(
            status,
            cancelled,
            timed_out || self.timed_out.load(Ordering::Acquire),
        );
        self.exit.send_replace(true);
        Ok(())
    }
    fn record_exit(&mut self, status: std::process::ExitStatus, cancelled: bool, timed_out: bool) {
        self.state["status"] = json!(if cancelled { "cancelled" } else { "exited" });
        self.state["returncode"] = json!(status.code().or_else(|| {
            use std::os::unix::process::ExitStatusExt;
            status.signal().map(|s| -s)
        }));
        self.state["truncated"] = json!(self.truncated.load(Ordering::Acquire));
        self.state["timed_out"] = json!(timed_out);
    }
    fn close_output(&mut self) -> Result<()> {
        self.reader_stop.store(true, Ordering::Release);
        if let Some(reader) = self.reader.take() {
            reader
                .join()
                .map_err(|_| anyhow::anyhow!("output collector failed"))??;
        }
        self.artifact.sync_all()?;
        Ok(())
    }
}
impl Drop for Job {
    fn drop(&mut self) {
        let _ = self.finish(true, false);
    }
}
fn output_pipe() -> Result<(File, File)> {
    let mut fds = [0; 2];
    if unsafe { libc::pipe(fds.as_mut_ptr()) } < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let read = unsafe { File::from_raw_fd(fds[0]) };
    let write = unsafe { File::from_raw_fd(fds[1]) };
    for file in [&read, &write] {
        if unsafe { libc::fcntl(file.as_raw_fd(), libc::F_SETFD, libc::FD_CLOEXEC) } < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
    }
    if unsafe { libc::fcntl(read.as_raw_fd(), libc::F_SETFL, libc::O_NONBLOCK) } < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok((read, write))
}
struct Supervisor {
    group: Arc<Mutex<Option<i32>>>,
    timed_out: Arc<AtomicBool>,
    deadline: Instant,
    exit: Arc<watch::Sender<bool>>,
    activity: Arc<watch::Sender<u64>>,
}
impl Supervisor {
    /// Whether the leader has exited; signals the owned group when it must stop.
    fn tick(&self, truncated: bool) -> Result<bool> {
        let group = self
            .group
            .lock()
            .map_err(|_| anyhow::anyhow!("process ownership lock failed"))?;
        let Some(pid) = *group else {
            return Ok(false);
        };
        let exited = leader_exited(pid as u32)?;
        let expired = Instant::now() >= self.deadline && !exited;
        if expired {
            self.timed_out.store(true, Ordering::Release);
        }
        if exited || expired || truncated {
            signal_owned_group(pid)?;
        }
        Ok(exited)
    }
    /// Announce exit once the output collected so far has reached the artifact.
    fn announce_exit(&self) {
        self.exit
            .send_if_modified(|done| !std::mem::replace(done, true));
    }
}
fn copy_output(
    read: &mut File,
    sink: &mut File,
    stop: &AtomicBool,
    truncated: &AtomicBool,
    supervisor: &Supervisor,
) -> std::io::Result<()> {
    let mut buffer = [0; 8192];
    let mut remaining = ARTIFACT_LIMIT;
    loop {
        let exited = supervisor
            .tick(truncated.load(Ordering::Acquire))
            .map_err(|_| std::io::Error::other("process supervision failed"))?;
        if copy_available(read, sink, &mut buffer, &mut remaining, truncated)? {
            supervisor.activity.send_modify(|count| *count += 1);
            continue;
        }
        if exited {
            supervisor.announce_exit();
        }
        if stop.load(Ordering::Acquire) {
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    sink.sync_all()
}
fn copy_available(
    read: &mut File,
    sink: &mut File,
    buffer: &mut [u8],
    remaining: &mut usize,
    truncated: &AtomicBool,
) -> std::io::Result<bool> {
    match read.read(buffer) {
        Ok(0) => Ok(false),
        Ok(n) => {
            let keep = n.min(*remaining);
            sink.write_all(&buffer[..keep])?;
            *remaining -= keep;
            if keep < n {
                truncated.store(true, Ordering::Release);
            }
            Ok(true)
        }
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(false),
        Err(e) if e.kind() == std::io::ErrorKind::Interrupted => Ok(true),
        Err(e) => Err(e),
    }
}
fn signal_owned_group(pid: i32) -> Result<()> {
    let rc = unsafe { libc::kill(-pid, libc::SIGKILL) };
    if rc < 0 && group_has_live(pid)? {
        bail!("process group cleanup failed");
    }
    Ok(())
}
fn task_env(value: Option<&Value>) -> Result<BTreeMap<String, String>> {
    let mut env: BTreeMap<_, _> = std::env::vars()
        .filter(|(k, _)| !control_variable(k))
        .collect();
    if let Some(value) = value {
        for (key, value) in value.as_object().context("env must be an object")? {
            let value = value
                .as_str()
                .context("environment values must be strings")?;
            if !control_variable(key) {
                env.insert(key.clone(), value.into());
            }
        }
    }
    Ok(env)
}
fn allowed(args: &HashMap<String, Value>, names: &[&str]) -> Result<()> {
    if args.keys().any(|key| !names.contains(&key.as_str())) {
        bail!("unknown compute arguments");
    }
    Ok(())
}
fn string<'a>(args: &'a HashMap<String, Value>, name: &str) -> Result<&'a str> {
    args.get(name)
        .and_then(Value::as_str)
        .context("required string argument missing")
}
fn integer(args: &HashMap<String, Value>, name: &str, default: u64) -> Result<u64> {
    args.get(name)
        .map(|v| v.as_u64().context("expected nonnegative integer"))
        .unwrap_or(Ok(default))
}
fn resolve(workspace: &Path, path: &str) -> PathBuf {
    workspace.join(path)
}
fn write_text(args: &HashMap<String, Value>, workspace: &Path) -> Result<Value> {
    allowed(args, &["path", "text"])?;
    let path = resolve(workspace, string(args, "path")?);
    let text = string(args, "text")?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .custom_flags(libc::O_NONBLOCK | libc::O_CLOEXEC)
        .open(path)?;
    if !file.metadata()?.is_file() {
        bail!("workspace.write_text requires a regular file");
    }
    file.set_len(0)?;
    file.write_all(text.as_bytes())?;
    file.sync_all()?;
    Ok(json!({"written":true}))
}

fn confirm_group_exit(group: i32) -> Result<()> {
    let until = Instant::now() + Duration::from_secs(2);
    while group_has_live(group)? {
        if Instant::now() >= until {
            bail!("cleanup_unconfirmed: process group did not exit");
        }
        // A concurrent fork can join the group after the first signal. The caller
        // still holds the ownership lock and unreaped leader, so repeat group
        // termination until absence is proven; never signal recovered numeric PIDs.
        signal_owned_group(group)?;
        std::thread::sleep(Duration::from_millis(2));
    }
    Ok(())
}
#[cfg(target_os = "macos")]
fn group_has_live(group: i32) -> Result<bool> {
    let mut pids = [0i32; 4096];
    let count = unsafe {
        libc::proc_listpgrppids(
            group,
            pids.as_mut_ptr().cast(),
            std::mem::size_of_val(&pids) as i32,
        )
    };
    if count < 0 || count as usize >= pids.len() {
        bail!("cannot enumerate owned process group");
    }
    for pid in &pids[..count as usize] {
        let mut info: libc::proc_bsdinfo = unsafe { std::mem::zeroed() };
        let n = unsafe {
            libc::proc_pidinfo(
                *pid,
                libc::PROC_PIDTBSDINFO,
                0,
                (&mut info as *mut libc::proc_bsdinfo).cast(),
                std::mem::size_of_val(&info) as i32,
            )
        };
        if n == 0 {
            process_probe::<()>(|| Err(std::io::Error::last_os_error()))?;
            continue;
        }
        if n as usize != std::mem::size_of_val(&info) {
            bail!("cannot verify process state");
        }
        if info.pbi_status != libc::SZOMB {
            return Ok(true);
        }
    }
    Ok(false)
}
#[cfg(target_os = "linux")]
fn group_has_live(group: i32) -> Result<bool> {
    for entry in std::fs::read_dir("/proc")? {
        let entry = entry?;
        let Some(pid) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.parse::<i32>().ok())
        else {
            continue;
        };
        let observed = observed_group(pid)?;
        if observed != Some(group) {
            continue;
        }
        let Some(stat) = process_probe(|| std::fs::read_to_string(entry.path().join("stat")))?
        else {
            continue;
        };
        if live_group_stat(&stat, group)? {
            return Ok(true);
        }
    }
    Ok(false)
}
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn group_has_live(_: i32) -> Result<bool> {
    bail!("native process cleanup requires macOS or Linux")
}

fn read_page(
    artifact: &File,
    path: &Path,
    args: &HashMap<String, Value>,
    mut state: Value,
    limit: u64,
) -> Result<Value> {
    let size = artifact.metadata()?.len();
    let offset = integer(args, "offset", size.saturating_sub(limit))?;
    let mut data = vec![0; limit as usize];
    let count = artifact.read_at(&mut data, offset)?;
    data.truncate(count);
    let next = offset.saturating_add(data.len() as u64);
    for (key, value) in [
        ("output", json!(String::from_utf8_lossy(&data))),
        ("offset", json!(offset)),
        ("next_offset", json!(next)),
        ("total_bytes", json!(size)),
        ("omitted_before", json!(offset.min(size))),
        ("omitted_after", json!(size.saturating_sub(next))),
        ("artifact_path", json!(path)),
    ] {
        state[key] = value;
    }
    Ok(state)
}

fn leader_exited(pid: u32) -> Result<bool> {
    let mut info: libc::siginfo_t = unsafe { std::mem::zeroed() };
    let rc = unsafe {
        libc::waitid(
            libc::P_PID,
            pid,
            &mut info,
            libc::WEXITED | libc::WNOHANG | libc::WNOWAIT,
        )
    };
    if rc < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(unsafe { info.si_pid() } != 0)
}

/// An unavailable OS observation is absence only when the process vanished.
pub(crate) fn process_probe<T>(probe: impl FnOnce() -> std::io::Result<T>) -> Result<Option<T>> {
    match probe() {
        Ok(value) => Ok(Some(value)),
        Err(error) if matches!(error.raw_os_error(), Some(libc::ENOENT) | Some(libc::ESRCH)) => {
            Ok(None)
        }
        Err(error) => Err(error.into()),
    }
}
#[cfg(target_os = "linux")]
fn live_group_stat(stat: &str, group: i32) -> Result<bool> {
    let (_, tail) = stat
        .rsplit_once(") ")
        .context("invalid process state observation")?;
    let fields = tail.split_whitespace().collect::<Vec<_>>();
    if fields.len() < 3 {
        bail!("incomplete process state observation");
    }
    Ok(fields[2].parse::<i32>()? == group && fields[0] != "Z")
}

#[cfg(target_os = "linux")]
fn observed_group(pid: i32) -> Result<Option<i32>> {
    process_probe(|| {
        let pgid = unsafe { libc::getpgid(pid) };
        if pgid < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(pgid)
        }
    })
}

/// Exit of a running owned job: resolves at once when it already finished.
async fn exited(mut signal: watch::Receiver<bool>, until: Instant) {
    let exit = async move { signal.wait_for(|done| *done).await.is_ok() };
    let _ = tokio::time::timeout_at(until.into(), exit).await;
}

type Watcher = std::pin::Pin<Box<dyn std::future::Future<Output = Outcome> + Send>>;

/// One condition's result: `observed` when it fired, otherwise why not.
#[derive(Clone)]
struct Outcome {
    observed: Option<Value>,
    reason: Option<&'static str>,
}
impl Outcome {
    fn hit(value: Value) -> Self {
        Self {
            observed: Some(value),
            reason: None,
        }
    }
    fn reason(reason: &'static str) -> Self {
        Self {
            observed: None,
            reason: Some(reason),
        }
    }
    fn fired(&self) -> bool {
        self.observed.is_some()
    }
}

#[derive(Clone, Copy)]
enum PathState {
    Exists,
    Missing,
    Changed,
}

/// Canonical wait conditions; every field is required and none is extra.
enum Condition {
    Exit(String),
    Path { path: PathBuf, state: PathState },
    Log { handle: String, pattern: String },
    Port { host: String, port: u16 },
    Http { url: String, status: u16 },
}
impl Condition {
    fn parse(item: &Value, workspace: &Path) -> Result<Self> {
        let object = item.as_object().context("invalid wait condition")?;
        let kind = object.get("kind").and_then(Value::as_str).unwrap_or("");
        let fields: &[&str] = match kind {
            "exit" => &["kind", "handle_id"],
            "path" => &["kind", "path", "state"],
            "log" => &["kind", "handle_id", "pattern"],
            "port" => &["kind", "port", "host"],
            "http" => &["kind", "url", "status"],
            _ => bail!("invalid wait condition"),
        };
        if object.len() != fields.len() || !fields.iter().all(|f| object.contains_key(*f)) {
            bail!("invalid wait condition");
        }
        Self::build(kind, item, workspace)
    }
    fn build(kind: &str, item: &Value, workspace: &Path) -> Result<Self> {
        match kind {
            "exit" => Ok(Self::Exit(handle(&item["handle_id"])?)),
            "path" => Self::path(item, workspace),
            "log" => Self::log(item),
            "port" => Self::port(item),
            _ => Self::http(item),
        }
    }
    fn path(item: &Value, workspace: &Path) -> Result<Self> {
        Ok(Self::Path {
            path: resolve(workspace, text(&item["path"])?),
            state: path_state(&item["state"])?,
        })
    }
    fn log(item: &Value) -> Result<Self> {
        Ok(Self::Log {
            handle: handle(&item["handle_id"])?,
            pattern: text(&item["pattern"])?.to_owned(),
        })
    }
    fn port(item: &Value) -> Result<Self> {
        Ok(Self::Port {
            host: text(&item["host"])?.to_owned(),
            port: bounded(&item["port"], 1, 65535)?,
        })
    }
    fn http(item: &Value) -> Result<Self> {
        Ok(Self::Http {
            url: http_url(&item["url"])?,
            status: bounded(&item["status"], 100, 599)?,
        })
    }
}
fn text(value: &Value) -> Result<&str> {
    value
        .as_str()
        .filter(|s| !s.is_empty() && !s.contains('\0'))
        .context("invalid wait condition")
}
fn handle(value: &Value) -> Result<String> {
    let id = text(value)?;
    if id.len() != 32 || !id.bytes().all(|c| matches!(c, b'0'..=b'9' | b'a'..=b'f')) {
        bail!("invalid wait condition");
    }
    Ok(id.to_owned())
}
fn path_state(value: &Value) -> Result<PathState> {
    Ok(match value.as_str() {
        Some("exists") => PathState::Exists,
        Some("missing") => PathState::Missing,
        Some("changed") => PathState::Changed,
        _ => bail!("invalid wait condition"),
    })
}
fn bounded(value: &Value, low: u64, high: u64) -> Result<u16> {
    let n = value
        .as_u64()
        .filter(|n| (low..=high).contains(n))
        .context("invalid wait condition")?;
    Ok(n as u16)
}
fn http_url(value: &Value) -> Result<String> {
    let url = text(value)?;
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        bail!("invalid wait condition");
    }
    Ok(url.to_owned())
}
/// The host always names the candidates; this executor never guesses them.
fn handle_ids(value: Option<&Value>) -> Result<Vec<String>> {
    let mut ids: Vec<String> = Vec::new();
    for item in value
        .and_then(Value::as_array)
        .context("handle_ids must be a list of process ids")?
    {
        let id = item
            .as_str()
            .filter(|s| !s.is_empty())
            .context("handle_ids must be a list of process ids")?;
        if !ids.iter().any(|seen| seen == id) {
            ids.push(id.to_owned());
        }
    }
    Ok(ids)
}

/// Run watchers until one fires, none remain, or the deadline. Every watcher
/// that finished at the same instant is kept, so list order breaks ties.
async fn watch_until_fired(watchers: Vec<Watcher>, until: Instant) -> (Vec<Option<Outcome>>, bool) {
    use futures_util::{stream::FuturesUnordered, FutureExt, StreamExt};
    let mut outcomes = vec![None; watchers.len()];
    let mut pending = watchers
        .into_iter()
        .enumerate()
        .map(|(i, watcher)| watcher.map(move |outcome| (i, outcome)))
        .collect::<FuturesUnordered<_>>();
    let deadline = tokio::time::sleep_until(until.into());
    tokio::pin!(deadline);
    loop {
        let next = tokio::select! {
            next = pending.next() => next,
            _ = &mut deadline => break,
        };
        let Some((i, outcome)) = next else { break };
        let mut fired = outcome.fired();
        outcomes[i] = Some(outcome);
        while let Some(Some((j, outcome))) = pending.next().now_or_never() {
            fired |= outcome.fired();
            outcomes[j] = Some(outcome);
        }
        if fired {
            break;
        }
    }
    (outcomes, Instant::now() >= until)
}
fn wait_status(entries: &[Value]) -> &'static str {
    if entries.iter().any(|e| e["fired"] == true) {
        return "fired";
    }
    let settled = |e: &Value| {
        matches!(
            e["reason"].as_str(),
            Some("exited" | "error" | "unsupported")
        )
    };
    if entries.iter().all(settled) {
        "unfired"
    } else {
        "timeout"
    }
}

fn path_stat(path: &Path) -> Option<(i64, i64, u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    let m = std::fs::metadata(path).ok()?;
    Some((m.mtime(), m.mtime_nsec(), m.size(), m.ino()))
}
async fn watch_path(path: PathBuf, state: PathState) -> Outcome {
    let initial = path_stat(&path);
    loop {
        let now = path_stat(&path);
        let done = match state {
            PathState::Exists => now.is_some(),
            PathState::Missing => now.is_none(),
            PathState::Changed => now != initial,
        };
        if done {
            return Outcome::hit(json!({}));
        }
        tokio::time::sleep(RECHECK).await;
    }
}
async fn watch_port(host: String, port: u16) -> Outcome {
    loop {
        let attempt = tokio::net::TcpStream::connect((host.as_str(), port));
        if let Ok(Ok(_)) = tokio::time::timeout(RECHECK.max(Duration::from_secs(1)), attempt).await
        {
            return Outcome::hit(json!({}));
        }
        tokio::time::sleep(RECHECK).await;
    }
}
/// A fresh client with no executor credential: the URL is chosen by the task.
async fn watch_http(url: String, status: u16, until: Instant) -> Outcome {
    let Ok(client) = reqwest::Client::builder().build() else {
        return Outcome::reason("error");
    };
    loop {
        let remaining = until.saturating_duration_since(Instant::now());
        let attempt = remaining.clamp(Duration::from_millis(50), Duration::from_secs(5));
        let response = client.get(&url).timeout(attempt).send().await;
        if response.is_ok_and(|r| r.status().as_u16() == status) {
            return Outcome::hit(json!({"status": status}));
        }
        tokio::time::sleep(RECHECK).await;
    }
}

/// Regular-expression search of only the bytes appended since the last scan, keeping
/// a short overlap so a match may span two reads.
struct LogScanner {
    artifact: File,
    offset: u64,
    carry: Vec<u8>,
    pattern: regex::bytes::Regex,
    matched: Option<String>,
}
impl LogScanner {
    fn new(artifact: File, pattern: regex::bytes::Regex) -> Self {
        Self {
            artifact,
            offset: 0,
            carry: Vec::new(),
            pattern,
            matched: None,
        }
    }
    fn scan(&mut self) -> std::io::Result<bool> {
        let mut chunk = vec![0; 64 * 1024];
        loop {
            let count = self.artifact.read_at(&mut chunk, self.offset)?;
            if count == 0 {
                return Ok(false);
            }
            self.offset += count as u64;
            let mut text = std::mem::take(&mut self.carry);
            text.extend_from_slice(&chunk[..count]);
            if let Some(found) = self.pattern.find(&text) {
                let found = String::from_utf8_lossy(found.as_bytes());
                self.matched = Some(found.chars().take(MATCH_CHARS).collect());
                return Ok(true);
            }
            self.carry = text.split_off(text.len() - text.len().min(CARRY_BYTES));
        }
    }
    fn observed(&self) -> Value {
        json!({"match": self.matched})
    }
}
/// Wakes on new output and on exit; the exit flag is sampled before the final scan.
async fn watch_log(
    mut scanner: LogScanner,
    signals: Option<(watch::Receiver<bool>, watch::Receiver<u64>)>,
) -> Outcome {
    let Some((mut exit, mut activity)) = signals else {
        return log_result(&mut scanner, true).unwrap_or(Outcome::reason("exited"));
    };
    loop {
        let finished = *exit.borrow_and_update();
        activity.mark_unchanged();
        if let Some(outcome) = log_result(&mut scanner, finished) {
            return outcome;
        }
        tokio::select! {
            _ = activity.changed() => {},
            _ = exit.changed() => {},
            _ = tokio::time::sleep(RECHECK) => {},
        }
    }
}
/// None while the pattern may still appear.
fn log_result(scanner: &mut LogScanner, finished: bool) -> Option<Outcome> {
    match scanner.scan() {
        Ok(true) => Some(Outcome::hit(scanner.observed())),
        Ok(false) if finished => Some(Outcome::reason("exited")),
        Ok(false) => None,
        Err(_) => Some(Outcome::reason("error")),
    }
}
