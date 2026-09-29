pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentRevision {
    #[serde(default)]
    pub actor: String,
    #[serde(rename = "baseLiveVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_live_version_id: Option<String>,
    #[serde(rename = "contentOrigin")]
    #[serde(default)]
    pub content_origin: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub document: String,
    #[serde(default)]
    pub revision: i64,
}

impl DocumentRevision {
    pub fn builder() -> DocumentRevisionBuilder {
        <DocumentRevisionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentRevisionBuilder {
    actor: Option<String>,
    base_live_version_id: Option<String>,
    content_origin: Option<String>,
    created_at: Option<String>,
    document: Option<String>,
    revision: Option<i64>,
}

impl DocumentRevisionBuilder {
    pub fn actor(mut self, value: impl Into<String>) -> Self {
        self.actor = Some(value.into());
        self
    }

    pub fn base_live_version_id(mut self, value: impl Into<String>) -> Self {
        self.base_live_version_id = Some(value.into());
        self
    }

    pub fn content_origin(mut self, value: impl Into<String>) -> Self {
        self.content_origin = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn document(mut self, value: impl Into<String>) -> Self {
        self.document = Some(value.into());
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentRevision`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actor`](DocumentRevisionBuilder::actor)
    /// - [`content_origin`](DocumentRevisionBuilder::content_origin)
    /// - [`created_at`](DocumentRevisionBuilder::created_at)
    /// - [`document`](DocumentRevisionBuilder::document)
    /// - [`revision`](DocumentRevisionBuilder::revision)
    pub fn build(self) -> Result<DocumentRevision, BuildError> {
        Ok(DocumentRevision {
            actor: self.actor.ok_or_else(|| BuildError::missing_field("actor"))?,
            base_live_version_id: self.base_live_version_id,
            content_origin: self.content_origin.ok_or_else(|| BuildError::missing_field("content_origin"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            document: self.document.ok_or_else(|| BuildError::missing_field("document"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
        })
    }
}
