pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get_judge_alignment
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetJudgeAlignmentQueryRequest {
    #[serde(default)]
    pub evaluator: String,
    #[serde(default)]
    pub revision: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<GetJudgeAlignmentJudgeAlignmentRequestEnvironment>,
}

impl GetJudgeAlignmentQueryRequest {
    pub fn builder() -> GetJudgeAlignmentQueryRequestBuilder {
        <GetJudgeAlignmentQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetJudgeAlignmentQueryRequestBuilder {
    evaluator: Option<String>,
    revision: Option<String>,
    environment: Option<GetJudgeAlignmentJudgeAlignmentRequestEnvironment>,
}

impl GetJudgeAlignmentQueryRequestBuilder {
    pub fn evaluator(mut self, value: impl Into<String>) -> Self {
        self.evaluator = Some(value.into());
        self
    }

    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn environment(mut self, value: GetJudgeAlignmentJudgeAlignmentRequestEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetJudgeAlignmentQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evaluator`](GetJudgeAlignmentQueryRequestBuilder::evaluator)
    /// - [`revision`](GetJudgeAlignmentQueryRequestBuilder::revision)
    pub fn build(self) -> Result<GetJudgeAlignmentQueryRequest, BuildError> {
        Ok(GetJudgeAlignmentQueryRequest {
            evaluator: self.evaluator.ok_or_else(|| BuildError::missing_field("evaluator"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
            environment: self.environment,
        })
    }
}

