pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AgentDefinitionSourceEncoding {
    Utf8,
    Base64,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AgentDefinitionSourceEncoding {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Utf8 => serializer.serialize_str("utf-8"),
            Self::Base64 => serializer.serialize_str("base64"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AgentDefinitionSourceEncoding {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "utf-8" => Ok(Self::Utf8),
            "base64" => Ok(Self::Base64),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AgentDefinitionSourceEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Utf8 => write!(f, "utf-8"),
            Self::Base64 => write!(f, "base64"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
