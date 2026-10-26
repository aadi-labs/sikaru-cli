pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranscriptProgressBlockAvailability {
    Retained,
    Redacted,
    Expired,
    NotRecorded,
    Unsupported,
    NotLoaded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TranscriptProgressBlockAvailability {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Retained => serializer.serialize_str("retained"),
            Self::Redacted => serializer.serialize_str("redacted"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::NotRecorded => serializer.serialize_str("not_recorded"),
            Self::Unsupported => serializer.serialize_str("unsupported"),
            Self::NotLoaded => serializer.serialize_str("not_loaded"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TranscriptProgressBlockAvailability {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "retained" => Ok(Self::Retained),
            "redacted" => Ok(Self::Redacted),
            "expired" => Ok(Self::Expired),
            "not_recorded" => Ok(Self::NotRecorded),
            "unsupported" => Ok(Self::Unsupported),
            "not_loaded" => Ok(Self::NotLoaded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TranscriptProgressBlockAvailability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Retained => write!(f, "retained"),
            Self::Redacted => write!(f, "redacted"),
            Self::Expired => write!(f, "expired"),
            Self::NotRecorded => write!(f, "not_recorded"),
            Self::Unsupported => write!(f, "unsupported"),
            Self::NotLoaded => write!(f, "not_loaded"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
