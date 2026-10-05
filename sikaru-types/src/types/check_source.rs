pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The dataset example a check was compiled from; datasetName is None once the dataset is deleted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckSource {
    #[serde(rename = "agentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "datasetId")]
    #[serde(default)]
    pub dataset_id: String,
    #[serde(rename = "datasetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "datasetVersion")]
    #[serde(default)]
    pub dataset_version: i64,
    #[serde(rename = "exampleId")]
    #[serde(default)]
    pub example_id: String,
}

impl CheckSource {
    pub fn builder() -> CheckSourceBuilder {
        <CheckSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckSourceBuilder {
    agent_version: Option<String>,
    dataset_id: Option<String>,
    dataset_name: Option<String>,
    dataset_version: Option<i64>,
    example_id: Option<String>,
}

impl CheckSourceBuilder {
    pub fn agent_version(mut self, value: impl Into<String>) -> Self {
        self.agent_version = Some(value.into());
        self
    }

    pub fn dataset_id(mut self, value: impl Into<String>) -> Self {
        self.dataset_id = Some(value.into());
        self
    }

    pub fn dataset_name(mut self, value: impl Into<String>) -> Self {
        self.dataset_name = Some(value.into());
        self
    }

    pub fn dataset_version(mut self, value: i64) -> Self {
        self.dataset_version = Some(value);
        self
    }

    pub fn example_id(mut self, value: impl Into<String>) -> Self {
        self.example_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dataset_id`](CheckSourceBuilder::dataset_id)
    /// - [`dataset_version`](CheckSourceBuilder::dataset_version)
    /// - [`example_id`](CheckSourceBuilder::example_id)
    pub fn build(self) -> Result<CheckSource, BuildError> {
        Ok(CheckSource {
            agent_version: self.agent_version,
            dataset_id: self.dataset_id.ok_or_else(|| BuildError::missing_field("dataset_id"))?,
            dataset_name: self.dataset_name,
            dataset_version: self.dataset_version.ok_or_else(|| BuildError::missing_field("dataset_version"))?,
            example_id: self.example_id.ok_or_else(|| BuildError::missing_field("example_id"))?,
        })
    }
}
