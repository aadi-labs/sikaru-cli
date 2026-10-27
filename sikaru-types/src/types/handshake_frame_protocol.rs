pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HandshakeFrameProtocol {
    #[serde(rename = "sikaru-compute-channel-v1")]
    SikaruComputeChannelV1,
}
impl fmt::Display for HandshakeFrameProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SikaruComputeChannelV1 => "sikaru-compute-channel-v1",
        };
        write!(f, "{}", s)
    }
}
