pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateConnection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_hosts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default)]
    pub expected_version: i64,
}

impl UpdateConnection {
    pub fn builder() -> UpdateConnectionBuilder {
        <UpdateConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateConnectionBuilder {
    allowed_hosts: Option<Vec<String>>,
    display_name: Option<String>,
    expected_version: Option<i64>,
}

impl UpdateConnectionBuilder {
    pub fn allowed_hosts(mut self, value: Vec<String>) -> Self {
        self.allowed_hosts = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn expected_version(mut self, value: i64) -> Self {
        self.expected_version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expected_version`](UpdateConnectionBuilder::expected_version)
    pub fn build(self) -> Result<UpdateConnection, BuildError> {
        Ok(UpdateConnection {
            allowed_hosts: self.allowed_hosts,
            display_name: self.display_name,
            expected_version: self.expected_version.ok_or_else(|| BuildError::missing_field("expected_version"))?,
        })
    }
}

