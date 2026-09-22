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
    #[serde(rename = "costSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_summary: Option<HashMap<String, serde_json::Value>>,
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
    cost_summary: Option<HashMap<String, serde_json::Value>>,
    events_url: Option<String>,
    harness_id: Option<String>,
    harness_version_id: Option<String>,
    latency_ms: Option<f64>,
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

    pub fn cost_summary(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.cost_summary = Some(value);
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
            cost_summary: self.cost_summary,
            events_url: self.events_url.ok_or_else(|| BuildError::missing_field("events_url"))?,
            harness_id: self.harness_id.ok_or_else(|| BuildError::missing_field("harness_id"))?,
            harness_version_id: self.harness_version_id.ok_or_else(|| BuildError::missing_field("harness_version_id"))?,
            latency_ms: self.latency_ms,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            usage_summary: self.usage_summary,
        })
    }
}
