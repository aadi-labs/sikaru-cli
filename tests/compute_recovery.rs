#![cfg(unix)]
#![allow(dead_code)]
#[path = "../cli/sikaru/compute_config.rs"]
mod config;
#[path = "../cli/sikaru/compute_journal.rs"]
mod journal;
#[path = "../cli/sikaru/compute_process.rs"]
mod process;
use journal::{Binding, Journal};
use serde_json::{json, Value};
use std::{collections::HashMap, time::Duration};
fn setup() -> (tempfile::TempDir, config::Bootstrap) {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().canonicalize().unwrap();
    std::fs::create_dir(path.join("workspace")).unwrap();
    let b=serde_json::from_value(json!({"project_id":"project","session_id":"session","attachment_id":"attachment","owner_epoch":1,
        "workspace_generation":"generation","journal_id":"journal","credential_id":"credential","token":"executor-secret",
        "workspace_provenance":{"kind":"existing_directory","identity":"opaque-customer-identity"},"workspace":path.join("workspace"),"state_dir":path.join("state")})).unwrap();
    (root, b)
}
fn open(b: &config::Bootstrap) -> Journal {
    Journal::open(
        &b.state_dir,
        Binding::from_bootstrap(b).unwrap(),
        "instance".into(),
    )
    .unwrap()
}
fn args(v: Value) -> HashMap<String, Value> {
    serde_json::from_value(v).unwrap()
}
#[test]
fn immutable_receipt_replay_rejects_copied_digest_with_changed_arguments() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let request = json!({"digest":"opaque","command":"echo once"});
    assert!(j.intent("run/tool", request.clone()).unwrap().is_none());
    let receipt = json!({"payload":{"written":true}});
    j.receipt("run/tool", receipt.clone()).unwrap();
    assert_eq!(j.intent("run/tool", request).unwrap(), Some(receipt));
    assert!(j
        .intent(
            "run/tool",
            json!({"digest":"opaque","command":"echo twice"})
        )
        .is_err());
    j.ack("run/tool").unwrap();
    j.ack("run/tool").unwrap();
}
#[test]
fn uncertain_intent_never_reexecutes_after_restart() {
    let (_root, mut b) = setup();
    let mut j = open(&b);
    j.intent("run/tool", json!({"command":"effect"})).unwrap();
    drop(j);
    b.owner_epoch = 2;
    b.credential_id = "next".into();
    assert!(Journal::open_verified(
        &b.state_dir,
        Binding::from_bootstrap(&b).unwrap(),
        "next-instance".into(),
        true
    )
    .is_err());
}
#[test]
fn crash_after_receipt_before_ack_can_reconcile_after_external_teardown() {
    let (_root, mut b) = setup();
    let mut j = open(&b);
    j.intent("run/tool", json!({"command":"effect"})).unwrap();
    j.receipt("run/tool", json!({"payload":{"written":true}}))
        .unwrap();
    j.handle(
        "handle",
        json!({"id":"handle","status":"running","returncode":null}),
    )
    .unwrap();
    drop(j);
    b.owner_epoch = 2;
    b.credential_id = "next".into();
    let j = Journal::open_verified(
        &b.state_dir,
        Binding::from_bootstrap(&b).unwrap(),
        "next-instance".into(),
        true,
    )
    .unwrap();
    assert!(!j.entries["run/tool"].acknowledged);
    assert!(j.entries["run/tool"].receipt.is_some());
    assert_eq!(j.handles["handle"]["status"], "cancelled");
}
#[test]
fn multiple_epoch_headers_cannot_replace_requested_identity() {
    let (_root, mut b) = setup();
    let mut j = open(&b);
    j.mark_clean().unwrap();
    drop(j);
    b.owner_epoch = 2;
    b.credential_id = "two".into();
    let mut j = open(&b);
    j.mark_clean().unwrap();
    drop(j);
    b.owner_epoch = 3;
    b.credential_id = "three".into();
    for field in [
        "attachment_id",
        "workspace_generation",
        "workspace_provenance",
    ] {
        let mut binding = Binding::from_bootstrap(&b).unwrap();
        match field {
            "attachment_id" => binding.attachment_id = "other".into(),
            "workspace_generation" => binding.workspace_generation = "other".into(),
            _ => binding.workspace_provenance.identity = "other".into(),
        }
        assert!(Journal::open(&b.state_dir, binding, "three".into()).is_err());
    }
    let j = open(&b);
    assert_eq!(j.binding.owner_epoch, 3);
}
#[test]
fn exclusive_lock_private_permissions_and_symlinks() {
    use std::os::unix::fs::{symlink, MetadataExt};
    let (root, b) = setup();
    let j = open(&b);
    assert!(Journal::open(
        &b.state_dir,
        Binding::from_bootstrap(&b).unwrap(),
        "other".into()
    )
    .is_err());
    assert_eq!(b.state_dir.metadata().unwrap().mode() & 0o777, 0o700);
    assert_eq!(
        b.state_dir.join("journal.jsonl").metadata().unwrap().mode() & 0o777,
        0o600
    );
    drop(j);
    std::fs::rename(b.state_dir.join("journal.jsonl"), root.path().join("saved")).unwrap();
    symlink(root.path().join("saved"), b.state_dir.join("journal.jsonl")).unwrap();
    assert!(Journal::open(
        &b.state_dir,
        Binding::from_bootstrap(&b).unwrap(),
        "instance".into()
    )
    .is_err());
}
#[test]
fn workspace_replacement_rejects_new_effect_intent() {
    let (_root, b) = setup();
    let mut j = open(&b);
    std::fs::rename(&b.workspace, b.workspace.with_file_name("original")).unwrap();
    std::fs::create_dir(&b.workspace).unwrap();
    assert!(j.intent("run/tool", json!({"effect":true})).is_err());
}
#[tokio::test]
async fn shell_output_uses_byte_offsets_and_preserves_control_characters() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(2));
    let start = p
        .execute(
            "bash.start",
            &args(json!({"command":"printf '\\000\\001hello\\377'"})),
            &mut j,
        )
        .await
        .unwrap();
    let id = &start["id"];
    let end = p
        .execute("bash.wait", &args(json!({"handle_id":id})), &mut j)
        .await
        .unwrap();
    assert_eq!(end["status"], "exited");
    let read = p
        .execute(
            "bash.read",
            &args(json!({"handle_id":id,"offset":0,"limit":100})),
            &mut j,
        )
        .await
        .unwrap();
    assert_eq!(read["next_offset"], 8);
    assert_eq!(read["output"], "\0\u{1}hello�");
    assert_eq!(
        std::fs::read(read["artifact_path"].as_str().unwrap()).unwrap(),
        b"\0\x01hello\xff"
    );
}
#[tokio::test]
async fn leader_exit_terminates_live_descendants_before_reaping() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(2));
    let start = p
        .execute(
            "bash.start",
            &args(json!({"command":"(sleep 0.4; touch escaped) & exit 0"})),
            &mut j,
        )
        .await
        .unwrap();
    p.execute("bash.wait", &args(json!({"handle_id":start["id"]})), &mut j)
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(600)).await;
    assert!(!b.workspace.join("escaped").exists());
}
#[tokio::test]
async fn deadline_and_cancel_stop_the_process_group() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_millis(80));
    let start = p
        .execute(
            "bash.start",
            &args(json!({"command":"sleep 0.4; touch escaped"})),
            &mut j,
        )
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(120)).await;
    p.maintenance(&mut j).unwrap();
    let read = p
        .execute("bash.read", &args(json!({"handle_id":start["id"]})), &mut j)
        .await
        .unwrap();
    assert_eq!(read["timed_out"], true);
    tokio::time::sleep(Duration::from_millis(400)).await;
    assert!(!b.workspace.join("escaped").exists());
}
#[tokio::test]
async fn task_environment_cannot_reintroduce_controller_credentials() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(2));
    let start=p.execute("bash.start",&args(json!({"command":"printf '%s/%s' \"${SIKARU_API_KEY-unset}\" \"$ALLOWED\"","env":{"SIKARU_API_KEY":"controller-secret","SIKARU_EXECUTOR_TOKEN":"executor-secret","ALLOWED":"yes"}})),&mut j).await.unwrap();
    p.execute("bash.wait", &args(json!({"handle_id":start["id"]})), &mut j)
        .await
        .unwrap();
    let read = p
        .execute("bash.read", &args(json!({"handle_id":start["id"]})), &mut j)
        .await
        .unwrap();
    assert_eq!(read["output"], "unset/yes");
}
#[tokio::test]
async fn concurrent_reads_do_not_move_the_output_writer_offset() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(3));
    let start = p
        .execute(
            "bash.start",
            &args(json!({"command":"for i in {1..100}; do printf 0123456789; sleep 0.002; done"})),
            &mut j,
        )
        .await
        .unwrap();
    for _ in 0..25 {
        p.execute(
            "bash.read",
            &args(json!({"handle_id":start["id"],"offset":0,"limit":3})),
            &mut j,
        )
        .await
        .unwrap();
        tokio::time::sleep(Duration::from_millis(4)).await;
    }
    p.execute("bash.wait", &args(json!({"handle_id":start["id"]})), &mut j)
        .await
        .unwrap();
    let read = p
        .execute(
            "bash.read",
            &args(json!({"handle_id":start["id"],"offset":0})),
            &mut j,
        )
        .await
        .unwrap();
    assert_eq!(read["output"], "0123456789".repeat(100));
}
#[tokio::test]
async fn parked_handle_returns_terminal_state_without_restarting() {
    let (_root, mut b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(2));
    let start = p
        .execute(
            "bash.start",
            &args(json!({"command":"echo once >> effect; printf retained; sleep 10"})),
            &mut j,
        )
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(30)).await;
    p.cleanup(&mut j).unwrap();
    j.mark_clean().unwrap();
    drop(p);
    drop(j);
    b.owner_epoch = 2;
    b.credential_id = "two".into();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(2));
    let wait = p
        .execute("bash.wait", &args(json!({"handle_id":start["id"]})), &mut j)
        .await
        .unwrap();
    assert_eq!(wait["status"], "cancelled");
    let read = p
        .execute(
            "bash.read",
            &args(json!({"handle_id":start["id"],"offset":0})),
            &mut j,
        )
        .await
        .unwrap();
    assert_eq!(read["output"], "retained");
    assert_eq!(read["total_bytes"], 8);
    assert_eq!(
        std::fs::read_to_string(b.workspace.join("effect")).unwrap(),
        "once\n"
    );
}

#[path = "../cli/sikaru/compute_workspace.rs"]
mod workspace;
#[path = "../cli/sikaru/compute_workspace_flow.rs"]
mod workspace_flow;
#[path = "../cli/sikaru/compute_runtime.rs"]
mod runtime;
#[path = "../cli/sikaru/compute_transport.rs"]
mod transport;
#[tokio::test]
async fn external_teardown_replays_two_large_receipts_individually_before_ready() {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, Request, ResponseTemplate};
    let (_root, mut b) = setup();
    let mut j = open(&b);
    for id in ["one", "two"] {
        j.intent(id, json!({"command":"already executed"})).unwrap();
        j.receipt(id,json!({"run_id":"run","tool_call_id":id,"tool_provider_id":"provider","capability_name":"compute.execute",
            "idempotency_key":id,"request_digest":"original-opaque-digest","status":"completed","payload":{"output":"x".repeat(180000)}})).unwrap();
    }
    drop(j);
    b.owner_epoch = 2;
    b.credential_id = "new-credential".into();
    let server = MockServer::start().await;
    Mock::given(path("/v1/projects/project/compute-credentials/renew"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"credential_id":"new-credential","expires_at":2000000000.0})),
        )
        .mount(&server)
        .await;
    let base = "/v1/projects/project/compute-attachments/attachment";
    let a = json!({"id":"attachment","session_id":"session","project_id":"project","environment_id":"environment","provider_id":"provider","journal_id":"journal",
        "workspace_generation":"generation","workspace_provenance":{"kind":"existing_directory","identity":"opaque-customer-identity"},"status":"starting","owner_epoch":2,"owner_id":"worker",
        "lease_until":2000000000.0,"lease_ttl_seconds":60,"startup_deadline":2000000000.0,"capabilities":["compute.execute"],"cleanup_status":"confirmed","processes":[],"uncertain_operations":[]});
    for suffix in ["status", "connect", "ready", "cleanup"] {
        Mock::given(path(format!("{base}/{suffix}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(a.clone()))
            .mount(&server)
            .await;
    }
    Mock::given(path(format!("{base}/work"))).respond_with(ResponseTemplate::new(200).set_body_json(json!({"attachment":a,"execution_phase":"terminal","execution":{"run_id":"run","status":"completed","terminal":true,"approval_required":false},"issued_operations":[],"live_handles":[],"operations":[]}))).mount(&server).await;
    Mock::given(method("POST"))
        .and(path(format!("{base}/receipts")))
        .respond_with(|request: &Request| {
            assert!(request.body.len() < 256 * 1024);
            let body: Value = serde_json::from_slice(&request.body).unwrap();
            assert_eq!(body["payload"]["output"].as_str().unwrap().len(), 180000);
            ResponseTemplate::new(200)
                .set_body_json(json!({"created":false,"tool_call_id":body["tool_call_id"]}))
        })
        .expect(2)
        .mount(&server)
        .await;
    Mock::given(path(format!("{base}/reconcile")))
        .and(wiremock::matchers::body_partial_json(
            json!({"receipts":[]}),
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"attachment":a,"receipts":[]})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let result = runtime::serve(b, server.uri(), reqwest::Client::new())
        .await
        .unwrap();
    assert_eq!(result["status"], "completed");
}
#[test]
fn interrupted_record_and_same_epoch_credential_change_are_rejected() {
    use std::io::Write;
    let (_root, mut b) = setup();
    let j = open(&b);
    drop(j);
    b.credential_id = "replacement".into();
    assert!(Journal::open_verified(
        &b.state_dir,
        Binding::from_bootstrap(&b).unwrap(),
        "new-instance".into(),
        true
    )
    .is_err());
    b.owner_epoch = 2;
    std::fs::OpenOptions::new()
        .append(true)
        .open(b.state_dir.join("journal.jsonl"))
        .unwrap()
        .write_all(b"{\"record\":")
        .unwrap();
    assert!(Journal::open_verified(
        &b.state_dir,
        Binding::from_bootstrap(&b).unwrap(),
        "new-instance".into(),
        true
    )
    .is_err());
}
#[tokio::test]
async fn output_overflow_is_bounded_and_stops_owned_process() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(3));
    let start = p
        .execute("bash.start", &args(json!({"command":"yes x"})), &mut j)
        .await
        .unwrap();
    let end = p
        .execute("bash.wait", &args(json!({"handle_id":start["id"]})), &mut j)
        .await
        .unwrap();
    assert_eq!(end["truncated"], true);
    let read = p
        .execute(
            "bash.read",
            &args(json!({"handle_id":start["id"],"offset":0})),
            &mut j,
        )
        .await
        .unwrap();
    assert_eq!(read["total_bytes"], 1048576);
}

#[tokio::test]
async fn file_write_refuses_a_fifo_instead_of_blocking_the_lease_loop() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(1));
    let path = std::ffi::CString::new(b.workspace.join("pipe").to_str().unwrap()).unwrap();
    assert_eq!(unsafe { libc::mkfifo(path.as_ptr(), 0o600) }, 0);
    assert!(p
        .execute(
            "workspace.write_text",
            &args(json!({"path":"pipe","text":"data"})),
            &mut j
        )
        .await
        .is_err());
}

#[tokio::test]
async fn command_deadline_kills_children_without_polling_process_maintenance() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_millis(80));
    let start = p
        .execute(
            "bash.start",
            &args(json!({"command":"exec >/dev/null 2>&1; sleep 0.3; touch escaped"})),
            &mut j,
        )
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(450)).await;
    assert!(!b.workspace.join("escaped").exists());
    let read = p
        .execute("bash.read", &args(json!({"handle_id":start["id"]})), &mut j)
        .await
        .unwrap();
    assert_eq!(read["timed_out"], true);
}

#[tokio::test]
async fn completed_command_does_not_become_timed_out_during_slow_poll() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_millis(80));
    let start = p
        .execute(
            "bash.start",
            &args(json!({"command":"printf complete"})),
            &mut j,
        )
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(200)).await;
    let read = p
        .execute("bash.read", &args(json!({"handle_id":start["id"]})), &mut j)
        .await
        .unwrap();
    assert_eq!(read["timed_out"], false);
    assert_eq!(read["output"], "complete");
}

#[test]
fn os_probe_denial_is_not_evidence_that_children_vanished() {
    assert!(
        process::process_probe::<()>(|| Err(std::io::Error::from_raw_os_error(libc::EPERM)))
            .is_err()
    );
    assert!(
        process::process_probe::<()>(|| Err(std::io::Error::from_raw_os_error(libc::EIO))).is_err()
    );
    assert_eq!(
        process::process_probe::<()>(|| Err(std::io::Error::from_raw_os_error(libc::ESRCH)))
            .unwrap(),
        None
    );
}

#[path = "../cli/sikaru/compute_state.rs"]
mod state;

#[test]
fn workflow_roster_survives_many_completed_jobs_and_retains_unfinished_launch() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().canonicalize().unwrap().join("worker");
    let mut ledger = state::State::open(
        &path,
        Some(json!({"entries":[], "pending":{"launch_intent":true,"id":"unfinished"}})),
    )
    .unwrap();
    for index in 0..700 {
        ledger.value["entries"]
            .as_array_mut()
            .unwrap()
            .push(json!({"id":index,"done":true,"identity":"x".repeat(100)}));
        ledger.save().unwrap();
    }
    let expected = ledger.value.clone();
    drop(ledger);
    let restored = state::State::<Value>::open(&path, None)
        .expect("completed job history must not make unfinished work unrecoverable");
    assert_eq!(restored.value, expected);
    assert!(
        std::fs::metadata(path.join("workflow.jsonl"))
            .unwrap()
            .len()
            < 1024 * 1024
    );
}

#[test]
fn workflow_interrupted_staging_preserves_last_fsynced_launch() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().canonicalize().unwrap().join("worker");
    let expected = json!({"launch_intent":true,"launch_id":"do-not-relaunch"});
    drop(state::State::open(&path, Some(expected.clone())).unwrap());
    std::fs::write(path.join("workflow.next"), b"{\"launch_intent\":").unwrap();
    let restored = state::State::<Value>::open(&path, None).unwrap();
    assert_eq!(restored.value, expected);
}

#[test]
fn workflow_failed_save_preserves_previous_snapshot_and_lock() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().canonicalize().unwrap().join("worker");
    let expected = json!({"launch_intent":true,"launch_id":"original"});
    let mut ledger = state::State::open(&path, Some(expected.clone())).unwrap();
    ledger.value = json!({"oversized":"x".repeat(16 * 1024 * 1024)});
    assert!(ledger.save().is_err());
    assert!(state::State::<Value>::open(&path, None).is_err());
    drop(ledger);
    assert_eq!(
        state::State::<Value>::open(&path, None).unwrap().value,
        expected
    );
}

#[tokio::test]
async fn cancellation_during_shell_child_spawn_proves_group_cleanup() {
    for attempt in 0..40 {
        let (_root, b) = setup();
        let mut journal = open(&b);
        let mut processes = process::Processes::new(&journal, Duration::from_secs(10));
        let started = b.workspace.join("started");
        let handle = processes
            .execute(
                "bash.start",
                &args(json!({"command":"touch started; sleep 10; touch escaped"})),
                &mut journal,
            )
            .await
            .unwrap();
        let end = std::time::Instant::now() + Duration::from_secs(2);
        while !started.exists() {
            assert!(std::time::Instant::now() < end);
            tokio::time::sleep(Duration::from_micros(100)).await;
        }
        let result = processes
            .execute(
                "bash.cancel",
                &args(json!({"handle_id":handle["id"]})),
                &mut journal,
            )
            .await;
        assert!(result.is_ok(), "attempt {attempt}: {result:?}");
        assert_eq!(result.unwrap()["status"], "cancelled");
        assert!(!b.workspace.join("escaped").exists());
    }
}

#[test]
fn acknowledged_history_over_64_mib_remains_recoverable_without_reexecution() {
    let (_root, mut b) = setup();
    let mut j = open(&b);
    let request = json!({"command":"effect exactly once"});
    let receipt = json!({"payload":{"output":"x".repeat(256 * 1024 - 128)}});
    for index in 0..257 {
        let key = format!("operation-{index}");
        assert!(j.intent(&key, request.clone()).unwrap().is_none());
        j.receipt(&key, receipt.clone()).unwrap();
        j.ack(&key).unwrap();
    }
    j.mark_clean().unwrap();
    assert!(b.state_dir.join("journal.jsonl").metadata().unwrap().len() > 64 * 1024 * 1024);
    drop(j);
    b.owner_epoch += 1;
    b.credential_id = "replacement".into();
    let mut restored = open(&b);
    assert_eq!(restored.entries.len(), 257);
    for index in 0..257 {
        let key = format!("operation-{index}");
        assert!(restored.entries[&key].acknowledged);
        assert_eq!(
            restored.intent(&key, request.clone()).unwrap(),
            Some(receipt.clone())
        );
        assert!(restored.intent(&key, json!({"command":"changed"})).is_err());
    }
    restored.mark_clean().unwrap();
    drop(restored);
    b.owner_epoch += 1;
    assert_eq!(open(&b).entries.len(), 257);
}

#[test]
fn journal_compaction_preserves_uncertainty_handles_lock_and_interrupted_staging() {
    let (_root, b) = setup();
    let mut j = open(&b);
    j.intent("uncertain", json!({"command":"may have happened"}))
        .unwrap();
    j.intent("unacked", json!({"command":"already happened"}))
        .unwrap();
    let receipt = json!({"payload":{"written":true}});
    j.receipt("unacked", receipt.clone()).unwrap();
    // Reach the real compaction threshold using accepted handle observations.
    for offset in 0..66 {
        j.handle(
            "process",
            json!({"status":"running","offset":offset,"observation":"x".repeat(1024 * 1024 - 128)}),
        )
        .unwrap();
    }
    j.handle("process", json!({"status":"cancelled","offset":66}))
        .unwrap();
    drop(j);
    let ledger = b.state_dir.join("journal.jsonl");
    let before = ledger.metadata().unwrap().len();
    assert!(before > 64 * 1024 * 1024);
    // A staging obstruction must leave the authoritative ledger intact.
    std::fs::create_dir(b.state_dir.join("journal.next")).unwrap();
    assert!(Journal::open(
        &b.state_dir,
        Binding::from_bootstrap(&b).unwrap(),
        "instance".into()
    )
    .is_err());
    assert_eq!(ledger.metadata().unwrap().len(), before);
    std::fs::remove_dir(b.state_dir.join("journal.next")).unwrap();
    std::fs::write(b.state_dir.join("journal.next"), b"{\"record\":").unwrap();
    let mut restored = open(&b);
    assert!(ledger.metadata().unwrap().len() < 4096);
    assert!(restored.has_uncertain_effects());
    assert!(restored
        .intent("uncertain", json!({"command":"may have happened"}))
        .is_err());
    assert_eq!(
        restored
            .intent("unacked", json!({"command":"already happened"}))
            .unwrap(),
        Some(receipt)
    );
    assert!(!restored.entries["unacked"].acknowledged);
    assert_eq!(
        restored.handles["process"],
        json!({"status":"cancelled","offset":66})
    );
    assert!(Journal::open(
        &b.state_dir,
        Binding::from_bootstrap(&b).unwrap(),
        "instance".into()
    )
    .is_err());
    restored.ack("unacked").unwrap();
    drop(restored);
    assert!(open(&b).entries["unacked"].acknowledged);
}

#[test]
fn journal_recovery_rejects_oversized_records_and_incomplete_large_tail() {
    use std::io::Write;
    for tail in [
        vec![b'x'; 1024 * 1024 + 2],
        b"{\"record\":\"Clean\"}".to_vec(),
    ] {
        let (_root, b) = setup();
        drop(open(&b));
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(b.state_dir.join("journal.jsonl"))
            .unwrap();
        file.write_all(&tail).unwrap();
        assert!(Journal::open(
            &b.state_dir,
            Binding::from_bootstrap(&b).unwrap(),
            "instance".into()
        )
        .is_err());
    }
}

#[tokio::test]
async fn bash_run_returns_exit_and_first_page_without_killing_for_page_limit() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(3));
    let result = p
        .execute(
            "bash.run",
            &args(json!({"command":"printf abcdef", "limit":3})),
            &mut j,
        )
        .await
        .unwrap();
    assert_eq!(result["status"], "exited");
    assert_eq!(result["returncode"], 0);
    assert_eq!(result["output"], "abc");
    assert_eq!(result["offset"], 0);
    assert_eq!(result["next_offset"], 3);
    assert_eq!(result["omitted_after"], 3);
    let rest = p
        .execute(
            "bash.read",
            &args(json!({"handle_id":result["id"],"offset":3})),
            &mut j,
        )
        .await
        .unwrap();
    assert_eq!(rest["output"], "def");
    p.cleanup(&mut j).unwrap();
}

#[tokio::test]
async fn bash_run_yield_preserves_process_for_later_wait() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(3));
    let result = p
        .execute(
            "bash.run",
            &args(json!({"command":"sleep 0.15; printf done", "yield_seconds":0})),
            &mut j,
        )
        .await
        .unwrap();
    assert_eq!(result["status"], "running");
    assert_eq!(result["timed_out"], false);
    let end = p
        .execute(
            "bash.wait",
            &args(json!({"handle_id":result["id"]})),
            &mut j,
        )
        .await
        .unwrap();
    assert_eq!(end["returncode"], 0);
    p.cleanup(&mut j).unwrap();
}

#[tokio::test]
async fn bash_run_invalid_observation_never_spawns() {
    for extra in [
        json!({"yield_seconds":-1}),
        json!({"yield_seconds":"bad"}),
        json!({"limit":0}),
        json!({"limit":1.5}),
    ] {
        let (_root, b) = setup();
        let mut j = open(&b);
        let mut p = process::Processes::new(&j, Duration::from_secs(3));
        let mut request = args(extra);
        request.insert("command".into(), json!("touch effect"));
        assert!(p.execute("bash.run", &request, &mut j).await.is_err());
        p.cleanup(&mut j).unwrap();
        assert!(!b.workspace.join("effect").exists());
        assert!(j.handles.is_empty());
    }
}

#[tokio::test]
async fn bash_run_interrupted_wait_retains_operation_handle_and_refuses_replay() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(3));
    let request = json!({"command":"printf once >> effect; sleep 1", "yield_seconds":2});
    j.intent("run/tool", request.clone()).unwrap();
    let result = tokio::time::timeout(
        Duration::from_millis(60),
        p.execute_owned("bash.run", &args(request.clone()), &mut j, Some("run/tool")),
    )
    .await;
    assert!(result.is_err());
    assert_eq!(j.handles.len(), 1);
    assert_eq!(
        j.handles.values().next().unwrap()["operation_key"],
        "run/tool"
    );
    assert!(j.intent("run/tool", request).is_err());
    p.cleanup(&mut j).unwrap();
    assert_eq!(
        std::fs::read_to_string(b.workspace.join("effect")).unwrap(),
        "once"
    );
}

#[tokio::test]
async fn wrong_process_handle_can_be_corrected_without_restarting_command() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut processes = process::Processes::new(&j, Duration::from_secs(30));
    let started = processes
        .execute(
            "bash.run",
            &args(json!({"command":"printf once >> marker; printf answer", "yield_seconds":0})),
            &mut j,
        )
        .await
        .unwrap();
    for method in ["bash.read", "bash.wait", "bash.cancel"] {
        let observation = processes
            .execute(
                method,
                &args(json!({"handle_id":"tool-call-not-process"})),
                &mut j,
            )
            .await
            .unwrap();
        assert_eq!(observation["error"]["code"], "unknown_process_handle");
        assert_eq!(observation["status"], "error");
        assert_eq!(observation["handle_id"], "tool-call-not-process");
    }
    processes
        .execute(
            "bash.wait",
            &args(json!({"handle_id":started["id"]})),
            &mut j,
        )
        .await
        .unwrap();
    let output = processes
        .execute(
            "bash.read",
            &args(json!({"handle_id":started["id"]})),
            &mut j,
        )
        .await
        .unwrap();
    assert_eq!(output["output"], "answer");
    assert_eq!(
        std::fs::read_to_string(b.workspace.join("marker")).unwrap(),
        "once"
    );
}

// ---- Event-driven waits: one call returns on the event, bounded by its deadline ----

async fn start(p: &mut process::Processes, j: &mut Journal, command: &str) -> String {
    let started = p
        .execute("bash.start", &args(json!({ "command": command })), j)
        .await
        .unwrap();
    started["id"].as_str().unwrap().to_owned()
}
fn elapsed_near(started: std::time::Instant, event: f64, slack: f64) {
    let seconds = started.elapsed().as_secs_f64();
    assert!(
        seconds >= event - 0.05 && seconds <= event + slack,
        "returned after {seconds:.3}s, expected about {event}s"
    );
}
#[tokio::test]
async fn bash_wait_returns_once_on_exit_not_at_the_deadline() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(30));
    let id = start(&mut p, &mut j, "sleep 1.2; printf done").await;
    let started = std::time::Instant::now();
    let end = p
        .execute(
            "bash.wait",
            &args(json!({"handle_id": id, "timeout": 20})),
            &mut j,
        )
        .await
        .unwrap();
    elapsed_near(started, 1.2, 0.3);
    assert_eq!(end["status"], "exited");
    assert_eq!(end["returncode"], 0);
    p.cleanup(&mut j).unwrap();
}
#[tokio::test]
async fn failing_command_returns_at_once_with_its_status() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(30));
    let started = std::time::Instant::now();
    let result = p
        .execute(
            "bash.run",
            &args(json!({"command": "echo broken >&2; exit 3", "yield_seconds": 20})),
            &mut j,
        )
        .await
        .unwrap();
    assert!(started.elapsed() < Duration::from_millis(500));
    assert_eq!(result["status"], "exited");
    assert_eq!(result["returncode"], 3);
    assert_eq!(result["output"], "broken\n");
    p.cleanup(&mut j).unwrap();
}
async fn wait_for(
    p: &mut process::Processes,
    j: &mut Journal,
    conditions: Value,
    timeout: f64,
) -> Value {
    p.execute(
        "bash.wait_for",
        &args(json!({"conditions": conditions, "timeout": timeout})),
        j,
    )
    .await
    .unwrap()
}
#[tokio::test]
async fn wait_for_fires_once_near_each_kind_of_event() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(30));
    // Process exit.
    let id = start(&mut p, &mut j, "sleep 0.8; exit 7").await;
    let started = std::time::Instant::now();
    let r = wait_for(
        &mut p,
        &mut j,
        json!([{"kind":"exit","handle_id":id}]),
        10.0,
    )
    .await;
    elapsed_near(started, 0.8, 0.3);
    assert_eq!(r["status"], "fired");
    assert_eq!(
        r["conditions"][0]["observed"],
        json!({"status":"exited","returncode":7})
    );
    assert_eq!(r["conditions"][0]["reason"], Value::Null);
    // A path appearing, relative to the workspace.
    start(&mut p, &mut j, "sleep 0.8; touch ready").await;
    let started = std::time::Instant::now();
    let r = wait_for(
        &mut p,
        &mut j,
        json!([{"kind":"path","path":"ready","state":"exists"}]),
        10.0,
    )
    .await;
    elapsed_near(started, 0.8, 0.35);
    assert_eq!(r["conditions"][0]["fired"], true);
    // A log line, while the process keeps running.
    let id = start(
        &mut p,
        &mut j,
        "sleep 0.8; echo 'server listening'; sleep 20",
    )
    .await;
    let started = std::time::Instant::now();
    let r = wait_for(
        &mut p,
        &mut j,
        json!([{"kind":"log","handle_id":id,"pattern":"listening"}]),
        10.0,
    )
    .await;
    elapsed_near(started, 0.8, 0.3);
    assert_eq!(r["conditions"][0]["observed"], json!({"match":"listening"}));
    p.cleanup(&mut j).unwrap();
}
#[tokio::test]
async fn wait_for_port_and_http_fire_when_the_server_starts() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(30));
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let server = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(800)).await;
        let listener = tokio::net::TcpListener::bind(("127.0.0.1", port))
            .await
            .unwrap();
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                use tokio::io::{AsyncReadExt, AsyncWriteExt};
                let mut buffer = [0; 1024];
                let _ = socket.read(&mut buffer).await;
                let _ = socket
                    .write_all(b"HTTP/1.1 204 No Content\r\nconnection: close\r\n\r\n")
                    .await;
            });
        }
    });
    let started = std::time::Instant::now();
    let r = wait_for(
        &mut p,
        &mut j,
        json!([{"kind":"port","port":port,"host":"127.0.0.1"}]),
        10.0,
    )
    .await;
    elapsed_near(started, 0.8, 0.35);
    assert_eq!(r["status"], "fired");
    let url = format!("http://127.0.0.1:{port}/health");
    let r = wait_for(
        &mut p,
        &mut j,
        json!([{"kind":"http","url":url,"status":204}]),
        10.0,
    )
    .await;
    assert_eq!(r["conditions"][0]["observed"], json!({"status":204}));
    server.abort();
}
#[tokio::test]
async fn wait_for_reports_unfired_conditions_without_raising() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(30));
    let started = std::time::Instant::now();
    let r = wait_for(
        &mut p,
        &mut j,
        json!([{"kind":"path","path":"never","state":"exists"}]),
        0.4,
    )
    .await;
    elapsed_near(started, 0.4, 0.3);
    assert_eq!(r["status"], "timeout");
    assert_eq!(r["conditions"][0]["reason"], "deadline");
    assert!(r["elapsed_seconds"].as_f64().unwrap() >= 0.4);
    // Log patterns that can no longer appear, and one this engine cannot watch (look-ahead).
    let id = start(&mut p, &mut j, "echo nothing here").await;
    let r = wait_for(
        &mut p,
        &mut j,
        json!([{"kind":"log","handle_id":id,"pattern":"re+ady"},
               {"kind":"log","handle_id":id,"pattern":"(?=ready)"}]),
        10.0,
    )
    .await;
    assert_eq!(r["status"], "unfired");
    assert_eq!(r["conditions"][0]["reason"], "exited");
    assert_eq!(r["conditions"][1]["reason"], "unsupported");
    // The first to fire wins; the others are reported pending.
    let id = start(&mut p, &mut j, "sleep 0.3").await;
    let r = wait_for(
        &mut p,
        &mut j,
        json!([{"kind":"path","path":"never","state":"exists"},{"kind":"exit","handle_id":id}]),
        10.0,
    )
    .await;
    assert_eq!(r["status"], "fired");
    assert_eq!(r["conditions"][0]["reason"], "pending");
    assert_eq!(r["conditions"][1]["fired"], true);
    for invalid in [
        json!([]),
        json!([{"kind":"exit","handle_id":"short"}]),
        json!([{"kind":"port","port":70000,"host":"127.0.0.1"}]),
        json!([{"kind":"path","path":"x","state":"exists","extra":1}]),
        json!([{"kind":"http","url":"file:///etc/hosts","status":200}]),
    ] {
        assert!(p
            .execute(
                "bash.wait_for",
                &args(json!({"conditions": invalid, "timeout": 1})),
                &mut j
            )
            .await
            .is_err());
    }
    p.cleanup(&mut j).unwrap();
}
#[tokio::test]
async fn next_completed_returns_the_first_finished_job_as_a_notice() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(30));
    let slow = start(&mut p, &mut j, "sleep 3").await;
    let fast = start(&mut p, &mut j, "sleep 0.6; printf 'built ok'; exit 2").await;
    let started = std::time::Instant::now();
    let r = p
        .execute(
            "jobs.next_completed",
            &args(json!({"handle_ids": [slow, fast], "timeout": 10})),
            &mut j,
        )
        .await
        .unwrap();
    elapsed_near(started, 0.6, 0.3);
    assert_eq!(
        r["completed"],
        json!({"id": fast, "status": "exited", "returncode": 2, "tail": "built ok", "omitted_before": 0})
    );
    assert_eq!(r["pending"], json!([slow]));
    let r = p
        .execute(
            "jobs.next_completed",
            &args(json!({"handle_ids": [slow], "timeout": 0.2})),
            &mut j,
        )
        .await
        .unwrap();
    assert_eq!(r, json!({"completed": null, "pending": [slow]}));
    p.cleanup(&mut j).unwrap();
}
#[tokio::test]
async fn log_conditions_match_regular_expressions_and_report_the_matched_text() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(30));
    let id = start(
        &mut p,
        &mut j,
        "echo 'listening on port'; sleep 0.4; echo 'server listening on 8080'; sleep 20",
    )
    .await;
    let r = wait_for(
        &mut p,
        &mut j,
        json!([{"kind":"log","handle_id":id,"pattern":r"listening on \d{2,5}|^fatal"}]),
        10.0,
    )
    .await;
    assert_eq!(r["status"], "fired");
    assert_eq!(
        r["conditions"][0]["observed"],
        json!({"match": "listening on 8080"})
    );
    p.cleanup(&mut j).unwrap();
}
#[tokio::test]
async fn completion_notices_carry_the_requested_tail_and_candidates_must_be_named() {
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(30));
    let id = start(&mut p, &mut j, "printf 'compiling\nbuilt ok'").await;
    let r = p
        .execute(
            "jobs.next_completed",
            &args(json!({"handle_ids": [id], "timeout": 10, "tail_bytes": 2})),
            &mut j,
        )
        .await
        .unwrap();
    assert_eq!(r["completed"]["tail"], "ok");
    assert_eq!(r["completed"]["omitted_before"], 16);
    for unnamed in [json!({"timeout": 1}), json!({"handle_ids": null, "timeout": 1})] {
        assert!(p
            .execute("jobs.next_completed", &args(unnamed), &mut j)
            .await
            .is_err());
    }
    let zero = json!({"handle_ids": [id], "timeout": 1, "tail_bytes": 0});
    assert!(p
        .execute("jobs.next_completed", &args(zero), &mut j)
        .await
        .is_err());
    p.cleanup(&mut j).unwrap();
}
/// Executor results deserialize into the generated contract types and survive them.
#[tokio::test]
async fn condition_wait_results_are_generated_contract_payloads() {
    use sikaru_sdk::api::{NextCompletedResult, WaitForResult};
    let (_root, b) = setup();
    let mut j = open(&b);
    let mut p = process::Processes::new(&j, Duration::from_secs(30));
    let id = start(&mut p, &mut j, "echo ready; exit 4").await;
    let fired = wait_for(
        &mut p,
        &mut j,
        json!([{"kind":"log","handle_id":id,"pattern":"re(a)dy"},{"kind":"path","path":"never","state":"exists"}]),
        5.0,
    )
    .await;
    let typed: WaitForResult = serde_json::from_value(fired.clone()).unwrap();
    assert_eq!(typed.conditions.len(), 2);
    let done = p
        .execute(
            "jobs.next_completed",
            &args(json!({"handle_ids": [id], "timeout": 5})),
            &mut j,
        )
        .await
        .unwrap();
    let typed: NextCompletedResult = serde_json::from_value(done.clone()).unwrap();
    assert_eq!(typed.completed.map(|n| n.returncode), Some(Some(4)));
    assert_eq!(done["completed"]["tail"], "ready\n");
    p.cleanup(&mut j).unwrap();
}
