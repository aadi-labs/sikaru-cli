pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperationViewMethod {
    BashRun,
    BashStart,
    BashRead,
    BashWait,
    BashCancel,
    WorkspaceWriteText,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OperationViewMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BashRun => serializer.serialize_str("bash.run"),
            Self::BashStart => serializer.serialize_str("bash.start"),
            Self::BashRead => serializer.serialize_str("bash.read"),
            Self::BashWait => serializer.serialize_str("bash.wait"),
            Self::BashCancel => serializer.serialize_str("bash.cancel"),
            Self::WorkspaceWriteText => serializer.serialize_str("workspace.write_text"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OperationViewMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "bash.run" => Ok(Self::BashRun),
            "bash.start" => Ok(Self::BashStart),
            "bash.read" => Ok(Self::BashRead),
            "bash.wait" => Ok(Self::BashWait),
            "bash.cancel" => Ok(Self::BashCancel),
            "workspace.write_text" => Ok(Self::WorkspaceWriteText),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OperationViewMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BashRun => write!(f, "bash.run"),
            Self::BashStart => write!(f, "bash.start"),
            Self::BashRead => write!(f, "bash.read"),
            Self::BashWait => write!(f, "bash.wait"),
            Self::BashCancel => write!(f, "bash.cancel"),
            Self::WorkspaceWriteText => write!(f, "workspace.write_text"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
