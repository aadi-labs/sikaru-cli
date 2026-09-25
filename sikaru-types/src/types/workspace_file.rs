pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceFile {
    #[serde(default)]
    pub chunks: Vec<WorkspaceChunk>,
    #[serde(default)]
    pub mode: i64,
    #[serde(default)]
    pub sha256: String,
    #[serde(default)]
    pub size: i64,
}

impl WorkspaceFile {
    pub fn builder() -> WorkspaceFileBuilder {
        <WorkspaceFileBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceFileBuilder {
    chunks: Option<Vec<WorkspaceChunk>>,
    mode: Option<i64>,
    sha256: Option<String>,
    size: Option<i64>,
}

impl WorkspaceFileBuilder {
    pub fn chunks(mut self, value: Vec<WorkspaceChunk>) -> Self {
        self.chunks = Some(value);
        self
    }

    pub fn mode(mut self, value: i64) -> Self {
        self.mode = Some(value);
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

    /// Consumes the builder and constructs a [`WorkspaceFile`].
    /// This method will fail if any of the following fields are not set:
    /// - [`chunks`](WorkspaceFileBuilder::chunks)
    /// - [`mode`](WorkspaceFileBuilder::mode)
    /// - [`sha256`](WorkspaceFileBuilder::sha256)
    /// - [`size`](WorkspaceFileBuilder::size)
    pub fn build(self) -> Result<WorkspaceFile, BuildError> {
        Ok(WorkspaceFile {
            chunks: self.chunks.ok_or_else(|| BuildError::missing_field("chunks"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            sha256: self.sha256.ok_or_else(|| BuildError::missing_field("sha256"))?,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
        })
    }
}
