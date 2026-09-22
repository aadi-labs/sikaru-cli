pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrialStatusState {
    Available,
    Active,
    Exhausted,
    Expired,
    Paused,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TrialStatusState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Available => serializer.serialize_str("available"),
            Self::Active => serializer.serialize_str("active"),
            Self::Exhausted => serializer.serialize_str("exhausted"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TrialStatusState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "available" => Ok(Self::Available),
            "active" => Ok(Self::Active),
            "exhausted" => Ok(Self::Exhausted),
            "expired" => Ok(Self::Expired),
            "paused" => Ok(Self::Paused),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TrialStatusState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Available => write!(f, "available"),
            Self::Active => write!(f, "active"),
            Self::Exhausted => write!(f, "exhausted"),
            Self::Expired => write!(f, "expired"),
            Self::Paused => write!(f, "paused"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
