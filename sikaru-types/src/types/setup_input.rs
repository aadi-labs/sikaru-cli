pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetupInput {
    #[serde(default)]
    pub idempotency_key: String,
}

impl SetupInput {
    pub fn builder() -> SetupInputBuilder {
        <SetupInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupInputBuilder {
    idempotency_key: Option<String>,
}

impl SetupInputBuilder {
    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetupInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](SetupInputBuilder::idempotency_key)
    pub fn build(self) -> Result<SetupInput, BuildError> {
        Ok(SetupInput {
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
        })
    }
}

