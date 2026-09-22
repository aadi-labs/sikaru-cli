pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopeType {
    Global,
    Organization,
    Project,
    Product,
    Agent,
    Workflow,
    Cohort,
    User,
    Run,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ScopeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Global => serializer.serialize_str("global"),
            Self::Organization => serializer.serialize_str("organization"),
            Self::Project => serializer.serialize_str("project"),
            Self::Product => serializer.serialize_str("product"),
            Self::Agent => serializer.serialize_str("agent"),
            Self::Workflow => serializer.serialize_str("workflow"),
            Self::Cohort => serializer.serialize_str("cohort"),
            Self::User => serializer.serialize_str("user"),
            Self::Run => serializer.serialize_str("run"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ScopeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "global" => Ok(Self::Global),
            "organization" => Ok(Self::Organization),
            "project" => Ok(Self::Project),
            "product" => Ok(Self::Product),
            "agent" => Ok(Self::Agent),
            "workflow" => Ok(Self::Workflow),
            "cohort" => Ok(Self::Cohort),
            "user" => Ok(Self::User),
            "run" => Ok(Self::Run),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ScopeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Global => write!(f, "global"),
            Self::Organization => write!(f, "organization"),
            Self::Project => write!(f, "project"),
            Self::Product => write!(f, "product"),
            Self::Agent => write!(f, "agent"),
            Self::Workflow => write!(f, "workflow"),
            Self::Cohort => write!(f, "cohort"),
            Self::User => write!(f, "user"),
            Self::Run => write!(f, "run"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
