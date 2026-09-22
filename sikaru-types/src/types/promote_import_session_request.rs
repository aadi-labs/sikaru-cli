pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromoteImportSessionRequest {
    #[serde(rename = "activeHarnessVersionId")]
    #[serde(default)]
    pub active_harness_version_id: String,
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
}

impl PromoteImportSessionRequest {
    pub fn builder() -> PromoteImportSessionRequestBuilder {
        <PromoteImportSessionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromoteImportSessionRequestBuilder {
    active_harness_version_id: Option<String>,
    agent_slug: Option<String>,
    compatibility_profile_id: Option<String>,
    display_name: Option<String>,
    harness_id: Option<String>,
}

impl PromoteImportSessionRequestBuilder {
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

    /// Consumes the builder and constructs a [`PromoteImportSessionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active_harness_version_id`](PromoteImportSessionRequestBuilder::active_harness_version_id)
    /// - [`agent_slug`](PromoteImportSessionRequestBuilder::agent_slug)
    /// - [`compatibility_profile_id`](PromoteImportSessionRequestBuilder::compatibility_profile_id)
    /// - [`display_name`](PromoteImportSessionRequestBuilder::display_name)
    /// - [`harness_id`](PromoteImportSessionRequestBuilder::harness_id)
    pub fn build(self) -> Result<PromoteImportSessionRequest, BuildError> {
        Ok(PromoteImportSessionRequest {
            active_harness_version_id: self.active_harness_version_id.ok_or_else(|| BuildError::missing_field("active_harness_version_id"))?,
            agent_slug: self.agent_slug.ok_or_else(|| BuildError::missing_field("agent_slug"))?,
            compatibility_profile_id: self.compatibility_profile_id.ok_or_else(|| BuildError::missing_field("compatibility_profile_id"))?,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            harness_id: self.harness_id.ok_or_else(|| BuildError::missing_field("harness_id"))?,
        })
    }
}

