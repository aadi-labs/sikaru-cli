pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AgentWebCapabilitiesProvider {
        AgentWebCapabilitiesProviderZero(AgentWebCapabilitiesProviderZero),

        ConnectionToolRef(ConnectionToolRef),
}

impl AgentWebCapabilitiesProvider {
    pub fn is_agent_web_capabilities_provider_zero(&self) -> bool {
        matches!(self, Self::AgentWebCapabilitiesProviderZero(_))
    }

    pub fn is_connection_tool_ref(&self) -> bool {
        matches!(self, Self::ConnectionToolRef(_))
    }


    pub fn as_agent_web_capabilities_provider_zero(&self) -> Option<&AgentWebCapabilitiesProviderZero> {
        match self {
                    Self::AgentWebCapabilitiesProviderZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_agent_web_capabilities_provider_zero(self) -> Option<AgentWebCapabilitiesProviderZero> {
        match self {
                    Self::AgentWebCapabilitiesProviderZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_connection_tool_ref(&self) -> Option<&ConnectionToolRef> {
        match self {
                    Self::ConnectionToolRef(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_connection_tool_ref(self) -> Option<ConnectionToolRef> {
        match self {
                    Self::ConnectionToolRef(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for AgentWebCapabilitiesProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AgentWebCapabilitiesProviderZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConnectionToolRef(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
