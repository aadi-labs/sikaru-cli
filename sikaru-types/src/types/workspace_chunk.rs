pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceChunk {
    #[serde(default)]
    pub sha256: String,
    #[serde(default)]
    pub size: i64,
}

impl WorkspaceChunk {
    pub fn builder() -> WorkspaceChunkBuilder {
        <WorkspaceChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceChunkBuilder {
    sha256: Option<String>,
    size: Option<i64>,
}

impl WorkspaceChunkBuilder {
    pub fn sha256(mut self, value: impl Into<String>) -> Self {
        self.sha256 = Some(value.into());
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceChunk`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sha256`](WorkspaceChunkBuilder::sha256)
    /// - [`size`](WorkspaceChunkBuilder::size)
    pub fn build(self) -> Result<WorkspaceChunk, BuildError> {
        Ok(WorkspaceChunk {
            sha256: self.sha256.ok_or_else(|| BuildError::missing_field("sha256"))?,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
        })
    }
}
