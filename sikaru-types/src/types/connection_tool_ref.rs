pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A tool from a Composio or MCP connection granted to this agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectionToolRef {
    #[serde(default)]
    pub connection_id: String,
    #[serde(default)]
    pub tool: String,
}

impl ConnectionToolRef {
    pub fn builder() -> ConnectionToolRefBuilder {
        <ConnectionToolRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionToolRefBuilder {
    connection_id: Option<String>,
    tool: Option<String>,
}

impl ConnectionToolRefBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn tool(mut self, value: impl Into<String>) -> Self {
        self.tool = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectionToolRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](ConnectionToolRefBuilder::connection_id)
    /// - [`tool`](ConnectionToolRefBuilder::tool)
    pub fn build(self) -> Result<ConnectionToolRef, BuildError> {
        Ok(ConnectionToolRef {
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            tool: self.tool.ok_or_else(|| BuildError::missing_field("tool"))?,
        })
    }
}
