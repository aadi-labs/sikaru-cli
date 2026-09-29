pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdoptSuggestion {
    #[serde(rename = "expectedRevision")]
    #[serde(default)]
    pub expected_revision: i64,
}

impl AdoptSuggestion {
    pub fn builder() -> AdoptSuggestionBuilder {
        <AdoptSuggestionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdoptSuggestionBuilder {
    expected_revision: Option<i64>,
}

impl AdoptSuggestionBuilder {
    pub fn expected_revision(mut self, value: i64) -> Self {
        self.expected_revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdoptSuggestion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expected_revision`](AdoptSuggestionBuilder::expected_revision)
    pub fn build(self) -> Result<AdoptSuggestion, BuildError> {
        Ok(AdoptSuggestion {
            expected_revision: self.expected_revision.ok_or_else(|| BuildError::missing_field("expected_revision"))?,
        })
    }
}

