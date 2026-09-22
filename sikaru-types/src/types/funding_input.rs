pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FundingInput {
    #[serde(default)]
    pub amount_usd: String,
    #[serde(default)]
    pub idempotency_key: String,
}

impl FundingInput {
    pub fn builder() -> FundingInputBuilder {
        <FundingInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FundingInputBuilder {
    amount_usd: Option<String>,
    idempotency_key: Option<String>,
}

impl FundingInputBuilder {
    pub fn amount_usd(mut self, value: impl Into<String>) -> Self {
        self.amount_usd = Some(value.into());
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FundingInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_usd`](FundingInputBuilder::amount_usd)
    /// - [`idempotency_key`](FundingInputBuilder::idempotency_key)
    pub fn build(self) -> Result<FundingInput, BuildError> {
        Ok(FundingInput {
            amount_usd: self.amount_usd.ok_or_else(|| BuildError::missing_field("amount_usd"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
        })
    }
}

