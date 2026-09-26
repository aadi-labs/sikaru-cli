pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ClientFrame {
        #[serde(rename = "heartbeat")]
        #[non_exhaustive]
        Heartbeat {
            #[serde(flatten)]
            data: HeartbeatFrame,
        },

        #[serde(rename = "receipt")]
        #[non_exhaustive]
        Receipt {
            receipt: ReceiptInput,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ClientFrame {
    pub fn heartbeat(data: HeartbeatFrame) -> Self {
        Self::Heartbeat { data }
    }

    pub fn receipt(receipt: ReceiptInput) -> Self {
        Self::Receipt { receipt }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
