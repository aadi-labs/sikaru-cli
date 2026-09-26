pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEvalSeedRequest {
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "datasetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "evaluatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluator_name: Option<String>,
    #[serde(rename = "issueId")]
    #[serde(default)]
    pub issue_id: String,
    #[serde(rename = "issueTitle")]
    #[serde(default)]
    pub issue_title: String,
    #[serde(rename = "traceIds")]
    #[serde(default)]
    pub trace_ids: Vec<String>,
}

impl CreateEvalSeedRequest {
    pub fn builder() -> CreateEvalSeedRequestBuilder {
        <CreateEvalSeedRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEvalSeedRequestBuilder {
    agent_id: Option<String>,
    dataset_name: Option<String>,
    evaluator_name: Option<String>,
    issue_id: Option<String>,
    issue_title: Option<String>,
    trace_ids: Option<Vec<String>>,
}

impl CreateEvalSeedRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn dataset_name(mut self, value: impl Into<String>) -> Self {
        self.dataset_name = Some(value.into());
        self
    }

    pub fn evaluator_name(mut self, value: impl Into<String>) -> Self {
        self.evaluator_name = Some(value.into());
        self
    }

    pub fn issue_id(mut self, value: impl Into<String>) -> Self {
        self.issue_id = Some(value.into());
        self
    }

    pub fn issue_title(mut self, value: impl Into<String>) -> Self {
        self.issue_title = Some(value.into());
        self
    }

    pub fn trace_ids(mut self, value: Vec<String>) -> Self {
        self.trace_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateEvalSeedRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`issue_id`](CreateEvalSeedRequestBuilder::issue_id)
    /// - [`issue_title`](CreateEvalSeedRequestBuilder::issue_title)
    /// - [`trace_ids`](CreateEvalSeedRequestBuilder::trace_ids)
    pub fn build(self) -> Result<CreateEvalSeedRequest, BuildError> {
        Ok(CreateEvalSeedRequest {
            agent_id: self.agent_id,
            dataset_name: self.dataset_name,
            evaluator_name: self.evaluator_name,
            issue_id: self.issue_id.ok_or_else(|| BuildError::missing_field("issue_id"))?,
            issue_title: self.issue_title.ok_or_else(|| BuildError::missing_field("issue_title"))?,
            trace_ids: self.trace_ids.ok_or_else(|| BuildError::missing_field("trace_ids"))?,
        })
    }
}

