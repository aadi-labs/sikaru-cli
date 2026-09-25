pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkspaceCheckpointViewStatus {
    Requested,
    Unsupported,
    Publishing,
    Published,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkspaceCheckpointViewStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Requested => serializer.serialize_str("requested"),
            Self::Unsupported => serializer.serialize_str("unsupported"),
            Self::Publishing => serializer.serialize_str("publishing"),
            Self::Published => serializer.serialize_str("published"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkspaceCheckpointViewStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "requested" => Ok(Self::Requested),
            "unsupported" => Ok(Self::Unsupported),
            "publishing" => Ok(Self::Publishing),
            "published" => Ok(Self::Published),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkspaceCheckpointViewStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Requested => write!(f, "requested"),
            Self::Unsupported => write!(f, "unsupported"),
            Self::Publishing => write!(f, "publishing"),
            Self::Published => write!(f, "published"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
