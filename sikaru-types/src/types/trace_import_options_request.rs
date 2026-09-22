pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TraceImportOptionsRequest {
    #[serde(rename = "includeFeedback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_feedback: Option<bool>,
    #[serde(rename = "includePendingSpans")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_pending_spans: Option<bool>,
    #[serde(rename = "inferAgentBoundaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_agent_boundaries: Option<bool>,
    #[serde(rename = "inferToolCallLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infer_tool_call_links: Option<bool>,
}

impl TraceImportOptionsRequest {
    pub fn builder() -> TraceImportOptionsRequestBuilder {
        <TraceImportOptionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceImportOptionsRequestBuilder {
    include_feedback: Option<bool>,
    include_pending_spans: Option<bool>,
    infer_agent_boundaries: Option<bool>,
    infer_tool_call_links: Option<bool>,
}

impl TraceImportOptionsRequestBuilder {
    pub fn include_feedback(mut self, value: bool) -> Self {
        self.include_feedback = Some(value);
        self
    }

    pub fn include_pending_spans(mut self, value: bool) -> Self {
        self.include_pending_spans = Some(value);
        self
    }

    pub fn infer_agent_boundaries(mut self, value: bool) -> Self {
        self.infer_agent_boundaries = Some(value);
        self
    }

    pub fn infer_tool_call_links(mut self, value: bool) -> Self {
        self.infer_tool_call_links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TraceImportOptionsRequest`].
    pub fn build(self) -> Result<TraceImportOptionsRequest, BuildError> {
        Ok(TraceImportOptionsRequest {
            include_feedback: self.include_feedback,
            include_pending_spans: self.include_pending_spans,
            infer_agent_boundaries: self.infer_agent_boundaries,
            infer_tool_call_links: self.infer_tool_call_links,
        })
    }
}
