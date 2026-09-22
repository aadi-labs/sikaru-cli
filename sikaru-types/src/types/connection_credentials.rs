pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectionCredentials {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl ConnectionCredentials {
    pub fn builder() -> ConnectionCredentialsBuilder {
        <ConnectionCredentialsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionCredentialsBuilder {
    oauth_client_secret: Option<String>,
    token: Option<String>,
}

impl ConnectionCredentialsBuilder {
    pub fn oauth_client_secret(mut self, value: impl Into<String>) -> Self {
        self.oauth_client_secret = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectionCredentials`].
    pub fn build(self) -> Result<ConnectionCredentials, BuildError> {
        Ok(ConnectionCredentials {
            oauth_client_secret: self.oauth_client_secret,
            token: self.token,
        })
    }
}
