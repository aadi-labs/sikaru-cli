pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ImprovementInputObjective {
    Quality,
    Cost,
    Balanced,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ImprovementInputObjective {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Quality => serializer.serialize_str("quality"),
            Self::Cost => serializer.serialize_str("cost"),
            Self::Balanced => serializer.serialize_str("balanced"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ImprovementInputObjective {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "quality" => Ok(Self::Quality),
            "cost" => Ok(Self::Cost),
            "balanced" => Ok(Self::Balanced),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ImprovementInputObjective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Quality => write!(f, "quality"),
            Self::Cost => write!(f, "cost"),
            Self::Balanced => write!(f, "balanced"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
