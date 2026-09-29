pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DefinitionRevisionView {
    #[serde(default)]
    pub revision: DefinitionRevision,
}

impl DefinitionRevisionView {
    pub fn builder() -> DefinitionRevisionViewBuilder {
        <DefinitionRevisionViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DefinitionRevisionViewBuilder {
    revision: Option<DefinitionRevision>,
}

impl DefinitionRevisionViewBuilder {
    pub fn revision(mut self, value: DefinitionRevision) -> Self {
        self.revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DefinitionRevisionView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`revision`](DefinitionRevisionViewBuilder::revision)
    pub fn build(self) -> Result<DefinitionRevisionView, BuildError> {
        Ok(DefinitionRevisionView {
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
        })
    }
}
