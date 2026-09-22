pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ComputeError {
    #[serde(default)]
    pub detail: String,
}

impl ComputeError {
    pub fn builder() -> ComputeErrorBuilder {
        <ComputeErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComputeErrorBuilder {
    detail: Option<String>,
}

impl ComputeErrorBuilder {
    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ComputeError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`detail`](ComputeErrorBuilder::detail)
    pub fn build(self) -> Result<ComputeError, BuildError> {
        Ok(ComputeError {
            detail: self.detail.ok_or_else(|| BuildError::missing_field("detail"))?,
        })
    }
}
