pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentImportSourceRefRequest {
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
    #[serde(rename = "exportPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<AgentImportSourceRefRequestExportPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub role: String,
    #[serde(rename = "storageRef")]
    #[serde(default)]
    pub storage_ref: String,
}

impl AgentImportSourceRefRequest {
    pub fn builder() -> AgentImportSourceRefRequestBuilder {
        <AgentImportSourceRefRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentImportSourceRefRequestBuilder {
    content_digest: Option<String>,
    export_policy: Option<AgentImportSourceRefRequestExportPolicy>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    role: Option<String>,
    storage_ref: Option<String>,
}

impl AgentImportSourceRefRequestBuilder {
    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    pub fn export_policy(mut self, value: AgentImportSourceRefRequestExportPolicy) -> Self {
        self.export_policy = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn storage_ref(mut self, value: impl Into<String>) -> Self {
        self.storage_ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentImportSourceRefRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_digest`](AgentImportSourceRefRequestBuilder::content_digest)
    /// - [`role`](AgentImportSourceRefRequestBuilder::role)
    /// - [`storage_ref`](AgentImportSourceRefRequestBuilder::storage_ref)
    pub fn build(self) -> Result<AgentImportSourceRefRequest, BuildError> {
        Ok(AgentImportSourceRefRequest {
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
            export_policy: self.export_policy,
            metadata: self.metadata,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            storage_ref: self.storage_ref.ok_or_else(|| BuildError::missing_field("storage_ref"))?,
        })
    }
}
