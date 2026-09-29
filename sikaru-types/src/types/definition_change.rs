pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DefinitionChange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<serde_json::Value>,
    #[serde(default)]
    pub path: String,
}

impl DefinitionChange {
    pub fn builder() -> DefinitionChangeBuilder {
        <DefinitionChangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DefinitionChangeBuilder {
    after: Option<serde_json::Value>,
    before: Option<serde_json::Value>,
    path: Option<String>,
}

impl DefinitionChangeBuilder {
    pub fn after(mut self, value: serde_json::Value) -> Self {
        self.after = Some(value);
        self
    }

    pub fn before(mut self, value: serde_json::Value) -> Self {
        self.before = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DefinitionChange`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](DefinitionChangeBuilder::path)
    pub fn build(self) -> Result<DefinitionChange, BuildError> {
        Ok(DefinitionChange {
            after: self.after,
            before: self.before,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
