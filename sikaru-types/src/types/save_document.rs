pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SaveDocument {
    #[serde(default)]
    pub document: String,
    #[serde(rename = "expectedRevision")]
    #[serde(default)]
    pub expected_revision: i64,
}

impl SaveDocument {
    pub fn builder() -> SaveDocumentBuilder {
        <SaveDocumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SaveDocumentBuilder {
    document: Option<String>,
    expected_revision: Option<i64>,
}

impl SaveDocumentBuilder {
    pub fn document(mut self, value: impl Into<String>) -> Self {
        self.document = Some(value.into());
        self
    }

    pub fn expected_revision(mut self, value: i64) -> Self {
        self.expected_revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SaveDocument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document`](SaveDocumentBuilder::document)
    /// - [`expected_revision`](SaveDocumentBuilder::expected_revision)
    pub fn build(self) -> Result<SaveDocument, BuildError> {
        Ok(SaveDocument {
            document: self.document.ok_or_else(|| BuildError::missing_field("document"))?,
            expected_revision: self.expected_revision.ok_or_else(|| BuildError::missing_field("expected_revision"))?,
        })
    }
}

