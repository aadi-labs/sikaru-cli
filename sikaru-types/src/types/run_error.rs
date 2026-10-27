pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Why a run failed: a stable code, a customer-facing message and whether `retry` can run it again.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RunError {
    /// Stable failure code. Known values: `credits_exhausted`, `spend_limit_reached`, `billing_unavailable`, `plan_inactive`, `trial_ended`, `model_request_rejected`, `model_not_available`, `model_access_denied`, `effect_outcome_uncertain`, `model_timeout`, `model_unavailable`, `setup_failed`, `tool_not_allowed`, `evidence_unavailable`, `interrupted`, `runtime_incompatible`, `startup_failed`, `capacity_unavailable`, `compute_unavailable`, `recovery_unavailable`, `execution_connection_lost`, `execution_process_exited`, `execution_session_failed`, `execution_failed`. Treat an unrecognized code like `execution_failed`.
    #[serde(default)]
    pub code: String,
    /// Customer-facing explanation and next step.
    #[serde(default)]
    pub message: String,
    /// Whether `retry` can run the failed turn again. When false, follow the message instead, for example by starting a new session.
    #[serde(default)]
    pub recoverable: bool,
    /// Execution stage where the failure occurred.
    pub stage: RunErrorStage,
}

impl RunError {
    pub fn builder() -> RunErrorBuilder {
        <RunErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunErrorBuilder {
    code: Option<String>,
    message: Option<String>,
    recoverable: Option<bool>,
    stage: Option<RunErrorStage>,
}

impl RunErrorBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn recoverable(mut self, value: bool) -> Self {
        self.recoverable = Some(value);
        self
    }

    pub fn stage(mut self, value: RunErrorStage) -> Self {
        self.stage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](RunErrorBuilder::code)
    /// - [`message`](RunErrorBuilder::message)
    /// - [`recoverable`](RunErrorBuilder::recoverable)
    /// - [`stage`](RunErrorBuilder::stage)
    pub fn build(self) -> Result<RunError, BuildError> {
        Ok(RunError {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            recoverable: self.recoverable.ok_or_else(|| BuildError::missing_field("recoverable"))?,
            stage: self.stage.ok_or_else(|| BuildError::missing_field("stage"))?,
        })
    }
}
