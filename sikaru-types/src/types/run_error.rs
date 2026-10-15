pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Why a run failed: a stable code, a customer-facing message and whether `retry` can run it again.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunError {
    /// Stable failure code. Known values: `session_budget_exhausted`, `model_timeout`, `model_unavailable`, `insufficient_credits`, `setup_failed`, `tool_not_allowed`, `evidence_unavailable`, `interrupted`, `capacity_unavailable`, `execution_failed`. Treat an unrecognized code like `execution_failed`.
    #[serde(default)]
    pub code: String,
    /// Customer-facing explanation and next step.
    #[serde(default)]
    pub message: String,
    /// Whether `retry` can run the failed turn again. When false, follow the message instead, for example by starting a new session.
    #[serde(default)]
    pub recoverable: bool,
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

    /// Consumes the builder and constructs a [`RunError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](RunErrorBuilder::code)
    /// - [`message`](RunErrorBuilder::message)
    /// - [`recoverable`](RunErrorBuilder::recoverable)
    pub fn build(self) -> Result<RunError, BuildError> {
        Ok(RunError {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            recoverable: self.recoverable.ok_or_else(|| BuildError::missing_field("recoverable"))?,
        })
    }
}
