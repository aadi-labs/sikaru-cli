pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UploadRequest {
    /// CSV with input, expected and tags columns, or JSONL in the export shape.
    #[serde(default)]
    pub content: String,
    pub format: UploadRequestFormat,
    #[serde(default)]
    pub idempotency_key: String,
}

impl UploadRequest {
    pub fn builder() -> UploadRequestBuilder {
        <UploadRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadRequestBuilder {
    content: Option<String>,
    format: Option<UploadRequestFormat>,
    idempotency_key: Option<String>,
}

impl UploadRequestBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn format(mut self, value: UploadRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UploadRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](UploadRequestBuilder::content)
    /// - [`format`](UploadRequestBuilder::format)
    /// - [`idempotency_key`](UploadRequestBuilder::idempotency_key)
    pub fn build(self) -> Result<UploadRequest, BuildError> {
        Ok(UploadRequest {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            format: self.format.ok_or_else(|| BuildError::missing_field("format"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
        })
    }
}

