pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhookInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub url: String,
}

impl WebhookInput {
    pub fn builder() -> WebhookInputBuilder {
        <WebhookInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookInputBuilder {
    idempotency_key: Option<String>,
    run_id: Option<String>,
    url: Option<String>,
}

impl WebhookInputBuilder {
    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run_id`](WebhookInputBuilder::run_id)
    /// - [`url`](WebhookInputBuilder::url)
    pub fn build(self) -> Result<WebhookInput, BuildError> {
        Ok(WebhookInput {
            idempotency_key: self.idempotency_key,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}

