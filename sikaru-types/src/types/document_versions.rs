pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DocumentVersions {
    #[serde(default)]
    pub versions: Vec<DocumentPublication>,
}

impl DocumentVersions {
    pub fn builder() -> DocumentVersionsBuilder {
        <DocumentVersionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentVersionsBuilder {
    versions: Option<Vec<DocumentPublication>>,
}

impl DocumentVersionsBuilder {
    pub fn versions(mut self, value: Vec<DocumentPublication>) -> Self {
        self.versions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentVersions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`versions`](DocumentVersionsBuilder::versions)
    pub fn build(self) -> Result<DocumentVersions, BuildError> {
        Ok(DocumentVersions {
            versions: self.versions.ok_or_else(|| BuildError::missing_field("versions"))?,
        })
    }
}
