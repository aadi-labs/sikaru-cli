//! Receive-pack over Basic auth against a local smart-HTTP remote.
#![cfg(unix)]
#![allow(dead_code)]
#[path = "../cli/sikaru/compute_git_http.rs"]
mod git_http;
#[path = "support/git_remote.rs"]
mod git_remote;
use git_http::{
    parse_advertisement, parse_report, pkt_line, pkt_lines, GitHttpError, PushOutcome, Remote,
};
use git_remote::{git, git_with_input, text, Faults, GitRemote, REF, ZERO};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

fn commit_pack(dir: &std::path::Path, file: &str, parent: Option<&str>) -> (String, Vec<u8>) {
    std::fs::write(dir.join(file), file).unwrap();
    git(dir, &["add", file]);
    git(dir, &["commit", "-qm", file]);
    let head = text(git(dir, &["rev-parse", "HEAD"]));
    let range = parent.map_or(head.clone(), |p| format!("{p}..{head}"));
    let objects = git(dir, &["rev-list", "--objects", &range]);
    (
        head,
        git_with_input(dir, &["pack-objects", "--stdout", "-q"], Some(&objects)),
    )
}

#[tokio::test]
async fn push_advances_the_branch_and_reports_progress() {
    let remote = GitRemote::start(Faults::default()).await;
    let work = tempfile::tempdir().unwrap();
    git(work.path(), &["init", "-q"]);
    let (first, pack) = commit_pack(work.path(), "a.txt", None);
    let http = reqwest::Client::new();
    let url = remote.url();
    let r = Remote {
        http: &http,
        url: &url,
        username: "x-token",
        token: "token-1",
        refname: REF,
    };
    assert_eq!(r.advertisement().await.unwrap().head, None);
    let sent = Arc::new(AtomicU64::new(0));
    let seen = sent.clone();
    let total = pack.len() as u64;
    let outcome = r
        .push(None, &first, pack.into(), move |done, _| {
            seen.store(done, Ordering::SeqCst);
        })
        .await
        .unwrap();
    assert_eq!(outcome, PushOutcome::Accepted);
    assert!(sent.load(Ordering::SeqCst) >= total);
    // The service rejects or strips side-band, atomic, push-options and signed pushes: none is requested.
    let capabilities = remote.seen.lock().unwrap().pushes[0].capabilities.clone();
    assert_eq!(capabilities, "report-status agent=sikaru");
    assert_eq!(remote.head().as_deref(), Some(first.as_str()));
    assert_eq!(
        r.advertisement().await.unwrap().head.as_deref(),
        Some(first.as_str())
    );
}

#[tokio::test]
async fn a_push_from_a_stale_head_is_refused_not_forced() {
    let remote = GitRemote::start(Faults::default()).await;
    let work = tempfile::tempdir().unwrap();
    git(work.path(), &["init", "-q"]);
    let (first, pack) = commit_pack(work.path(), "a.txt", None);
    let (second, pack2) = commit_pack(work.path(), "b.txt", Some(&first));
    let http = reqwest::Client::new();
    let url = remote.url();
    let r = Remote {
        http: &http,
        url: &url,
        username: "x-token",
        token: "token-1",
        refname: REF,
    };
    r.push(None, &first, pack.into(), |_, _| {}).await.unwrap();
    assert_eq!(
        r.push(None, &second, pack2.into(), |_, _| {})
            .await
            .unwrap(),
        PushOutcome::Refused
    );
    assert_eq!(remote.head().as_deref(), Some(first.as_str()));
}

#[tokio::test]
async fn wrong_credentials_are_unauthorized_without_echoing_the_token() {
    let remote = GitRemote::start(Faults::default()).await;
    let http = reqwest::Client::new();
    let url = remote.url();
    let r = Remote {
        http: &http,
        url: &url,
        username: "x-token",
        token: "not-a-token",
        refname: REF,
    };
    let error = r.advertisement().await.unwrap_err();
    assert!(matches!(
        error.downcast_ref::<GitHttpError>(),
        Some(GitHttpError::Unauthorized)
    ));
    assert!(!format!("{error:#}").contains("not-a-token"));
}

#[tokio::test]
async fn a_conflict_status_is_a_refusal() {
    let server = wiremock::MockServer::start().await;
    wiremock::Mock::given(wiremock::matchers::method("POST"))
        .respond_with(wiremock::ResponseTemplate::new(409))
        .mount(&server)
        .await;
    let http = reqwest::Client::new();
    let url = server.uri();
    let r = Remote {
        http: &http,
        url: &url,
        username: "x-token",
        token: "token-1",
        refname: REF,
    };
    let new = "1".repeat(40);
    assert_eq!(
        r.push(None, &new, bytes::Bytes::from_static(b"PACK"), |_, _| {})
            .await
            .unwrap(),
        PushOutcome::Refused
    );
}

#[test]
fn pkt_lines_round_trip_and_reject_truncation() {
    let mut stream = pkt_line(b"hello\n");
    stream.extend_from_slice(b"0000");
    assert_eq!(
        pkt_lines(&stream).unwrap(),
        vec![Some(&b"hello\n"[..]), None]
    );
    assert!(pkt_lines(b"00ffshort").is_err());
}

#[test]
fn advertisements_name_only_the_session_ref_and_its_haves() {
    let oid = "a".repeat(40);
    let other = "b".repeat(40);
    let mut body = pkt_line(b"# service=git-receive-pack\n");
    body.extend_from_slice(b"0000");
    body.extend(pkt_line(
        format!("{oid} {REF}\0report-status agent=x\n").as_bytes(),
    ));
    body.extend(pkt_line(format!("{other} .have\n").as_bytes()));
    body.extend_from_slice(b"0000");
    let advertised = parse_advertisement(&body, REF).unwrap();
    assert_eq!(advertised.head.as_deref(), Some(oid.as_str()));
    assert_eq!(advertised.haves, vec![other]);
    let mut unborn = pkt_line(format!("{ZERO} capabilities^{{}}\0report-status\n").as_bytes());
    unborn.extend_from_slice(b"0000");
    assert_eq!(parse_advertisement(&unborn, REF).unwrap().head, None);
}

#[test]
fn reports_accept_refuse_or_fail_closed() {
    let report = |lines: &[&str]| {
        let mut body = Vec::new();
        for line in lines {
            body.extend(pkt_line(line.as_bytes()));
        }
        body.extend_from_slice(b"0000");
        body
    };
    assert_eq!(
        parse_report(&report(&["unpack ok\n", &format!("ok {REF}\n")]), REF).unwrap(),
        PushOutcome::Accepted
    );
    assert_eq!(
        parse_report(&report(&["unpack ok\n", &format!("ng {REF} stale\n")]), REF).unwrap(),
        PushOutcome::Refused
    );
    assert!(parse_report(&report(&["unpack error\n", &format!("ok {REF}\n")]), REF).is_err());
    assert!(parse_report(&report(&["unpack ok\n"]), REF).is_err());
}

#[test]
fn malformed_advertisements_are_not_treated_as_an_unborn_branch() {
    for body in [
        Vec::new(),
        pkt_line(b"not a git advertisement\n"),
        pkt_line(format!("{} {REF}\n", "z".repeat(40)).as_bytes()),
        pkt_line(b"version 2\n"),
    ] {
        assert!(parse_advertisement(&body, REF).is_err(), "{body:?}");
    }
}

#[test]
fn conflicting_or_incomplete_reports_never_claim_success() {
    let valid = || {
        let mut body = pkt_line(b"unpack ok\n");
        body.extend(pkt_line(format!("ok {REF}\n").as_bytes()));
        body
    };
    assert!(parse_report(&valid(), REF).is_err(), "missing flush");
    let mut conflicting = valid();
    conflicting.extend(pkt_line(format!("ng {REF} denied\n").as_bytes()));
    conflicting.extend_from_slice(b"0000");
    assert!(
        parse_report(&conflicting, REF).is_err(),
        "contradictory status"
    );
    let mut trailing = valid();
    trailing.extend_from_slice(b"0000");
    trailing.extend(pkt_line(b"unpack error\n"));
    assert!(parse_report(&trailing, REF).is_err(), "status after flush");
}
