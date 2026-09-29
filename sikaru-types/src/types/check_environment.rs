pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckEnvironment {
    /// Required for local: the compute environment the run executes on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_id: Option<String>,
    /// managed runs in the Sikaru sandbox; local runs on a named compute environment.
    pub kind: CheckEnvironmentKind,
    /// Required for local: the workspace the environment provides.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_provenance: Option<WorkspaceProvenance>,
}

impl CheckEnvironment {
    pub fn builder() -> CheckEnvironmentBuilder {
        <CheckEnvironmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckEnvironmentBuilder {
    compute_environment_id: Option<String>,
    kind: Option<CheckEnvironmentKind>,
    workspace_provenance: Option<WorkspaceProvenance>,
}

impl CheckEnvironmentBuilder {
    pub fn compute_environment_id(mut self, value: impl Into<String>) -> Self {
        self.compute_environment_id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: CheckEnvironmentKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn workspace_provenance(mut self, value: WorkspaceProvenance) -> Self {
        self.workspace_provenance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckEnvironment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](CheckEnvironmentBuilder::kind)
    pub fn build(self) -> Result<CheckEnvironment, BuildError> {
        Ok(CheckEnvironment {
            compute_environment_id: self.compute_environment_id,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            workspace_provenance: self.workspace_provenance,
        })
    }
}
