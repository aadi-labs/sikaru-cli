pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReadyInput {
    /// Optional features beyond compute.execute. 'condition-waits-v1' serves bash.wait_for (WaitForArguments -> WaitForResult) and jobs.next_completed (NextCompletedArguments -> NextCompletedResult); only executors declaring it receive those operations.
    #[serde(default)]
    pub capabilities: Vec<ReadyInputCapabilitiesItem>,
    #[serde(default)]
    pub executor_instance_id: String,
    #[serde(default)]
    pub journal_id: String,
    pub protocol_version: ReadyInputProtocolVersion,
    pub workspace_provenance: WorkspaceProvenance,
}

impl ReadyInput {
    pub fn builder() -> ReadyInputBuilder {
        <ReadyInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadyInputBuilder {
    capabilities: Option<Vec<ReadyInputCapabilitiesItem>>,
    executor_instance_id: Option<String>,
    journal_id: Option<String>,
    protocol_version: Option<ReadyInputProtocolVersion>,
    workspace_provenance: Option<WorkspaceProvenance>,
}

impl ReadyInputBuilder {
    pub fn capabilities(mut self, value: Vec<ReadyInputCapabilitiesItem>) -> Self {
        self.capabilities = Some(value);
        self
    }

    pub fn executor_instance_id(mut self, value: impl Into<String>) -> Self {
        self.executor_instance_id = Some(value.into());
        self
    }

    pub fn journal_id(mut self, value: impl Into<String>) -> Self {
        self.journal_id = Some(value.into());
        self
    }

    pub fn protocol_version(mut self, value: ReadyInputProtocolVersion) -> Self {
        self.protocol_version = Some(value);
        self
    }

    pub fn workspace_provenance(mut self, value: WorkspaceProvenance) -> Self {
        self.workspace_provenance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReadyInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capabilities`](ReadyInputBuilder::capabilities)
    /// - [`executor_instance_id`](ReadyInputBuilder::executor_instance_id)
    /// - [`journal_id`](ReadyInputBuilder::journal_id)
    /// - [`protocol_version`](ReadyInputBuilder::protocol_version)
    /// - [`workspace_provenance`](ReadyInputBuilder::workspace_provenance)
    pub fn build(self) -> Result<ReadyInput, BuildError> {
        Ok(ReadyInput {
            capabilities: self.capabilities.ok_or_else(|| BuildError::missing_field("capabilities"))?,
            executor_instance_id: self.executor_instance_id.ok_or_else(|| BuildError::missing_field("executor_instance_id"))?,
            journal_id: self.journal_id.ok_or_else(|| BuildError::missing_field("journal_id"))?,
            protocol_version: self.protocol_version.ok_or_else(|| BuildError::missing_field("protocol_version"))?,
            workspace_provenance: self.workspace_provenance.ok_or_else(|| BuildError::missing_field("workspace_provenance"))?,
        })
    }
}
