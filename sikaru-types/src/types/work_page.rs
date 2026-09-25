pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkPage {
    pub attachment: AttachmentView,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<ExecutionView>,
    pub execution_phase: WorkPageExecutionPhase,
    #[serde(default)]
    pub issued_operations: Vec<UncertainOperation>,
    #[serde(default)]
    pub live_handles: Vec<LiveHandle>,
    #[serde(default)]
    pub operations: Vec<OperationView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_after_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_checkpoint: Option<WorkspaceCheckpointView>,
}

impl WorkPage {
    pub fn builder() -> WorkPageBuilder {
        <WorkPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkPageBuilder {
    attachment: Option<AttachmentView>,
    execution: Option<ExecutionView>,
    execution_phase: Option<WorkPageExecutionPhase>,
    issued_operations: Option<Vec<UncertainOperation>>,
    live_handles: Option<Vec<LiveHandle>>,
    operations: Option<Vec<OperationView>>,
    poll_after_seconds: Option<i64>,
    workspace_checkpoint: Option<WorkspaceCheckpointView>,
}

impl WorkPageBuilder {
    pub fn attachment(mut self, value: AttachmentView) -> Self {
        self.attachment = Some(value);
        self
    }

    pub fn execution(mut self, value: ExecutionView) -> Self {
        self.execution = Some(value);
        self
    }

    pub fn execution_phase(mut self, value: WorkPageExecutionPhase) -> Self {
        self.execution_phase = Some(value);
        self
    }

    pub fn issued_operations(mut self, value: Vec<UncertainOperation>) -> Self {
        self.issued_operations = Some(value);
        self
    }

    pub fn live_handles(mut self, value: Vec<LiveHandle>) -> Self {
        self.live_handles = Some(value);
        self
    }

    pub fn operations(mut self, value: Vec<OperationView>) -> Self {
        self.operations = Some(value);
        self
    }

    pub fn poll_after_seconds(mut self, value: i64) -> Self {
        self.poll_after_seconds = Some(value);
        self
    }

    pub fn workspace_checkpoint(mut self, value: WorkspaceCheckpointView) -> Self {
        self.workspace_checkpoint = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkPage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attachment`](WorkPageBuilder::attachment)
    /// - [`execution_phase`](WorkPageBuilder::execution_phase)
    /// - [`issued_operations`](WorkPageBuilder::issued_operations)
    /// - [`live_handles`](WorkPageBuilder::live_handles)
    /// - [`operations`](WorkPageBuilder::operations)
    pub fn build(self) -> Result<WorkPage, BuildError> {
        Ok(WorkPage {
            attachment: self.attachment.ok_or_else(|| BuildError::missing_field("attachment"))?,
            execution: self.execution,
            execution_phase: self.execution_phase.ok_or_else(|| BuildError::missing_field("execution_phase"))?,
            issued_operations: self.issued_operations.ok_or_else(|| BuildError::missing_field("issued_operations"))?,
            live_handles: self.live_handles.ok_or_else(|| BuildError::missing_field("live_handles"))?,
            operations: self.operations.ok_or_else(|| BuildError::missing_field("operations"))?,
            poll_after_seconds: self.poll_after_seconds,
            workspace_checkpoint: self.workspace_checkpoint,
        })
    }
}
