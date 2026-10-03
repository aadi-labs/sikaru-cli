pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DocumentSuggestions {
    #[serde(default)]
    pub suggestions: Vec<DocumentSuggestion>,
}

impl DocumentSuggestions {
    pub fn builder() -> DocumentSuggestionsBuilder {
        <DocumentSuggestionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentSuggestionsBuilder {
    suggestions: Option<Vec<DocumentSuggestion>>,
}

impl DocumentSuggestionsBuilder {
    pub fn suggestions(mut self, value: Vec<DocumentSuggestion>) -> Self {
        self.suggestions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentSuggestions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`suggestions`](DocumentSuggestionsBuilder::suggestions)
    pub fn build(self) -> Result<DocumentSuggestions, BuildError> {
        Ok(DocumentSuggestions {
            suggestions: self.suggestions.ok_or_else(|| BuildError::missing_field("suggestions"))?,
        })
    }
}
