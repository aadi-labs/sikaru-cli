pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RenamedManagedAgent {
    #[serde(rename = "managedAgent")]
    #[serde(default)]
    pub managed_agent: ManagedAgentView,
}

impl RenamedManagedAgent {
    pub fn builder() -> RenamedManagedAgentBuilder {
        <RenamedManagedAgentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RenamedManagedAgentBuilder {
    managed_agent: Option<ManagedAgentView>,
}

impl RenamedManagedAgentBuilder {
    pub fn managed_agent(mut self, value: ManagedAgentView) -> Self {
        self.managed_agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RenamedManagedAgent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`managed_agent`](RenamedManagedAgentBuilder::managed_agent)
    pub fn build(self) -> Result<RenamedManagedAgent, BuildError> {
        Ok(RenamedManagedAgent {
            managed_agent: self.managed_agent.ok_or_else(|| BuildError::missing_field("managed_agent"))?,
        })
    }
}
