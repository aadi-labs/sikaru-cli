pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateManagedAgentRequest {
    #[serde(rename = "activeHarnessVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_harness_version_id: Option<String>,
    #[serde(rename = "agentSlug")]
    #[serde(default)]
    pub agent_slug: String,
    #[serde(rename = "compatibilityProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_profile_id: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "harnessId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harness_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<CreateManagedAgentSourceRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreateManagedAgentRequestStatus>,
}

impl CreateManagedAgentRequest {
    pub fn builder() -> CreateManagedAgentRequestBuilder {
        <CreateManagedAgentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateManagedAgentRequestBuilder {
    active_harness_version_id: Option<String>,
    agent_slug: Option<String>,
    compatibility_profile_id: Option<String>,
    display_name: Option<String>,
    harness_id: Option<String>,
    source: Option<CreateManagedAgentSourceRequest>,
    status: Option<CreateManagedAgentRequestStatus>,
}

impl CreateManagedAgentRequestBuilder {
    pub fn active_harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.active_harness_version_id = Some(value.into());
        self
    }

    pub fn agent_slug(mut self, value: impl Into<String>) -> Self {
        self.agent_slug = Some(value.into());
        self
    }

    pub fn compatibility_profile_id(mut self, value: impl Into<String>) -> Self {
        self.compatibility_profile_id = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn harness_id(mut self, value: impl Into<String>) -> Self {
        self.harness_id = Some(value.into());
        self
    }

    pub fn source(mut self, value: CreateManagedAgentSourceRequest) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(mut self, value: CreateManagedAgentRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateManagedAgentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_slug`](CreateManagedAgentRequestBuilder::agent_slug)
    pub fn build(self) -> Result<CreateManagedAgentRequest, BuildError> {
        Ok(CreateManagedAgentRequest {
            active_harness_version_id: self.active_harness_version_id,
            agent_slug: self.agent_slug.ok_or_else(|| BuildError::missing_field("agent_slug"))?,
            compatibility_profile_id: self.compatibility_profile_id,
            display_name: self.display_name,
            harness_id: self.harness_id,
            source: self.source,
            status: self.status,
        })
    }
}

