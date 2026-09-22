pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptView {
    #[serde(default)]
    pub created: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiptViewStatus>,
    #[serde(default)]
    pub tool_call_id: String,
}

impl ReceiptView {
    pub fn builder() -> ReceiptViewBuilder {
        <ReceiptViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptViewBuilder {
    created: Option<bool>,
    status: Option<ReceiptViewStatus>,
    tool_call_id: Option<String>,
}

impl ReceiptViewBuilder {
    pub fn created(mut self, value: bool) -> Self {
        self.created = Some(value);
        self
    }

    pub fn status(mut self, value: ReceiptViewStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceiptView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](ReceiptViewBuilder::created)
    /// - [`tool_call_id`](ReceiptViewBuilder::tool_call_id)
    pub fn build(self) -> Result<ReceiptView, BuildError> {
        Ok(ReceiptView {
            created: self.created.ok_or_else(|| BuildError::missing_field("created"))?,
            status: self.status,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
        })
    }
}
