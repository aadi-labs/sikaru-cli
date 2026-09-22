pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AttachmentInput {
    #[serde(default)]
    pub environment_id: String,
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_existing: Option<bool>,
    pub workspace_provenance: WorkspaceProvenance,
}

impl AttachmentInput {
    pub fn builder() -> AttachmentInputBuilder {
        <AttachmentInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttachmentInputBuilder {
    environment_id: Option<String>,
    idempotency_key: Option<String>,
    replace_existing: Option<bool>,
    workspace_provenance: Option<WorkspaceProvenance>,
}

impl AttachmentInputBuilder {
    pub fn environment_id(mut self, value: impl Into<String>) -> Self {
        self.environment_id = Some(value.into());
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn replace_existing(mut self, value: bool) -> Self {
        self.replace_existing = Some(value);
        self
    }

    pub fn workspace_provenance(mut self, value: WorkspaceProvenance) -> Self {
        self.workspace_provenance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AttachmentInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment_id`](AttachmentInputBuilder::environment_id)
    /// - [`idempotency_key`](AttachmentInputBuilder::idempotency_key)
    /// - [`workspace_provenance`](AttachmentInputBuilder::workspace_provenance)
    pub fn build(self) -> Result<AttachmentInput, BuildError> {
        Ok(AttachmentInput {
            environment_id: self.environment_id.ok_or_else(|| BuildError::missing_field("environment_id"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            replace_existing: self.replace_existing,
            workspace_provenance: self.workspace_provenance.ok_or_else(|| BuildError::missing_field("workspace_provenance"))?,
        })
    }
}

