pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DiscardDocument {
    #[serde(rename = "expectedLiveVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_live_version_id: Option<String>,
    #[serde(rename = "expectedRevision")]
    #[serde(default)]
    pub expected_revision: i64,
}

impl DiscardDocument {
    pub fn builder() -> DiscardDocumentBuilder {
        <DiscardDocumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DiscardDocumentBuilder {
    expected_live_version_id: Option<String>,
    expected_revision: Option<i64>,
}

impl DiscardDocumentBuilder {
    pub fn expected_live_version_id(mut self, value: impl Into<String>) -> Self {
        self.expected_live_version_id = Some(value.into());
        self
    }

    pub fn expected_revision(mut self, value: i64) -> Self {
        self.expected_revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DiscardDocument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expected_revision`](DiscardDocumentBuilder::expected_revision)
    pub fn build(self) -> Result<DiscardDocument, BuildError> {
        Ok(DiscardDocument {
            expected_live_version_id: self.expected_live_version_id,
            expected_revision: self.expected_revision.ok_or_else(|| BuildError::missing_field("expected_revision"))?,
        })
    }
}

