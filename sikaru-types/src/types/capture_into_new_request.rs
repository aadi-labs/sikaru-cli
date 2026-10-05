pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CaptureIntoNewRequest {
    #[serde(default)]
    pub dataset: NewDataset,
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(default)]
    pub items: Vec<CaptureItem>,
}

impl CaptureIntoNewRequest {
    pub fn builder() -> CaptureIntoNewRequestBuilder {
        <CaptureIntoNewRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureIntoNewRequestBuilder {
    dataset: Option<NewDataset>,
    idempotency_key: Option<String>,
    items: Option<Vec<CaptureItem>>,
}

impl CaptureIntoNewRequestBuilder {
    pub fn dataset(mut self, value: NewDataset) -> Self {
        self.dataset = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<CaptureItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureIntoNewRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dataset`](CaptureIntoNewRequestBuilder::dataset)
    /// - [`idempotency_key`](CaptureIntoNewRequestBuilder::idempotency_key)
    /// - [`items`](CaptureIntoNewRequestBuilder::items)
    pub fn build(self) -> Result<CaptureIntoNewRequest, BuildError> {
        Ok(CaptureIntoNewRequest {
            dataset: self.dataset.ok_or_else(|| BuildError::missing_field("dataset"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}

