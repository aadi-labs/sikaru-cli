pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StartHarnessRunRequestRunMode {
    Interactive,
    BackgroundSubagent,
    SandboxExecution,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for StartHarnessRunRequestRunMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Interactive => serializer.serialize_str("interactive"),
            Self::BackgroundSubagent => serializer.serialize_str("background_subagent"),
            Self::SandboxExecution => serializer.serialize_str("sandbox_execution"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for StartHarnessRunRequestRunMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "interactive" => Ok(Self::Interactive),
            "background_subagent" => Ok(Self::BackgroundSubagent),
            "sandbox_execution" => Ok(Self::SandboxExecution),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for StartHarnessRunRequestRunMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Interactive => write!(f, "interactive"),
            Self::BackgroundSubagent => write!(f, "background_subagent"),
            Self::SandboxExecution => write!(f, "sandbox_execution"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
