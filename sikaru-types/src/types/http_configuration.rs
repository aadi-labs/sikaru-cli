pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HttpConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_cap: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_minute: Option<i64>,
}

impl HttpConfiguration {
    pub fn builder() -> HttpConfigurationBuilder {
        <HttpConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HttpConfigurationBuilder {
    name: Option<String>,
    pending_cap: Option<i64>,
    per_minute: Option<i64>,
}

impl HttpConfigurationBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn pending_cap(mut self, value: i64) -> Self {
        self.pending_cap = Some(value);
        self
    }

    pub fn per_minute(mut self, value: i64) -> Self {
        self.per_minute = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HttpConfiguration`].
    pub fn build(self) -> Result<HttpConfiguration, BuildError> {
        Ok(HttpConfiguration {
            name: self.name,
            pending_cap: self.pending_cap,
            per_minute: self.per_minute,
        })
    }
}

