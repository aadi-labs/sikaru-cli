pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CredentialRevoked {
    #[serde(default)]
    pub credential_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub revoked_at: f64,
}

impl CredentialRevoked {
    pub fn builder() -> CredentialRevokedBuilder {
        <CredentialRevokedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CredentialRevokedBuilder {
    credential_id: Option<String>,
    revoked_at: Option<f64>,
}

impl CredentialRevokedBuilder {
    pub fn credential_id(mut self, value: impl Into<String>) -> Self {
        self.credential_id = Some(value.into());
        self
    }

    pub fn revoked_at(mut self, value: f64) -> Self {
        self.revoked_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CredentialRevoked`].
    /// This method will fail if any of the following fields are not set:
    /// - [`credential_id`](CredentialRevokedBuilder::credential_id)
    /// - [`revoked_at`](CredentialRevokedBuilder::revoked_at)
    pub fn build(self) -> Result<CredentialRevoked, BuildError> {
        Ok(CredentialRevoked {
            credential_id: self.credential_id.ok_or_else(|| BuildError::missing_field("credential_id"))?,
            revoked_at: self.revoked_at.ok_or_else(|| BuildError::missing_field("revoked_at"))?,
        })
    }
}
