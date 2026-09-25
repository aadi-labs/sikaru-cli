pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkspaceCheckpointView {
    #[serde(default)]
    pub checkpoint_id: String,
    #[serde(default)]
    pub owner_epoch: i64,
    #[serde(default)]
    pub run_id: String,
    pub status: WorkspaceCheckpointViewStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_id: Option<String>,
    #[serde(default)]
    pub workspace_generation: String,
}

impl WorkspaceCheckpointView {
    pub fn builder() -> WorkspaceCheckpointViewBuilder {
        <WorkspaceCheckpointViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceCheckpointViewBuilder {
    checkpoint_id: Option<String>,
    owner_epoch: Option<i64>,
    run_id: Option<String>,
    status: Option<WorkspaceCheckpointViewStatus>,
    tree_id: Option<String>,
    workspace_generation: Option<String>,
}

impl WorkspaceCheckpointViewBuilder {
    pub fn checkpoint_id(mut self, value: impl Into<String>) -> Self {
        self.checkpoint_id = Some(value.into());
        self
    }

    pub fn owner_epoch(mut self, value: i64) -> Self {
        self.owner_epoch = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: WorkspaceCheckpointViewStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tree_id(mut self, value: impl Into<String>) -> Self {
        self.tree_id = Some(value.into());
        self
    }

    pub fn workspace_generation(mut self, value: impl Into<String>) -> Self {
        self.workspace_generation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceCheckpointView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`checkpoint_id`](WorkspaceCheckpointViewBuilder::checkpoint_id)
    /// - [`owner_epoch`](WorkspaceCheckpointViewBuilder::owner_epoch)
    /// - [`run_id`](WorkspaceCheckpointViewBuilder::run_id)
    /// - [`status`](WorkspaceCheckpointViewBuilder::status)
    /// - [`workspace_generation`](WorkspaceCheckpointViewBuilder::workspace_generation)
    pub fn build(self) -> Result<WorkspaceCheckpointView, BuildError> {
        Ok(WorkspaceCheckpointView {
            checkpoint_id: self.checkpoint_id.ok_or_else(|| BuildError::missing_field("checkpoint_id"))?,
            owner_epoch: self.owner_epoch.ok_or_else(|| BuildError::missing_field("owner_epoch"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            tree_id: self.tree_id,
            workspace_generation: self.workspace_generation.ok_or_else(|| BuildError::missing_field("workspace_generation"))?,
        })
    }
}
