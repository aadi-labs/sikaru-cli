pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentTemplate {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub document: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl DocumentTemplate {
    pub fn builder() -> DocumentTemplateBuilder {
        <DocumentTemplateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentTemplateBuilder {
    description: Option<String>,
    document: Option<String>,
    id: Option<String>,
    name: Option<String>,
}

impl DocumentTemplateBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn document(mut self, value: impl Into<String>) -> Self {
        self.document = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentTemplate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](DocumentTemplateBuilder::description)
    /// - [`document`](DocumentTemplateBuilder::document)
    /// - [`id`](DocumentTemplateBuilder::id)
    /// - [`name`](DocumentTemplateBuilder::name)
    pub fn build(self) -> Result<DocumentTemplate, BuildError> {
        Ok(DocumentTemplate {
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            document: self.document.ok_or_else(|| BuildError::missing_field("document"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
