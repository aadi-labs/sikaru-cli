use std::process::Command;
use wiremock::{
    matchers::{header, method, path},
    Mock, MockServer, ResponseTemplate,
};

// Exercise the shipped command and its embedded contract: a header alone must
// not override the endpoint's x-fern-retries.disabled policy.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mutation_endpoint_is_not_replayed_even_with_an_idempotency_header() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/project/workflows/workflow/runs"))
        .and(header("Idempotency-Key", "retry-regression"))
        .respond_with(ResponseTemplate::new(503))
        .expect(1)
        .mount(&server)
        .await;
    let url = server.uri();
    let output = tokio::task::spawn_blocking(move || {
        Command::new(env!("CARGO_BIN_EXE_sikaru"))
            .args([
                "workflows",
                "start_project_workflow_run",
                "--project-id",
                "project",
                "--workflow-id",
                "workflow",
                "--input",
                "{}",
                "--idempotency-key",
                "retry-regression",
                "--base-url",
                &url,
                "--format",
                "json",
            ])
            .env("SIKARU_API_KEY", "test-only")
            .output()
            .unwrap()
    })
    .await
    .unwrap();
    assert!(
        !output.status.success(),
        "503 must be reported as a failure"
    );
    assert_eq!(
        server.received_requests().await.unwrap().len(),
        1,
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
