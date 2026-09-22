pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitToolResultRequest {
    #[serde(default)]
    pub capability_name: String,
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(default)]
    pub payload: HashMap<String, serde_json::Value>,
    pub status: SubmitToolResultRequestStatus,
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub tool_provider_id: String,
}

impl SubmitToolResultRequest {
    pub fn builder() -> SubmitToolResultRequestBuilder {
        <SubmitToolResultRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitToolResultRequestBuilder {
    capability_name: Option<String>,
    idempotency_key: Option<String>,
    payload: Option<HashMap<String, serde_json::Value>>,
    status: Option<SubmitToolResultRequestStatus>,
    tool_call_id: Option<String>,
    tool_provider_id: Option<String>,
}

impl SubmitToolResultRequestBuilder {
    pub fn capability_name(mut self, value: impl Into<String>) -> Self {
        self.capability_name = Some(value.into());
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn payload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn status(mut self, value: SubmitToolResultRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn tool_provider_id(mut self, value: impl Into<String>) -> Self {
        self.tool_provider_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubmitToolResultRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capability_name`](SubmitToolResultRequestBuilder::capability_name)
    /// - [`idempotency_key`](SubmitToolResultRequestBuilder::idempotency_key)
    /// - [`payload`](SubmitToolResultRequestBuilder::payload)
    /// - [`status`](SubmitToolResultRequestBuilder::status)
    /// - [`tool_call_id`](SubmitToolResultRequestBuilder::tool_call_id)
    /// - [`tool_provider_id`](SubmitToolResultRequestBuilder::tool_provider_id)
    pub fn build(self) -> Result<SubmitToolResultRequest, BuildError> {
        Ok(SubmitToolResultRequest {
            capability_name: self.capability_name.ok_or_else(|| BuildError::missing_field("capability_name"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            payload: self.payload.ok_or_else(|| BuildError::missing_field("payload"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool_provider_id: self.tool_provider_id.ok_or_else(|| BuildError::missing_field("tool_provider_id"))?,
        })
    }
}

