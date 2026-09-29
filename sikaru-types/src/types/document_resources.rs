pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DocumentResources {
    #[serde(default)]
    pub resources: Vec<MentionResource>,
}

impl DocumentResources {
    pub fn builder() -> DocumentResourcesBuilder {
        <DocumentResourcesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentResourcesBuilder {
    resources: Option<Vec<MentionResource>>,
}

impl DocumentResourcesBuilder {
    pub fn resources(mut self, value: Vec<MentionResource>) -> Self {
        self.resources = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentResources`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resources`](DocumentResourcesBuilder::resources)
    pub fn build(self) -> Result<DocumentResources, BuildError> {
        Ok(DocumentResources {
            resources: self.resources.ok_or_else(|| BuildError::missing_field("resources"))?,
        })
    }
}
