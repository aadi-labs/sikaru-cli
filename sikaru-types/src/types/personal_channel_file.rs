pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PersonalChannelFile {
    #[serde(rename = "contentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(default)]
    pub direction: String,
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub sha256: String,
    #[serde(default)]
    pub size: i64,
    #[serde(rename = "snapshotDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_digest: Option<String>,
    #[serde(rename = "sourceRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_run_id: Option<String>,
}

impl PersonalChannelFile {
    pub fn builder() -> PersonalChannelFileBuilder {
        <PersonalChannelFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonalChannelFileBuilder {
    content_type: Option<String>,
    created_at: Option<f64>,
    direction: Option<String>,
    filename: Option<String>,
    id: Option<String>,
    path: Option<String>,
    sha256: Option<String>,
    size: Option<i64>,
    snapshot_digest: Option<String>,
    source_run_id: Option<String>,
}

impl PersonalChannelFileBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn direction(mut self, value: impl Into<String>) -> Self {
        self.direction = Some(value.into());
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn sha256(mut self, value: impl Into<String>) -> Self {
        self.sha256 = Some(value.into());
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn snapshot_digest(mut self, value: impl Into<String>) -> Self {
        self.snapshot_digest = Some(value.into());
        self
    }

    pub fn source_run_id(mut self, value: impl Into<String>) -> Self {
        self.source_run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PersonalChannelFile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_type`](PersonalChannelFileBuilder::content_type)
    /// - [`created_at`](PersonalChannelFileBuilder::created_at)
    /// - [`direction`](PersonalChannelFileBuilder::direction)
    /// - [`filename`](PersonalChannelFileBuilder::filename)
    /// - [`id`](PersonalChannelFileBuilder::id)
    /// - [`path`](PersonalChannelFileBuilder::path)
    /// - [`sha256`](PersonalChannelFileBuilder::sha256)
    /// - [`size`](PersonalChannelFileBuilder::size)
    pub fn build(self) -> Result<PersonalChannelFile, BuildError> {
        Ok(PersonalChannelFile {
            content_type: self.content_type.ok_or_else(|| BuildError::missing_field("content_type"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            direction: self.direction.ok_or_else(|| BuildError::missing_field("direction"))?,
            filename: self.filename.ok_or_else(|| BuildError::missing_field("filename"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            sha256: self.sha256.ok_or_else(|| BuildError::missing_field("sha256"))?,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
            snapshot_digest: self.snapshot_digest,
            source_run_id: self.source_run_id,
        })
    }
}
