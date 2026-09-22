pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ClaimInput {
    #[serde(default)]
    pub idempotency_key: String,
}

impl ClaimInput {
    pub fn builder() -> ClaimInputBuilder {
        <ClaimInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClaimInputBuilder {
    idempotency_key: Option<String>,
}

impl ClaimInputBuilder {
    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClaimInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](ClaimInputBuilder::idempotency_key)
    pub fn build(self) -> Result<ClaimInput, BuildError> {
        Ok(ClaimInput {
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
        })
    }
}

