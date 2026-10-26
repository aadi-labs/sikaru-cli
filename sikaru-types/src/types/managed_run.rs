pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ManagedRun {
    #[serde(rename = "compatibilityProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_profile_id: Option<String>,
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<PersonalConnectionPrompt>>,
    #[serde(rename = "contentVisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_visible: Option<bool>,
    #[serde(rename = "costSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_summary: Option<HashMap<String, serde_json::Value>>,
    /// Why the run failed; set when `status` is `failed`, otherwise null. Omitted when `contentVisible` is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RunError>,
    #[serde(rename = "eventsUrl")]
    #[serde(default)]
    pub events_url: String,
    #[serde(rename = "harnessId")]
    #[serde(default)]
    pub harness_id: String,
    #[serde(rename = "harnessVersionId")]
    #[serde(default)]
    pub harness_version_id: String,
    #[serde(rename = "latencyMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<f64>,
    #[serde(rename = "measurementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurement_status: Option<ManagedRunMeasurementStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause: Option<ManagedRunPause>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal: Option<bool>,
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "usageSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_summary: Option<HashMap<String, serde_json::Value>>,
}

impl ManagedRun {
    pub fn builder() -> ManagedRunBuilder {
        <ManagedRunBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManagedRunBuilder {
    compatibility_profile_id: Option<String>,
    completed_at: Option<String>,
    connections: Option<Vec<PersonalConnectionPrompt>>,
    content_visible: Option<bool>,
    cost_summary: Option<HashMap<String, serde_json::Value>>,
    error: Option<RunError>,
    events_url: Option<String>,
    harness_id: Option<String>,
    harness_version_id: Option<String>,
    latency_ms: Option<f64>,
    measurement_status: Option<ManagedRunMeasurementStatus>,
    pause: Option<ManagedRunPause>,
    personal: Option<bool>,
    run_id: Option<String>,
    status: Option<String>,
    usage_summary: Option<HashMap<String, serde_json::Value>>,
}

impl ManagedRunBuilder {
    pub fn compatibility_profile_id(mut self, value: impl Into<String>) -> Self {
        self.compatibility_profile_id = Some(value.into());
        self
    }

    pub fn completed_at(mut self, value: impl Into<String>) -> Self {
        self.completed_at = Some(value.into());
        self
    }

    pub fn connections(mut self, value: Vec<PersonalConnectionPrompt>) -> Self {
        self.connections = Some(value);
        self
    }

    pub fn content_visible(mut self, value: bool) -> Self {
        self.content_visible = Some(value);
        self
    }

    pub fn cost_summary(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.cost_summary = Some(value);
        self
    }

    pub fn error(mut self, value: RunError) -> Self {
        self.error = Some(value);
        self
    }

    pub fn events_url(mut self, value: impl Into<String>) -> Self {
        self.events_url = Some(value.into());
        self
    }

    pub fn harness_id(mut self, value: impl Into<String>) -> Self {
        self.harness_id = Some(value.into());
        self
    }

    pub fn harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.harness_version_id = Some(value.into());
        self
    }

    pub fn latency_ms(mut self, value: f64) -> Self {
        self.latency_ms = Some(value);
        self
    }

    pub fn measurement_status(mut self, value: ManagedRunMeasurementStatus) -> Self {
        self.measurement_status = Some(value);
        self
    }

    pub fn pause(mut self, value: ManagedRunPause) -> Self {
        self.pause = Some(value);
        self
    }

    pub fn personal(mut self, value: bool) -> Self {
        self.personal = Some(value);
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

    pub fn usage_summary(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.usage_summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ManagedRun`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events_url`](ManagedRunBuilder::events_url)
    /// - [`harness_id`](ManagedRunBuilder::harness_id)
    /// - [`harness_version_id`](ManagedRunBuilder::harness_version_id)
    /// - [`run_id`](ManagedRunBuilder::run_id)
    /// - [`status`](ManagedRunBuilder::status)
    pub fn build(self) -> Result<ManagedRun, BuildError> {
        Ok(ManagedRun {
            compatibility_profile_id: self.compatibility_profile_id,
            completed_at: self.completed_at,
            connections: self.connections,
            content_visible: self.content_visible,
            cost_summary: self.cost_summary,
            error: self.error,
            events_url: self.events_url.ok_or_else(|| BuildError::missing_field("events_url"))?,
            harness_id: self.harness_id.ok_or_else(|| BuildError::missing_field("harness_id"))?,
            harness_version_id: self.harness_version_id.ok_or_else(|| BuildError::missing_field("harness_version_id"))?,
            latency_ms: self.latency_ms,
            measurement_status: self.measurement_status,
            pause: self.pause,
            personal: self.personal,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            usage_summary: self.usage_summary,
        })
    }
}
