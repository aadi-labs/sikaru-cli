use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Output, Stdio};
use wiremock::{
    matchers::{header, method, path},
    Mock, MockServer, ResponseTemplate,
};

// Native `sikaru inference` commands against a mock edge: the wire request, what lands on
// stdout versus stderr, the catalog table and the edge's named-field errors.
async fn sikaru(server: &MockServer, args: &[&str]) -> Output {
    sikaru_with_stdin(server, args, "").await
}

async fn sikaru_with_stdin(server: &MockServer, args: &[&str], stdin: &str) -> Output {
    let mut args: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
    args.extend(["--base-url".into(), server.uri()]);
    let stdin = stdin.to_owned();
    tokio::task::spawn_blocking(move || {
        let mut child = Command::new(env!("CARGO_BIN_EXE_sikaru"))
            .args(args)
            .env("SIKARU_API_KEY", "test-only")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(stdin.as_bytes())
            .unwrap();
        child.wait_with_output().unwrap()
    })
    .await
    .unwrap()
}

async fn sent_bodies(server: &MockServer) -> Vec<Value> {
    let requests = server.received_requests().await.unwrap();
    requests
        .iter()
        .map(|request| serde_json::from_slice(&request.body).unwrap())
        .collect()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn sse(events: &[Value]) -> String {
    let mut body: String = events
        .iter()
        .map(|event| format!("data: {event}\n\n"))
        .collect();
    body.push_str("data: [DONE]\n\n");
    body
}

fn completion(content: &str) -> Value {
    json!({"id": "c", "object": "chat.completion", "created": 0, "model": "m",
        "choices": [{"index": 0, "finish_reason": "stop", "message": {"role": "assistant", "content": content}}],
        "usage": {"prompt_tokens": 3, "completion_tokens": 2, "total_tokens": 5}})
}

async fn mount_completion(server: &MockServer, content: &str) {
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(header("Authorization", "Bearer test-only"))
        .respond_with(ResponseTemplate::new(200).set_body_json(completion(content)))
        .mount(server)
        .await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn chat_sends_one_request_by_default_and_prints_the_reply_and_usage() {
    let server = MockServer::start().await;
    mount_completion(&server, "Hi there").await;

    let output = sikaru(
        &server,
        &[
            "inference",
            "chat",
            "m",
            "Hello",
            "-s",
            "Be brief",
            "-t",
            "0.5",
            "--max-tokens",
            "100",
            "--reasoning-effort",
            "max",
        ],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    assert_eq!(text(&output.stdout), "Hi there\n");
    assert!(text(&output.stderr).contains("3 prompt + 2 completion = 5 tokens"));
    let body = &sent_bodies(&server).await[0];
    assert_eq!(body["stream"], false);
    assert_eq!(body["model"], "m");
    assert_eq!(body["max_tokens"], 100);
    assert_eq!(body["temperature"], 0.5);
    assert_eq!(body["reasoning_effort"], "max");
    assert_eq!(
        body["messages"],
        json!([{"role": "system", "content": "Be brief"}, {"role": "user", "content": "Hello"}])
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn chat_reads_the_message_from_stdin_when_it_is_omitted() {
    let server = MockServer::start().await;
    mount_completion(&server, "Sure").await;

    let output = sikaru_with_stdin(&server, &["inference", "chat", "m"], "  explain RL\n").await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    assert_eq!(text(&output.stdout), "Sure\n");
    assert_eq!(
        sent_bodies(&server).await[0]["messages"],
        json!([{"role": "user", "content": "explain RL"}])
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn chat_without_a_message_or_stdin_fails_before_sending() {
    let server = MockServer::start().await;

    let output = sikaru(&server, &["inference", "chat", "m"]).await;

    assert!(!output.status.success());
    let shown = format!("{}{}", text(&output.stdout), text(&output.stderr));
    assert!(shown.contains("empty message from stdin"), "{shown}");
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn stream_sends_answer_to_stdout_and_reasoning_to_stderr() {
    let server = MockServer::start().await;
    let delta = |field: &str, piece: &str| {
        json!({"id": "c", "object": "chat.completion.chunk",
        "choices": [{"index": 0, "delta": {field: piece}}]})
    };
    let usage = json!({"id": "c", "object": "chat.completion.chunk", "choices": [],
        "usage": {"prompt_tokens": 7, "completion_tokens": 2, "total_tokens": 9}});
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            sse(&[
                delta("reasoning_content", "Think"),
                delta("reasoning_content", "ing"),
                delta("content", "Hel"),
                delta("content", "lo!"),
                usage,
            ]),
            "text/event-stream",
        ))
        .expect(1)
        .mount(&server)
        .await;

    let output = sikaru(&server, &["inference", "chat", "m", "Hello", "--stream"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    assert_eq!(text(&output.stdout), "Hello!\n");
    let stderr = text(&output.stderr);
    assert!(
        stderr.starts_with("<think>\nThinking\n</think>\n"),
        "{stderr}"
    );
    assert!(
        stderr.contains("7 prompt + 2 completion = 9 tokens"),
        "{stderr}"
    );
    let body = &sent_bodies(&server).await[0];
    assert_eq!(body["stream"], true);
    assert_eq!(body["stream_options"]["include_usage"], true);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn chat_output_json_prints_the_full_completion() {
    let server = MockServer::start().await;
    mount_completion(&server, "Hello").await;

    let output = sikaru(&server, &["inference", "chat", "m", "hi", "-o", "json"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown, completion("Hello"));
    assert_eq!(sent_bodies(&server).await[0]["stream"], false);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn stream_with_output_json_is_rejected() {
    let server = MockServer::start().await;

    let output = sikaru(
        &server,
        &[
            "inference",
            "chat",
            "m",
            "hi",
            "--stream",
            "--output",
            "json",
        ],
    )
    .await;

    assert!(!output.status.success());
    let shown = format!("{}{}", text(&output.stdout), text(&output.stderr));
    assert!(
        shown.contains("--stream is not supported with --output json"),
        "{shown}"
    );
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_rejected_field_prints_its_param_and_message_and_fails() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({"error": {
            "message": "Model m does not support top_k", "type": "invalid_request_error",
            "param": "top_k", "code": "unsupported_parameter"}})))
        .mount(&server)
        .await;

    let output = sikaru(&server, &["inference", "chat", "m", "Hello"]).await;

    assert!(!output.status.success());
    let shown = format!("{}{}", text(&output.stdout), text(&output.stderr));
    assert!(
        shown.contains("top_k: Model m does not support top_k"),
        "{shown}"
    );
}

fn card(id: &str, name: &str, input: Value, output: f64, reasoning: bool) -> Value {
    let effort = json!({"efforts": ["low", "high", "max"], "default_effort": "max", "budget": false,
        "history": ["interleaved"], "default_history": "interleaved", "summary": false,
        "normalization": {"none": "low", "minimal": "low", "low": "low", "medium": "low", "high": "high",
            "xhigh": "high", "max": "max", "true": "max", "false": null}});
    json!({"id": id, "object": "model", "created": 0, "owned_by": "sikaru", "display_name": name,
        "pricing": {"input_usd_per_mtok": input, "output_usd_per_mtok": output},
        "specs": {"context_window": 1000000, "max_output_tokens": 65536,
            "modalities": {"input": ["text", "image"], "output": ["text"]},
            "endpoints": ["chat"], "supports_reasoning": reasoning,
            "reasoning": if reasoning { effort } else { Value::Null },
            "supported_parameters": ["max_tokens", "reasoning_effort", "temperature"]}})
}

fn catalog() -> Vec<Value> {
    vec![
        card("org/beta-flash", "Beta Flash", json!(0.1875), 0.625, true),
        card("org/alpha-pro", "Alpha Pro", json!(3.0), 15.0, false),
        card("other/gamma", "Gamma", Value::Null, 1.0, false),
    ]
}

async fn mount_catalog(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/v1/models"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"object": "list", "data": catalog()})),
        )
        .mount(server)
        .await;
}

async fn listed_ids(server: &MockServer, extra: &[&str]) -> Vec<String> {
    let mut args = vec!["inference", "models", "-o", "json"];
    args.extend_from_slice(extra);
    let output = sikaru(server, &args).await;
    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown["object"], "list");
    shown["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|model| model["id"].as_str().unwrap().to_string())
        .collect()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn models_table_shows_prices_specs_and_the_reasoning_legend() {
    let server = MockServer::start().await;
    mount_catalog(&server).await;

    let output = sikaru(&server, &["inference", "models"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown = text(&output.stdout);
    let lines: Vec<&str> = shown.lines().collect();
    let header: Vec<&str> = lines[0]
        .split("  ")
        .map(str::trim)
        .filter(|c| !c.is_empty())
        .collect();
    assert_eq!(
        header,
        [
            "id",
            "name",
            "input $/1M tok",
            "output $/1M tok",
            "context",
            "max out",
            "reasoning"
        ]
    );
    let row = |id: &str| -> Vec<String> {
        let line = lines.iter().find(|line| line.starts_with(id)).unwrap();
        line.split_whitespace().map(str::to_string).collect()
    };
    assert_eq!(
        row("org/beta-flash"),
        [
            "org/beta-flash",
            "Beta",
            "Flash",
            "$0.1875",
            "$0.625",
            "1M",
            "65.5k",
            "✓"
        ]
    );
    assert_eq!(row("other/gamma")[2], "—");
    assert_eq!(row("org/alpha-pro").last().unwrap(), "—");
    assert_eq!(
        lines.last().unwrap().trim(),
        "reasoning ✓ = supports reasoning effort"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn models_search_sort_and_order_select_the_listing() {
    let server = MockServer::start().await;
    mount_catalog(&server).await;

    assert_eq!(
        listed_ids(&server, &[]).await,
        ["org/alpha-pro", "org/beta-flash", "other/gamma"]
    );
    assert_eq!(
        listed_ids(&server, &["--search", "ORG/"]).await,
        ["org/alpha-pro", "org/beta-flash"]
    );
    assert_eq!(
        listed_ids(&server, &["--order", "desc"]).await,
        ["other/gamma", "org/beta-flash", "org/alpha-pro"]
    );
    // A model without a price sorts last in either direction.
    assert_eq!(
        listed_ids(&server, &["--sort", "input"]).await,
        ["org/beta-flash", "org/alpha-pro", "other/gamma"]
    );
    assert_eq!(
        listed_ids(&server, &["--sort", "input", "--order", "desc"]).await,
        ["org/alpha-pro", "org/beta-flash", "other/gamma"]
    );
    assert_eq!(
        listed_ids(&server, &["--sort", "output"]).await,
        ["org/beta-flash", "other/gamma", "org/alpha-pro"]
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn one_model_describes_specs_and_effort_normalization() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/models/org/beta-flash"))
        .respond_with(ResponseTemplate::new(200).set_body_json(catalog()[0].clone()))
        .mount(&server)
        .await;

    let output = sikaru(&server, &["inference", "models", "org/beta-flash"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown = text(&output.stdout);
    for expected in [
        "org/beta-flash  Beta Flash",
        "context window: 1000000",
        "max output tokens: 65536",
        "input modalities: text, image",
        "endpoints: chat",
        "parameters: max_tokens, reasoning_effort, temperature",
        "reasoning efforts: low, high, max (default max)",
        "max      -> max",
        "medium   -> low",
        "false    -> rejected",
    ] {
        assert!(shown.contains(expected), "missing {expected:?} in\n{shown}");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_model_without_reasoning_says_so() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/models/org/alpha-pro"))
        .respond_with(ResponseTemplate::new(200).set_body_json(catalog()[1].clone()))
        .mount(&server)
        .await;

    let output = sikaru(&server, &["inference", "models", "org/alpha-pro"]).await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown = text(&output.stdout);
    assert!(shown.contains("reasoning: not supported"), "{shown}");
    assert!(!shown.contains("normalization"), "{shown}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn one_model_output_json_prints_the_card() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/models/org/beta-flash"))
        .respond_with(ResponseTemplate::new(200).set_body_json(catalog()[0].clone()))
        .mount(&server)
        .await;

    let output = sikaru(
        &server,
        &["inference", "models", "org/beta-flash", "--output", "json"],
    )
    .await;

    assert!(output.status.success(), "{}", text(&output.stderr));
    let shown: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(shown, catalog()[0]);
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

#[test]
fn generated_inference_groups_stay_out_of_the_top_level() {
    let top = help(&["--help"]);
    for name in ["completions", "responses", "models"] {
        assert!(
            !top.lines()
                .any(|line| line.trim_start().starts_with(&format!("{name} "))),
            "{name} in\n{top}"
        );
        let output = Command::new(env!("CARGO_BIN_EXE_sikaru"))
            .args([name, "--help"])
            .output()
            .unwrap();
        assert!(
            !output.status.success(),
            "{name} must not be a top-level command"
        );
    }
}

// Building each command's help asserts its flags against the CLI's global ones; a duplicate
// short flag would panic here.
#[test]
fn inference_help_lists_chat_and_catalog_flags() {
    let chat = help(&["inference", "chat", "--help"]);
    for flag in [
        "-s, --system",
        "-t, --temperature",
        "--max-tokens",
        "--reasoning-effort",
        "--stream",
        "-o, --output",
    ] {
        assert!(chat.contains(flag), "missing {flag:?} in\n{chat}");
    }
    assert!(!chat.contains("--no-stream"), "{chat}");
    let models = help(&["inference", "models", "--help"]);
    for flag in ["-o, --output", "--search", "--sort", "--order"] {
        assert!(models.contains(flag), "missing {flag:?} in\n{models}");
    }
}
