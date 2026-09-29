pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentWebCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<AgentWebCapabilitiesProvider>,
}

impl AgentWebCapabilities {
    pub fn builder() -> AgentWebCapabilitiesBuilder {
        <AgentWebCapabilitiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentWebCapabilitiesBuilder {
    allow_domains: Option<Vec<String>>,
    block_domains: Option<Vec<String>>,
    provider: Option<AgentWebCapabilitiesProvider>,
}

impl AgentWebCapabilitiesBuilder {
    pub fn allow_domains(mut self, value: Vec<String>) -> Self {
        self.allow_domains = Some(value);
        self
    }

    pub fn block_domains(mut self, value: Vec<String>) -> Self {
        self.block_domains = Some(value);
        self
    }

    pub fn provider(mut self, value: AgentWebCapabilitiesProvider) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentWebCapabilities`].
    pub fn build(self) -> Result<AgentWebCapabilities, BuildError> {
        Ok(AgentWebCapabilities {
            allow_domains: self.allow_domains,
            block_domains: self.block_domains,
            provider: self.provider,
        })
    }
}
