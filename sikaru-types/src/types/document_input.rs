pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentInput {
    #[serde(default)]
    pub document: String,
}

impl DocumentInput {
    pub fn builder() -> DocumentInputBuilder {
        <DocumentInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentInputBuilder {
    document: Option<String>,
}

impl DocumentInputBuilder {
    pub fn document(mut self, value: impl Into<String>) -> Self {
        self.document = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document`](DocumentInputBuilder::document)
    pub fn build(self) -> Result<DocumentInput, BuildError> {
        Ok(DocumentInput {
            document: self.document.ok_or_else(|| BuildError::missing_field("document"))?,
        })
    }
}

