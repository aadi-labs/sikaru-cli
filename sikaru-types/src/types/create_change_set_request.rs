pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateChangeSetRequest {
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "baseReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_release_id: Option<String>,
    #[serde(rename = "candidateReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_release_id: Option<String>,
    #[serde(rename = "publicSummary")]
    #[serde(default)]
    pub public_summary: String,
    #[serde(rename = "publicTitle")]
    #[serde(default)]
    pub public_title: String,
    #[serde(rename = "scopeRef")]
    #[serde(default)]
    pub scope_ref: String,
    #[serde(rename = "scopeType")]
    pub scope_type: ScopeType,
    #[serde(rename = "sourceType")]
    pub source_type: SourceType,
    #[serde(rename = "workflowId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

impl CreateChangeSetRequest {
    pub fn builder() -> CreateChangeSetRequestBuilder {
        <CreateChangeSetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateChangeSetRequestBuilder {
    agent_id: Option<String>,
    base_release_id: Option<String>,
    candidate_release_id: Option<String>,
    public_summary: Option<String>,
    public_title: Option<String>,
    scope_ref: Option<String>,
    scope_type: Option<ScopeType>,
    source_type: Option<SourceType>,
    workflow_id: Option<String>,
}

impl CreateChangeSetRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn base_release_id(mut self, value: impl Into<String>) -> Self {
        self.base_release_id = Some(value.into());
        self
    }

    pub fn candidate_release_id(mut self, value: impl Into<String>) -> Self {
        self.candidate_release_id = Some(value.into());
        self
    }

    pub fn public_summary(mut self, value: impl Into<String>) -> Self {
        self.public_summary = Some(value.into());
        self
    }

    pub fn public_title(mut self, value: impl Into<String>) -> Self {
        self.public_title = Some(value.into());
        self
    }

    pub fn scope_ref(mut self, value: impl Into<String>) -> Self {
        self.scope_ref = Some(value.into());
        self
    }

    pub fn scope_type(mut self, value: ScopeType) -> Self {
        self.scope_type = Some(value);
        self
    }

    pub fn source_type(mut self, value: SourceType) -> Self {
        self.source_type = Some(value);
        self
    }

    pub fn workflow_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateChangeSetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`public_summary`](CreateChangeSetRequestBuilder::public_summary)
    /// - [`public_title`](CreateChangeSetRequestBuilder::public_title)
    /// - [`scope_ref`](CreateChangeSetRequestBuilder::scope_ref)
    /// - [`scope_type`](CreateChangeSetRequestBuilder::scope_type)
    /// - [`source_type`](CreateChangeSetRequestBuilder::source_type)
    pub fn build(self) -> Result<CreateChangeSetRequest, BuildError> {
        Ok(CreateChangeSetRequest {
            agent_id: self.agent_id,
            base_release_id: self.base_release_id,
            candidate_release_id: self.candidate_release_id,
            public_summary: self.public_summary.ok_or_else(|| BuildError::missing_field("public_summary"))?,
            public_title: self.public_title.ok_or_else(|| BuildError::missing_field("public_title"))?,
            scope_ref: self.scope_ref.ok_or_else(|| BuildError::missing_field("scope_ref"))?,
            scope_type: self.scope_type.ok_or_else(|| BuildError::missing_field("scope_type"))?,
            source_type: self.source_type.ok_or_else(|| BuildError::missing_field("source_type"))?,
            workflow_id: self.workflow_id,
        })
    }
}

