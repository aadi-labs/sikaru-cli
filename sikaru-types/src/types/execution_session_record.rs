pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExecutionSessionRecord {
    #[serde(rename = "activeRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_run_id: Option<String>,
    #[serde(rename = "agentSlug")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_slug: Option<String>,
    #[serde(rename = "autoImprove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_improve: Option<bool>,
    #[serde(rename = "channelOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_origin: Option<ExecutionSessionRecordChannelOrigin>,
    #[serde(rename = "contentVisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_visible: Option<bool>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "draftRevision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_revision: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ExecutionSessionRecordEnvironment>,
    #[serde(rename = "finalOutputSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_output_schema: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "harnessId")]
    #[serde(default)]
    pub harness_id: String,
    #[serde(rename = "harnessVersionId")]
    #[serde(default)]
    pub harness_version_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "modelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(rename = "nextTurn")]
    #[serde(default)]
    pub next_turn: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<String>,
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    #[serde(rename = "reasoningEffort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ExecutionSessionRecord {
    pub fn builder() -> ExecutionSessionRecordBuilder {
        <ExecutionSessionRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionSessionRecordBuilder {
    active_run_id: Option<String>,
    agent_slug: Option<String>,
    auto_improve: Option<bool>,
    channel_origin: Option<ExecutionSessionRecordChannelOrigin>,
    content_visible: Option<bool>,
    created_at: Option<String>,
    draft_revision: Option<i64>,
    environment: Option<ExecutionSessionRecordEnvironment>,
    final_output_schema: Option<HashMap<String, serde_json::Value>>,
    harness_id: Option<String>,
    harness_version_id: Option<String>,
    id: Option<String>,
    model_id: Option<String>,
    next_turn: Option<i64>,
    personal: Option<bool>,
    preview: Option<String>,
    project_id: Option<String>,
    reasoning_effort: Option<String>,
}

impl ExecutionSessionRecordBuilder {
    pub fn active_run_id(mut self, value: impl Into<String>) -> Self {
        self.active_run_id = Some(value.into());
        self
    }

    pub fn agent_slug(mut self, value: impl Into<String>) -> Self {
        self.agent_slug = Some(value.into());
        self
    }

    pub fn auto_improve(mut self, value: bool) -> Self {
        self.auto_improve = Some(value);
        self
    }

    pub fn channel_origin(mut self, value: ExecutionSessionRecordChannelOrigin) -> Self {
        self.channel_origin = Some(value);
        self
    }

    pub fn content_visible(mut self, value: bool) -> Self {
        self.content_visible = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn draft_revision(mut self, value: i64) -> Self {
        self.draft_revision = Some(value);
        self
    }

    pub fn environment(mut self, value: ExecutionSessionRecordEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn final_output_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.final_output_schema = Some(value);
        self
    }

    pub fn harness_id(mut self, value: impl Into<String>) -> Self {
        self.harness_id = Some(value.into());
        self
    }

    pub fn harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.harness_version_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn model_id(mut self, value: impl Into<String>) -> Self {
        self.model_id = Some(value.into());
        self
    }

    pub fn next_turn(mut self, value: i64) -> Self {
        self.next_turn = Some(value);
        self
    }

    pub fn personal(mut self, value: bool) -> Self {
        self.personal = Some(value);
        self
    }

    pub fn preview(mut self, value: impl Into<String>) -> Self {
        self.preview = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn reasoning_effort(mut self, value: impl Into<String>) -> Self {
        self.reasoning_effort = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExecutionSessionRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ExecutionSessionRecordBuilder::created_at)
    /// - [`harness_id`](ExecutionSessionRecordBuilder::harness_id)
    /// - [`harness_version_id`](ExecutionSessionRecordBuilder::harness_version_id)
    /// - [`id`](ExecutionSessionRecordBuilder::id)
    /// - [`next_turn`](ExecutionSessionRecordBuilder::next_turn)
    /// - [`project_id`](ExecutionSessionRecordBuilder::project_id)
    pub fn build(self) -> Result<ExecutionSessionRecord, BuildError> {
        Ok(ExecutionSessionRecord {
            active_run_id: self.active_run_id,
            agent_slug: self.agent_slug,
            auto_improve: self.auto_improve,
            channel_origin: self.channel_origin,
            content_visible: self.content_visible,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            draft_revision: self.draft_revision,
            environment: self.environment,
            final_output_schema: self.final_output_schema,
            harness_id: self.harness_id.ok_or_else(|| BuildError::missing_field("harness_id"))?,
            harness_version_id: self.harness_version_id.ok_or_else(|| BuildError::missing_field("harness_version_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            model_id: self.model_id,
            next_turn: self.next_turn.ok_or_else(|| BuildError::missing_field("next_turn"))?,
            personal: self.personal,
            preview: self.preview,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            reasoning_effort: self.reasoning_effort,
            extra: Default::default(),
        })
    }
}
