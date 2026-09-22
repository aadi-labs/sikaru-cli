pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionConfigAuth {
    None,
    Bearer,
    ApiKey,
    Oauth,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConnectionConfigAuth {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Bearer => serializer.serialize_str("bearer"),
            Self::ApiKey => serializer.serialize_str("api_key"),
            Self::Oauth => serializer.serialize_str("oauth"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConnectionConfigAuth {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "bearer" => Ok(Self::Bearer),
            "api_key" => Ok(Self::ApiKey),
            "oauth" => Ok(Self::Oauth),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConnectionConfigAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Bearer => write!(f, "bearer"),
            Self::ApiKey => write!(f, "api_key"),
            Self::Oauth => write!(f, "oauth"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
