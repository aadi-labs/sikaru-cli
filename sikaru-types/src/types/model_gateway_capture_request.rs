pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModelGatewayCaptureRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub request: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<HashMap<String, serde_json::Value>>,
}

impl ModelGatewayCaptureRequest {
    pub fn builder() -> ModelGatewayCaptureRequestBuilder {
        <ModelGatewayCaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelGatewayCaptureRequestBuilder {
    error: Option<HashMap<String, serde_json::Value>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    request: Option<HashMap<String, serde_json::Value>>,
    response: Option<HashMap<String, serde_json::Value>>,
}

impl ModelGatewayCaptureRequestBuilder {
    pub fn error(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.error = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn request(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.request = Some(value);
        self
    }

    pub fn response(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModelGatewayCaptureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request`](ModelGatewayCaptureRequestBuilder::request)
    pub fn build(self) -> Result<ModelGatewayCaptureRequest, BuildError> {
        Ok(ModelGatewayCaptureRequest {
            error: self.error,
            metadata: self.metadata,
            request: self.request.ok_or_else(|| BuildError::missing_field("request"))?,
            response: self.response,
        })
    }
}

