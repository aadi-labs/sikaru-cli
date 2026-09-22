pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateAgentImportRequest {
    #[serde(rename = "evalSuites")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_suites: Option<Vec<AgentImportEvalSuiteRequest>>,
    #[serde(default)]
    pub improve: AgentImportImproveRequest,
    #[serde(rename = "modelCapture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_capture: Option<AgentImportModelCaptureRequest>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner: Option<AgentImportRunnerRequest>,
    #[serde(rename = "sourceRefs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_refs: Option<Vec<AgentImportSourceRefRequest>>,
}

impl CreateAgentImportRequest {
    pub fn builder() -> CreateAgentImportRequestBuilder {
        <CreateAgentImportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentImportRequestBuilder {
    eval_suites: Option<Vec<AgentImportEvalSuiteRequest>>,
    improve: Option<AgentImportImproveRequest>,
    model_capture: Option<AgentImportModelCaptureRequest>,
    name: Option<String>,
    runner: Option<AgentImportRunnerRequest>,
    source_refs: Option<Vec<AgentImportSourceRefRequest>>,
}

impl CreateAgentImportRequestBuilder {
    pub fn eval_suites(mut self, value: Vec<AgentImportEvalSuiteRequest>) -> Self {
        self.eval_suites = Some(value);
        self
    }

    pub fn improve(mut self, value: AgentImportImproveRequest) -> Self {
        self.improve = Some(value);
        self
    }

    pub fn model_capture(mut self, value: AgentImportModelCaptureRequest) -> Self {
        self.model_capture = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn runner(mut self, value: AgentImportRunnerRequest) -> Self {
        self.runner = Some(value);
        self
    }

    pub fn source_refs(mut self, value: Vec<AgentImportSourceRefRequest>) -> Self {
        self.source_refs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentImportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`improve`](CreateAgentImportRequestBuilder::improve)
    /// - [`name`](CreateAgentImportRequestBuilder::name)
    pub fn build(self) -> Result<CreateAgentImportRequest, BuildError> {
        Ok(CreateAgentImportRequest {
            eval_suites: self.eval_suites,
            improve: self.improve.ok_or_else(|| BuildError::missing_field("improve"))?,
            model_capture: self.model_capture,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            runner: self.runner,
            source_refs: self.source_refs,
        })
    }
}

