pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSandboxExecutionRequest {
    #[serde(rename = "contextPackageRef")]
    #[serde(default)]
    pub context_package_ref: String,
    #[serde(rename = "idempotencyKey")]
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(rename = "timeoutSeconds")]
    #[serde(default)]
    pub timeout_seconds: i64,
    #[serde(rename = "workflowRef")]
    #[serde(default)]
    pub workflow_ref: String,
}

impl CreateSandboxExecutionRequest {
    pub fn builder() -> CreateSandboxExecutionRequestBuilder {
        <CreateSandboxExecutionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSandboxExecutionRequestBuilder {
    context_package_ref: Option<String>,
    idempotency_key: Option<String>,
    timeout_seconds: Option<i64>,
    workflow_ref: Option<String>,
}

impl CreateSandboxExecutionRequestBuilder {
    pub fn context_package_ref(mut self, value: impl Into<String>) -> Self {
        self.context_package_ref = Some(value.into());
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn timeout_seconds(mut self, value: i64) -> Self {
        self.timeout_seconds = Some(value);
        self
    }

    pub fn workflow_ref(mut self, value: impl Into<String>) -> Self {
        self.workflow_ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSandboxExecutionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`context_package_ref`](CreateSandboxExecutionRequestBuilder::context_package_ref)
    /// - [`idempotency_key`](CreateSandboxExecutionRequestBuilder::idempotency_key)
    /// - [`timeout_seconds`](CreateSandboxExecutionRequestBuilder::timeout_seconds)
    /// - [`workflow_ref`](CreateSandboxExecutionRequestBuilder::workflow_ref)
    pub fn build(self) -> Result<CreateSandboxExecutionRequest, BuildError> {
        Ok(CreateSandboxExecutionRequest {
            context_package_ref: self.context_package_ref.ok_or_else(|| BuildError::missing_field("context_package_ref"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            timeout_seconds: self.timeout_seconds.ok_or_else(|| BuildError::missing_field("timeout_seconds"))?,
            workflow_ref: self.workflow_ref.ok_or_else(|| BuildError::missing_field("workflow_ref"))?,
        })
    }
}

