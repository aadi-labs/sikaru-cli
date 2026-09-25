//! Frozen task workspace capture. Private executor state must live outside the task root.
use anyhow::{bail, Context, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    ffi::CString,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::{
        fd::{AsRawFd, FromRawFd},
        unix::fs::{MetadataExt, OpenOptionsExt},
    },
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};
const CHUNK: usize = 1_000_000;
const MAX_BYTES: u64 = 1 << 30;
const MAX_ENTRIES: usize = 100_000;
const MAX_DEPTH: usize = 128;
static TEMPORARY_ID: AtomicU64 = AtomicU64::new(0);

pub struct FrozenWorkspace {
    pub tree: Value,
    directory: File,
}
impl FrozenWorkspace {
    pub fn chunk_ids(&self) -> Result<BTreeMap<String, u64>> {
        let mut chunks = BTreeMap::new();
        for entry in self.tree["files"]
            .as_object()
            .context("invalid saved workspace")?
            .values()
        {
            for chunk in entry["chunks"].as_array().context("invalid saved chunks")? {
                let hash = chunk["sha256"].as_str().context("invalid saved digest")?;
                validate_hash(hash)?;
                let size = chunk["size"].as_u64().context("invalid saved size")?;
                if size > CHUNK as u64 {
                    bail!("invalid saved chunk size");
                }
                if chunks
                    .insert(hash.to_owned(), size)
                    .is_some_and(|old| old != size)
                {
                    bail!("conflicting saved chunk sizes");
                }
            }
        }
        Ok(chunks)
    }
    pub fn chunk(&self, hash: &str, size: u64) -> Result<Vec<u8>> {
        validate_hash(hash)?;
        let bytes = private_read(&self.directory, hash, CHUNK)?;
        if bytes.len() as u64 != size || digest(&bytes) != hash {
            bail!("workspace staging integrity failure");
        }
        Ok(bytes)
    }
    #[cfg(test)]
    pub fn chunks(&self) -> Result<Vec<(String, Vec<u8>)>> {
        self.chunk_ids()?
            .into_iter()
            .map(|(id, size)| Ok((id.clone(), self.chunk(&id, size)?)))
            .collect()
    }
}
fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn validate_hash(value: &str) -> Result<()> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        bail!("invalid workspace digest");
    }
    Ok(())
}
pub fn freeze(workspace: &Path, state: &Path, identity: &str) -> Result<FrozenWorkspace> {
    let root = workspace.canonicalize()?;
    let state = state.canonicalize()?;
    if state.starts_with(&root) {
        bail!("private executor state must be outside the task workspace");
    }
    let state_anchor = directory_file(&state)?;
    let name = format!("workspace-{}", digest(identity.as_bytes()));
    let directory = prepare_staging(&state_anchor, &name)?;
    if let Some(tree) = saved_tree(&directory)? {
        return Ok(FrozenWorkspace { tree, directory });
    }
    let anchor = directory_file(&root)?;
    let mut capture = Capture {
        files: BTreeMap::new(),
        bytes: 0,
        entries: 0,
        staging: &directory,
    };
    capture.visit(&anchor, &root, "", 0)?;
    // A second anchored traversal detects edits, additions and removals during capture.
    let tree = json!({"files":capture.files});
    let mut verify = Capture {
        files: BTreeMap::new(),
        bytes: 0,
        entries: 0,
        staging: &directory,
    };
    verify.visit(&anchor, &root, "", 0)?;
    if tree != json!({"files":verify.files})
        || !same_node(&anchor.metadata()?, &fs::metadata(&root)?)
    {
        bail!("workspace changed during capture; no checkpoint was published");
    }
    let bytes = serde_json::to_vec(&tree)?;
    if bytes.len() > 8_000_000 {
        bail!("workspace manifest exceeds size limit");
    }
    write_private(&directory, "tree.json", &bytes)?;
    Ok(FrozenWorkspace { tree, directory })
}
fn saved_tree(directory: &File) -> Result<Option<Value>> {
    match private_read(directory, "tree.json", 8_000_000) {
        Ok(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
        Err(error)
            if error
                .downcast_ref::<std::io::Error>()
                .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) =>
        {
            Ok(None)
        }
        Err(error) => Err(error),
    }
}
fn prepare_staging(state: &File, name: &str) -> Result<File> {
    let name_c = CString::new(name)?;
    if unsafe { libc::mkdirat(state.as_raw_fd(), name_c.as_ptr(), 0o700) } < 0 {
        let error = std::io::Error::last_os_error();
        if error.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(error.into());
        }
    }
    let directory = open_relative(state, name, libc::O_RDONLY | libc::O_DIRECTORY)?;
    let metadata = directory.metadata()?;
    if !metadata.is_dir()
        || metadata.mode() & 0o077 != 0
        || metadata.uid() != unsafe { libc::geteuid() }
    {
        bail!("workspace staging is not private");
    }
    state.sync_all()?;
    Ok(directory)
}

struct Capture<'a> {
    files: BTreeMap<String, Value>,
    bytes: u64,
    entries: usize,
    staging: &'a File,
}
impl Capture<'_> {
    fn visit(&mut self, parent: &File, path: &Path, prefix: &str, depth: usize) -> Result<()> {
        if depth > MAX_DEPTH {
            bail!("workspace exceeds directory depth limit");
        }
        let before = parent.metadata()?;
        for child in fs::read_dir(path)? {
            self.entries += 1;
            if self.entries > MAX_ENTRIES {
                bail!("workspace exceeds traversal entry limit");
            }
            self.visit_entry(parent, child?, prefix, depth)?;
        }
        if !unchanged(&before, &parent.metadata()?) {
            bail!("workspace directory changed during capture");
        }
        Ok(())
    }
    fn visit_entry(
        &mut self,
        parent: &File,
        child: fs::DirEntry,
        prefix: &str,
        depth: usize,
    ) -> Result<()> {
        let name = child
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("workspace paths must be UTF-8"))?;
        let relative = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };
        validate_path(&relative)?;
        let file = open_child(parent, &name)?;
        let metadata = file.metadata()?;
        if metadata.is_dir() {
            self.visit(&file, &child.path(), &relative, depth + 1)?;
        } else {
            self.capture_file(file, &relative, metadata)?;
        }
        Ok(())
    }
    fn capture_file(&mut self, mut file: File, path: &str, before: fs::Metadata) -> Result<()> {
        if !before.is_file() || before.nlink() != 1 {
            bail!("workspace requires regular files without external hard links");
        }
        self.bytes = self
            .bytes
            .checked_add(before.len())
            .context("workspace size overflow")?;
        if self.bytes > MAX_BYTES || self.files.len() >= 50_000 {
            bail!("workspace exceeds capture limits");
        }
        let (chunks, whole, size) = stage_file(&mut file, before.len(), self.staging)?;
        if size != before.len() || !unchanged(&before, &file.metadata()?) {
            bail!("workspace file changed during capture");
        }
        self.files.insert(path.into(),json!({"sha256":format!("{:x}",whole.finalize()),"size":size,"mode":before.mode() & 0o777,"chunks":chunks}));
        Ok(())
    }
}
fn stage_file(file: &mut File, expected: u64, staging: &File) -> Result<(Vec<Value>, Sha256, u64)> {
    let mut chunks = Vec::new();
    let mut whole = Sha256::new();
    let mut size = 0u64;
    loop {
        let mut bytes = Vec::new();
        (&mut *file).take(CHUNK as u64).read_to_end(&mut bytes)?;
        if bytes.is_empty() && !chunks.is_empty() {
            break;
        }
        size += bytes.len() as u64;
        if size > expected {
            bail!("workspace file changed during capture");
        }
        whole.update(&bytes);
        let hash = digest(&bytes);
        write_private(staging, &hash, &bytes)?;
        chunks.push(json!({"sha256":hash,"size":bytes.len()}));
        if bytes.len() < CHUNK {
            break;
        }
    }
    Ok((chunks, whole, size))
}

fn validate_path(path: &str) -> Result<()> {
    if path.len() > 512 || path.chars().any(|c| c.is_control() || c == '\\') {
        bail!("unsupported workspace path");
    }
    Ok(())
}
fn directory_file(path: &Path) -> Result<File> {
    Ok(OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(path)?)
}
fn open_child(parent: &File, name: &str) -> Result<File> {
    open_relative(parent, name, libc::O_RDONLY | libc::O_NONBLOCK)
}
fn open_relative(parent: &File, name: &str, flags: i32) -> Result<File> {
    let name = CString::new(name)?;
    let fd = unsafe {
        libc::openat(
            parent.as_raw_fd(),
            name.as_ptr(),
            flags | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            0o600,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(unsafe { File::from_raw_fd(fd) })
}
fn same_node(a: &fs::Metadata, b: &fs::Metadata) -> bool {
    a.dev() == b.dev() && a.ino() == b.ino()
}
fn unchanged(a: &fs::Metadata, b: &fs::Metadata) -> bool {
    same_node(a, b)
        && a.len() == b.len()
        && a.mtime() == b.mtime()
        && a.mtime_nsec() == b.mtime_nsec()
        && a.ctime() == b.ctime()
        && a.ctime_nsec() == b.ctime_nsec()
        && a.mode() == b.mode()
}
fn private_read(directory: &File, name: &str, limit: usize) -> Result<Vec<u8>> {
    let mut file = open_relative(directory, name, libc::O_RDONLY | libc::O_NONBLOCK)?;
    let before = file.metadata()?;
    validate_private(&before, limit)?;
    let mut bytes = Vec::new();
    (&mut file).take(limit as u64 + 1).read_to_end(&mut bytes)?;
    if bytes.len() > limit || !unchanged(&before, &file.metadata()?) {
        bail!("private workspace staging changed while reading");
    }
    Ok(bytes)
}
fn validate_private(metadata: &fs::Metadata, limit: usize) -> Result<()> {
    if !metadata.is_file()
        || metadata.nlink() != 1
        || metadata.mode() & 0o077 != 0
        || metadata.uid() != unsafe { libc::geteuid() }
        || metadata.len() > limit as u64
    {
        bail!("invalid private workspace staging file");
    }
    Ok(())
}
struct PendingFile<'a> {
    directory: &'a File,
    name: CString,
}
impl Drop for PendingFile<'_> {
    fn drop(&mut self) {
        unsafe {
            libc::unlinkat(self.directory.as_raw_fd(), self.name.as_ptr(), 0);
        }
    }
}
fn temporary(directory: &File) -> Result<(PendingFile<'_>, File)> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let id = TEMPORARY_ID.fetch_add(1, Ordering::Relaxed);
    let name = format!(".partial-{}-{timestamp}-{id}", std::process::id());
    let file = open_relative(
        directory,
        &name,
        libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL,
    )?;
    Ok((
        PendingFile {
            directory,
            name: CString::new(name)?,
        },
        file,
    ))
}
fn write_private(directory: &File, name: &str, bytes: &[u8]) -> Result<()> {
    write_private_with(directory, name, bytes, |file, data| {
        Ok(file.write_all(data)?)
    })
}
fn write_private_with(
    directory: &File,
    name: &str,
    bytes: &[u8],
    write: impl FnOnce(&mut File, &[u8]) -> Result<()>,
) -> Result<()> {
    if existing_private(directory, name, bytes)? {
        directory.sync_all()?;
        return Ok(());
    }
    let (pending, mut file) = temporary(directory)?;
    write(&mut file, bytes)?;
    file.sync_all()?;
    commit_private(directory, &pending.name, name, bytes)
}
fn commit_private(directory: &File, pending: &CString, name: &str, bytes: &[u8]) -> Result<()> {
    match install_private(directory, pending, &CString::new(name)?) {
        Ok(()) => (),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            if private_read(directory, name, bytes.len())? != bytes {
                bail!("immutable workspace staging conflict");
            }
        }
        Err(error) => return Err(error.into()),
    }
    directory.sync_all()?;
    Ok(())
}
fn existing_private(directory: &File, name: &str, bytes: &[u8]) -> Result<bool> {
    match private_read(directory, name, bytes.len()) {
        Ok(existing) if existing == bytes => Ok(true),
        Ok(_) => bail!("immutable workspace staging conflict"),
        Err(error)
            if error
                .downcast_ref::<std::io::Error>()
                .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) =>
        {
            Ok(false)
        }
        Err(error) => Err(error),
    }
}
#[cfg(target_os = "linux")]
fn install_private(directory: &File, from: &CString, to: &CString) -> std::io::Result<()> {
    let result = unsafe {
        libc::syscall(
            libc::SYS_renameat2,
            directory.as_raw_fd(),
            from.as_ptr(),
            directory.as_raw_fd(),
            to.as_ptr(),
            libc::RENAME_NOREPLACE,
        )
    };
    if result < 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}
#[cfg(target_os = "macos")]
fn install_private(directory: &File, from: &CString, to: &CString) -> std::io::Result<()> {
    let result = unsafe {
        libc::renameatx_np(
            directory.as_raw_fd(),
            from.as_ptr(),
            directory.as_raw_fd(),
            to.as_ptr(),
            libc::RENAME_EXCL,
        )
    };
    if result < 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}
#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn install_private(_: &File, _: &CString, _: &CString) -> std::io::Result<()> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "atomic workspace capture is unavailable on this platform",
    ))
}

#[cfg(test)]
mod atomic_tests {
    use super::*;
    #[test]
    fn failed_write_leaves_no_partial_final_and_retry_succeeds() {
        let root = tempfile::tempdir().unwrap();
        let directory = directory_file(root.path()).unwrap();
        let failure = write_private_with(&directory, "tree.json", b"complete", |file, _| {
            file.write_all(b"part")?;
            bail!("simulated storage interruption");
        });
        assert!(failure.is_err());
        assert!(!root.path().join("tree.json").exists());
        write_private(&directory, "tree.json", b"complete").unwrap();
        assert_eq!(
            private_read(&directory, "tree.json", 8).unwrap(),
            b"complete"
        );
        assert!(write_private(&directory, "tree.json", b"different").is_err());
        assert_eq!(
            private_read(&directory, "tree.json", 8).unwrap(),
            b"complete"
        );
    }
    #[test]
    fn empty_directories_consume_traversal_budget() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir(root.path().join("empty")).unwrap();
        let directory = directory_file(root.path()).unwrap();
        let mut capture = Capture {
            files: BTreeMap::new(),
            bytes: 0,
            entries: MAX_ENTRIES,
            staging: &directory,
        };
        assert!(capture
            .visit(&directory, root.path(), "", 0)
            .unwrap_err()
            .to_string()
            .contains("entry limit"));
    }
}
