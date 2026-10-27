pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WaitForResultStatus {
    Fired,
    Timeout,
    Unfired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WaitForResultStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Fired => serializer.serialize_str("fired"),
            Self::Timeout => serializer.serialize_str("timeout"),
            Self::Unfired => serializer.serialize_str("unfired"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WaitForResultStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "fired" => Ok(Self::Fired),
            "timeout" => Ok(Self::Timeout),
            "unfired" => Ok(Self::Unfired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WaitForResultStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fired => write!(f, "fired"),
            Self::Timeout => write!(f, "timeout"),
            Self::Unfired => write!(f, "unfired"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
