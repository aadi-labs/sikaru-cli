pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkspaceProvenance {
    #[serde(default)]
    pub identity: String,
    pub kind: WorkspaceProvenanceKind,
}

impl WorkspaceProvenance {
    pub fn builder() -> WorkspaceProvenanceBuilder {
        <WorkspaceProvenanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceProvenanceBuilder {
    identity: Option<String>,
    kind: Option<WorkspaceProvenanceKind>,
}

impl WorkspaceProvenanceBuilder {
    pub fn identity(mut self, value: impl Into<String>) -> Self {
        self.identity = Some(value.into());
        self
    }

    pub fn kind(mut self, value: WorkspaceProvenanceKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceProvenance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`identity`](WorkspaceProvenanceBuilder::identity)
    /// - [`kind`](WorkspaceProvenanceBuilder::kind)
    pub fn build(self) -> Result<WorkspaceProvenance, BuildError> {
        Ok(WorkspaceProvenance {
            identity: self.identity.ok_or_else(|| BuildError::missing_field("identity"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
