pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateRetentionPolicyUpdateRequest {
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "backfillWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backfill_window: Option<String>,
    #[serde(rename = "filterExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_expression: Option<String>,
    #[serde(rename = "issueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_id: Option<String>,
    #[serde(rename = "issueTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_title: Option<String>,
    #[serde(rename = "retentionDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i64>,
    #[serde(rename = "ruleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "ruleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<String>,
    #[serde(rename = "signalTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "traceIds")]
    #[serde(default)]
    pub trace_ids: Vec<String>,
}

impl CreateRetentionPolicyUpdateRequest {
    pub fn builder() -> CreateRetentionPolicyUpdateRequestBuilder {
        <CreateRetentionPolicyUpdateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRetentionPolicyUpdateRequestBuilder {
    agent_id: Option<String>,
    backfill_window: Option<String>,
    filter_expression: Option<String>,
    issue_id: Option<String>,
    issue_title: Option<String>,
    retention_days: Option<i64>,
    rule_id: Option<String>,
    rule_name: Option<String>,
    sample_rate: Option<String>,
    signal_tags: Option<Vec<String>>,
    target: Option<String>,
    trace_ids: Option<Vec<String>>,
}

impl CreateRetentionPolicyUpdateRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn backfill_window(mut self, value: impl Into<String>) -> Self {
        self.backfill_window = Some(value.into());
        self
    }

    pub fn filter_expression(mut self, value: impl Into<String>) -> Self {
        self.filter_expression = Some(value.into());
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

    pub fn retention_days(mut self, value: i64) -> Self {
        self.retention_days = Some(value);
        self
    }

    pub fn rule_id(mut self, value: impl Into<String>) -> Self {
        self.rule_id = Some(value.into());
        self
    }

    pub fn rule_name(mut self, value: impl Into<String>) -> Self {
        self.rule_name = Some(value.into());
        self
    }

    pub fn sample_rate(mut self, value: impl Into<String>) -> Self {
        self.sample_rate = Some(value.into());
        self
    }

    pub fn signal_tags(mut self, value: Vec<String>) -> Self {
        self.signal_tags = Some(value);
        self
    }

    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.target = Some(value.into());
        self
    }

    pub fn trace_ids(mut self, value: Vec<String>) -> Self {
        self.trace_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRetentionPolicyUpdateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trace_ids`](CreateRetentionPolicyUpdateRequestBuilder::trace_ids)
    pub fn build(self) -> Result<CreateRetentionPolicyUpdateRequest, BuildError> {
        Ok(CreateRetentionPolicyUpdateRequest {
            agent_id: self.agent_id,
            backfill_window: self.backfill_window,
            filter_expression: self.filter_expression,
            issue_id: self.issue_id,
            issue_title: self.issue_title,
            retention_days: self.retention_days,
            rule_id: self.rule_id,
            rule_name: self.rule_name,
            sample_rate: self.sample_rate,
            signal_tags: self.signal_tags,
            target: self.target,
            trace_ids: self.trace_ids.ok_or_else(|| BuildError::missing_field("trace_ids"))?,
        })
    }
}

