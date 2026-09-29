pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A draft revision staged by agent creation. Get the revision to see its changes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StagedDefinitionRevision {
    #[serde(rename = "baseHarnessVersionId")]
    #[serde(default)]
    pub base_harness_version_id: String,
    #[serde(rename = "candidateHarnessVersionId")]
    #[serde(default)]
    pub candidate_harness_version_id: String,
    #[serde(default)]
    pub changeset: HashMap<String, serde_json::Value>,
    #[serde(rename = "definitionDigest")]
    #[serde(default)]
    pub definition_digest: String,
}

impl StagedDefinitionRevision {
    pub fn builder() -> StagedDefinitionRevisionBuilder {
        <StagedDefinitionRevisionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StagedDefinitionRevisionBuilder {
    base_harness_version_id: Option<String>,
    candidate_harness_version_id: Option<String>,
    changeset: Option<HashMap<String, serde_json::Value>>,
    definition_digest: Option<String>,
}

impl StagedDefinitionRevisionBuilder {
    pub fn base_harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.base_harness_version_id = Some(value.into());
        self
    }

    pub fn candidate_harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.candidate_harness_version_id = Some(value.into());
        self
    }

    pub fn changeset(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.changeset = Some(value);
        self
    }

    pub fn definition_digest(mut self, value: impl Into<String>) -> Self {
        self.definition_digest = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StagedDefinitionRevision`].
    /// This method will fail if any of the following fields are not set:
    /// - [`base_harness_version_id`](StagedDefinitionRevisionBuilder::base_harness_version_id)
    /// - [`candidate_harness_version_id`](StagedDefinitionRevisionBuilder::candidate_harness_version_id)
    /// - [`changeset`](StagedDefinitionRevisionBuilder::changeset)
    /// - [`definition_digest`](StagedDefinitionRevisionBuilder::definition_digest)
    pub fn build(self) -> Result<StagedDefinitionRevision, BuildError> {
        Ok(StagedDefinitionRevision {
            base_harness_version_id: self.base_harness_version_id.ok_or_else(|| BuildError::missing_field("base_harness_version_id"))?,
            candidate_harness_version_id: self.candidate_harness_version_id.ok_or_else(|| BuildError::missing_field("candidate_harness_version_id"))?,
            changeset: self.changeset.ok_or_else(|| BuildError::missing_field("changeset"))?,
            definition_digest: self.definition_digest.ok_or_else(|| BuildError::missing_field("definition_digest"))?,
        })
    }
}
