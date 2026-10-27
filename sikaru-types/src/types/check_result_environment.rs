pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CheckResultEnvironment {
    #[serde(rename = "managed")]
    Managed,
}
impl fmt::Display for CheckResultEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Managed => "managed",
        };
        write!(f, "{}", s)
    }
}
