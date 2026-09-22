pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkPageExecutionPhase {
    Idle,
    Running,
    WaitingApproval,
    Terminal,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkPageExecutionPhase {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Idle => serializer.serialize_str("idle"),
            Self::Running => serializer.serialize_str("running"),
            Self::WaitingApproval => serializer.serialize_str("waiting_approval"),
            Self::Terminal => serializer.serialize_str("terminal"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkPageExecutionPhase {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "idle" => Ok(Self::Idle),
            "running" => Ok(Self::Running),
            "waiting_approval" => Ok(Self::WaitingApproval),
            "terminal" => Ok(Self::Terminal),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkPageExecutionPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Idle => write!(f, "idle"),
            Self::Running => write!(f, "running"),
            Self::WaitingApproval => write!(f, "waiting_approval"),
            Self::Terminal => write!(f, "terminal"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
