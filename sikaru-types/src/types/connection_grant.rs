pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionGrant {
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub agent_id: String,
    #[serde(default)]
    pub approval: String,
    #[serde(default)]
    pub environment: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub tools: HashMap<String, String>,
}

impl ConnectionGrant {
    pub fn builder() -> ConnectionGrantBuilder {
        <ConnectionGrantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionGrantBuilder {
    active: Option<bool>,
    agent_id: Option<String>,
    approval: Option<String>,
    environment: Option<String>,
    id: Option<String>,
    tools: Option<HashMap<String, String>>,
}

impl ConnectionGrantBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn approval(mut self, value: impl Into<String>) -> Self {
        self.approval = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn tools(mut self, value: HashMap<String, String>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectionGrant`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active`](ConnectionGrantBuilder::active)
    /// - [`agent_id`](ConnectionGrantBuilder::agent_id)
    /// - [`approval`](ConnectionGrantBuilder::approval)
    /// - [`environment`](ConnectionGrantBuilder::environment)
    /// - [`id`](ConnectionGrantBuilder::id)
    /// - [`tools`](ConnectionGrantBuilder::tools)
    pub fn build(self) -> Result<ConnectionGrant, BuildError> {
        Ok(ConnectionGrant {
            active: self.active.ok_or_else(|| BuildError::missing_field("active"))?,
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            approval: self.approval.ok_or_else(|| BuildError::missing_field("approval"))?,
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
        })
    }
}
