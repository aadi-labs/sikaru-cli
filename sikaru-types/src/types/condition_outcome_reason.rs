pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConditionOutcomeReason {
    Deadline,
    Pending,
    Exited,
    Error,
    Unsupported,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConditionOutcomeReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Deadline => serializer.serialize_str("deadline"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Exited => serializer.serialize_str("exited"),
            Self::Error => serializer.serialize_str("error"),
            Self::Unsupported => serializer.serialize_str("unsupported"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConditionOutcomeReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "deadline" => Ok(Self::Deadline),
            "pending" => Ok(Self::Pending),
            "exited" => Ok(Self::Exited),
            "error" => Ok(Self::Error),
            "unsupported" => Ok(Self::Unsupported),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConditionOutcomeReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deadline => write!(f, "deadline"),
            Self::Pending => write!(f, "pending"),
            Self::Exited => write!(f, "exited"),
            Self::Error => write!(f, "error"),
            Self::Unsupported => write!(f, "unsupported"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
