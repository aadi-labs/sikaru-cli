pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomerDm {
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub identity_app_id: String,
    #[serde(default)]
    pub installation_id: String,
    #[serde(default)]
    pub verification_id: String,
}

impl CustomerDm {
    pub fn builder() -> CustomerDmBuilder {
        <CustomerDmBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerDmBuilder {
    agent_id: Option<String>,
    identity_app_id: Option<String>,
    installation_id: Option<String>,
    verification_id: Option<String>,
}

impl CustomerDmBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn identity_app_id(mut self, value: impl Into<String>) -> Self {
        self.identity_app_id = Some(value.into());
        self
    }

    pub fn installation_id(mut self, value: impl Into<String>) -> Self {
        self.installation_id = Some(value.into());
        self
    }

    pub fn verification_id(mut self, value: impl Into<String>) -> Self {
        self.verification_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CustomerDm`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](CustomerDmBuilder::agent_id)
    /// - [`identity_app_id`](CustomerDmBuilder::identity_app_id)
    /// - [`installation_id`](CustomerDmBuilder::installation_id)
    /// - [`verification_id`](CustomerDmBuilder::verification_id)
    pub fn build(self) -> Result<CustomerDm, BuildError> {
        Ok(CustomerDm {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            identity_app_id: self.identity_app_id.ok_or_else(|| BuildError::missing_field("identity_app_id"))?,
            installation_id: self.installation_id.ok_or_else(|| BuildError::missing_field("installation_id"))?,
            verification_id: self.verification_id.ok_or_else(|| BuildError::missing_field("verification_id"))?,
        })
    }
}

