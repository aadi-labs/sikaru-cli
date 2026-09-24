use std::process::Command;
#[test]
fn primary_binary_has_fresh_exec_and_explicit_resume() {
    let out = Command::new(
        std::env::var("SIKARU_TEST_INSTALLED")
            .unwrap_or_else(|_| env!("CARGO_BIN_EXE_sikaru").to_owned()),
    )
    .args(["exec", "--help"])
    .output()
    .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let help = String::from_utf8_lossy(&out.stdout);
    assert!(help.contains("--workspace") && help.contains("--resume"));
}
#[cfg(unix)]
mod workflow {
    use serde_json::{json, Value};
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
        time::Duration,
    };
    use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};
    #[derive(Default)]
    struct Server {
        sessions: usize,
        models: Vec<Value>,
        environments: usize,
        attachments: HashMap<String, Value>,
        ready: bool,
        turns: usize,
        receipts: usize,
        polls: usize,
        first_poll: Option<std::time::Instant>,
        epoch: i64,
        credential: usize,
    }
    #[derive(Clone)]
    struct Oracle {
        state: Arc<Mutex<Server>>,
        scenario: &'static str,
    }
    fn ok(v: Value) -> ResponseTemplate {
        ResponseTemplate::new(200).set_body_json(v)
    }
    impl Respond for Oracle {
        fn respond(&self, r: &Request) -> ResponseTemplate {
            let body: Value = serde_json::from_slice(&r.body).unwrap_or(Value::Null);
            let path = r.url.path();
            if let Some(response) = self.fault(path) {
                return response;
            }
            let mut s = self.state.lock().unwrap();
            if path.ends_with("/execution-sessions") {
                s.sessions += 1;
                s.models.push(body["model"].clone());
                return ok(json!({"session":{"id":format!("session-{}",s.sessions)}}));
            }
            if path.ends_with("/compute-environments") {
                return self.environment(&mut s, &body);
            }

            if path.ends_with("/compute-attachments") {
                let id = format!("attachment-{}", s.sessions);
                let a = json!({"id":id,"session_id":format!("session-{}",s.sessions),"project_id":"project","environment_id":"environment","provider_id":"provider","workspace_generation":format!("generation-{}",s.sessions),"workspace_provenance":body["workspace_provenance"],"journal_id":format!("journal-{}",s.sessions),"status":"pending","owner_id":"controller","owner_epoch":0,"lease_until":2000000000.0,"startup_deadline":2000000000.0,"lease_ttl_seconds":60,"capabilities":["compute.execute"],"protocol_version":"sikaru-compute-v1","cleanup_status":"unconfirmed","uncertain_operations":[],"processes":[]});
                s.attachments.insert(id, a.clone());
                s.ready = false;
                s.turns = 0;
                s.receipts = 0;
                s.polls = 0;
                return ok(a);
            }
            if path.ends_with("/turns") {
                assert!(s.ready, "turn before readiness");
                assert!(!body["input"]["goal"].as_str().unwrap().is_empty());
                assert_eq!(body["capability_grants"], json!(["compute.execute"]));
                s.turns += 1;
                if self.scenario == "completedresume" {
                    s.receipts = 0;
                }
                return ok(json!({"run":{"id":self.run_id(&s)}}));
            }
            if path.ends_with("/events") {
                return ok(
                    json!({"events":[{"id":"answer","sequence":1,"eventType":"run.completed","createdAt":"now","payload":{"output":"wrote output"}}],"nextAfter":1}),
                );
            }
            if path.contains("/runs/") {
                return ok(
                    json!({"runId":"run","status":"completed","eventsUrl":"events","harnessId":"agent","harnessVersionId":"version"}),
                );
            }
            self.lifecycle(r, &mut s, path, body)
        }
    }
    impl Oracle {
        fn environment(&self, s: &mut Server, body: &Value) -> ResponseTemplate {
            s.environments += 1;
            if self.scenario == "partialsetup" && s.environments == 1 {
                return ResponseTemplate::new(503);
            }
            ok(
                json!({"id":"environment","status":"active","project_id":"project","product_id":"product","environment_slug":body["environment_slug"]}),
            )
        }
        fn lifecycle(
            &self,
            r: &Request,
            s: &mut Server,
            path: &str,
            body: Value,
        ) -> ResponseTemplate {
            if path.ends_with("/renew") {
                return ok(
                    json!({"credential_id":format!("credential-{}",s.credential),"expires_at":2000000000.0}),
                );
            }
            if path.ends_with("/revoke") {
                return ok(json!({"credential_id":"credential","revoked":true}));
            }
            let id = path.split('/').nth(5).unwrap();
            let mut a = s.attachments.get(id).expect(path).clone();
            let suffix = path.rsplit('/').next().unwrap();
            match suffix {
                "claim" => {
                    s.ready = false;
                    s.epoch += 1;
                    a["owner_epoch"] = json!(s.epoch);
                    a["status"] = json!("starting");
                    s.attachments.insert(id.to_owned(), a);
                    ok(
                        json!({"id":"claim","attachment_id":id,"owner_id":"controller","owner_epoch":s.epoch,"lease_until":2000000000.0,"startup_ttl_seconds":180,"status":"claimed"}),
                    )
                }
                "credentials" => {
                    s.credential += 1;
                    ok(
                        json!({"credential_id":format!("credential-{}",s.credential),"token":"scoped","expires_at":2000000000.0}),
                    )
                }
                "status" | "connect" | "heartbeat" => ok(a),
                "ready" => {
                    s.ready = true;
                    a["status"] = json!("ready");
                    s.attachments.insert(id.to_owned(), a.clone());
                    ok(a)
                }
                "reconcile" => ok(json!({"attachment":a,"receipts":[]})),
                "work" => self.work(s, a),
                "receipts" => {
                    assert_eq!(r.headers.get("authorization").unwrap(), "Bearer scoped");
                    assert_eq!(body["status"], "completed", "{body}");
                    s.receipts += 1;
                    ok(
                        json!({"created":true,"tool_call_id":body["tool_call_id"],"status":"accepted"}),
                    )
                }
                "cleanup" => {
                    a["status"] = json!("cleaned");
                    a["cleanup_status"] = json!("confirmed");
                    s.attachments.insert(id.to_owned(), a.clone());
                    ok(a)
                }
                "stop" => {
                    a["status"] = json!("stopping");
                    ok(a)
                }
                _ => {
                    let response = ok(a);
                    if self.scenario == "completedresume" && s.ready && s.turns == 1 {
                        response.set_delay(Duration::from_millis(200))
                    } else {
                        response
                    }
                }
            }
        }
    }
    impl Oracle {
        fn fault(&self, path: &str) -> Option<ResponseTemplate> {
            match (self.scenario, path.rsplit('/').next().unwrap()) {
                ("billing", "execution-sessions") => {
                    Some(ResponseTemplate::new(402).set_body_string("secret-provider-token"))
                }
                ("startupfail", "status") | ("cleanupfail", "cleanup") => {
                    Some(ResponseTemplate::new(503))
                }
                _ => None,
            }
        }
        fn run_id(&self, s: &Server) -> String {
            if self.scenario == "completedresume" {
                format!("run-{}", s.turns)
            } else {
                "run".into()
            }
        }
        fn task_text(&self, s: &Server) -> &str {
            if self.scenario == "completedresume" && s.turns == 2 {
                "second effect"
            } else {
                "native effect"
            }
        }
        fn approval(&self, s: &Server) -> bool {
            (self.scenario == "approval" && s.epoch == 1)
                || (matches!(self.scenario, "late" | "cleared")
                    && s.first_poll.unwrap().elapsed() < Duration::from_millis(300))
        }
        fn work(&self, s: &mut Server, a: Value) -> ResponseTemplate {
            let mut page = json!({"attachment":a,"execution_phase":"running","execution":null,"operations":[],"issued_operations":[],"live_handles":[],"poll_after_seconds":1});
            if !s.ready || s.turns == 0 {
                return ok(page);
            }
            s.first_poll.get_or_insert_with(std::time::Instant::now);
            s.polls += 1;
            if self.scenario == "hang" {
                return ok(page);
            }
            if self.scenario == "failed" {
                page["execution"] = json!({"run_id":"run","status":"failed","terminal":true,"approval_required":false});
                return ok(page);
            }
            let approval = self.approval(s);
            page["execution"] = json!({"run_id":self.run_id(s),"status":if s.receipts>0 {"completed"}else{"running"},"terminal":s.receipts>0,"approval_required":approval});
            if self.scenario == "cleared" {
                return ok(page);
            }
            if !approval && s.receipts == 0 {
                page["operations"] = json!([{"run_id":self.run_id(s),"tool_call_id":"write","tool_provider_id":"provider","capability_name":"compute.execute","method":"workspace.write_text","arguments":{"path":"output.txt","text":self.task_text(s)},"request_digest":"digest","owner_epoch":s.epoch,"workspace_generation":page["attachment"]["workspace_generation"]}]);
            }
            ok(page)
        }
    }
    async fn launch(
        server: &MockServer,
        workspace: &std::path::Path,
        extra: &[&str],
    ) -> (i32, Value) {
        let output = tokio::time::timeout(
            Duration::from_secs(35),
            tokio::process::Command::new(
                std::env::var("SIKARU_TEST_INSTALLED")
                    .unwrap_or_else(|_| env!("CARGO_BIN_EXE_sikaru").to_owned()),
            )
            .env("SIKARU_API_KEY", "controller-secret")
            .env("PATH", "/no-python-runtime")
            .args([
                "--base-url",
                &server.uri(),
                "exec",
                "--project",
                "project",
                "--workspace",
                workspace.to_str().unwrap(),
            ])
            .args(extra)
            .output(),
        )
        .await
        .expect("workflow hung")
        .unwrap();
        let value = serde_json::from_slice(&output.stdout).unwrap_or_else(|e| {
            panic!(
                "{e}: stdout {} stderr {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        });
        (output.status.code().unwrap(), value)
    }
    async fn setup(scenario: &'static str) -> (MockServer, Arc<Mutex<Server>>, tempfile::TempDir) {
        let server = MockServer::start().await;
        let state = Arc::new(Mutex::new(Server::default()));
        Mock::given(wiremock::matchers::any())
            .respond_with(Oracle {
                state: state.clone(),
                scenario,
            })
            .mount(&server)
            .await;
        (server, state, tempfile::tempdir().unwrap())
    }
    #[tokio::test]
    async fn missing_prompt_fails_before_creating_state_or_remote_session() {
        let (server, state, dir) = setup("completed").await;
        let (code, result) = launch(&server, dir.path(), &["--agent", "agent"]).await;
        assert_eq!(code, 1);
        assert_eq!(result["reason"], "invalid_input");
        assert_eq!(state.lock().unwrap().sessions, 0);
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
    }

    #[tokio::test]
    async fn positional_prompt_and_environment_defaults_complete_in_current_directory() {
        let (server, state, dir) = setup("completed").await;
        let output = tokio::process::Command::new(env!("CARGO_BIN_EXE_sikaru"))
            .current_dir(dir.path())
            .env("SIKARU_API_KEY", "controller-secret")
            .env("SIKARU_PROJECT", "project")
            .env("SIKARU_AGENT", "agent")
            .args([
                "--base-url",
                &server.uri(),
                "exec",
                "--model",
                "kimi-k3",
                "Write output",
            ])
            .output()
            .await
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let result: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(result["status"], "completed");
        assert_eq!(state.lock().unwrap().sessions, 1);
        assert_eq!(state.lock().unwrap().models, vec![json!("kimi-k3")]);
        assert_eq!(
            std::fs::read_to_string(dir.path().join("output.txt")).unwrap(),
            "native effect"
        );
    }

    #[tokio::test]
    async fn conflicting_resume_model_is_rejected_without_creating_another_session() {
        let (server, state, dir) = setup("completed").await;
        let (code, result) = launch(
            &server,
            dir.path(),
            &["--agent", "agent", "--model", "kimi-k3", "--prompt", "task"],
        )
        .await;
        assert_eq!(code, 0, "{result}");
        let (code, _) = launch(
            &server,
            dir.path(),
            &[
                "--resume",
                result["state_dir"].as_str().unwrap(),
                "--model",
                "glm-5p3-flash",
                "--prompt",
                "next",
            ],
        )
        .await;
        assert_ne!(code, 0);
        assert_eq!(state.lock().unwrap().sessions, 1);
    }

    #[tokio::test]
    async fn billing_failure_provides_advice_without_exposing_response_body() {
        let (server, _, dir) = setup("billing").await;
        let (code, result) = launch(
            &server,
            dir.path(),
            &["--agent", "agent", "--prompt", "task"],
        )
        .await;
        assert_eq!(code, 4);
        assert!(
            result["help"].as_str().unwrap().contains("subscription"),
            "{result}"
        );
        assert!(!result.to_string().contains("secret-provider-token"));
        assert!(result["state_dir"].is_string());
    }

    #[tokio::test]
    async fn piped_task_executes_once_and_conflicting_inputs_are_rejected() {
        use tokio::io::AsyncWriteExt;
        let (server, state, dir) = setup("normal").await;
        let mut child = tokio::process::Command::new(env!("CARGO_BIN_EXE_sikaru"))
            .env("SIKARU_API_KEY", "controller-secret")
            .args([
                "--base-url",
                &server.uri(),
                "exec",
                "--project",
                "project",
                "--agent",
                "agent",
                "--workspace",
                dir.path().to_str().unwrap(),
                "-",
            ])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(b"Write the output")
            .await
            .unwrap();
        let output = child.wait_with_output().await.unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(state.lock().unwrap().turns, 1);
        let output = tokio::process::Command::new(env!("CARGO_BIN_EXE_sikaru"))
            .args([
                "exec",
                "--project",
                "project",
                "--agent",
                "agent",
                "task",
                "--prompt",
                "other",
            ])
            .output()
            .await
            .unwrap();
        assert!(!output.status.success());
    }

    #[tokio::test]
    async fn terminal_exec_continues_one_session_for_two_turns() {
        terminal_task("exec", false).await;
    }

    #[tokio::test]
    async fn chat_alias_continues_the_same_session() {
        terminal_task("chat", false).await;
    }

    #[tokio::test]
    async fn terminal_print_exits_after_one_task() {
        terminal_task("exec", true).await;
    }

    async fn terminal_task(command: &str, print: bool) {
        use std::io::Write;
        use std::os::fd::FromRawFd;
        let (server, state, dir) = setup("completedresume").await;
        let (mut master, mut slave) = (0, 0);
        assert_eq!(
            unsafe {
                libc::openpty(
                    &mut master,
                    &mut slave,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            },
            0
        );
        let mut input = unsafe { std::fs::File::from_raw_fd(master) };
        let terminal = unsafe { std::fs::File::from_raw_fd(slave) };
        let child = tokio::process::Command::new(env!("CARGO_BIN_EXE_sikaru"))
            .env("SIKARU_API_KEY", "controller-secret")
            .args([
                "--base-url",
                &server.uri(),
                command,
                "--model",
                "kimi-k3",
                "--project",
                "project",
                "--agent",
                "agent",
                "--workspace",
                dir.path().to_str().unwrap(),
            ])
            .args(if print {
                vec!["--print", "first task"]
            } else {
                vec![]
            })
            .stdin(terminal)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        if !print {
            input
                .write_all(b"first task\nsecond task\n/exit\n")
                .unwrap();
        }
        let output = tokio::time::timeout(Duration::from_secs(35), child.wait_with_output())
            .await
            .unwrap()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let progress = String::from_utf8_lossy(&output.stderr);
        if print {
            assert!(!progress.contains("You>"));
            assert!(progress.contains("turn_submitted"));
        } else {
            assert!(progress.contains("You>"));
            assert!(progress.contains("Working…"));
            assert!(!progress.contains("turn_submitted"));
        }
        assert_eq!(state.lock().unwrap().sessions, 1);
        assert_eq!(state.lock().unwrap().models, vec![json!("kimi-k3")]);
        assert_eq!(state.lock().unwrap().turns, if print { 1 } else { 2 });
        assert_eq!(
            std::fs::read_to_string(dir.path().join("output.txt")).unwrap(),
            if print {
                "native effect"
            } else {
                "second effect"
            }
        );
        let result: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(result["status"], "completed");
    }

    #[tokio::test]
    async fn dry_run_validates_without_remote_requests_or_local_journals() {
        let (server, state, dir) = setup("normal").await;
        let (code, result) = launch(
            &server,
            dir.path(),
            &["--agent", "agent", "--prompt", "task", "--dry-run"],
        )
        .await;
        assert_eq!(code, 0);
        assert_eq!(result["dry_run"], true);
        assert_eq!(state.lock().unwrap().sessions, 0);
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
    }

    #[tokio::test]
    async fn doctor_uses_only_project_read_and_redacts_auth_error_bodies() {
        let server = MockServer::start().await;
        Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path(
                "/v1/projects/project/managed-agents",
            ))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(json!({"detail":"secret-provider-token"})),
            )
            .expect(1)
            .mount(&server)
            .await;
        let output = tokio::process::Command::new(env!("CARGO_BIN_EXE_sikaru"))
            .env("SIKARU_API_KEY", "controller-secret")
            .args([
                "--base-url",
                &server.uri(),
                "doctor",
                "--project",
                "project",
            ])
            .output()
            .await
            .unwrap();
        assert!(!output.status.success());
        let result: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(result["checks"][1]["ok"], false);
        assert!(result.to_string().contains("auth login"), "{result}");
        assert!(!result.to_string().contains("secret-provider-token"));
        assert_eq!(server.received_requests().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn running_workflow_emits_identities_before_single_final_json() {
        use tokio::io::{AsyncBufReadExt, BufReader};
        let (server, _state, dir) = setup("hang").await;
        let workspace = dir.path().canonicalize().unwrap();
        let mut child = tokio::process::Command::new(
            std::env::var("SIKARU_TEST_INSTALLED")
                .unwrap_or_else(|_| env!("CARGO_BIN_EXE_sikaru").to_owned()),
        )
        .env("SIKARU_API_KEY", "controller-secret")
        .args([
            "--base-url",
            &server.uri(),
            "exec",
            "--project",
            "project",
            "--agent",
            "agent",
            "--workspace",
            workspace.to_str().unwrap(),
            "--prompt",
            "hold",
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
        let mut lines = BufReader::new(child.stderr.take().unwrap()).lines();
        let mut events = Vec::new();
        for _ in 0..4 {
            let line = tokio::time::timeout(Duration::from_secs(3), lines.next_line())
                .await
                .unwrap()
                .unwrap()
                .unwrap();
            assert!(!line.contains("controller-secret") && !line.contains("scoped"));
            events.push(serde_json::from_str::<Value>(&line).unwrap());
        }
        assert_eq!(events[0]["event"], "workspace_opened");
        assert!(events[0]["state_dir"].is_string());
        assert!(events
            .iter()
            .any(|v| v["event"] == "turn_submitted" && v["run_id"] == "run"));
        assert!(
            child.try_wait().unwrap().is_none(),
            "progress must precede completion"
        );
        unsafe {
            libc::kill(child.id().unwrap() as i32, libc::SIGTERM);
        }
        let output = child.wait_with_output().await.unwrap();
        let result: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(result["session_id"], "session-1");
        assert_eq!(result["attachment_id"], "attachment-1");
    }
    #[tokio::test]
    async fn one_invocation_leaves_effect_and_each_fresh_call_creates_new_session() {
        let (server, state, dir) = setup("normal").await;
        let workspace = dir.path().canonicalize().unwrap();
        std::fs::write(workspace.join("keep"), "original").unwrap();
        let (code, first) = launch(
            &server,
            &workspace,
            &["--agent", "agent", "--prompt", "write output"],
        )
        .await;
        assert_eq!(code, 0, "{first}");
        assert_eq!(
            std::fs::read_to_string(workspace.join("output.txt")).unwrap(),
            "native effect"
        );
        assert_eq!(first["cleanup"], "confirmed");
        assert_eq!(first["usage"]["available"], false);
        assert_eq!(
            first["events"]["events"][0]["payload"]["output"],
            "wrote output"
        );
        let (code, second) = launch(
            &server,
            &workspace,
            &["--agent", "agent", "--prompt", "write again"],
        )
        .await;
        assert_eq!(code, 0, "{second}");
        assert_ne!(first["session_id"], second["session_id"]);
        assert_ne!(first["state_dir"], second["state_dir"]);
        assert_eq!(state.lock().unwrap().sessions, 2);
        assert_eq!(
            std::fs::read_to_string(workspace.join("keep")).unwrap(),
            "original"
        );
    }
    #[tokio::test]
    async fn approval_parks_cleanly_and_explicit_wait_allows_late_approval() {
        let (server, _, dir) = setup("approval").await;
        let workspace = dir.path().canonicalize().unwrap();
        let (code, result) = launch(
            &server,
            &workspace,
            &["--agent", "agent", "--prompt", "write"],
        )
        .await;
        assert_eq!(code, 2, "{result}");
        assert_eq!(result["cleanup"], "confirmed");
        assert!(!workspace.join("output.txt").exists());
        let (server, _, dir) = setup("late").await;
        let workspace = dir.path().canonicalize().unwrap();
        let (code, result) = launch(
            &server,
            &workspace,
            &[
                "--agent",
                "agent",
                "--prompt",
                "write",
                "--approval-wait",
                "2",
            ],
        )
        .await;
        assert_eq!(code, 0, "{result}");
        assert!(workspace.join("output.txt").exists());
    }
    #[tokio::test]
    async fn cleared_approval_does_not_disable_idle_backoff_after_wait_budget() {
        let (server, state, dir) = setup("cleared").await;
        let (code, result) = launch(
            &server,
            &dir.path().canonicalize().unwrap(),
            &[
                "--agent",
                "agent",
                "--prompt",
                "work",
                "--approval-wait",
                "2",
                "--timeout",
                "4",
            ],
        )
        .await;
        assert_eq!(code, 3, "{result}");
        assert_eq!(result["reason"], "deadline");
        assert!(
            state.lock().unwrap().polls <= 6,
            "cleared approval must retain idle backoff"
        );
    }
    #[tokio::test]
    async fn explicit_resume_rejects_replaced_workspace_before_new_claim() {
        let (server, state, dir) = setup("approval").await;
        let workspace = dir.path().canonicalize().unwrap();
        let (_, result) = launch(
            &server,
            &workspace,
            &["--agent", "agent", "--prompt", "write"],
        )
        .await;
        let other = tempfile::tempdir().unwrap();
        let (code, _) = launch(
            &server,
            &other.path().canonicalize().unwrap(),
            &["--resume", result["state_dir"].as_str().unwrap()],
        )
        .await;
        assert_eq!(code, 4);
        assert_eq!(state.lock().unwrap().epoch, 1);
    }
    #[tokio::test]
    async fn successful_effect_with_unconfirmed_cleanup_is_not_success() {
        let (server, _, dir) = setup("cleanupfail").await;
        let (code, result) = launch(
            &server,
            &dir.path().canonicalize().unwrap(),
            &["--agent", "agent", "--prompt", "write"],
        )
        .await;
        assert_eq!(code, 4, "{result}");
        assert_eq!(result["cleanup"], "unconfirmed");
        assert!(dir.path().join("output.txt").exists());
    }
    #[tokio::test]
    async fn resume_observes_original_parked_run_with_new_claim_and_original_journal() {
        let (server, state, dir) = setup("approval").await;
        let workspace = dir.path().canonicalize().unwrap();
        let (code, parked) = launch(
            &server,
            &workspace,
            &["--agent", "agent", "--prompt", "write"],
        )
        .await;
        assert_eq!(code, 2);
        let (code, resumed) = launch(
            &server,
            &workspace,
            &["--resume", parked["state_dir"].as_str().unwrap()],
        )
        .await;
        assert_eq!(code, 0, "{resumed}");
        assert_eq!(resumed["session_id"], parked["session_id"]);
        assert_eq!(state.lock().unwrap().turns, 1);
        assert_eq!(state.lock().unwrap().epoch, 2);
        assert_eq!(state.lock().unwrap().credential, 2);
        assert_eq!(
            std::fs::read_to_string(workspace.join("output.txt")).unwrap(),
            "native effect"
        );
    }
    #[tokio::test]
    async fn failed_startup_failed_run_and_deadline_are_distinct() {
        for (scenario, expected, reason) in [
            ("startupfail", 4, Some("executor_startup_failed")),
            ("failed", 1, None),
            ("hang", 3, Some("deadline")),
        ] {
            let (server, state, dir) = setup(scenario).await;
            let (code, result) = launch(
                &server,
                &dir.path().canonicalize().unwrap(),
                &["--agent", "agent", "--prompt", "work", "--timeout", "1"],
            )
            .await;
            assert_eq!(code, expected, "{result}");
            if let Some(reason) = reason {
                assert_eq!(result["reason"], reason, "{result}");
            }
            if scenario == "hang" {
                assert!(
                    state.lock().unwrap().polls <= 3,
                    "idle polls must honor server backoff"
                );
                assert_eq!(result["cleanup"], "confirmed");
                assert_eq!(result["cancel_acknowledged"], true);
            }
        }
    }
    #[tokio::test]
    async fn interrupted_setup_resumes_idempotently_without_losing_session_identity() {
        let (server, state, dir) = setup("partialsetup").await;
        let workspace = dir.path().canonicalize().unwrap();
        let (code, failed) = launch(
            &server,
            &workspace,
            &["--agent", "agent", "--prompt", "write"],
        )
        .await;
        assert_eq!(code, 4);
        assert_eq!(state.lock().unwrap().sessions, 1);
        let (code, resumed) = launch(
            &server,
            &workspace,
            &[
                "--resume",
                failed["state_dir"].as_str().unwrap(),
                "--prompt",
                "write",
            ],
        )
        .await;
        assert_eq!(code, 0, "{resumed}");
        assert_eq!(resumed["session_id"], failed["session_id"]);
        assert_eq!(state.lock().unwrap().sessions, 1);
        assert!(workspace.join("output.txt").exists());
    }
    #[tokio::test]
    async fn new_prompt_on_resume_cannot_finish_using_previous_turn() {
        let (server, state, dir) = setup("completedresume").await;
        let workspace = dir.path().canonicalize().unwrap();
        let (code, first) = launch(
            &server,
            &workspace,
            &["--agent", "agent", "--prompt", "first"],
        )
        .await;
        assert_eq!(code, 0, "{first}");
        let (code, second) = launch(
            &server,
            &workspace,
            &[
                "--resume",
                first["state_dir"].as_str().unwrap(),
                "--prompt",
                "second",
            ],
        )
        .await;
        assert_eq!(code, 0, "{second}");
        assert_eq!(second["run_id"], "run-2");
        assert_eq!(state.lock().unwrap().turns, 2);
        assert_eq!(
            std::fs::read_to_string(workspace.join("output.txt")).unwrap(),
            "second effect"
        );
    }
}
