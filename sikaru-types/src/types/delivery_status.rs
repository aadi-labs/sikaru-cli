pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeliveryStatus {
    Pending,
    Sending,
    Delivered,
    Failed,
    DeliveryUnknown,
    Suppressed,
    Expired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DeliveryStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Sending => serializer.serialize_str("sending"),
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::DeliveryUnknown => serializer.serialize_str("delivery_unknown"),
            Self::Suppressed => serializer.serialize_str("suppressed"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DeliveryStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "sending" => Ok(Self::Sending),
            "delivered" => Ok(Self::Delivered),
            "failed" => Ok(Self::Failed),
            "delivery_unknown" => Ok(Self::DeliveryUnknown),
            "suppressed" => Ok(Self::Suppressed),
            "expired" => Ok(Self::Expired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DeliveryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Sending => write!(f, "sending"),
            Self::Delivered => write!(f, "delivered"),
            Self::Failed => write!(f, "failed"),
            Self::DeliveryUnknown => write!(f, "delivery_unknown"),
            Self::Suppressed => write!(f, "suppressed"),
            Self::Expired => write!(f, "expired"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
