pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExecutionTurnRun {
    #[serde(rename = "harnessVersionId")]
    #[serde(default)]
    pub harness_version_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "inputId")]
    #[serde(default)]
    pub input_id: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

impl ExecutionTurnRun {
    pub fn builder() -> ExecutionTurnRunBuilder {
        <ExecutionTurnRunBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionTurnRunBuilder {
    harness_version_id: Option<String>,
    id: Option<String>,
    input_id: Option<String>,
    session_id: Option<String>,
    status: Option<String>,
    trace_id: Option<String>,
}

impl ExecutionTurnRunBuilder {
    pub fn harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.harness_version_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn input_id(mut self, value: impl Into<String>) -> Self {
        self.input_id = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExecutionTurnRun`].
    /// This method will fail if any of the following fields are not set:
    /// - [`harness_version_id`](ExecutionTurnRunBuilder::harness_version_id)
    /// - [`id`](ExecutionTurnRunBuilder::id)
    /// - [`input_id`](ExecutionTurnRunBuilder::input_id)
    /// - [`session_id`](ExecutionTurnRunBuilder::session_id)
    /// - [`status`](ExecutionTurnRunBuilder::status)
    pub fn build(self) -> Result<ExecutionTurnRun, BuildError> {
        Ok(ExecutionTurnRun {
            harness_version_id: self.harness_version_id.ok_or_else(|| BuildError::missing_field("harness_version_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            input_id: self.input_id.ok_or_else(|| BuildError::missing_field("input_id"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            trace_id: self.trace_id,
        })
    }
}
