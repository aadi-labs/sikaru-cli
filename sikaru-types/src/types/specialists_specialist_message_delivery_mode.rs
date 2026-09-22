pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpecialistMessageDeliveryMode {
    Steer,
    Queue,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SpecialistMessageDeliveryMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Steer => serializer.serialize_str("steer"),
            Self::Queue => serializer.serialize_str("queue"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SpecialistMessageDeliveryMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "steer" => Ok(Self::Steer),
            "queue" => Ok(Self::Queue),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SpecialistMessageDeliveryMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Steer => write!(f, "steer"),
            Self::Queue => write!(f, "queue"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
