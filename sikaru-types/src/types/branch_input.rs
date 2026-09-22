pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BranchInput {
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(default)]
    pub source_run_id: String,
}

impl BranchInput {
    pub fn builder() -> BranchInputBuilder {
        <BranchInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BranchInputBuilder {
    idempotency_key: Option<String>,
    source_run_id: Option<String>,
}

impl BranchInputBuilder {
    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn source_run_id(mut self, value: impl Into<String>) -> Self {
        self.source_run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BranchInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](BranchInputBuilder::idempotency_key)
    /// - [`source_run_id`](BranchInputBuilder::source_run_id)
    pub fn build(self) -> Result<BranchInput, BuildError> {
        Ok(BranchInput {
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            source_run_id: self.source_run_id.ok_or_else(|| BuildError::missing_field("source_run_id"))?,
        })
    }
}

