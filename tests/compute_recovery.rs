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
