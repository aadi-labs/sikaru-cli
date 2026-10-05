pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DatasetCheckGroup {
    #[serde(rename = "agentSlug")]
    #[serde(default)]
    pub agent_slug: String,
    #[serde(rename = "agentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "datasetVersion")]
    #[serde(default)]
    pub dataset_version: i64,
    #[serde(default)]
    pub failed: i64,
    #[serde(default)]
    pub failures: Vec<DatasetCheckFailure>,
    #[serde(rename = "notStarted")]
    #[serde(default)]
    pub not_started: i64,
    #[serde(rename = "passRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pass_rate: Option<f64>,
    #[serde(default)]
    pub passed: i64,
    #[serde(default)]
    pub running: i64,
    #[serde(default)]
    pub total: i64,
}

impl DatasetCheckGroup {
    pub fn builder() -> DatasetCheckGroupBuilder {
        <DatasetCheckGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetCheckGroupBuilder {
    agent_slug: Option<String>,
    agent_version: Option<String>,
    dataset_version: Option<i64>,
    failed: Option<i64>,
    failures: Option<Vec<DatasetCheckFailure>>,
    not_started: Option<i64>,
    pass_rate: Option<f64>,
    passed: Option<i64>,
    running: Option<i64>,
    total: Option<i64>,
}

impl DatasetCheckGroupBuilder {
    pub fn agent_slug(mut self, value: impl Into<String>) -> Self {
        self.agent_slug = Some(value.into());
        self
    }

    pub fn agent_version(mut self, value: impl Into<String>) -> Self {
        self.agent_version = Some(value.into());
        self
    }

    pub fn dataset_version(mut self, value: i64) -> Self {
        self.dataset_version = Some(value);
        self
    }

    pub fn failed(mut self, value: i64) -> Self {
        self.failed = Some(value);
        self
    }

    pub fn failures(mut self, value: Vec<DatasetCheckFailure>) -> Self {
        self.failures = Some(value);
        self
    }

    pub fn not_started(mut self, value: i64) -> Self {
        self.not_started = Some(value);
        self
    }

    pub fn pass_rate(mut self, value: f64) -> Self {
        self.pass_rate = Some(value);
        self
    }

    pub fn passed(mut self, value: i64) -> Self {
        self.passed = Some(value);
        self
    }

    pub fn running(mut self, value: i64) -> Self {
        self.running = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetCheckGroup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_slug`](DatasetCheckGroupBuilder::agent_slug)
    /// - [`dataset_version`](DatasetCheckGroupBuilder::dataset_version)
    /// - [`failed`](DatasetCheckGroupBuilder::failed)
    /// - [`failures`](DatasetCheckGroupBuilder::failures)
    /// - [`not_started`](DatasetCheckGroupBuilder::not_started)
    /// - [`passed`](DatasetCheckGroupBuilder::passed)
    /// - [`running`](DatasetCheckGroupBuilder::running)
    /// - [`total`](DatasetCheckGroupBuilder::total)
    pub fn build(self) -> Result<DatasetCheckGroup, BuildError> {
        Ok(DatasetCheckGroup {
            agent_slug: self.agent_slug.ok_or_else(|| BuildError::missing_field("agent_slug"))?,
            agent_version: self.agent_version,
            dataset_version: self.dataset_version.ok_or_else(|| BuildError::missing_field("dataset_version"))?,
            failed: self.failed.ok_or_else(|| BuildError::missing_field("failed"))?,
            failures: self.failures.ok_or_else(|| BuildError::missing_field("failures"))?,
            not_started: self.not_started.ok_or_else(|| BuildError::missing_field("not_started"))?,
            pass_rate: self.pass_rate,
            passed: self.passed.ok_or_else(|| BuildError::missing_field("passed"))?,
            running: self.running.ok_or_else(|| BuildError::missing_field("running"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
