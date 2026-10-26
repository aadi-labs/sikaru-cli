pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ManagedRunMeasurementStatus {
    Available,
    Unmeasured,
    Unavailable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ManagedRunMeasurementStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Available => serializer.serialize_str("available"),
            Self::Unmeasured => serializer.serialize_str("unmeasured"),
            Self::Unavailable => serializer.serialize_str("unavailable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ManagedRunMeasurementStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "available" => Ok(Self::Available),
            "unmeasured" => Ok(Self::Unmeasured),
            "unavailable" => Ok(Self::Unavailable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ManagedRunMeasurementStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Available => write!(f, "available"),
            Self::Unmeasured => write!(f, "unmeasured"),
            Self::Unavailable => write!(f, "unavailable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
