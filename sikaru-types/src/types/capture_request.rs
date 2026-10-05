pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CaptureRequest {
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(default)]
    pub items: Vec<CaptureItem>,
}

impl CaptureRequest {
    pub fn builder() -> CaptureRequestBuilder {
        <CaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureRequestBuilder {
    idempotency_key: Option<String>,
    items: Option<Vec<CaptureItem>>,
}

impl CaptureRequestBuilder {
    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn items(mut self, value: Vec<CaptureItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](CaptureRequestBuilder::idempotency_key)
    /// - [`items`](CaptureRequestBuilder::items)
    pub fn build(self) -> Result<CaptureRequest, BuildError> {
        Ok(CaptureRequest {
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}

