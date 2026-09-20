use std::process::Command;

#[test]
fn execution_environment_is_managed_or_local_only() {
    for environment in [None, Some("managed"), Some("local"), Some("box")] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_sikaru"));
        command.args(["runs", "start", "--project-id", "project", "--harness-id", "agent",
            "--tenant-id", "tenant", "--user-id", "user", "--input", "{}",
            "--product-context", "{}", "--policy", "{}", "--dry-run", "--format", "json"]);
        if let Some(value) = environment {
            command.args(["--execution-environment", value]);
        }
        if environment == Some("local") {
            command.args(["--compute-provider-id", "my-machine"]);
        }
        let output = command.env("SIKARU_API_KEY", "test-only").output().unwrap();
        if environment == Some("box") {
            assert!(!output.status.success());
            continue;
        }
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(value["body"].get("compute_provider").is_none());
        if let Some(environment) = environment {
            assert_eq!(value["body"]["execution_environment"], environment);
        } else {
            assert!(value["body"]["execution_environment"].is_null()
                || value["body"]["execution_environment"] == "managed");
        }
    }
}
