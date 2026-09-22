pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionAuthorization {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub expires_at: f64,
    #[serde(default)]
    pub state: String,
    #[serde(default)]
    pub url: String,
}

impl ConnectionAuthorization {
    pub fn builder() -> ConnectionAuthorizationBuilder {
        <ConnectionAuthorizationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionAuthorizationBuilder {
    expires_at: Option<f64>,
    state: Option<String>,
    url: Option<String>,
}

impl ConnectionAuthorizationBuilder {
    pub fn expires_at(mut self, value: f64) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectionAuthorization`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expires_at`](ConnectionAuthorizationBuilder::expires_at)
    /// - [`state`](ConnectionAuthorizationBuilder::state)
    /// - [`url`](ConnectionAuthorizationBuilder::url)
    pub fn build(self) -> Result<ConnectionAuthorization, BuildError> {
        Ok(ConnectionAuthorization {
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
