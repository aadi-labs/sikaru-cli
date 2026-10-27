pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HandshakeFrameTransport {
    Channel,
    Poll,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for HandshakeFrameTransport {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Channel => serializer.serialize_str("channel"),
            Self::Poll => serializer.serialize_str("poll"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for HandshakeFrameTransport {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "channel" => Ok(Self::Channel),
            "poll" => Ok(Self::Poll),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for HandshakeFrameTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Channel => write!(f, "channel"),
            Self::Poll => write!(f, "poll"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
