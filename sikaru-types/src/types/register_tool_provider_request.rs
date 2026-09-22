pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RegisterToolProviderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_policy_ref: Option<String>,
    #[serde(default)]
    pub broker_endpoint_ref: String,
    #[serde(default)]
    pub capability_prefix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_owner: Option<String>,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub provider_type: String,
    #[serde(default)]
    pub tool_catalog_ref: String,
}

impl RegisterToolProviderRequest {
    pub fn builder() -> RegisterToolProviderRequestBuilder {
        <RegisterToolProviderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RegisterToolProviderRequestBuilder {
    approval_policy_ref: Option<String>,
    broker_endpoint_ref: Option<String>,
    capability_prefix: Option<String>,
    credential_owner: Option<String>,
    display_name: Option<String>,
    provider_type: Option<String>,
    tool_catalog_ref: Option<String>,
}

impl RegisterToolProviderRequestBuilder {
    pub fn approval_policy_ref(mut self, value: impl Into<String>) -> Self {
        self.approval_policy_ref = Some(value.into());
        self
    }

    pub fn broker_endpoint_ref(mut self, value: impl Into<String>) -> Self {
        self.broker_endpoint_ref = Some(value.into());
        self
    }

    pub fn capability_prefix(mut self, value: impl Into<String>) -> Self {
        self.capability_prefix = Some(value.into());
        self
    }

    pub fn credential_owner(mut self, value: impl Into<String>) -> Self {
        self.credential_owner = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn provider_type(mut self, value: impl Into<String>) -> Self {
        self.provider_type = Some(value.into());
        self
    }

    pub fn tool_catalog_ref(mut self, value: impl Into<String>) -> Self {
        self.tool_catalog_ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RegisterToolProviderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`broker_endpoint_ref`](RegisterToolProviderRequestBuilder::broker_endpoint_ref)
    /// - [`capability_prefix`](RegisterToolProviderRequestBuilder::capability_prefix)
    /// - [`display_name`](RegisterToolProviderRequestBuilder::display_name)
    /// - [`provider_type`](RegisterToolProviderRequestBuilder::provider_type)
    /// - [`tool_catalog_ref`](RegisterToolProviderRequestBuilder::tool_catalog_ref)
    pub fn build(self) -> Result<RegisterToolProviderRequest, BuildError> {
        Ok(RegisterToolProviderRequest {
            approval_policy_ref: self.approval_policy_ref,
            broker_endpoint_ref: self.broker_endpoint_ref.ok_or_else(|| BuildError::missing_field("broker_endpoint_ref"))?,
            capability_prefix: self.capability_prefix.ok_or_else(|| BuildError::missing_field("capability_prefix"))?,
            credential_owner: self.credential_owner,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            provider_type: self.provider_type.ok_or_else(|| BuildError::missing_field("provider_type"))?,
            tool_catalog_ref: self.tool_catalog_ref.ok_or_else(|| BuildError::missing_field("tool_catalog_ref"))?,
        })
    }
}

