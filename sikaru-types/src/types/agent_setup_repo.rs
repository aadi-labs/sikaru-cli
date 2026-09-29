pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentSetupRepo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_credential: Option<String>,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub url: String,
}

impl AgentSetupRepo {
    pub fn builder() -> AgentSetupRepoBuilder {
        <AgentSetupRepoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSetupRepoBuilder {
    git_credential: Option<String>,
    path: Option<String>,
    url: Option<String>,
}

impl AgentSetupRepoBuilder {
    pub fn git_credential(mut self, value: impl Into<String>) -> Self {
        self.git_credential = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSetupRepo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](AgentSetupRepoBuilder::path)
    /// - [`url`](AgentSetupRepoBuilder::url)
    pub fn build(self) -> Result<AgentSetupRepo, BuildError> {
        Ok(AgentSetupRepo {
            git_credential: self.git_credential,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
