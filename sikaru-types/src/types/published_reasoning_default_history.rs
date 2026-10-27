pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PublishedReasoningDefaultHistory {
    Disabled,
    Interleaved,
    Preserved,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PublishedReasoningDefaultHistory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Disabled => serializer.serialize_str("disabled"),
            Self::Interleaved => serializer.serialize_str("interleaved"),
            Self::Preserved => serializer.serialize_str("preserved"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PublishedReasoningDefaultHistory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "disabled" => Ok(Self::Disabled),
            "interleaved" => Ok(Self::Interleaved),
            "preserved" => Ok(Self::Preserved),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PublishedReasoningDefaultHistory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Disabled => write!(f, "disabled"),
            Self::Interleaved => write!(f, "interleaved"),
            Self::Preserved => write!(f, "preserved"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
