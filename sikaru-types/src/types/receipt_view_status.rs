pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReceiptViewStatus {
    #[serde(rename = "accepted")]
    Accepted,
}
impl fmt::Display for ReceiptViewStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Accepted => "accepted",
        };
        write!(f, "{}", s)
    }
}
