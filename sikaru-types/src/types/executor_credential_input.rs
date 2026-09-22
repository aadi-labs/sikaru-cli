pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExecutorCredentialInput {
    #[serde(default)]
    pub owner_epoch: i64,
    #[serde(default)]
    pub owner_id: String,
}

impl ExecutorCredentialInput {
    pub fn builder() -> ExecutorCredentialInputBuilder {
        <ExecutorCredentialInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutorCredentialInputBuilder {
    owner_epoch: Option<i64>,
    owner_id: Option<String>,
}

impl ExecutorCredentialInputBuilder {
    pub fn owner_epoch(mut self, value: i64) -> Self {
        self.owner_epoch = Some(value);
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExecutorCredentialInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`owner_epoch`](ExecutorCredentialInputBuilder::owner_epoch)
    /// - [`owner_id`](ExecutorCredentialInputBuilder::owner_id)
    pub fn build(self) -> Result<ExecutorCredentialInput, BuildError> {
        Ok(ExecutorCredentialInput {
            owner_epoch: self.owner_epoch.ok_or_else(|| BuildError::missing_field("owner_epoch"))?,
            owner_id: self.owner_id.ok_or_else(|| BuildError::missing_field("owner_id"))?,
        })
    }
}

