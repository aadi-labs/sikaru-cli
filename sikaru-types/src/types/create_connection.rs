pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateConnection {
    #[serde(default)]
    pub config: ConnectionConfig,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<ConnectionCredentials>,
    #[serde(default)]
    pub display_name: String,
    pub kind: CreateConnectionKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<CreateConnectionOwnership>,
}

impl CreateConnection {
    pub fn builder() -> CreateConnectionBuilder {
        <CreateConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConnectionBuilder {
    config: Option<ConnectionConfig>,
    credentials: Option<ConnectionCredentials>,
    display_name: Option<String>,
    kind: Option<CreateConnectionKind>,
    ownership: Option<CreateConnectionOwnership>,
}

impl CreateConnectionBuilder {
    pub fn config(mut self, value: ConnectionConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn credentials(mut self, value: ConnectionCredentials) -> Self {
        self.credentials = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn kind(mut self, value: CreateConnectionKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn ownership(mut self, value: CreateConnectionOwnership) -> Self {
        self.ownership = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](CreateConnectionBuilder::config)
    /// - [`display_name`](CreateConnectionBuilder::display_name)
    /// - [`kind`](CreateConnectionBuilder::kind)
    pub fn build(self) -> Result<CreateConnection, BuildError> {
        Ok(CreateConnection {
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
            credentials: self.credentials,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            ownership: self.ownership,
        })
    }
}

