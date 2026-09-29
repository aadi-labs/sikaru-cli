pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevertDocument {
    #[serde(rename = "acknowledgeRemovals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledge_removals: Option<bool>,
    #[serde(rename = "acknowledgeWidening")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledge_widening: Option<bool>,
    #[serde(rename = "expectedLiveVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_live_version_id: Option<String>,
    #[serde(rename = "harnessVersionId")]
    #[serde(default)]
    pub harness_version_id: String,
    #[serde(default)]
    pub revision: i64,
}

impl RevertDocument {
    pub fn builder() -> RevertDocumentBuilder {
        <RevertDocumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevertDocumentBuilder {
    acknowledge_removals: Option<bool>,
    acknowledge_widening: Option<bool>,
    expected_live_version_id: Option<String>,
    harness_version_id: Option<String>,
    revision: Option<i64>,
}

impl RevertDocumentBuilder {
    pub fn acknowledge_removals(mut self, value: bool) -> Self {
        self.acknowledge_removals = Some(value);
        self
    }

    pub fn acknowledge_widening(mut self, value: bool) -> Self {
        self.acknowledge_widening = Some(value);
        self
    }

    pub fn expected_live_version_id(mut self, value: impl Into<String>) -> Self {
        self.expected_live_version_id = Some(value.into());
        self
    }

    pub fn harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.harness_version_id = Some(value.into());
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RevertDocument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`harness_version_id`](RevertDocumentBuilder::harness_version_id)
    /// - [`revision`](RevertDocumentBuilder::revision)
    pub fn build(self) -> Result<RevertDocument, BuildError> {
        Ok(RevertDocument {
            acknowledge_removals: self.acknowledge_removals,
            acknowledge_widening: self.acknowledge_widening,
            expected_live_version_id: self.expected_live_version_id,
            harness_version_id: self.harness_version_id.ok_or_else(|| BuildError::missing_field("harness_version_id"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
        })
    }
}

