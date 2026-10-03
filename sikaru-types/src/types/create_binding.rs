pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateBinding {
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_kind: Option<CreateBindingDestinationKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
    pub transport: CreateBindingTransport,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_id: Option<String>,
}

impl CreateBinding {
    pub fn builder() -> CreateBindingBuilder {
        <CreateBindingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBindingBuilder {
    agent_id: Option<String>,
    destination_id: Option<String>,
    destination_kind: Option<CreateBindingDestinationKind>,
    installation_id: Option<String>,
    recipient_id: Option<String>,
    transport: Option<CreateBindingTransport>,
    verification_id: Option<String>,
}

impl CreateBindingBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn destination_id(mut self, value: impl Into<String>) -> Self {
        self.destination_id = Some(value.into());
        self
    }

    pub fn destination_kind(mut self, value: CreateBindingDestinationKind) -> Self {
        self.destination_kind = Some(value);
        self
    }

    pub fn installation_id(mut self, value: impl Into<String>) -> Self {
        self.installation_id = Some(value.into());
        self
    }

    pub fn recipient_id(mut self, value: impl Into<String>) -> Self {
        self.recipient_id = Some(value.into());
        self
    }

    pub fn transport(mut self, value: CreateBindingTransport) -> Self {
        self.transport = Some(value);
        self
    }

    pub fn verification_id(mut self, value: impl Into<String>) -> Self {
        self.verification_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateBinding`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](CreateBindingBuilder::agent_id)
    /// - [`transport`](CreateBindingBuilder::transport)
    pub fn build(self) -> Result<CreateBinding, BuildError> {
        Ok(CreateBinding {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            destination_id: self.destination_id,
            destination_kind: self.destination_kind,
            installation_id: self.installation_id,
            recipient_id: self.recipient_id,
            transport: self.transport.ok_or_else(|| BuildError::missing_field("transport"))?,
            verification_id: self.verification_id,
        })
    }
}

