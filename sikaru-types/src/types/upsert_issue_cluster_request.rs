pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpsertIssueClusterRequest {
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<i64>,
    #[serde(rename = "issueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    #[serde(rename = "issueTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_title: Option<String>,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "signalTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_tags: Option<Vec<String>>,
    #[serde(rename = "traceIds")]
    #[serde(default)]
    pub trace_ids: Vec<String>,
}

impl UpsertIssueClusterRequest {
    pub fn builder() -> UpsertIssueClusterRequestBuilder {
        <UpsertIssueClusterRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertIssueClusterRequestBuilder {
    agent_id: Option<String>,
    impact: Option<i64>,
    issue_id: Option<String>,
    issue_title: Option<String>,
    label: Option<String>,
    signal_tags: Option<Vec<String>>,
    trace_ids: Option<Vec<String>>,
}

impl UpsertIssueClusterRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn impact(mut self, value: i64) -> Self {
        self.impact = Some(value);
        self
    }

    pub fn issue_id(mut self, value: impl Into<String>) -> Self {
        self.issue_id = Some(value.into());
        self
    }

    pub fn issue_title(mut self, value: impl Into<String>) -> Self {
        self.issue_title = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn signal_tags(mut self, value: Vec<String>) -> Self {
        self.signal_tags = Some(value);
        self
    }

    pub fn trace_ids(mut self, value: Vec<String>) -> Self {
        self.trace_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpsertIssueClusterRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](UpsertIssueClusterRequestBuilder::label)
    /// - [`trace_ids`](UpsertIssueClusterRequestBuilder::trace_ids)
    pub fn build(self) -> Result<UpsertIssueClusterRequest, BuildError> {
        Ok(UpsertIssueClusterRequest {
            agent_id: self.agent_id,
            impact: self.impact,
            issue_id: self.issue_id,
            issue_title: self.issue_title,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            signal_tags: self.signal_tags,
            trace_ids: self.trace_ids.ok_or_else(|| BuildError::missing_field("trace_ids"))?,
        })
    }
}

