//! Protected executor bootstrap; never Debug or serialize the credential.
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use sikaru_sdk::api::WorkspaceProvenance;
use std::{io::Read, path::PathBuf};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bootstrap {
    pub project_id: String,
    pub session_id: String,
    pub attachment_id: String,
    pub owner_epoch: i64,
    pub workspace_generation: String,
    pub journal_id: String,
    pub credential_id: String,
    pub token: String,
    pub workspace_provenance: WorkspaceProvenance,
    pub workspace: PathBuf,
    pub state_dir: PathBuf,
    #[serde(default = "command_timeout")]
    pub command_timeout_seconds: u64,
}
fn command_timeout() -> u64 {
    120
}
impl Bootstrap {
    pub fn read(source: &str) -> Result<Self> {
        let value: Self = read_private_json(source)?;
        value.validate()?;
        Ok(value)
    }
    fn validate(&self) -> Result<()> {
        for field in [
            &self.project_id,
            &self.session_id,
            &self.attachment_id,
            &self.workspace_generation,
            &self.journal_id,
            &self.credential_id,
            &self.token,
            &self.workspace_provenance.identity,
        ] {
            if field.is_empty() {
                bail!("bootstrap identity and credential fields must be nonempty");
            }
        }
        if self.owner_epoch < 1 || !(1..=86400).contains(&self.command_timeout_seconds) {
            bail!("invalid executor epoch or command deadline");
        }
        Ok(())
    }
}
#[cfg(unix)]
fn protected_input(source: &str) -> Result<std::fs::File> {
    use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
    let file = std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .open(source)
        .context("open private bootstrap")?;
    let m = file.metadata()?;
    if !m.is_file()
        || m.mode() & 0o077 != 0
        || m.uid() != unsafe { libc::geteuid() }
        || m.nlink() != 1
    {
        bail!("bootstrap must be a private, owned regular file");
    }
    Ok(file)
}
#[cfg(not(unix))]
fn protected_input(_: &str) -> Result<std::fs::File> {
    bail!("native executor requires Unix")
}

pub fn control_variable(name: &str) -> bool {
    name.to_ascii_uppercase().starts_with("SIKARU_")
}

pub fn read_private_json<T: serde::de::DeserializeOwned>(source: &str) -> Result<T> {
    let mut data = Vec::new();
    if source == "-" {
        std::io::stdin().take(65537).read_to_end(&mut data)?;
    } else {
        protected_input(source)?
            .take(65537)
            .read_to_end(&mut data)?;
    }
    if data.len() > 65536 {
        bail!("bootstrap exceeds 64 KiB");
    }
    serde_json::from_slice(&data).map_err(|_| anyhow::anyhow!("invalid private bootstrap"))
}
