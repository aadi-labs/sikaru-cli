pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkspaceCheckpointInputTrigger {
    Turn,
    Stop,
    Signal,
    LeaseLost,
    Interrupted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkspaceCheckpointInputTrigger {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Turn => serializer.serialize_str("turn"),
            Self::Stop => serializer.serialize_str("stop"),
            Self::Signal => serializer.serialize_str("signal"),
            Self::LeaseLost => serializer.serialize_str("lease_lost"),
            Self::Interrupted => serializer.serialize_str("interrupted"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkspaceCheckpointInputTrigger {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "turn" => Ok(Self::Turn),
            "stop" => Ok(Self::Stop),
            "signal" => Ok(Self::Signal),
            "lease_lost" => Ok(Self::LeaseLost),
            "interrupted" => Ok(Self::Interrupted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkspaceCheckpointInputTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Turn => write!(f, "turn"),
            Self::Stop => write!(f, "stop"),
            Self::Signal => write!(f, "signal"),
            Self::LeaseLost => write!(f, "lease_lost"),
            Self::Interrupted => write!(f, "interrupted"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
