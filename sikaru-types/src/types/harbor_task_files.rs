pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HarborTaskFiles {
    /// A Harbor task directory as relative path to text: instruction.md, optional task.toml, and tests/test.sh (writes /logs/verifier/reward.txt) or tests/rubric.toml.
    #[serde(default)]
    pub files: HashMap<String, String>,
}

impl HarborTaskFiles {
    pub fn builder() -> HarborTaskFilesBuilder {
        <HarborTaskFilesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HarborTaskFilesBuilder {
    files: Option<HashMap<String, String>>,
}

impl HarborTaskFilesBuilder {
    pub fn files(mut self, value: HashMap<String, String>) -> Self {
        self.files = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HarborTaskFiles`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](HarborTaskFilesBuilder::files)
    pub fn build(self) -> Result<HarborTaskFiles, BuildError> {
        Ok(HarborTaskFiles {
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
        })
    }
}
