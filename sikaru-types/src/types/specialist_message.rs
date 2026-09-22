pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SpecialistMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_mode: Option<SpecialistMessageDeliveryMode>,
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(default)]
    pub input: HashMap<String, serde_json::Value>,
}

impl SpecialistMessage {
    pub fn builder() -> SpecialistMessageBuilder {
        <SpecialistMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpecialistMessageBuilder {
    delivery_mode: Option<SpecialistMessageDeliveryMode>,
    idempotency_key: Option<String>,
    input: Option<HashMap<String, serde_json::Value>>,
}

impl SpecialistMessageBuilder {
    pub fn delivery_mode(mut self, value: SpecialistMessageDeliveryMode) -> Self {
        self.delivery_mode = Some(value);
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

    /// Consumes the builder and constructs a [`SpecialistMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](SpecialistMessageBuilder::idempotency_key)
    /// - [`input`](SpecialistMessageBuilder::input)
    pub fn build(self) -> Result<SpecialistMessage, BuildError> {
        Ok(SpecialistMessage {
            delivery_mode: self.delivery_mode,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            input: self.input.ok_or_else(|| BuildError::missing_field("input"))?,
        })
    }
}

