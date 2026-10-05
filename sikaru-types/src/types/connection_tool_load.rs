pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The latest tool discovery: loaded, unchanged, or failed (retry by discovering again).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConnectionToolLoad {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub at: f64,
    /// Tools the provider no longer offers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<Vec<String>>,
    /// Agents whose selected tools include a removed tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_used_by: Option<Vec<String>>,
    pub status: ConnectionToolLoadStatus,
    #[serde(default)]
    pub tool_count: i64,
}

impl ConnectionToolLoad {
    pub fn builder() -> ConnectionToolLoadBuilder {
        <ConnectionToolLoadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionToolLoadBuilder {
    at: Option<f64>,
    removed: Option<Vec<String>>,
    removed_used_by: Option<Vec<String>>,
    status: Option<ConnectionToolLoadStatus>,
    tool_count: Option<i64>,
}

impl ConnectionToolLoadBuilder {
    pub fn at(mut self, value: f64) -> Self {
        self.at = Some(value);
        self
    }

    pub fn removed(mut self, value: Vec<String>) -> Self {
        self.removed = Some(value);
        self
    }

    pub fn removed_used_by(mut self, value: Vec<String>) -> Self {
        self.removed_used_by = Some(value);
        self
    }

    pub fn status(mut self, value: ConnectionToolLoadStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tool_count(mut self, value: i64) -> Self {
        self.tool_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectionToolLoad`].
    /// This method will fail if any of the following fields are not set:
    /// - [`at`](ConnectionToolLoadBuilder::at)
    /// - [`status`](ConnectionToolLoadBuilder::status)
    /// - [`tool_count`](ConnectionToolLoadBuilder::tool_count)
    pub fn build(self) -> Result<ConnectionToolLoad, BuildError> {
        Ok(ConnectionToolLoad {
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
            removed: self.removed,
            removed_used_by: self.removed_used_by,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            tool_count: self.tool_count.ok_or_else(|| BuildError::missing_field("tool_count"))?,
        })
    }
}
