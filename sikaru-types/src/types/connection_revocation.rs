pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionRevocation {
    #[serde(default)]
    pub agents: Vec<ConnectionAgentUsage>,
    #[serde(default)]
    pub connection: Connection,
    #[serde(default)]
    pub count: i64,
    #[serde(default)]
    pub schedules: Vec<String>,
}

impl ConnectionRevocation {
    pub fn builder() -> ConnectionRevocationBuilder {
        <ConnectionRevocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionRevocationBuilder {
    agents: Option<Vec<ConnectionAgentUsage>>,
    connection: Option<Connection>,
    count: Option<i64>,
    schedules: Option<Vec<String>>,
}

impl ConnectionRevocationBuilder {
    pub fn agents(mut self, value: Vec<ConnectionAgentUsage>) -> Self {
        self.agents = Some(value);
        self
    }

    pub fn connection(mut self, value: Connection) -> Self {
        self.connection = Some(value);
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

    /// Consumes the builder and constructs a [`ConnectionRevocation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agents`](ConnectionRevocationBuilder::agents)
    /// - [`connection`](ConnectionRevocationBuilder::connection)
    /// - [`count`](ConnectionRevocationBuilder::count)
    /// - [`schedules`](ConnectionRevocationBuilder::schedules)
    pub fn build(self) -> Result<ConnectionRevocation, BuildError> {
        Ok(ConnectionRevocation {
            agents: self.agents.ok_or_else(|| BuildError::missing_field("agents"))?,
            connection: self.connection.ok_or_else(|| BuildError::missing_field("connection"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            schedules: self.schedules.ok_or_else(|| BuildError::missing_field("schedules"))?,
        })
    }
}
