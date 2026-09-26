pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Fires when a GET of ``url`` returns ``status``.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HttpCondition {
    #[serde(default)]
    pub status: i64,
    #[serde(default)]
    pub url: String,
}

impl HttpCondition {
    pub fn builder() -> HttpConditionBuilder {
        <HttpConditionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HttpConditionBuilder {
    status: Option<i64>,
    url: Option<String>,
}

impl HttpConditionBuilder {
    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HttpCondition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](HttpConditionBuilder::status)
    /// - [`url`](HttpConditionBuilder::url)
    pub fn build(self) -> Result<HttpCondition, BuildError> {
        Ok(HttpCondition {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
