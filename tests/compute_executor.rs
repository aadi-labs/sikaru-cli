//! Native executor acceptance. `runtime::serve_with_options` runs in a child copy of this test
//! binary (see `in_process_executor_child`), so each case observes real stdout, stderr, exit
//! status and signals without a CLI serve command.
#![allow(dead_code)]
use std::process::Command;
#[test]
fn generated_api_commands_remain_available() {
    let output = Command::new(
        std::env::var("SIKARU_TEST_INSTALLED")
            .unwrap_or_else(|_| env!("CARGO_BIN_EXE_sikaru").to_owned()),
    )
    .args(["agent", "agents", "--help"])
    .output()
    .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("create_managed_session"));
}

#[cfg(unix)]
#[path = "support/git_remote.rs"]
mod git_remote;

#[cfg(unix)]
#[path = "../cli/sikaru/compute_checkpoint.rs"]
mod checkpoint;
#[cfg(unix)]
#[path = "../cli/sikaru/compute_checkpoint_repo.rs"]
mod checkpoint_repo;
#[cfg(unix)]
#[path = "../cli/sikaru/compute_config.rs"]
mod config;
#[cfg(unix)]
#[path = "../cli/sikaru/compute_journal.rs"]
mod journal;
#[cfg(unix)]
#[path = "../cli/sikaru/compute_process.rs"]
mod process;
#[cfg(unix)]
#[path = "../cli/sikaru/compute_runtime.rs"]
mod runtime;
#[cfg(unix)]
#[path = "../cli/sikaru/compute_transport.rs"]
mod transport;

/// Selects child mode and carries the fake gateway's base URL.
const CHILD_BASE_URL: &str = "SIKARU_TEST_EXECUTOR_BASE_URL";
const CHILD_SKIP_CHECKPOINTS: &str = "SIKARU_TEST_EXECUTOR_SKIP_CHECKPOINTS";

/// The executor process each case drives: a no-op unless the parent test selected child mode. It
/// reads the private bootstrap from stdin, serves the attachment in-process, prints the result and
/// exits with the native outcome code.
#[cfg(unix)]
#[test]
fn in_process_executor_child() {
    let Ok(base_url) = std::env::var(CHILD_BASE_URL) else {
        return;
    };
    let options = runtime::RunOptions {
        skip_checkpoints: std::env::var_os(CHILD_SKIP_CHECKPOINTS).is_some(),
        ..Default::default()
    };
    let served = tokio::runtime::Runtime::new().unwrap().block_on(async {
        let bootstrap: config::Bootstrap = serde_json::from_reader(std::io::stdin())?;
        let config = fern_cli_sdk::http::HttpConfig::new("sikaru")?;
        let http = transport::HttpIdentity::from_config(&config)?;
        runtime::serve_with_options(bootstrap, base_url, http, options).await
    });
    let result = served.unwrap_or_else(|_| {
        serde_json::json!({"status":"recovery_required","reason":"bootstrap_or_journal_rejected"})
    });
    println!("{result}");
    use std::io::Write;
    let _ = std::io::stdout().flush();
    std::process::exit(outcome_code(&result));
}

/// The documented native outcome codes.
fn outcome_code(result: &serde_json::Value) -> i32 {
    match result["status"].as_str() {
        Some("completed") => 0,
        Some("approval_required") => 2,
        Some("cancelled") => 3,
        Some("recovery_required") => 4,
        _ => 1,
    }
}

#[cfg(unix)]
mod wire {
    use super::git_remote::{self, WorkspaceUploads};
    use serde_json::{json, Value};
    use std::{
        process::Stdio,
        sync::{Arc, Mutex},
        time::Duration,
    };
    use tokio::io::AsyncWriteExt;
    use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};
    #[derive(Default)]
    struct State {
        policy_reads: usize,
        records: Vec<Value>,
        order: Vec<&'static str>,
        lease_lost: bool,
        record_attempts: usize,
        reconcile_at_refusal: usize,
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
        http_receipts: usize,
        http_user_agents: Vec<String>,
        failed_receipts: Vec<Value>,
        burst: usize,
        refused_at: Vec<std::time::Instant>,
        channel: ChannelState,
    }
    /// What the fake gateway saw on its executor channel endpoint.
    #[derive(Default)]
    struct ChannelState {
        user_agents: Vec<String>,
        connections: usize,
        refusals: usize,
        unauthorized: usize,
        heartbeats: usize,
        operations_sent: Vec<String>,
        receipts: Vec<Value>,
        arrivals: Vec<std::time::SystemTime>,
        resend: Option<Value>,
        switched: bool,
        /// A second operation has been pushed alongside the still-running first.
        second_pushed: bool,
    }
    #[derive(Clone)]
    struct Oracle {
        state: Arc<Mutex<State>>,
        scenario: &'static str,
        effect: std::path::PathBuf,
        git: Option<Arc<WorkspaceUploads>>,
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
            state.http_user_agents.push(
                request
                    .headers
                    .get("user-agent")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or_default()
                    .to_owned(),
            );
            let body: Value = serde_json::from_slice(&request.body).unwrap_or(Value::Null);
            let path = request.url.path();
            if let Some(response) = self.fault(&mut state, path, &body) {
                return response;
            }
            if path.ends_with("/workspace-policy") {
                if state.lease_lost {
                    return ResponseTemplate::new(409)
                        .set_body_json(json!({"detail":"lease_expired"}));
                }
                if self.git.is_some() {
                    assert!(request
                        .url
                        .query_pairs()
                        .any(|(k, v)| k == "run_id" && v == "run"));
                }
                state.policy_reads += 1;
                return match &self.git {
                    Some(_) => ok(
                        json!({"ignore":git_remote::IGNORE_DEFAULTS,"credentials":[".env", "*.pem"]}),
                    ),
                    None => ResponseTemplate::new(404),
                };
            }
            if path.ends_with("/workspace-content") {
                if self.scenario == "content_size_mismatch" {
                    use sha2::{Digest, Sha256};
                    return ok(
                        json!({"sha256":format!("{:x}",Sha256::digest(&request.body)), "size":request.body.len()+1}),
                    );
                }
                return self.git.as_ref().unwrap().upload(&request.body);
            }
            if path.ends_with("/workspace-checkpoints") {
                return self.record_checkpoint(&mut state, &body);
            }
            if path.ends_with("/work") {
                return self.work(&mut state);
            }
            if path.ends_with("/receipts") {
                return self.receipt(&mut state, body);
            }
            self.lifecycle(&mut state, path, body)
        }
    }
    impl Oracle {
        fn record_checkpoint(&self, s: &mut State, body: &Value) -> ResponseTemplate {
            s.record_attempts += 1;
            s.order.push("record");
            if self.scenario == "git_record_conflict" {
                return ResponseTemplate::new(409)
                    .set_body_json(json!({"detail":"turn_already_checkpointed"}));
            }
            if self.scenario == "git_not_quiescent" {
                if s.record_attempts == 1 {
                    s.reconcile_at_refusal = s.reconciliations.len();
                    return ResponseTemplate::new(409)
                        .set_body_json(json!({"detail":"workspace_not_quiescent"}));
                }
                assert!(
                    s.reconciliations.len() > s.reconcile_at_refusal,
                    "fresh reconciliation before retry"
                );
                let observations = s.reconciliations.last().unwrap()["processes"]
                    .as_array()
                    .unwrap();
                assert_eq!(observations.len(), 1);
                assert_eq!(observations[0]["handle_id"], s.handle);
                assert_eq!(observations[0]["status"], "exited");
            }
            self.git
                .as_ref()
                .unwrap()
                .record(body["manifest_digest"].as_str().unwrap());
            s.records.push(body.clone());
            ok(
                json!({"commit_sha":body["manifest_digest"],"branch":git_remote::BRANCH,"recorded_at":"2026-09-29T00:00:00Z"}),
            )
        }
        fn fault(&self, s: &mut State, path: &str, body: &Value) -> Option<ResponseTemplate> {
            if self.credential_revoked() {
                return Some(ResponseTemplate::new(401));
            }
            match (self.scenario, path.rsplit('/').next().unwrap()) {
                // More consecutive receipt failures than any fixed retry count.
                ("uploadfail", "receipts") if s.failed_receipts.len() < 6 => {
                    if let Some(first) = s.failed_receipts.first() {
                        assert_eq!(first, body);
                    }
                    s.failed_receipts.push(body.clone());
                    Some(ResponseTemplate::new(503))
                }
                // The service will never record the first receipt (a warm call that never parked).
                ("receipt_conflict", "receipts") if s.stage == 0 => {
                    s.handle = body["payload"]["id"].as_str().unwrap().into();
                    s.failed_receipts.push(body.clone());
                    s.receipts.push(body.clone());
                    s.stage = 1;
                    Some(ResponseTemplate::new(409).set_body_json(json!({"detail":"conflict"})))
                }
                // The first receipt is acknowledged without reaching the ledger; the operation has
                // since been issued durably, so the reconnecting executor must replay it.
                ("acked_issued", "receipts") if s.stage == 0 => {
                    s.handle = body["payload"]["id"].as_str().unwrap().into();
                    s.failed_receipts.push(body.clone());
                    s.stage = 1;
                    Some(ResponseTemplate::new(409).set_body_json(json!({"detail":"conflict"})))
                }
                ("acked_issued", "receipts") if body["tool_call_id"] == format!("{0:0128}", 0) => {
                    s.order.push(if s.ready {
                        "replay_after_ready"
                    } else {
                        "replay"
                    });
                    s.receipts.push(body.clone());
                    Some(ok(
                        json!({"created":true,"tool_call_id":body["tool_call_id"]}),
                    ))
                }
                // The connection drops once the refused receipt's operation is issued.
                ("acked_issued", "work") if s.ready && s.stage == 1 && !s.lost_issued => {
                    s.lost_issued = true;
                    Some(ResponseTemplate::new(503))
                }
                // The service cannot reconcile an issued operation whose result it never recorded.
                ("acked_issued", "reconcile") if s.lost_issued && s.receipts.is_empty() => Some(
                    ResponseTemplate::new(409).set_body_json(json!({"detail":"issued_unresolved"})),
                ),
                // Every receipt is refused while the service keeps issuing the same operation.
                ("refusal_loop", "receipts") => {
                    s.failed_receipts.push(body.clone());
                    s.refused_at.push(std::time::Instant::now());
                    Some(ResponseTemplate::new(409).set_body_json(json!({"detail":"conflict"})))
                }
                // After the first receipt, a 5xx burst across the poll and every reconnect step.
                ("transient_burst", "work" | "connect" | "reconcile" | "ready")
                    if s.stage >= 1 && s.burst < 6 =>
                {
                    s.burst += 1;
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
            let expiring = matches!(self.scenario, "expiry" | "git_lease_lost");
            let a = attachment("ready", if expiring { 1 } else { 3 });
            if path.ends_with("/ready") {
                s.ready = true;
                s.ready_body = Some(body);
                return ok(a);
            }
            if path.ends_with("/connect") {
                s.ready = false;
                let mut starting = attachment("starting", 3);
                starting["owner_epoch"] = a["owner_epoch"].clone();
                return ok(starting);
            }
            if path.ends_with("/heartbeat") {
                s.heartbeat += 1;
                if expiring {
                    s.lease_lost = true;
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
                s.order.push("cleanup");
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
            if self.scenario == "git_not_quiescent" && !s.handle.is_empty() {
                page["live_handles"] =
                    json!([{"handle_id":s.handle,"run_id":"run","tool_call_id":"start"}]);
            }
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
            if self.finished(s) {
                return self.completed_work(page);
            }
            if self.withholds_operation(s) {
                return ok(page);
            }
            let op = self.next(s);
            page["operations"] = json!([op]);
            ok(page)
        }
        fn completed_work(&self, mut page: Value) -> ResponseTemplate {
            page["execution"]["terminal"] = json!(true);
            page["execution"]["status"] = json!("completed");
            ok(page)
        }
        fn next(&self, s: &State) -> Value {
            if self.scenario == "altered" && s.stage == 1 {
                return operation(0, "bash.start", json!({"command":"echo twice >> effect"}));
            }
            if s.stage == 0 {
                let command=match self.scenario {
                    "credential_outage"|"renew_denied"|"credential_expiry"|"expiry"|"signal"|"revoked"|"git_lease_lost"=>"touch started; sleep 2; touch escaped",
                    "git_force_added"=>"mkdir -p .venv && echo pkg > .venv/pkg.py && git add -f .venv/pkg.py && git -c user.name=agent -c user.email=agent@example.com commit -qm agent && echo once >> effect; head -c 65536 /dev/zero",
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
    fn ok(value: Value) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(value)
    }
    struct Launch {
        root: tempfile::TempDir,
        path: std::path::PathBuf,
        server: MockServer,
        oracle: Oracle,
        base_url: String,
    }
    fn prepare_workspace(scenario: &str, ws: &std::path::Path) {
        match scenario {
            "git_task_repo" | "git_force_added" => git_remote::task_repository(ws),
            "git_split" => {
                for i in 0..6u8 {
                    std::fs::write(
                        ws.join(format!("part-{i}.bin")),
                        git_remote::noise(60_000, i),
                    )
                    .unwrap();
                }
                std::fs::write(ws.join("huge.bin"), git_remote::noise(300_000, 99)).unwrap();
            }
            _ => {}
        }
    }
    async fn prepare_launch(scenario: &'static str) -> Launch {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().canonicalize().unwrap();
        std::fs::create_dir(path.join("workspace")).unwrap();
        prepare_workspace(scenario, &path.join("workspace"));
        let git = match scenario.starts_with("git_") || scenario.starts_with("content_") {
            true => Some(Arc::new(WorkspaceUploads::new(
                if scenario == "git_slow_push" {
                    Some(Duration::from_secs(20))
                } else {
                    None
                },
                scenario == "content_mismatch",
            ))),
            false => None,
        };
        let server = MockServer::start().await;
        let oracle = Oracle {
            state: Default::default(),
            scenario,
            effect: path.join("workspace/effect"),
            git,
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
        Launch {
            root,
            path,
            server,
            oracle,
            base_url,
        }
    }
    async fn start(l: &Launch) -> tokio::process::Child {
        let command_timeout = if l.oracle.scenario == "channel_long" {
            60
        } else {
            10
        };
        spawn_executor(&l.path, &l.base_url, command_timeout, 1).await
    }
    async fn launch(
        scenario: &'static str,
    ) -> (tempfile::TempDir, MockServer, Oracle, tokio::process::Child) {
        let l = prepare_launch(scenario).await;
        let child = start(&l).await;
        (l.root, l.server, l.oracle, child)
    }
    fn lifecycle_events(output: &std::process::Output) -> Vec<Value> {
        git_remote::json_events(&output.stderr)
            .into_iter()
            .filter(|e| {
                e["event"].as_str().is_some_and(|n| {
                    (n.starts_with("checkpoint_") && n != "checkpoint_progress")
                        || n == "run_finished"
                })
            })
            .collect()
    }

    #[tokio::test]
    async fn quiescence_refusal_reconciles_real_terminal_process_state_before_one_retry() {
        let l = prepare_launch("git_not_quiescent").await;
        let output = result(start(&l).await).await;
        completed(&output);
        let state = l.oracle.state.lock().unwrap();
        assert_eq!(state.record_attempts, 2);
        assert_eq!(state.records.len(), 1);
        assert_eq!(state.reconciliations.len(), state.reconcile_at_refusal + 1);
        assert_eq!(state.order, vec!["record", "record", "cleanup"]);
        assert!(l.oracle.git.as_ref().unwrap().head().is_some());
    }
    #[tokio::test]
    async fn manifest_checkpoint_preserves_working_tree_and_task_repository() {
        let l = prepare_launch("git_task_repo").await;
        let ws = l.path.join("workspace");
        let before = git_remote::snapshot(&ws.join(".git"));
        let output = result(start(&l).await).await;
        completed(&output);
        assert_eq!(
            git_remote::snapshot(&ws.join(".git")),
            before,
            "the task repository is read-only"
        );
        assert_eq!(
            git_remote::git(&ws, &["diff", "--cached", "--name-only"]),
            b"staged.txt\n"
        );
        let git = l.oracle.git.as_ref().unwrap();
        let head = git.head().expect("one checkpoint pushed");
        let events = lifecycle_events(&output);
        let names: Vec<_> = events
            .iter()
            .map(|e| e["event"].as_str().unwrap())
            .collect();
        assert_eq!(
            names,
            vec!["run_finished", "checkpoint_started", "checkpoint_pushed"]
        );
        assert_eq!(events[2]["commit_sha"], head);
        assert_eq!(
            events[2]["skipped"]["link_outside_workspace"], 2,
            "escaping links are skipped, not fatal"
        );
        let tree: std::collections::BTreeMap<_, _> = git
            .tree(&head)
            .into_iter()
            .map(|(mode, path)| (path, mode))
            .collect();
        assert_eq!(git.read(&head, "tracked.txt"), b"working copy");
        assert_eq!(git.read(&head, "result.txt"), b"finished");
        assert_eq!(tree["alias"], "120000");
        assert_eq!(tree["run.sh"], "100755");
        for absent in [
            "app.log",
            ".venv/lib/site.py",
            "node_modules/left-pad/index.js",
            "escape",
            "absolute",
            ".env",
            "deploy.pem",
        ] {
            assert!(!tree.contains_key(absent), "{absent}");
        }
        assert!(!tree.contains_key(".env.example"));
        assert!(tree.values().all(|mode| mode != "160000"));
        let s = l.oracle.state.lock().unwrap();
        assert_eq!(s.records.len(), 1);
        assert_eq!(s.records[0]["manifest_digest"], head);
        assert_eq!(s.records[0]["trigger"], "turn");
        assert_eq!(s.records[0]["run_id"], "run", "the completed run");
        assert_eq!(
            s.order,
            vec!["record", "cleanup"],
            "recorded under the lease, before cleanup"
        );
    }
    #[tokio::test]
    async fn force_added_ignored_paths_never_reach_manifest() {
        let l = prepare_launch("git_force_added").await;
        completed(&result(start(&l).await).await);
        let git = l.oracle.git.as_ref().unwrap();
        let head = git.head().unwrap();
        assert!(git.tree(&head).iter().all(|(_, p)| !p.starts_with(".venv/")
            && !p.starts_with("node_modules/")
            && !p.starts_with(".git/")));
    }
    #[tokio::test]
    async fn uploads_all_admitted_blobs_before_recording_manifest() {
        let l = prepare_launch("git_split").await;
        let output = result(start(&l).await).await;
        completed(&output);
        let git = l.oracle.git.as_ref().unwrap();
        let head = git.head().unwrap();
        assert_eq!(
            git.tree(&head)
                .iter()
                .filter(|(_, p)| p.starts_with("part-"))
                .count(),
            6
        );
        let s = l.oracle.state.lock().unwrap();
        assert_eq!(s.records.len(), 1);
        assert_eq!(s.records[0]["manifest_digest"], head);
        assert_eq!(head.len(), 64);
    }
    #[tokio::test]
    async fn mismatched_upload_receipt_never_records_or_acknowledges_checkpoint() {
        for scenario in ["content_mismatch", "content_size_mismatch"] {
            let l = prepare_launch(scenario).await;
            let output = result(start(&l).await).await;
            completed(&output);
            assert!(l.oracle.state.lock().unwrap().records.is_empty());
            let events = lifecycle_events(&output);
            assert!(events
                .iter()
                .any(|e| e["event"] == "checkpoint_failed" && e["reason"] == "content_mismatch"));
            assert!(!events.iter().any(|e| e["event"] == "checkpoint_pushed"));
        }
    }
    #[tokio::test]
    async fn expired_lease_cannot_upload_or_acknowledge_a_new_checkpoint() {
        let l = prepare_launch("git_lease_lost").await;
        let output = result(start(&l).await).await;
        assert_eq!(output.status.code(), Some(4));
        let state = l.oracle.state.lock().unwrap();
        assert!(state.lease_lost);
        assert!(state.records.is_empty());
        assert_eq!(
            l.oracle
                .git
                .as_ref()
                .unwrap()
                .seen
                .lock()
                .unwrap()
                .uploads_started,
            0
        );
        let events = lifecycle_events(&output);
        assert!(events.iter().any(|e| e["event"] == "checkpoint_failed"));
        assert!(!events.iter().any(|e| e["event"] == "checkpoint_pushed"));
    }
    #[tokio::test]
    async fn a_refused_record_is_reported_without_changing_the_result() {
        let l = prepare_launch("git_record_conflict").await;
        let output = result(start(&l).await).await;
        completed(&output);
        let failed = lifecycle_events(&output)
            .into_iter()
            .find(|e| e["event"] == "checkpoint_failed")
            .expect("failure reported");
        assert_eq!(failed["reason"], "turn_already_checkpointed");
    }
    #[tokio::test]
    async fn sigterm_during_a_checkpoint_push_reports_failure_and_exits_promptly() {
        let l = prepare_launch("git_slow_push").await;
        let child = start(&l).await;
        let git = l.oracle.git.clone().unwrap();
        tokio::time::timeout(Duration::from_secs(20), async {
            while git.seen.lock().unwrap().uploads_started == 0 {
                tokio::time::sleep(Duration::from_millis(20)).await;
            }
        })
        .await
        .unwrap();
        let signalled = std::time::Instant::now();
        unsafe {
            libc::kill(child.id().unwrap() as i32, libc::SIGTERM);
        }
        let output = tokio::time::timeout(Duration::from_secs(15), served(child))
            .await
            .unwrap();
        assert!(
            signalled.elapsed() < Duration::from_secs(10),
            "{:?}",
            signalled.elapsed()
        );
        completed(&output);
        assert!(
            output.status.success(),
            "a failed checkpoint keeps the result's exit status"
        );
        let failed = lifecycle_events(&output)
            .into_iter()
            .find(|e| e["event"] == "checkpoint_failed")
            .expect("failure reported");
        assert_eq!(failed["reason"], "signal");
        let s = l.oracle.state.lock().unwrap();
        assert!(s.records.is_empty());
        assert!(
            s.heartbeat >= 2,
            "lease keeps renewing during the slow checkpoint"
        );
        assert!(
            s.cleanup.is_some(),
            "cleanup still runs after an interrupted checkpoint"
        );
    }
    #[tokio::test]
    async fn disabled_checkpoints_skip_checkpoint_remote_and_private_repository() {
        let l = prepare_launch("git_task_repo").await;
        let child = spawn_executor_with(&l.path, &l.base_url, 10, 1, true).await;
        let output = result(child).await;
        completed(&output);
        assert_eq!(
            lifecycle_events(&output),
            vec![json!({"event":"checkpoint_skipped","reason":"disabled"})]
        );
        let state = l.oracle.state.lock().unwrap();
        assert_eq!((state.policy_reads, state.records.len()), (0, 0));
        assert!(state.cleanup.is_some());
        assert!(!l.path.join("state/workspace.git").exists());
        let git = l.oracle.git.as_ref().unwrap();
        assert!(git.head().is_none());
        assert_eq!(git.seen.lock().unwrap().uploads_started, 0);
    }
    async fn spawn_executor(
        path: &std::path::Path,
        base_url: &str,
        command_timeout: u64,
        owner_epoch: i64,
    ) -> tokio::process::Child {
        spawn_executor_with(path, base_url, command_timeout, owner_epoch, false).await
    }
    async fn spawn_executor_with(
        path: &std::path::Path,
        base_url: &str,
        command_timeout: u64,
        owner_epoch: i64,
        skip_checkpoints: bool,
    ) -> tokio::process::Child {
        let bootstrap = json!({"project_id":"project","session_id":"session","attachment_id":"attachment","owner_epoch":owner_epoch,"workspace_generation":"generation","journal_id":"journal","credential_id":"credential",
            "token":"restricted-executor-secret","workspace_provenance":{"kind":"existing_directory","identity":"original"},"workspace":path.join("workspace"),"state_dir":path.join("state"),"command_timeout_seconds":command_timeout});
        let mut command = tokio::process::Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "in_process_executor_child",
                "--nocapture",
                "--test-threads=1",
                "-q",
            ])
            .env(super::CHILD_BASE_URL, base_url)
            .env("SIKARU_API_KEY", "controller-secret")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);
        if skip_checkpoints {
            command.env(super::CHILD_SKIP_CHECKPOINTS, "1");
        }
        let mut child = command.spawn().unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(bootstrap.to_string().as_bytes())
            .await
            .unwrap();
        child
    }
    async fn result(child: tokio::process::Child) -> std::process::Output {
        tokio::time::timeout(Duration::from_secs(30), served(child))
            .await
            .unwrap()
    }
    /// The child's output, its stdout reduced to the result line the executor printed after the
    /// test harness banner.
    async fn served(child: tokio::process::Child) -> std::process::Output {
        let mut output = child.wait_with_output().await.unwrap();
        let printed = String::from_utf8_lossy(&output.stdout).into_owned();
        let last = printed.lines().rev().find(|line| !line.trim().is_empty());
        output.stdout = last.unwrap_or_default().as_bytes().to_vec();
        output
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
        let capabilities = state.ready_body.as_ref().unwrap()["capabilities"]
            .as_array()
            .unwrap();
        assert!(!capabilities.contains(&json!("filesystem-checkpoint-v1")));
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
    async fn receipt_upload_failures_retry_the_immutable_result_without_reexecution() {
        let (root, _server, oracle, child) = launch("uploadfail").await;
        completed(&result(child).await);
        {
            let s = oracle.state.lock().unwrap();
            assert_eq!(s.failed_receipts.len(), 6);
            assert_eq!(
                s.receipts[0], s.failed_receipts[0],
                "the same receipt, retried"
            );
            assert_eq!(s.receipts.len(), 4);
        }
        let journal = std::fs::read_to_string(root.path().join("state/journal.jsonl")).unwrap();
        let receipts = journal
            .lines()
            .filter_map(|line| serde_json::from_str::<Value>(line).ok())
            .filter(|v| v["record"] == "Receipt")
            .count();
        assert_eq!(receipts, 4, "one immutable receipt per operation");
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
    }
    #[tokio::test]
    async fn a_refused_receipt_settles_delivery_without_retry_or_reexecution() {
        let (root, _server, oracle, child) = launch("receipt_conflict").await;
        completed(&result(child).await);
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
        let s = oracle.state.lock().unwrap();
        assert_eq!(
            s.failed_receipts.len(),
            1,
            "a refused receipt is never re-sent"
        );
        assert_eq!(s.receipts.len(), 4);
    }
    #[tokio::test]
    async fn a_receipt_refused_while_reissued_ends_the_run_after_paced_refusals() {
        let (_root, _server, oracle, child) = launch("refusal_loop").await;
        let output = result(child).await;
        let value: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(value["status"], "recovery_required", "{value}");
        assert_eq!(
            value["reason"],
            "receipt_delivery_failed: immutable receipt retained"
        );
        assert_eq!(value["cleanup"], "confirmed", "{value}");
        let s = oracle.state.lock().unwrap();
        assert_eq!(
            s.failed_receipts.len(),
            3,
            "the run gives up after 3 refusals"
        );
        assert!(s.failed_receipts.iter().all(|r| r == &s.failed_receipts[0]));
        let paced = s.refused_at[2].duration_since(s.refused_at[0]);
        assert!(
            paced >= Duration::from_millis(1800),
            "refusals at the idle pace: {paced:?}"
        );
    }
    #[tokio::test]
    async fn an_acknowledged_receipt_for_an_issued_operation_is_replayed_on_reconnect() {
        let (root, _server, oracle, child) = launch("acked_issued").await;
        completed(&result(child).await);
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
        let s = oracle.state.lock().unwrap();
        assert!(s.lost_issued, "the connection dropped after the refusal");
        assert_eq!(
            s.order,
            vec!["replay", "cleanup"],
            "replayed once, while reconnecting"
        );
        assert_eq!(
            s.receipts[0], s.failed_receipts[0],
            "the same immutable receipt"
        );
        assert!(s
            .reconciliations
            .iter()
            .all(|r| r["uncertain_operation_ids"] == json!([])));
        assert_eq!(s.receipts.len(), 4);
    }
    #[tokio::test]
    async fn transient_failures_on_poll_and_reconnect_do_not_end_the_run() {
        let (root, _server, oracle, child) = launch("transient_burst").await;
        completed(&result(child).await);
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
        let s = oracle.state.lock().unwrap();
        assert_eq!(s.burst, 6, "the whole burst was absorbed");
        assert_eq!(s.receipts.len(), 4);
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
                s.channel.user_agents.push(
                    request
                        .headers()
                        .get("user-agent")
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or_default()
                        .to_owned(),
                );
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
                if self.scenario == "channel_duplicate" {
                    // The same operation again while the first delivery is still running.
                    tokio::time::sleep(Duration::from_millis(300)).await;
                    let again = {
                        let mut s = self.state.lock().unwrap();
                        let op = self.channel_operation(&s);
                        self.page_with(&mut s, op)
                    };
                    send(&mut sink, operations(again)).await;
                }
                if self.scenario == "channel_concurrent" {
                    // A second warm call arrives while the first is still running.
                    tokio::time::sleep(Duration::from_millis(300)).await;
                    let both = {
                        let mut s = self.state.lock().unwrap();
                        s.channel.second_pushed = true;
                        self.unanswered_page(&mut s)
                    };
                    send(&mut sink, operations(both)).await;
                }
                // Like the gateway, heartbeat while waiting: the executor treats silence as loss.
                let mut beat = tokio::time::interval(Duration::from_millis(300));
                loop {
                    let message = tokio::select! {
                        message = stream.next() => message,
                        _ = beat.tick() => {
                            send(&mut sink, json!({"type":"heartbeat"})).await;
                            continue;
                        }
                    };
                    let Some(Ok(message)) = message else {
                        return;
                    };
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
                // A duplicate delivery is answered before the turn moves on.
                let hold = self.scenario == "channel_duplicate" && created;
                let next = (!close && !hold).then(|| self.channel_page(&mut s));
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
                if self.scenario == "channel_concurrent" {
                    return self.unanswered_page(s);
                }
                let op = self.channel_operation(s);
                self.page_with(s, op)
            }
            /// Every pushed operation without a recorded receipt, as the gateway repeats them.
            fn unanswered_page(&self, s: &mut State) -> Value {
                let long = operation(
                    0,
                    "bash.run",
                    json!({"command":"sleep 3; echo once >> long","yield_seconds":10,"limit":8192}),
                );
                let quick = operation(
                    1,
                    "bash.run",
                    json!({"command":"echo once >> quick","yield_seconds":5,"limit":8192}),
                );
                let pushed = match s.channel.second_pushed {
                    true => vec![long, quick],
                    false => vec![long],
                };
                let unanswered = pushed
                    .into_iter()
                    .filter(|op| {
                        !s.receipts
                            .iter()
                            .any(|r| r["tool_call_id"] == op["tool_call_id"])
                    })
                    .collect::<Vec<_>>();
                for op in &unanswered {
                    s.channel
                        .operations_sent
                        .push(op["tool_call_id"].as_str().unwrap().into());
                }
                let mut page = running_page();
                page["transport"] = json!("channel");
                page["operations"] = json!(unanswered);
                page
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
                    ("channel_duplicate", _) => operation(
                        0,
                        "bash.run",
                        json!({"command":"sleep 1; echo once >> effect","yield_seconds":5,"limit":8192}),
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
        fn finished(&self, s: &State) -> bool {
            // A refused receipt never clears: the turn never finishes while it is refused.
            s.refused_at.is_empty() && s.stage >= self.final_stage()
        }
        fn final_stage(&self) -> usize {
            if matches!(self.scenario, "channel_long" | "channel_duplicate") {
                1
            } else if self.scenario == "channel_concurrent" {
                2
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
    async fn channel_upgrade_sends_the_same_user_agent_as_http_requests() {
        let (_root, _server, oracle, child) = launch("channel").await;
        completed(&result(child).await);
        let s = oracle.state.lock().unwrap();
        let http: std::collections::BTreeSet<_> = s.http_user_agents.iter().cloned().collect();
        assert_eq!(http.len(), 1, "{http:?}");
        let expected = http.iter().next().unwrap();
        assert!(expected.starts_with("sikaru-cli/"), "{expected}");
        assert!(!s.channel.user_agents.is_empty());
        assert!(
            s.channel.user_agents.iter().all(|agent| agent == expected),
            "{:?}",
            s.channel.user_agents
        );
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
    async fn duplicate_delivery_of_a_running_operation_runs_once_and_answers_both() {
        let (root, _server, oracle, child) = launch("channel_duplicate").await;
        completed(&result(child).await);
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
        let s = oracle.state.lock().unwrap();
        assert_eq!(s.channel.operations_sent.len(), 2);
        assert_eq!(s.channel.operations_sent[0], s.channel.operations_sent[1]);
        assert_eq!(s.channel.receipts.len(), 2, "both deliveries are answered");
        assert_eq!(s.channel.receipts[0], s.channel.receipts[1]);
        assert_eq!(s.receipts.len(), 1);
    }
    #[tokio::test]
    async fn refused_channel_falls_back_to_polling() {
        let (root, _server, oracle, child) = launch("channel_refused").await;
        let output = result(child).await;
        completed(&output);
        let fallbacks = git_remote::json_events(&output.stderr)
            .into_iter()
            .filter(|e| e["event"] == "executor_transport")
            .collect::<Vec<_>>();
        assert_eq!(
            fallbacks,
            vec![
                json!({"event":"executor_transport","transport":"poll","reason":"channel_unavailable"})
            ]
        );
        assert_eq!(
            std::fs::read_to_string(root.path().join("workspace/effect")).unwrap(),
            "once\n"
        );
        let s = oracle.state.lock().unwrap();
        assert!(s.channel.refusals >= 1);
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
    async fn an_operation_pushed_during_a_long_command_is_answered_before_it_finishes() {
        let (root, _server, oracle, child) = launch("channel_concurrent").await;
        completed(&result(child).await);
        let workspace = root.path().join("workspace");
        assert_eq!(
            std::fs::read_to_string(workspace.join("long")).unwrap(),
            "once\n"
        );
        assert_eq!(
            std::fs::read_to_string(workspace.join("quick")).unwrap(),
            "once\n"
        );
        let s = oracle.state.lock().unwrap();
        let order = s
            .receipts
            .iter()
            .map(|r| r["tool_call_id"].as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            order,
            vec![format!("{:0128}", 1), format!("{:0128}", 0)],
            "the second operation's receipt arrives while the first still runs"
        );
        let mut long_receipts = s
            .channel
            .receipts
            .iter()
            .filter(|r| r["tool_call_id"] == format!("{:0128}", 0));
        assert!(
            long_receipts.all(|r| r == &s.receipts[1]),
            "a repeat of the running operation gets the same receipt"
        );
    }
    #[tokio::test]
    async fn long_command_is_answered_once_and_its_receipt_follows_exit_promptly() {
        let (root, _server, oracle, child) = launch("channel_long").await;
        let output = tokio::time::timeout(Duration::from_secs(60), served(child))
            .await
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
