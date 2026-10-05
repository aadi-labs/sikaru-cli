pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UploadRequestFormat {
    Csv,
    Jsonl,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UploadRequestFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Csv => serializer.serialize_str("csv"),
            Self::Jsonl => serializer.serialize_str("jsonl"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UploadRequestFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "csv" => Ok(Self::Csv),
            "jsonl" => Ok(Self::Jsonl),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UploadRequestFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Csv => write!(f, "csv"),
            Self::Jsonl => write!(f, "jsonl"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
