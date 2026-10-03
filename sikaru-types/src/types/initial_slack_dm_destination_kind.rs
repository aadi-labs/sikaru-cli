pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InitialSlackDmDestinationKind {
    #[serde(rename = "dm")]
    Dm,
}
impl fmt::Display for InitialSlackDmDestinationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Dm => "dm",
        };
        write!(f, "{}", s)
    }
}
