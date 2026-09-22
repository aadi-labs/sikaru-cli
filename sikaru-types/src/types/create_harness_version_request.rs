pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateHarnessVersionRequest {
    #[serde(rename = "baseHarnessProfileId")]
    #[serde(default)]
    pub base_harness_profile_id: String,
    #[serde(rename = "baseHarnessVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_harness_version_id: Option<String>,
    #[serde(rename = "compatibilityProfileId")]
    #[serde(default)]
    pub compatibility_profile_id: String,
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(rename = "harnessId")]
    #[serde(default)]
    pub harness_id: String,
    #[serde(rename = "sourceArtifactId")]
    #[serde(default)]
    pub source_artifact_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreateHarnessVersionRequestStatus>,
}

impl CreateHarnessVersionRequest {
    pub fn builder() -> CreateHarnessVersionRequestBuilder {
        <CreateHarnessVersionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateHarnessVersionRequestBuilder {
    base_harness_profile_id: Option<String>,
    base_harness_version_id: Option<String>,
    compatibility_profile_id: Option<String>,
    display_name: Option<String>,
    harness_id: Option<String>,
    source_artifact_id: Option<String>,
    status: Option<CreateHarnessVersionRequestStatus>,
}

impl CreateHarnessVersionRequestBuilder {
    pub fn base_harness_profile_id(mut self, value: impl Into<String>) -> Self {
        self.base_harness_profile_id = Some(value.into());
        self
    }

    pub fn base_harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.base_harness_version_id = Some(value.into());
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

    pub fn source_artifact_id(mut self, value: impl Into<String>) -> Self {
        self.source_artifact_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: CreateHarnessVersionRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateHarnessVersionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`base_harness_profile_id`](CreateHarnessVersionRequestBuilder::base_harness_profile_id)
    /// - [`compatibility_profile_id`](CreateHarnessVersionRequestBuilder::compatibility_profile_id)
    /// - [`display_name`](CreateHarnessVersionRequestBuilder::display_name)
    /// - [`harness_id`](CreateHarnessVersionRequestBuilder::harness_id)
    /// - [`source_artifact_id`](CreateHarnessVersionRequestBuilder::source_artifact_id)
    pub fn build(self) -> Result<CreateHarnessVersionRequest, BuildError> {
        Ok(CreateHarnessVersionRequest {
            base_harness_profile_id: self.base_harness_profile_id.ok_or_else(|| BuildError::missing_field("base_harness_profile_id"))?,
            base_harness_version_id: self.base_harness_version_id,
            compatibility_profile_id: self.compatibility_profile_id.ok_or_else(|| BuildError::missing_field("compatibility_profile_id"))?,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            harness_id: self.harness_id.ok_or_else(|| BuildError::missing_field("harness_id"))?,
            source_artifact_id: self.source_artifact_id.ok_or_else(|| BuildError::missing_field("source_artifact_id"))?,
            status: self.status,
        })
    }
}

