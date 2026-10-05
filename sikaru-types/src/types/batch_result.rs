pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BatchResult {
    #[serde(default)]
    pub counts: BatchCounts,
    pub dataset: Dataset,
    #[serde(default)]
    pub items: Vec<BatchItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<DatasetVersion>,
}

impl BatchResult {
    pub fn builder() -> BatchResultBuilder {
        <BatchResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchResultBuilder {
    counts: Option<BatchCounts>,
    dataset: Option<Dataset>,
    items: Option<Vec<BatchItem>>,
    version: Option<DatasetVersion>,
}

impl BatchResultBuilder {
    pub fn counts(mut self, value: BatchCounts) -> Self {
        self.counts = Some(value);
        self
    }

    pub fn dataset(mut self, value: Dataset) -> Self {
        self.dataset = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<BatchItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn version(mut self, value: DatasetVersion) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`counts`](BatchResultBuilder::counts)
    /// - [`dataset`](BatchResultBuilder::dataset)
    /// - [`items`](BatchResultBuilder::items)
    pub fn build(self) -> Result<BatchResult, BuildError> {
        Ok(BatchResult {
            counts: self.counts.ok_or_else(|| BuildError::missing_field("counts"))?,
            dataset: self.dataset.ok_or_else(|| BuildError::missing_field("dataset"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            version: self.version,
        })
    }
}
