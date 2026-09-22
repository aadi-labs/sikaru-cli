pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OpenInferenceSpan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openinference_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_span_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_attributes: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub span_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(default)]
    pub trace_id: String,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl OpenInferenceSpan {
    pub fn builder() -> OpenInferenceSpanBuilder {
        <OpenInferenceSpanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenInferenceSpanBuilder {
    attributes: Option<HashMap<String, serde_json::Value>>,
    end_time: Option<String>,
    name: Option<String>,
    openinference_kind: Option<String>,
    parent_span_id: Option<String>,
    raw: Option<HashMap<String, serde_json::Value>>,
    resource_attributes: Option<HashMap<String, serde_json::Value>>,
    span_id: Option<String>,
    start_time: Option<String>,
    status_code: Option<String>,
    status_message: Option<String>,
    trace_id: Option<String>,
}

impl OpenInferenceSpanBuilder {
    pub fn attributes(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn end_time(mut self, value: impl Into<String>) -> Self {
        self.end_time = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn openinference_kind(mut self, value: impl Into<String>) -> Self {
        self.openinference_kind = Some(value.into());
        self
    }

    pub fn parent_span_id(mut self, value: impl Into<String>) -> Self {
        self.parent_span_id = Some(value.into());
        self
    }

    pub fn raw(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.raw = Some(value);
        self
    }

    pub fn resource_attributes(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.resource_attributes = Some(value);
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn start_time(mut self, value: impl Into<String>) -> Self {
        self.start_time = Some(value.into());
        self
    }

    pub fn status_code(mut self, value: impl Into<String>) -> Self {
        self.status_code = Some(value.into());
        self
    }

    pub fn status_message(mut self, value: impl Into<String>) -> Self {
        self.status_message = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OpenInferenceSpan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`span_id`](OpenInferenceSpanBuilder::span_id)
    /// - [`trace_id`](OpenInferenceSpanBuilder::trace_id)
    pub fn build(self) -> Result<OpenInferenceSpan, BuildError> {
        Ok(OpenInferenceSpan {
            attributes: self.attributes,
            end_time: self.end_time,
            name: self.name,
            openinference_kind: self.openinference_kind,
            parent_span_id: self.parent_span_id,
            raw: self.raw,
            resource_attributes: self.resource_attributes,
            span_id: self.span_id.ok_or_else(|| BuildError::missing_field("span_id"))?,
            start_time: self.start_time,
            status_code: self.status_code,
            status_message: self.status_message,
            trace_id: self.trace_id.ok_or_else(|| BuildError::missing_field("trace_id"))?,
            extra: Default::default(),
        })
    }
}
