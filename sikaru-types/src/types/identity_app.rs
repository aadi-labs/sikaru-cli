pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct IdentityApp {
    #[serde(default)]
    pub audience: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_callback_url: Option<String>,
    #[serde(default)]
    pub issuer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_channel_url: Option<String>,
    #[serde(default)]
    pub public_jwk: HashMap<String, serde_json::Value>,
}

impl IdentityApp {
    pub fn builder() -> IdentityAppBuilder {
        <IdentityAppBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IdentityAppBuilder {
    audience: Option<String>,
    connection_callback_url: Option<String>,
    issuer: Option<String>,
    private_channel_url: Option<String>,
    public_jwk: Option<HashMap<String, serde_json::Value>>,
}

impl IdentityAppBuilder {
    pub fn audience(mut self, value: impl Into<String>) -> Self {
        self.audience = Some(value.into());
        self
    }

    pub fn connection_callback_url(mut self, value: impl Into<String>) -> Self {
        self.connection_callback_url = Some(value.into());
        self
    }

    pub fn issuer(mut self, value: impl Into<String>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    pub fn private_channel_url(mut self, value: impl Into<String>) -> Self {
        self.private_channel_url = Some(value.into());
        self
    }

    pub fn public_jwk(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.public_jwk = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`IdentityApp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audience`](IdentityAppBuilder::audience)
    /// - [`issuer`](IdentityAppBuilder::issuer)
    /// - [`public_jwk`](IdentityAppBuilder::public_jwk)
    pub fn build(self) -> Result<IdentityApp, BuildError> {
        Ok(IdentityApp {
            audience: self.audience.ok_or_else(|| BuildError::missing_field("audience"))?,
            connection_callback_url: self.connection_callback_url,
            issuer: self.issuer.ok_or_else(|| BuildError::missing_field("issuer"))?,
            private_channel_url: self.private_channel_url,
            public_jwk: self.public_jwk.ok_or_else(|| BuildError::missing_field("public_jwk"))?,
        })
    }
}

