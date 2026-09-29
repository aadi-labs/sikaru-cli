#![cfg(unix)]
#![allow(dead_code)]
#[path = "../cli/sikaru/compute_workspace.rs"]
mod workspace;
use std::{
    fs,
    os::unix::fs::{symlink, PermissionsExt},
};

#[test]
fn frozen_tree_retains_binary_modes_deletions_and_immutable_retry() {
    let root = tempfile::tempdir().unwrap();
    let work = root.path().join("work");
    let state = root.path().join("state");
    fs::create_dir(&work).unwrap();
    fs::create_dir(&state).unwrap();
    fs::write(work.join("output.bin"), [0, 255, 1]).unwrap();
    fs::set_permissions(work.join("output.bin"), fs::Permissions::from_mode(0o600)).unwrap();
    let first = workspace::freeze(&work, &state, "capture-one").unwrap();
    assert_eq!(first.tree["files"]["output.bin"]["mode"], 0o600);
    assert_eq!(first.chunks().unwrap()[0].1, vec![0, 255, 1]);
    fs::remove_file(work.join("output.bin")).unwrap();
    assert_eq!(
        workspace::freeze(&work, &state, "capture-one")
            .unwrap()
            .tree,
        first.tree
    );
    assert_eq!(
        workspace::freeze(&work, &state, "capture-two")
            .unwrap()
            .tree["files"],
        serde_json::json!({})
    );
}

#[test]
fn capture_skips_symlinks_without_reading_targets() {
    let root = tempfile::tempdir().unwrap();
    let work = root.path().join("work");
    let state = root.path().join("state");
    let outside = root.path().join("outside");
    fs::create_dir_all(work.join("src")).unwrap();
    fs::create_dir(&state).unwrap();
    fs::create_dir(&outside).unwrap();
    fs::write(root.path().join("secret"), b"private").unwrap();
    fs::write(outside.join("inner"), b"private").unwrap();
    fs::write(work.join("src/main.py"), b"print(42)\n").unwrap();
    symlink(root.path().join("secret"), work.join("file-link")).unwrap();
    symlink(&outside, work.join("dir-link")).unwrap();
    symlink("main.py", work.join("src/relative-link")).unwrap();
    let frozen = workspace::freeze(&work, &state, "capture").unwrap();
    let files = frozen.tree["files"].as_object().unwrap();
    assert_eq!(files.keys().collect::<Vec<_>>(), vec!["src/main.py"]);
    assert!(frozen
        .chunks()
        .unwrap()
        .iter()
        .all(|(_, bytes)| bytes.as_slice() != b"private"));
}

#[test]
fn capture_versions_hard_linked_files_as_independent_copies() {
    let root = tempfile::tempdir().unwrap();
    let work = root.path().join("work");
    let state = root.path().join("state");
    fs::create_dir(&work).unwrap();
    fs::create_dir(&state).unwrap();
    fs::write(root.path().join("cached.py"), b"cached = True\n").unwrap();
    fs::hard_link(root.path().join("cached.py"), work.join("installed.py")).unwrap();
    fs::write(work.join("a.txt"), b"shared\n").unwrap();
    fs::hard_link(work.join("a.txt"), work.join("b.txt")).unwrap();
    let frozen = workspace::freeze(&work, &state, "capture").unwrap();
    let files = &frozen.tree["files"];
    assert_eq!(files["installed.py"]["size"], 14);
    assert_eq!(files["a.txt"]["sha256"], files["b.txt"]["sha256"]);
    assert_eq!(files.as_object().unwrap().len(), 3);
}

#[test]
fn capture_rejects_private_state_inside_workspace() {
    let root = tempfile::tempdir().unwrap();
    let work = root.path().join("work");
    fs::create_dir(&work).unwrap();
    assert!(workspace::freeze(&work, &work, "capture").is_err());
}

#[test]
fn capture_bounds_empty_directory_depth() {
    let root = tempfile::tempdir().unwrap();
    let work = root.path().join("work");
    let state = root.path().join("state");
    fs::create_dir(&work).unwrap();
    fs::create_dir(&state).unwrap();
    let mut directory = work.clone();
    for _ in 0..129 {
        directory = directory.join("d");
        fs::create_dir(&directory).unwrap();
    }
    assert!(workspace::freeze(&work, &state, "capture").is_err());
}

#[test]
fn frozen_chunks_remain_bound_to_original_staging_directory() {
    let root = tempfile::tempdir().unwrap();
    let work = root.path().join("work");
    let state = root.path().join("state");
    fs::create_dir(&work).unwrap();
    fs::create_dir(&state).unwrap();
    fs::write(work.join("data"), b"original").unwrap();
    let frozen = workspace::freeze(&work, &state, "capture").unwrap();
    let staging = fs::read_dir(&state)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    fs::rename(&staging, state.join("retained")).unwrap();
    symlink(root.path(), &staging).unwrap();
    assert_eq!(frozen.chunks().unwrap()[0].1, b"original");
}

fn upload_scope() -> workspace::UploadScope<'static> {
    workspace::UploadScope {
        project_id: "project",
        attachment_id: "attachment",
        checkpoint_id: "capture",
        run_id: "run",
        workspace_generation: "generation",
        owner_epoch: 1,
        tree_id: "tree-one",
    }
}
const ABC_HASH: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";

#[test]
fn accepted_upload_survives_reopen_only_for_the_same_capture_authority() {
    let root = tempfile::tempdir().unwrap();
    let work = root.path().join("work");
    let state = root.path().join("state");
    fs::create_dir(&work).unwrap();
    fs::create_dir(&state).unwrap();
    fs::write(work.join("file"), b"abc").unwrap();
    let frozen = workspace::freeze(&work, &state, "capture").unwrap();
    assert!(!frozen.uploaded(&upload_scope(), ABC_HASH, 3).unwrap());
    frozen.acknowledge(&upload_scope(), ABC_HASH, 3).unwrap();
    drop(frozen);
    fs::write(work.join("file"), b"changed").unwrap();
    let reopened = workspace::freeze(&work, &state, "capture").unwrap();
    assert!(reopened.uploaded(&upload_scope(), ABC_HASH, 3).unwrap());
    assert_eq!(reopened.chunk(ABC_HASH, 3).unwrap(), b"abc");
    for field in [
        "project",
        "attachment",
        "checkpoint",
        "run",
        "generation",
        "epoch",
        "tree",
    ] {
        let mut scope = upload_scope();
        match field {
            "project" => scope.project_id = "other",
            "attachment" => scope.attachment_id = "other",
            "checkpoint" => scope.checkpoint_id = "other",
            "run" => scope.run_id = "other",
            "generation" => scope.workspace_generation = "other",
            "epoch" => scope.owner_epoch = 2,
            _ => scope.tree_id = "other",
        }
        assert!(!reopened.uploaded(&scope, ABC_HASH, 3).unwrap(), "{field}");
    }
    assert!(reopened.uploaded(&upload_scope(), ABC_HASH, 4).is_err());
}

#[test]
fn altered_upload_acknowledgments_fail_closed() {
    for fault in [
        "malformed",
        "digest",
        "size",
        "scope",
        "symlink",
        "hardlink",
        "public",
    ] {
        let root = tempfile::tempdir().unwrap();
        let work = root.path().join("work");
        let state = root.path().join("state");
        fs::create_dir(&work).unwrap();
        fs::create_dir(&state).unwrap();
        fs::write(work.join("file"), b"abc").unwrap();
        let frozen = workspace::freeze(&work, &state, "capture").unwrap();
        frozen.acknowledge(&upload_scope(), ABC_HASH, 3).unwrap();
        let staging = fs::read_dir(&state)
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        let receipt = fs::read_dir(staging)
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| p.file_name().unwrap().to_string_lossy().starts_with("ack-"))
            .unwrap();
        let mut value: serde_json::Value =
            serde_json::from_slice(&fs::read(&receipt).unwrap()).unwrap();
        match fault {
            "malformed" => fs::write(&receipt, b"{").unwrap(),
            "digest" | "size" | "scope" => {
                match fault {
                    "digest" => value["sha256"] = serde_json::json!("0".repeat(64)),
                    "size" => value["size"] = serde_json::json!(4),
                    _ => value["scope"]["owner_epoch"] = serde_json::json!(2),
                }
                fs::write(&receipt, serde_json::to_vec(&value).unwrap()).unwrap();
            }
            "symlink" => {
                let outside = root.path().join("receipt");
                fs::rename(&receipt, &outside).unwrap();
                symlink(outside, &receipt).unwrap();
            }
            "hardlink" => fs::hard_link(&receipt, root.path().join("receipt")).unwrap(),
            _ => fs::set_permissions(&receipt, fs::Permissions::from_mode(0o644)).unwrap(),
        }
        assert!(
            frozen.uploaded(&upload_scope(), ABC_HASH, 3).is_err(),
            "{fault}"
        );
        assert!(
            frozen.acknowledge(&upload_scope(), ABC_HASH, 3).is_err(),
            "{fault}"
        );
    }
}
