pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentTemplates {
    #[serde(default)]
    pub templates: Vec<DocumentTemplate>,
}

impl DocumentTemplates {
    pub fn builder() -> DocumentTemplatesBuilder {
        <DocumentTemplatesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentTemplatesBuilder {
    templates: Option<Vec<DocumentTemplate>>,
}

impl DocumentTemplatesBuilder {
    pub fn templates(mut self, value: Vec<DocumentTemplate>) -> Self {
        self.templates = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentTemplates`].
    /// This method will fail if any of the following fields are not set:
    /// - [`templates`](DocumentTemplatesBuilder::templates)
    pub fn build(self) -> Result<DocumentTemplates, BuildError> {
        Ok(DocumentTemplates {
            templates: self.templates.ok_or_else(|| BuildError::missing_field("templates"))?,
        })
    }
}
