pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for poll
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ComputeWorkersPollQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ComputeWorkersPollQueryRequest {
    pub fn builder() -> ComputeWorkersPollQueryRequestBuilder {
        <ComputeWorkersPollQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComputeWorkersPollQueryRequestBuilder {
    wait_seconds: Option<i64>,
    limit: Option<i64>,
}

impl ComputeWorkersPollQueryRequestBuilder {
    pub fn wait_seconds(mut self, value: i64) -> Self {
        self.wait_seconds = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ComputeWorkersPollQueryRequest`].
    pub fn build(self) -> Result<ComputeWorkersPollQueryRequest, BuildError> {
        Ok(ComputeWorkersPollQueryRequest {
            wait_seconds: self.wait_seconds,
            limit: self.limit,
        })
    }
}

