pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubscriptionStatusMode {
    #[serde(rename = "usage")]
    Usage,
}
impl fmt::Display for SubscriptionStatusMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Usage => "usage",
        };
        write!(f, "{}", s)
    }
}
