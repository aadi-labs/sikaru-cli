pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The created or existing agent. `definitionRevision` is present when a changed
/// definition for an existing agent was staged as a draft revision.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatedManagedAgent {
    #[serde(rename = "definitionRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition_revision: Option<StagedDefinitionRevision>,
    #[serde(rename = "harnessVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harness_version: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "managedAgent")]
    #[serde(default)]
    pub managed_agent: HashMap<String, serde_json::Value>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl CreatedManagedAgent {
    pub fn builder() -> CreatedManagedAgentBuilder {
        <CreatedManagedAgentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatedManagedAgentBuilder {
    definition_revision: Option<StagedDefinitionRevision>,
    harness_version: Option<HashMap<String, serde_json::Value>>,
    managed_agent: Option<HashMap<String, serde_json::Value>>,
}

impl CreatedManagedAgentBuilder {
    pub fn definition_revision(mut self, value: StagedDefinitionRevision) -> Self {
        self.definition_revision = Some(value);
        self
    }

    pub fn harness_version(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.harness_version = Some(value);
        self
    }

    pub fn managed_agent(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.managed_agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatedManagedAgent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`managed_agent`](CreatedManagedAgentBuilder::managed_agent)
    pub fn build(self) -> Result<CreatedManagedAgent, BuildError> {
        Ok(CreatedManagedAgent {
            definition_revision: self.definition_revision,
            harness_version: self.harness_version,
            managed_agent: self.managed_agent.ok_or_else(|| BuildError::missing_field("managed_agent"))?,
            extra: Default::default(),
        })
    }
}
