pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StartWorkflowRunRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "workflowVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_version_id: Option<String>,
}

impl StartWorkflowRunRequest {
    pub fn builder() -> StartWorkflowRunRequestBuilder {
        <StartWorkflowRunRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartWorkflowRunRequestBuilder {
    input: Option<HashMap<String, serde_json::Value>>,
    workflow_version_id: Option<String>,
}

impl StartWorkflowRunRequestBuilder {
    pub fn input(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input = Some(value);
        self
    }

    pub fn workflow_version_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StartWorkflowRunRequest`].
    pub fn build(self) -> Result<StartWorkflowRunRequest, BuildError> {
        Ok(StartWorkflowRunRequest {
            input: self.input,
            workflow_version_id: self.workflow_version_id,
        })
    }
}

