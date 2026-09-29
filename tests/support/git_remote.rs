//! Git fixtures and a local smart-HTTP remote for checkpoint tests.
#![allow(dead_code)]
use std::{
    collections::BTreeMap,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub const IGNORE_DEFAULTS: [&str; 21] = [
    "node_modules/",
    ".git/",
    ".venv/",
    "venv/",
    "__pycache__/",
    "*.pyc",
    ".mypy_cache/",
    ".pytest_cache/",
    ".ruff_cache/",
    ".tox/",
    ".nox/",
    "dist/",
    "build/",
    "target/",
    ".next/",
    ".nuxt/",
    ".turbo/",
    ".gradle/",
    "coverage/",
    ".coverage",
    "*.egg-info/",
];

pub fn ignore_defaults() -> Vec<String> {
    IGNORE_DEFAULTS.iter().map(|s| s.to_string()).collect()
}

/// Run git with no user or system configuration; tests need a `git` binary.
pub fn git(dir: &Path, args: &[&str]) -> Vec<u8> {
    git_with_input(dir, args, None)
}
pub fn git_with_input(dir: &Path, args: &[&str], input: Option<&[u8]>) -> Vec<u8> {
    let mut child = Command::new("git")
        .args(args)
        .current_dir(dir)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_AUTHOR_NAME", "fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@example.com")
        .env("GIT_COMMITTER_NAME", "fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@example.com")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("checkpoint tests require git");
    let mut stdin = child.stdin.take().unwrap();
    if let Some(input) = input {
        stdin.write_all(input).unwrap();
    }
    drop(stdin);
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}
pub fn text(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap().trim().to_owned()
}

/// Every file (and link target) under `dir`, by relative path.
pub fn snapshot(dir: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn walk(root: &Path, dir: &Path, out: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            let kind = entry.file_type().unwrap();
            let relative = path.strip_prefix(root).unwrap().to_owned();
            if kind.is_symlink() {
                out.insert(
                    relative,
                    std::fs::read_link(&path)
                        .unwrap()
                        .into_os_string()
                        .into_encoded_bytes(),
                );
            } else if kind.is_dir() {
                walk(root, &path, out);
            } else {
                out.insert(relative, std::fs::read(&path).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(dir, dir, &mut out);
    out
}
/// `git status` that never refreshes or rewrites the index.
pub fn porcelain(dir: &Path) -> String {
    String::from_utf8(git(
        dir,
        &[
            "--no-optional-locks",
            "status",
            "--porcelain=v1",
            "--untracked-files=all",
        ],
    ))
    .unwrap()
}
/// Deterministic incompressible bytes.
pub fn noise(len: usize, seed: u8) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let mut out = Vec::with_capacity(len + 32);
    let mut block = Sha256::digest([seed]).to_vec();
    while out.len() < len {
        block = Sha256::digest(&block).to_vec();
        out.extend_from_slice(&block);
    }
    out.truncate(len);
    out
}
pub fn executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).unwrap();
}
/// A task repository with history, a dirty index, ignored files, links and a nested repository.
pub fn task_repository(ws: &Path) {
    git(ws, &["init", "-q", "-b", "main"]);
    std::fs::write(ws.join(".gitignore"), "*.log\n").unwrap();
    std::fs::write(ws.join("tracked.txt"), "committed").unwrap();
    std::fs::create_dir_all(ws.join("node_modules/left-pad")).unwrap();
    std::fs::write(
        ws.join("node_modules/left-pad/index.js"),
        "module.exports = 1",
    )
    .unwrap();
    git(ws, &["add", ".gitignore", "tracked.txt"]);
    git(ws, &["add", "-f", "node_modules/left-pad/index.js"]);
    git(ws, &["commit", "-qm", "base"]);
    std::fs::write(ws.join("tracked.txt"), "working copy").unwrap();
    std::fs::write(ws.join("staged.txt"), "staged").unwrap();
    git(ws, &["add", "staged.txt"]);
    std::fs::write(ws.join("app.log"), "ignored").unwrap();
    std::fs::create_dir_all(ws.join(".venv/lib")).unwrap();
    std::fs::write(ws.join(".venv/lib/site.py"), "environment").unwrap();
    std::fs::write(ws.join("run.sh"), "#!/bin/sh\n").unwrap();
    executable(&ws.join("run.sh"));
    std::os::unix::fs::symlink("tracked.txt", ws.join("alias")).unwrap();
    std::os::unix::fs::symlink("../outside", ws.join("escape")).unwrap();
    std::os::unix::fs::symlink("/etc/hosts", ws.join("absolute")).unwrap();
    std::fs::create_dir_all(ws.join("vendor/lib")).unwrap();
    std::fs::write(ws.join("vendor/lib/lib.txt"), "nested").unwrap();
    git(&ws.join("vendor/lib"), &["init", "-q"]);
    git(&ws.join("vendor/lib"), &["add", "lib.txt"]);
    git(&ws.join("vendor/lib"), &["commit", "-qm", "nested"]);
}
/// Parse the JSON lines of a process's stderr.
pub fn json_events(stderr: &[u8]) -> Vec<serde_json::Value> {
    String::from_utf8_lossy(stderr)
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

use base64::Engine;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

pub const BRANCH: &str = "sessions/session-test";
pub const REF: &str = "refs/heads/sessions/session-test";
pub const ZERO: &str = "0000000000000000000000000000000000000000";
const REPO: &str = "remote/workspace.git";

#[derive(Clone, Copy, Default)]
pub struct Faults {
    /// Receive-pack refuses `token-1` as expired.
    pub refuse_first_token: bool,
    /// Another writer moves the branch just before the first push arrives.
    pub race_first_push: bool,
    /// Delay each receive-pack response (the push is applied first).
    pub push_delay: Option<Duration>,
}
#[derive(Clone, Debug)]
pub struct Push {
    pub old: String,
    pub new: String,
    pub token: String,
    pub body_bytes: usize,
    /// Capabilities requested after the NUL of the command line.
    pub capabilities: String,
    pub accepted: bool,
}
#[derive(Default)]
pub struct Seen {
    pub pushes: Vec<Push>,
    pub pushes_started: usize,
    pub unauthorized: usize,
}
/// A local smart-HTTP remote: `git http-backend` behind Basic auth, with one-ref
/// compare-and-swap and a parent check on each push, like the service's ref policy.
pub struct GitRemote {
    pub server: MockServer,
    pub seen: Arc<Mutex<Seen>>,
    root: tempfile::TempDir,
}
impl GitRemote {
    pub async fn start(faults: Faults) -> Self {
        let root = tempfile::tempdir().unwrap();
        let repo = root.path().join(REPO);
        std::fs::create_dir_all(&repo).unwrap();
        git(&repo, &["init", "-q", "--bare", "."]);
        for (key, value) in [
            ("http.receivepack", "true"),
            ("receive.denyDeletes", "true"),
            ("receive.denyNonFastForwards", "true"),
        ] {
            git(&repo, &["config", key, value]);
        }
        let seen = Arc::new(Mutex::new(Seen::default()));
        let server = MockServer::start().await;
        Mock::given(wiremock::matchers::any())
            .respond_with(Backend {
                root: root.path().to_owned(),
                seen: seen.clone(),
                faults,
            })
            .mount(&server)
            .await;
        Self { server, seen, root }
    }
    pub fn url(&self) -> String {
        format!("{}/{REPO}", self.server.uri())
    }
    pub fn repo(&self) -> PathBuf {
        self.root.path().join(REPO)
    }
    pub fn head(&self) -> Option<String> {
        head(&self.repo())
    }
    /// (mode, path) of every blob at `rev`.
    pub fn tree(&self, rev: &str) -> Vec<(String, String)> {
        String::from_utf8(git(&self.repo(), &["ls-tree", "-r", "--full-tree", rev]))
            .unwrap()
            .lines()
            .map(|l| {
                let (meta, path) = l.split_once('\t').unwrap();
                (meta.split(' ').next().unwrap().to_owned(), path.to_owned())
            })
            .collect()
    }
    pub fn read(&self, rev: &str, path: &str) -> Vec<u8> {
        git(
            &self.repo(),
            &["cat-file", "blob", &format!("{rev}:{path}")],
        )
    }
    pub fn parents(&self, rev: &str) -> Vec<String> {
        text(git(
            &self.repo(),
            &["rev-list", "--parents", "-n", "1", rev],
        ))
        .split(' ')
        .skip(1)
        .map(str::to_owned)
        .collect()
    }
    pub fn reachable(&self) -> Vec<String> {
        String::from_utf8(git(&self.repo(), &["rev-list", "--all"]))
            .unwrap()
            .lines()
            .map(str::to_owned)
            .collect()
    }
}
struct Backend {
    root: PathBuf,
    seen: Arc<Mutex<Seen>>,
    faults: Faults,
}
fn challenge() -> ResponseTemplate {
    ResponseTemplate::new(401).insert_header("WWW-Authenticate", "Basic realm=\"workspace\"")
}
impl Respond for Backend {
    fn respond(&self, request: &Request) -> ResponseTemplate {
        let Some(token) = token(request) else {
            self.seen.lock().unwrap().unauthorized += 1;
            return challenge();
        };
        if request.url.path().ends_with("/git-receive-pack") {
            return self.receive(request, &token);
        }
        cgi(&self.root, request)
    }
}
impl Backend {
    fn receive(&self, request: &Request, token: &str) -> ResponseTemplate {
        let repo = self.root.join(REPO);
        if self.faults.refuse_first_token && token == "token-1" {
            self.seen.lock().unwrap().unauthorized += 1;
            return challenge();
        }
        if request.body.as_slice() == b"0000" {
            // git's probe POST: an empty 200, as the service answers it.
            return ResponseTemplate::new(200);
        }
        let (old, new, refname, capabilities) = first_command(&request.body);
        let first = {
            let mut s = self.seen.lock().unwrap();
            s.pushes_started += 1;
            s.pushes_started == 1
        };
        if first && self.faults.race_first_push {
            advance(&repo);
        }
        let current = head(&repo).unwrap_or_else(|| ZERO.to_owned());
        let mut push = Push {
            old: old.clone(),
            new: new.clone(),
            token: token.to_owned(),
            body_bytes: request.body.len(),
            capabilities,
            accepted: false,
        };
        let response = if refname != REF || old != current {
            report(&refname, false)
        } else {
            self.apply_push(request, &repo, &mut push)
        };
        self.seen.lock().unwrap().pushes.push(push);
        match self.faults.push_delay {
            Some(delay) => response.set_delay(delay),
            None => response,
        }
    }
    fn apply_push(&self, request: &Request, repo: &Path, push: &mut Push) -> ResponseTemplate {
        let response = cgi(&self.root, request);
        if head(repo).as_deref() != Some(push.new.as_str()) {
            return response;
        }
        push.accepted = legal_parent(repo, &push.old, &push.new);
        if push.accepted {
            return response;
        }
        git(repo, &["update-ref", REF, &push.old]);
        report(REF, false)
    }
}
fn legal_parent(repo: &Path, old: &str, new: &str) -> bool {
    old == ZERO
        || (parents(repo, new) == vec![old.to_owned()]
            && text(git(
                repo,
                &["rev-list", "--count", &format!("{old}..{new}")],
            )) == "1")
}
fn token(request: &Request) -> Option<String> {
    let header = request.headers.get("authorization")?.to_str().ok()?;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(header.strip_prefix("Basic ")?)
        .ok()?;
    let (user, token) = String::from_utf8(decoded)
        .ok()?
        .split_once(':')
        .map(|(u, t)| (u.to_owned(), t.to_owned()))?;
    (user == "x-token" && token.starts_with("token-")).then_some(token)
}
fn first_command(body: &[u8]) -> (String, String, String, String) {
    let size = usize::from_str_radix(std::str::from_utf8(&body[..4]).unwrap(), 16).unwrap();
    let line = String::from_utf8_lossy(&body[4..size]).into_owned();
    let (command, capabilities) = line.split_once('\0').unwrap_or((line.as_str(), ""));
    let mut parts = command.trim_end().split(' ');
    let (old, new, refname) = (
        parts.next().unwrap().into(),
        parts.next().unwrap().into(),
        parts.next().unwrap().into(),
    );
    (old, new, refname, capabilities.trim_end().to_owned())
}
fn pkt(data: &str) -> Vec<u8> {
    let mut out = format!("{:04x}", data.len() + 4).into_bytes();
    out.extend_from_slice(data.as_bytes());
    out
}
fn report(refname: &str, ok: bool) -> ResponseTemplate {
    let mut body = pkt("unpack ok\n");
    body.extend(pkt(&if ok {
        format!("ok {refname}\n")
    } else {
        format!("ng {refname} stale\n")
    }));
    body.extend_from_slice(b"0000");
    ResponseTemplate::new(200)
        .insert_header("Content-Type", "application/x-git-receive-pack-result")
        .set_body_bytes(body)
}
fn head(repo: &Path) -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--verify", "-q", REF])
        .current_dir(repo)
        .output()
        .ok()?;
    output.status.success().then(|| text(output.stdout))
}
fn parents(repo: &Path, rev: &str) -> Vec<String> {
    text(git(repo, &["rev-list", "--parents", "-n", "1", rev]))
        .split(' ')
        .skip(1)
        .map(str::to_owned)
        .collect()
}
/// Another writer commits `other.txt` on the branch.
fn advance(repo: &Path) {
    let blob = text(git_with_input(
        repo,
        &["hash-object", "-w", "--stdin"],
        Some(b"foreign"),
    ));
    let tree = text(git_with_input(
        repo,
        &["mktree"],
        Some(format!("100644 blob {blob}\tother.txt\n").as_bytes()),
    ));
    let mut args = vec![
        "commit-tree".to_owned(),
        tree,
        "-m".into(),
        "foreign".into(),
    ];
    if let Some(parent) = head(repo) {
        args.extend(["-p".into(), parent]);
    }
    let commit = text(git(
        repo,
        &args.iter().map(String::as_str).collect::<Vec<_>>(),
    ));
    git(repo, &["update-ref", REF, &commit]);
}
fn cgi(root: &Path, request: &Request) -> ResponseTemplate {
    let content_type = request
        .headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_owned();
    let mut child = Command::new("git")
        .arg("http-backend")
        .env_clear()
        .env("PATH", std::env::var("PATH").unwrap_or_default())
        .env("HOME", root)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_PROJECT_ROOT", root)
        .env("GIT_HTTP_EXPORT_ALL", "1")
        .env("REMOTE_USER", "x-token")
        .env("REQUEST_METHOD", request.method.as_str())
        .env("PATH_INFO", request.url.path())
        .env("QUERY_STRING", request.url.query().unwrap_or(""))
        .env("CONTENT_TYPE", content_type)
        .env("CONTENT_LENGTH", request.body.len().to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("git http-backend");
    let mut stdin = child.stdin.take().unwrap();
    let body = request.body.clone();
    let writer = std::thread::spawn(move || stdin.write_all(&body));
    let output = child.wait_with_output().unwrap();
    writer.join().unwrap().unwrap();
    let raw = output.stdout;
    let (end, gap) = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .map(|i| (i, 4))
        .or_else(|| raw.windows(2).position(|w| w == b"\n\n").map(|i| (i, 2)))
        .expect("CGI headers");
    let mut status = 200;
    let mut headers = Vec::new();
    for line in String::from_utf8_lossy(&raw[..end]).lines() {
        let (name, value) = line.split_once(':').unwrap();
        if name.eq_ignore_ascii_case("status") {
            status = value.trim()[..3].parse().unwrap();
        } else {
            headers.push((name.trim().to_owned(), value.trim().to_owned()));
        }
    }
    let mut response = ResponseTemplate::new(status).set_body_bytes(raw[end + gap..].to_vec());
    for (name, value) in headers {
        response = response.insert_header(name.as_str(), value.as_str());
    }
    response
}
/// RFC 3339 UTC for a Unix time, for fake service responses.
pub fn rfc3339(unix: u64) -> String {
    let (days, rem) = ((unix / 86_400) as i64, unix % 86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        rem % 3600 / 60,
        rem % 60
    )
}
pub fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
