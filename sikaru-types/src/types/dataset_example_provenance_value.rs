pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DatasetExampleProvenanceValue {
    Customer,
    Human,
    SikaruDerived,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DatasetExampleProvenanceValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Customer => serializer.serialize_str("customer"),
            Self::Human => serializer.serialize_str("human"),
            Self::SikaruDerived => serializer.serialize_str("sikaru_derived"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DatasetExampleProvenanceValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "customer" => Ok(Self::Customer),
            "human" => Ok(Self::Human),
            "sikaru_derived" => Ok(Self::SikaruDerived),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DatasetExampleProvenanceValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Customer => write!(f, "customer"),
            Self::Human => write!(f, "human"),
            Self::SikaruDerived => write!(f, "sikaru_derived"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
