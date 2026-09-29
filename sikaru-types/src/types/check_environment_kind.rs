pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// managed runs in the Sikaru sandbox; local runs on a named compute environment.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckEnvironmentKind {
    Managed,
    Local,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckEnvironmentKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Managed => serializer.serialize_str("managed"),
            Self::Local => serializer.serialize_str("local"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckEnvironmentKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "managed" => Ok(Self::Managed),
            "local" => Ok(Self::Local),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckEnvironmentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Managed => write!(f, "managed"),
            Self::Local => write!(f, "local"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
