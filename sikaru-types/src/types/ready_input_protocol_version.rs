pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReadyInputProtocolVersion {
    #[serde(rename = "sikaru-compute-v1")]
    SikaruComputeV1,
}
impl fmt::Display for ReadyInputProtocolVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SikaruComputeV1 => "sikaru-compute-v1",
        };
        write!(f, "{}", s)
    }
}
