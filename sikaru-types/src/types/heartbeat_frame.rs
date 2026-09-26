pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Sent by both sides every ``heartbeat_interval_seconds``; any inbound frame counts as liveness.
/// 
/// Heartbeats keep the connection alive only; the attachment lease is renewed over HTTP.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HeartbeatFrame {
}

impl HeartbeatFrame {
    pub fn builder() -> HeartbeatFrameBuilder {
        <HeartbeatFrameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HeartbeatFrameBuilder {
}

impl HeartbeatFrameBuilder {

    /// Consumes the builder and constructs a [`HeartbeatFrame`].
    pub fn build(self) -> Result<HeartbeatFrame, BuildError> {
        Ok(HeartbeatFrame {
        })
    }
}
