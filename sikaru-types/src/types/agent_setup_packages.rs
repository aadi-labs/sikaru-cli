pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentSetupPackages {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub npm: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pip: Option<Vec<String>>,
}

impl AgentSetupPackages {
    pub fn builder() -> AgentSetupPackagesBuilder {
        <AgentSetupPackagesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSetupPackagesBuilder {
    npm: Option<Vec<String>>,
    pip: Option<Vec<String>>,
}

impl AgentSetupPackagesBuilder {
    pub fn npm(mut self, value: Vec<String>) -> Self {
        self.npm = Some(value);
        self
    }

    pub fn pip(mut self, value: Vec<String>) -> Self {
        self.pip = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentSetupPackages`].
    pub fn build(self) -> Result<AgentSetupPackages, BuildError> {
        Ok(AgentSetupPackages {
            npm: self.npm,
            pip: self.pip,
        })
    }
}
