pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Draft sessions test the pinned agent definition without activation. Creating or appending draft sessions also requires harness:write.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SessionInputEnvironment {
    Production,
    Draft,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SessionInputEnvironment {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Production => serializer.serialize_str("production"),
            Self::Draft => serializer.serialize_str("draft"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SessionInputEnvironment {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "production" => Ok(Self::Production),
            "draft" => Ok(Self::Draft),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SessionInputEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Production => write!(f, "production"),
            Self::Draft => write!(f, "draft"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
