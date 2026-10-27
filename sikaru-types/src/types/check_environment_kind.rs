pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// managed runs in the Sikaru sandbox.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CheckEnvironmentKind {
    #[serde(rename = "managed")]
    Managed,
}
impl fmt::Display for CheckEnvironmentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Managed => "managed",
        };
        write!(f, "{}", s)
    }
}
