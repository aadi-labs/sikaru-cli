pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEvaluatorRunRequest {
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "backfillWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfill_window: Option<String>,
    #[serde(rename = "datasetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "evaluatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluator_name: Option<String>,
    #[serde(rename = "filterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    #[serde(rename = "issueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    #[serde(rename = "issueTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_title: Option<String>,
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<String>,
    #[serde(rename = "signalTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_tags: Option<Vec<String>>,
    #[serde(rename = "traceIds")]
    #[serde(default)]
    pub trace_ids: Vec<String>,
}

impl CreateEvaluatorRunRequest {
    pub fn builder() -> CreateEvaluatorRunRequestBuilder {
        <CreateEvaluatorRunRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEvaluatorRunRequestBuilder {
    agent_id: Option<String>,
    backfill_window: Option<String>,
    dataset_name: Option<String>,
    evaluator_name: Option<String>,
    filter_expression: Option<String>,
    issue_id: Option<String>,
    issue_title: Option<String>,
    sample_rate: Option<String>,
    signal_tags: Option<Vec<String>>,
    trace_ids: Option<Vec<String>>,
}

impl CreateEvaluatorRunRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn backfill_window(mut self, value: impl Into<String>) -> Self {
        self.backfill_window = Some(value.into());
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

    pub fn filter_expression(mut self, value: impl Into<String>) -> Self {
        self.filter_expression = Some(value.into());
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

    pub fn sample_rate(mut self, value: impl Into<String>) -> Self {
        self.sample_rate = Some(value.into());
        self
    }

    pub fn signal_tags(mut self, value: Vec<String>) -> Self {
        self.signal_tags = Some(value);
        self
    }

    pub fn trace_ids(mut self, value: Vec<String>) -> Self {
        self.trace_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateEvaluatorRunRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trace_ids`](CreateEvaluatorRunRequestBuilder::trace_ids)
    pub fn build(self) -> Result<CreateEvaluatorRunRequest, BuildError> {
        Ok(CreateEvaluatorRunRequest {
            agent_id: self.agent_id,
            backfill_window: self.backfill_window,
            dataset_name: self.dataset_name,
            evaluator_name: self.evaluator_name,
            filter_expression: self.filter_expression,
            issue_id: self.issue_id,
            issue_title: self.issue_title,
            sample_rate: self.sample_rate,
            signal_tags: self.signal_tags,
            trace_ids: self.trace_ids.ok_or_else(|| BuildError::missing_field("trace_ids"))?,
        })
    }
}

