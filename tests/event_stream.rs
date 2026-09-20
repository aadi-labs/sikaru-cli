use std::process::Command;
use wiremock::{matchers::{method, path, query_param}, Mock, MockServer, ResponseTemplate};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn streams_event_data_and_forwards_resume_cursor() {
    let server = MockServer::start().await;
    let data = r#"{"id":"e2","sequence":2,"eventType":"run.completed","createdAt":"2026-09-19T00:00:00Z","payload":{"text":"streamed-result"}}"#;
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/runs/r/events/stream"))
        .and(query_param("after", "1"))
        .respond_with(ResponseTemplate::new(200)
            .insert_header("content-type", "text/event-stream")
            .set_body_string(format!(": heartbeat\n\nid: 2\ndata: {data}\n\n")))
        .expect(1).mount(&server).await;
    let url = server.uri();
    let output = tokio::task::spawn_blocking(move || Command::new(env!("CARGO_BIN_EXE_sikaru"))
        .args(["runs", "stream_events", "--project-id", "p", "--run-id", "r", "--after", "1",
            "--base-url", &url, "--format", "json"])
        .env("SIKARU_API_KEY", "test-only").output().unwrap()).await.unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stdout).contains("streamed-result"));
}
