pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkspaceCheckpointInput {
    #[serde(default)]
    pub commit_sha: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    pub trigger: WorkspaceCheckpointInputTrigger,
}

impl WorkspaceCheckpointInput {
    pub fn builder() -> WorkspaceCheckpointInputBuilder {
        <WorkspaceCheckpointInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceCheckpointInputBuilder {
    commit_sha: Option<String>,
    run_id: Option<String>,
    trigger: Option<WorkspaceCheckpointInputTrigger>,
}

impl WorkspaceCheckpointInputBuilder {
    pub fn commit_sha(mut self, value: impl Into<String>) -> Self {
        self.commit_sha = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn trigger(mut self, value: WorkspaceCheckpointInputTrigger) -> Self {
        self.trigger = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceCheckpointInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`commit_sha`](WorkspaceCheckpointInputBuilder::commit_sha)
    /// - [`trigger`](WorkspaceCheckpointInputBuilder::trigger)
    pub fn build(self) -> Result<WorkspaceCheckpointInput, BuildError> {
        Ok(WorkspaceCheckpointInput {
            commit_sha: self.commit_sha.ok_or_else(|| BuildError::missing_field("commit_sha"))?,
            run_id: self.run_id,
            trigger: self.trigger.ok_or_else(|| BuildError::missing_field("trigger"))?,
        })
    }
}

