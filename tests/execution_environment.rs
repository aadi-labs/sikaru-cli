use std::process::Command;

#[test]
fn direct_run_uses_environment_and_workspace_provenance() {
    let output = Command::new(env!("CARGO_BIN_EXE_sikaru"))
        .args([
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
            "{}",
            "--product-context",
            "{}",
            "--policy",
            "{}",
            "--compute-environment-id",
            "environment",
            "--compute-workspace-provenance",
            r#"{"kind":"existing_directory","identity":"workspace"}"#,
            "--dry-run",
            "--format",
            "json",
        ])
        .env("SIKARU_API_KEY", "test-only")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["body"]["compute_environment_id"], "environment");
    assert_eq!(
        value["body"]["compute_workspace_provenance"]["identity"],
        "workspace"
    );
    assert!(value["body"].get("execution_environment").is_none());
    assert!(value["body"].get("compute_provider_id").is_none());
}
