pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TranscriptRun {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    pub environment: TranscriptRunEnvironment,
    #[serde(rename = "harnessVersionId")]
    #[serde(default)]
    pub harness_version_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    pub started_at: String,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

impl TranscriptRun {
    pub fn builder() -> TranscriptRunBuilder {
        <TranscriptRunBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptRunBuilder {
    account_id: Option<String>,
    completed_at: Option<String>,
    conversation_id: Option<String>,
    environment: Option<TranscriptRunEnvironment>,
    harness_version_id: Option<String>,
    id: Option<String>,
    name: Option<String>,
    session_id: Option<String>,
    started_at: Option<String>,
    status: Option<String>,
    trace_id: Option<String>,
}

impl TranscriptRunBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn completed_at(mut self, value: impl Into<String>) -> Self {
        self.completed_at = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn environment(mut self, value: TranscriptRunEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.harness_version_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: impl Into<String>) -> Self {
        self.started_at = Some(value.into());
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

    /// Consumes the builder and constructs a [`TranscriptRun`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](TranscriptRunBuilder::account_id)
    /// - [`environment`](TranscriptRunBuilder::environment)
    /// - [`harness_version_id`](TranscriptRunBuilder::harness_version_id)
    /// - [`id`](TranscriptRunBuilder::id)
    /// - [`name`](TranscriptRunBuilder::name)
    /// - [`session_id`](TranscriptRunBuilder::session_id)
    /// - [`started_at`](TranscriptRunBuilder::started_at)
    /// - [`status`](TranscriptRunBuilder::status)
    pub fn build(self) -> Result<TranscriptRun, BuildError> {
        Ok(TranscriptRun {
            account_id: self.account_id.ok_or_else(|| BuildError::missing_field("account_id"))?,
            completed_at: self.completed_at,
            conversation_id: self.conversation_id,
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            harness_version_id: self.harness_version_id.ok_or_else(|| BuildError::missing_field("harness_version_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            started_at: self.started_at.ok_or_else(|| BuildError::missing_field("started_at"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            trace_id: self.trace_id,
        })
    }
}
