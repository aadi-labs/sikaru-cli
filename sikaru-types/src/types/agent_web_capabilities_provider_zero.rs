pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentWebCapabilitiesProviderZero {
    #[serde(rename = "sikaru")]
    Sikaru,
}
impl fmt::Display for AgentWebCapabilitiesProviderZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Sikaru => "sikaru",
        };
        write!(f, "{}", s)
    }
}
