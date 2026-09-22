pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TraceStreamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,
    pub format: TraceStreamRequestFormat,
    #[serde(default)]
    pub metadata: TraceMetadata,
    #[serde(rename = "resourceSpans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_spans: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spans: Option<Vec<OpenInferenceSpan>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trajectories: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<HashMap<String, serde_json::Value>>,
}

impl TraceStreamRequest {
    pub fn builder() -> TraceStreamRequestBuilder {
        <TraceStreamRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceStreamRequestBuilder {
    dataset: Option<String>,
    format: Option<TraceStreamRequestFormat>,
    metadata: Option<TraceMetadata>,
    resource_spans: Option<Vec<HashMap<String, serde_json::Value>>>,
    spans: Option<Vec<OpenInferenceSpan>>,
    trajectories: Option<Vec<HashMap<String, serde_json::Value>>>,
    trajectory: Option<HashMap<String, serde_json::Value>>,
}

impl TraceStreamRequestBuilder {
    pub fn dataset(mut self, value: impl Into<String>) -> Self {
        self.dataset = Some(value.into());
        self
    }

    pub fn format(mut self, value: TraceStreamRequestFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn metadata(mut self, value: TraceMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn resource_spans(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.resource_spans = Some(value);
        self
    }

    pub fn spans(mut self, value: Vec<OpenInferenceSpan>) -> Self {
        self.spans = Some(value);
        self
    }

    pub fn trajectories(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.trajectories = Some(value);
        self
    }

    pub fn trajectory(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.trajectory = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TraceStreamRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`format`](TraceStreamRequestBuilder::format)
    /// - [`metadata`](TraceStreamRequestBuilder::metadata)
    pub fn build(self) -> Result<TraceStreamRequest, BuildError> {
        Ok(TraceStreamRequest {
            dataset: self.dataset,
            format: self.format.ok_or_else(|| BuildError::missing_field("format"))?,
            metadata: self.metadata.ok_or_else(|| BuildError::missing_field("metadata"))?,
            resource_spans: self.resource_spans,
            spans: self.spans,
            trajectories: self.trajectories,
            trajectory: self.trajectory,
        })
    }
}

