pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Execution stage where the failure occurred.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunErrorStage {
    Admission,
    Setup,
    Model,
    Tool,
    Python,
    Recovery,
    Persistence,
    Execution,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RunErrorStage {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Admission => serializer.serialize_str("admission"),
            Self::Setup => serializer.serialize_str("setup"),
            Self::Model => serializer.serialize_str("model"),
            Self::Tool => serializer.serialize_str("tool"),
            Self::Python => serializer.serialize_str("python"),
            Self::Recovery => serializer.serialize_str("recovery"),
            Self::Persistence => serializer.serialize_str("persistence"),
            Self::Execution => serializer.serialize_str("execution"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RunErrorStage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "admission" => Ok(Self::Admission),
            "setup" => Ok(Self::Setup),
            "model" => Ok(Self::Model),
            "tool" => Ok(Self::Tool),
            "python" => Ok(Self::Python),
            "recovery" => Ok(Self::Recovery),
            "persistence" => Ok(Self::Persistence),
            "execution" => Ok(Self::Execution),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RunErrorStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Admission => write!(f, "admission"),
            Self::Setup => write!(f, "setup"),
            Self::Model => write!(f, "model"),
            Self::Tool => write!(f, "tool"),
            Self::Python => write!(f, "python"),
            Self::Recovery => write!(f, "recovery"),
            Self::Persistence => write!(f, "persistence"),
            Self::Execution => write!(f, "execution"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
