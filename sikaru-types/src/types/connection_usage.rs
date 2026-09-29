pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectionUsage {
    #[serde(default)]
    pub agents: Vec<ConnectionAgentUsage>,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub schedules: Vec<String>,
}

impl ConnectionUsage {
    pub fn builder() -> ConnectionUsageBuilder {
        <ConnectionUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionUsageBuilder {
    agents: Option<Vec<ConnectionAgentUsage>>,
    count: Option<i64>,
    schedules: Option<Vec<String>>,
}

impl ConnectionUsageBuilder {
    pub fn agents(mut self, value: Vec<ConnectionAgentUsage>) -> Self {
        self.agents = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn schedules(mut self, value: Vec<String>) -> Self {
        self.schedules = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectionUsage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agents`](ConnectionUsageBuilder::agents)
    /// - [`count`](ConnectionUsageBuilder::count)
    /// - [`schedules`](ConnectionUsageBuilder::schedules)
    pub fn build(self) -> Result<ConnectionUsage, BuildError> {
        Ok(ConnectionUsage {
            agents: self.agents.ok_or_else(|| BuildError::missing_field("agents"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            schedules: self.schedules.ok_or_else(|| BuildError::missing_field("schedules"))?,
        })
    }
}
