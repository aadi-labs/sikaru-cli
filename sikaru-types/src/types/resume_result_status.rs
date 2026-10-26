pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResumeResultStatus {
    Queued,
    StillBlocked,
    NotResumable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResumeResultStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Queued => serializer.serialize_str("queued"),
            Self::StillBlocked => serializer.serialize_str("still_blocked"),
            Self::NotResumable => serializer.serialize_str("not_resumable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResumeResultStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "queued" => Ok(Self::Queued),
            "still_blocked" => Ok(Self::StillBlocked),
            "not_resumable" => Ok(Self::NotResumable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResumeResultStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Queued => write!(f, "queued"),
            Self::StillBlocked => write!(f, "still_blocked"),
            Self::NotResumable => write!(f, "not_resumable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
