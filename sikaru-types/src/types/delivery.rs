pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Delivery {
    #[serde(default)]
    pub attempts: i64,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_ts: Option<String>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub retry_at: f64,
    #[serde(default)]
    pub run_id: String,
    pub status: DeliveryStatus,
}

impl Delivery {
    pub fn builder() -> DeliveryBuilder {
        <DeliveryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveryBuilder {
    attempts: Option<i64>,
    id: Option<String>,
    message_ts: Option<String>,
    retry_at: Option<f64>,
    run_id: Option<String>,
    status: Option<DeliveryStatus>,
}

impl DeliveryBuilder {
    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn message_ts(mut self, value: impl Into<String>) -> Self {
        self.message_ts = Some(value.into());
        self
    }

    pub fn retry_at(mut self, value: f64) -> Self {
        self.retry_at = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: DeliveryStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Delivery`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempts`](DeliveryBuilder::attempts)
    /// - [`id`](DeliveryBuilder::id)
    /// - [`retry_at`](DeliveryBuilder::retry_at)
    /// - [`run_id`](DeliveryBuilder::run_id)
    /// - [`status`](DeliveryBuilder::status)
    pub fn build(self) -> Result<Delivery, BuildError> {
        Ok(Delivery {
            attempts: self.attempts.ok_or_else(|| BuildError::missing_field("attempts"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            message_ts: self.message_ts,
            retry_at: self.retry_at.ok_or_else(|| BuildError::missing_field("retry_at"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
