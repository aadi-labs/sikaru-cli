pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubmitToolResultRequestStatus {
    Completed,
    Failed,
    RequiresApproval,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubmitToolResultRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Completed => serializer.serialize_str("completed"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::RequiresApproval => serializer.serialize_str("requires_approval"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubmitToolResultRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "requires_approval" => Ok(Self::RequiresApproval),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubmitToolResultRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::RequiresApproval => write!(f, "requires_approval"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
