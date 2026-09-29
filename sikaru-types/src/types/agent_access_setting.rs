pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentAccessSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<AgentAccessSettingOwnership>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<AgentAccessSettingPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,
}

impl AgentAccessSetting {
    pub fn builder() -> AgentAccessSettingBuilder {
        <AgentAccessSettingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentAccessSettingBuilder {
    ownership: Option<AgentAccessSettingOwnership>,
    policy: Option<AgentAccessSettingPolicy>,
    tools: Option<Vec<String>>,
}

impl AgentAccessSettingBuilder {
    pub fn ownership(mut self, value: AgentAccessSettingOwnership) -> Self {
        self.ownership = Some(value);
        self
    }

    pub fn policy(mut self, value: AgentAccessSettingPolicy) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<String>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentAccessSetting`].
    pub fn build(self) -> Result<AgentAccessSetting, BuildError> {
        Ok(AgentAccessSetting {
            ownership: self.ownership,
            policy: self.policy,
            tools: self.tools,
        })
    }
}
