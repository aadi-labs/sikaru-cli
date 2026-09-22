pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateContextRegistryChangeRequestAction {
    Review,
    Promote,
    Rollback,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateContextRegistryChangeRequestAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Review => serializer.serialize_str("review"),
            Self::Promote => serializer.serialize_str("promote"),
            Self::Rollback => serializer.serialize_str("rollback"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateContextRegistryChangeRequestAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "review" => Ok(Self::Review),
            "promote" => Ok(Self::Promote),
            "rollback" => Ok(Self::Rollback),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateContextRegistryChangeRequestAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Review => write!(f, "review"),
            Self::Promote => write!(f, "promote"),
            Self::Rollback => write!(f, "rollback"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
