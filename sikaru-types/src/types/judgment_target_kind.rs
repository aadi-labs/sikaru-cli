pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JudgmentTargetKind {
    Message,
    Span,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for JudgmentTargetKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Message => serializer.serialize_str("message"),
            Self::Span => serializer.serialize_str("span"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for JudgmentTargetKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "message" => Ok(Self::Message),
            "span" => Ok(Self::Span),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for JudgmentTargetKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Message => write!(f, "message"),
            Self::Span => write!(f, "span"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
