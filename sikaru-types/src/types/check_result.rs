pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckResult {
    #[serde(rename = "checkId")]
    #[serde(default)]
    pub check_id: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    pub environment: CheckResultEnvironment,
    #[serde(rename = "finishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub reward: Option<f64>,
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
    pub status: CheckResultStatus,
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
    #[serde(rename = "trajectoryUrl")]
    #[serde(default)]
    pub trajectory_url: String,
}

impl CheckResult {
    pub fn builder() -> CheckResultBuilder {
        <CheckResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckResultBuilder {
    check_id: Option<String>,
    created_at: Option<String>,
    environment: Option<CheckResultEnvironment>,
    finished_at: Option<String>,
    id: Option<String>,
    reason: Option<String>,
    reward: Option<f64>,
    run_id: Option<String>,
    session_id: Option<String>,
    status: Option<CheckResultStatus>,
    trace_id: Option<String>,
    trajectory_url: Option<String>,
}

impl CheckResultBuilder {
    pub fn check_id(mut self, value: impl Into<String>) -> Self {
        self.check_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn environment(mut self, value: CheckResultEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn finished_at(mut self, value: impl Into<String>) -> Self {
        self.finished_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn reward(mut self, value: f64) -> Self {
        self.reward = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: CheckResultStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn trajectory_url(mut self, value: impl Into<String>) -> Self {
        self.trajectory_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`check_id`](CheckResultBuilder::check_id)
    /// - [`created_at`](CheckResultBuilder::created_at)
    /// - [`environment`](CheckResultBuilder::environment)
    /// - [`id`](CheckResultBuilder::id)
    /// - [`reason`](CheckResultBuilder::reason)
    /// - [`run_id`](CheckResultBuilder::run_id)
    /// - [`session_id`](CheckResultBuilder::session_id)
    /// - [`status`](CheckResultBuilder::status)
    /// - [`trace_id`](CheckResultBuilder::trace_id)
    /// - [`trajectory_url`](CheckResultBuilder::trajectory_url)
    pub fn build(self) -> Result<CheckResult, BuildError> {
        Ok(CheckResult {
            check_id: self.check_id.ok_or_else(|| BuildError::missing_field("check_id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            finished_at: self.finished_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            reward: self.reward,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            trace_id: self.trace_id.ok_or_else(|| BuildError::missing_field("trace_id"))?,
            trajectory_url: self.trajectory_url.ok_or_else(|| BuildError::missing_field("trajectory_url"))?,
        })
    }
}
