pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DocumentValidationView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<DocumentEditorMetadata>,
    #[serde(default)]
    pub errors: Vec<DocumentIssue>,
    #[serde(default)]
    pub mentions: Vec<ResolvedMention>,
    #[serde(default)]
    pub publishable: bool,
}

impl DocumentValidationView {
    pub fn builder() -> DocumentValidationViewBuilder {
        <DocumentValidationViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentValidationViewBuilder {
    editor: Option<DocumentEditorMetadata>,
    errors: Option<Vec<DocumentIssue>>,
    mentions: Option<Vec<ResolvedMention>>,
    publishable: Option<bool>,
}

impl DocumentValidationViewBuilder {
    pub fn editor(mut self, value: DocumentEditorMetadata) -> Self {
        self.editor = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<DocumentIssue>) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn mentions(mut self, value: Vec<ResolvedMention>) -> Self {
        self.mentions = Some(value);
        self
    }

    pub fn publishable(mut self, value: bool) -> Self {
        self.publishable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentValidationView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`errors`](DocumentValidationViewBuilder::errors)
    /// - [`mentions`](DocumentValidationViewBuilder::mentions)
    /// - [`publishable`](DocumentValidationViewBuilder::publishable)
    pub fn build(self) -> Result<DocumentValidationView, BuildError> {
        Ok(DocumentValidationView {
            editor: self.editor,
            errors: self.errors.ok_or_else(|| BuildError::missing_field("errors"))?,
            mentions: self.mentions.ok_or_else(|| BuildError::missing_field("mentions"))?,
            publishable: self.publishable.ok_or_else(|| BuildError::missing_field("publishable"))?,
        })
    }
}
