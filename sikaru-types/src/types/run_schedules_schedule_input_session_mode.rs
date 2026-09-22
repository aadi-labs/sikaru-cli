pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScheduleInputSessionMode {
    Persistent,
    Fresh,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScheduleInputSessionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Persistent => serializer.serialize_str("persistent"),
            Self::Fresh => serializer.serialize_str("fresh"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScheduleInputSessionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "persistent" => Ok(Self::Persistent),
            "fresh" => Ok(Self::Fresh),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScheduleInputSessionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Persistent => write!(f, "persistent"),
            Self::Fresh => write!(f, "fresh"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
