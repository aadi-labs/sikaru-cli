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
