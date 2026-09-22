pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateConnectionKind {
    Mcp,
    Composio,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateConnectionKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mcp => serializer.serialize_str("mcp"),
            Self::Composio => serializer.serialize_str("composio"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateConnectionKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mcp" => Ok(Self::Mcp),
            "composio" => Ok(Self::Composio),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateConnectionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mcp => write!(f, "mcp"),
            Self::Composio => write!(f, "composio"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
