pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DatasetResponse {
    pub dataset: Dataset,
}

impl DatasetResponse {
    pub fn builder() -> DatasetResponseBuilder {
        <DatasetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetResponseBuilder {
    dataset: Option<Dataset>,
}

impl DatasetResponseBuilder {
    pub fn dataset(mut self, value: Dataset) -> Self {
        self.dataset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dataset`](DatasetResponseBuilder::dataset)
    pub fn build(self) -> Result<DatasetResponse, BuildError> {
        Ok(DatasetResponse {
            dataset: self.dataset.ok_or_else(|| BuildError::missing_field("dataset"))?,
        })
    }
}
