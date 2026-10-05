pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BatchItemOutcome {
    Added,
    Duplicate,
    Skipped,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BatchItemOutcome {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Added => serializer.serialize_str("added"),
            Self::Duplicate => serializer.serialize_str("duplicate"),
            Self::Skipped => serializer.serialize_str("skipped"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BatchItemOutcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "added" => Ok(Self::Added),
            "duplicate" => Ok(Self::Duplicate),
            "skipped" => Ok(Self::Skipped),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BatchItemOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Added => write!(f, "added"),
            Self::Duplicate => write!(f, "duplicate"),
            Self::Skipped => write!(f, "skipped"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
