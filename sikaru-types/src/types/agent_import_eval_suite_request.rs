pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentImportEvalSuiteRequest {
    #[serde(default)]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "primaryMetric")]
    #[serde(default)]
    pub primary_metric: String,
    #[serde(rename = "storageRef")]
    #[serde(default)]
    pub storage_ref: String,
}

impl AgentImportEvalSuiteRequest {
    pub fn builder() -> AgentImportEvalSuiteRequestBuilder {
        <AgentImportEvalSuiteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentImportEvalSuiteRequestBuilder {
    kind: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    primary_metric: Option<String>,
    storage_ref: Option<String>,
}

impl AgentImportEvalSuiteRequestBuilder {
    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn primary_metric(mut self, value: impl Into<String>) -> Self {
        self.primary_metric = Some(value.into());
        self
    }

    pub fn storage_ref(mut self, value: impl Into<String>) -> Self {
        self.storage_ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentImportEvalSuiteRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](AgentImportEvalSuiteRequestBuilder::kind)
    /// - [`primary_metric`](AgentImportEvalSuiteRequestBuilder::primary_metric)
    /// - [`storage_ref`](AgentImportEvalSuiteRequestBuilder::storage_ref)
    pub fn build(self) -> Result<AgentImportEvalSuiteRequest, BuildError> {
        Ok(AgentImportEvalSuiteRequest {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            metadata: self.metadata,
            primary_metric: self.primary_metric.ok_or_else(|| BuildError::missing_field("primary_metric"))?,
            storage_ref: self.storage_ref.ok_or_else(|| BuildError::missing_field("storage_ref"))?,
        })
    }
}
