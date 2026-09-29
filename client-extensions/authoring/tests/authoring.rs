#[path = "../../../cli/sikaru/commands.rs"]
mod commands;

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
async fn dev_tests_saved_revision_without_creating_an_agent() {
    use wiremock::{
        matchers::{body_partial_json, header, method, path},
        Mock, MockServer, ResponseTemplate,
    };
    let server = MockServer::start().await;
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("agent");
    authoring::initialize(&root, "demo").unwrap();
    let slug = "demo";
    Mock::given(method("POST")).and(path("/v1/projects/project/managed-agents"))
        .respond_with(ResponseTemplate::new(500)).expect(0).mount(&server).await;
    Mock::given(method("POST")).and(header("authorization", "Bearer companion-test-key"))
        .and(path(format!(
            "/v1/projects/project/harnesses/{slug}/execution-sessions"
        )))
        .and(body_partial_json(
            serde_json::json!({"environment":"draft","draft_revision":7,"tenant_id":"tenant","user_id":"user"}),
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
                "demo",
                "--revision",
                "7",
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
    assert!(code.status.success(), "stderr={} stdout={}", String::from_utf8_lossy(&code.stderr), String::from_utf8_lossy(&code.stdout));
}
#[test]
fn dev_dry_run_never_contacts_the_server() {
    use fern_cli_sdk::{app::CliApp, openapi::OpenApiBinding};
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("agent");
    authoring::initialize(&root, "demo").unwrap();
    let code = authoring::install(
        CliApp::new("sikaru-authoring")
            .binding(OpenApiBinding::new().commands(commands::description())),
    )
    .try_run_from([
        "sikaru-authoring",
        "dev",
        "demo",
        "--revision",
        "7",
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

fn write_manifest(root: &std::path::Path, sections: serde_json::Value) {
    let mut manifest =
        serde_json::json!({"name":"demo","sources":[{"path":"instructions.md","kind":"agent_md"}]});
    for (key, value) in sections.as_object().unwrap() {
        manifest[key] = value.clone();
    }
    fs::write(root.join("sikaru.json"), manifest.to_string()).unwrap();
}

fn agent_with(sections: serde_json::Value) -> tempfile::TempDir {
    let temp = tempfile::tempdir().unwrap();
    fs::write(
        temp.path().join("instructions.md"),
        "Research the question.",
    )
    .unwrap();
    write_manifest(temp.path(), sections);
    temp
}

#[test]
fn default_init_manifest_has_only_name_and_sources() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("agent");
    authoring::initialize(&root, "demo").unwrap();
    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("sikaru.json")).unwrap()).unwrap();
    let keys: Vec<_> = manifest.as_object().unwrap().keys().cloned().collect();
    assert_eq!(keys, ["name", "sources"]);
}

#[test]
fn init_with_capabilities_scaffolds_sections_that_package() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("agent");
    authoring::initialize_with_capabilities(&root, "demo").unwrap();
    let definition = authoring::package(&root).unwrap()["definition"].clone();
    assert_eq!(definition["web"]["provider"], "sikaru");
    assert_eq!(definition["setup"]["commands"], serde_json::json!([]));
    assert!(definition["tools"].is_object());
}

#[test]
fn declared_capability_sections_are_packaged_as_authored() {
    let sections = serde_json::json!({
        "web": {"provider": {"connection_id": "conn_search", "tool": "search"},
                "allow_domains": ["docs.example.com"], "block_domains": ["ads.example.com"]},
        "tools": {"bash": {"policy": "require_approval"}, "memory": {"enabled": false}},
        "setup": {"packages": {"pip": ["pandas==2.2"], "npm": ["@scope/tool"]},
                  "commands": ["make deps"],
                  "repos": [{"url": "https://git.example.com/team/app", "path": "src/app", "git_credential": "gitcred_1"},
                            {"url": "https://git.example.com/team/lib", "path": "src/lib"}]}});
    let temp = agent_with(sections.clone());
    let definition = authoring::package(temp.path()).unwrap()["definition"].clone();
    for key in ["web", "tools", "setup"] {
        assert_eq!(definition[key], sections[key], "{key}");
    }
}

#[test]
fn manifest_without_capabilities_packages_sources_only() {
    let temp = agent_with(serde_json::json!({}));
    let definition = authoring::package(temp.path()).unwrap()["definition"].clone();
    let keys: Vec<_> = definition.as_object().unwrap().keys().cloned().collect();
    assert_eq!(keys, ["schema", "sources"]);
}

#[test]
fn invalid_capability_sections_are_rejected_locally() {
    let repo = |url: &str, path: &str| serde_json::json!({"url": url, "path": path});
    for sections in [
        serde_json::json!({"web": {"allow_domains": ["Docs.Example.com"]}}),
        serde_json::json!({"web": {"allow_domains": ["localhost"]}}),
        serde_json::json!({"web": {"allow_domains": ["a.example.com"], "block_domains": ["a.example.com"]}}),
        serde_json::json!({"web": {"provider": "other"}}),
        serde_json::json!({"web": {"proxy": true}}),
        serde_json::json!({"tools": {"shell": {"enabled": true}}}),
        serde_json::json!({"tools": {"bash": {"policy": "maybe"}}}),
        serde_json::json!({"tools": {"bash": {"enabled": "true"}}}),
        serde_json::json!({"setup": {"repos": [repo("http://git.example.com/team/app", "app")]}}),
        serde_json::json!({"setup": {"repos": [repo("https://user:pw@git.example.com/team/app", "app")]}}),
        serde_json::json!({"setup": {"repos": [repo("https://git.example.com:8443/team/app", "app")]}}),
        serde_json::json!({"setup": {"repos": [repo("https://git.example.com/", "app")]}}),
        serde_json::json!({"setup": {"repos": [repo("https://git.example.com/a?ref=main", "app")]}}),
        serde_json::json!({"setup": {"repos": [repo("https://git.example.com/a", "../app")]}}),
        serde_json::json!({"setup": {"repos": [repo("https://git.example.com/a", "app"), repo("https://git.example.com/b", "app/lib")]}}),
        serde_json::json!({"setup": {"commands": ["   "]}}),
        serde_json::json!({"setup": {"packages": {"pip": ["two words"]}}}),
        serde_json::json!({"setup": {"git": true}}),
    ] {
        let temp = agent_with(sections.clone());
        assert!(
            authoring::package(temp.path()).is_err(),
            "accepted {sections}"
        );
    }
}

fn violation_pairs(
    definition: serde_json::Value,
    ceilings: serde_json::Value,
) -> Vec<(String, String)> {
    authoring::ceiling_violations(&definition, &ceilings)
        .unwrap()
        .iter()
        .map(|v| {
            (
                v["ceiling"].as_str().unwrap().to_owned(),
                v["field"].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}

#[test]
fn ceiling_violations_name_each_explicit_conflict() {
    let definition = serde_json::json!({
        "schema": "sikaru.agent.contract.v1",
        "web": {"allow_domains": ["api.tracker.example", "docs.example.com"]},
        "tools": {"bash": {"policy": "allow"}, "memory": {"policy": "deny"}},
        "setup": {"commands": ["make deps"],
                  "repos": [{"url": "https://git.internal.example/team/app", "path": "app"}]}});
    let ceilings = serde_json::json!({
        "egressEnabled": false, "domainDenylist": ["*.tracker.example"],
        "disallowedTools": ["bash", "memory", "web_fetch"], "allowedGitHosts": ["git.example.com"]});
    let pair = |c: &str, f: &str| (c.to_owned(), f.to_owned());
    assert_eq!(
        violation_pairs(definition, ceilings),
        vec![
            pair("egress", "setup.commands"),
            pair("egress", "setup.repos"),
            pair("domain_denylist", "web.allow_domains"),
            pair("disallowed_tools", "tools.bash"),
            pair("allowed_git_hosts", "setup.repos"),
        ]
    );
}

#[test]
fn reach_left_unset_never_violates_a_ceiling() {
    let definition = serde_json::json!({"schema": "sikaru.agent.contract.v1", "tools": {}, "web": {"provider": "sikaru"}});
    let ceilings = serde_json::json!({"ceilings": {"egressEnabled": false, "disallowedTools": ["bash"],
        "allowedGitHosts": []}, "canEdit": false});
    assert!(violation_pairs(definition, ceilings).is_empty());
}

#[test]
fn check_reports_ceiling_violations_and_fails() {
    let temp = agent_with(serde_json::json!({"tools": {"bash": {"enabled": true}}}));
    let ceilings = temp.path().join("ceilings.json");
    fs::write(
        &ceilings,
        r#"{"ceilings":{"disallowedTools":["bash"]},"canEdit":false}"#,
    )
    .unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_sikaru-authoring"))
        .args([
            "check",
            temp.path().to_str().unwrap(),
            "--ceilings",
            ceilings.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        report["error"]["message"]
            .as_str()
            .unwrap()
            .contains("tools.bash"),
        "{report}"
    );
    fs::write(&ceilings, r#"{"disallowedTools":["memory"]}"#).unwrap();
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_sikaru-authoring"))
        .args([
            "check",
            temp.path().to_str().unwrap(),
            "--ceilings",
            ceilings.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn init_capabilities_flag_selects_the_scaffold() {
    let temp = tempfile::tempdir().unwrap();
    let plain = temp.path().join("plain");
    let scaffolded = temp.path().join("scaffolded");
    for (root, flag) in [(&plain, None), (&scaffolded, Some("--capabilities"))] {
        let mut args = vec!["init", root.to_str().unwrap()];
        args.extend(flag);
        let output = std::process::Command::new(env!("CARGO_BIN_EXE_sikaru-authoring"))
            .args(args)
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stdout));
    }
    let manifest = |root: &std::path::Path| -> serde_json::Value {
        serde_json::from_slice(&fs::read(root.join("sikaru.json")).unwrap()).unwrap()
    };
    assert!(manifest(&plain).get("web").is_none());
    assert_eq!(manifest(&scaffolded)["web"]["provider"], "sikaru");
}
