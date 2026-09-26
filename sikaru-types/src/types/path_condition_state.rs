pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathConditionState {
    Exists,
    Missing,
    Changed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PathConditionState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Exists => serializer.serialize_str("exists"),
            Self::Missing => serializer.serialize_str("missing"),
            Self::Changed => serializer.serialize_str("changed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PathConditionState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "exists" => Ok(Self::Exists),
            "missing" => Ok(Self::Missing),
            "changed" => Ok(Self::Changed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PathConditionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exists => write!(f, "exists"),
            Self::Missing => write!(f, "missing"),
            Self::Changed => write!(f, "changed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
