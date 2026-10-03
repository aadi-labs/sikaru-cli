pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentToolCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<BuiltInToolSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bash: Option<BuiltInToolSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<MemoryToolSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_fetch: Option<BuiltInToolSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search: Option<BuiltInToolSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<BuiltInToolSetting>,
}

impl AgentToolCapabilities {
    pub fn builder() -> AgentToolCapabilitiesBuilder {
        <AgentToolCapabilitiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentToolCapabilitiesBuilder {
    agents: Option<BuiltInToolSetting>,
    bash: Option<BuiltInToolSetting>,
    memory: Option<MemoryToolSetting>,
    web_fetch: Option<BuiltInToolSetting>,
    web_search: Option<BuiltInToolSetting>,
    workspace: Option<BuiltInToolSetting>,
}

impl AgentToolCapabilitiesBuilder {
    pub fn agents(mut self, value: BuiltInToolSetting) -> Self {
        self.agents = Some(value);
        self
    }

    pub fn bash(mut self, value: BuiltInToolSetting) -> Self {
        self.bash = Some(value);
        self
    }

    pub fn memory(mut self, value: MemoryToolSetting) -> Self {
        self.memory = Some(value);
        self
    }

    pub fn web_fetch(mut self, value: BuiltInToolSetting) -> Self {
        self.web_fetch = Some(value);
        self
    }

    pub fn web_search(mut self, value: BuiltInToolSetting) -> Self {
        self.web_search = Some(value);
        self
    }

    pub fn workspace(mut self, value: BuiltInToolSetting) -> Self {
        self.workspace = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentToolCapabilities`].
    pub fn build(self) -> Result<AgentToolCapabilities, BuildError> {
        Ok(AgentToolCapabilities {
            agents: self.agents,
            bash: self.bash,
            memory: self.memory,
            web_fetch: self.web_fetch,
            web_search: self.web_search,
            workspace: self.workspace,
        })
    }
}
