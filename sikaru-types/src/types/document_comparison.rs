pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentComparison {
    #[serde(default)]
    pub diff: String,
    #[serde(default)]
    pub draft: DocumentRevision,
    #[serde(rename = "liveDocument")]
    #[serde(default)]
    pub live_document: String,
    #[serde(rename = "liveVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_version_id: Option<String>,
}

impl DocumentComparison {
    pub fn builder() -> DocumentComparisonBuilder {
        <DocumentComparisonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentComparisonBuilder {
    diff: Option<String>,
    draft: Option<DocumentRevision>,
    live_document: Option<String>,
    live_version_id: Option<String>,
}

impl DocumentComparisonBuilder {
    pub fn diff(mut self, value: impl Into<String>) -> Self {
        self.diff = Some(value.into());
        self
    }

    pub fn draft(mut self, value: DocumentRevision) -> Self {
        self.draft = Some(value);
        self
    }

    pub fn live_document(mut self, value: impl Into<String>) -> Self {
        self.live_document = Some(value.into());
        self
    }

    pub fn live_version_id(mut self, value: impl Into<String>) -> Self {
        self.live_version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentComparison`].
    /// This method will fail if any of the following fields are not set:
    /// - [`diff`](DocumentComparisonBuilder::diff)
    /// - [`draft`](DocumentComparisonBuilder::draft)
    /// - [`live_document`](DocumentComparisonBuilder::live_document)
    pub fn build(self) -> Result<DocumentComparison, BuildError> {
        Ok(DocumentComparison {
            diff: self.diff.ok_or_else(|| BuildError::missing_field("diff"))?,
            draft: self.draft.ok_or_else(|| BuildError::missing_field("draft"))?,
            live_document: self.live_document.ok_or_else(|| BuildError::missing_field("live_document"))?,
            live_version_id: self.live_version_id,
        })
    }
}
