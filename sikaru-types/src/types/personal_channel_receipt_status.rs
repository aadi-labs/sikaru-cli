pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PersonalChannelReceiptStatus {
    Accepted,
    Queued,
    Running,
    ApprovalNeeded,
    ConnectionNeeded,
    Completed,
    Failed,
    Suppressed,
    Expired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PersonalChannelReceiptStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Accepted => serializer.serialize_str("accepted"),
            Self::Queued => serializer.serialize_str("queued"),
            Self::Running => serializer.serialize_str("running"),
            Self::ApprovalNeeded => serializer.serialize_str("approval-needed"),
            Self::ConnectionNeeded => serializer.serialize_str("connection-needed"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::Suppressed => serializer.serialize_str("suppressed"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PersonalChannelReceiptStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "accepted" => Ok(Self::Accepted),
            "queued" => Ok(Self::Queued),
            "running" => Ok(Self::Running),
            "approval-needed" => Ok(Self::ApprovalNeeded),
            "connection-needed" => Ok(Self::ConnectionNeeded),
            "completed" => Ok(Self::Completed),
            "failed" => Ok(Self::Failed),
            "suppressed" => Ok(Self::Suppressed),
            "expired" => Ok(Self::Expired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PersonalChannelReceiptStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Accepted => write!(f, "accepted"),
            Self::Queued => write!(f, "queued"),
            Self::Running => write!(f, "running"),
            Self::ApprovalNeeded => write!(f, "approval-needed"),
            Self::ConnectionNeeded => write!(f, "connection-needed"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Suppressed => write!(f, "suppressed"),
            Self::Expired => write!(f, "expired"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
