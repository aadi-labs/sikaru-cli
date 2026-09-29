pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GrantGitCredential {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
}

impl GrantGitCredential {
    pub fn builder() -> GrantGitCredentialBuilder {
        <GrantGitCredentialBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GrantGitCredentialBuilder {
    agent_id: Option<String>,
}

impl GrantGitCredentialBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GrantGitCredential`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](GrantGitCredentialBuilder::agent_id)
    pub fn build(self) -> Result<GrantGitCredential, BuildError> {
        Ok(GrantGitCredential {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
        })
    }
}

