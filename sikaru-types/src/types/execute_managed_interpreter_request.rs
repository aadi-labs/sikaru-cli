pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExecuteManagedInterpreterRequest {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "timeoutSeconds")]
    #[serde(default)]
    pub timeout_seconds: i64,
}

impl ExecuteManagedInterpreterRequest {
    pub fn builder() -> ExecuteManagedInterpreterRequestBuilder {
        <ExecuteManagedInterpreterRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecuteManagedInterpreterRequestBuilder {
    code: Option<String>,
    timeout_seconds: Option<i64>,
}

impl ExecuteManagedInterpreterRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn timeout_seconds(mut self, value: i64) -> Self {
        self.timeout_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExecuteManagedInterpreterRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ExecuteManagedInterpreterRequestBuilder::code)
    /// - [`timeout_seconds`](ExecuteManagedInterpreterRequestBuilder::timeout_seconds)
    pub fn build(self) -> Result<ExecuteManagedInterpreterRequest, BuildError> {
        Ok(ExecuteManagedInterpreterRequest {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            timeout_seconds: self.timeout_seconds.ok_or_else(|| BuildError::missing_field("timeout_seconds"))?,
        })
    }
}

