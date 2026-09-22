pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for execution_runtime_lineage
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExecutionRuntimeLineageQueryRequest {
    #[serde(default)]
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_after: Option<String>,
}

impl ExecutionRuntimeLineageQueryRequest {
    pub fn builder() -> ExecutionRuntimeLineageQueryRequestBuilder {
        <ExecutionRuntimeLineageQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionRuntimeLineageQueryRequestBuilder {
    account_id: Option<String>,
    inference_after: Option<String>,
}

impl ExecutionRuntimeLineageQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn inference_after(mut self, value: impl Into<String>) -> Self {
        self.inference_after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExecutionRuntimeLineageQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ExecutionRuntimeLineageQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<ExecutionRuntimeLineageQueryRequest, BuildError> {
        Ok(ExecutionRuntimeLineageQueryRequest {
            account_id: self.account_id.ok_or_else(|| BuildError::missing_field("account_id"))?,
            inference_after: self.inference_after,
        })
    }
}

