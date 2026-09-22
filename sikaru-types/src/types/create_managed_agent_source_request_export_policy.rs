pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateManagedAgentSourceRequestExportPolicy {
    CustomerSourceExportable,
    CustomerSourcePrivate,
    ReferenceOnly,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateManagedAgentSourceRequestExportPolicy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CustomerSourceExportable => serializer.serialize_str("customer_source_exportable"),
            Self::CustomerSourcePrivate => serializer.serialize_str("customer_source_private"),
            Self::ReferenceOnly => serializer.serialize_str("reference_only"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateManagedAgentSourceRequestExportPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "customer_source_exportable" => Ok(Self::CustomerSourceExportable),
            "customer_source_private" => Ok(Self::CustomerSourcePrivate),
            "reference_only" => Ok(Self::ReferenceOnly),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateManagedAgentSourceRequestExportPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CustomerSourceExportable => write!(f, "customer_source_exportable"),
            Self::CustomerSourcePrivate => write!(f, "customer_source_private"),
            Self::ReferenceOnly => write!(f, "reference_only"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
