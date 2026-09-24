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
const ARTIFACT_LIMIT: usize = 1024 * 1024;
const PAGE_LIMIT: usize = 24 * 1024; // JSON escaping of every byte still fits the receipt limit.
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
        match method {
            "bash.run" => self.run(args, journal, operation_key).await,
            "workspace.write_text" => write_text(args, journal.binding.anchor.path()),
            "bash.start" => self.start(args, journal, operation_key),
            "bash.read" => self.read(args, journal),
            "bash.wait" => self.wait(args, journal).await,
            "bash.cancel" => self.cancel(args, journal),
            _ => bail!("unsupported compute operation"),
        }
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
            .context("unknown process handle")
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
        let seconds = args
            .get("timeout")
            .map(|v| v.as_f64().context("timeout must be numeric"))
            .transpose()?
            .unwrap_or(self.timeout.as_secs_f64());
        if !seconds.is_finite() || seconds < 0.0 {
            bail!("invalid wait timeout");
        }
        let until =
            Instant::now() + Duration::from_secs_f64(seconds.min(self.timeout.as_secs_f64()));
        loop {
            self.maintenance(journal)?;
            let state = self.state(id)?;
            if state["status"] != "running" || Instant::now() >= until {
                return Ok(state);
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
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
        let supervisor = Supervisor {
            group: group.clone(),
            timed_out: timed_out.clone(),
            deadline: Instant::now() + timeout,
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
}
impl Supervisor {
    fn tick(&self, truncated: bool) -> Result<()> {
        let group = self
            .group
            .lock()
            .map_err(|_| anyhow::anyhow!("process ownership lock failed"))?;
        let Some(pid) = *group else {
            return Ok(());
        };
        let exited = leader_exited(pid as u32)?;
        let expired = Instant::now() >= self.deadline && !exited;
        if expired {
            self.timed_out.store(true, Ordering::Release);
        }
        if exited || expired || truncated {
            signal_owned_group(pid)?;
        }
        Ok(())
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
        supervisor
            .tick(truncated.load(Ordering::Acquire))
            .map_err(|_| std::io::Error::other("process supervision failed"))?;
        if copy_available(read, sink, &mut buffer, &mut remaining, truncated)? {
            continue;
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
