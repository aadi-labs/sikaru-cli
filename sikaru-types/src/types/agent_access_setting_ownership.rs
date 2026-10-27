pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentAccessSettingOwnership {
    Shared,
    Personal,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentAccessSettingOwnership {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Shared => serializer.serialize_str("shared"),
            Self::Personal => serializer.serialize_str("personal"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentAccessSettingOwnership {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "shared" => Ok(Self::Shared),
            "personal" => Ok(Self::Personal),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentAccessSettingOwnership {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Shared => write!(f, "shared"),
            Self::Personal => write!(f, "personal"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
