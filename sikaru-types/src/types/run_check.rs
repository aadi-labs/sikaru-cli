pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunCheck {
    #[serde(default)]
    pub idempotency_key: String,
}

impl RunCheck {
    pub fn builder() -> RunCheckBuilder {
        <RunCheckBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunCheckBuilder {
    idempotency_key: Option<String>,
}

impl RunCheckBuilder {
    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunCheck`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](RunCheckBuilder::idempotency_key)
    pub fn build(self) -> Result<RunCheck, BuildError> {
        Ok(RunCheck {
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
        })
    }
}

