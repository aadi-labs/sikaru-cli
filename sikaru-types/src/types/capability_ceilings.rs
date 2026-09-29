pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Per-project ceilings. The defaults impose no limit.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CapabilityCeilings {
    #[serde(rename = "allowedGitHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_git_hosts: Option<Vec<String>>,
    #[serde(rename = "disallowedTools")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallowed_tools: Option<Vec<CapabilityCeilingsDisallowedToolsItem>>,
    #[serde(rename = "domainDenylist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_denylist: Option<Vec<String>>,
    #[serde(rename = "egressEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_enabled: Option<bool>,
}

impl CapabilityCeilings {
    pub fn builder() -> CapabilityCeilingsBuilder {
        <CapabilityCeilingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilityCeilingsBuilder {
    allowed_git_hosts: Option<Vec<String>>,
    disallowed_tools: Option<Vec<CapabilityCeilingsDisallowedToolsItem>>,
    domain_denylist: Option<Vec<String>>,
    egress_enabled: Option<bool>,
}

impl CapabilityCeilingsBuilder {
    pub fn allowed_git_hosts(mut self, value: Vec<String>) -> Self {
        self.allowed_git_hosts = Some(value);
        self
    }

    pub fn disallowed_tools(mut self, value: Vec<CapabilityCeilingsDisallowedToolsItem>) -> Self {
        self.disallowed_tools = Some(value);
        self
    }

    pub fn domain_denylist(mut self, value: Vec<String>) -> Self {
        self.domain_denylist = Some(value);
        self
    }

    pub fn egress_enabled(mut self, value: bool) -> Self {
        self.egress_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilityCeilings`].
    pub fn build(self) -> Result<CapabilityCeilings, BuildError> {
        Ok(CapabilityCeilings {
            allowed_git_hosts: self.allowed_git_hosts,
            disallowed_tools: self.disallowed_tools,
            domain_denylist: self.domain_denylist,
            egress_enabled: self.egress_enabled,
        })
    }
}
