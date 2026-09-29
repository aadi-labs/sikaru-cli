pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckResultResponse {
    pub result: CheckResult,
}

impl CheckResultResponse {
    pub fn builder() -> CheckResultResponseBuilder {
        <CheckResultResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckResultResponseBuilder {
    result: Option<CheckResult>,
}

impl CheckResultResponseBuilder {
    pub fn result(mut self, value: CheckResult) -> Self {
        self.result = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckResultResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`result`](CheckResultResponseBuilder::result)
    pub fn build(self) -> Result<CheckResultResponse, BuildError> {
        Ok(CheckResultResponse {
            result: self.result.ok_or_else(|| BuildError::missing_field("result"))?,
        })
    }
}
