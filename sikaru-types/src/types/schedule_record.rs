pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScheduleRecord {
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "intervalSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_seconds: Option<i64>,
    #[serde(rename = "nextAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "sessionMode")]
    pub session_mode: ScheduleRecordSessionMode,
    #[serde(default)]
    pub timezone: String,
}

impl ScheduleRecord {
    pub fn builder() -> ScheduleRecordBuilder {
        <ScheduleRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleRecordBuilder {
    agent_id: Option<String>,
    creator: Option<String>,
    cron: Option<String>,
    environment: Option<String>,
    id: Option<String>,
    interval_seconds: Option<i64>,
    next_at: Option<f64>,
    paused: Option<bool>,
    session_id: Option<String>,
    session_mode: Option<ScheduleRecordSessionMode>,
    timezone: Option<String>,
}

impl ScheduleRecordBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn creator(mut self, value: impl Into<String>) -> Self {
        self.creator = Some(value.into());
        self
    }

    pub fn cron(mut self, value: impl Into<String>) -> Self {
        self.cron = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn interval_seconds(mut self, value: i64) -> Self {
        self.interval_seconds = Some(value);
        self
    }

    pub fn next_at(mut self, value: f64) -> Self {
        self.next_at = Some(value);
        self
    }

    pub fn paused(mut self, value: bool) -> Self {
        self.paused = Some(value);
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn session_mode(mut self, value: ScheduleRecordSessionMode) -> Self {
        self.session_mode = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScheduleRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ScheduleRecordBuilder::id)
    /// - [`session_mode`](ScheduleRecordBuilder::session_mode)
    /// - [`timezone`](ScheduleRecordBuilder::timezone)
    pub fn build(self) -> Result<ScheduleRecord, BuildError> {
        Ok(ScheduleRecord {
            agent_id: self.agent_id,
            creator: self.creator,
            cron: self.cron,
            environment: self.environment,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            interval_seconds: self.interval_seconds,
            next_at: self.next_at,
            paused: self.paused,
            session_id: self.session_id,
            session_mode: self.session_mode.ok_or_else(|| BuildError::missing_field("session_mode"))?,
            timezone: self.timezone.ok_or_else(|| BuildError::missing_field("timezone"))?,
        })
    }
}
