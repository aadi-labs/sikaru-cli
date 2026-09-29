//! Publish one retained workspace capture before reporting task completion.
use super::{
    journal::Journal,
    transport::Transport,
    workspace::{self, FrozenWorkspace, UploadScope},
};
use anyhow::{bail, Context, Result};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use sikaru_sdk::api::*;
use tokio::{sync::watch, time::Instant};

pub fn validate(page: &WorkPage, journal: &Journal) -> Result<()> {
    let capture = page
        .workspace_checkpoint
        .as_ref()
        .context("missing workspace capture")?;
    let execution = page
        .execution
        .as_ref()
        .context("workspace capture has no execution")?;
    if capture.run_id != execution.run_id
        || capture.owner_epoch != journal.binding.owner_epoch
        || capture.workspace_generation != journal.binding.workspace_generation
        || capture.checkpoint_id.is_empty()
    {
        bail!("workspace capture authority mismatch");
    }
    Ok(())
}
pub async fn publish(
    transport: &Transport,
    journal: &Journal,
    page: &WorkPage,
    lease: &watch::Receiver<Instant>,
) -> Result<()> {
    validate(page, journal)?;
    let requested = page
        .workspace_checkpoint
        .as_ref()
        .context("missing workspace capture")?;
    // Copy the deadline first: a watch::Ref held across .await deadlocks lease renewal.
    let deadline = *lease.borrow();
    let current = transport
        .workspace_checkpoint(&requested.run_id, deadline)
        .await?;
    validate_receipt(requested, &current)?;
    match current.status {
        WorkspaceCheckpointViewStatus::Published | WorkspaceCheckpointViewStatus::Publishing => {
            return Ok(())
        }
        WorkspaceCheckpointViewStatus::Requested => (),
        _ => bail!("workspace checkpoint unsupported by this execution"),
    }
    let frozen = freeze(journal, &current.checkpoint_id).await?;
    let tree_id = tree_digest(&frozen.tree)?;
    upload_chunks(transport, journal, &current, &frozen, &tree_id, lease).await?;
    commit_tree(transport, journal, &current, frozen, &tree_id, lease).await
}
async fn upload_chunks(
    transport: &Transport,
    journal: &Journal,
    current: &WorkspaceCheckpointView,
    frozen: &FrozenWorkspace,
    tree_id: &str,
    lease: &watch::Receiver<Instant>,
) -> Result<()> {
    let scope = UploadScope {
        project_id: &journal.binding.project_id,
        attachment_id: &journal.binding.attachment_id,
        checkpoint_id: &current.checkpoint_id,
        run_id: &current.run_id,
        workspace_generation: &current.workspace_generation,
        owner_epoch: current.owner_epoch,
        tree_id,
    };
    for (hash, size) in frozen.chunk_ids()? {
        journal.verify()?;
        if frozen.uploaded(&scope, &hash, size)? {
            continue;
        }
        let bytes = frozen.chunk(&hash, size)?;
        let deadline = *lease.borrow();
        let receipt = transport
            .workspace_blob(&current.run_id, &hash, bytes, deadline)
            .await?;
        if receipt.sha256 != hash || receipt.size != size as i64 {
            bail!("workspace blob receipt mismatch");
        }
        journal.verify()?;
        frozen.acknowledge(&scope, &hash, size)?;
    }
    Ok(())
}
async fn commit_tree(
    transport: &Transport,
    journal: &Journal,
    current: &WorkspaceCheckpointView,
    frozen: FrozenWorkspace,
    expected: &str,
    lease: &watch::Receiver<Instant>,
) -> Result<()> {
    journal.verify()?;
    let tree: WorkspaceTreeInput = serde_json::from_value(frozen.tree)?;
    let deadline = *lease.borrow();
    let receipt = transport
        .workspace_tree(&current.run_id, &tree, deadline)
        .await?;
    validate_receipt(current, &receipt)?;
    if receipt.tree_id.as_deref() != Some(expected)
        || receipt.status != WorkspaceCheckpointViewStatus::Published
    {
        bail!("workspace tree receipt mismatch");
    }
    Ok(())
}

fn validate_receipt(
    expected: &WorkspaceCheckpointView,
    actual: &WorkspaceCheckpointView,
) -> Result<()> {
    if expected.checkpoint_id != actual.checkpoint_id
        || expected.run_id != actual.run_id
        || expected.workspace_generation != actual.workspace_generation
        || expected.owner_epoch != actual.owner_epoch
    {
        bail!("workspace checkpoint receipt identity mismatch");
    }
    if expected.tree_id.is_some() && expected.tree_id != actual.tree_id {
        bail!("workspace checkpoint tree changed");
    }
    Ok(())
}
async fn freeze(journal: &Journal, identity: &str) -> Result<FrozenWorkspace> {
    journal.verify()?;
    let root = journal.binding.anchor.path().to_owned();
    let state = journal.state_path().to_owned();
    let identity = identity.to_owned();
    let frozen =
        tokio::task::spawn_blocking(move || workspace::freeze(&root, &state, &identity)).await??;
    journal.verify()?;
    Ok(frozen)
}
#[derive(Serialize)]
struct CanonicalChunk<'a> {
    sha256: &'a str,
    size: u64,
}
#[derive(Serialize)]
struct CanonicalFile<'a> {
    sha256: &'a str,
    size: u64,
    mode: u64,
    chunks: Vec<CanonicalChunk<'a>>,
}
fn canonical_file(value: &Value) -> Result<CanonicalFile<'_>> {
    let chunks = value["chunks"]
        .as_array()
        .context("invalid workspace chunks")?
        .iter()
        .map(|chunk| {
            Ok(CanonicalChunk {
                sha256: chunk["sha256"].as_str().context("invalid chunk hash")?,
                size: chunk["size"].as_u64().context("invalid chunk size")?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(CanonicalFile {
        sha256: value["sha256"].as_str().context("invalid file hash")?,
        size: value["size"].as_u64().context("invalid file size")?,
        mode: value["mode"].as_u64().context("invalid file mode")?,
        chunks,
    })
}
pub fn tree_digest(tree: &Value) -> Result<String> {
    let files = tree["files"]
        .as_object()
        .context("invalid workspace tree")?;
    let mut paths = files.keys().collect::<Vec<_>>();
    paths.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
    let mut bytes = b"{\"files\":{".to_vec();
    for (index, path) in paths.iter().enumerate() {
        if index > 0 {
            bytes.push(b',');
        }
        serde_json::to_writer(&mut bytes, path)?;
        bytes.push(b':');
        serde_json::to_writer(&mut bytes, &canonical_file(&files[*path])?)?;
    }
    bytes.extend_from_slice(b"}}");
    Ok(format!("{:x}", Sha256::digest(bytes)))
}
