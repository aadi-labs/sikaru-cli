pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SubscriptionCancellationState {
    #[serde(rename = "cancellation_pending")]
    CancellationPending,
}
impl fmt::Display for SubscriptionCancellationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CancellationPending => "cancellation_pending",
        };
        write!(f, "{}", s)
    }
}
