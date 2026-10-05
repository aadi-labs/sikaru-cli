pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatasetVersionList {
    #[serde(default)]
    pub versions: Vec<DatasetVersion>,
}

impl DatasetVersionList {
    pub fn builder() -> DatasetVersionListBuilder {
        <DatasetVersionListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetVersionListBuilder {
    versions: Option<Vec<DatasetVersion>>,
}

impl DatasetVersionListBuilder {
    pub fn versions(mut self, value: Vec<DatasetVersion>) -> Self {
        self.versions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetVersionList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`versions`](DatasetVersionListBuilder::versions)
    pub fn build(self) -> Result<DatasetVersionList, BuildError> {
        Ok(DatasetVersionList {
            versions: self.versions.ok_or_else(|| BuildError::missing_field("versions"))?,
        })
    }
}
