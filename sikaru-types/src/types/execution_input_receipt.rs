pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExecutionInputReceipt {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "idempotencyKey")]
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(default)]
    pub mode: String,
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub status: String,
}

impl ExecutionInputReceipt {
    pub fn builder() -> ExecutionInputReceiptBuilder {
        <ExecutionInputReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionInputReceiptBuilder {
    id: Option<String>,
    idempotency_key: Option<String>,
    mode: Option<String>,
    run_id: Option<String>,
    status: Option<String>,
}

impl ExecutionInputReceiptBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn mode(mut self, value: impl Into<String>) -> Self {
        self.mode = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExecutionInputReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExecutionInputReceiptBuilder::id)
    /// - [`idempotency_key`](ExecutionInputReceiptBuilder::idempotency_key)
    /// - [`mode`](ExecutionInputReceiptBuilder::mode)
    /// - [`run_id`](ExecutionInputReceiptBuilder::run_id)
    /// - [`status`](ExecutionInputReceiptBuilder::status)
    pub fn build(self) -> Result<ExecutionInputReceipt, BuildError> {
        Ok(ExecutionInputReceipt {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
