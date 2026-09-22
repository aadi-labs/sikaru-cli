pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ImportWorkflowRequest {
    #[serde(default)]
    pub payload: HashMap<String, serde_json::Value>,
    #[serde(rename = "sourceArtifactId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_artifact_id: Option<String>,
    #[serde(rename = "workflowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

impl ImportWorkflowRequest {
    pub fn builder() -> ImportWorkflowRequestBuilder {
        <ImportWorkflowRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImportWorkflowRequestBuilder {
    payload: Option<HashMap<String, serde_json::Value>>,
    source_artifact_id: Option<String>,
    workflow_id: Option<String>,
}

impl ImportWorkflowRequestBuilder {
    pub fn payload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn source_artifact_id(mut self, value: impl Into<String>) -> Self {
        self.source_artifact_id = Some(value.into());
        self
    }

    pub fn workflow_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ImportWorkflowRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payload`](ImportWorkflowRequestBuilder::payload)
    pub fn build(self) -> Result<ImportWorkflowRequest, BuildError> {
        Ok(ImportWorkflowRequest {
            payload: self.payload.ok_or_else(|| BuildError::missing_field("payload"))?,
            source_artifact_id: self.source_artifact_id,
            workflow_id: self.workflow_id,
        })
    }
}

