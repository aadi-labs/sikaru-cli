pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PersonalAccessConfiguration {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub identity_app_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_tenant_id: Option<String>,
}

impl PersonalAccessConfiguration {
    pub fn builder() -> PersonalAccessConfigurationBuilder {
        <PersonalAccessConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonalAccessConfigurationBuilder {
    enabled: Option<bool>,
    identity_app_ids: Option<Vec<String>>,
    shared_tenant_id: Option<String>,
}

impl PersonalAccessConfigurationBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn identity_app_ids(mut self, value: Vec<String>) -> Self {
        self.identity_app_ids = Some(value);
        self
    }

    pub fn shared_tenant_id(mut self, value: impl Into<String>) -> Self {
        self.shared_tenant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PersonalAccessConfiguration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](PersonalAccessConfigurationBuilder::enabled)
    /// - [`identity_app_ids`](PersonalAccessConfigurationBuilder::identity_app_ids)
    pub fn build(self) -> Result<PersonalAccessConfiguration, BuildError> {
        Ok(PersonalAccessConfiguration {
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            identity_app_ids: self.identity_app_ids.ok_or_else(|| BuildError::missing_field("identity_app_ids"))?,
            shared_tenant_id: self.shared_tenant_id,
        })
    }
}
