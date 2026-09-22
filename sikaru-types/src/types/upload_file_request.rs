pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UploadFileRequest {
    #[serde(skip)]
    #[serde(default)]
    pub body: Vec<u8>,
    #[serde(skip)]
    #[serde(default)]
    pub filename: String,
}

impl UploadFileRequest {
    pub fn builder() -> UploadFileRequestBuilder {
        <UploadFileRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UploadFileRequestBuilder {
    body: Option<Vec<u8>>,
    filename: Option<String>,
}

impl UploadFileRequestBuilder {
    pub fn body(mut self, value: Vec<u8>) -> Self {
        self.body = Some(value);
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UploadFileRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](UploadFileRequestBuilder::body)
    /// - [`filename`](UploadFileRequestBuilder::filename)
    pub fn build(self) -> Result<UploadFileRequest, BuildError> {
        Ok(UploadFileRequest {
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
            filename: self.filename.ok_or_else(|| BuildError::missing_field("filename"))?,
        })
    }
}

