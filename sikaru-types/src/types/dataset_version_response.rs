pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatasetVersionResponse {
    #[serde(default)]
    pub version: DatasetVersion,
}

impl DatasetVersionResponse {
    pub fn builder() -> DatasetVersionResponseBuilder {
        <DatasetVersionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetVersionResponseBuilder {
    version: Option<DatasetVersion>,
}

impl DatasetVersionResponseBuilder {
    pub fn version(mut self, value: DatasetVersion) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetVersionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](DatasetVersionResponseBuilder::version)
    pub fn build(self) -> Result<DatasetVersionResponse, BuildError> {
        Ok(DatasetVersionResponse {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
