pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CheckEnvironmentViewKind {
    #[serde(rename = "managed")]
    Managed,
}
impl fmt::Display for CheckEnvironmentViewKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Managed => "managed",
        };
        write!(f, "{}", s)
    }
}
