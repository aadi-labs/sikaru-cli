pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgentDefinitionSchema {
    #[serde(rename = "sikaru.agent.contract.v1")]
    SikaruAgentContractV1,
}
impl fmt::Display for AgentDefinitionSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SikaruAgentContractV1 => "sikaru.agent.contract.v1",
        };
        write!(f, "{}", s)
    }
}
