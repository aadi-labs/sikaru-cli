pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for policy
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PolicyQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

impl PolicyQueryRequest {
    pub fn builder() -> PolicyQueryRequestBuilder {
        <PolicyQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PolicyQueryRequestBuilder {
    run_id: Option<String>,
}

impl PolicyQueryRequestBuilder {
    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PolicyQueryRequest`].
    pub fn build(self) -> Result<PolicyQueryRequest, BuildError> {
        Ok(PolicyQueryRequest {
            run_id: self.run_id,
        })
    }
}

