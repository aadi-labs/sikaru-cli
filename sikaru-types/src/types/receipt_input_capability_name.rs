pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReceiptInputCapabilityName {
    #[serde(rename = "compute.execute")]
    ComputeExecute,
}
impl fmt::Display for ReceiptInputCapabilityName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ComputeExecute => "compute.execute",
        };
        write!(f, "{}", s)
    }
}
