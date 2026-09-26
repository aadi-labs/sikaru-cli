//! Native executor acceptance at the installed command boundary.
use std::process::Command;
#[test]
fn serve_is_available_in_the_primary_binary() {
    let output = Command::new(
        std::env::var("SIKARU_TEST_INSTALLED")
            .unwrap_or_else(|_| env!("CARGO_BIN_EXE_sikaru").to_owned()),
    )
    .args(["compute", "serve", "--help"])
    .output()
    .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("bootstrap"));
}
#[test]
fn generated_api_commands_remain_available() {
    let output = Command::new(
        std::env::var("SIKARU_TEST_INSTALLED")
            .unwrap_or_else(|_| env!("CARGO_BIN_EXE_sikaru").to_owned()),
    )
    .args(["agents", "--help"])
    .output()
    .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("create_managed_session"));
}

#[cfg(unix)]
mod wire {
    use serde_json::{json, Value};
    use sha2::{Digest, Sha256};
    use std::collections::BTreeMap;
    use std::{
        process::Stdio,
        sync::{Arc, Mutex},
        time::Duration,
    };
    use tokio::io::AsyncWriteExt;
    use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};
    #[derive(Default)]
    struct State {
        ready: bool,
        stage: usize,
        handle: String,
        receipts: Vec<Value>,
        heartbeat: usize,
        renewals: usize,
        polls: usize,
        cleanup: Option<Value>,
        duplicate: Option<Value>,
        reconciliations: Vec<Value>,
        lost_issued: bool,
        ready_body: Option<Value>,
        checkpoint_reads: usize,
        tree_posts: usize,
        tree: Option<Value>,
        tree_id: Option<String>,
        blobs: BTreeMap<String, Vec<u8>>,
        checkpoint_events: Vec<&'static str>,
        http_receipts: usize,
        channel: ChannelState,
    }
    /// What the fake gateway saw on its executor channel endpoint.
    #[derive(Default)]
    struct ChannelState {
        connections: usize,
        refusals: usize,
        unauthorized: usize,
        heartbeats: usize,
        operations_sent: Vec<String>,
        receipts: Vec<Value>,
        arrivals: Vec<std::time::SystemTime>,
        resend: Option<Value>,
        switched: bool,
    }
    #[derive(Clone)]
    struct Oracle {
        state: Arc<Mutex<State>>,
        scenario: &'static str,
        effect: std::path::PathBuf,
    }
    fn attachment(status: &str, ttl: u64) -> Value {
        json!({"id":"attachment","session_id":"session","project_id":"project","environment_id":"environment","provider_id":"provider",
            "workspace_generation":"generation","workspace_provenance":{"kind":"existing_directory","identity":"original"},"journal_id":"journal",
            "status":status,"owner_id":"worker","owner_epoch":1,"lease_until":2000000000.0,"lease_ttl_seconds":ttl,"startup_deadline":2000000000.0,
            "capabilities":["compute.execute"],"protocol_version":"sikaru-compute-v1","cleanup_status":"unconfirmed","uncertain_operations":[],"processes":[]})
    }
    fn operation(stage: usize, method: &str, args: Value) -> Value {
        json!({"run_id":"r".repeat(128),"tool_call_id":format!("{stage:0128}"),"tool_provider_id":"provider","capability_name":"compute.execute","method":method,
            "arguments":args,"request_digest":format!("opaque-{stage}"),"owner_epoch":1,"workspace_generation":"generation"})
    }
    impl Respond for Oracle {
        fn respond(&self, request: &Request) -> ResponseTemplate {
            assert_eq!(
                request.headers.get("authorization").unwrap(),
                "Bearer restricted-executor-secret"
            );
            assert!(request.headers.get("api_key").is_none());
            let mut state = self.state.lock().unwrap();
            let body: Value = serde_json::from_slice(&request.body).unwrap_or(Value::Null);
            let path = request.url.path();
            if let Some(response) = self.fault(&mut state, path, &body) {
                return response;
            }
            if path.ends_with("/work") {
                return self.work(&mut state);
            }
            if path.ends_with("/receipts") {
                return self.receipt(&mut state, body);
            }
            if path.contains("/workspace-checkpoints/") {
                return self.checkpoint(&mut state, request, body);
            }
            self.lifecycle(&mut state, path, body)
        }
    }
    impl Oracle {
        fn fault(&self, s: &mut State, path: &str, body: &Value) -> Option<ResponseTemplate> {
            if self.credential_revoked() {
                return Some(ResponseTemplate::new(401));
            }
            match (self.scenario, path.rsplit('/').next().unwrap()) {
                ("uploadfail", "receipts") => {
                    if s.receipts.is_empty() {
                        s.receipts.push(body.clone());
                    } else {
                        assert_eq!(&s.receipts[0], body);
                    }
                    Some(ResponseTemplate::new(503))
                }
                ("late", "ready") => {
                    s.ready = true;
                    Some(ok(attachment("ready", 1)).set_delay(Duration::from_millis(1200)))
                }
                ("lostpoll", "work") if s.ready => {
                    s.lost_issued = true;
                    Some(ResponseTemplate::new(503))
                }
                ("cleanupfail", "cleanup") => Some(ResponseTemplate::new(503)),
                ("mismatch", "status") => {
                    let mut a = attachment("ready", 3);
                    a["workspace_provenance"]["identity"] = json!("replacement");
                    Some(ok(a))
                }
                _ => None,
            }
        }
        fn credential_revoked(&self) -> bool {
            self.scenario == "revoked" && self.effect.with_file_name("started").exists()
        }
        fn lifecycle(&self, s: &mut State, path: &str, body: Value) -> ResponseTemplate {
            let a = attachment("ready", if self.scenario == "expiry" { 1 } else { 3 });
            if path.ends_with("/ready") {
                s.ready = true;
                s.ready_body = Some(body);
                return ok(a);
            }
            if path.ends_with("/connect") {
                s.ready = false;
                return ok(attachment("starting", 3));
            }
            if path.ends_with("/heartbeat") {
                s.heartbeat += 1;
                if self.scenario == "expiry" {
                    return ResponseTemplate::new(503)
                        .set_body_json(json!({"detail":"restricted-executor-secret"}));
                }
                return ok(a);
            }
            if path.ends_with("/renew") {
                return self.renew(s);
            }
            if path.ends_with("/reconcile") {
                s.reconciliations.push(body.clone());
                assert_eq!(body["receipts"], json!([]));
                return ok(json!({"attachment":a,"receipts":[]}));
            }
            if path.ends_with("/cleanup") {
                s.cleanup = Some(body);
                return ok(attachment("cleaned", 3));
            }
            if path.ends_with("/stop") {
                return ok(attachment("stopping", 3));
            }
            assert!(path.ends_with("/status"));
            ok(a)
        }
        fn renew(&self, s: &mut State) -> ResponseTemplate {
            s.renewals += 1;
            if self.scenario == "credential_outage" || self.scenario == "renew_denied" {
                if s.renewals > 1 {
                    return ResponseTemplate::new(if self.scenario == "renew_denied" {
                        403
                    } else {
                        503
                    });
                }
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64();
                return ok(json!({"credential_id":"credential","expires_at":now+1.5}));
            }
            if self.scenario == "renew_retry" && s.renewals == 1 {
                return ResponseTemplate::new(503).set_delay(Duration::from_millis(1500));
            }
            if self.scenario == "credential_expiry" {
                return ok(json!({"credential_id":"credential","expires_at":0.0}));
            }
            return ok(json!({"credential_id":"credential","expires_at":2000000000.0}));
        }
        fn work(&self, s: &mut State) -> ResponseTemplate {
            s.polls += 1;
            let mut page = self.advertised_page(s);
            if !s.ready {
                if s.lost_issued {
                    page["issued_operations"] = json!([{ "run_id":"r".repeat(128),"tool_call_id":format!("{0:0128}",0),"request_digest":"opaque-0","owner_epoch":1,"workspace_generation":"generation","method":"bash.start"}]);
                }
                return ok(page);
            }
            if self.scenario == "approval" {
                page["execution"]["approval_required"] = json!(true);
                return ok(page);
            }
            if self.scenario == "hang" {
                return ok(page).set_delay(Duration::from_secs(2));
            }
            if s.stage >= self.final_stage() {
                return self.completed_work(s, page);
            }
            if self.withholds_operation(s) {
                return ok(page);
            }
            let op = self.next(s);
            page["operations"] = json!([op]);
            ok(page)
        }
        fn completed_work(&self, s: &mut State, mut page: Value) -> ResponseTemplate {
            if self.scenario.starts_with("checkpoint") {
                return self.checkpoint_page(s, page);
            }
            page["execution"]["terminal"] = json!(true);
            page["execution"]["status"] = json!("completed");
            ok(page)
        }
        fn checkpoint_view(&self, s: &State) -> Value {
            json!({"checkpoint_id":"capture-one","run_id":"run",
                "workspace_generation":"generation","owner_epoch":1,
                "status":if s.tree.is_some() {"published"} else {"requested"},
                "tree_id":s.tree_id})
        }
        fn checkpoint_page(&self, s: &mut State, mut page: Value) -> ResponseTemplate {
            page["execution"]["status"] = json!("completed");
            if s.tree.is_some() && s.checkpoint_events.last() == Some(&"ack") {
                s.checkpoint_events.push("terminal");
                page["execution_phase"] = json!("terminal");
                page["execution"]["terminal"] = json!(true);
            } else {
                page["execution_phase"] = json!("checkpointing");
                page["workspace_checkpoint"] = self.checkpoint_view(s);
                if self.scenario == "checkpoint_generation" {
                    page["workspace_checkpoint"]["workspace_generation"] = json!("replacement");
                }
            }
            ok(page)
        }
        fn checkpoint(&self, s: &mut State, request: &Request, body: Value) -> ResponseTemplate {
            let path = request.url.path();
            assert!(path.contains("/workspace-checkpoints/run"));
            if path.contains("/blobs/") {
                assert_eq!(request.method.as_str(), "PUT");
                assert!(request.body.len() <= 1_000_000);
                let hash = path.rsplit('/').next().unwrap();
                assert_eq!(hash, format!("{:x}", Sha256::digest(&request.body)));
                s.blobs.insert(hash.into(), request.body.clone());
                s.checkpoint_events.push("blob");
                let receipt = ok(json!({"sha256":hash,"size":request.body.len()}));
                if self.scenario == "checkpoint_slow_upload" {
                    // Outlasts the heartbeat interval so a renewal lands mid-upload.
                    return receipt.set_delay(Duration::from_millis(1500));
                }
                return receipt;
            }
            if path.ends_with("/tree") {
                return self.commit_tree(s, body);
            }
            assert_eq!(request.method.as_str(), "GET");
            s.checkpoint_reads += 1;
            s.checkpoint_events
                .push(if s.tree.is_some() { "ack" } else { "read" });
            ok(self.checkpoint_view(s))
        }
        fn commit_tree(&self, s: &mut State, body: Value) -> ResponseTemplate {
            s.tree_posts += 1;
            assert_eq!(s.tree_posts, 1, "uncertain commit must use status lookup");
            let files = body["files"].as_object().unwrap();
            assert_eq!(files.len(), 3);
            for file in files.values() {
                let mut bytes = Vec::new();
                for chunk in file["chunks"].as_array().unwrap() {
                    let data = &s.blobs[chunk["sha256"].as_str().unwrap()];
                    assert_eq!(data.len() as u64, chunk["size"].as_u64().unwrap());
                    bytes.extend(data);
                }
                assert_eq!(file["sha256"], format!("{:x}", Sha256::digest(&bytes)));
                assert_eq!(file["size"], bytes.len());
            }
            s.tree_id = Some(canonical_tree_id(&body));
            s.tree = Some(body);
            s.checkpoint_events.push("commit");
            if self.scenario == "checkpoint_lost" {
                std::fs::write(
                    self.effect.with_file_name("result.txt"),
                    "changed after commit",
                )
                .unwrap();
                return ResponseTemplate::new(503);
            }
            s.checkpoint_events.push("ack");
            ok(self.checkpoint_view(s))
        }
        fn next(&self, s: &State) -> Value {
            if self.scenario == "altered" && s.stage == 1 {
                return operation(0, "bash.start", json!({"command":"echo twice >> effect"}));
            }
            if s.stage == 0 {
                let command=match self.scenario {
                    "credential_outage"|"renew_denied"|"credential_expiry"|"expiry"|"signal"|"revoked"=>"touch started; sleep 2; touch escaped",
                    "renew_retry"=>"sleep 4; echo once >> effect; head -c 65536 /dev/zero",
                    _=>"echo once >> effect; printf '%s' \"${SIKARU_API_KEY-unset}\" > env; head -c 65536 /dev/zero",
                };
                return operation(
                    0,
                    "bash.start",
                    json!({"command":command,"env":{"SIKARU_API_KEY":"controller-secret","SIKARU_EXECUTOR_TOKEN":"restricted-executor-secret"}}),
                );
            }
            if s.stage == 1 {
                return operation(1, "bash.wait", json!({"handle_id":s.handle,"timeout":10}));
            }
            if s.stage == 2 {
                return operation(
                    2,
                    "bash.read",
                    json!({"handle_id":s.handle,"offset":0,"limit":65536}),
                );
            }
            operation(
                3,
                "workspace.write_text",
                json!({"path":"result.txt","text":"finished"}),
            )
        }
        fn receipt(&self, s: &mut State, body: Value) -> ResponseTemplate {
            s.http_receipts += 1;
            assert!(serde_json::to_vec(&body).unwrap().len() <= 256 * 1024);
            assert!(body["idempotency_key"].as_str().unwrap().len() <= 128);
            if let Some(previous) = s.duplicate.take() {
                assert_eq!(body, previous);
                return ok(json!({"created":false,"tool_call_id":body["tool_call_id"]}));
            }
            if s.stage == 0 {
                s.handle = body["payload"]["id"].as_str().unwrap().into();
            }
            s.receipts.push(body.clone());
            s.stage += 1;
            if s.stage == 1 && self.scenario == "normal" {
                s.duplicate = Some(body);
                return ResponseTemplate::new(503)
                    .set_body_json(json!({"detail":"restricted-executor-secret"}));
            }
            ok(json!({"created":true,"tool_call_id":body["tool_call_id"]}))
        }
    }
    fn running_page() -> Value {
        json!({"attachment":attachment("ready",3),"execution_phase":"running","execution":{"run_id":"run","status":"running","terminal":false,"approval_required":false},"operations":[],"issued_operations":[],"live_handles":[]})
    }
    fn canonical_tree_id(tree: &Value) -> String {
        // The public manifest contract specifies field order independent of JSON map order.
        let mut entries = Vec::new();
        for (path, file) in tree["files"].as_object().unwrap() {
            let chunks = file["chunks"]
                .as_array()
                .unwrap()
                .iter()
                .map(|chunk| {
                    format!(
                        "{{\"sha256\":{},\"size\":{}}}",
                        chunk["sha256"], chunk["size"]
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            entries.push(format!(
                "{}:{{\"sha256\":{},\"size\":{},\"mode\":{},\"chunks\":[{}]}}",
                serde_json::to_string(path).unwrap(),
                file["sha256"],
                file["size"],
                file["mode"],
                chunks
            ));
        }
        format!(
            "{:x}",
            Sha256::digest(format!("{{\"files\":{{{}}}}}", entries.join(",")))
        )
    }
    fn ok(value: Value) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(value)
    }
    async fn launch(
        scenario: &'static str,
    ) -> (tempfile::TempDir, MockServer, Oracle, tokio::process::Child) {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().canonicalize().unwrap();
        std::fs::create_dir(path.join("workspace")).unwrap();
        let server = MockServer::start().await;
        let oracle = Oracle {
            state: Default::default(),
            scenario,
            effect: path.join("workspace/effect"),
        };
        Mock::given(wiremock::matchers::any())
            .respond_with(oracle.clone())
            .mount(&server)
            .await;
        let base_url = if oracle.channel_scenario() {
            channel::front(*server.address(), oracle.clone()).await
        } else {
            server.uri()
        };
        let command_timeout = if scenario == "channel_long" { 60 } else { 10 };
        let bootstrap = json!({"project_id":"project","session_id":"session","attachment_id":"attachment","owner_epoch":1,"workspace_generation":"generation","journal_id":"journal","credential_id":"credential",
            "token":"restricted-executor-secret","workspace_provenance":{"kind":"existing_directory","identity":"original"},"workspace":path.join("workspace"),"state_dir":path.join("state"),"command_timeout_seconds":command_timeout});
        let mut child = tokio::process::Command::new(
            std::env::var("SIKARU_TEST_INSTALLED")
                .unwrap_or_else(|_| env!("CARGO_BIN_EXE_sikaru").to_owned()),
        )
        .args([
            "compute",
            "serve",
            "--bootstrap",
            "-",
            "--base-url",
            &base_url,
        ])
        .env("SIKARU_API_KEY", "controller-secret")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(bootstrap.to_string().as_bytes())
            .await
            .unwrap();
        (root, server, oracle, child)
    }
    async fn result(child: tokio::process::Child) -> std::process::Output {
        tokio::time::timeout(Duration::from_secs(30), child.wait_with_output())
            .await
            .unwrap()
            .unwrap()
    }
    #[tokio::test]
    async fn generated_transport_receipt_replay_and_native_payloads() {
        let (root, _server, oracle, child) = launch("normal").await;
        let output = result(child).await;
        assert!(
            output.status.success(),
            "{} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["status"], "completed");
        assert_eq!(value["cleanup"], "confirmed");
        assert_eq!(value["usage"]["available"], false);
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/env")).unwrap(),
            "unset"
        );
        let state = oracle.state.lock().unwrap();
        assert_eq!(state.receipts.len(), 4);
        if let Ok(path) = std::env::var("U6_NATIVE_RECEIPTS") {
            std::fs::write(path, serde_json::to_vec(&state.receipts).unwrap()).unwrap();
        }
        let read = &state.receipts[2]["payload"];
        assert_eq!(read["output"].as_str().unwrap(), "\0".repeat(24576));
        assert_eq!(read["next_offset"], 24576);
        assert_eq!(read["total_bytes"], 65536);
        assert_eq!(read["omitted_after"], 40960);
        assert_eq!(
            std::fs::metadata(read["artifact_path"].as_str().unwrap())
                .unwrap()
                .len(),
            65536
        );
        let journal = std::fs::read_to_string(root.path().join("state/journal.jsonl")).unwrap();
        for secret in ["controller-secret", "restricted-executor-secret"] {
            assert!(!String::from_utf8_lossy(&output.stdout).contains(secret));
            assert!(!String::from_utf8_lossy(&output.stderr).contains(secret));
            assert!(!journal.contains(secret));
        }
    }
    #[tokio::test]
    async fn workspace_checkpoint_is_acknowledged_before_terminal_completion() {
        let (_root, _server, oracle, child) = launch("checkpoint").await;
        let output = result(child).await;
        assert!(
            output.status.success(),
            "{} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let state = oracle.state.lock().unwrap();
        assert!(state.ready_body.as_ref().unwrap()["capabilities"]
            .as_array()
            .unwrap()
            .contains(&json!("filesystem-checkpoint-v1")));
        assert_eq!(state.receipts.len(), 4);
        assert_eq!(state.tree_posts, 1);
        assert_eq!(
            &state.checkpoint_events[state.checkpoint_events.len() - 3..],
            &["commit", "ack", "terminal"]
        );
        assert_eq!(
            state.tree.as_ref().unwrap()["files"]["result.txt"]["sha256"],
            format!("{:x}", Sha256::digest(b"finished"))
        );
    }
    #[tokio::test]
    async fn workspace_lost_commit_ack_recovers_same_capture_without_reexecution() {
        let (root, _server, oracle, child) = launch("checkpoint_lost").await;
        let output = result(child).await;
        assert!(
            output.status.success(),
            "{} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/result.txt")).unwrap(),
            "changed after commit"
        );
        let state = oracle.state.lock().unwrap();
        assert_eq!(state.tree_posts, 1);
        assert!(state.checkpoint_reads >= 2);
        assert_eq!(state.receipts.len(), 4);
        assert_eq!(
            &state.checkpoint_events[state.checkpoint_events.len() - 3..],
            &["commit", "ack", "terminal"]
        );
        assert_eq!(
            state.tree.as_ref().unwrap()["files"]["result.txt"]["sha256"],
            format!("{:x}", Sha256::digest(b"finished"))
        );
    }
    #[tokio::test]
    async fn lease_renewal_during_checkpoint_upload_does_not_deadlock_executor() {
        let (_root, _server, oracle, child) = launch("checkpoint_slow_upload").await;
        let output = result(child).await;
        assert!(
            output.status.success(),
            "{} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let state = oracle.state.lock().unwrap();
        assert_eq!(
            &state.checkpoint_events[state.checkpoint_events.len() - 3..],
            &["commit", "ack", "terminal"]
        );
    }
    #[tokio::test]
    async fn workspace_checkpoint_wrong_generation_cannot_capture_files() {
        let (_root, _server, oracle, child) = launch("checkpoint_generation").await;
        let output = result(child).await;
        assert_eq!(output.status.code(), Some(4));
        let state = oracle.state.lock().unwrap();
        assert_eq!(state.receipts.len(), 4);
        assert_eq!(state.checkpoint_reads, 0);
        assert!(state.blobs.is_empty());
        assert!(state.tree.is_none());
    }
    #[tokio::test]
    async fn heartbeat_continues_while_poll_is_slow() {
        let (_root, _server, oracle, child) = launch("hang").await;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
        while !oracle.state.lock().unwrap().ready {
            assert!(tokio::time::Instant::now() < deadline);
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        tokio::time::sleep(Duration::from_secs(4)).await;
        assert!(oracle.state.lock().unwrap().heartbeat >= 2);
        unsafe {
            libc::kill(child.id().unwrap() as i32, libc::SIGTERM);
        }
        let output = result(child).await;
        assert!(!output.status.success());
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["status"], "cancelled", "{value}");
        assert_eq!(value["cancel_acknowledged"], true);
    }
    #[tokio::test]
    async fn transient_credential_renewal_preserves_live_work_and_heartbeats() {
        let (root, _server, oracle, child) = launch("renew_retry").await;
        let output = result(child).await;
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["status"], "completed", "{value}");
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
        let state = oracle.state.lock().unwrap();
        assert!(state.renewals >= 2);
        assert!(
            state.heartbeat >= 3,
            "credential request must not block heartbeat"
        );
    }
    #[tokio::test]
    async fn credential_outage_and_revocation_fence_live_work_despite_healthy_lease() {
        for scenario in ["credential_outage", "renew_denied"] {
            let (root, _server, oracle, child) = launch(scenario).await;
            let output = result(child).await;
            let value: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(value["status"], "recovery_required", "{scenario}: {value}");
            assert!(root.path().join("workspace/started").exists());
            assert!(oracle.state.lock().unwrap().renewals >= 2);
            tokio::time::sleep(Duration::from_secs(2)).await;
            assert!(!root.path().join("workspace/escaped").exists());
        }
    }
    #[tokio::test]
    async fn expired_credential_fences_even_when_heartbeat_succeeds() {
        let (root, _server, _oracle, child) = launch("credential_expiry").await;
        let output = result(child).await;
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["status"], "recovery_required", "{value}");
        tokio::time::sleep(Duration::from_secs(2)).await;
        assert!(!root.path().join("workspace/escaped").exists());
    }
    #[tokio::test]
    async fn failed_renewal_expires_and_kills_live_work() {
        let (root, _server, _oracle, child) = launch("expiry").await;
        let output = result(child).await;
        assert!(!output.status.success());
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["status"], "recovery_required");
        tokio::time::sleep(Duration::from_secs(2)).await;
        assert!(!root.path().join("workspace/escaped").exists());
    }
    #[tokio::test]
    async fn revoked_credential_stops_children_without_claiming_remote_cleanup() {
        let (root, _server, _oracle, child) = launch("revoked").await;
        let output = result(child).await;
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(output.status.code(), Some(4), "{value}");
        assert_eq!(value["cleanup"], "unconfirmed");
        assert!(root.path().join("workspace/started").exists());
        tokio::time::sleep(Duration::from_secs(2)).await;
        assert!(!root.path().join("workspace/escaped").exists());
    }
    #[tokio::test]
    async fn signals_await_cleanup_of_spawned_work() {
        for signal in [libc::SIGINT, libc::SIGTERM] {
            let (root, _server, _oracle, child) = launch("signal").await;
            let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
            while !root.path().join("workspace/started").exists() {
                assert!(tokio::time::Instant::now() < deadline);
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
            unsafe {
                libc::kill(child.id().unwrap() as i32, signal);
            }
            let output = result(child).await;
            let value: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert!(
                matches!(
                    value["status"].as_str(),
                    Some("cancelled" | "recovery_required")
                ),
                "{value}"
            );
            assert_eq!(value["cancel_acknowledged"], true);
            assert_eq!(
                value["cleanup"],
                "confirmed",
                "{value} {}",
                String::from_utf8_lossy(&output.stderr)
            );
            tokio::time::sleep(Duration::from_millis(2200)).await;
            assert!(!root.path().join("workspace/escaped").exists());
        }
    }
    #[tokio::test]
    async fn approval_parks_cleanly_with_nonzero_exit() {
        let (_root, _server, _oracle, child) = launch("approval").await;
        let output = result(child).await;
        assert!(!output.status.success());
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["status"], "approval_required");
        assert_eq!(value["cleanup"], "confirmed");
    }
    #[tokio::test]
    async fn changed_operation_with_copied_digest_cannot_repeat_effect() {
        let (root, _server, _oracle, child) = launch("altered").await;
        let output = result(child).await;
        assert_eq!(output.status.code(), Some(4));
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
    }
    #[tokio::test]
    async fn receipt_upload_failure_retains_immutable_result_without_reexecution() {
        let (root, _server, oracle, child) = launch("uploadfail").await;
        let output = result(child).await;
        assert_eq!(output.status.code(), Some(4));
        assert_eq!(oracle.state.lock().unwrap().receipts.len(), 1);
        let journal = std::fs::read_to_string(root.path().join("state/journal.jsonl")).unwrap();
        let receipts = journal
            .lines()
            .filter_map(|line| serde_json::from_str::<Value>(line).ok())
            .filter(|v| v["record"] == "Receipt")
            .count();
        assert_eq!(receipts, 1);
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
    }
    #[tokio::test]
    async fn missing_poll_response_reports_issued_uncertainty_without_running_it() {
        let (root, _server, oracle, child) = launch("lostpoll").await;
        let output = result(child).await;
        assert_eq!(
            output.status.code(),
            Some(4),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        assert!(!root.path().join("workspace/effect").exists());
        let s = oracle.state.lock().unwrap();
        assert_eq!(
            s.reconciliations.last().unwrap()["uncertain_operation_ids"],
            json!([format!("{0:0128}", 0)])
        );
    }
    #[tokio::test]
    async fn late_ready_ack_and_wrong_workspace_never_start_task_io() {
        for scenario in ["late", "mismatch"] {
            let (root, _server, _oracle, child) = launch(scenario).await;
            let output = result(child).await;
            assert_eq!(output.status.code(), Some(4));
            assert!(!root.path().join("workspace/effect").exists());
        }
    }
    #[tokio::test]
    async fn remote_cleanup_failure_is_never_successful_completion() {
        let (_root, _server, _oracle, child) = launch("cleanupfail").await;
        let output = result(child).await;
        assert_eq!(output.status.code(), Some(4));
        let v: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(v["cleanup"], "unconfirmed");
    }

    /// Executor channel endpoint of the fake gateway. One front port routes each
    /// connection: a WebSocket upgrade for the channel path is served here, and
    /// everything else is piped unchanged to the HTTP oracle.
    mod channel {
        use super::*;
        use futures_util::{SinkExt, StreamExt};
        use tokio::net::TcpStream;
        use tokio_tungstenite::tungstenite::{
            handshake::server::{ErrorResponse, Request as Upgrade, Response as Accept},
            http,
            protocol::{frame::coding::CloseCode, CloseFrame},
            Message,
        };
        type Socket = tokio_tungstenite::WebSocketStream<TcpStream>;
        type Sink = futures_util::stream::SplitSink<Socket, Message>;

        pub(super) async fn front(http_oracle: std::net::SocketAddr, oracle: Oracle) -> String {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            tokio::spawn(async move {
                while let Ok((stream, _)) = listener.accept().await {
                    tokio::spawn(route(stream, http_oracle, oracle.clone()));
                }
            });
            format!("http://{address}")
        }
        async fn route(mut stream: TcpStream, http_oracle: std::net::SocketAddr, oracle: Oracle) {
            let Some(head) = request_head(&stream).await else {
                return;
            };
            if head.starts_with("get ")
                && head.contains("/channel")
                && head.contains("upgrade: websocket")
            {
                return upgrade(stream, oracle).await;
            }
            let mut upstream = TcpStream::connect(http_oracle).await.unwrap();
            let _ = tokio::io::copy_bidirectional(&mut stream, &mut upstream).await;
        }
        async fn request_head(stream: &TcpStream) -> Option<String> {
            let mut head = [0u8; 4096];
            loop {
                let n = stream.peek(&mut head).await.ok().filter(|n| *n > 0)?;
                if head[..n].windows(4).any(|w| w == b"\r\n\r\n") || n == head.len() {
                    return Some(String::from_utf8_lossy(&head[..n]).to_ascii_lowercase());
                }
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        }
        #[allow(clippy::result_large_err)] // the upgrade callback signature is fixed
        async fn upgrade(stream: TcpStream, oracle: Oracle) {
            let state = oracle.state.clone();
            let refuse = oracle.scenario == "channel_refused";
            let check = move |request: &Upgrade, accept: Accept| -> Result<Accept, ErrorResponse> {
                let mut s = state.lock().unwrap();
                let bearer = request
                    .headers()
                    .get("authorization")
                    .map(|v| v.as_bytes().to_vec());
                if bearer.as_deref() != Some(b"Bearer restricted-executor-secret".as_slice())
                    || request.uri().query().is_some()
                {
                    s.channel.unauthorized += 1;
                }
                if refuse {
                    s.channel.refusals += 1;
                    return Err(http::Response::builder().status(503).body(None).unwrap());
                }
                Ok(accept)
            };
            if let Ok(socket) = tokio_tungstenite::accept_hdr_async(stream, check).await {
                oracle.serve_channel(socket).await;
            }
        }
        async fn send(sink: &mut Sink, value: Value) {
            let _ = sink.send(Message::Text(value.to_string())).await;
        }
        fn operations(page: Value) -> Value {
            json!({"type":"operations","page":page})
        }
        enum Reply {
            /// Drop the connection without accepting the receipt.
            Drop,
            Accept {
                frame: Value,
                next: Option<Value>,
                close: bool,
            },
        }
        impl Oracle {
            async fn serve_channel(&self, socket: Socket) {
                let (mut sink, mut stream) = socket.split();
                send(&mut sink, json!({"type":"handshake","protocol":"sikaru-compute-channel-v1","attachment_id":"attachment",
                    "owner_epoch":1,"workspace_generation":"generation","transport":"channel","heartbeat_interval_seconds":0.3,
                    "heartbeat_timeout_seconds":5,"max_frame_bytes":262144,"inbound_frame_limit":2000,"inbound_frame_window_seconds":300})).await;
                let first = {
                    let mut s = self.state.lock().unwrap();
                    s.channel.connections += 1;
                    match s.channel.resend.take() {
                        Some(op) => self.page_with(&mut s, op),
                        None => self.channel_page(&mut s),
                    }
                };
                send(&mut sink, operations(first)).await;
                while let Some(Ok(message)) = stream.next().await {
                    let Message::Text(text) = message else {
                        continue;
                    };
                    let frame: Value = serde_json::from_str(&text).unwrap();
                    if frame == json!({"type":"heartbeat"}) {
                        self.state.lock().unwrap().channel.heartbeats += 1;
                        continue;
                    }
                    assert_eq!(frame["type"], "receipt", "{frame}");
                    let reply = self.channel_receipt(frame["receipt"].clone());
                    let Reply::Accept { frame, next, close } = reply else {
                        return;
                    };
                    if self.scenario == "channel_slow" {
                        // Outlasts the lease renewal interval while the receipt awaits acceptance.
                        tokio::time::sleep(Duration::from_millis(1500)).await;
                    }
                    send(&mut sink, frame).await;
                    if let Some(page) = next {
                        send(&mut sink, operations(page)).await;
                    }
                    if close {
                        send(&mut sink, json!({"type":"close","reason":"transport_poll","detail":"The current turn selects polling"})).await;
                        let _ = sink
                            .send(Message::Close(Some(CloseFrame {
                                code: CloseCode::Normal,
                                reason: "transport_poll".into(),
                            })))
                            .await;
                        return;
                    }
                }
            }
            fn channel_receipt(&self, body: Value) -> Reply {
                let mut s = self.state.lock().unwrap();
                s.channel.receipts.push(body.clone());
                s.channel.arrivals.push(std::time::SystemTime::now());
                if self.scenario == "channel_drop" && s.channel.connections == 1 {
                    // Delivered and executed, but the receipt never returns on this connection.
                    s.channel.resend = Some(operation(
                        0,
                        "bash.start",
                        json!({"command":"echo once >> effect"}),
                    ));
                    return Reply::Drop;
                }
                let created = !s
                    .receipts
                    .iter()
                    .any(|r| r["tool_call_id"] == body["tool_call_id"]);
                if created {
                    if s.stage == 0 {
                        s.handle = body["payload"]["id"].as_str().unwrap_or_default().into();
                    }
                    s.receipts.push(body.clone());
                    s.stage += 1;
                }
                let close = self.scenario == "channel_switch" && s.stage == 2;
                s.channel.switched |= close;
                let frame = json!({"type":"receipt_accepted","run_id":body["run_id"],
                    "receipt":{"tool_call_id":body["tool_call_id"],"created":created,"status":"accepted"}});
                let next = (!close).then(|| self.channel_page(&mut s));
                Reply::Accept { frame, next, close }
            }
            fn channel_page(&self, s: &mut State) -> Value {
                let mut page = running_page();
                page["transport"] = json!("channel");
                if s.stage >= self.final_stage() {
                    page["execution"]["terminal"] = json!(true);
                    page["execution"]["status"] = json!("completed");
                    return page;
                }
                if self.scenario == "channel_signal" && s.stage >= 1 {
                    return page;
                }
                let op = self.channel_operation(s);
                self.page_with(s, op)
            }
            fn page_with(&self, s: &mut State, op: Value) -> Value {
                s.channel
                    .operations_sent
                    .push(op["tool_call_id"].as_str().unwrap().into());
                let mut page = running_page();
                page["transport"] = json!("channel");
                page["operations"] = json!([op]);
                page
            }
            fn channel_operation(&self, s: &State) -> Value {
                match (self.scenario, s.stage) {
                    ("channel_long", _) => operation(
                        0,
                        "bash.run",
                        json!({"command":"sleep 20; touch finished","yield_seconds":30,"limit":8192}),
                    ),
                    ("channel_signal", _) => operation(
                        0,
                        "bash.start",
                        json!({"command":"touch started; sleep 2; touch escaped"}),
                    ),
                    ("channel_drop", 0) => {
                        operation(0, "bash.start", json!({"command":"echo once >> effect"}))
                    }
                    _ => self.next(s),
                }
            }
        }
    }
    impl Oracle {
        /// The HTTP work page, advertising the channel in channel scenarios.
        fn advertised_page(&self, s: &State) -> Value {
            let mut page = running_page();
            if self.channel_scenario() {
                let transport = if s.channel.switched {
                    "poll"
                } else {
                    "channel"
                };
                page["transport"] = json!(transport);
            }
            page
        }
        fn withholds_operation(&self, s: &State) -> bool {
            let altered = self.scenario == "altered" && s.stage == 1 && !self.effect.exists();
            altered || self.channel_serves_operations(s)
        }
        fn channel_scenario(&self) -> bool {
            self.scenario.starts_with("channel")
        }
        /// Channel scenarios push operations on the channel until they switch to polling.
        fn channel_serves_operations(&self, s: &State) -> bool {
            self.channel_scenario() && self.scenario != "channel_refused" && !s.channel.switched
        }
        fn final_stage(&self) -> usize {
            if self.scenario == "channel_long" {
                1
            } else {
                4
            }
        }
    }
    fn completed(output: &std::process::Output) -> Value {
        let value: Value = serde_json::from_slice(&output.stdout).unwrap_or(Value::Null);
        assert_eq!(
            value["status"],
            "completed",
            "{value} {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(value["cleanup"], "confirmed", "{value}");
        value
    }
    #[tokio::test]
    async fn channel_operations_execute_once_with_receipts_on_the_channel() {
        let (root, _server, oracle, child) = launch("channel").await;
        completed(&result(child).await);
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
        let s = oracle.state.lock().unwrap();
        assert_eq!(s.channel.connections, 1);
        assert_eq!(s.channel.unauthorized, 0);
        assert_eq!(s.channel.receipts.len(), 4);
        assert_eq!(s.receipts.len(), 4);
        assert_eq!(s.http_receipts, 0, "receipts return on the channel");
        let mut sent = s.channel.operations_sent.clone();
        sent.dedup();
        assert_eq!(sent.len(), 4, "each operation is pushed once and runs once");
    }
    #[tokio::test]
    async fn dropped_channel_redelivers_the_operation_without_reexecuting_it() {
        let (root, _server, oracle, child) = launch("channel_drop").await;
        completed(&result(child).await);
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
        let s = oracle.state.lock().unwrap();
        assert_eq!(s.channel.connections, 2);
        assert_eq!(
            s.channel.operations_sent[0], s.channel.operations_sent[1],
            "same operation identity re-sent"
        );
        assert_eq!(
            s.channel.receipts[0], s.channel.receipts[1],
            "the recorded receipt, not a second execution"
        );
        assert_eq!(s.receipts.len(), 4);
        assert!(
            s.reconciliations.len() >= 2,
            "the lost channel is reconciled before reconnecting"
        );
    }
    #[tokio::test]
    async fn refused_channel_falls_back_to_polling_after_three_attempts() {
        let (root, _server, oracle, child) = launch("channel_refused").await;
        completed(&result(child).await);
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
        let s = oracle.state.lock().unwrap();
        assert_eq!(s.channel.refusals, 3);
        assert_eq!(s.receipts.len(), 4);
        assert!(s.http_receipts >= 4);
    }
    #[tokio::test]
    async fn advertised_switch_to_polling_is_followed_mid_run() {
        let (root, _server, oracle, child) = launch("channel_switch").await;
        completed(&result(child).await);
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
        let s = oracle.state.lock().unwrap();
        assert_eq!(
            s.channel.connections, 1,
            "a transport switch is not a failure to reconnect"
        );
        assert_eq!(s.channel.receipts.len(), 2);
        assert_eq!(s.receipts.len(), 4);
        assert_eq!(s.http_receipts, 2);
    }
    #[tokio::test]
    async fn lease_renewal_during_a_channel_send_does_not_block_the_executor() {
        let (_root, _server, oracle, child) = launch("channel_slow").await;
        completed(&result(child).await);
        let s = oracle.state.lock().unwrap();
        assert_eq!(s.channel.receipts.len(), 4);
        assert!(
            s.heartbeat >= 4,
            "lease renewed {} times across 6 s of sends",
            s.heartbeat
        );
        assert!(
            s.channel.heartbeats >= 2,
            "channel heartbeats continue while a receipt waits"
        );
    }
    #[tokio::test]
    async fn sigterm_during_a_channel_wait_cancels_with_cleanup_confirmed() {
        let (root, _server, oracle, child) = launch("channel_signal").await;
        let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
        while oracle.state.lock().unwrap().channel.receipts.is_empty() {
            assert!(tokio::time::Instant::now() < deadline);
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        tokio::time::sleep(Duration::from_millis(300)).await;
        unsafe {
            libc::kill(child.id().unwrap() as i32, libc::SIGTERM);
        }
        let output = result(child).await;
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["status"], "cancelled", "{value}");
        assert_eq!(value["cleanup"], "confirmed", "{value}");
        assert_eq!(value["cancel_acknowledged"], true);
        assert!(root.path().join("workspace/started").exists());
        tokio::time::sleep(Duration::from_millis(2200)).await;
        assert!(!root.path().join("workspace/escaped").exists());
    }
    #[tokio::test]
    async fn long_command_is_answered_once_and_its_receipt_follows_exit_promptly() {
        let (root, _server, oracle, child) = launch("channel_long").await;
        let output = tokio::time::timeout(Duration::from_secs(60), child.wait_with_output())
            .await
            .unwrap()
            .unwrap();
        completed(&output);
        let s = oracle.state.lock().unwrap();
        assert_eq!(
            s.channel.operations_sent.len(),
            1,
            "one call, not a chain of polling calls"
        );
        assert_eq!(s.channel.receipts.len(), 1);
        let payload = &s.channel.receipts[0]["payload"];
        assert_eq!(payload["status"], "exited");
        assert_eq!(payload["returncode"], 0);
        let exited = std::fs::metadata(root.path().join("workspace/finished"))
            .unwrap()
            .modified()
            .unwrap();
        let latency = s.channel.arrivals[0].duration_since(exited).unwrap();
        eprintln!(
            "receipt arrived {} ms after process exit",
            latency.as_millis()
        );
        assert!(latency < Duration::from_millis(300), "{latency:?}");
    }
}
