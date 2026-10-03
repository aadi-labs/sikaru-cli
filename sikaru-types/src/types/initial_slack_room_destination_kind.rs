pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InitialSlackRoomDestinationKind {
    #[serde(rename = "room")]
    Room,
}
impl fmt::Display for InitialSlackRoomDestinationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Room => "room",
        };
        write!(f, "{}", s)
    }
}
