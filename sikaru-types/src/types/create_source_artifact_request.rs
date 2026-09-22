pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateSourceArtifactRequest {
    #[serde(rename = "analysisSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_summary: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
    #[serde(rename = "exportPolicy")]
    pub export_policy: CreateSourceArtifactRequestExportPolicy,
    #[serde(rename = "sourceKind")]
    pub source_kind: CreateSourceArtifactRequestSourceKind,
    #[serde(rename = "storageRef")]
    #[serde(default)]
    pub storage_ref: String,
}

impl CreateSourceArtifactRequest {
    pub fn builder() -> CreateSourceArtifactRequestBuilder {
        <CreateSourceArtifactRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSourceArtifactRequestBuilder {
    analysis_summary: Option<HashMap<String, serde_json::Value>>,
    content_digest: Option<String>,
    export_policy: Option<CreateSourceArtifactRequestExportPolicy>,
    source_kind: Option<CreateSourceArtifactRequestSourceKind>,
    storage_ref: Option<String>,
}

impl CreateSourceArtifactRequestBuilder {
    pub fn analysis_summary(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.analysis_summary = Some(value);
        self
    }

    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    pub fn export_policy(mut self, value: CreateSourceArtifactRequestExportPolicy) -> Self {
        self.export_policy = Some(value);
        self
    }

    pub fn source_kind(mut self, value: CreateSourceArtifactRequestSourceKind) -> Self {
        self.source_kind = Some(value);
        self
    }

    pub fn storage_ref(mut self, value: impl Into<String>) -> Self {
        self.storage_ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSourceArtifactRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_digest`](CreateSourceArtifactRequestBuilder::content_digest)
    /// - [`export_policy`](CreateSourceArtifactRequestBuilder::export_policy)
    /// - [`source_kind`](CreateSourceArtifactRequestBuilder::source_kind)
    /// - [`storage_ref`](CreateSourceArtifactRequestBuilder::storage_ref)
    pub fn build(self) -> Result<CreateSourceArtifactRequest, BuildError> {
        Ok(CreateSourceArtifactRequest {
            analysis_summary: self.analysis_summary,
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
            export_policy: self.export_policy.ok_or_else(|| BuildError::missing_field("export_policy"))?,
            source_kind: self.source_kind.ok_or_else(|| BuildError::missing_field("source_kind"))?,
            storage_ref: self.storage_ref.ok_or_else(|| BuildError::missing_field("storage_ref"))?,
        })
    }
}

