pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CloseFrameReason {
    TransportPoll,
    CredentialInvalid,
    EnvironmentDisabled,
    StaleOwner,
    WorkspaceGenerationChanged,
    AttachmentStopped,
    HeartbeatTimeout,
    RateLimited,
    FrameTooLarge,
    InvalidFrame,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CloseFrameReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TransportPoll => serializer.serialize_str("transport_poll"),
            Self::CredentialInvalid => serializer.serialize_str("credential_invalid"),
            Self::EnvironmentDisabled => serializer.serialize_str("environment_disabled"),
            Self::StaleOwner => serializer.serialize_str("stale_owner"),
            Self::WorkspaceGenerationChanged => serializer.serialize_str("workspace_generation_changed"),
            Self::AttachmentStopped => serializer.serialize_str("attachment_stopped"),
            Self::HeartbeatTimeout => serializer.serialize_str("heartbeat_timeout"),
            Self::RateLimited => serializer.serialize_str("rate_limited"),
            Self::FrameTooLarge => serializer.serialize_str("frame_too_large"),
            Self::InvalidFrame => serializer.serialize_str("invalid_frame"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CloseFrameReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "transport_poll" => Ok(Self::TransportPoll),
            "credential_invalid" => Ok(Self::CredentialInvalid),
            "environment_disabled" => Ok(Self::EnvironmentDisabled),
            "stale_owner" => Ok(Self::StaleOwner),
            "workspace_generation_changed" => Ok(Self::WorkspaceGenerationChanged),
            "attachment_stopped" => Ok(Self::AttachmentStopped),
            "heartbeat_timeout" => Ok(Self::HeartbeatTimeout),
            "rate_limited" => Ok(Self::RateLimited),
            "frame_too_large" => Ok(Self::FrameTooLarge),
            "invalid_frame" => Ok(Self::InvalidFrame),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CloseFrameReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TransportPoll => write!(f, "transport_poll"),
            Self::CredentialInvalid => write!(f, "credential_invalid"),
            Self::EnvironmentDisabled => write!(f, "environment_disabled"),
            Self::StaleOwner => write!(f, "stale_owner"),
            Self::WorkspaceGenerationChanged => write!(f, "workspace_generation_changed"),
            Self::AttachmentStopped => write!(f, "attachment_stopped"),
            Self::HeartbeatTimeout => write!(f, "heartbeat_timeout"),
            Self::RateLimited => write!(f, "rate_limited"),
            Self::FrameTooLarge => write!(f, "frame_too_large"),
            Self::InvalidFrame => write!(f, "invalid_frame"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
