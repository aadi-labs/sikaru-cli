pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateTraceImportRequestMode {
    Historical,
    Selective,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateTraceImportRequestMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Historical => serializer.serialize_str("historical"),
            Self::Selective => serializer.serialize_str("selective"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateTraceImportRequestMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "historical" => Ok(Self::Historical),
            "selective" => Ok(Self::Selective),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateTraceImportRequestMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Historical => write!(f, "historical"),
            Self::Selective => write!(f, "selective"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
