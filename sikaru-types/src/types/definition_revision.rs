pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DefinitionRevision {
    #[serde(rename = "baseHarnessVersionId")]
    #[serde(default)]
    pub base_harness_version_id: String,
    #[serde(rename = "candidateHarnessVersionId")]
    #[serde(default)]
    pub candidate_harness_version_id: String,
    #[serde(default)]
    pub changes: Vec<DefinitionChange>,
    #[serde(default)]
    pub changeset: HashMap<String, serde_json::Value>,
    #[serde(rename = "definitionDigest")]
    #[serde(default)]
    pub definition_digest: String,
}

impl DefinitionRevision {
    pub fn builder() -> DefinitionRevisionBuilder {
        <DefinitionRevisionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DefinitionRevisionBuilder {
    base_harness_version_id: Option<String>,
    candidate_harness_version_id: Option<String>,
    changes: Option<Vec<DefinitionChange>>,
    changeset: Option<HashMap<String, serde_json::Value>>,
    definition_digest: Option<String>,
}

impl DefinitionRevisionBuilder {
    pub fn base_harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.base_harness_version_id = Some(value.into());
        self
    }

    pub fn candidate_harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.candidate_harness_version_id = Some(value.into());
        self
    }

    pub fn changes(mut self, value: Vec<DefinitionChange>) -> Self {
        self.changes = Some(value);
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

    /// Consumes the builder and constructs a [`DefinitionRevision`].
    /// This method will fail if any of the following fields are not set:
    /// - [`base_harness_version_id`](DefinitionRevisionBuilder::base_harness_version_id)
    /// - [`candidate_harness_version_id`](DefinitionRevisionBuilder::candidate_harness_version_id)
    /// - [`changes`](DefinitionRevisionBuilder::changes)
    /// - [`changeset`](DefinitionRevisionBuilder::changeset)
    /// - [`definition_digest`](DefinitionRevisionBuilder::definition_digest)
    pub fn build(self) -> Result<DefinitionRevision, BuildError> {
        Ok(DefinitionRevision {
            base_harness_version_id: self.base_harness_version_id.ok_or_else(|| BuildError::missing_field("base_harness_version_id"))?,
            candidate_harness_version_id: self.candidate_harness_version_id.ok_or_else(|| BuildError::missing_field("candidate_harness_version_id"))?,
            changes: self.changes.ok_or_else(|| BuildError::missing_field("changes"))?,
            changeset: self.changeset.ok_or_else(|| BuildError::missing_field("changeset"))?,
            definition_digest: self.definition_digest.ok_or_else(|| BuildError::missing_field("definition_digest"))?,
        })
    }
}
