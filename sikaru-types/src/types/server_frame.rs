pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ServerFrame {
        #[serde(rename = "close")]
        #[non_exhaustive]
        Close {
            #[serde(default)]
            detail: String,
            reason: CloseFrameReason,
        },

        #[serde(rename = "handshake")]
        #[non_exhaustive]
        Handshake {
            #[serde(default)]
            attachment_id: String,
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers")]
            heartbeat_interval_seconds: f64,
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers")]
            heartbeat_timeout_seconds: f64,
            #[serde(default)]
            inbound_frame_limit: i64,
            #[serde(default)]
            #[serde(with = "crate::core::number_serializers")]
            inbound_frame_window_seconds: f64,
            #[serde(default)]
            max_frame_bytes: i64,
            #[serde(default)]
            owner_epoch: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            protocol: Option<HandshakeFrameProtocol>,
            transport: HandshakeFrameTransport,
            #[serde(default)]
            workspace_generation: String,
        },

        #[serde(rename = "heartbeat")]
        #[non_exhaustive]
        Heartbeat {
            #[serde(flatten)]
            data: HeartbeatFrame,
        },

        #[serde(rename = "operations")]
        #[non_exhaustive]
        Operations {
            page: WorkPage,
        },

        #[serde(rename = "receipt_accepted")]
        #[non_exhaustive]
        ReceiptAccepted {
            #[serde(default)]
            receipt: ReceiptView,
            #[serde(default)]
            run_id: String,
        },

        #[serde(rename = "receipt_rejected")]
        #[non_exhaustive]
        ReceiptRejected {
            #[serde(default)]
            detail: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            run_id: Option<String>,
            #[serde(default)]
            status: i64,
            #[serde(skip_serializing_if = "Option::is_none")]
            tool_call_id: Option<String>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ServerFrame {
    pub fn close(detail: String, reason: CloseFrameReason) -> Self {
        Self::Close { detail, reason }
    }

    pub fn handshake(attachment_id: String, heartbeat_interval_seconds: f64, heartbeat_timeout_seconds: f64, inbound_frame_limit: i64, inbound_frame_window_seconds: f64, max_frame_bytes: i64, owner_epoch: i64, transport: HandshakeFrameTransport, workspace_generation: String) -> Self {
        Self::Handshake { attachment_id, heartbeat_interval_seconds, heartbeat_timeout_seconds, inbound_frame_limit, inbound_frame_window_seconds, max_frame_bytes, owner_epoch, protocol: None, transport, workspace_generation }
    }

    pub fn heartbeat(data: HeartbeatFrame) -> Self {
        Self::Heartbeat { data }
    }

    pub fn operations(page: WorkPage) -> Self {
        Self::Operations { page }
    }

    pub fn receipt_accepted(receipt: ReceiptView, run_id: String) -> Self {
        Self::ReceiptAccepted { receipt, run_id }
    }

    pub fn receipt_rejected(detail: String, status: i64) -> Self {
        Self::ReceiptRejected { detail, run_id: None, status, tool_call_id: None }
    }

    pub fn handshake_with_protocol(attachment_id: String, heartbeat_interval_seconds: f64, heartbeat_timeout_seconds: f64, inbound_frame_limit: i64, inbound_frame_window_seconds: f64, max_frame_bytes: i64, owner_epoch: i64, protocol: HandshakeFrameProtocol, transport: HandshakeFrameTransport, workspace_generation: String) -> Self {
        Self::Handshake { attachment_id, heartbeat_interval_seconds, heartbeat_timeout_seconds, inbound_frame_limit, inbound_frame_window_seconds, max_frame_bytes, owner_epoch, protocol: Some(protocol), transport, workspace_generation }
    }

    pub fn receipt_rejected_with_run_id(detail: String, run_id: String, status: i64, tool_call_id: Option<String>) -> Self {
        Self::ReceiptRejected { detail, run_id: Some(run_id), status, tool_call_id }
    }

    pub fn receipt_rejected_with_tool_call_id(detail: String, run_id: Option<String>, status: i64, tool_call_id: String) -> Self {
        Self::ReceiptRejected { detail, run_id, status, tool_call_id: Some(tool_call_id) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
