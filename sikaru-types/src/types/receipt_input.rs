pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReceiptInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_name: Option<ReceiptInputCapabilityName>,
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(default)]
    pub payload: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub request_digest: String,
    #[serde(default)]
    pub run_id: String,
    pub status: ReceiptInputStatus,
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub tool_provider_id: String,
}

impl ReceiptInput {
    pub fn builder() -> ReceiptInputBuilder {
        <ReceiptInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptInputBuilder {
    capability_name: Option<ReceiptInputCapabilityName>,
    idempotency_key: Option<String>,
    payload: Option<HashMap<String, serde_json::Value>>,
    request_digest: Option<String>,
    run_id: Option<String>,
    status: Option<ReceiptInputStatus>,
    tool_call_id: Option<String>,
    tool_provider_id: Option<String>,
}

impl ReceiptInputBuilder {
    pub fn capability_name(mut self, value: ReceiptInputCapabilityName) -> Self {
        self.capability_name = Some(value);
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

    pub fn request_digest(mut self, value: impl Into<String>) -> Self {
        self.request_digest = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ReceiptInputStatus) -> Self {
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

    /// Consumes the builder and constructs a [`ReceiptInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](ReceiptInputBuilder::idempotency_key)
    /// - [`payload`](ReceiptInputBuilder::payload)
    /// - [`request_digest`](ReceiptInputBuilder::request_digest)
    /// - [`run_id`](ReceiptInputBuilder::run_id)
    /// - [`status`](ReceiptInputBuilder::status)
    /// - [`tool_call_id`](ReceiptInputBuilder::tool_call_id)
    /// - [`tool_provider_id`](ReceiptInputBuilder::tool_provider_id)
    pub fn build(self) -> Result<ReceiptInput, BuildError> {
        Ok(ReceiptInput {
            capability_name: self.capability_name,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            payload: self.payload.ok_or_else(|| BuildError::missing_field("payload"))?,
            request_digest: self.request_digest.ok_or_else(|| BuildError::missing_field("request_digest"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool_provider_id: self.tool_provider_id.ok_or_else(|| BuildError::missing_field("tool_provider_id"))?,
        })
    }
}
