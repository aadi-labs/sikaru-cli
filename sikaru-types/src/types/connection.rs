pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Connection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status: Option<String>,
    #[serde(default)]
    pub config: ConnectionConfig,
    #[serde(default)]
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    #[serde(default)]
    pub grants: Vec<ConnectionGrant>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub owner: String,
    #[serde(default)]
    pub ownership: String,
    #[serde(default)]
    pub revocation: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_load: Option<ConnectionToolLoad>,
    #[serde(default)]
    pub tools: Vec<ConnectionTool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_by: Option<ConnectionUsage>,
    #[serde(default)]
    pub version: i64,
}

impl Connection {
    pub fn builder() -> ConnectionBuilder {
        <ConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionBuilder {
    account_status: Option<String>,
    config: Option<ConnectionConfig>,
    display_name: Option<String>,
    expires_at: Option<f64>,
    grants: Option<Vec<ConnectionGrant>>,
    id: Option<String>,
    kind: Option<String>,
    owner: Option<String>,
    ownership: Option<String>,
    revocation: Option<String>,
    slug: Option<String>,
    status: Option<String>,
    tool_count: Option<i64>,
    tool_load: Option<ConnectionToolLoad>,
    tools: Option<Vec<ConnectionTool>>,
    used_by: Option<ConnectionUsage>,
    version: Option<i64>,
}

impl ConnectionBuilder {
    pub fn account_status(mut self, value: impl Into<String>) -> Self {
        self.account_status = Some(value.into());
        self
    }

    pub fn config(mut self, value: ConnectionConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: f64) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn grants(mut self, value: Vec<ConnectionGrant>) -> Self {
        self.grants = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn owner(mut self, value: impl Into<String>) -> Self {
        self.owner = Some(value.into());
        self
    }

    pub fn ownership(mut self, value: impl Into<String>) -> Self {
        self.ownership = Some(value.into());
        self
    }

    pub fn revocation(mut self, value: impl Into<String>) -> Self {
        self.revocation = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn tool_count(mut self, value: i64) -> Self {
        self.tool_count = Some(value);
        self
    }

    pub fn tool_load(mut self, value: ConnectionToolLoad) -> Self {
        self.tool_load = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<ConnectionTool>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn used_by(mut self, value: ConnectionUsage) -> Self {
        self.used_by = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Connection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config`](ConnectionBuilder::config)
    /// - [`display_name`](ConnectionBuilder::display_name)
    /// - [`grants`](ConnectionBuilder::grants)
    /// - [`id`](ConnectionBuilder::id)
    /// - [`kind`](ConnectionBuilder::kind)
    /// - [`owner`](ConnectionBuilder::owner)
    /// - [`ownership`](ConnectionBuilder::ownership)
    /// - [`revocation`](ConnectionBuilder::revocation)
    /// - [`slug`](ConnectionBuilder::slug)
    /// - [`status`](ConnectionBuilder::status)
    /// - [`tools`](ConnectionBuilder::tools)
    /// - [`version`](ConnectionBuilder::version)
    pub fn build(self) -> Result<Connection, BuildError> {
        Ok(Connection {
            account_status: self.account_status,
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            expires_at: self.expires_at,
            grants: self.grants.ok_or_else(|| BuildError::missing_field("grants"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            owner: self.owner.ok_or_else(|| BuildError::missing_field("owner"))?,
            ownership: self.ownership.ok_or_else(|| BuildError::missing_field("ownership"))?,
            revocation: self.revocation.ok_or_else(|| BuildError::missing_field("revocation"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            tool_count: self.tool_count,
            tool_load: self.tool_load,
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
            used_by: self.used_by,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
