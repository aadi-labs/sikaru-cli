use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::process::{Command, Output};
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

// Native `sikaru agent` verbs against a mock gateway: what each verb sends, what it prints, and
// which verbs default help lists.
async fn sikaru(server: &MockServer, args: &[&str]) -> (Output, tempfile::TempDir) {
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

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn agent_summary(slug: &str) -> Value {
    json!({"id": format!("agent_{slug}"), "organizationId": "org", "projectId": "p", "agentSlug": slug,
        "displayName": "Support Bot", "harnessId": slug, "activeHarnessVersionId": "hv_1",
        "compatibilityProfileId": "support-bot-v0", "status": "active", "personal": false})
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_prints_the_agent() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/managed-agents/support-bot"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"managedAgent": agent_summary("support-bot")})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "get", "support-bot", "-p", "p"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown["agentSlug"], "support-bot");
    assert_eq!(shown["displayName"], "Support Bot");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_of_an_unknown_agent_fails_with_not_found() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/managed-agents/missing"))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(json!({"detail": "Managed agent not found"})),
        )
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "get", "missing", "--project", "p"]).await;

    assert!(!output.status.success());
    let shown = format!("{}{}", text(&output.stdout), text(&output.stderr));
    assert!(shown.contains("Managed agent not found"), "{shown}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn list_prints_the_projects_agents() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/managed-agents"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"managedAgents": [agent_summary("support-bot"), agent_summary("sales-bot")]}),
        ))
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "list", "-p", "p"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    let slugs: Vec<&str> = shown
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["agentSlug"].as_str().unwrap())
        .collect();
    assert_eq!(slugs, ["support-bot", "sales-bot"]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn create_sends_the_agent_document() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/p/managed-agents"))
        .respond_with(ResponseTemplate::new(201).set_body_json(
            json!({"managedAgent": agent_summary("support-bot"), "harnessVersion": null}),
        ))
        .expect(1)
        .mount(&server)
        .await;
    let document = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(document.path(), "# Support bot\n").unwrap();

    let (output, _home) = sikaru(
        &server,
        &[
            "agent",
            "create",
            "support-bot",
            "-p",
            "p",
            "--file",
            document.path().to_str().unwrap(),
            "--personal",
        ],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let requests = server.received_requests().await.unwrap();
    let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["agentSlug"], "support-bot");
    assert_eq!(body["personal"], true);
    assert_eq!(body["source"]["sourceKind"], "agent_document");
    assert_eq!(body["source"]["document"], "# Support bot\n");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn publish_sends_the_reviewed_revision_to_the_document_publish_route() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/p/managed-agents/support-bot/document/publish",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(
        &server,
        &[
            "agent",
            "publish",
            "support-bot",
            "-p",
            "p",
            "--revision",
            "3",
            "--access-digest",
            "digest-1",
            "--live-version",
            "hv_1",
            "--acknowledge-widening",
        ],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let requests = server.received_requests().await.unwrap();
    let body: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(body["revision"], 3);
    assert_eq!(body["expectedAccessDigest"], "digest-1");
    assert_eq!(body["expectedLiveVersionId"], "hv_1");
    assert_eq!(body["acknowledgeWidening"], true);
    assert_eq!(body["acknowledgeRemovals"], false);
}

fn sse(events: &[Value]) -> ResponseTemplate {
    let body: String = events
        .iter()
        .map(|event| format!("id: {}\ndata: {event}\n\n", event["sequence"]))
        .collect();
    ResponseTemplate::new(200).set_body_raw(body, "text/event-stream")
}

fn event(sequence: i64, event_type: &str) -> Value {
    json!({"id": format!("e{sequence}"), "sequence": sequence, "eventType": event_type,
        "createdAt": "2026-10-25T00:00:00Z", "payload": {"text": format!("step {sequence}")}})
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn logs_follow_resumes_a_closed_stream_until_a_terminal_event() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r/events/stream"))
        .and(query_param("after", "0"))
        .respond_with(sse(&[event(1, "run.started")]))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r/events/stream"))
        .and(query_param("after", "1"))
        .respond_with(sse(&[event(2, "run.completed")]))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "logs", "r", "-p", "p", "-f"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let printed: Vec<Value> = text(&output.stdout)
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let types: Vec<&str> = printed
        .iter()
        .map(|e| e[0]["eventType"].as_str().unwrap())
        .collect();
    assert_eq!(types, ["run.started", "run.completed"]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn logs_follow_stops_at_a_personal_session_it_cannot_read() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r/events/stream"))
        .respond_with(sse(&[event(0, "session.personal")]))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "logs", "r", "-p", "p", "--follow"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    assert!(text(&output.stdout).contains("session.personal"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn logs_without_follow_prints_recorded_events_once() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r/events"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            json!({"events": [event(1, "run.started"), event(2, "run.tool_call.completed")], "nextAfter": 2}),
        ))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "logs", "r", "-p", "p"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    assert!(text(&output.stdout).contains("run.tool_call.completed"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn stop_cancels_the_run() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/p/runs/r/cancel"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "cancelled"})))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["agent", "stop", "r", "-p", "p"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    assert!(text(&output.stdout).contains("cancelled"));
}

fn help(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_sikaru"))
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{args:?}: {}",
        text(&output.stderr)
    );
    text(&output.stdout)
}

fn listed_commands(help: &str) -> BTreeSet<String> {
    help.lines()
        .skip_while(|line| !line.starts_with("Commands:"))
        .skip(1)
        .take_while(|line| line.starts_with("  "))
        .filter_map(|line| line.split_whitespace().next())
        .map(str::to_owned)
        .collect()
}

#[test]
fn agent_help_lists_the_curated_verbs_only() {
    let listed = listed_commands(&help(&["agent", "--help"]));
    for verb in [
        "list", "get", "create", "run", "logs", "stop", "pull", "push", "validate", "review",
        "publish",
    ] {
        assert!(listed.contains(verb), "{verb} missing from {listed:?}");
    }
    assert!(!listed.contains("snippets"), "{listed:?}");
    help(&["agent", "snippets", "--help"]);
}

// `agent run` on Sikaru: a session, a turn without a compute attachment, the run's events on
// stderr, and one JSON result whose status sets the exit code.
async fn managed_gateway(final_event: &str, status: &str) -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/v1/projects/p/harnesses/support-bot/execution-sessions",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"session": {
            "id": "s1", "projectId": "p", "harnessId": "support-bot",
            "harnessVersionId": "hv_1", "createdAt": "2026-10-25T00:00:00Z", "nextTurn": 0}})))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/p/execution-sessions/s1/turns"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "input": {"id": "i1", "idempotencyKey": "k", "mode": "immediate",
                "status": "accepted", "runId": "r1"},
            "run": {"id": "r1", "inputId": "i1", "sessionId": "s1", "status": "queued",
                "harnessVersionId": "hv_1", "traceId": null}})))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r1/events/stream"))
        .respond_with(sse(&[event(1, "run.started"), event(2, final_event)]))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r1/events"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"events": [
            {"id": "e2", "sequence": 2, "eventType": "run.completed",
             "createdAt": "2026-10-25T00:00:00Z", "payload": {"output": "3 tickets open"}}],
            "nextAfter": 2})))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r1"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"runId": "r1",
            "status": status, "eventsUrl": "events", "harnessId": "support-bot",
            "harnessVersionId": "hv_1"})),
        )
        .mount(&server)
        .await;
    server
}

async fn turn_bodies(server: &MockServer) -> Vec<Value> {
    server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .filter(|r| r.url.path().ends_with("/turns"))
        .map(|r| serde_json::from_slice(&r.body).unwrap())
        .collect()
}

async fn touched_local_compute(server: &MockServer) -> bool {
    server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .any(|r| r.url.path().contains("/compute-"))
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn managed_run_appends_a_turn_without_compute_and_exits_zero() {
    let server = managed_gateway("run.completed", "completed").await;

    let (output, _home) = sikaru(
        &server,
        &[
            "agent",
            "run",
            "support-bot",
            "summarize open tickets",
            "-p",
            "p",
        ],
    )
    .await;

    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["status"], "completed");
    assert_eq!(result["session_id"], "s1");
    assert_eq!(result["run_id"], "r1");
    assert_eq!(result["final_output"]["output"], "3 tickets open");
    let turns = turn_bodies(&server).await;
    assert_eq!(turns.len(), 1);
    assert_eq!(turns[0]["input"]["goal"], "summarize open tickets");
    assert!(
        turns[0].get("compute_attachment_id").is_none(),
        "{}",
        turns[0]
    );
    assert!(!touched_local_compute(&server).await);
    assert!(
        text(&output.stderr).contains("run.started"),
        "progress streams"
    );
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn managed_run_waiting_for_approval_exits_two() {
    let server = managed_gateway("run.requires_approval", "waiting_for_human").await;

    let (output, _home) = sikaru(
        &server,
        &["agent", "run", "support-bot", "deploy", "-p", "p"],
    )
    .await;

    assert_eq!(output.status.code(), Some(2), "{}", text(&output.stderr));
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["status"], "approval_required");
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn managed_run_paused_for_funding_exits_five_and_says_how_to_resume() {
    let server = managed_gateway("run.paused", "paused").await;

    let (output, _home) = sikaru(
        &server,
        &["agent", "run", "support-bot", "deploy", "-p", "p"],
    )
    .await;

    assert_eq!(output.status.code(), Some(5), "{}", text(&output.stderr));
    let result: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["status"], "paused");
    let next = result["next"].as_str().unwrap();
    assert!(next.contains("sikaru agent runs resume --project-id p --run-id r1"), "{next}");
    let cancels = server.received_requests().await.unwrap();
    assert!(!cancels.iter().any(|r| r.url.path().ends_with("/cancel")), "a paused run is kept");
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn managed_dry_run_and_local_only_options_send_nothing() {
    let server = MockServer::start().await;

    let (dry, _home) = sikaru(
        &server,
        &[
            "agent",
            "run",
            "support-bot",
            "task",
            "-p",
            "p",
            "--dry-run",
        ],
    )
    .await;
    let (resumed, _home) = sikaru(
        &server,
        &[
            "agent",
            "run",
            "support-bot",
            "task",
            "-p",
            "p",
            "--resume",
            "state",
        ],
    )
    .await;

    assert!(dry.status.success(), "{}", text(&dry.stderr));
    let validated: Value = serde_json::from_slice(&dry.stdout).unwrap();
    assert_eq!(validated["dry_run"], true);
    assert_eq!(resumed.status.code(), Some(1));
    let refused: Value = serde_json::from_slice(&resumed.stdout).unwrap();
    assert_eq!(refused["reason"], "invalid_input");
    assert!(server.received_requests().await.unwrap().is_empty());
}

/// `agent run` takes its agent only as the first positional: `SIKARU_AGENT` never fills it, so
/// `agent run "task"` can't silently run the env agent with the task misread as the agent.
fn run_with_env_agent(server: &MockServer, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_sikaru"))
        .args(["--base-url", &server.uri(), "agent", "run"])
        .args(args)
        .args(["-p", "p", "--dry-run", "--format", "json"])
        .env("SIKARU_AGENT", "env-bot")
        .env("SIKARU_API_KEY", "test-only")
        .output()
        .unwrap()
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn run_agent_is_positional_only() {
    let server = MockServer::start().await;

    let missing = run_with_env_agent(&server, &["--prompt", "task"]);
    let named = run_with_env_agent(&server, &["support-bot", "task"]);

    assert!(!missing.status.success());
    let refused: Value = serde_json::from_slice(&missing.stdout).unwrap();
    assert_eq!(refused["error"]["help"], "<agent>", "{refused}");
    assert!(named.status.success(), "{}", text(&named.stderr));
    let validated: Value = serde_json::from_slice(&named.stdout).unwrap();
    assert_eq!(validated["agent"], "support-bot");
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn managed_conversation_continues_one_session() {
    use std::io::Write;
    use std::os::fd::FromRawFd;
    let server = managed_gateway("run.completed", "completed").await;
    let (mut master, mut slave) = (0, 0);
    let opened = unsafe {
        libc::openpty(
            &mut master,
            &mut slave,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };
    assert_eq!(opened, 0);
    let mut input = unsafe { std::fs::File::from_raw_fd(master) };
    let terminal = unsafe { std::fs::File::from_raw_fd(slave) };
    let child = tokio::process::Command::new(env!("CARGO_BIN_EXE_sikaru"))
        .args([
            "--base-url",
            &server.uri(),
            "agent",
            "run",
            "support-bot",
            "-p",
            "p",
        ])
        .env("SIKARU_API_KEY", "test-only")
        .env_remove("SIKARU_AGENT")
        .stdin(terminal)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    input
        .write_all(b"first task\nsecond task\n/exit\n")
        .unwrap();

    let output = tokio::time::timeout(std::time::Duration::from_secs(30), child.wait_with_output())
        .await
        .unwrap()
        .unwrap();

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown = text(&output.stderr);
    assert!(
        shown.contains("You>") && shown.contains("3 tickets open"),
        "{shown}"
    );
    let goals: Vec<String> = turn_bodies(&server)
        .await
        .iter()
        .map(|turn| turn["input"]["goal"].as_str().unwrap().trim().to_owned())
        .collect();
    assert_eq!(goals, ["first task", "second task"]);
}
