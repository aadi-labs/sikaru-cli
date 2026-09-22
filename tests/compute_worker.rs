#![allow(dead_code)]
use std::process::Command;
#[test]
fn primary_binary_has_customer_worker() {
    let out = Command::new(
        std::env::var("SIKARU_TEST_INSTALLED")
            .unwrap_or_else(|_| env!("CARGO_BIN_EXE_sikaru").to_owned()),
    )
    .args(["compute", "worker", "--help"])
    .output()
    .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stdout).contains("--launcher"));
}
#[cfg(unix)]
#[path = "../cli/sikaru/compute_config.rs"]
mod config;
#[cfg(unix)]
#[path = "../cli/sikaru/compute_launcher.rs"]
mod launcher;
#[cfg(unix)]
mod worker {
    use serde_json::{json, Value};
    use std::{
        os::unix::fs::PermissionsExt,
        sync::{Arc, Mutex},
        time::Duration,
    };
    use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};
    #[derive(Clone)]
    struct Oracle {
        startup_fail: bool,
        claims: Arc<Mutex<usize>>,
        teardowns: Arc<Mutex<usize>>,
    }
    fn attachment() -> Value {
        json!({"id":"attachment","session_id":"session","project_id":"project","environment_id":"environment","provider_id":"provider","workspace_generation":"generation","workspace_provenance":{"kind":"sandbox","identity":"customer-original"},"journal_id":"journal","status":"ready","owner_id":"worker","owner_epoch":1,"lease_until":2000000000.0,"startup_deadline":2000000000.0,"lease_ttl_seconds":60,"capabilities":["compute.execute"],"protocol_version":"sikaru-compute-v1","cleanup_status":"unconfirmed","uncertain_operations":[],"processes":[]})
    }
    impl Respond for Oracle {
        fn respond(&self, r: &Request) -> ResponseTemplate {
            assert_eq!(r.headers.get("authorization").unwrap(), "Bearer worker-key");
            assert!(r.headers.get("api_key").is_none());
            let path = r.url.path();
            let mut a = attachment();
            if self.startup_fail {
                a["status"] = json!("starting");
            }
            let value = if path.ends_with("/renew") {
                json!({"credential_id":"worker","expires_at":2000000000.0})
            } else if path.ends_with("/queue") {
                json!({"attachments":[a],"poll_after_seconds":1})
            } else if path.ends_with("/claim") {
                let mut claims = self.claims.lock().unwrap();
                *claims += 1;
                if *claims > 1 {
                    return ResponseTemplate::new(409);
                }
                json!({"id":"claim","attachment_id":"attachment","owner_id":"worker","owner_epoch":1,"status":"claimed","lease_until":2000000000.0,"startup_ttl_seconds":180})
            } else if path.ends_with("/credentials") {
                json!({"credential_id":"executor","token":"executor-key","expires_at":2000000000.0})
            } else if path.ends_with("/teardown") {
                let body: Value = serde_json::from_slice(&r.body).unwrap();
                assert_eq!(body["owner_epoch"], 1);
                assert_eq!(body["children_terminated"], true);
                *self.teardowns.lock().unwrap() += 1;
                let mut a = a;
                a["status"] = json!("cleaned");
                a["cleanup_status"] = json!("confirmed");
                a
            } else {
                a
            };
            ResponseTemplate::new(200).set_body_json(value)
        }
    }
    fn script(dir: &std::path::Path, mode: &str) -> std::path::PathBuf {
        let path = dir.join("launcher.py");
        let root = serde_json::to_string(dir.to_str().unwrap()).unwrap();
        std::fs::write(&path,format!(r#"#!/usr/bin/env python3
import json,sys,os,time
from pathlib import Path
root=Path({root})
v=json.load(sys.stdin)
assert not any(k.startswith('SIKARU_') for k in os.environ)
assert v['version']==1
handle={{'kind':'sandbox','id':'sandbox-immutable','proof':'creation-nonce'}}
if v['operation']=='launch':
    assert v['credential']['token']=='executor-key'
    if 'claim' in v:
        ledgers=[p.read_text() for p in root.rglob('workflow.jsonl')]
        assert any(json.loads(t.splitlines()[-1]).get('launch_intent') for t in ledgers)
        assert all('executor-key' not in t and 'worker-key' not in t for t in ledgers)
    with (root/'effects').open('a') as f: f.write('effect\n'); f.flush(); os.fsync(f.fileno())
    (root/'launch-id').write_text(v['launch_id'])
    if '{mode}'=='lost': sys.exit(1)
    if '{mode}'=='timeout': time.sleep(30)
if v['operation']!='launch': assert v['launch_id']==(root/'launch-id').read_text()
print(json.dumps({{'version':1,'launch_id':v['launch_id'],'status':'not_launched' if '{mode}'=='contradict' and v['operation']=='teardown' else 'unknown' if '{mode}'=='unconfirmed' and v['operation']=='teardown' else 'running' if '{mode}'=='hold' and v['operation']=='status' else 'launched' if v['operation']=='launch' else 'terminated','handle':None if '{mode}'=='contradict' and v['operation']=='teardown' else handle,'evidence':'sandbox deleted and absence verified'}}))
"#)).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700)).unwrap();
        path
    }
    async fn run(
        server: &MockServer,
        state: &std::path::Path,
        launcher: &std::path::Path,
    ) -> (i32, Value) {
        let child = spawn_worker(server, state, launcher).await;
        let out = tokio::time::timeout(Duration::from_secs(30), child.wait_with_output())
            .await
            .expect("worker hung")
            .unwrap();
        let value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
            panic!(
                "{e}: {} {}",
                String::from_utf8_lossy(&out.stdout),
                String::from_utf8_lossy(&out.stderr)
            )
        });
        (out.status.code().unwrap(), value)
    }
    async fn spawn_worker(
        server: &MockServer,
        state: &std::path::Path,
        launcher: &std::path::Path,
    ) -> tokio::process::Child {
        spawn_worker_mode(server, state, launcher, true).await
    }
    async fn spawn_worker_mode(
        server: &MockServer,
        state: &std::path::Path,
        launcher: &std::path::Path,
        once: bool,
    ) -> tokio::process::Child {
        use tokio::io::AsyncWriteExt;
        let mut child = tokio::process::Command::new(
            std::env::var("SIKARU_TEST_INSTALLED")
                .unwrap_or_else(|_| env!("CARGO_BIN_EXE_sikaru").to_owned()),
        )
        .env("SIKARU_API_KEY", "must-not-leak-controller")
        .env("SIKARU_EXECUTOR_TOKEN", "must-not-leak-executor")
        .args([
            "--base-url",
            &server.uri(),
            "compute",
            "worker",
            "--bootstrap",
            "-",
            "--launcher",
            launcher.to_str().unwrap(),
            "--concurrency",
            "2",
        ])
        .args(if once { vec!["--once"] } else { vec![] })
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
        child.stdin.take().unwrap().write_all(json!({"project_id":"project","environment_id":"environment","token":"worker-key","state_dir":state}).to_string().as_bytes()).await.unwrap();
        child
    }
    #[derive(Clone)]
    struct FaultOracle {
        oracle: Oracle,
        queue: Arc<Mutex<usize>>,
        claim_failure: bool,
        renewals: Arc<Mutex<usize>>,
    }
    impl Respond for FaultOracle {
        fn respond(&self, r: &Request) -> ResponseTemplate {
            if r.url.path().ends_with("/renew") {
                let mut count = self.renewals.lock().unwrap();
                *count += 1;
                if *count == 2 {
                    return ResponseTemplate::new(503);
                }
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64();
                return ResponseTemplate::new(200)
                    .set_body_json(json!({"credential_id":"worker","expires_at":now+6.0}));
            }
            if self.claim_failure && r.url.path().ends_with("/claim") {
                return ResponseTemplate::new(503);
            }
            if r.url.path().ends_with("/queue") {
                let mut count = self.queue.lock().unwrap();
                *count += 1;
                if *count == 2 {
                    return ResponseTemplate::new(503);
                }
            }
            self.oracle.respond(r)
        }
    }
    fn fault_oracle(claim_failure: bool) -> FaultOracle {
        FaultOracle {
            oracle: Oracle {
                startup_fail: false,
                claims: Arc::new(Mutex::new(0)),
                teardowns: Arc::new(Mutex::new(0)),
            },
            queue: Arc::new(Mutex::new(0)),
            claim_failure,
            renewals: Arc::new(Mutex::new(0)),
        }
    }
    #[derive(Clone)]
    struct ExpiringOracle {
        inner: Oracle,
        renewals: Arc<Mutex<usize>>,
        queues: Arc<Mutex<usize>>,
    }
    impl Respond for ExpiringOracle {
        fn respond(&self, r: &Request) -> ResponseTemplate {
            if r.url.path().ends_with("/renew") {
                let mut count = self.renewals.lock().unwrap();
                *count += 1;
                if *count > 1 {
                    return ResponseTemplate::new(503);
                }
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64();
                return ResponseTemplate::new(200)
                    .set_body_json(json!({"credential_id":"worker","expires_at":now+2.0}));
            }
            if r.url.path().ends_with("/queue") {
                let mut count = self.queues.lock().unwrap();
                *count += 1;
                if *count > 1 {
                    return ResponseTemplate::new(200)
                        .set_body_json(json!({"attachments":[],"poll_after_seconds":1}))
                        .set_delay(Duration::from_secs(10));
                }
            }
            self.inner.respond(r)
        }
    }
    #[tokio::test]
    async fn worker_credential_expiry_interrupts_slow_queue_and_cleans_active_sandbox() {
        let server = MockServer::start().await;
        let base = fault_oracle(false);
        let oracle = ExpiringOracle {
            inner: base.oracle,
            renewals: Default::default(),
            queues: Default::default(),
        };
        Mock::given(wiremock::matchers::any())
            .respond_with(oracle.clone())
            .mount(&server)
            .await;
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        let launcher = script(&root, "hold");
        let child = spawn_worker_mode(&server, &root.join("state"), &launcher, false).await;
        let output = tokio::time::timeout(Duration::from_secs(6), child.wait_with_output())
            .await
            .expect("credential expiry waited for queue transport")
            .unwrap();
        let result: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(result["status"], "recovery_required");
        assert_eq!(result["launches"][0]["cleanup"], "confirmed");
        assert!(*oracle.renewals.lock().unwrap() >= 2);
        assert!(
            *oracle.queues.lock().unwrap() >= 2,
            "queue must be in flight before expiry"
        );
        assert_eq!(*oracle.inner.teardowns.lock().unwrap(), 1);
        assert_eq!(
            std::fs::read_to_string(root.join("effects")).unwrap(),
            "effect\n"
        );
    }
    #[tokio::test]
    async fn ambiguous_claim_result_retains_durable_launch_identity() {
        let server = MockServer::start().await;
        Mock::given(wiremock::matchers::any())
            .respond_with(fault_oracle(true))
            .mount(&server)
            .await;
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        let launcher = script(&root, "normal");
        let (_, result) = run(&server, &root.join("state"), &launcher).await;
        let saved: Value = serde_json::from_str(
            &std::fs::read_to_string(root.join("state/attachment/workflow.jsonl")).unwrap(),
        )
        .unwrap();
        assert_eq!(result["launches"][0]["attachment_id"], "attachment");
        assert_eq!(result["launches"][0]["launch_id"], saved["key"]);
        assert!(
            !root.join("effects").exists(),
            "uncertain claim must not launch"
        );
    }
    #[tokio::test]
    async fn transient_queue_failure_keeps_active_sandbox_and_emits_early_identity() {
        use tokio::io::{AsyncBufReadExt, BufReader};
        let server = MockServer::start().await;
        let oracle = fault_oracle(false);
        Mock::given(wiremock::matchers::any())
            .respond_with(oracle.clone())
            .mount(&server)
            .await;
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        let launcher = script(&root, "hold");
        let mut child = spawn_worker_mode(&server, &root.join("state"), &launcher, false).await;
        let pid = child.id().unwrap();
        let mut stderr = BufReader::new(child.stderr.take().unwrap()).lines();
        // Collect while running: final stdout is reserved for the single terminal result.
        let progress = tokio::time::timeout(Duration::from_secs(3), stderr.next_line()).await;
        let end = tokio::time::Instant::now() + Duration::from_secs(6);
        while *oracle.renewals.lock().unwrap() < 3 && tokio::time::Instant::now() < end {
            tokio::time::sleep(Duration::from_millis(30)).await;
        }
        let polls = *oracle.queue.lock().unwrap();
        let renewals = *oracle.renewals.lock().unwrap();
        let early_teardowns = *oracle.oracle.teardowns.lock().unwrap();
        unsafe {
            libc::kill(pid as i32, libc::SIGTERM);
        }
        let out = child.wait_with_output().await.unwrap();
        assert!(
            polls >= 3,
            "transient queue failure stopped polling: {}",
            String::from_utf8_lossy(&out.stdout)
        );
        assert!(
            renewals >= 3,
            "transient credential renewal must retry while launch remains active"
        );
        assert_eq!(
            early_teardowns, 0,
            "transient queue failure tore down active sandbox"
        );
        let line = progress.expect("no early progress").unwrap().unwrap();
        let event: Value = serde_json::from_str(&line).unwrap();
        assert_eq!(event["attachment_id"], "attachment");
        assert!(event["launch_id"].is_string());
        assert!(!line.contains("worker-key") && !line.contains("executor-key"));
        let result: Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(result["cleanup"], "confirmed");
        assert_eq!(
            std::fs::read_to_string(root.join("effects")).unwrap(),
            "effect\n"
        );
    }
    #[tokio::test]
    async fn idle_worker_honors_poll_hint_and_signal_interrupts_wait() {
        use tokio::io::AsyncWriteExt;
        let server = MockServer::start().await;
        Mock::given(wiremock::matchers::any())
            .respond_with(|r: &Request| {
                let value = if r.url.path().ends_with("/renew") {
                    json!({"credential_id":"worker","expires_at":2000000000.0})
                } else {
                    json!({"attachments":[],"poll_after_seconds":5})
                };
                ResponseTemplate::new(200).set_body_json(value)
            })
            .mount(&server)
            .await;
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        let mut child = tokio::process::Command::new(
            std::env::var("SIKARU_TEST_INSTALLED")
                .unwrap_or_else(|_| env!("CARGO_BIN_EXE_sikaru").to_owned()),
        )
        .args([
            "--base-url",
            &server.uri(),
            "compute",
            "worker",
            "--bootstrap",
            "-",
            "--launcher",
            "/bin/sh",
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
        child.stdin.take().unwrap().write_all(json!({"project_id":"project","environment_id":"environment","token":"worker-key","state_dir":root.join("state")}).to_string().as_bytes()).await.unwrap();
        let startup = tokio::time::Instant::now() + Duration::from_secs(5);
        while !server
            .received_requests()
            .await
            .unwrap()
            .iter()
            .any(|r| r.url.path().ends_with("/queue"))
        {
            assert!(tokio::time::Instant::now() < startup, "worker never polled");
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        tokio::time::sleep(Duration::from_millis(1200)).await;
        let requests = server.received_requests().await.unwrap();
        assert_eq!(
            requests
                .iter()
                .filter(|r| r.url.path().ends_with("/queue"))
                .count(),
            1,
            "idle queue must honor five second hint"
        );
        unsafe {
            libc::kill(child.id().unwrap() as i32, libc::SIGTERM);
        }
        let out = tokio::time::timeout(Duration::from_secs(1), child.wait_with_output())
            .await
            .expect("signal delayed by idle backoff")
            .unwrap();
        let result: Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(result["status"], "cancelled");
        assert_eq!(result["cleanup"], "confirmed");
    }
    #[tokio::test]
    async fn two_workers_launch_once_and_lost_ack_is_reconciled_without_relaunch() {
        let server = MockServer::start().await;
        let claims = Arc::new(Mutex::new(0));
        let teardowns = Arc::new(Mutex::new(0));
        Mock::given(wiremock::matchers::any())
            .respond_with(Oracle {
                startup_fail: false,
                claims: claims.clone(),
                teardowns: teardowns.clone(),
            })
            .mount(&server)
            .await;
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        let launcher = script(&root, "lost");
        let left = root.join("left");
        let right = root.join("right");
        let (a, b) = tokio::join!(
            run(&server, &left, &launcher),
            run(&server, &right, &launcher)
        );
        assert_eq!(a.0, 0, "{}", a.1);
        assert_eq!(b.0, 0, "{}", b.1);
        for result in [&a.1, &b.1] {
            assert_eq!(result["launches"][0]["attachment_id"], "attachment");
            assert!(result["launches"][0]["launch_id"].is_string());
        }
        assert_eq!(
            std::fs::read_to_string(root.join("effects")).unwrap(),
            "effect\n"
        );
        assert_eq!(*teardowns.lock().unwrap(), 1);
        // Restarting either worker cannot create another sandbox after launch intent was durable.
        let _ = run(&server, &left, &launcher).await;
        let _ = run(&server, &right, &launcher).await;
        assert_eq!(
            std::fs::read_to_string(root.join("effects")).unwrap(),
            "effect\n"
        );
    }
    #[tokio::test]
    async fn launcher_timeout_keeps_effect_for_status_reconciliation() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        let launcher = script(&root, "timeout");
        let input = json!({"version":1,"operation":"launch","launch_id":"unique","credential":{"token":"executor-key"}});
        let result = super::launcher::invoke(&launcher, input, Duration::from_secs(10)).await;
        assert!(result.is_err());
        eprintln!("launcher error: {}", result.err().unwrap());
        assert_eq!(
            std::fs::read_to_string(root.join("effects")).unwrap(),
            "effect\n"
        );
        let result = super::launcher::invoke(
            &launcher,
            json!({"version":1,"operation":"status","launch_id":"unique"}),
            Duration::from_secs(2),
        )
        .await
        .unwrap();
        assert_eq!(result.status, "terminated");
        assert!(result.handle.is_some());
    }
    #[tokio::test]
    async fn failed_startup_and_unconfirmed_teardown_have_nonzero_distinct_outcomes() {
        for (mode, startup_fail, expected, cleanup) in [
            ("normal", true, 1, "confirmed"),
            ("unconfirmed", false, 4, "unconfirmed"),
            ("contradict", false, 4, "unconfirmed"),
        ] {
            let server = MockServer::start().await;
            Mock::given(wiremock::matchers::any())
                .respond_with(Oracle {
                    claims: Arc::new(Mutex::new(0)),
                    teardowns: Arc::new(Mutex::new(0)),
                    startup_fail,
                })
                .mount(&server)
                .await;
            let dir = tempfile::tempdir().unwrap();
            let root = dir.path().canonicalize().unwrap();
            let launcher = script(&root, mode);
            let (code, result) = run(&server, &root.join("state"), &launcher).await;
            assert_eq!(code, expected, "{result}");
            assert_eq!(result["cleanup"], cleanup);
        }
    }
    #[tokio::test]
    async fn long_worker_renews_scoped_credential_and_signal_awaits_teardown() {
        let server = MockServer::start().await;
        let teardowns = Arc::new(Mutex::new(0));
        Mock::given(wiremock::matchers::any())
            .respond_with(Oracle {
                startup_fail: false,
                claims: Arc::new(Mutex::new(0)),
                teardowns: teardowns.clone(),
            })
            .mount(&server)
            .await;
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        let launcher = script(&root, "hold");
        let child = spawn_worker(&server, &root.join("state"), &launcher).await;
        let pid = child.id().unwrap();
        let end = tokio::time::Instant::now() + Duration::from_secs(40);
        let journal = root.join("state/attachment/workflow.jsonl");
        let mut idle_journal_bytes = None;
        loop {
            let requests = server.received_requests().await.unwrap();
            if idle_journal_bytes.is_none()
                && requests
                    .iter()
                    .filter(|r| r.url.path().ends_with("/status"))
                    .count()
                    >= 3
            {
                idle_journal_bytes = Some(std::fs::metadata(&journal).unwrap().len());
            }
            if idle_journal_bytes.is_some()
                && requests.iter().any(|r| r.url.path().ends_with("/renew"))
            {
                break;
            }
            assert!(
                tokio::time::Instant::now() < end,
                "worker never renewed restricted credential"
            );
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        let journal_bytes = std::fs::metadata(&journal).unwrap().len();
        unsafe {
            libc::kill(pid as i32, libc::SIGTERM);
        }
        let output = tokio::time::timeout(Duration::from_secs(15), child.wait_with_output())
            .await
            .unwrap()
            .unwrap();
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(
            output.status.code(),
            Some(3),
            "{value} {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(value["cleanup"], "confirmed");
        assert_eq!(*teardowns.lock().unwrap(), 1);
        assert_eq!(
            Some(journal_bytes),
            idle_journal_bytes,
            "unchanged sandbox observations must not grow durable recovery state"
        );
        assert_eq!(
            std::fs::read_to_string(root.join("effects")).unwrap(),
            "effect\n"
        );
    }
}
