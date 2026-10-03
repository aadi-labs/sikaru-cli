pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EditDocumentSetting {
    #[serde(default)]
    pub document: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<EditDocumentSettingOp>,
    #[serde(default)]
    pub path: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl EditDocumentSetting {
    pub fn builder() -> EditDocumentSettingBuilder {
        <EditDocumentSettingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditDocumentSettingBuilder {
    document: Option<String>,
    op: Option<EditDocumentSettingOp>,
    path: Option<Vec<String>>,
    value: Option<serde_json::Value>,
}

impl EditDocumentSettingBuilder {
    pub fn document(mut self, value: impl Into<String>) -> Self {
        self.document = Some(value.into());
        self
    }

    pub fn op(mut self, value: EditDocumentSettingOp) -> Self {
        self.op = Some(value);
        self
    }

    pub fn path(mut self, value: Vec<String>) -> Self {
        self.path = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EditDocumentSetting`].
    /// This method will fail if any of the following fields are not set:
    /// - [`document`](EditDocumentSettingBuilder::document)
    /// - [`path`](EditDocumentSettingBuilder::path)
    pub fn build(self) -> Result<EditDocumentSetting, BuildError> {
        Ok(EditDocumentSetting {
            document: self.document.ok_or_else(|| BuildError::missing_field("document"))?,
            op: self.op,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            value: self.value,
        })
    }
}

