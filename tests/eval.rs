use serde_json::{json, Value};
use wiremock::{
    matchers::{method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

#[path = "support/cli.rs"]
mod cli;
use cli::{listed_commands, request_bodies, sikaru, text};

// `sikaru eval` against a mock gateway: starting a dataset's Checks, following them to a
// summary, refusals, and reading results per agent.
const CHECKS: &str = "/v1/projects/p/datasets/ds/checks";
const RESULTS: &str = "/v1/projects/p/datasets/ds/checks/results";

fn started() -> Value {
    json!({"datasetVersion": 2, "agentVersion": "hv_live", "notEligible": {"multi_turn": 1},
        "checks": [
            {"exampleId": "e1", "outcome": "started", "checkId": "c1", "resultId": "x1", "runId": "run_1"},
            {"exampleId": "e2", "outcome": "started", "checkId": "c2", "resultId": "x2", "runId": "run_2"},
            {"exampleId": "e3", "outcome": "failed", "reason": "credits exhausted"}]})
}

fn group(agent: &str, passed: i64, failed: i64, running: i64) -> Value {
    let failures: Vec<Value> = (0..failed)
        .map(|i| {
            json!({"exampleId": format!("{agent}-e{i}"), "checkId": "c", "checkName": "n",
            "runId": format!("{agent}-run{i}"), "reason": "wrong answer"})
        })
        .collect();
    json!({"agentSlug": agent, "agentVersion": "hv_live", "datasetVersion": 2,
        "total": passed + failed + running, "passed": passed, "failed": failed, "running": running,
        "notStarted": 0, "passRate": null, "failures": failures})
}

fn results(groups: Vec<Value>) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"results": groups}))
}

#[test]
fn eval_help_lists_the_curated_verbs_only() {
    let listed: Vec<String> = listed_commands(&["eval"]).into_iter().collect();
    assert_eq!(listed, ["get", "list", "logs", "run", "samples", "stop"]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn run_starts_checks_on_the_live_version_and_prints_run_ids() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(CHECKS))
        .respond_with(ResponseTemplate::new(202).set_body_json(started()))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(
        &server,
        &["eval", "run", "ds", "-a", "support-bot", "-p", "p"],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    let runs: Vec<&str> = shown["checks"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|check| check["runId"].as_str())
        .collect();
    assert_eq!(runs, ["run_1", "run_2"]);
    let sent = &request_bodies(&server, "POST").await[0];
    assert_eq!(sent["agent_slug"], "support-bot");
    assert!(!sent["idempotency_key"].as_str().unwrap().is_empty());
    assert!(sent.get("version").is_none_or(Value::is_null), "{sent}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn follow_waits_until_this_runs_checks_settle() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(CHECKS))
        .respond_with(ResponseTemplate::new(202).set_body_json(started()))
        .mount(&server)
        .await;
    // An earlier run of the same versions already settled three checks; this run adds two.
    for response in [
        results(vec![group("support-bot", 3, 0, 0)]),
        results(vec![group("support-bot", 4, 0, 0)]),
    ] {
        Mock::given(method("GET"))
            .and(path(RESULTS))
            .respond_with(response)
            .up_to_n_times(1)
            .mount(&server)
            .await;
    }
    Mock::given(method("GET"))
        .and(path(RESULTS))
        .and(query_param("version", "2"))
        .respond_with(results(vec![group("support-bot", 4, 1, 0)]))
        .mount(&server)
        .await;

    let (output, _home) = sikaru(
        &server,
        &[
            "eval",
            "run",
            "ds",
            "-a",
            "support-bot",
            "-p",
            "p",
            "--follow",
        ],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let summary: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        (summary["passed"].clone(), summary["failed"].clone()),
        (json!(4), json!(1))
    );
    assert_eq!(summary["failures"][0]["runId"], "support-bot-run0");
    let progress = text(&output.stderr);
    for shown in ["run_1", "run_2", "credits exhausted"] {
        assert!(progress.contains(shown), "{shown} missing from {progress}");
    }
}

async fn refused(status: u16, detail: &str) -> (std::process::Output, tempfile::TempDir) {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(CHECKS))
        .respond_with(ResponseTemplate::new(status).set_body_json(json!({"detail": detail})))
        .mount(&server)
        .await;
    sikaru(&server, &["eval", "run", "ds", "-a", "my-bot", "-p", "p"]).await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn refusals_print_the_gateways_reason_and_fail() {
    for (status, detail) in [
        (409, "Checks run on shared agents; my-bot is personal"),
        (409, "my-bot has no available production release"),
        (503, "Check execution is unavailable"),
    ] {
        let (output, _home) = refused(status, detail).await;

        assert!(!output.status.success(), "{detail}");
        let shown = format!("{}{}", text(&output.stdout), text(&output.stderr));
        assert!(shown.contains(detail), "{shown}");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn samples_lists_only_the_chosen_agents_failures() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(RESULTS))
        .respond_with(results(vec![
            group("support-bot", 1, 2, 0),
            group("sales-bot", 0, 1, 0),
        ]))
        .mount(&server)
        .await;

    let (output, _home) = sikaru(
        &server,
        &["eval", "samples", "ds", "-a", "support-bot", "-p", "p"],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let rows: Value = serde_json::from_slice(&output.stdout).unwrap();
    let runs: Vec<(&str, &str)> = rows
        .as_array()
        .unwrap()
        .iter()
        .map(|row| {
            (
                row["agentSlug"].as_str().unwrap(),
                row["runId"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        runs,
        [
            ("support-bot", "support-bot-run0"),
            ("support-bot", "support-bot-run1")
        ]
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn get_filters_by_dataset_version() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(RESULTS))
        .and(query_param("version", "2"))
        .respond_with(results(vec![
            group("support-bot", 2, 0, 0),
            group("sales-bot", 1, 0, 0),
        ]))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(
        &server,
        &["eval", "get", "ds", "--dataset-version", "2", "-p", "p"],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let groups: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(groups.as_array().unwrap().len(), 2);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn list_shows_only_datasets_run_as_checks() {
    let server = MockServer::start().await;
    let dataset = |id: &str, checks: i64| {
        json!({"id": id, "name": id, "description": "", "exampleCount": 3, "version": 1,
            "createdAt": "t", "createdBy": "u", "updatedAt": "t",
            "usedBy": {"agents": [], "checks": checks}})
    };
    Mock::given(method("GET"))
        .and(path("/v1/projects/p/datasets"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"datasets": [dataset("checked", 4), dataset("unused", 0)]})),
        )
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["eval", "list", "-p", "p"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    let ids: Vec<&str> = shown
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, ["checked"]);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn stop_cancels_one_example_run() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/projects/p/runs/run_1/cancel"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"status": "cancelled"})))
        .expect(1)
        .mount(&server)
        .await;

    let (output, _home) = sikaru(&server, &["eval", "stop", "run_1", "-p", "p"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    assert!(text(&output.stdout).contains("cancelled"));
}
