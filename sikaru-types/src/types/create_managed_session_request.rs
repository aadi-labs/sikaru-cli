pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateManagedSessionRequest {
    #[serde(rename = "environmentId")]
    #[serde(default)]
    pub environment_id: String,
    #[serde(rename = "externalRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_run_id: Option<String>,
    #[serde(rename = "externalThreadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_thread_id: Option<String>,
    #[serde(rename = "idempotencyKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
}

impl CreateManagedSessionRequest {
    pub fn builder() -> CreateManagedSessionRequestBuilder {
        <CreateManagedSessionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateManagedSessionRequestBuilder {
    environment_id: Option<String>,
    external_run_id: Option<String>,
    external_thread_id: Option<String>,
    idempotency_key: Option<String>,
}

impl CreateManagedSessionRequestBuilder {
    pub fn environment_id(mut self, value: impl Into<String>) -> Self {
        self.environment_id = Some(value.into());
        self
    }

    pub fn external_run_id(mut self, value: impl Into<String>) -> Self {
        self.external_run_id = Some(value.into());
        self
    }

    pub fn external_thread_id(mut self, value: impl Into<String>) -> Self {
        self.external_thread_id = Some(value.into());
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateManagedSessionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment_id`](CreateManagedSessionRequestBuilder::environment_id)
    pub fn build(self) -> Result<CreateManagedSessionRequest, BuildError> {
        Ok(CreateManagedSessionRequest {
            environment_id: self.environment_id.ok_or_else(|| BuildError::missing_field("environment_id"))?,
            external_run_id: self.external_run_id,
            external_thread_id: self.external_thread_id,
            idempotency_key: self.idempotency_key,
        })
    }
}

