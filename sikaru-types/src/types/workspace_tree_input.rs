pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WorkspaceTreeInput {
    #[serde(default)]
    pub files: HashMap<String, WorkspaceFile>,
}

impl WorkspaceTreeInput {
    pub fn builder() -> WorkspaceTreeInputBuilder {
        <WorkspaceTreeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceTreeInputBuilder {
    files: Option<HashMap<String, WorkspaceFile>>,
}

impl WorkspaceTreeInputBuilder {
    pub fn files(mut self, value: HashMap<String, WorkspaceFile>) -> Self {
        self.files = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceTreeInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`files`](WorkspaceTreeInputBuilder::files)
    pub fn build(self) -> Result<WorkspaceTreeInput, BuildError> {
        Ok(WorkspaceTreeInput {
            files: self.files.ok_or_else(|| BuildError::missing_field("files"))?,
        })
    }
}

