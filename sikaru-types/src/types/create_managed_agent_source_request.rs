pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateManagedAgentSourceRequest {
    #[serde(rename = "analysisSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_summary: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "exportPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_policy: Option<CreateManagedAgentSourceRequestExportPolicy>,
    #[serde(rename = "sourceKind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_kind: Option<CreateManagedAgentSourceRequestSourceKind>,
    #[serde(rename = "storageRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_ref: Option<String>,
}

impl CreateManagedAgentSourceRequest {
    pub fn builder() -> CreateManagedAgentSourceRequestBuilder {
        <CreateManagedAgentSourceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateManagedAgentSourceRequestBuilder {
    analysis_summary: Option<HashMap<String, serde_json::Value>>,
    content_digest: Option<String>,
    definition: Option<HashMap<String, serde_json::Value>>,
    export_policy: Option<CreateManagedAgentSourceRequestExportPolicy>,
    source_kind: Option<CreateManagedAgentSourceRequestSourceKind>,
    storage_ref: Option<String>,
}

impl CreateManagedAgentSourceRequestBuilder {
    pub fn analysis_summary(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.analysis_summary = Some(value);
        self
    }

    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    pub fn definition(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.definition = Some(value);
        self
    }

    pub fn export_policy(mut self, value: CreateManagedAgentSourceRequestExportPolicy) -> Self {
        self.export_policy = Some(value);
        self
    }

    pub fn source_kind(mut self, value: CreateManagedAgentSourceRequestSourceKind) -> Self {
        self.source_kind = Some(value);
        self
    }

    pub fn storage_ref(mut self, value: impl Into<String>) -> Self {
        self.storage_ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateManagedAgentSourceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_digest`](CreateManagedAgentSourceRequestBuilder::content_digest)
    pub fn build(self) -> Result<CreateManagedAgentSourceRequest, BuildError> {
        Ok(CreateManagedAgentSourceRequest {
            analysis_summary: self.analysis_summary,
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
            definition: self.definition,
            export_policy: self.export_policy,
            source_kind: self.source_kind,
            storage_ref: self.storage_ref,
        })
    }
}
