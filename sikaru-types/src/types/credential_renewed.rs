pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CredentialRenewed {
    #[serde(default)]
    pub credential_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub expires_at: f64,
}

impl CredentialRenewed {
    pub fn builder() -> CredentialRenewedBuilder {
        <CredentialRenewedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CredentialRenewedBuilder {
    credential_id: Option<String>,
    expires_at: Option<f64>,
}

impl CredentialRenewedBuilder {
    pub fn credential_id(mut self, value: impl Into<String>) -> Self {
        self.credential_id = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: f64) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CredentialRenewed`].
    /// This method will fail if any of the following fields are not set:
    /// - [`credential_id`](CredentialRenewedBuilder::credential_id)
    /// - [`expires_at`](CredentialRenewedBuilder::expires_at)
    pub fn build(self) -> Result<CredentialRenewed, BuildError> {
        Ok(CredentialRenewed {
            credential_id: self.credential_id.ok_or_else(|| BuildError::missing_field("credential_id"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
