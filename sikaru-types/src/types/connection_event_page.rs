pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionEventPage {
    #[serde(default)]
    pub items: Vec<ConnectionEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl ConnectionEventPage {
    pub fn builder() -> ConnectionEventPageBuilder {
        <ConnectionEventPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionEventPageBuilder {
    items: Option<Vec<ConnectionEvent>>,
    next_cursor: Option<String>,
}

impl ConnectionEventPageBuilder {
    pub fn items(mut self, value: Vec<ConnectionEvent>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectionEventPage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](ConnectionEventPageBuilder::items)
    pub fn build(self) -> Result<ConnectionEventPage, BuildError> {
        Ok(ConnectionEventPage {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            next_cursor: self.next_cursor,
        })
    }
}
