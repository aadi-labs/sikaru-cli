//! Runs the built `sikaru` against a mock gateway, with JSON output and a throwaway home.
#![allow(dead_code)]
use std::collections::BTreeSet;
use std::process::{Command, Output};
use wiremock::MockServer;

pub async fn sikaru(server: &MockServer, args: &[&str]) -> (Output, tempfile::TempDir) {
    let home = tempfile::tempdir().unwrap();
    let mut args: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
    args.extend([
        "--base-url".into(),
        server.uri(),
        "--format".into(),
        "json".into(),
    ]);
    let (dir, home_path) = (home.path().to_owned(), home.path().to_owned());
    let output = tokio::task::spawn_blocking(move || {
        Command::new(env!("CARGO_BIN_EXE_sikaru"))
            .args(args)
            .current_dir(dir)
            .env("HOME", home_path)
            .env_remove("SIKARU_PROJECT")
            .env_remove("SIKARU_AGENT")
            .env("SIKARU_API_KEY", "test-only")
            .output()
            .unwrap()
    })
    .await
    .unwrap();
    (output, home)
}

pub fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

/// Command names listed in `<args> --help`'s `Commands:` section.
pub fn listed_commands(args: &[&str]) -> BTreeSet<String> {
    let output = Command::new(env!("CARGO_BIN_EXE_sikaru"))
        .args(args)
        .arg("--help")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{args:?}: {}",
        text(&output.stderr)
    );
    text(&output.stdout)
        .lines()
        .skip_while(|line| !line.starts_with("Commands:"))
        .skip(1)
        .take_while(|line| line.starts_with("  "))
        .filter_map(|line| line.split_whitespace().next())
        .filter(|name| *name != "help")
        .map(str::to_owned)
        .collect()
}

pub async fn request_bodies(server: &MockServer, method: &str) -> Vec<serde_json::Value> {
    server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .filter(|request| request.method.as_str() == method)
        .map(|request| serde_json::from_slice(&request.body).unwrap())
        .collect()
}
