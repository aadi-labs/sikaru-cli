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

#[cfg(test)]
pub const EXECUTABLE: u32 = 0o100755;
pub const LINK: u32 = 0o120000;
pub const GITLINK: u32 = 0o160000;
/// Maximum admitted content object size.
const MAX_BLOB_BYTES: u64 = 32_000_000;

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
pub struct Blob {
    pub id: Oid,
    pub sha256: String,
    pub size: usize,
}
pub struct Snapshot {
    pub manifest: Vec<u8>,
    pub blobs: Vec<Blob>,
    pub skipped: Vec<Skipped>,
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
    #[cfg(test)]
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
            init(git_dir, &cancellation)?;
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
        credentials: &[String],
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
        rules.push_str(&private_rules(&workspace, private_dirs));
        self.repo.add_ignore_rule(&credentials.join("\n"))?;
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
    pub fn snapshot(&self) -> Result<Snapshot> {
        let staged = self.stage()?;
        let mut files = BTreeMap::new();
        let mut blobs = Vec::new();
        let mut skipped = staged.skipped;
        for (path, (id, mode)) in
            entries(Some(&self.repo.find_tree(staged.tree)?), &self.cancellation)?
        {
            self.capture_entry(path, id, mode, &mut files, &mut blobs, &mut skipped)?;
        }
        Ok(Snapshot {
            manifest: serde_json::to_vec(&serde_json::json!({"files": files}))?,
            blobs,
            skipped,
        })
    }
    fn capture_entry(
        &self,
        path: String,
        id: Oid,
        mode: u32,
        files: &mut BTreeMap<String, serde_json::Value>,
        blobs: &mut Vec<Blob>,
        skipped: &mut Vec<Skipped>,
    ) -> Result<()> {
        use sha2::{Digest, Sha256};
        self.cancellation.check()?;
        let blob = self.repo.find_blob(id)?;
        if blob.size() as u64 > MAX_BLOB_BYTES {
            skipped.push(Skipped {
                path,
                reason: SkipReason::TooLarge,
            });
            return Ok(());
        }
        let entry = if mode == LINK {
            serde_json::json!({"link": std::str::from_utf8(blob.content())?})
        } else {
            let sha256 = format!("{:x}", Sha256::digest(blob.content()));
            let size = blob.size();
            let entry = serde_json::json!({"sha256": sha256, "size": size, "mode": mode & 0o777});
            blobs.push(Blob { id, sha256, size });
            entry
        };
        files.insert(path, entry);
        Ok(())
    }
    pub fn blob(&self, id: Oid) -> Result<Vec<u8>> {
        self.cancellation.check()?;
        Ok(self.repo.find_blob(id)?.content().to_vec())
    }
}

fn init(git_dir: &Path, cancellation: &Cancellation) -> Result<()> {
    cancellation.check()?;
    let staging = git_dir.with_extension("init");
    let _ = std::fs::remove_dir_all(&staging);
    let mut options = RepositoryInitOptions::new();
    options.bare(true).mkpath(true);
    let repo = Repository::init_opts(&staging, &options)?;
    let mut config = repo.config()?.open_level(ConfigLevel::Local)?;
    configure_private_repository(&mut config)?;
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
fn private_rules(workspace: &Path, private_dirs: &[PathBuf]) -> String {
    let mut rules = String::new();
    for dir in private_dirs {
        let dir = dir.canonicalize().unwrap_or_else(|_| dir.clone());
        if let Ok(relative) = dir.strip_prefix(workspace) {
            if !relative.as_os_str().is_empty() {
                rules.push_str(&anchored(relative));
            }
        }
    }
    rules
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
