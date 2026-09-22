pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpecialistReceipt {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    pub status: String,
    #[serde(rename = "threadId")]
    #[serde(default)]
    pub thread_id: String,
}

impl SpecialistReceipt {
    pub fn builder() -> SpecialistReceiptBuilder {
        <SpecialistReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpecialistReceiptBuilder {
    id: Option<String>,
    reason: Option<String>,
    status: Option<String>,
    thread_id: Option<String>,
}

impl SpecialistReceiptBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn thread_id(mut self, value: impl Into<String>) -> Self {
        self.thread_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpecialistReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SpecialistReceiptBuilder::id)
    /// - [`status`](SpecialistReceiptBuilder::status)
    /// - [`thread_id`](SpecialistReceiptBuilder::thread_id)
    pub fn build(self) -> Result<SpecialistReceipt, BuildError> {
        Ok(SpecialistReceipt {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reason: self.reason,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            thread_id: self.thread_id.ok_or_else(|| BuildError::missing_field("thread_id"))?,
        })
    }
}
