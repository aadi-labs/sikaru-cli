pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ManagedAgentRead {
    #[serde(rename = "managedAgent")]
    #[serde(default)]
    pub managed_agent: ManagedAgentView,
}

impl ManagedAgentRead {
    pub fn builder() -> ManagedAgentReadBuilder {
        <ManagedAgentReadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManagedAgentReadBuilder {
    managed_agent: Option<ManagedAgentView>,
}

impl ManagedAgentReadBuilder {
    pub fn managed_agent(mut self, value: ManagedAgentView) -> Self {
        self.managed_agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ManagedAgentRead`].
    /// This method will fail if any of the following fields are not set:
    /// - [`managed_agent`](ManagedAgentReadBuilder::managed_agent)
    pub fn build(self) -> Result<ManagedAgentRead, BuildError> {
        Ok(ManagedAgentRead {
            managed_agent: self.managed_agent.ok_or_else(|| BuildError::missing_field("managed_agent"))?,
        })
    }
}
