pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ImportDocumentFiles {
    #[serde(default)]
    pub files: HashMap<String, String>,
}

impl ImportDocumentFiles {
    pub fn builder() -> ImportDocumentFilesBuilder {
        <ImportDocumentFilesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportDocumentFilesBuilder {
    files: Option<HashMap<String, String>>,
}

impl ImportDocumentFilesBuilder {
    pub fn files(mut self, value: HashMap<String, String>) -> Self {
        self.files = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImportDocumentFiles`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](ImportDocumentFilesBuilder::files)
    pub fn build(self) -> Result<ImportDocumentFiles, BuildError> {
        Ok(ImportDocumentFiles {
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
        })
    }
}

