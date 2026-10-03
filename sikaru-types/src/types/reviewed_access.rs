pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReviewedAccess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_hosts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub tools: HashMap<String, String>,
}

impl ReviewedAccess {
    pub fn builder() -> ReviewedAccessBuilder {
        <ReviewedAccessBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReviewedAccessBuilder {
    allowed_hosts: Option<Vec<String>>,
    ownership: Option<String>,
    policy: Option<String>,
    resource_id: Option<String>,
    tools: Option<HashMap<String, String>>,
}

impl ReviewedAccessBuilder {
    pub fn allowed_hosts(mut self, value: Vec<String>) -> Self {
        self.allowed_hosts = Some(value);
        self
    }

    pub fn ownership(mut self, value: impl Into<String>) -> Self {
        self.ownership = Some(value.into());
        self
    }

    pub fn policy(mut self, value: impl Into<String>) -> Self {
        self.policy = Some(value.into());
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn tools(mut self, value: HashMap<String, String>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReviewedAccess`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tools`](ReviewedAccessBuilder::tools)
    pub fn build(self) -> Result<ReviewedAccess, BuildError> {
        Ok(ReviewedAccess {
            allowed_hosts: self.allowed_hosts,
            ownership: self.ownership,
            policy: self.policy,
            resource_id: self.resource_id,
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
        })
    }
}
