use serde_json::{json, Value};
use std::io::Write;
use std::process::{Command, Output, Stdio};
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

// Top-level account utilities against a mock gateway. Every run gets its own home with the file
// credential store, so the developer's keychain, saved settings and environment are never read.
struct Home(tempfile::TempDir);

impl Home {
    fn new() -> Self {
        Home(tempfile::tempdir().unwrap())
    }

    fn command(&self, args: &[&str]) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_sikaru"));
        command
            .args(args)
            .env("HOME", self.0.path())
            .env("XDG_CONFIG_HOME", self.0.path().join(".config"))
            .env("FERN_CLI_CREDENTIAL_STORE", "file")
            .env_remove("SIKARU_API_KEY")
            .env_remove("SIKARU_PROJECT");
        command
    }

    async fn run(&self, args: &[&str], stdin: &str) -> Output {
        let mut command = self.command(args);
        let stdin = stdin.to_owned();
        tokio::task::spawn_blocking(move || {
            let mut child = command
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

    async fn ok(&self, args: &[&str]) -> String {
        let output = self.run(args, "").await;
        assert!(
            output.status.success(),
            "{args:?}: {}",
            text(&output.stderr)
        );
        text(&output.stdout)
    }

    async fn doctor(&self, server: &MockServer, extra: &[&str]) -> (Output, Value) {
        let uri = server.uri();
        let mut args = vec!["--base-url", uri.as_str(), "doctor"];
        args.extend(extra);
        let output = self.run(&args, "").await;
        let result = serde_json::from_slice(&output.stdout).unwrap();
        (output, result)
    }
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

fn project_access(result: &Value) -> &Value {
    result["checks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|check| check["name"] == "project_access")
        .unwrap()
}

async fn gateway_with_project(project: &str) -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(format!("/v1/projects/{project}/managed-agents")))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"agents": []})))
        .mount(&server)
        .await;
    server
}

async fn bearer_headers(server: &MockServer) -> Vec<Option<String>> {
    server
        .received_requests()
        .await
        .unwrap()
        .iter()
        .map(|request| {
            request
                .headers
                .get("authorization")
                .map(|value| value.to_str().unwrap().to_owned())
        })
        .collect()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn login_with_token_signs_requests_and_logout_removes_it() {
    let home = Home::new();
    let server = gateway_with_project("proj").await;

    let login = home
        .run(&["login", "--with-token"], "token-from-stdin\n")
        .await;
    assert!(login.status.success(), "{}", text(&login.stderr));
    let (_, signed_in) = home.doctor(&server, &["--project", "proj"]).await;
    assert_eq!(project_access(&signed_in)["ok"], true, "{signed_in}");

    home.ok(&["logout"]).await;
    home.doctor(&server, &["--project", "proj"]).await;

    let headers = bearer_headers(&server).await;
    assert_eq!(
        headers.first().cloned().flatten().as_deref(),
        Some("Bearer token-from-stdin")
    );
    assert!(
        headers[1..]
            .iter()
            .all(|header| header.as_deref() != Some("Bearer token-from-stdin")),
        "credential still sent after logout: {headers:?}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn login_with_empty_token_fails_without_storing_anything() {
    let home = Home::new();
    let login = home.run(&["login", "--with-token"], "\n").await;
    assert!(!login.status.success());

    let server = gateway_with_project("proj").await;
    home.doctor(&server, &["--project", "proj"]).await;
    assert!(bearer_headers(&server).await.iter().all(Option::is_none));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn config_set_persists_the_active_project_and_organization() {
    let home = Home::new();
    let empty: Value = serde_json::from_str(&home.ok(&["config", "show"]).await).unwrap();
    assert_eq!(empty, json!({"project": null, "organization": null}));

    home.ok(&["config", "set", "project", "proj-saved"]).await;
    home.ok(&["config", "set", "organization", "org-saved"])
        .await;
    let saved: Value = serde_json::from_str(&home.ok(&["config", "show"]).await).unwrap();
    assert_eq!(
        saved,
        json!({"project": "proj-saved", "organization": "org-saved"})
    );

    let overridden = home
        .command(&["config", "show"])
        .env("SIKARU_PROJECT", "proj-env")
        .output()
        .unwrap();
    let overridden: Value = serde_json::from_slice(&overridden.stdout).unwrap();
    assert_eq!(overridden["project"], "proj-env");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn saved_project_is_used_when_no_project_flag_is_given() {
    let home = Home::new();
    home.run(&["login", "--with-token"], "token\n").await;
    home.ok(&["config", "set", "project", "proj-saved"]).await;
    let server = gateway_with_project("proj-saved").await;

    let (_, result) = home.doctor(&server, &[]).await;
    assert_eq!(project_access(&result)["ok"], true, "{result}");

    let flagged = gateway_with_project("proj-flag").await;
    let (_, result) = home.doctor(&flagged, &["--project", "proj-flag"]).await;
    assert_eq!(project_access(&result)["ok"], true, "{result}");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn config_set_rejects_unknown_settings() {
    let home = Home::new();
    let output = home.run(&["config", "set", "region", "x"], "").await;
    assert!(!output.status.success());
    let shown: Value = serde_json::from_str(&home.ok(&["config", "show"]).await).unwrap();
    assert_eq!(shown, json!({"project": null, "organization": null}));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn doctor_without_a_project_reports_missing_access_and_how_to_set_it() {
    let home = Home::new();
    let server = MockServer::start().await;
    let (output, result) = home.doctor(&server, &[]).await;
    assert!(!output.status.success());
    let access = project_access(&result);
    assert_eq!(access["ok"], false);
    let help = access["help"].as_str().unwrap();
    assert!(
        help.contains("sikaru config set project") && help.contains("sikaru login"),
        "{help}"
    );
    assert!(server.received_requests().await.unwrap().is_empty());
}
