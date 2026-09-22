pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CredentialIssued {
    #[serde(default)]
    pub credential_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub expires_at: f64,
    /// Secret returned only on issuance. Never passed as a command-line argument.
    #[serde(default)]
    pub token: String,
}

impl CredentialIssued {
    pub fn builder() -> CredentialIssuedBuilder {
        <CredentialIssuedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CredentialIssuedBuilder {
    credential_id: Option<String>,
    expires_at: Option<f64>,
    token: Option<String>,
}

impl CredentialIssuedBuilder {
    pub fn credential_id(mut self, value: impl Into<String>) -> Self {
        self.credential_id = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: f64) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CredentialIssued`].
    /// This method will fail if any of the following fields are not set:
    /// - [`credential_id`](CredentialIssuedBuilder::credential_id)
    /// - [`expires_at`](CredentialIssuedBuilder::expires_at)
    /// - [`token`](CredentialIssuedBuilder::token)
    pub fn build(self) -> Result<CredentialIssued, BuildError> {
        Ok(CredentialIssued {
            credential_id: self.credential_id.ok_or_else(|| BuildError::missing_field("credential_id"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
        })
    }
}
