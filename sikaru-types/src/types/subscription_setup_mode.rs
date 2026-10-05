pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubscriptionSetupMode {
    #[serde(rename = "usage")]
    Usage,
}
impl fmt::Display for SubscriptionSetupMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Usage => "usage",
        };
        write!(f, "{}", s)
    }
}
