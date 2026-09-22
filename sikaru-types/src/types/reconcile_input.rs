pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReconcileInput {
    #[serde(default)]
    pub executor_instance_id: String,
    #[serde(default)]
    pub journal_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processes: Option<Vec<ProcessObservation>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipts: Option<Vec<ReceiptInput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncertain_operation_ids: Option<Vec<String>>,
    pub workspace_provenance: WorkspaceProvenance,
}

impl ReconcileInput {
    pub fn builder() -> ReconcileInputBuilder {
        <ReconcileInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReconcileInputBuilder {
    executor_instance_id: Option<String>,
    journal_id: Option<String>,
    processes: Option<Vec<ProcessObservation>>,
    receipts: Option<Vec<ReceiptInput>>,
    uncertain_operation_ids: Option<Vec<String>>,
    workspace_provenance: Option<WorkspaceProvenance>,
}

impl ReconcileInputBuilder {
    pub fn executor_instance_id(mut self, value: impl Into<String>) -> Self {
        self.executor_instance_id = Some(value.into());
        self
    }

    pub fn journal_id(mut self, value: impl Into<String>) -> Self {
        self.journal_id = Some(value.into());
        self
    }

    pub fn processes(mut self, value: Vec<ProcessObservation>) -> Self {
        self.processes = Some(value);
        self
    }

    pub fn receipts(mut self, value: Vec<ReceiptInput>) -> Self {
        self.receipts = Some(value);
        self
    }

    pub fn uncertain_operation_ids(mut self, value: Vec<String>) -> Self {
        self.uncertain_operation_ids = Some(value);
        self
    }

    pub fn workspace_provenance(mut self, value: WorkspaceProvenance) -> Self {
        self.workspace_provenance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReconcileInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`executor_instance_id`](ReconcileInputBuilder::executor_instance_id)
    /// - [`journal_id`](ReconcileInputBuilder::journal_id)
    /// - [`workspace_provenance`](ReconcileInputBuilder::workspace_provenance)
    pub fn build(self) -> Result<ReconcileInput, BuildError> {
        Ok(ReconcileInput {
            executor_instance_id: self.executor_instance_id.ok_or_else(|| BuildError::missing_field("executor_instance_id"))?,
            journal_id: self.journal_id.ok_or_else(|| BuildError::missing_field("journal_id"))?,
            processes: self.processes,
            receipts: self.receipts,
            uncertain_operation_ids: self.uncertain_operation_ids,
            workspace_provenance: self.workspace_provenance.ok_or_else(|| BuildError::missing_field("workspace_provenance"))?,
        })
    }
}

