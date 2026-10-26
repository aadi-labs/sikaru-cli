//! Git fixtures and a local smart-HTTP remote for checkpoint tests.
#![allow(dead_code)]
use std::{
    collections::BTreeMap,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

/// The `ignore_defaults` a checkpoint remote advertises: build and dependency output, then credentials.
pub const IGNORE_DEFAULTS: &[&str] = &[
    ".*",
    "!.gitignore",
    "!.gitattributes",
    "node_modules/",
    "venv/",
    "__pycache__/",
    "*.pyc",
    "dist/",
    "build/",
    "target/",
    "out/",
    "release/",
    "coverage/",
    "*.egg-info/",
    "*.whl",
    "*.tar.gz",
    "*.zip",
    "*.jar",
    "*.so",
    "*.dylib",
    "*.o",
    "*.class",
    ".env",
    ".env.*",
    "*.pem",
    "*.key",
    "*.p12",
    "*.pfx",
    "*.jks",
    "*.keystore",
    "*.tfvars",
    "*.tfstate",
    "*.tfstate.*",
    "id_rsa",
    "id_dsa",
    "id_ecdsa",
    "id_ed25519",
    ".netrc",
    ".pgpass",
    ".git-credentials",
    ".pypirc",
    ".ssh/",
    ".aws/",
    ".gnupg/",
    ".docker/config.json",
    ".kube/config",
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
    std::fs::write(ws.join(".env"), "TOKEN=secret").unwrap();
    std::fs::write(ws.join(".env.example"), "TOKEN=").unwrap();
    std::fs::write(ws.join("deploy.pem"), "-----BEGIN PRIVATE KEY-----").unwrap();
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

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::sync::Mutex;
use std::time::Duration;
use wiremock::ResponseTemplate;

pub const BRANCH: &str = "sessions/session-test";
#[derive(Default)]
pub struct Seen {
    pub uploads_started: usize,
    pub objects: BTreeMap<String, Vec<u8>>,
    pub recorded: Option<String>,
}
pub struct WorkspaceUploads {
    pub seen: Mutex<Seen>,
    pub delay: Option<Duration>,
    pub mismatch: bool,
}
impl WorkspaceUploads {
    pub fn new(delay: Option<Duration>, mismatch: bool) -> Self {
        Self {
            seen: Mutex::new(Seen::default()),
            delay,
            mismatch,
        }
    }
    pub fn upload(&self, bytes: &[u8]) -> ResponseTemplate {
        let digest = format!("{:x}", Sha256::digest(bytes));
        let mut seen = self.seen.lock().unwrap();
        seen.uploads_started += 1;
        seen.objects.insert(digest.clone(), bytes.to_vec());
        let returned = if self.mismatch {
            "0".repeat(64)
        } else {
            digest
        };
        let response =
            ResponseTemplate::new(200).set_body_json(json!({"sha256":returned,"size":bytes.len()}));
        match self.delay {
            Some(delay) => response.set_delay(delay),
            None => response,
        }
    }
    pub fn record(&self, digest: &str) {
        let mut seen = self.seen.lock().unwrap();
        let manifest: Value = serde_json::from_slice(&seen.objects[digest]).unwrap();
        for entry in manifest["files"].as_object().unwrap().values() {
            if let Some(sha) = entry["sha256"].as_str() {
                assert_eq!(
                    seen.objects[sha].len() as u64,
                    entry["size"].as_u64().unwrap()
                );
            }
        }
        seen.recorded = Some(digest.to_owned());
    }
    pub fn head(&self) -> Option<String> {
        self.seen.lock().unwrap().recorded.clone()
    }
    pub fn tree(&self, digest: &str) -> Vec<(String, String)> {
        let seen = self.seen.lock().unwrap();
        let manifest: Value = serde_json::from_slice(&seen.objects[digest]).unwrap();
        manifest["files"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(p, e)| {
                let mode = if e.get("link").is_some() {
                    "120000"
                } else if e["mode"] == 493 {
                    "100755"
                } else {
                    "100644"
                };
                (mode.to_owned(), p.clone())
            })
            .collect()
    }
    pub fn read(&self, digest: &str, path: &str) -> Vec<u8> {
        let seen = self.seen.lock().unwrap();
        let manifest: Value = serde_json::from_slice(&seen.objects[digest]).unwrap();
        seen.objects[manifest["files"][path]["sha256"].as_str().unwrap()].clone()
    }
}
