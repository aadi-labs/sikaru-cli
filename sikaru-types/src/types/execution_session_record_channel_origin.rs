pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExecutionSessionRecordChannelOrigin {
    Http,
    Slack,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExecutionSessionRecordChannelOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Http => serializer.serialize_str("http"),
            Self::Slack => serializer.serialize_str("slack"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExecutionSessionRecordChannelOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "http" => Ok(Self::Http),
            "slack" => Ok(Self::Slack),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExecutionSessionRecordChannelOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http => write!(f, "http"),
            Self::Slack => write!(f, "slack"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
