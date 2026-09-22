pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<ConnectionConfigAuth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolkit: Option<String>,
}

impl ConnectionConfig {
    pub fn builder() -> ConnectionConfigBuilder {
        <ConnectionConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionConfigBuilder {
    auth: Option<ConnectionConfigAuth>,
    endpoint: Option<String>,
    header_name: Option<String>,
    oauth_client_id: Option<String>,
    oauth_scopes: Option<Vec<String>>,
    toolkit: Option<String>,
}

impl ConnectionConfigBuilder {
    pub fn auth(mut self, value: ConnectionConfigAuth) -> Self {
        self.auth = Some(value);
        self
    }

    pub fn endpoint(mut self, value: impl Into<String>) -> Self {
        self.endpoint = Some(value.into());
        self
    }

    pub fn header_name(mut self, value: impl Into<String>) -> Self {
        self.header_name = Some(value.into());
        self
    }

    pub fn oauth_client_id(mut self, value: impl Into<String>) -> Self {
        self.oauth_client_id = Some(value.into());
        self
    }

    pub fn oauth_scopes(mut self, value: Vec<String>) -> Self {
        self.oauth_scopes = Some(value);
        self
    }

    pub fn toolkit(mut self, value: impl Into<String>) -> Self {
        self.toolkit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectionConfig`].
    pub fn build(self) -> Result<ConnectionConfig, BuildError> {
        Ok(ConnectionConfig {
            auth: self.auth,
            endpoint: self.endpoint,
            header_name: self.header_name,
            oauth_client_id: self.oauth_client_id,
            oauth_scopes: self.oauth_scopes,
            toolkit: self.toolkit,
        })
    }
}
