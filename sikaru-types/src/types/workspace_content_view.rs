pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceContentView {
    #[serde(default)]
    pub sha256: String,
    #[serde(default)]
    pub size: i64,
}

impl WorkspaceContentView {
    pub fn builder() -> WorkspaceContentViewBuilder {
        <WorkspaceContentViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceContentViewBuilder {
    sha256: Option<String>,
    size: Option<i64>,
}

impl WorkspaceContentViewBuilder {
    pub fn sha256(mut self, value: impl Into<String>) -> Self {
        self.sha256 = Some(value.into());
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceContentView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`sha256`](WorkspaceContentViewBuilder::sha256)
    /// - [`size`](WorkspaceContentViewBuilder::size)
    pub fn build(self) -> Result<WorkspaceContentView, BuildError> {
        Ok(WorkspaceContentView {
            sha256: self.sha256.ok_or_else(|| BuildError::missing_field("sha256"))?,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
        })
    }
}
