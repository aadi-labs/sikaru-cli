//! Customer launcher protocol v1. Invocation acknowledgement is not executor readiness.
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{path::Path, process::Stdio, time::Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[derive(Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Handle {
    pub kind: String,
    pub id: String,
    pub proof: String,
}
impl Handle {
    pub fn validate(&self) -> Result<()> {
        if !["container", "sandbox"].contains(&self.kind.as_str())
            || self.id.is_empty()
            || self.proof.is_empty()
        {
            bail!("launcher must prove sandbox identity, never a bare PID");
        }
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Response {
    pub version: u32,
    pub launch_id: String,
    pub status: String,
    pub handle: Option<Handle>,
    pub evidence: Option<String>,
}
pub async fn invoke(path: &Path, input: Value, timeout: Duration) -> Result<Response> {
    let mut command = tokio::process::Command::new(path);
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .kill_on_drop(true);
    for (name, _) in std::env::vars_os() {
        if super::config::control_variable(&name.to_string_lossy()) {
            command.env_remove(name);
        }
    }
    let mut child = command.spawn()?;
    let operation = exchange(&mut child, &input);
    match tokio::time::timeout(timeout, operation).await {
        Ok(value) => validate_response(value?, &input),
        Err(_) => {
            let _ = child.kill().await;
            let _ = child.wait().await;
            bail!("launcher acknowledgement timeout")
        }
    }
}

async fn exchange(child: &mut tokio::process::Child, input: &Value) -> Result<Vec<u8>> {
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(&serde_json::to_vec(input)?).await?;
    stdin.shutdown().await?;
    drop(stdin);
    let mut bytes = Vec::new();
    child
        .stdout
        .take()
        .unwrap()
        .take(65537)
        .read_to_end(&mut bytes)
        .await?;
    if bytes.len() > 65536 {
        bail!("launcher response exceeds 64 KiB");
    }
    if !child.wait().await?.success() {
        bail!("launcher acknowledgement unavailable");
    }
    Ok(bytes)
}
fn validate_response(bytes: Vec<u8>, input: &Value) -> Result<Response> {
    let response: Response =
        serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid launcher response"))?;
    if response.version != 1 || input["launch_id"] != response.launch_id {
        bail!("launcher identity mismatch");
    }
    if let Some(handle) = &response.handle {
        handle.validate()?;
    }
    Ok(response)
}
