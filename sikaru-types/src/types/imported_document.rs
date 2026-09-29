pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ImportedDocument {
    #[serde(default)]
    pub document: String,
    #[serde(default)]
    pub validation: DocumentValidationView,
}

impl ImportedDocument {
    pub fn builder() -> ImportedDocumentBuilder {
        <ImportedDocumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportedDocumentBuilder {
    document: Option<String>,
    validation: Option<DocumentValidationView>,
}

impl ImportedDocumentBuilder {
    pub fn document(mut self, value: impl Into<String>) -> Self {
        self.document = Some(value.into());
        self
    }

    pub fn validation(mut self, value: DocumentValidationView) -> Self {
        self.validation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportedDocument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document`](ImportedDocumentBuilder::document)
    /// - [`validation`](ImportedDocumentBuilder::validation)
    pub fn build(self) -> Result<ImportedDocument, BuildError> {
        Ok(ImportedDocument {
            document: self.document.ok_or_else(|| BuildError::missing_field("document"))?,
            validation: self.validation.ok_or_else(|| BuildError::missing_field("validation"))?,
        })
    }
}
