pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PersonalChannelFiles {
    #[serde(default)]
    pub files: Vec<PersonalChannelFile>,
}

impl PersonalChannelFiles {
    pub fn builder() -> PersonalChannelFilesBuilder {
        <PersonalChannelFilesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonalChannelFilesBuilder {
    files: Option<Vec<PersonalChannelFile>>,
}

impl PersonalChannelFilesBuilder {
    pub fn files(mut self, value: Vec<PersonalChannelFile>) -> Self {
        self.files = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PersonalChannelFiles`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](PersonalChannelFilesBuilder::files)
    pub fn build(self) -> Result<PersonalChannelFiles, BuildError> {
        Ok(PersonalChannelFiles {
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
        })
    }
}
