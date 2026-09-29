pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckResponse {
    pub check: Check,
}

impl CheckResponse {
    pub fn builder() -> CheckResponseBuilder {
        <CheckResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckResponseBuilder {
    check: Option<Check>,
}

impl CheckResponseBuilder {
    pub fn check(mut self, value: Check) -> Self {
        self.check = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`check`](CheckResponseBuilder::check)
    pub fn build(self) -> Result<CheckResponse, BuildError> {
        Ok(CheckResponse {
            check: self.check.ok_or_else(|| BuildError::missing_field("check"))?,
        })
    }
}
