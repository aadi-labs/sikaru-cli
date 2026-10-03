pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InitialSlackRoomTransport {
    #[serde(rename = "slack")]
    Slack,
}
impl fmt::Display for InitialSlackRoomTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Slack => "slack",
        };
        write!(f, "{}", s)
    }
}
