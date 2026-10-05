pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DatasetChecksStarted {
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(default)]
    pub checks: Vec<StartedCheck>,
    #[serde(rename = "datasetVersion")]
    #[serde(default)]
    pub dataset_version: i64,
    #[serde(rename = "notEligible")]
    #[serde(default)]
    pub not_eligible: HashMap<String, i64>,
}

impl DatasetChecksStarted {
    pub fn builder() -> DatasetChecksStartedBuilder {
        <DatasetChecksStartedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetChecksStartedBuilder {
    agent_version: Option<String>,
    checks: Option<Vec<StartedCheck>>,
    dataset_version: Option<i64>,
    not_eligible: Option<HashMap<String, i64>>,
}

impl DatasetChecksStartedBuilder {
    pub fn agent_version(mut self, value: impl Into<String>) -> Self {
        self.agent_version = Some(value.into());
        self
    }

    pub fn checks(mut self, value: Vec<StartedCheck>) -> Self {
        self.checks = Some(value);
        self
    }

    pub fn dataset_version(mut self, value: i64) -> Self {
        self.dataset_version = Some(value);
        self
    }

    pub fn not_eligible(mut self, value: HashMap<String, i64>) -> Self {
        self.not_eligible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetChecksStarted`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_version`](DatasetChecksStartedBuilder::agent_version)
    /// - [`checks`](DatasetChecksStartedBuilder::checks)
    /// - [`dataset_version`](DatasetChecksStartedBuilder::dataset_version)
    /// - [`not_eligible`](DatasetChecksStartedBuilder::not_eligible)
    pub fn build(self) -> Result<DatasetChecksStarted, BuildError> {
        Ok(DatasetChecksStarted {
            agent_version: self.agent_version.ok_or_else(|| BuildError::missing_field("agent_version"))?,
            checks: self.checks.ok_or_else(|| BuildError::missing_field("checks"))?,
            dataset_version: self.dataset_version.ok_or_else(|| BuildError::missing_field("dataset_version"))?,
            not_eligible: self.not_eligible.ok_or_else(|| BuildError::missing_field("not_eligible"))?,
        })
    }
}
