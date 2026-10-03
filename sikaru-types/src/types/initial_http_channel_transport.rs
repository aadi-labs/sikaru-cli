pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InitialHttpChannelTransport {
    #[serde(rename = "http")]
    Http,
}
impl fmt::Display for InitialHttpChannelTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Http => "http",
        };
        write!(f, "{}", s)
    }
}
