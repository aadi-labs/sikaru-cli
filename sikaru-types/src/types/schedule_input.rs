pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScheduleInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ScheduleInputEnvironment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub input: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_seconds: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_mode: Option<ScheduleInputSessionMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl ScheduleInput {
    pub fn builder() -> ScheduleInputBuilder {
        <ScheduleInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleInputBuilder {
    agent_slug: Option<String>,
    cron: Option<String>,
    environment: Option<ScheduleInputEnvironment>,
    idempotency_key: Option<String>,
    input: Option<HashMap<String, serde_json::Value>>,
    interval_seconds: Option<i64>,
    session_id: Option<String>,
    session_mode: Option<ScheduleInputSessionMode>,
    timezone: Option<String>,
}

impl ScheduleInputBuilder {
    pub fn agent_slug(mut self, value: impl Into<String>) -> Self {
        self.agent_slug = Some(value.into());
        self
    }

    pub fn cron(mut self, value: impl Into<String>) -> Self {
        self.cron = Some(value.into());
        self
    }

    pub fn environment(mut self, value: ScheduleInputEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn input(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input = Some(value);
        self
    }

    pub fn interval_seconds(mut self, value: i64) -> Self {
        self.interval_seconds = Some(value);
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn session_mode(mut self, value: ScheduleInputSessionMode) -> Self {
        self.session_mode = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScheduleInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](ScheduleInputBuilder::input)
    pub fn build(self) -> Result<ScheduleInput, BuildError> {
        Ok(ScheduleInput {
            agent_slug: self.agent_slug,
            cron: self.cron,
            environment: self.environment,
            idempotency_key: self.idempotency_key,
            input: self.input.ok_or_else(|| BuildError::missing_field("input"))?,
            interval_seconds: self.interval_seconds,
            session_id: self.session_id,
            session_mode: self.session_mode,
            timezone: self.timezone,
        })
    }
}

