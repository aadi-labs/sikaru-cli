pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeliveredMessage {
    #[serde(default)]
    pub content: String,
    #[serde(rename = "deliveredAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub delivered_at: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<DeliveredMessageEnvironment>,
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
    #[serde(default)]
    pub position: i64,
    pub role: DeliveredMessageRole,
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub account_id: String,
}

impl DeliveredMessage {
    pub fn builder() -> DeliveredMessageBuilder {
        <DeliveredMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveredMessageBuilder {
    content: Option<String>,
    delivered_at: Option<DateTime<FixedOffset>>,
    environment: Option<DeliveredMessageEnvironment>,
    message_id: Option<String>,
    position: Option<i64>,
    role: Option<DeliveredMessageRole>,
    trace_id: Option<String>,
    account_id: Option<String>,
}

impl DeliveredMessageBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn delivered_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.delivered_at = Some(value);
        self
    }

    pub fn environment(mut self, value: DeliveredMessageEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn position(mut self, value: i64) -> Self {
        self.position = Some(value);
        self
    }

    pub fn role(mut self, value: DeliveredMessageRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeliveredMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](DeliveredMessageBuilder::content)
    /// - [`delivered_at`](DeliveredMessageBuilder::delivered_at)
    /// - [`message_id`](DeliveredMessageBuilder::message_id)
    /// - [`position`](DeliveredMessageBuilder::position)
    /// - [`role`](DeliveredMessageBuilder::role)
    /// - [`account_id`](DeliveredMessageBuilder::account_id)
    pub fn build(self) -> Result<DeliveredMessage, BuildError> {
        Ok(DeliveredMessage {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            delivered_at: self.delivered_at.ok_or_else(|| BuildError::missing_field("delivered_at"))?,
            environment: self.environment,
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            position: self.position.ok_or_else(|| BuildError::missing_field("position"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            trace_id: self.trace_id,
            account_id: self.account_id.ok_or_else(|| BuildError::missing_field("account_id"))?,
        })
    }
}

