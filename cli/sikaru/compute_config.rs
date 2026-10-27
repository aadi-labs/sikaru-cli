//! Executor bootstrap; never Debug or serialize the credential.
use serde::Deserialize;
use sikaru_sdk::api::WorkspaceProvenance;
use std::path::PathBuf;

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
pub fn control_variable(name: &str) -> bool {
    name.to_ascii_uppercase().starts_with("SIKARU_")
}
