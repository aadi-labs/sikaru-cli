pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceiptSummary {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default)]
    pub status: String,
}

impl ReceiptSummary {
    pub fn builder() -> ReceiptSummaryBuilder {
        <ReceiptSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptSummaryBuilder {
    created_at: Option<f64>,
    id: Option<String>,
    run_id: Option<String>,
    session_id: Option<String>,
    status: Option<String>,
}

impl ReceiptSummaryBuilder {
    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceiptSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ReceiptSummaryBuilder::created_at)
    /// - [`id`](ReceiptSummaryBuilder::id)
    /// - [`status`](ReceiptSummaryBuilder::status)
    pub fn build(self) -> Result<ReceiptSummary, BuildError> {
        Ok(ReceiptSummary {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            run_id: self.run_id,
            session_id: self.session_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
