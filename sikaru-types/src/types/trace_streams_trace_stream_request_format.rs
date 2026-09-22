pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TraceStreamRequestFormat {
    OpeninferenceV1,
    OpentelemetryOtlpV1,
    HarborAtifV1,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TraceStreamRequestFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OpeninferenceV1 => serializer.serialize_str("openinference.v1"),
            Self::OpentelemetryOtlpV1 => serializer.serialize_str("opentelemetry.otlp.v1"),
            Self::HarborAtifV1 => serializer.serialize_str("harbor.atif.v1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TraceStreamRequestFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "openinference.v1" => Ok(Self::OpeninferenceV1),
            "opentelemetry.otlp.v1" => Ok(Self::OpentelemetryOtlpV1),
            "harbor.atif.v1" => Ok(Self::HarborAtifV1),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TraceStreamRequestFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpeninferenceV1 => write!(f, "openinference.v1"),
            Self::OpentelemetryOtlpV1 => write!(f, "opentelemetry.otlp.v1"),
            Self::HarborAtifV1 => write!(f, "harbor.atif.v1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
