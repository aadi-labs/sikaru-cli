pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_results
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListResultsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ListResultsEvaluationResultsRequestEnvironment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verdict: Option<ListResultsEvaluationResultsRequestVerdict>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListResultsQueryRequest {
    pub fn builder() -> ListResultsQueryRequestBuilder {
        <ListResultsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResultsQueryRequestBuilder {
    environment: Option<ListResultsEvaluationResultsRequestEnvironment>,
    evaluator: Option<String>,
    verdict: Option<ListResultsEvaluationResultsRequestVerdict>,
    limit: Option<i64>,
    cursor: Option<String>,
}

impl ListResultsQueryRequestBuilder {
    pub fn environment(mut self, value: ListResultsEvaluationResultsRequestEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn evaluator(mut self, value: impl Into<String>) -> Self {
        self.evaluator = Some(value.into());
        self
    }

    pub fn verdict(mut self, value: ListResultsEvaluationResultsRequestVerdict) -> Self {
        self.verdict = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListResultsQueryRequest`].
    pub fn build(self) -> Result<ListResultsQueryRequest, BuildError> {
        Ok(ListResultsQueryRequest {
            environment: self.environment,
            evaluator: self.evaluator,
            verdict: self.verdict,
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}

