pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateMemoryRegistryChangeRequestAction {
    Review,
    Approve,
    Reject,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateMemoryRegistryChangeRequestAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Review => serializer.serialize_str("review"),
            Self::Approve => serializer.serialize_str("approve"),
            Self::Reject => serializer.serialize_str("reject"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateMemoryRegistryChangeRequestAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "review" => Ok(Self::Review),
            "approve" => Ok(Self::Approve),
            "reject" => Ok(Self::Reject),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateMemoryRegistryChangeRequestAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Review => write!(f, "review"),
            Self::Approve => write!(f, "approve"),
            Self::Reject => write!(f, "reject"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
