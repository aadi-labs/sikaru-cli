pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PersonalChannelAction {
    #[serde(default)]
    pub approval: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(default)]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_provider_id: Option<String>,
}

impl PersonalChannelAction {
    pub fn builder() -> PersonalChannelActionBuilder {
        <PersonalChannelActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonalChannelActionBuilder {
    approval: Option<String>,
    arguments: Option<serde_json::Value>,
    capability_name: Option<String>,
    execution_owner: Option<String>,
    idempotency_key: Option<String>,
    input: Option<serde_json::Value>,
    status: Option<String>,
    tool_call_id: Option<String>,
    tool_provider_id: Option<String>,
}

impl PersonalChannelActionBuilder {
    pub fn approval(mut self, value: impl Into<String>) -> Self {
        self.approval = Some(value.into());
        self
    }

    pub fn arguments(mut self, value: serde_json::Value) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn capability_name(mut self, value: impl Into<String>) -> Self {
        self.capability_name = Some(value.into());
        self
    }

    pub fn execution_owner(mut self, value: impl Into<String>) -> Self {
        self.execution_owner = Some(value.into());
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn input(mut self, value: serde_json::Value) -> Self {
        self.input = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
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

    /// Consumes the builder and constructs a [`PersonalChannelAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`approval`](PersonalChannelActionBuilder::approval)
    /// - [`status`](PersonalChannelActionBuilder::status)
    pub fn build(self) -> Result<PersonalChannelAction, BuildError> {
        Ok(PersonalChannelAction {
            approval: self.approval.ok_or_else(|| BuildError::missing_field("approval"))?,
            arguments: self.arguments,
            capability_name: self.capability_name,
            execution_owner: self.execution_owner,
            idempotency_key: self.idempotency_key,
            input: self.input,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            tool_call_id: self.tool_call_id,
            tool_provider_id: self.tool_provider_id,
        })
    }
}
