pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ImprovementInput {
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective: Option<ImprovementInputObjective>,
}

impl ImprovementInput {
    pub fn builder() -> ImprovementInputBuilder {
        <ImprovementInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ImprovementInputBuilder {
    idempotency_key: Option<String>,
    objective: Option<ImprovementInputObjective>,
}

impl ImprovementInputBuilder {
    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn objective(mut self, value: ImprovementInputObjective) -> Self {
        self.objective = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ImprovementInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`idempotency_key`](ImprovementInputBuilder::idempotency_key)
    pub fn build(self) -> Result<ImprovementInput, BuildError> {
        Ok(ImprovementInput {
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            objective: self.objective,
        })
    }
}

