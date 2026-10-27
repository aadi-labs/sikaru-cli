use serde_json::{json, Value};
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

#[path = "support/cli.rs"]
mod cli;
use cli::{listed_commands, request_bodies, sikaru, text};

// `sikaru environment` against a mock gateway: datasets with their examples, versions and check
// results; push creates and uploads, pull writes the export.
fn dataset(id: &str) -> Value {
    json!({"id": id, "name": "Support questions", "description": "", "exampleCount": 2,
        "version": 3, "createdAt": "t", "createdBy": "u", "updatedAt": "t",
        "usedBy": {"agents": ["support-bot"], "checks": 2}})
}

fn example(id: &str, question: &str) -> Value {
    json!({"id": id, "input": [{"role": "user", "content": question}], "source": {"kind": "upload"}})
}

#[test]
fn environment_help_lists_the_curated_verbs_only() {
    let listed: Vec<String> = listed_commands(&["environment"]).into_iter().collect();
    assert_eq!(listed, ["info", "list", "pull", "push"]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn info_shows_examples_and_check_results() {
    let server = MockServer::start().await;
    let root = "/v1/projects/p/datasets/ds";
    let responses = [
        (root.to_owned(), json!({"dataset": dataset("ds")})),
        (
            format!("{root}/examples"),
            json!({"version": 3, "latest": 3, "total": 2, "examples": [example("e1", "Where is my order?"),
                example("e2", "Refund please")]}),
        ),
        (
            format!("{root}/versions"),
            json!({"versions": [{"number": 3}, {"number": 2}]}),
        ),
        (
            format!("{root}/checks/results"),
            json!({"results": [{"agentSlug": "support-bot", "agentVersion": "hv_1",
                "datasetVersion": 3, "total": 2, "passed": 1, "failed": 1, "running": 0,
                "notStarted": 0, "passRate": 0.5, "failures": []}]}),
        ),
    ];
    for (route, body) in responses {
        Mock::given(method("GET"))
            .and(path(route))
            .respond_with(ResponseTemplate::new(200).set_body_json(body))
            .mount(&server)
            .await;
    }

    let (output, _home) = sikaru(&server, &["environment", "info", "ds", "-p", "p"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown["dataset"]["name"], "Support questions");
    let inputs: Vec<&str> = shown["examples"]["examples"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["input"][0]["content"].as_str().unwrap())
        .collect();
    assert_eq!(inputs, ["Where is my order?", "Refund please"]);
    assert_eq!(shown["checks"][0]["agentSlug"], "support-bot");
    assert_eq!(shown["checks"][0]["passRate"], 0.5);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn push_creates_the_dataset_then_uploads_the_file() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/p/datasets"))
        .respond_with(
            ResponseTemplate::new(201).set_body_json(json!({"dataset": dataset("ds_new")})),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/p/datasets/ds_new/uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"items": []})))
        .expect(1)
        .mount(&server)
        .await;
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("support-questions.jsonl");
    let content = "{\"input\": \"Where is my order?\"}\n{\"input\": \"Refund please\"}\n";
    std::fs::write(&file, content).unwrap();

    let (output, _home) = sikaru(
        &server,
        &["environment", "push", file.to_str().unwrap(), "-p", "p"],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let sent = request_bodies(&server, "POST").await;
    assert_eq!(sent[0]["name"], "support-questions");
    assert_eq!(sent[1]["content"], content);
    assert_eq!(sent[1]["format"], "jsonl");
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown["dataset"]["id"], "ds_new");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn push_sends_csv_files_as_csv() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/p/datasets"))
        .respond_with(
            ResponseTemplate::new(201).set_body_json(json!({"dataset": dataset("ds_new")})),
        )
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/p/datasets/ds_new/uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"items": []})))
        .mount(&server)
        .await;
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("rows.CSV");
    std::fs::write(&file, "input,expected,tags\nhi,hello,\n").unwrap();

    let (output, _home) = sikaru(
        &server,
        &[
            "environment",
            "push",
            file.to_str().unwrap(),
            "--name",
            "Greetings",
            "-p",
            "p",
        ],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let sent = request_bodies(&server, "POST").await;
    assert_eq!(sent[0]["name"], "Greetings");
    assert_eq!(sent[1]["format"], "csv");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pull_writes_the_export_unchanged() {
    let server = MockServer::start().await;
    let export = "{\"dataset\":\"ds\",\"version\":2}\n{\"input\":\"Where is my order?\"}\n";
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/datasets/ds/export"))
        .and(query_param("version", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(export, "application/x-ndjson"))
        .expect(1)
        .mount(&server)
        .await;

    let (output, home) = sikaru(
        &server,
        &[
            "environment",
            "pull",
            "ds",
            "--dataset-version",
            "2",
            "-p",
            "p",
        ],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let written = std::fs::read_to_string(home.path().join("ds.jsonl")).unwrap();
    assert_eq!(written, export);
}
