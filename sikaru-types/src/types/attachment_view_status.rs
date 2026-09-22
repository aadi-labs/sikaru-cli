pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttachmentViewStatus {
    Pending,
    Starting,
    Ready,
    Stopping,
    Cleaned,
    StartupExpired,
    RecoveryRequired,
    Abandoned,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AttachmentViewStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Starting => serializer.serialize_str("starting"),
            Self::Ready => serializer.serialize_str("ready"),
            Self::Stopping => serializer.serialize_str("stopping"),
            Self::Cleaned => serializer.serialize_str("cleaned"),
            Self::StartupExpired => serializer.serialize_str("startup_expired"),
            Self::RecoveryRequired => serializer.serialize_str("recovery_required"),
            Self::Abandoned => serializer.serialize_str("abandoned"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AttachmentViewStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "starting" => Ok(Self::Starting),
            "ready" => Ok(Self::Ready),
            "stopping" => Ok(Self::Stopping),
            "cleaned" => Ok(Self::Cleaned),
            "startup_expired" => Ok(Self::StartupExpired),
            "recovery_required" => Ok(Self::RecoveryRequired),
            "abandoned" => Ok(Self::Abandoned),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AttachmentViewStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Starting => write!(f, "starting"),
            Self::Ready => write!(f, "ready"),
            Self::Stopping => write!(f, "stopping"),
            Self::Cleaned => write!(f, "cleaned"),
            Self::StartupExpired => write!(f, "startup_expired"),
            Self::RecoveryRequired => write!(f, "recovery_required"),
            Self::Abandoned => write!(f, "abandoned"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
