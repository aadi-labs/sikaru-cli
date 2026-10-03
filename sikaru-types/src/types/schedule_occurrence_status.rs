pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScheduleOccurrenceStatus {
    Admitted,
    SkippedOverlap,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScheduleOccurrenceStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Admitted => serializer.serialize_str("admitted"),
            Self::SkippedOverlap => serializer.serialize_str("skipped_overlap"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScheduleOccurrenceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "admitted" => Ok(Self::Admitted),
            "skipped_overlap" => Ok(Self::SkippedOverlap),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScheduleOccurrenceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admitted => write!(f, "admitted"),
            Self::SkippedOverlap => write!(f, "skipped_overlap"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
