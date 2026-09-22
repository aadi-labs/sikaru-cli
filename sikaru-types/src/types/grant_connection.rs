pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GrantConnection {
    #[serde(default)]
    pub agent_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval: Option<GrantConnectionApproval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<GrantConnectionEnvironment>,
    #[serde(default)]
    pub tools: Vec<String>,
}

impl GrantConnection {
    pub fn builder() -> GrantConnectionBuilder {
        <GrantConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GrantConnectionBuilder {
    agent_id: Option<String>,
    approval: Option<GrantConnectionApproval>,
    environment: Option<GrantConnectionEnvironment>,
    tools: Option<Vec<String>>,
}

impl GrantConnectionBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn approval(mut self, value: GrantConnectionApproval) -> Self {
        self.approval = Some(value);
        self
    }

    pub fn environment(mut self, value: GrantConnectionEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<String>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GrantConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](GrantConnectionBuilder::agent_id)
    /// - [`tools`](GrantConnectionBuilder::tools)
    pub fn build(self) -> Result<GrantConnection, BuildError> {
        Ok(GrantConnection {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            approval: self.approval,
            environment: self.environment,
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
        })
    }
}

