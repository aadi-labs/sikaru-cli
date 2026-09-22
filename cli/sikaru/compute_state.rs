//! Private fsynced workflow ledger; secrets are never persisted here.
use super::journal::{acquire_lock, prepare_directory, private_file, Anchor};
use anyhow::{bail, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    os::fd::AsRawFd,
    path::Path,
};
pub struct State<T> {
    pub value: T,
    directory: File,
    _lock: File,
    anchor: Anchor,
}
impl<T: Serialize + DeserializeOwned> State<T> {
    pub fn open(path: &Path, initial: Option<T>) -> Result<Self> {
        let (dir, anchor) = open_directory(path)?;
        let lock = private_file(&dir, "workflow.lock", libc::O_RDWR | libc::O_CREAT)?;
        acquire_lock(&lock)?;
        let flags = libc::O_RDWR
            | libc::O_APPEND
            | if initial.is_some() {
                libc::O_CREAT | libc::O_EXCL
            } else {
                0
            };
        let mut file = private_file(&dir, "workflow.jsonl", flags)?;
        let value = load_value(&mut file, initial)?;
        let mut state = Self {
            value,
            directory: dir,
            _lock: lock,
            anchor,
        };
        state.save()?;
        state.directory.sync_all()?;
        Ok(state)
    }
    pub fn save(&mut self) -> Result<()> {
        self.verify()?;
        let mut bytes = serde_json::to_vec(&self.value)?;
        bytes.push(b'\n');
        if bytes.len() > 16 * 1024 * 1024 {
            bail!("workflow snapshot exceeds supported size");
        }
        self.replace_snapshot(&bytes)
    }
    fn replace_snapshot(&self, bytes: &[u8]) -> Result<()> {
        // Only the last snapshot is authoritative. Stage beside it under the
        // same directory lock; interrupted staging never damages that snapshot.
        self.remove_staging()?;
        let mut next = private_file(
            &self.directory,
            "workflow.next",
            libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL,
        )?;
        next.write_all(bytes)?;
        next.sync_all()?;
        self.verify()?;
        self.commit_snapshot()?;
        self.directory.sync_all()?;
        Ok(())
    }
    fn remove_staging(&self) -> Result<()> {
        let result =
            unsafe { libc::unlinkat(self.directory.as_raw_fd(), c"workflow.next".as_ptr(), 0) };
        if result < 0 {
            let error = std::io::Error::last_os_error();
            if error.kind() != std::io::ErrorKind::NotFound {
                return Err(error.into());
            }
        }
        Ok(())
    }
    fn commit_snapshot(&self) -> Result<()> {
        let fd = self.directory.as_raw_fd();
        let result = unsafe {
            libc::renameat(
                fd,
                c"workflow.next".as_ptr(),
                fd,
                c"workflow.jsonl".as_ptr(),
            )
        };
        if result < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        Ok(())
    }
    pub fn verify(&self) -> Result<()> {
        self.anchor.verify()
    }
}
pub fn identity() -> String {
    format!("{:032x}", rand::random::<u128>())
}
pub async fn call<T>(
    future: impl std::future::Future<Output = std::result::Result<T, sikaru_sdk::ApiError>>,
) -> Result<T> {
    tokio::time::timeout(std::time::Duration::from_secs(10), future)
        .await
        .map_err(|_| anyhow::Error::new(super::transport::TransportFailure::Transient))?
        .map_err(super::transport::classify)
}
pub fn failure(reason: &str) -> serde_json::Value {
    serde_json::json!({"status":"recovery_required","reason":reason,"execution":null,"cleanup":"unconfirmed","cancel_acknowledged":null,"usage":{"available":false}})
}

fn open_directory(path: &Path) -> Result<(File, Anchor)> {
    use std::os::unix::fs::OpenOptionsExt;
    prepare_directory(path)?;
    let anchor = Anchor::capture(path)?;
    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(path)?;
    Ok((file, anchor))
}
fn load_value<T: DeserializeOwned>(file: &mut File, initial: Option<T>) -> Result<T> {
    if let Some(value) = initial {
        return Ok(value);
    }
    let mut bytes = String::new();
    file.take(16 * 1024 * 1024 + 1).read_to_string(&mut bytes)?;
    if bytes.len() > 16 * 1024 * 1024 || !bytes.ends_with('\n') {
        bail!("incomplete or oversized workflow journal");
    }
    let line = bytes
        .lines()
        .last()
        .ok_or_else(|| anyhow::anyhow!("missing workflow journal"))?;
    Ok(serde_json::from_str(line)?)
}
