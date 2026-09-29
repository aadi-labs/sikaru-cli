pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DocumentAccessDelta {
    #[serde(default)]
    pub added: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(default)]
    pub changed: Vec<String>,
    #[serde(default)]
    pub removed: Vec<String>,
}

impl DocumentAccessDelta {
    pub fn builder() -> DocumentAccessDeltaBuilder {
        <DocumentAccessDeltaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentAccessDeltaBuilder {
    added: Option<Vec<String>>,
    capabilities: Option<Vec<HashMap<String, serde_json::Value>>>,
    changed: Option<Vec<String>>,
    removed: Option<Vec<String>>,
}

impl DocumentAccessDeltaBuilder {
    pub fn added(mut self, value: Vec<String>) -> Self {
        self.added = Some(value);
        self
    }

    pub fn capabilities(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.capabilities = Some(value);
        self
    }

    pub fn changed(mut self, value: Vec<String>) -> Self {
        self.changed = Some(value);
        self
    }

    pub fn removed(mut self, value: Vec<String>) -> Self {
        self.removed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentAccessDelta`].
    /// This method will fail if any of the following fields are not set:
    /// - [`added`](DocumentAccessDeltaBuilder::added)
    /// - [`changed`](DocumentAccessDeltaBuilder::changed)
    /// - [`removed`](DocumentAccessDeltaBuilder::removed)
    pub fn build(self) -> Result<DocumentAccessDelta, BuildError> {
        Ok(DocumentAccessDelta {
            added: self.added.ok_or_else(|| BuildError::missing_field("added"))?,
            capabilities: self.capabilities,
            changed: self.changed.ok_or_else(|| BuildError::missing_field("changed"))?,
            removed: self.removed.ok_or_else(|| BuildError::missing_field("removed"))?,
        })
    }
}
