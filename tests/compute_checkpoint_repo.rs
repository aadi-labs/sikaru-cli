//! The private checkpoint repository against real temporary workspaces.
#![cfg(unix)]
#![allow(dead_code)]
#[path = "../cli/sikaru/compute_checkpoint_repo.rs"]
mod checkpoint_repo;
#[path = "support/git_remote.rs"]
mod git_remote;
use checkpoint_repo::{link_escapes, PrivateRepo, SkipReason, EXECUTABLE, LINK};
use git_remote::{git, ignore_defaults, noise, porcelain, snapshot, task_repository, text};
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
    repo.write_exclude(&ignore_defaults(), &dirs).unwrap();
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
    let chain = repo
        .chain(None, &[], staged.tree, 50_000_000, "checkpoint")
        .unwrap();
    repo.pack(chain.commits[0], &[]).unwrap();
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
    ] {
        assert!(
            !tree.contains_key(ignored),
            "{ignored} must never be committed"
        );
    }
    assert!(tree.keys().all(|p| !p.starts_with(".git/")));
    assert!(tree.contains_key(".gitignore"));
    let seed = text(git(&f.ws, &["rev-parse", "HEAD"]));
    assert_eq!(repo.seed().unwrap().to_string(), seed);
    let chain = repo
        .chain(None, &[], staged.tree, 50_000_000, "checkpoint")
        .unwrap();
    assert_eq!(chain.parent.unwrap().to_string(), seed);
    assert!(chain.history);
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
    )
    .unwrap();
    let staged = repo.stage().unwrap();
    let repo2 = git2::Repository::open_bare(state.join("workspace.git")).unwrap();
    let tree = repo2.find_tree(staged.tree).unwrap();
    let names: Vec<String> = tree.iter().map(|e| e.name().unwrap().to_owned()).collect();
    assert_eq!(names, vec!["keep.txt".to_owned()]);
}

#[test]
fn large_change_sets_split_into_chained_commits_within_budget() {
    let f = fixture();
    for i in 0..6u8 {
        std::fs::write(f.ws.join(format!("part-{i}.bin")), noise(60_000, i)).unwrap();
    }
    std::fs::write(f.ws.join("huge.bin"), noise(300_000, 99)).unwrap();
    let repo = open(&f, &[]);
    let staged = repo.stage().unwrap();
    let chain = repo
        .chain(None, &[], staged.tree, 150_000, "checkpoint")
        .unwrap();
    assert!(chain.commits.len() >= 3, "{}", chain.commits.len());
    assert_eq!(chain.parent, None);
    assert!(chain
        .skipped
        .iter()
        .any(|s| s.path == "huge.bin" && s.reason == SkipReason::TooLarge));
    let mut previous: Option<git2::Oid> = None;
    for commit in &chain.commits {
        let pack = repo.pack(*commit, previous.as_slice()).unwrap();
        assert!(pack.bytes.len() <= 150_000, "{}", pack.bytes.len());
        previous = Some(*commit);
    }
    let last = git2::Repository::open_bare(f.state.join("workspace.git")).unwrap();
    let head = last.find_commit(*chain.commits.last().unwrap()).unwrap();
    let final_tree = files(&f, head.tree_id());
    assert_eq!(
        final_tree.keys().filter(|p| p.starts_with("part-")).count(),
        6
    );
    assert!(!final_tree.contains_key("huge.bin"));
    for pair in chain.commits.windows(2) {
        let child = last.find_commit(pair[1]).unwrap();
        assert_eq!(child.parent_ids().collect::<Vec<_>>(), vec![pair[0]]);
    }
}

#[test]
fn seeded_history_is_packed_once_and_later_packs_carry_only_changes() {
    let f = fixture();
    task_repository(&f.ws);
    std::fs::write(f.ws.join("second.txt"), "two").unwrap();
    git(&f.ws, &["add", "second.txt"]);
    git(&f.ws, &["commit", "-qm", "second"]);
    let repo = open(&f, &[]);
    let staged = repo.stage().unwrap();
    let first = repo
        .chain(None, &[], staged.tree, 50_000_000, "checkpoint")
        .unwrap();
    assert!(first.history);
    let history = repo.pack(first.commits[0], &[]).unwrap();
    assert!(
        history.objects >= 8,
        "seed history travels in the first pack: {}",
        history.objects
    );
    repo.set_pushed("refs/heads/sessions/test", first.commits[0])
        .unwrap();
    std::fs::write(f.ws.join("tracked.txt"), "changed again").unwrap();
    let staged = repo.stage().unwrap();
    let next = repo
        .chain(
            Some(first.commits[0]),
            &[],
            staged.tree,
            50_000_000,
            "checkpoint",
        )
        .unwrap();
    assert!(!next.history);
    let delta = repo.pack(next.commits[0], &[first.commits[0]]).unwrap();
    assert!(
        delta.objects <= 3,
        "only the commit, its tree and one blob: {}",
        delta.objects
    );
    let unchanged = repo
        .chain(
            Some(next.commits[0]),
            &[],
            repo.stage().unwrap().tree,
            50_000_000,
            "checkpoint",
        )
        .unwrap();
    assert!(unchanged.commits.is_empty());
}

#[test]
fn only_a_whole_repository_workspace_with_full_history_is_seeded() {
    let f = fixture();
    task_repository(&f.ws);
    let sub = f.ws.join("sub");
    std::fs::create_dir_all(&sub).unwrap();
    std::fs::write(sub.join("x"), "x").unwrap();
    let state = f.state.join("sub");
    assert!(
        PrivateRepo::open_or_init(&state.join("workspace.git"), &sub)
            .unwrap()
            .seed()
            .is_none()
    );
    let unborn = f.state.join("unborn-ws");
    std::fs::create_dir_all(&unborn).unwrap();
    git(&unborn, &["init", "-q"]);
    assert!(
        PrivateRepo::open_or_init(&f.state.join("unborn/workspace.git"), &unborn)
            .unwrap()
            .seed()
            .is_none()
    );
    let shallow = f.state.join("shallow-ws");
    let url = format!("file://{}", f.ws.display());
    git(
        &f.state,
        &[
            "clone",
            "-q",
            "--depth",
            "1",
            &url,
            shallow.to_str().unwrap(),
        ],
    );
    assert!(
        PrivateRepo::open_or_init(&f.state.join("shallow/workspace.git"), &shallow)
            .unwrap()
            .seed()
            .is_none()
    );
}

#[test]
fn a_second_session_seeded_from_the_same_head_omits_the_advertised_history() {
    let f = fixture();
    task_repository(&f.ws);
    for i in 0..5 {
        std::fs::write(
            f.ws.join(format!("history-{i}.txt")),
            format!("revision {i}"),
        )
        .unwrap();
        git(&f.ws, &["add", "."]);
        git(&f.ws, &["commit", "-qm", &format!("history {i}")]);
    }
    let seed = git2::Oid::from_str(&text(git(&f.ws, &["rev-parse", "HEAD"]))).unwrap();
    let first_session = open(&f, &[]);
    let staged = first_session.stage().unwrap();
    let first = first_session
        .chain(None, &[], staged.tree, 50_000_000, "checkpoint")
        .unwrap();
    let full = first_session.pack(first.commits[0], &[]).unwrap();
    // A second session: its own state dir, the same task HEAD, and the scope advertising the seed.
    let state = f.state.join("second");
    let second_session = PrivateRepo::open_or_init(&state.join("workspace.git"), &f.ws).unwrap();
    second_session
        .write_exclude(&ignore_defaults(), std::slice::from_ref(&state))
        .unwrap();
    assert_eq!(second_session.seed(), Some(seed));
    let staged = second_session.stage().unwrap();
    let chain = second_session
        .chain(None, &[seed], staged.tree, 50_000_000, "checkpoint")
        .unwrap();
    assert!(chain.history);
    assert_eq!(chain.parent, Some(seed));
    let thin = second_session.pack(chain.commits[0], &[seed]).unwrap();
    assert!(
        thin.objects <= 8,
        "only the checkpoint commit, its trees and changed blobs: {}",
        thin.objects
    );
    assert!(
        thin.objects * 3 < full.objects,
        "history excluded: {} vs {}",
        thin.objects,
        full.objects
    );
    assert!(thin.bytes.len() < full.bytes.len());
    // An advertised seed also lifts the size gate: a small budget still seeds.
    let tight = second_session
        .chain(None, &[seed], staged.tree, 1_000, "checkpoint")
        .unwrap();
    assert_eq!(tight.parent, Some(seed));
}

#[test]
fn a_head_missing_locally_gets_a_full_tree_commit_on_top_of_it() {
    let f = fixture();
    std::fs::write(f.ws.join("a.txt"), "a").unwrap();
    let repo = open(&f, &[]);
    let staged = repo.stage().unwrap();
    let foreign = git2::Oid::from_str("1111111111111111111111111111111111111111").unwrap();
    let chain = repo
        .chain(Some(foreign), &[], staged.tree, 50_000_000, "checkpoint")
        .unwrap();
    assert_eq!(chain.parent, Some(foreign));
    let pack = repo.pack(chain.commits[0], &[foreign]).unwrap();
    assert!(pack.objects >= 3, "commit, tree and blob: {}", pack.objects);
}

#[test]
fn seed_is_captured_once_and_deletions_follow_the_working_tree() {
    let f = fixture();
    task_repository(&f.ws);
    let repo = open(&f, &[]);
    let seed = repo.seed();
    std::fs::write(f.ws.join("later.txt"), "later").unwrap();
    git(&f.ws, &["add", "later.txt"]);
    git(&f.ws, &["commit", "-qm", "later"]);
    assert_eq!(open(&f, &[]).seed(), seed);
    let first = repo
        .chain(
            None,
            &[],
            repo.stage().unwrap().tree,
            50_000_000,
            "checkpoint",
        )
        .unwrap();
    std::fs::remove_file(f.ws.join("tracked.txt")).unwrap();
    let next = repo
        .chain(
            first.commits.last().copied(),
            &[],
            repo.stage().unwrap().tree,
            50_000_000,
            "checkpoint",
        )
        .unwrap();
    let private = git2::Repository::open_bare(f.state.join("workspace.git")).unwrap();
    assert!(
        !files(&f, private.find_commit(next.commits[0]).unwrap().tree_id())
            .contains_key("tracked.txt")
    );
}

#[test]
fn empty_initial_workspace_still_has_a_checkpoint_commit() {
    let f = fixture();
    let repo = open(&f, &[]);
    let chain = repo
        .chain(
            None,
            &[],
            repo.stage().unwrap().tree,
            50_000_000,
            "checkpoint",
        )
        .unwrap();
    assert_eq!(chain.commits.len(), 1);
}

#[test]
fn a_seed_with_an_unchanged_tree_still_creates_an_unborn_branch_child() {
    let f = fixture();
    git(&f.ws, &["init", "-q"]);
    std::fs::write(f.ws.join("file"), "content").unwrap();
    git(&f.ws, &["add", "file"]);
    git(&f.ws, &["commit", "-qm", "base"]);
    let repo = open(&f, &[]);
    let chain = repo
        .chain(
            None,
            &[],
            repo.stage().unwrap().tree,
            50_000_000,
            "checkpoint",
        )
        .unwrap();
    assert_eq!(chain.commits.len(), 1);
    assert_eq!(chain.parent, repo.seed());
}

#[test]
fn pack_overhead_does_not_push_a_file_past_the_budget() {
    let f = fixture();
    std::fs::write(f.ws.join("almost-full.bin"), noise(4_000, 9)).unwrap();
    let repo = open(&f, &[]);
    let chain = repo
        .chain(None, &[], repo.stage().unwrap().tree, 4_010, "checkpoint")
        .unwrap();
    assert!(chain
        .skipped
        .iter()
        .any(|s| s.path == "almost-full.bin" && s.reason == SkipReason::TooLarge));
    assert!(repo.pack(chain.commits[0], &[]).unwrap().bytes.len() <= 4_010);
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
fn unknown_advertised_objects_do_not_prevent_local_history_packing() {
    let f = fixture();
    std::fs::write(f.ws.join("file"), "content").unwrap();
    let repo = open(&f, &[]);
    let chain = repo
        .chain(
            None,
            &[],
            repo.stage().unwrap().tree,
            50_000_000,
            "checkpoint",
        )
        .unwrap();
    let unknown = git2::Oid::from_str("2222222222222222222222222222222222222222").unwrap();
    assert_eq!(repo.pack(chain.commits[0], &[unknown]).unwrap().objects, 3);
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
fn split_after_a_foreign_head_retains_every_admitted_file() {
    let f = fixture();
    for i in 0..6u8 {
        std::fs::write(f.ws.join(format!("part-{i}.bin")), noise(60_000, i)).unwrap();
    }
    let repo = open(&f, &[]);
    let foreign = git2::Oid::from_str("1111111111111111111111111111111111111111").unwrap();
    let chain = repo
        .chain(
            Some(foreign),
            &[],
            repo.stage().unwrap().tree,
            150_000,
            "checkpoint",
        )
        .unwrap();
    assert!(chain.skipped.is_empty());
    let private = git2::Repository::open_bare(f.state.join("workspace.git")).unwrap();
    let final_tree = private
        .find_commit(*chain.commits.last().unwrap())
        .unwrap()
        .tree_id();
    assert_eq!(files(&f, final_tree).len(), 6);
    let mut previous = foreign;
    for commit in chain.commits {
        assert!(repo.pack(commit, &[previous]).unwrap().bytes.len() <= 150_000);
        previous = commit;
    }
}

#[test]
fn cancelled_checkpoint_does_not_stage_pack_or_advance_refs() {
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
    let chain = repo
        .chain(None, &[], staged.tree, 1_000_000, "before")
        .unwrap();
    let commit = chain.commits[0];
    cancellation.cancel();
    std::fs::write(f.ws.join("task.txt"), "after").unwrap();
    assert!(repo.stage().is_err());
    assert!(repo
        .chain(None, &[], staged.tree, 1_000_000, "after")
        .is_err());
    assert!(repo.pack(commit, &[]).is_err());
    assert!(repo.set_pushed("refs/heads/session", commit).is_err());
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
