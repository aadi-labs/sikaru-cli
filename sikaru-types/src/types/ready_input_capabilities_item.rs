pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReadyInputCapabilitiesItem {
    ComputeExecute,
    BashRun,
    FilesystemCheckpointV1,
    ConditionWaitsV1,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReadyInputCapabilitiesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ComputeExecute => serializer.serialize_str("compute.execute"),
            Self::BashRun => serializer.serialize_str("bash.run"),
            Self::FilesystemCheckpointV1 => serializer.serialize_str("filesystem-checkpoint-v1"),
            Self::ConditionWaitsV1 => serializer.serialize_str("condition-waits-v1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReadyInputCapabilitiesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "compute.execute" => Ok(Self::ComputeExecute),
            "bash.run" => Ok(Self::BashRun),
            "filesystem-checkpoint-v1" => Ok(Self::FilesystemCheckpointV1),
            "condition-waits-v1" => Ok(Self::ConditionWaitsV1),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReadyInputCapabilitiesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComputeExecute => write!(f, "compute.execute"),
            Self::BashRun => write!(f, "bash.run"),
            Self::FilesystemCheckpointV1 => write!(f, "filesystem-checkpoint-v1"),
            Self::ConditionWaitsV1 => write!(f, "condition-waits-v1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
