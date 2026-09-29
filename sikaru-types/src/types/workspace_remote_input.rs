pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceRemoteInput {
}

impl WorkspaceRemoteInput {
    pub fn builder() -> WorkspaceRemoteInputBuilder {
        <WorkspaceRemoteInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceRemoteInputBuilder {
}

impl WorkspaceRemoteInputBuilder {

    /// Consumes the builder and constructs a [`WorkspaceRemoteInput`].
    pub fn build(self) -> Result<WorkspaceRemoteInput, BuildError> {
        Ok(WorkspaceRemoteInput {
        })
    }
}

