pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateFeedbackRequest {
    #[serde(rename = "issueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    pub kind: CreateFeedbackRequestKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "spanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub target: CreateFeedbackRequestTarget,
    #[serde(rename = "targetId")]
    #[serde(default)]
    pub target_id: String,
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

impl CreateFeedbackRequest {
    pub fn builder() -> CreateFeedbackRequestBuilder {
        <CreateFeedbackRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFeedbackRequestBuilder {
    issue_id: Option<String>,
    kind: Option<CreateFeedbackRequestKind>,
    note: Option<String>,
    session_id: Option<String>,
    span_id: Option<String>,
    tag: Option<String>,
    target: Option<CreateFeedbackRequestTarget>,
    target_id: Option<String>,
    trace_id: Option<String>,
}

impl CreateFeedbackRequestBuilder {
    pub fn issue_id(mut self, value: impl Into<String>) -> Self {
        self.issue_id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: CreateFeedbackRequestKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    pub fn target(mut self, value: CreateFeedbackRequestTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn target_id(mut self, value: impl Into<String>) -> Self {
        self.target_id = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateFeedbackRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](CreateFeedbackRequestBuilder::kind)
    /// - [`target`](CreateFeedbackRequestBuilder::target)
    /// - [`target_id`](CreateFeedbackRequestBuilder::target_id)
    pub fn build(self) -> Result<CreateFeedbackRequest, BuildError> {
        Ok(CreateFeedbackRequest {
            issue_id: self.issue_id,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            note: self.note,
            session_id: self.session_id,
            span_id: self.span_id,
            tag: self.tag,
            target: self.target.ok_or_else(|| BuildError::missing_field("target"))?,
            target_id: self.target_id.ok_or_else(|| BuildError::missing_field("target_id"))?,
            trace_id: self.trace_id,
        })
    }
}

