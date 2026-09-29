pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GitCredential {
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(default)]
    pub grants: Vec<String>,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub id: String,
}

impl GitCredential {
    pub fn builder() -> GitCredentialBuilder {
        <GitCredentialBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GitCredentialBuilder {
    agent_id: Option<String>,
    created_at: Option<f64>,
    grants: Option<Vec<String>>,
    host: Option<String>,
    id: Option<String>,
}

impl GitCredentialBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn grants(mut self, value: Vec<String>) -> Self {
        self.grants = Some(value);
        self
    }

    pub fn host(mut self, value: impl Into<String>) -> Self {
        self.host = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GitCredential`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](GitCredentialBuilder::created_at)
    /// - [`grants`](GitCredentialBuilder::grants)
    /// - [`host`](GitCredentialBuilder::host)
    /// - [`id`](GitCredentialBuilder::id)
    pub fn build(self) -> Result<GitCredential, BuildError> {
        Ok(GitCredential {
            agent_id: self.agent_id,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            grants: self.grants.ok_or_else(|| BuildError::missing_field("grants"))?,
            host: self.host.ok_or_else(|| BuildError::missing_field("host"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
