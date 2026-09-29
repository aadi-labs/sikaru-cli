pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateGitCredential {
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl CreateGitCredential {
    pub fn builder() -> CreateGitCredentialBuilder {
        <CreateGitCredentialBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateGitCredentialBuilder {
    agent_id: Option<String>,
    host: Option<String>,
    token: Option<String>,
    username: Option<String>,
}

impl CreateGitCredentialBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn host(mut self, value: impl Into<String>) -> Self {
        self.host = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateGitCredential`].
    /// This method will fail if any of the following fields are not set:
    /// - [`host`](CreateGitCredentialBuilder::host)
    /// - [`token`](CreateGitCredentialBuilder::token)
    pub fn build(self) -> Result<CreateGitCredential, BuildError> {
        Ok(CreateGitCredential {
            agent_id: self.agent_id,
            host: self.host.ok_or_else(|| BuildError::missing_field("host"))?,
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
            username: self.username,
        })
    }
}

