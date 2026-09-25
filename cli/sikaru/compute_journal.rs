//! Fsynced intent/receipt ledger. Numeric process IDs are never recovery authority.
use super::config::Bootstrap;
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::os::{
    fd::{AsRawFd, FromRawFd},
    unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt},
};
use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
};

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Anchor {
    path: PathBuf,
    device: u64,
    inode: u64,
}
impl Anchor {
    pub fn capture(path: &Path) -> Result<Self> {
        let path = path.canonicalize().context("workspace must exist")?;
        let meta = path.metadata()?;
        if !meta.is_dir() {
            bail!("workspace must be a directory");
        }
        Ok(Self {
            path,
            device: meta.dev(),
            inode: meta.ino(),
        })
    }
    pub fn verify(&self) -> Result<()> {
        if Self::capture(&self.path)? != *self {
            bail!("workspace or private state was replaced");
        }
        Ok(())
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
}
#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Binding {
    pub project_id: String,
    pub session_id: String,
    pub attachment_id: String,
    pub owner_epoch: i64,
    pub workspace_generation: String,
    pub journal_id: String,
    pub credential_id: String,
    pub workspace_provenance: sikaru_sdk::api::WorkspaceProvenance,
    pub anchor: Anchor,
}
impl Binding {
    pub fn from_bootstrap(b: &Bootstrap) -> Result<Self> {
        Ok(Self {
            project_id: b.project_id.clone(),
            session_id: b.session_id.clone(),
            attachment_id: b.attachment_id.clone(),
            owner_epoch: b.owner_epoch,
            workspace_generation: b.workspace_generation.clone(),
            journal_id: b.journal_id.clone(),
            credential_id: b.credential_id.clone(),
            workspace_provenance: b.workspace_provenance.clone(),
            anchor: Anchor::capture(&b.workspace)?,
        })
    }
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Entry {
    pub request: Value,
    pub receipt: Option<Value>,
    pub acknowledged: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "record")]
enum Record {
    Header { binding: Binding, instance: String },
    Intent { key: String, request: Value },
    Receipt { key: String, receipt: Value },
    Ack { key: String },
    Handle { id: String, state: Value },
    Clean,
}
pub struct Journal {
    pub binding: Binding,
    pub instance: String,
    pub entries: BTreeMap<String, Entry>,
    pub handles: BTreeMap<String, Value>,
    pub clean: bool,
    directory: File,
    state_anchor: Anchor,
    stream: File,
    _lock: File,
}
impl Journal {
    #[cfg(test)]
    pub fn open(path: &Path, binding: Binding, instance: String) -> Result<Self> {
        Self::open_verified(path, binding, instance, false)
    }
    pub fn open_verified(
        path: &Path,
        binding: Binding,
        instance: String,
        teardown_verified: bool,
    ) -> Result<Self> {
        prepare_directory(path)?;
        let directory = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
            .open(path)?;
        let lock = private_file(&directory, "lock", libc::O_RDWR | libc::O_CREAT)?;
        acquire_lock(&lock)?;
        let stream = private_file(
            &directory,
            "journal.jsonl",
            libc::O_RDWR | libc::O_CREAT | libc::O_APPEND,
        )?;
        let mut journal = Self {
            binding,
            instance,
            entries: BTreeMap::new(),
            handles: BTreeMap::new(),
            clean: false,
            directory,
            state_anchor: Anchor::capture(path)?,
            stream,
            _lock: lock,
        };
        journal.load(teardown_verified)?;
        Ok(journal)
    }
    fn load(&mut self, teardown_verified: bool) -> Result<()> {
        if self.stream.metadata()?.len() == 0 {
            return self.append(&Record::Header {
                binding: self.binding.clone(),
                instance: self.instance.clone(),
            });
        }
        let (old, old_instance) = self.restore_records()?;
        self.validate_restart(old, old_instance, teardown_verified)?;
        // Unique receipts remain authoritative forever. Compact only redundant
        // observations; a large valid history must still be readable afterward.
        if self.stream.metadata()?.len() > 64 * 1024 * 1024 {
            self.compact()?;
        }
        Ok(())
    }
    fn restore_records(&mut self) -> Result<(Binding, String)> {
        let mut reader = BufReader::new(self.stream.try_clone()?);
        let (mut old, mut old_instance) = match read_record(&mut reader)? {
            Some(Record::Header { binding, instance }) => (binding, instance),
            _ => bail!("invalid journal header"),
        };
        while let Some(record) = read_record(&mut reader)? {
            match record {
                Record::Header { binding, instance } => {
                    old = binding;
                    old_instance = instance;
                    self.clean = false;
                }
                record => self.restore(record)?,
            }
        }
        Ok((old, old_instance))
    }
    fn compact(&mut self) -> Result<()> {
        self.verify()?;
        self.remove_staging()?;
        let mut next = private_file(
            &self.directory,
            "journal.next",
            libc::O_RDWR | libc::O_APPEND | libc::O_CREAT | libc::O_EXCL,
        )?;
        self.write_snapshot(&mut next)?;
        next.sync_all()?;
        self.verify()?;
        let fd = self.directory.as_raw_fd();
        let result =
            unsafe { libc::renameat(fd, c"journal.next".as_ptr(), fd, c"journal.jsonl".as_ptr()) };
        if result < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        // Switch descriptors before directory fsync: even if it fails, never
        // append to the now-unlinked old ledger. The separate lock stays held.
        self.stream = next;
        self.directory.sync_all()?;
        Ok(())
    }
    fn remove_staging(&self) -> Result<()> {
        let result =
            unsafe { libc::unlinkat(self.directory.as_raw_fd(), c"journal.next".as_ptr(), 0) };
        if result < 0 {
            let error = std::io::Error::last_os_error();
            if error.kind() != std::io::ErrorKind::NotFound {
                return Err(error.into());
            }
        }
        Ok(())
    }
    fn write_snapshot(&self, next: &mut File) -> Result<()> {
        write_record(
            next,
            &Record::Header {
                binding: self.binding.clone(),
                instance: self.instance.clone(),
            },
        )?;
        for (key, entry) in &self.entries {
            write_entry(next, key, entry)?;
        }
        for (id, state) in &self.handles {
            write_record(
                next,
                &Record::Handle {
                    id: id.clone(),
                    state: state.clone(),
                },
            )?;
        }
        if self.clean {
            write_record(next, &Record::Clean)?;
        }
        Ok(())
    }
    fn validate_restart(
        &mut self,
        mut old: Binding,
        old_instance: String,
        teardown_verified: bool,
    ) -> Result<()> {
        if self.same_process(&old, &old_instance) {
            return Ok(());
        }
        if old.owner_epoch >= self.binding.owner_epoch {
            bail!("recovery_required: executor restart requires teardown and a new epoch");
        }
        old.owner_epoch = self.binding.owner_epoch;
        old.credential_id = self.binding.credential_id.clone();
        if old != self.binding || !(self.clean || teardown_verified) || self.has_uncertain_effects()
        {
            bail!("recovery_required: journal identity or cleanup is unproven");
        }
        if teardown_verified {
            self.record_external_teardown()?;
        }
        self.append(&Record::Header {
            binding: self.binding.clone(),
            instance: self.instance.clone(),
        })?;
        self.clean = false;
        Ok(())
    }
    fn same_process(&self, binding: &Binding, instance: &str) -> bool {
        binding == &self.binding && instance == self.instance
    }
    pub fn has_uncertain_effects(&self) -> bool {
        self.entries.values().any(|entry| entry.receipt.is_none())
    }
    fn record_external_teardown(&mut self) -> Result<()> {
        let running = self
            .handles
            .iter()
            .filter(|(_, v)| v["status"] == "running")
            .map(|(id, v)| (id.clone(), v.clone()))
            .collect::<Vec<_>>();
        for (id, mut state) in running {
            state["status"] = Value::String("cancelled".into());
            state["reason"] = Value::String("external_teardown_confirmed".into());
            self.handle(&id, state)?;
        }
        Ok(())
    }
    fn restore(&mut self, record: Record) -> Result<()> {
        match record {
            Record::Intent { key, request } => {
                self.entries.insert(
                    key,
                    Entry {
                        request,
                        receipt: None,
                        acknowledged: false,
                    },
                );
            }
            Record::Receipt { key, receipt } => {
                self.entries
                    .get_mut(&key)
                    .context("receipt without intent")?
                    .receipt = Some(receipt)
            }
            Record::Ack { key } => {
                self.entries
                    .get_mut(&key)
                    .context("ack without intent")?
                    .acknowledged = true
            }
            Record::Handle { id, state } => {
                self.handles.insert(id, state);
            }
            Record::Clean => self.clean = true,
            Record::Header { .. } => bail!("unexpected nested journal header"),
        }
        Ok(())
    }
    pub fn state_path(&self) -> &Path {
        self.state_anchor.path()
    }
    pub fn verify(&self) -> Result<()> {
        self.binding.anchor.verify()?;
        self.state_anchor.verify()
    }
    fn append(&mut self, record: &Record) -> Result<()> {
        self.verify()?;
        write_record(&mut self.stream, record)?;
        self.stream.sync_all()?;
        self.directory.sync_all()?;
        Ok(())
    }
    pub fn intent(&mut self, key: &str, request: Value) -> Result<Option<Value>> {
        self.verify()?;
        // Bind the complete incoming identity and arguments without retaining task-provided secrets.
        let request = Value::String(format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&request)?)
        ));
        if let Some(entry) = self.entries.get(key) {
            if entry.request != request {
                bail!("operation identity was reused with altered arguments");
            }
            return entry
                .receipt
                .clone()
                .map(Some)
                .context("recovery_required: uncertain effect must not be replayed");
        }
        self.append(&Record::Intent {
            key: key.into(),
            request: request.clone(),
        })?;
        self.entries.insert(
            key.into(),
            Entry {
                request,
                receipt: None,
                acknowledged: false,
            },
        );
        Ok(None)
    }
    pub fn receipt(&mut self, key: &str, receipt: Value) -> Result<()> {
        if serde_json::to_vec(&receipt)?.len() > 256 * 1024 {
            bail!("receipt exceeds transport limit; effect remains uncertain");
        }
        let entry = self.entries.get(key).context("receipt has no intent")?;
        if let Some(old) = &entry.receipt {
            if old != &receipt {
                bail!("immutable receipt conflict");
            }
            return Ok(());
        }
        self.append(&Record::Receipt {
            key: key.into(),
            receipt: receipt.clone(),
        })?;
        self.entries.get_mut(key).unwrap().receipt = Some(receipt);
        Ok(())
    }
    pub fn ack(&mut self, key: &str) -> Result<()> {
        if self.entries.get(key).context("unknown ack")?.acknowledged {
            return Ok(());
        }
        self.append(&Record::Ack { key: key.into() })?;
        self.entries.get_mut(key).unwrap().acknowledged = true;
        Ok(())
    }
    pub fn handle(&mut self, id: &str, state: Value) -> Result<()> {
        self.append(&Record::Handle {
            id: id.into(),
            state: state.clone(),
        })?;
        self.handles.insert(id.into(), state);
        Ok(())
    }
    pub fn mark_clean(&mut self) -> Result<()> {
        self.append(&Record::Clean)?;
        self.clean = true;
        Ok(())
    }
    pub fn existing_artifact(&self, id: &str) -> Result<(File, PathBuf)> {
        self.verify()?;
        if id.len() != 32 || !id.bytes().all(|c| c.is_ascii_hexdigit()) {
            bail!("invalid artifact identity");
        }
        let name = format!("output-{id}");
        Ok((
            private_file(&self.directory, &name, libc::O_RDONLY)?,
            self.state_anchor.path.join(name),
        ))
    }
    pub fn artifact(&self, id: &str) -> Result<(File, PathBuf)> {
        self.verify()?;
        let name = format!("output-{id}");
        let file = private_file(
            &self.directory,
            &name,
            libc::O_RDWR | libc::O_CREAT | libc::O_EXCL,
        )?;
        Ok((file, self.state_anchor.path.join(name)))
    }
}
pub(crate) fn prepare_directory(path: &Path) -> Result<()> {
    for ancestor in path.ancestors() {
        if let Ok(meta) = std::fs::symlink_metadata(ancestor) {
            if meta.file_type().is_symlink() {
                bail!("private state path contains a symlink");
            }
        }
    }
    if !path.exists() {
        std::fs::DirBuilder::new().mode(0o700).create(path)?;
    }
    let m = std::fs::symlink_metadata(path)?;
    if !m.is_dir() || m.mode() & 0o077 != 0 || m.uid() != unsafe { libc::geteuid() } {
        bail!("state directory must be private and owned");
    }
    Ok(())
}
pub(crate) fn private_file(directory: &File, name: &str, flags: i32) -> Result<File> {
    let name = std::ffi::CString::new(name)?;
    let fd = unsafe {
        libc::openat(
            directory.as_raw_fd(),
            name.as_ptr(),
            flags | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            0o600,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let file = unsafe { File::from_raw_fd(fd) };
    let m = file.metadata()?;
    if !m.is_file()
        || m.mode() & 0o077 != 0
        || m.uid() != unsafe { libc::geteuid() }
        || m.nlink() != 1
    {
        bail!("journal file is not private and owned");
    }
    Ok(file)
}

pub(crate) fn acquire_lock(lock: &File) -> Result<()> {
    // Concurrent fork/exec can temporarily retain a CLOEXEC descriptor after its
    // owner drops it. A bounded retry accommodates that window without takeover.
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(250);
    loop {
        if unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
            return Ok(());
        }
        let error = std::io::Error::last_os_error();
        if error.kind() != std::io::ErrorKind::WouldBlock {
            return Err(error.into());
        }
        if std::time::Instant::now() >= deadline {
            bail!("journal is already locked");
        }
        std::thread::sleep(std::time::Duration::from_millis(2));
    }
}

// Bound allocation by the accepted record size, not the lifetime of the journal.
fn read_record(reader: &mut impl BufRead) -> Result<Option<Record>> {
    loop {
        let mut bytes = Vec::new();
        reader.take(1024 * 1024 + 2).read_until(b'\n', &mut bytes)?;
        if bytes.is_empty() {
            return Ok(None);
        }
        if bytes.len() > 1024 * 1024 + 1 {
            bail!("journal record exceeds limit");
        }
        if bytes.pop() != Some(b'\n') {
            bail!("recovery_required: interrupted journal write");
        }
        if !bytes.is_empty() {
            return Ok(Some(serde_json::from_slice(&bytes)?));
        }
    }
}
fn write_record(stream: &mut File, record: &Record) -> Result<()> {
    let mut bytes = serde_json::to_vec(record)?;
    if bytes.len() > 1024 * 1024 {
        bail!("journal record exceeds limit");
    }
    bytes.push(b'\n');
    stream.write_all(&bytes)?;
    Ok(())
}

fn write_entry(next: &mut File, key: &str, entry: &Entry) -> Result<()> {
    write_record(
        next,
        &Record::Intent {
            key: key.into(),
            request: entry.request.clone(),
        },
    )?;
    if let Some(receipt) = &entry.receipt {
        write_record(
            next,
            &Record::Receipt {
                key: key.into(),
                receipt: receipt.clone(),
            },
        )?;
    }
    if entry.acknowledged {
        write_record(next, &Record::Ack { key: key.into() })?;
    }
    Ok(())
}
