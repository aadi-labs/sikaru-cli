pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_messages
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMessagesQueryRequest {
    #[serde(default)]
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListMessagesQueryRequest {
    pub fn builder() -> ListMessagesQueryRequestBuilder {
        <ListMessagesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMessagesQueryRequestBuilder {
    account_id: Option<String>,
    limit: Option<i64>,
    cursor: Option<String>,
}

impl ListMessagesQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListMessagesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ListMessagesQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<ListMessagesQueryRequest, BuildError> {
        Ok(ListMessagesQueryRequest {
            account_id: self.account_id.ok_or_else(|| BuildError::missing_field("account_id"))?,
            limit: self.limit,
            cursor: self.cursor,
        })
    }
}

