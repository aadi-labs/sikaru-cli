pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Binding {
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_kind: Option<String>,
    #[serde(default)]
    pub generation: i64,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_app_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_generation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_cap: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_minute: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_access: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_tenant_id: Option<String>,
    #[serde(default)]
    pub status: String,
    pub transport: BindingTransport,
}

impl Binding {
    pub fn builder() -> BindingBuilder {
        <BindingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BindingBuilder {
    agent_id: Option<String>,
    destination_id: Option<String>,
    destination_kind: Option<String>,
    generation: Option<i64>,
    id: Option<String>,
    identity_app_ids: Option<Vec<String>>,
    installation_generation: Option<i64>,
    installation_id: Option<String>,
    name: Option<String>,
    pending_cap: Option<i64>,
    per_minute: Option<i64>,
    personal_access: Option<bool>,
    recipient_id: Option<String>,
    shared_tenant_id: Option<String>,
    status: Option<String>,
    transport: Option<BindingTransport>,
}

impl BindingBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn destination_id(mut self, value: impl Into<String>) -> Self {
        self.destination_id = Some(value.into());
        self
    }

    pub fn destination_kind(mut self, value: impl Into<String>) -> Self {
        self.destination_kind = Some(value.into());
        self
    }

    pub fn generation(mut self, value: i64) -> Self {
        self.generation = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn identity_app_ids(mut self, value: Vec<String>) -> Self {
        self.identity_app_ids = Some(value);
        self
    }

    pub fn installation_generation(mut self, value: i64) -> Self {
        self.installation_generation = Some(value);
        self
    }

    pub fn installation_id(mut self, value: impl Into<String>) -> Self {
        self.installation_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn pending_cap(mut self, value: i64) -> Self {
        self.pending_cap = Some(value);
        self
    }

    pub fn per_minute(mut self, value: i64) -> Self {
        self.per_minute = Some(value);
        self
    }

    pub fn personal_access(mut self, value: bool) -> Self {
        self.personal_access = Some(value);
        self
    }

    pub fn recipient_id(mut self, value: impl Into<String>) -> Self {
        self.recipient_id = Some(value.into());
        self
    }

    pub fn shared_tenant_id(mut self, value: impl Into<String>) -> Self {
        self.shared_tenant_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn transport(mut self, value: BindingTransport) -> Self {
        self.transport = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Binding`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](BindingBuilder::agent_id)
    /// - [`generation`](BindingBuilder::generation)
    /// - [`id`](BindingBuilder::id)
    /// - [`status`](BindingBuilder::status)
    /// - [`transport`](BindingBuilder::transport)
    pub fn build(self) -> Result<Binding, BuildError> {
        Ok(Binding {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            destination_id: self.destination_id,
            destination_kind: self.destination_kind,
            generation: self.generation.ok_or_else(|| BuildError::missing_field("generation"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            identity_app_ids: self.identity_app_ids,
            installation_generation: self.installation_generation,
            installation_id: self.installation_id,
            name: self.name,
            pending_cap: self.pending_cap,
            per_minute: self.per_minute,
            personal_access: self.personal_access,
            recipient_id: self.recipient_id,
            shared_tenant_id: self.shared_tenant_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            transport: self.transport.ok_or_else(|| BuildError::missing_field("transport"))?,
        })
    }
}
