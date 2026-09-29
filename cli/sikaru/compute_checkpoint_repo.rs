//! A private git repository beside the workspace: its own git dir, index and ignore rules.
//! It never writes into the workspace or the workspace's own repository.
use anyhow::{Context, Result};
use git2::{ConfigLevel, IndexAddOption, ObjectType, Oid, Repository, RepositoryInitOptions, Tree};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

pub const FILE: u32 = 0o100644;
pub const EXECUTABLE: u32 = 0o100755;
pub const LINK: u32 = 0o120000;
pub const GITLINK: u32 = 0o160000;
const SEED_REF: &str = "refs/sikaru/seed";
const SEED_BYTES: &str = "sikaru.seedBytes";
const AUTHOR: &str = "Sikaru <checkpoints@sikaru.invalid>";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkipReason {
    LinkOutsideWorkspace,
    NestedRepository,
    TooLarge,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Skipped {
    pub path: String,
    pub reason: SkipReason,
}
pub struct Staged {
    pub tree: Oid,
    pub skipped: Vec<Skipped>,
}
pub struct Chain {
    pub commits: Vec<Oid>,
    pub parent: Option<Oid>,
    pub history: bool,
    pub skipped: Vec<Skipped>,
}
pub struct Pack {
    pub bytes: Vec<u8>,
    pub objects: usize,
}
/// Shared with the async caller: dropping its wait stops local checkpoint work.
#[derive(Clone, Default)]
pub struct Cancellation(Arc<AtomicBool>);
impl Cancellation {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
    }
    pub fn cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
    fn check(&self) -> Result<()> {
        anyhow::ensure!(!self.cancelled(), "checkpoint cancelled");
        Ok(())
    }
}
pub struct PrivateRepo {
    repo: Repository,
    cancellation: Cancellation,
}

impl PrivateRepo {
    pub fn open_or_init(git_dir: &Path, workspace: &Path) -> Result<Self> {
        Self::open_or_init_cancellable(git_dir, workspace, Cancellation::default())
    }
    pub fn open_or_init_cancellable(
        git_dir: &Path,
        workspace: &Path,
        cancellation: Cancellation,
    ) -> Result<Self> {
        cancellation.check()?;
        if !git_dir.join("HEAD").is_file() {
            init(git_dir, workspace, &cancellation)?;
        }
        cancellation.check()?;
        let mut repo = Self::open(git_dir, workspace)?;
        repo.cancellation = cancellation;
        Ok(repo)
    }
    pub fn open(git_dir: &Path, workspace: &Path) -> Result<Self> {
        let repo = Repository::open_bare(git_dir)?;
        // `false`: never write a gitlink file into the workspace.
        repo.set_workdir(workspace, false)?;
        Ok(Self {
            repo,
            cancellation: Cancellation::default(),
        })
    }
    pub fn write_exclude(
        &self,
        ignore_defaults: &[String],
        private_dirs: &[PathBuf],
    ) -> Result<()> {
        self.cancellation.check()?;
        let workspace = self
            .repo
            .workdir()
            .context("private repository has no workspace")?
            .canonicalize()?;
        let mut rules = String::from(".git\n/.sikaru-*/\n");
        for pattern in ignore_defaults
            .iter()
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
        {
            rules.push_str(pattern);
            rules.push('\n');
        }
        for dir in private_dirs {
            let dir = dir.canonicalize().unwrap_or_else(|_| dir.clone());
            if let Ok(relative) = dir.strip_prefix(&workspace) {
                if !relative.as_os_str().is_empty() {
                    rules.push_str(&anchored(relative));
                }
            }
        }
        let info = self.repo.path().join("info");
        std::fs::create_dir_all(&info)?;
        std::fs::write(info.join("exclude"), rules)?;
        Ok(())
    }
    pub fn stage(&self) -> Result<Staged> {
        self.cancellation.check()?;
        let (mut index, mut skipped) = self.index_workspace()?;
        self.filter_index(&mut index, &mut skipped)?;
        self.cancellation.check()?;
        index.write()?;
        Ok(Staged {
            tree: index.write_tree()?,
            skipped,
        })
    }
    fn index_workspace(&self) -> Result<(git2::Index, Vec<Skipped>)> {
        let mut index = self.repo.index()?;
        index.clear()?;
        self.cancellation.check()?;
        let workspace = self
            .repo
            .workdir()
            .context("private repository has no workspace")?;
        let mut nested = std::collections::BTreeSet::new();
        index.add_all(
            ["*"],
            IndexAddOption::DEFAULT,
            Some(&mut |path, _| {
                if self.cancellation.cancelled() {
                    return -1;
                }
                if let Some(path) = nested_repository(workspace, path) {
                    nested.insert(path);
                    return 1;
                }
                0
            }),
        )?;
        let skipped = nested
            .into_iter()
            .map(|path| Skipped {
                path,
                reason: SkipReason::NestedRepository,
            })
            .collect();
        Ok((index, skipped))
    }
    fn filter_index(&self, index: &mut git2::Index, skipped: &mut Vec<Skipped>) -> Result<()> {
        let entries: Vec<_> = index
            .iter()
            .map(|e| (String::from_utf8_lossy(&e.path).into_owned(), e.mode, e.id))
            .collect();
        for (path, mode, id) in entries {
            self.cancellation.check()?;
            let (drop, reason) = self.admission(&path, mode, id)?;
            if drop {
                index.remove_path(Path::new(&path))?;
            }
            if let Some(reason) = reason {
                skipped.push(Skipped { path, reason });
            }
        }
        Ok(())
    }
    /// Whether an index entry leaves the checkpoint, and the reason reported for it.
    /// Tracked paths that the ignore rules now match leave silently.
    fn admission(&self, path: &str, mode: u32, id: Oid) -> Result<(bool, Option<SkipReason>)> {
        if mode == GITLINK {
            return Ok((true, Some(SkipReason::NestedRepository)));
        }
        if mode == LINK && link_escapes(path, self.repo.find_blob(id)?.content()) {
            return Ok((true, Some(SkipReason::LinkOutsideWorkspace)));
        }
        Ok((self.repo.status_should_ignore(Path::new(path))?, None))
    }
    pub fn seed(&self) -> Option<Oid> {
        self.repo.refname_to_id(SEED_REF).ok()
    }
    fn seed_bytes(&self) -> u64 {
        self.repo
            .config()
            .and_then(|c| c.get_i64(SEED_BYTES))
            .map_or(u64::MAX, |n| n.max(0) as u64)
    }
    pub fn has_commit(&self, oid: Oid) -> bool {
        self.repo.find_commit(oid).is_ok()
    }
    pub fn chain(
        &self,
        head: Option<Oid>,
        remote_has: &[Oid],
        target: Oid,
        budget: u64,
        message: &str,
    ) -> Result<Chain> {
        self.cancellation.check()?;
        anyhow::ensure!(budget >= 512, "checkpoint pack budget is too small");
        let seeded = self.select_seed(head, remote_has, budget)?;
        let parent = head.or(seeded);
        let base = parent
            .and_then(|p| self.repo.find_commit(p).ok())
            .map(|c| c.tree())
            .transpose()?;
        let mut chain = Chain {
            commits: Vec::new(),
            parent,
            history: seeded.is_some(),
            skipped: Vec::new(),
        };
        let (mut index, changes) =
            self.changes(base.as_ref(), target, budget, &mut chain.skipped)?;
        let mut hide = remote_has.to_vec();
        if let Some(head) = head {
            hide.push(head);
        }
        let context = Packing {
            budget,
            message,
            hide,
        };
        for batch in batches(changes, budget / 2) {
            self.append_batch(&mut index, &batch, &context, &mut chain)?;
        }
        self.append_final_tree(&mut index, &context, &mut chain)?;
        Ok(chain)
    }
    fn select_seed(
        &self,
        head: Option<Oid>,
        remote_has: &[Oid],
        budget: u64,
    ) -> Result<Option<Oid>> {
        if head.is_some() {
            return Ok(None);
        }
        let Some(seed) = self.seed() else {
            return Ok(None);
        };
        if remote_has.contains(&seed) {
            return Ok(Some(seed));
        }
        if self.seed_bytes() > budget / 2 {
            return Ok(None);
        }
        Ok((self.pack(seed, &[])?.bytes.len() as u64 <= budget / 2).then_some(seed))
    }
    fn changes(
        &self,
        base: Option<&Tree>,
        target: Oid,
        budget: u64,
        skipped: &mut Vec<Skipped>,
    ) -> Result<(git2::Index, Vec<Change>)> {
        let before = entries(base, &self.cancellation)?;
        let after = entries(Some(&self.repo.find_tree(target)?), &self.cancellation)?;
        let mut index = git2::Index::new()?;
        let mut changes = Vec::new();
        for (path, (id, mode)) in after {
            let Some(change) = self.sized_change(path, id, mode, budget, skipped)? else {
                continue;
            };
            if before.get(&change.path) == Some(&(id, mode)) {
                index.add(&change.entry())?;
            } else {
                changes.push(change);
            }
        }
        Ok((index, changes))
    }
    fn sized_change(
        &self,
        path: String,
        id: Oid,
        mode: u32,
        budget: u64,
        skipped: &mut Vec<Skipped>,
    ) -> Result<Option<Change>> {
        self.cancellation.check()?;
        let size = self.repo.odb()?.read_header(id)?.0 as u64;
        if size > budget {
            skipped.push(Skipped {
                path,
                reason: SkipReason::TooLarge,
            });
            return Ok(None);
        }
        Ok(Some(Change {
            path,
            id,
            mode,
            size,
        }))
    }
    fn append_batch(
        &self,
        index: &mut git2::Index,
        batch: &[Change],
        context: &Packing,
        chain: &mut Chain,
    ) -> Result<()> {
        self.cancellation.check()?;
        let before = index.write_tree_to(&self.repo)?;
        let tree = self.stage_batch(index, batch)?;
        if self.try_append(tree, context, chain)? {
            return Ok(());
        }
        index.read_tree(&self.repo.find_tree(before)?)?;
        self.split_batch(index, batch, context, chain)
    }
    fn stage_batch(&self, index: &mut git2::Index, batch: &[Change]) -> Result<Oid> {
        for change in batch {
            self.cancellation.check()?;
            index.add(&change.entry())?;
        }
        Ok(index.write_tree_to(&self.repo)?)
    }
    fn split_batch(
        &self,
        index: &mut git2::Index,
        batch: &[Change],
        context: &Packing,
        chain: &mut Chain,
    ) -> Result<()> {
        if batch.len() == 1 {
            chain.skipped.push(Skipped {
                path: batch[0].path.clone(),
                reason: SkipReason::TooLarge,
            });
            return Ok(());
        }
        let middle = batch.len() / 2;
        self.append_batch(index, &batch[..middle], context, chain)?;
        self.append_batch(index, &batch[middle..], context, chain)
    }
    fn append_final_tree(
        &self,
        index: &mut git2::Index,
        context: &Packing,
        chain: &mut Chain,
    ) -> Result<()> {
        let tree = index.write_tree_to(&self.repo)?;
        let parent = chain.commits.last().copied().or(chain.parent);
        let unchanged = parent
            .and_then(|p| self.repo.find_commit(p).ok())
            .is_some_and(|p| p.tree_id() == tree);
        // An unborn branch always needs a checkpoint child, even if its seed tree is unchanged.
        if unchanged && (!chain.commits.is_empty() || !chain.history) {
            return Ok(());
        }
        anyhow::ensure!(
            self.try_append(tree, context, chain)?,
            "checkpoint tree exceeds pack budget"
        );
        Ok(())
    }
    fn try_append(&self, tree: Oid, context: &Packing, chain: &mut Chain) -> Result<bool> {
        let parent = chain.commits.last().copied().or(chain.parent);
        let commit = self.commit(tree, parent, context.message)?;
        let mut hide = context.hide.clone();
        if let Some(previous) = chain.commits.last() {
            hide.push(*previous);
        }
        if self.pack(commit, &hide)?.bytes.len() as u64 > context.budget {
            return Ok(false);
        }
        chain.commits.push(commit);
        Ok(true)
    }
    /// A commit object written directly, so a parent need not exist locally.
    fn commit(&self, tree: Oid, parent: Option<Oid>, message: &str) -> Result<Oid> {
        self.cancellation.check()?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        let mut text = format!("tree {tree}\n");
        if let Some(parent) = parent {
            text.push_str(&format!("parent {parent}\n"));
        }
        text.push_str(&format!(
            "author {AUTHOR} {now} +0000\ncommitter {AUTHOR} {now} +0000\n\n{message}\n"
        ));
        Ok(self
            .repo
            .odb()?
            .write(ObjectType::Commit, text.as_bytes())?)
    }
    fn parents_available(&self, commit: Oid) -> Result<bool> {
        let mut pending = vec![commit];
        let mut seen = std::collections::HashSet::new();
        while let Some(oid) = pending.pop() {
            self.cancellation.check()?;
            if !seen.insert(oid) {
                continue;
            }
            let Ok(commit) = self.repo.find_commit(oid) else {
                return Ok(false);
            };
            pending.extend(commit.parent_ids());
        }
        Ok(true)
    }
    pub fn pack(&self, commit: Oid, hide: &[Oid]) -> Result<Pack> {
        self.cancellation.check()?;
        let mut builder = self.repo.packbuilder()?;
        builder.set_progress_callback(|_, _, _| !self.cancellation.cancelled())?;
        if self.parents_available(commit)? {
            self.insert_history(&mut builder, commit, hide)?;
        } else {
            self.insert_reachable(&mut builder, commit, hide)?;
        }
        self.collect_pack(&mut builder)
    }
    fn insert_history(
        &self,
        builder: &mut git2::PackBuilder,
        commit: Oid,
        hide: &[Oid],
    ) -> Result<()> {
        let mut walk = self.repo.revwalk()?;
        walk.push(commit)?;
        for oid in hide.iter().filter(|oid| self.has_commit(**oid)) {
            self.cancellation.check()?;
            walk.hide(*oid)?;
        }
        builder.insert_walk(&mut walk)?;
        Ok(())
    }
    fn insert_reachable(
        &self,
        builder: &mut git2::PackBuilder,
        commit: Oid,
        hide: &[Oid],
    ) -> Result<()> {
        let hidden = self.reachable(hide)?;
        for oid in self.reachable(&[commit])?.difference(&hidden) {
            self.cancellation.check()?;
            builder.insert_object(*oid, None)?;
        }
        Ok(())
    }
    fn collect_pack(&self, builder: &mut git2::PackBuilder) -> Result<Pack> {
        let mut bytes = Vec::new();
        builder.foreach(|chunk| {
            if self.cancellation.cancelled() {
                return false;
            }
            bytes.extend_from_slice(chunk);
            true
        })?;
        self.cancellation.check()?;
        let objects = builder.object_count();
        Ok(Pack { bytes, objects })
    }
    fn reachable(&self, roots: &[Oid]) -> Result<std::collections::BTreeSet<Oid>> {
        let mut pending = roots.to_vec();
        let mut objects = std::collections::BTreeSet::new();
        while let Some(oid) = pending.pop() {
            self.cancellation.check()?;
            if objects.contains(&oid) {
                continue;
            }
            let object = match self.repo.find_object(oid, None) {
                Ok(object) => object,
                Err(error) if error.code() == git2::ErrorCode::NotFound => continue,
                Err(error) => return Err(error.into()),
            };
            objects.insert(oid);
            if let Some(commit) = object.as_commit() {
                pending.push(commit.tree_id());
                pending.extend(commit.parent_ids());
            }
            if let Some(tree) = object.as_tree() {
                pending.extend(
                    tree.iter()
                        .filter(|e| e.filemode() != GITLINK as i32)
                        .map(|e| e.id()),
                );
            }
        }
        Ok(objects)
    }
    pub fn set_pushed(&self, refname: &str, commit: Oid) -> Result<()> {
        self.cancellation.check()?;
        self.repo
            .reference(refname, commit, true, "checkpoint pushed")?;
        Ok(())
    }
}

fn init(git_dir: &Path, workspace: &Path, cancellation: &Cancellation) -> Result<()> {
    cancellation.check()?;
    let staging = git_dir.with_extension("init");
    let _ = std::fs::remove_dir_all(&staging);
    let mut options = RepositoryInitOptions::new();
    options.bare(true).mkpath(true);
    let repo = Repository::init_opts(&staging, &options)?;
    let mut config = repo.config()?.open_level(ConfigLevel::Local)?;
    configure_private_repository(&mut config)?;
    if let Some(seed) = task_seed(workspace, cancellation) {
        install_seed(&staging, &mut config, seed)?;
    }
    cancellation.check()?;
    std::fs::rename(&staging, git_dir)?;
    Ok(())
}
fn configure_private_repository(config: &mut git2::Config) -> Result<()> {
    config.set_bool("core.filemode", true)?;
    config.set_bool("core.symlinks", true)?;
    config.set_bool("core.ignorecase", false)?;
    config.set_str("core.autocrlf", "false")?;
    config.set_str("core.excludesfile", "/dev/null")?;
    config.set_str("core.attributesfile", "/dev/null")?;
    Ok(())
}
fn install_seed(staging: &Path, config: &mut git2::Config, seed: Seed) -> Result<()> {
    std::fs::write(
        staging.join("objects/info/alternates"),
        format!("{}\n", seed.objects.display()),
    )?;
    config.set_i64(SEED_BYTES, seed.bytes as i64)?;
    Repository::open_bare(staging)?.reference(SEED_REF, seed.head, true, "task HEAD at start")?;
    Ok(())
}
struct Seed {
    head: Oid,
    objects: PathBuf,
    bytes: u64,
}
/// The workspace's own HEAD, when the workspace is exactly a repository's full-history work tree.
fn task_seed(workspace: &Path, cancellation: &Cancellation) -> Option<Seed> {
    let task = Repository::open_ext(
        workspace,
        git2::RepositoryOpenFlags::NO_SEARCH,
        std::iter::empty::<&std::ffi::OsStr>(),
    )
    .ok()?;
    let root = task.workdir()?.canonicalize().ok()?;
    if root != workspace.canonicalize().ok()? || task.is_shallow() {
        return None;
    }
    let head = task.head().ok()?.peel_to_commit().ok()?.id();
    let objects = task.commondir().join("objects");
    Some(Seed {
        head,
        bytes: dir_bytes(&objects, cancellation).ok()?,
        objects,
    })
}
fn dir_bytes(path: &Path, cancellation: &Cancellation) -> Result<u64> {
    cancellation.check()?;
    let Ok(entries) = std::fs::read_dir(path) else {
        return Ok(0);
    };
    let mut total = 0;
    for entry in entries.flatten() {
        cancellation.check()?;
        total += entry_bytes(&entry, cancellation)?;
    }
    Ok(total)
}
fn entry_bytes(entry: &std::fs::DirEntry, cancellation: &Cancellation) -> Result<u64> {
    match entry.file_type() {
        Ok(kind) if kind.is_dir() => dir_bytes(&entry.path(), cancellation),
        Ok(kind) if kind.is_file() => Ok(entry.metadata().map_or(0, |m| m.len())),
        _ => Ok(0),
    }
}
fn entries(
    tree: Option<&Tree>,
    cancellation: &Cancellation,
) -> Result<BTreeMap<String, (Oid, u32)>> {
    cancellation.check()?;
    let mut out = BTreeMap::new();
    if let Some(tree) = tree {
        tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            if cancellation.cancelled() {
                return git2::TreeWalkResult::Abort;
            }
            if entry.kind() == Some(ObjectType::Blob) {
                let name = String::from_utf8_lossy(entry.name_bytes());
                out.insert(
                    format!("{root}{name}"),
                    (entry.id(), entry.filemode() as u32),
                );
            }
            git2::TreeWalkResult::Ok
        })?;
    }
    Ok(out)
}
fn index_entry(path: &str, id: Oid, mode: u32, size: u64) -> git2::IndexEntry {
    let zero = git2::IndexTime::new(0, 0);
    git2::IndexEntry {
        ctime: zero,
        mtime: zero,
        dev: 0,
        ino: 0,
        mode,
        uid: 0,
        gid: 0,
        file_size: size.min(u32::MAX as u64) as u32,
        id,
        flags: path.len().min(0xfff) as u16,
        flags_extended: 0,
        path: path.as_bytes().to_vec(),
    }
}
/// An exclude rule that matches exactly one workspace-relative directory.
fn anchored(relative: &Path) -> String {
    let mut rule = String::from("/");
    for c in relative.to_string_lossy().chars() {
        if matches!(c, '*' | '?' | '[' | ']' | '\\' | '!' | '#') {
            rule.push('\\');
        }
        rule.push(c);
    }
    rule.push_str("/\n");
    rule
}
/// A link target leaves the workspace when it is absolute, empty, not UTF-8, or climbs above
/// the root lexically. Targets are never resolved on disk.
pub fn link_escapes(link_path: &str, target: &[u8]) -> bool {
    let Ok(target) = std::str::from_utf8(target) else {
        return true;
    };
    if target.is_empty() || target.starts_with('/') {
        return true;
    }
    let mut depth = link_path.split('/').count() as i64 - 1;
    for part in target.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                depth -= 1;
                if depth < 0 {
                    return true;
                }
            }
            _ => depth += 1,
        }
    }
    false
}

struct Change {
    path: String,
    id: Oid,
    mode: u32,
    size: u64,
}
impl Change {
    fn entry(&self) -> git2::IndexEntry {
        index_entry(&self.path, self.id, self.mode, self.size)
    }
}
struct Packing<'a> {
    budget: u64,
    message: &'a str,
    hide: Vec<Oid>,
}
fn batches(changes: Vec<Change>, limit: u64) -> Vec<Vec<Change>> {
    let mut batches = Vec::new();
    let mut batch = Vec::new();
    let mut used = 0;
    for change in changes {
        if !batch.is_empty() && used + change.size > limit {
            batches.push(std::mem::take(&mut batch));
            used = 0;
        }
        used += change.size;
        batch.push(change);
    }
    if !batch.is_empty() {
        batches.push(batch);
    }
    batches
}
fn nested_repository(workspace: &Path, path: &Path) -> Option<String> {
    let mut candidate = workspace.join(path);
    while candidate != workspace {
        if candidate.is_dir() && candidate.join(".git").exists() {
            return Some(
                candidate
                    .strip_prefix(workspace)
                    .ok()?
                    .to_string_lossy()
                    .into_owned(),
            );
        }
        candidate = candidate.parent()?.to_path_buf();
    }
    None
}
