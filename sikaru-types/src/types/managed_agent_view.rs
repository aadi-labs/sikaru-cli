pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ManagedAgentView {
    #[serde(rename = "activeHarnessVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_harness_version_id: Option<String>,
    #[serde(rename = "agentSlug")]
    #[serde(default)]
    pub agent_slug: String,
    #[serde(rename = "compatibilityProfileId")]
    #[serde(default)]
    pub compatibility_profile_id: String,
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(rename = "harnessId")]
    #[serde(default)]
    pub harness_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "projectId")]
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub status: String,
}

impl ManagedAgentView {
    pub fn builder() -> ManagedAgentViewBuilder {
        <ManagedAgentViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManagedAgentViewBuilder {
    active_harness_version_id: Option<String>,
    agent_slug: Option<String>,
    compatibility_profile_id: Option<String>,
    display_name: Option<String>,
    harness_id: Option<String>,
    id: Option<String>,
    organization_id: Option<String>,
    project_id: Option<String>,
    status: Option<String>,
}

impl ManagedAgentViewBuilder {
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ManagedAgentView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_slug`](ManagedAgentViewBuilder::agent_slug)
    /// - [`compatibility_profile_id`](ManagedAgentViewBuilder::compatibility_profile_id)
    /// - [`display_name`](ManagedAgentViewBuilder::display_name)
    /// - [`harness_id`](ManagedAgentViewBuilder::harness_id)
    /// - [`id`](ManagedAgentViewBuilder::id)
    /// - [`organization_id`](ManagedAgentViewBuilder::organization_id)
    /// - [`project_id`](ManagedAgentViewBuilder::project_id)
    /// - [`status`](ManagedAgentViewBuilder::status)
    pub fn build(self) -> Result<ManagedAgentView, BuildError> {
        Ok(ManagedAgentView {
            active_harness_version_id: self.active_harness_version_id,
            agent_slug: self.agent_slug.ok_or_else(|| BuildError::missing_field("agent_slug"))?,
            compatibility_profile_id: self.compatibility_profile_id.ok_or_else(|| BuildError::missing_field("compatibility_profile_id"))?,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            harness_id: self.harness_id.ok_or_else(|| BuildError::missing_field("harness_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
