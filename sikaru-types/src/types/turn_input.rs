pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TurnInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_grants: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_attachment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<TurnInputDeliveryMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(default)]
    pub input: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_context: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_mode: Option<TurnInputRunMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_provider_refs: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl TurnInput {
    pub fn builder() -> TurnInputBuilder {
        <TurnInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TurnInputBuilder {
    capability_grants: Option<Vec<String>>,
    compute_attachment_id: Option<String>,
    delivery_mode: Option<TurnInputDeliveryMode>,
    file_ids: Option<Vec<String>>,
    idempotency_key: Option<String>,
    input: Option<HashMap<String, serde_json::Value>>,
    product_context: Option<HashMap<String, serde_json::Value>>,
    run_mode: Option<TurnInputRunMode>,
    tool_provider_refs: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl TurnInputBuilder {
    pub fn capability_grants(mut self, value: Vec<String>) -> Self {
        self.capability_grants = Some(value);
        self
    }

    pub fn compute_attachment_id(mut self, value: impl Into<String>) -> Self {
        self.compute_attachment_id = Some(value.into());
        self
    }

    pub fn delivery_mode(mut self, value: TurnInputDeliveryMode) -> Self {
        self.delivery_mode = Some(value);
        self
    }

    pub fn file_ids(mut self, value: Vec<String>) -> Self {
        self.file_ids = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn input(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input = Some(value);
        self
    }

    pub fn product_context(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.product_context = Some(value);
        self
    }

    pub fn run_mode(mut self, value: TurnInputRunMode) -> Self {
        self.run_mode = Some(value);
        self
    }

    pub fn tool_provider_refs(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.tool_provider_refs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TurnInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](TurnInputBuilder::idempotency_key)
    /// - [`input`](TurnInputBuilder::input)
    pub fn build(self) -> Result<TurnInput, BuildError> {
        Ok(TurnInput {
            capability_grants: self.capability_grants,
            compute_attachment_id: self.compute_attachment_id,
            delivery_mode: self.delivery_mode,
            file_ids: self.file_ids,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            input: self.input.ok_or_else(|| BuildError::missing_field("input"))?,
            product_context: self.product_context,
            run_mode: self.run_mode,
            tool_provider_refs: self.tool_provider_refs,
        })
    }
}

