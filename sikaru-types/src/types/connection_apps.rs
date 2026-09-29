pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectionApps {
    #[serde(default)]
    pub categories: Vec<ToolkitCategory>,
    #[serde(default)]
    pub items: Vec<ConnectionApp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default)]
    pub stale: bool,
}

impl ConnectionApps {
    pub fn builder() -> ConnectionAppsBuilder {
        <ConnectionAppsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionAppsBuilder {
    categories: Option<Vec<ToolkitCategory>>,
    items: Option<Vec<ConnectionApp>>,
    next_cursor: Option<String>,
    stale: Option<bool>,
}

impl ConnectionAppsBuilder {
    pub fn categories(mut self, value: Vec<ToolkitCategory>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<ConnectionApp>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn stale(mut self, value: bool) -> Self {
        self.stale = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectionApps`].
    /// This method will fail if any of the following fields are not set:
    /// - [`categories`](ConnectionAppsBuilder::categories)
    /// - [`items`](ConnectionAppsBuilder::items)
    /// - [`stale`](ConnectionAppsBuilder::stale)
    pub fn build(self) -> Result<ConnectionApps, BuildError> {
        Ok(ConnectionApps {
            categories: self.categories.ok_or_else(|| BuildError::missing_field("categories"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            next_cursor: self.next_cursor,
            stale: self.stale.ok_or_else(|| BuildError::missing_field("stale"))?,
        })
    }
}
