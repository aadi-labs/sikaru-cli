pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateManagedEnvironmentRequest {
    #[serde(rename = "configRefs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_refs: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "environmentSlug")]
    #[serde(default)]
    pub environment_slug: String,
    #[serde(rename = "providerType")]
    #[serde(default)]
    pub provider_type: String,
    #[serde(rename = "runtimeProvider")]
    #[serde(default)]
    pub runtime_provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl CreateManagedEnvironmentRequest {
    pub fn builder() -> CreateManagedEnvironmentRequestBuilder {
        <CreateManagedEnvironmentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateManagedEnvironmentRequestBuilder {
    config_refs: Option<HashMap<String, serde_json::Value>>,
    environment_slug: Option<String>,
    provider_type: Option<String>,
    runtime_provider: Option<String>,
    status: Option<String>,
}

impl CreateManagedEnvironmentRequestBuilder {
    pub fn config_refs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.config_refs = Some(value);
        self
    }

    pub fn environment_slug(mut self, value: impl Into<String>) -> Self {
        self.environment_slug = Some(value.into());
        self
    }

    pub fn provider_type(mut self, value: impl Into<String>) -> Self {
        self.provider_type = Some(value.into());
        self
    }

    pub fn runtime_provider(mut self, value: impl Into<String>) -> Self {
        self.runtime_provider = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateManagedEnvironmentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment_slug`](CreateManagedEnvironmentRequestBuilder::environment_slug)
    /// - [`provider_type`](CreateManagedEnvironmentRequestBuilder::provider_type)
    /// - [`runtime_provider`](CreateManagedEnvironmentRequestBuilder::runtime_provider)
    pub fn build(self) -> Result<CreateManagedEnvironmentRequest, BuildError> {
        Ok(CreateManagedEnvironmentRequest {
            config_refs: self.config_refs,
            environment_slug: self.environment_slug.ok_or_else(|| BuildError::missing_field("environment_slug"))?,
            provider_type: self.provider_type.ok_or_else(|| BuildError::missing_field("provider_type"))?,
            runtime_provider: self.runtime_provider.ok_or_else(|| BuildError::missing_field("runtime_provider"))?,
            status: self.status,
        })
    }
}

