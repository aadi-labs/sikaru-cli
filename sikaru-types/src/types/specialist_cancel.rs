pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpecialistCancel {
    #[serde(default)]
    pub idempotency_key: String,
}

impl SpecialistCancel {
    pub fn builder() -> SpecialistCancelBuilder {
        <SpecialistCancelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpecialistCancelBuilder {
    idempotency_key: Option<String>,
}

impl SpecialistCancelBuilder {
    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpecialistCancel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](SpecialistCancelBuilder::idempotency_key)
    pub fn build(self) -> Result<SpecialistCancel, BuildError> {
        Ok(SpecialistCancel {
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
        })
    }
}

