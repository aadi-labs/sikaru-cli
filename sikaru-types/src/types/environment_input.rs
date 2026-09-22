pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnvironmentInput {
    #[serde(default)]
    pub environment_slug: String,
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl EnvironmentInput {
    pub fn builder() -> EnvironmentInputBuilder {
        <EnvironmentInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentInputBuilder {
    environment_slug: Option<String>,
    idempotency_key: Option<String>,
    product_id: Option<String>,
    session_id: Option<String>,
}

impl EnvironmentInputBuilder {
    pub fn environment_slug(mut self, value: impl Into<String>) -> Self {
        self.environment_slug = Some(value.into());
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment_slug`](EnvironmentInputBuilder::environment_slug)
    /// - [`idempotency_key`](EnvironmentInputBuilder::idempotency_key)
    pub fn build(self) -> Result<EnvironmentInput, BuildError> {
        Ok(EnvironmentInput {
            environment_slug: self.environment_slug.ok_or_else(|| BuildError::missing_field("environment_slug"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            product_id: self.product_id,
            session_id: self.session_id,
        })
    }
}

