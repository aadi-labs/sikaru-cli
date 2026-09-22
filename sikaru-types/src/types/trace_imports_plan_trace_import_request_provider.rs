pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlanTraceImportRequestProvider {
    Langsmith,
    Langfuse,
    Braintrust,
    Logfire,
    Harbor,
    Opentelemetry,
    Openinference,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PlanTraceImportRequestProvider {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Langsmith => serializer.serialize_str("langsmith"),
            Self::Langfuse => serializer.serialize_str("langfuse"),
            Self::Braintrust => serializer.serialize_str("braintrust"),
            Self::Logfire => serializer.serialize_str("logfire"),
            Self::Harbor => serializer.serialize_str("harbor"),
            Self::Opentelemetry => serializer.serialize_str("opentelemetry"),
            Self::Openinference => serializer.serialize_str("openinference"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PlanTraceImportRequestProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "langsmith" => Ok(Self::Langsmith),
            "langfuse" => Ok(Self::Langfuse),
            "braintrust" => Ok(Self::Braintrust),
            "logfire" => Ok(Self::Logfire),
            "harbor" => Ok(Self::Harbor),
            "opentelemetry" => Ok(Self::Opentelemetry),
            "openinference" => Ok(Self::Openinference),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PlanTraceImportRequestProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Langsmith => write!(f, "langsmith"),
            Self::Langfuse => write!(f, "langfuse"),
            Self::Braintrust => write!(f, "braintrust"),
            Self::Logfire => write!(f, "logfire"),
            Self::Harbor => write!(f, "harbor"),
            Self::Opentelemetry => write!(f, "opentelemetry"),
            Self::Openinference => write!(f, "openinference"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
