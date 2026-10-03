pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScheduleOccurrence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "admittedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub admitted_at: f64,
    #[serde(rename = "harnessVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harness_version_id: Option<String>,
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "scheduledAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub scheduled_at: f64,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub status: ScheduleOccurrenceStatus,
}

impl ScheduleOccurrence {
    pub fn builder() -> ScheduleOccurrenceBuilder {
        <ScheduleOccurrenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleOccurrenceBuilder {
    access: Option<HashMap<String, serde_json::Value>>,
    admitted_at: Option<f64>,
    harness_version_id: Option<String>,
    run_id: Option<String>,
    scheduled_at: Option<f64>,
    session_id: Option<String>,
    status: Option<ScheduleOccurrenceStatus>,
}

impl ScheduleOccurrenceBuilder {
    pub fn access(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.access = Some(value);
        self
    }

    pub fn admitted_at(mut self, value: f64) -> Self {
        self.admitted_at = Some(value);
        self
    }

    pub fn harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.harness_version_id = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn scheduled_at(mut self, value: f64) -> Self {
        self.scheduled_at = Some(value);
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ScheduleOccurrenceStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScheduleOccurrence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`admitted_at`](ScheduleOccurrenceBuilder::admitted_at)
    /// - [`scheduled_at`](ScheduleOccurrenceBuilder::scheduled_at)
    /// - [`status`](ScheduleOccurrenceBuilder::status)
    pub fn build(self) -> Result<ScheduleOccurrence, BuildError> {
        Ok(ScheduleOccurrence {
            access: self.access,
            admitted_at: self.admitted_at.ok_or_else(|| BuildError::missing_field("admitted_at"))?,
            harness_version_id: self.harness_version_id,
            run_id: self.run_id,
            scheduled_at: self.scheduled_at.ok_or_else(|| BuildError::missing_field("scheduled_at"))?,
            session_id: self.session_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
