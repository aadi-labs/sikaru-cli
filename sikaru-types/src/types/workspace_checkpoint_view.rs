pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceCheckpointView {
    #[serde(default)]
    pub branch: String,
    #[serde(default)]
    pub commit_sha: String,
    #[serde(default)]
    pub recorded_at: String,
}

impl WorkspaceCheckpointView {
    pub fn builder() -> WorkspaceCheckpointViewBuilder {
        <WorkspaceCheckpointViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceCheckpointViewBuilder {
    branch: Option<String>,
    commit_sha: Option<String>,
    recorded_at: Option<String>,
}

impl WorkspaceCheckpointViewBuilder {
    pub fn branch(mut self, value: impl Into<String>) -> Self {
        self.branch = Some(value.into());
        self
    }

    pub fn commit_sha(mut self, value: impl Into<String>) -> Self {
        self.commit_sha = Some(value.into());
        self
    }

    pub fn recorded_at(mut self, value: impl Into<String>) -> Self {
        self.recorded_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceCheckpointView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branch`](WorkspaceCheckpointViewBuilder::branch)
    /// - [`commit_sha`](WorkspaceCheckpointViewBuilder::commit_sha)
    /// - [`recorded_at`](WorkspaceCheckpointViewBuilder::recorded_at)
    pub fn build(self) -> Result<WorkspaceCheckpointView, BuildError> {
        Ok(WorkspaceCheckpointView {
            branch: self.branch.ok_or_else(|| BuildError::missing_field("branch"))?,
            commit_sha: self.commit_sha.ok_or_else(|| BuildError::missing_field("commit_sha"))?,
            recorded_at: self.recorded_at.ok_or_else(|| BuildError::missing_field("recorded_at"))?,
        })
    }
}
