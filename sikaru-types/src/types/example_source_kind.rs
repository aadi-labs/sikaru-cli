pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExampleSourceKind {
    Run,
    Step,
    Upload,
    Flag,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExampleSourceKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Run => serializer.serialize_str("run"),
            Self::Step => serializer.serialize_str("step"),
            Self::Upload => serializer.serialize_str("upload"),
            Self::Flag => serializer.serialize_str("flag"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExampleSourceKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "run" => Ok(Self::Run),
            "step" => Ok(Self::Step),
            "upload" => Ok(Self::Upload),
            "flag" => Ok(Self::Flag),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExampleSourceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Run => write!(f, "run"),
            Self::Step => write!(f, "step"),
            Self::Upload => write!(f, "upload"),
            Self::Flag => write!(f, "flag"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
