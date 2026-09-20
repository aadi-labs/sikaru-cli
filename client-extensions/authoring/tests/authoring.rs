#[path = "../../cli/authoring.rs"]
#[allow(dead_code)]
mod authoring;
use std::fs;
#[test]
fn source_manifest_excludes_unlisted_private_files_and_is_stable() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("agent");
    authoring::initialize(&root, "demo").unwrap();
    fs::write(root.join("private.md"), "PRIVATE RUNTIME").unwrap();
    fs::write(root.join(".env"), "SECRET").unwrap();
    let first = authoring::package(&root).unwrap();
    assert!(!first.to_string().contains("PRIVATE"));
    assert!(!first.to_string().contains("SECRET"));
    assert_eq!(first, authoring::package(&root).unwrap());
    assert!(authoring::initialize(&root, "other").is_err());
    assert_eq!(first, authoring::package(&root).unwrap());
}
#[test]
fn rejects_traversal_unknown_kinds_and_empty_instructions() {
    let temp = tempfile::tempdir().unwrap();
    for path in [
        "../instructions.md",
        "skills/../../secret.md",
        "/instructions.md",
    ] {
        fs::write(
            temp.path().join("sikaru.json"),
            serde_json::json!({"name":"demo","sources":[{"path":path,"kind":"agent_skill"}]})
                .to_string(),
        )
        .unwrap();
        assert!(authoring::package(temp.path()).is_err());
    }
    fs::write(
        temp.path().join("sikaru.json"),
        r#"{"name":"demo","sources":[{"path":"instructions.md","kind":"agent_md"}]}"#,
    )
    .unwrap();
    fs::write(temp.path().join("instructions.md"), "  ").unwrap();
    assert!(authoring::package(temp.path()).is_err());
}
#[cfg(unix)]
#[test]
fn rejects_symlink_files_and_directories() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("agent");
    authoring::initialize(&root, "demo").unwrap();
    fs::remove_file(root.join("instructions.md")).unwrap();
    fs::write(temp.path().join("secret.md"), "private").unwrap();
    std::os::unix::fs::symlink(temp.path().join("secret.md"), root.join("instructions.md"))
        .unwrap();
    assert!(authoring::package(&root).is_err());
}
#[test]
fn assets_require_parent_skill_and_preserve_bytes() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("agent");
    authoring::initialize(&root, "demo").unwrap();
    fs::create_dir_all(root.join("skills/demo/assets")).unwrap();
    fs::write(root.join("skills/demo/assets/data.bin"), [0, 255, 1]).unwrap();
    let mut manifest = serde_json::json!({"name":"demo","sources":[
        {"path":"instructions.md","kind":"agent_md"},
        {"path":"skills/demo/assets/data.bin","kind":"skill_asset"}]});
    fs::write(root.join("sikaru.json"), manifest.to_string()).unwrap();
    assert!(authoring::package(&root).is_err());
    manifest["sources"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!({"path":"skills/demo/SKILL.md","kind":"agent_skill"}));
    fs::write(
        root.join("skills/demo/SKILL.md"),
        "---\nname: demo\ndescription: Customer skill\n---\nUse the data asset.",
    )
    .unwrap();
    fs::write(root.join("sikaru.json"), manifest.to_string()).unwrap();
    let package = authoring::package(&root).unwrap();
    let asset = package["definition"]["sources"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["kind"] == "skill_asset")
        .unwrap();
    assert_eq!(asset["content"], "AP8B");
    assert_eq!(asset["encoding"], "base64");
}
#[test]
fn rejects_oversized_sources() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("agent");
    authoring::initialize(&root, "demo").unwrap();
    fs::write(root.join("instructions.md"), vec![b'a'; 1_000_001]).unwrap();
    assert!(authoring::package(&root).is_err());
}
#[tokio::test]
async fn dev_uses_shared_executor_and_keeps_agent_inactive() {
    use wiremock::{
        matchers::{body_partial_json, header, method, path},
        Mock, MockServer, ResponseTemplate,
    };
    let server = MockServer::start().await;
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("agent");
    authoring::initialize(&root, "demo").unwrap();
    let package = authoring::package(&root).unwrap();
    let digest = package["contentDigest"].as_str().unwrap();
    let slug = format!("demo-draft-{}", &digest[7..19]);
    Mock::given(method("POST")).and(header("authorization", "Bearer companion-test-key")).and(path("/v1/projects/project/managed-agents"))
        .and(body_partial_json(serde_json::json!({"agentSlug":slug,"status":"inactive","source":{"definition":package["definition"],"contentDigest":digest}})))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({"managedAgent":{"id":"draft"}}))).expect(1).mount(&server).await;
    Mock::given(method("POST")).and(header("authorization", "Bearer companion-test-key"))
        .and(path(format!(
            "/v1/projects/project/harnesses/{slug}/execution-sessions"
        )))
        .and(body_partial_json(
            serde_json::json!({"environment":"draft","tenant_id":"tenant","user_id":"user"}),
        ))
        .respond_with(
            ResponseTemplate::new(201)
                .set_body_json(serde_json::json!({"session":{"id":"session"}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("POST")).and(header("authorization", "Bearer companion-test-key"))
        .and(path(
            "/v1/projects/project/execution-sessions/session/turns",
        ))
        .and(body_partial_json(
            serde_json::json!({"input":{"goal":"Hello"},"idempotency_key":"dev-session"}),
        ))
        .respond_with(
            ResponseTemplate::new(201).set_body_json(serde_json::json!({"run":{"id":"run"}})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let url = server.uri();
    let code =
        tokio::task::spawn_blocking(move || {
            std::process::Command::new(env!("CARGO_BIN_EXE_sikaru-authoring"))
            .env("SIKARU_API_KEY", "companion-test-key")
            .args([
                "dev",
                root.to_str().unwrap(),
                "--project",
                "project",
                "--tenant",
                "tenant",
                "--user",
                "user",
                "--prompt",
                "Hello",
                "--base-url",
                &url,
            ]).output().unwrap()
        })
        .await
        .unwrap();
    assert!(code.status.success(), "{}", String::from_utf8_lossy(&code.stderr));
}
#[test]
fn dev_dry_run_never_contacts_the_server() {
    use fern_cli_sdk::{app::CliApp, openapi::OpenApiBinding};
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("agent");
    authoring::initialize(&root, "demo").unwrap();
    let code = authoring::install(
        CliApp::new("sikaru-authoring")
            .binding(OpenApiBinding::new().spec(include_str!("../../../cli/sikaru/openapi0.json"))),
    )
    .try_run_from([
        "sikaru-authoring",
        "dev",
        root.to_str().unwrap(),
        "--project",
        "project",
        "--tenant",
        "tenant",
        "--user",
        "user",
        "--base-url",
        "http://127.0.0.1:1",
        "--dry-run",
    ]);
    assert_eq!(code, 0);
}
