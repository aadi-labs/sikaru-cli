pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Fires when a TCP connection to ``host``:``port`` succeeds.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PortCondition {
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: i64,
}

impl PortCondition {
    pub fn builder() -> PortConditionBuilder {
        <PortConditionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PortConditionBuilder {
    host: Option<String>,
    port: Option<i64>,
}

impl PortConditionBuilder {
    pub fn host(mut self, value: impl Into<String>) -> Self {
        self.host = Some(value.into());
        self
    }

    pub fn port(mut self, value: i64) -> Self {
        self.port = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PortCondition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`host`](PortConditionBuilder::host)
    /// - [`port`](PortConditionBuilder::port)
    pub fn build(self) -> Result<PortCondition, BuildError> {
        Ok(PortCondition {
            host: self.host.ok_or_else(|| BuildError::missing_field("host"))?,
            port: self.port.ok_or_else(|| BuildError::missing_field("port"))?,
        })
    }
}
