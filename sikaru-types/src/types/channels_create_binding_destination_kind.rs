pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateBindingDestinationKind {
    Room,
    Dm,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateBindingDestinationKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Room => serializer.serialize_str("room"),
            Self::Dm => serializer.serialize_str("dm"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateBindingDestinationKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "room" => Ok(Self::Room),
            "dm" => Ok(Self::Dm),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateBindingDestinationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Room => write!(f, "room"),
            Self::Dm => write!(f, "dm"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
