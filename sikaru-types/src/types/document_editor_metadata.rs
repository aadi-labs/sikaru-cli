pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DocumentEditorMetadata {
    #[serde(rename = "bodyOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_offset: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ceilings: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub settings: HashMap<String, serde_json::Value>,
}

impl DocumentEditorMetadata {
    pub fn builder() -> DocumentEditorMetadataBuilder {
        <DocumentEditorMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentEditorMetadataBuilder {
    body_offset: Option<i64>,
    ceilings: Option<HashMap<String, serde_json::Value>>,
    settings: Option<HashMap<String, serde_json::Value>>,
}

impl DocumentEditorMetadataBuilder {
    pub fn body_offset(mut self, value: i64) -> Self {
        self.body_offset = Some(value);
        self
    }

    pub fn ceilings(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.ceilings = Some(value);
        self
    }

    pub fn settings(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentEditorMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`settings`](DocumentEditorMetadataBuilder::settings)
    pub fn build(self) -> Result<DocumentEditorMetadata, BuildError> {
        Ok(DocumentEditorMetadata {
            body_offset: self.body_offset,
            ceilings: self.ceilings,
            settings: self.settings.ok_or_else(|| BuildError::missing_field("settings"))?,
        })
    }
}
