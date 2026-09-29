pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltInToolSettingPolicy {
    Allow,
    RequireApproval,
    Deny,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BuiltInToolSettingPolicy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Allow => serializer.serialize_str("allow"),
            Self::RequireApproval => serializer.serialize_str("require_approval"),
            Self::Deny => serializer.serialize_str("deny"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BuiltInToolSettingPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "allow" => Ok(Self::Allow),
            "require_approval" => Ok(Self::RequireApproval),
            "deny" => Ok(Self::Deny),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BuiltInToolSettingPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allow => write!(f, "allow"),
            Self::RequireApproval => write!(f, "require_approval"),
            Self::Deny => write!(f, "deny"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
