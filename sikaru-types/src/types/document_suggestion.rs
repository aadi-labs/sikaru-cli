pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentSuggestion {
    #[serde(default)]
    pub actor: String,
    #[serde(rename = "baseLiveVersionId")]
    #[serde(default)]
    pub base_live_version_id: String,
    #[serde(rename = "candidateVersionId")]
    #[serde(default)]
    pub candidate_version_id: String,
    #[serde(rename = "contentOrigin")]
    #[serde(default)]
    pub content_origin: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff: Option<String>,
    #[serde(default)]
    pub document: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub status: String,
}

impl DocumentSuggestion {
    pub fn builder() -> DocumentSuggestionBuilder {
        <DocumentSuggestionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentSuggestionBuilder {
    actor: Option<String>,
    base_live_version_id: Option<String>,
    candidate_version_id: Option<String>,
    content_origin: Option<String>,
    created_at: Option<String>,
    diff: Option<String>,
    document: Option<String>,
    id: Option<String>,
    status: Option<String>,
}

impl DocumentSuggestionBuilder {
    pub fn actor(mut self, value: impl Into<String>) -> Self {
        self.actor = Some(value.into());
        self
    }

    pub fn base_live_version_id(mut self, value: impl Into<String>) -> Self {
        self.base_live_version_id = Some(value.into());
        self
    }

    pub fn candidate_version_id(mut self, value: impl Into<String>) -> Self {
        self.candidate_version_id = Some(value.into());
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

    pub fn diff(mut self, value: impl Into<String>) -> Self {
        self.diff = Some(value.into());
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

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentSuggestion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actor`](DocumentSuggestionBuilder::actor)
    /// - [`base_live_version_id`](DocumentSuggestionBuilder::base_live_version_id)
    /// - [`candidate_version_id`](DocumentSuggestionBuilder::candidate_version_id)
    /// - [`content_origin`](DocumentSuggestionBuilder::content_origin)
    /// - [`created_at`](DocumentSuggestionBuilder::created_at)
    /// - [`document`](DocumentSuggestionBuilder::document)
    /// - [`id`](DocumentSuggestionBuilder::id)
    /// - [`status`](DocumentSuggestionBuilder::status)
    pub fn build(self) -> Result<DocumentSuggestion, BuildError> {
        Ok(DocumentSuggestion {
            actor: self.actor.ok_or_else(|| BuildError::missing_field("actor"))?,
            base_live_version_id: self.base_live_version_id.ok_or_else(|| BuildError::missing_field("base_live_version_id"))?,
            candidate_version_id: self.candidate_version_id.ok_or_else(|| BuildError::missing_field("candidate_version_id"))?,
            content_origin: self.content_origin.ok_or_else(|| BuildError::missing_field("content_origin"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            diff: self.diff,
            document: self.document.ok_or_else(|| BuildError::missing_field("document"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
