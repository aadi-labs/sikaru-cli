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
