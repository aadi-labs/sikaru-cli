pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExecutionView {
    #[serde(default)]
    pub approval_required: bool,
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub terminal: bool,
}

impl ExecutionView {
    pub fn builder() -> ExecutionViewBuilder {
        <ExecutionViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionViewBuilder {
    approval_required: Option<bool>,
    run_id: Option<String>,
    status: Option<String>,
    terminal: Option<bool>,
}

impl ExecutionViewBuilder {
    pub fn approval_required(mut self, value: bool) -> Self {
        self.approval_required = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn terminal(mut self, value: bool) -> Self {
        self.terminal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExecutionView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`approval_required`](ExecutionViewBuilder::approval_required)
    /// - [`run_id`](ExecutionViewBuilder::run_id)
    /// - [`status`](ExecutionViewBuilder::status)
    /// - [`terminal`](ExecutionViewBuilder::terminal)
    pub fn build(self) -> Result<ExecutionView, BuildError> {
        Ok(ExecutionView {
            approval_required: self.approval_required.ok_or_else(|| BuildError::missing_field("approval_required"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            terminal: self.terminal.ok_or_else(|| BuildError::missing_field("terminal"))?,
        })
    }
}
