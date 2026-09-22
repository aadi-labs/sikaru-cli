pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateImportSessionRequest {
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    pub mode: CreateImportSessionRequestMode,
}

impl CreateImportSessionRequest {
    pub fn builder() -> CreateImportSessionRequestBuilder {
        <CreateImportSessionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateImportSessionRequestBuilder {
    display_name: Option<String>,
    mode: Option<CreateImportSessionRequestMode>,
}

impl CreateImportSessionRequestBuilder {
    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn mode(mut self, value: CreateImportSessionRequestMode) -> Self {
        self.mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateImportSessionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display_name`](CreateImportSessionRequestBuilder::display_name)
    /// - [`mode`](CreateImportSessionRequestBuilder::mode)
    pub fn build(self) -> Result<CreateImportSessionRequest, BuildError> {
        Ok(CreateImportSessionRequest {
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}

