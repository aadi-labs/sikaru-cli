pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListChangesetsChangesetsRequestStatus {
    Draft,
    ReadyForReview,
    Approved,
    Staging,
    Production,
    Rejected,
    RolledBack,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListChangesetsChangesetsRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("draft"),
            Self::ReadyForReview => serializer.serialize_str("ready_for_review"),
            Self::Approved => serializer.serialize_str("approved"),
            Self::Staging => serializer.serialize_str("staging"),
            Self::Production => serializer.serialize_str("production"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::RolledBack => serializer.serialize_str("rolled_back"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListChangesetsChangesetsRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft" => Ok(Self::Draft),
            "ready_for_review" => Ok(Self::ReadyForReview),
            "approved" => Ok(Self::Approved),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            "rejected" => Ok(Self::Rejected),
            "rolled_back" => Ok(Self::RolledBack),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListChangesetsChangesetsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::ReadyForReview => write!(f, "ready_for_review"),
            Self::Approved => write!(f, "approved"),
            Self::Staging => write!(f, "staging"),
            Self::Production => write!(f, "production"),
            Self::Rejected => write!(f, "rejected"),
            Self::RolledBack => write!(f, "rolled_back"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
