pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DatasetChecksPreview {
    #[serde(default)]
    pub agents: Vec<CheckAgent>,
    #[serde(rename = "averageRunCostUsd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub average_run_cost_usd: Option<f64>,
    #[serde(rename = "datasetVersion")]
    #[serde(default)]
    pub dataset_version: i64,
    #[serde(default)]
    pub eligible: i64,
    #[serde(rename = "estimatedCostUsd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub estimated_cost_usd: Option<f64>,
    #[serde(rename = "notEligible")]
    #[serde(default)]
    pub not_eligible: HashMap<String, i64>,
}

impl DatasetChecksPreview {
    pub fn builder() -> DatasetChecksPreviewBuilder {
        <DatasetChecksPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetChecksPreviewBuilder {
    agents: Option<Vec<CheckAgent>>,
    average_run_cost_usd: Option<f64>,
    dataset_version: Option<i64>,
    eligible: Option<i64>,
    estimated_cost_usd: Option<f64>,
    not_eligible: Option<HashMap<String, i64>>,
}

impl DatasetChecksPreviewBuilder {
    pub fn agents(mut self, value: Vec<CheckAgent>) -> Self {
        self.agents = Some(value);
        self
    }

    pub fn average_run_cost_usd(mut self, value: f64) -> Self {
        self.average_run_cost_usd = Some(value);
        self
    }

    pub fn dataset_version(mut self, value: i64) -> Self {
        self.dataset_version = Some(value);
        self
    }

    pub fn eligible(mut self, value: i64) -> Self {
        self.eligible = Some(value);
        self
    }

    pub fn estimated_cost_usd(mut self, value: f64) -> Self {
        self.estimated_cost_usd = Some(value);
        self
    }

    pub fn not_eligible(mut self, value: HashMap<String, i64>) -> Self {
        self.not_eligible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetChecksPreview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agents`](DatasetChecksPreviewBuilder::agents)
    /// - [`dataset_version`](DatasetChecksPreviewBuilder::dataset_version)
    /// - [`eligible`](DatasetChecksPreviewBuilder::eligible)
    /// - [`not_eligible`](DatasetChecksPreviewBuilder::not_eligible)
    pub fn build(self) -> Result<DatasetChecksPreview, BuildError> {
        Ok(DatasetChecksPreview {
            agents: self.agents.ok_or_else(|| BuildError::missing_field("agents"))?,
            average_run_cost_usd: self.average_run_cost_usd,
            dataset_version: self.dataset_version.ok_or_else(|| BuildError::missing_field("dataset_version"))?,
            eligible: self.eligible.ok_or_else(|| BuildError::missing_field("eligible"))?,
            estimated_cost_usd: self.estimated_cost_usd,
            not_eligible: self.not_eligible.ok_or_else(|| BuildError::missing_field("not_eligible"))?,
        })
    }
}
