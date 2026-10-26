pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranscriptProgressEventKind {
    Message,
    Tool,
    Execution,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TranscriptProgressEventKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Message => serializer.serialize_str("message"),
            Self::Tool => serializer.serialize_str("tool"),
            Self::Execution => serializer.serialize_str("execution"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TranscriptProgressEventKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "message" => Ok(Self::Message),
            "tool" => Ok(Self::Tool),
            "execution" => Ok(Self::Execution),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TranscriptProgressEventKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message => write!(f, "message"),
            Self::Tool => write!(f, "tool"),
            Self::Execution => write!(f, "execution"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
