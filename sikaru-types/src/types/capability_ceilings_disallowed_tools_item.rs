pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CapabilityCeilingsDisallowedToolsItem {
    Bash,
    Workspace,
    Agents,
    Memory,
    WebSearch,
    WebFetch,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CapabilityCeilingsDisallowedToolsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Bash => serializer.serialize_str("bash"),
            Self::Workspace => serializer.serialize_str("workspace"),
            Self::Agents => serializer.serialize_str("agents"),
            Self::Memory => serializer.serialize_str("memory"),
            Self::WebSearch => serializer.serialize_str("web_search"),
            Self::WebFetch => serializer.serialize_str("web_fetch"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CapabilityCeilingsDisallowedToolsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bash" => Ok(Self::Bash),
            "workspace" => Ok(Self::Workspace),
            "agents" => Ok(Self::Agents),
            "memory" => Ok(Self::Memory),
            "web_search" => Ok(Self::WebSearch),
            "web_fetch" => Ok(Self::WebFetch),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CapabilityCeilingsDisallowedToolsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bash => write!(f, "bash"),
            Self::Workspace => write!(f, "workspace"),
            Self::Agents => write!(f, "agents"),
            Self::Memory => write!(f, "memory"),
            Self::WebSearch => write!(f, "web_search"),
            Self::WebFetch => write!(f, "web_fetch"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
