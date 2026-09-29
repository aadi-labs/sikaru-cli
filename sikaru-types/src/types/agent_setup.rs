pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentSetup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<AgentSetupPackages>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repos: Option<Vec<AgentSetupRepo>>,
}

impl AgentSetup {
    pub fn builder() -> AgentSetupBuilder {
        <AgentSetupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSetupBuilder {
    commands: Option<Vec<String>>,
    packages: Option<AgentSetupPackages>,
    repos: Option<Vec<AgentSetupRepo>>,
}

impl AgentSetupBuilder {
    pub fn commands(mut self, value: Vec<String>) -> Self {
        self.commands = Some(value);
        self
    }

    pub fn packages(mut self, value: AgentSetupPackages) -> Self {
        self.packages = Some(value);
        self
    }

    pub fn repos(mut self, value: Vec<AgentSetupRepo>) -> Self {
        self.repos = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentSetup`].
    pub fn build(self) -> Result<AgentSetup, BuildError> {
        Ok(AgentSetup {
            commands: self.commands,
            packages: self.packages,
            repos: self.repos,
        })
    }
}
