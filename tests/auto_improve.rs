use std::process::Command;

#[test]
fn startup_flag_is_optional_and_sent_as_a_boolean() {
    for (flag, expected) in [
        (None, None),
        (Some("true"), Some(true)),
        (Some("false"), Some(false)),
    ] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_sikaru"));
        command.args([
            "runs",
            "start",
            "--project-id",
            "project",
            "--harness-id",
            "agent",
            "--tenant-id",
            "tenant",
            "--user-id",
            "user",
            "--input",
            "{\"goal\":\"hello\"}",
            "--product-context",
            "{}",
            "--policy",
            "{}",
            "--dry-run",
            "--format",
            "json",
        ]);
        if let Some(flag) = flag {
            command.args(["--auto-improve", flag]);
        }
        let output = command.env("SIKARU_API_KEY", "test-only").output().unwrap();
        assert!(
            output.status.success(),
            "{} {}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
        let stdout = String::from_utf8(output.stdout).unwrap();
        if let Some(value) = expected {
            assert!(
                stdout.contains(&format!("\"auto_improve\": {value}")),
                "{stdout}"
            );
        } else {
            assert!(!stdout.contains("auto_improve"), "{stdout}");
        }
    }
}

#[test]
fn session_startup_flag_is_forwarded() {
    let output = Command::new(env!("CARGO_BIN_EXE_sikaru"))
        .args([
            "execution-sessions",
            "create",
            "--project-id",
            "project",
            "--harness-id",
            "agent",
            "--tenant-id",
            "tenant",
            "--user-id",
            "user",
            "--auto-improve",
            "true",
            "--dry-run",
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["body"]["auto_improve"], true);
}

#[test]
fn optional_workspace_provenance_flags_preserve_values() {
    let cases: &[(&[&str], bool)] = &[
        (&[], true),
        (&["--compute-workspace-provenance", "null"], true),
        (&["--compute-workspace-provenance", "{}"], true),
        (
            &["--compute-workspace-provenance", r#"{"kind":"container"}"#],
            true,
        ),
        (
            &[
                "--compute-workspace-provenance",
                r#"{"kind":"container","identity":"owned"}"#,
            ],
            true,
        ),
        (&["--compute-workspace-provenance.kind", "container"], false),
        (
            &[
                "--compute-workspace-provenance.kind",
                "container",
                "--compute-workspace-provenance.identity",
                "owned",
            ],
            true,
        ),
        (&["--json", r#"{"compute_workspace_provenance":{}}"#], true),
        (
            &[
                "--json",
                r#"{"compute_workspace_provenance":{"kind":"sandbox","identity":"owned"}}"#,
            ],
            true,
        ),
    ];
    for (extra, success) in cases {
        let mut command = Command::new(env!("CARGO_BIN_EXE_sikaru"));
        command.args([
            "runs",
            "start",
            "--project-id",
            "project",
            "--harness-id",
            "agent",
            "--dry-run",
            "--format",
            "json",
        ]);
        if let ["--json", value] = *extra {
            let mut request: serde_json::Value = serde_json::from_str(value).unwrap();
            request.as_object_mut().unwrap().extend(
                serde_json::json!({
                    "tenant_id":"tenant", "user_id":"user", "input":{},
                    "product_context":{}, "policy":{}
                })
                .as_object()
                .unwrap()
                .clone(),
            );
            command.args(["--json", &request.to_string()]);
        } else {
            command
                .args([
                    "--tenant-id",
                    "tenant",
                    "--user-id",
                    "user",
                    "--input",
                    "{}",
                    "--product-context",
                    "{}",
                    "--policy",
                    "{}",
                ])
                .args(*extra);
        }
        let output = command.env("SIKARU_API_KEY", "test-only").output().unwrap();
        assert_eq!(
            output.status.success(),
            *success,
            "{extra:?}: {} {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        if *success {
            let body: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
            let expected = match *extra {
                [] => None,
                ["--compute-workspace-provenance", value] => {
                    Some(serde_json::from_str(value).unwrap())
                }
                ["--json", value] => Some(
                    serde_json::from_str::<serde_json::Value>(value).unwrap()
                        ["compute_workspace_provenance"]
                        .clone(),
                ),
                _ => Some(serde_json::json!({"kind":"container", "identity":"owned"})),
            };
            assert_eq!(
                body["body"].get("compute_workspace_provenance"),
                expected.as_ref()
            );
        }
    }
}

#[test]
fn optional_object_schema_default_does_not_require_or_inject_it() {
    use fern_cli_sdk::{app::CliApp, openapi::OpenApiBinding};
    let schema = r#"{
        "openapi":"3.1.0", "info":{"title":"Optional defaults","version":"1"},
        "servers":[{"url":"http://127.0.0.1:1"}],
        "paths":{"/items":{"post":{"operationId":"create", "x-fern-sdk-group-name":"items",
            "requestBody":{"content":{"application/json":{"schema":{
                "type":"object", "properties":{"settings":{"type":"object", "default":{},
                    "required":["region"], "properties":{"region":{"type":"string"}}}}
            }}}}, "responses":{"200":{"description":"ok"}}
        }}}
    }"#;
    let mut output = Vec::new();
    let code = CliApp::new("fixture")
        .binding(OpenApiBinding::new().spec(schema))
        .try_run_from_with_output(
            [
                "fixture",
                "items",
                "create",
                "--dry-run",
                "--format",
                "json",
            ],
            &mut output,
        );
    assert_eq!(code, 0, "{}", String::from_utf8_lossy(&output));
    let value: serde_json::Value = serde_json::from_slice(&output).unwrap();
    assert!(value["body"].get("settings").is_none(), "{value}");
}
