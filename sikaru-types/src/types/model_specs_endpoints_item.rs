pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModelSpecsEndpointsItem {
    Chat,
    Completions,
    Responses,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ModelSpecsEndpointsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Chat => serializer.serialize_str("chat"),
            Self::Completions => serializer.serialize_str("completions"),
            Self::Responses => serializer.serialize_str("responses"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ModelSpecsEndpointsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "chat" => Ok(Self::Chat),
            "completions" => Ok(Self::Completions),
            "responses" => Ok(Self::Responses),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ModelSpecsEndpointsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Chat => write!(f, "chat"),
            Self::Completions => write!(f, "completions"),
            Self::Responses => write!(f, "responses"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
