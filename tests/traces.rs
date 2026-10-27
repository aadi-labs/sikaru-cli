use serde_json::{json, Value};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

#[path = "support/cli.rs"]
mod cli;
use cli::{listed_commands, request_bodies, sikaru, text};

// `sikaru traces` against a mock gateway: upload detects the file's format and posts one trace
// stream request; get prints a run's trajectory.
async fn accepting_gateway() -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/trace-streams"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({"accepted": true})))
        .mount(&server)
        .await;
    server
}

async fn upload(server: &MockServer, name: &str, content: &str) -> std::process::Output {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join(name);
    std::fs::write(&file, content).unwrap();
    let (output, _home) = sikaru(
        server,
        &["traces", "upload", file.to_str().unwrap(), "-p", "p"],
    )
    .await;
    output
}

#[test]
fn traces_help_lists_the_curated_verbs_only() {
    let listed: Vec<String> = listed_commands(&["traces"]).into_iter().collect();
    assert_eq!(listed, ["get", "upload"]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn jsonl_spans_upload_as_openinference() {
    let server = accepting_gateway().await;
    let spans = "{\"trace_id\": \"t1\", \"span_id\": \"s1\", \"name\": \"llm\"}\n\n\
                 {\"trace_id\": \"t1\", \"span_id\": \"s2\", \"parent_span_id\": \"s1\"}\n";

    let output = upload(&server, "trace.jsonl", spans).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let sent = &request_bodies(&server, "POST").await[0];
    assert_eq!(sent["format"], "openinference.v1");
    let ids: Vec<&str> = sent["spans"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["span_id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, ["s1", "s2"]);
    assert_eq!(sent["metadata"]["project_id"], "p");
    assert_eq!(sent["metadata"]["source"], "cli");
    assert!(sent.get("dataset").is_none(), "{sent}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn otlp_and_atif_files_upload_in_their_formats() {
    let otlp = json!({"resourceSpans": [{"resource": {}, "scopeSpans": []}]}).to_string();
    let atif = json!({"schema_version": "1.0", "steps": [{"source": "user", "message": "hi"}]})
        .to_string();
    let wrapped = json!({"trajectories": [{"steps": []}, {"steps": []}]}).to_string();
    for (content, format, field, count) in [
        (otlp, "opentelemetry.otlp.v1", "resourceSpans", 1),
        (atif, "harbor.atif.v1", "trajectories", 1),
        (wrapped, "harbor.atif.v1", "trajectories", 2),
    ] {
        let server = accepting_gateway().await;

        let output = upload(&server, "trace.json", &content).await;

        assert!(output.status.success(), "{}", text(&output.stderr));
        let sent = &request_bodies(&server, "POST").await[0];
        assert_eq!(sent["format"], format, "{sent}");
        assert_eq!(sent[field].as_array().unwrap().len(), count, "{sent}");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mixed_or_unknown_files_are_refused_before_sending() {
    let mixed = "{\"trace_id\": \"t1\", \"span_id\": \"s1\"}\n{\"steps\": []}\n";
    for content in [mixed, "{\"hello\": \"world\"}"] {
        let server = accepting_gateway().await;

        let output = upload(&server, "trace.jsonl", content).await;

        assert!(!output.status.success(), "{content}");
        assert!(server.received_requests().await.unwrap().is_empty());
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_prints_the_runs_trajectory() {
    let server = MockServer::start().await;
    let trajectory = json!({"schema_version": "1.0", "session_id": "run_1",
        "steps": [{"step_id": 1, "source": "user", "message": "summarize open tickets"}]});
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/run_1/trajectory"))
        .respond_with(ResponseTemplate::new(200).set_body_json(trajectory.clone()))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["traces", "get", "run_1", "-p", "p"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown, trajectory);
}
