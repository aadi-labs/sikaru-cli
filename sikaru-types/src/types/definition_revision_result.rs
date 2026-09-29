pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DefinitionRevisionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<DefinitionRevision>,
    #[serde(default)]
    pub unchanged: bool,
}

impl DefinitionRevisionResult {
    pub fn builder() -> DefinitionRevisionResultBuilder {
        <DefinitionRevisionResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DefinitionRevisionResultBuilder {
    revision: Option<DefinitionRevision>,
    unchanged: Option<bool>,
}

impl DefinitionRevisionResultBuilder {
    pub fn revision(mut self, value: DefinitionRevision) -> Self {
        self.revision = Some(value);
        self
    }

    pub fn unchanged(mut self, value: bool) -> Self {
        self.unchanged = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DefinitionRevisionResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unchanged`](DefinitionRevisionResultBuilder::unchanged)
    pub fn build(self) -> Result<DefinitionRevisionResult, BuildError> {
        Ok(DefinitionRevisionResult {
            revision: self.revision,
            unchanged: self.unchanged.ok_or_else(|| BuildError::missing_field("unchanged"))?,
        })
    }
}
