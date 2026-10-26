//! The private checkpoint repository against real temporary workspaces.
#![cfg(unix)]
#![allow(dead_code)]
#[path = "../cli/sikaru/compute_checkpoint_repo.rs"]
mod checkpoint_repo;
#[path = "support/git_remote.rs"]
mod git_remote;
use checkpoint_repo::{link_escapes, PrivateRepo, SkipReason, EXECUTABLE, LINK};
use git_remote::{git, ignore_defaults, porcelain, snapshot, task_repository};
use std::{collections::BTreeMap, path::PathBuf};

struct Fixture {
    _root: tempfile::TempDir,
    ws: PathBuf,
    state: PathBuf,
}
fn fixture() -> Fixture {
    let root = tempfile::tempdir().unwrap();
    let base = root.path().canonicalize().unwrap();
    std::fs::create_dir_all(base.join("workspace")).unwrap();
    std::fs::create_dir_all(base.join("state")).unwrap();
    Fixture {
        ws: base.join("workspace"),
        state: base.join("state"),
        _root: root,
    }
}
fn open(f: &Fixture, private: &[PathBuf]) -> PrivateRepo {
    let repo = PrivateRepo::open_or_init(&f.state.join("workspace.git"), &f.ws).unwrap();
    let mut dirs = vec![f.state.clone()];
    dirs.extend_from_slice(private);
    repo.write_exclude(&ignore_defaults(), &dirs, &[".env".into(), "*.pem".into()])
        .unwrap();
    repo
}
/// path -> (mode, content) of a tree in the private repository.
fn files(f: &Fixture, tree: git2::Oid) -> BTreeMap<String, (u32, Vec<u8>)> {
    let repo = git2::Repository::open_bare(f.state.join("workspace.git")).unwrap();
    let tree = repo.find_tree(tree).unwrap();
    let mut out = BTreeMap::new();
    tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
        if entry.kind() == Some(git2::ObjectType::Blob) {
            let blob = repo.find_blob(entry.id()).unwrap();
            out.insert(
                format!("{root}{}", entry.name().unwrap()),
                (entry.filemode() as u32, blob.content().to_vec()),
            );
        }
        git2::TreeWalkResult::Ok
    })
    .unwrap();
    out
}

#[test]
fn task_repository_git_dir_and_status_are_never_modified() {
    let f = fixture();
    task_repository(&f.ws);
    let before = (snapshot(&f.ws), porcelain(&f.ws));
    let repo = open(&f, &[]);
    let staged = repo.stage().unwrap();
    repo.snapshot().unwrap();
    assert_eq!(
        (snapshot(&f.ws), porcelain(&f.ws)),
        before,
        "the workspace and its repository are read-only"
    );
    let tree = files(&f, staged.tree);
    assert_eq!(
        tree["tracked.txt"].1, b"working copy",
        "checkpoints hold the working tree, not the task index"
    );
    assert_eq!(tree["staged.txt"].1, b"staged");
}

#[test]
fn ignore_rules_apply_and_tracked_ignored_paths_are_dropped() {
    let f = fixture();
    task_repository(&f.ws);
    for (path, body) in [
        ("src/__pycache__/m.pyc", "x"),
        ("target/debug/app", "x"),
        ("pkg.egg-info/PKG", "x"),
    ] {
        std::fs::create_dir_all(f.ws.join(path).parent().unwrap()).unwrap();
        std::fs::write(f.ws.join(path), body).unwrap();
    }
    let repo = open(&f, &[]);
    let staged = repo.stage().unwrap();
    let tree = files(&f, staged.tree);
    for ignored in [
        "app.log",
        ".venv/lib/site.py",
        "node_modules/left-pad/index.js",
        "src/__pycache__/m.pyc",
        "target/debug/app",
        "pkg.egg-info/PKG",
        ".env",
        "deploy.pem",
    ] {
        assert!(
            !tree.contains_key(ignored),
            "{ignored} must never be committed"
        );
    }
    assert!(tree.keys().all(|p| !p.starts_with(".git/")));
    assert!(tree.contains_key(".gitignore"));
    assert!(
        !tree.contains_key(".env.example"),
        "credential templates are never published"
    );
}

#[test]
fn links_and_modes_are_versioned_and_escaping_links_dropped() {
    let f = fixture();
    task_repository(&f.ws);
    std::fs::create_dir_all(f.ws.join("nested")).unwrap();
    std::os::unix::fs::symlink("../tracked.txt", f.ws.join("nested/ok")).unwrap();
    std::os::unix::fs::symlink("../../tracked.txt", f.ws.join("nested/deep")).unwrap();
    let staged = open(&f, &[]).stage().unwrap();
    let tree = files(&f, staged.tree);
    assert_eq!(tree["alias"], (LINK, b"tracked.txt".to_vec()));
    assert_eq!(tree["nested/ok"].0, LINK);
    assert_eq!(tree["run.sh"].0, EXECUTABLE);
    for escaped in ["escape", "absolute", "nested/deep"] {
        assert!(!tree.contains_key(escaped));
        assert!(
            staged
                .skipped
                .iter()
                .any(|s| s.path == escaped && s.reason == SkipReason::LinkOutsideWorkspace),
            "{escaped}"
        );
    }
    assert!(tree.keys().all(|path| !path.starts_with("vendor/lib")));
    assert!(staged
        .skipped
        .iter()
        .any(|s| s.path == "vendor/lib" && s.reason == SkipReason::NestedRepository));
}

#[test]
fn link_escapes_is_lexical() {
    for (path, target, escapes) in [
        ("a", "b", false),
        ("a", "./b", false),
        ("d/a", "../b", false),
        ("d/a", "../../b", true),
        ("a", "../b", true),
        ("a", "/etc/hosts", true),
        ("a", "", true),
        ("d/e/a", "../../x/../y", false),
        ("d/a", "x/../../..", true),
    ] {
        assert_eq!(
            link_escapes(path, target.as_bytes()),
            escapes,
            "{path} -> {target}"
        );
    }
}

#[test]
fn private_state_inside_the_workspace_is_excluded() {
    let f = fixture();
    std::fs::write(f.ws.join("keep.txt"), "keep").unwrap();
    let state = f.ws.join(".sikaru-abc/executor");
    std::fs::create_dir_all(&state).unwrap();
    std::fs::write(f.ws.join(".sikaru-abc/state.json"), "{}").unwrap();
    std::fs::create_dir_all(f.ws.join("custom state")).unwrap();
    std::fs::write(f.ws.join("custom state/journal"), "private").unwrap();
    let repo = PrivateRepo::open_or_init(&state.join("workspace.git"), &f.ws).unwrap();
    repo.write_exclude(
        &ignore_defaults(),
        &[state.clone(), f.ws.join("custom state")],
        &[],
    )
    .unwrap();
    let staged = repo.stage().unwrap();
    let repo2 = git2::Repository::open_bare(state.join("workspace.git")).unwrap();
    let tree = repo2.find_tree(staged.tree).unwrap();
    let names: Vec<String> = tree.iter().map(|e| e.name().unwrap().to_owned()).collect();
    assert_eq!(names, vec!["keep.txt".to_owned()]);
}

#[test]
fn uncommitted_nested_repositories_are_excluded() {
    let f = fixture();
    let nested = f.ws.join("nested");
    std::fs::create_dir(&nested).unwrap();
    git(&nested, &["init", "-q"]);
    std::fs::write(nested.join("private.txt"), "nested working copy").unwrap();
    let staged = open(&f, &[]).stage().unwrap();
    assert!(files(&f, staged.tree).is_empty());
    assert!(staged
        .skipped
        .iter()
        .any(|s| s.path == "nested" && s.reason == SkipReason::NestedRepository));
}

#[test]
fn workspace_ignore_negation_cannot_include_private_state() {
    let f = fixture();
    std::fs::write(
        f.ws.join(".gitignore"),
        "!.sikaru-state/\n!.sikaru-state/**\n!custom/\n!custom/**\n",
    )
    .unwrap();
    for dir in [".sikaru-state", "custom"] {
        std::fs::create_dir(f.ws.join(dir)).unwrap();
        std::fs::write(f.ws.join(dir).join("credential"), "private").unwrap();
    }
    let repo = open(&f, &[f.ws.join("custom")]);
    let staged = repo.stage().unwrap();
    let tree = files(&f, staged.tree);
    assert_eq!(tree.keys().cloned().collect::<Vec<_>>(), vec![".gitignore"]);
}

#[test]
fn cancelled_checkpoint_does_not_capture_or_advance_refs() {
    let f = fixture();
    std::fs::write(f.ws.join("task.txt"), "before").unwrap();
    let cancellation = checkpoint_repo::Cancellation::default();
    let repo = PrivateRepo::open_or_init_cancellable(
        &f.state.join("workspace.git"),
        &f.ws,
        cancellation.clone(),
    )
    .unwrap();
    let staged = repo.stage().unwrap();
    cancellation.cancel();
    std::fs::write(f.ws.join("task.txt"), "after").unwrap();
    assert!(repo.stage().is_err());
    assert!(repo.snapshot().is_err());
    let raw = git2::Repository::open_bare(f.state.join("workspace.git")).unwrap();
    assert!(raw.refname_to_id("refs/heads/session").is_err());
    assert_eq!(files(&f, staged.tree)["task.txt"].1, b"before");
}

#[test]
fn cancelled_preparation_does_not_initialize_a_repository() {
    let f = fixture();
    let cancellation = checkpoint_repo::Cancellation::default();
    cancellation.cancel();
    assert!(PrivateRepo::open_or_init_cancellable(
        &f.state.join("workspace.git"),
        &f.ws,
        cancellation,
    )
    .is_err());
    assert!(!f.state.join("workspace.git").exists());
}

#[test]
fn manifest_captures_modes_links_deletions_and_hard_credential_exclusion() {
    use sha2::{Digest, Sha256};
    let f = fixture();
    std::fs::write(f.ws.join("keep.txt"), "hello").unwrap();
    std::fs::write(f.ws.join(".env"), "secret").unwrap();
    std::fs::write(f.ws.join(".gitignore"), "!.env\n!deploy.pem\n").unwrap();
    std::fs::write(f.ws.join("deploy.pem"), "private").unwrap();
    std::os::unix::fs::symlink("keep.txt", f.ws.join("alias")).unwrap();
    let repo = open(&f, &[]);
    let snapshot = repo.snapshot().unwrap();
    let manifest: serde_json::Value = serde_json::from_slice(&snapshot.manifest).unwrap();
    assert_eq!(
        manifest["files"]["keep.txt"]["sha256"],
        format!("{:x}", Sha256::digest(b"hello"))
    );
    assert_eq!(manifest["files"]["keep.txt"]["mode"], 420);
    assert_eq!(manifest["files"]["alias"]["link"], "keep.txt");
    assert!(manifest["files"].get(".env").is_none());
    assert!(manifest["files"].get("deploy.pem").is_none());
    std::fs::remove_file(f.ws.join("keep.txt")).unwrap();
    let next: serde_json::Value =
        serde_json::from_slice(&repo.snapshot().unwrap().manifest).unwrap();
    assert!(next["files"].get("keep.txt").is_none());
}
#[test]
fn empty_workspace_has_a_manifest_and_large_blobs_are_reported() {
    let f = fixture();
    let repo = open(&f, &[]);
    assert_eq!(repo.snapshot().unwrap().manifest, b"{\"files\":{}}");
    std::fs::write(f.ws.join("large.bin"), vec![0u8; 32_000_001]).unwrap();
    let snapshot = repo.snapshot().unwrap();
    assert!(snapshot.blobs.is_empty());
    assert_eq!(snapshot.skipped[0].reason, SkipReason::TooLarge);
}
