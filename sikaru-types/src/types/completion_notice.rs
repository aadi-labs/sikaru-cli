pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A finished process with the last ``tail_bytes`` of its output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompletionNotice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub omitted_before: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returncode: Option<i64>,
    pub status: CompletionNoticeStatus,
    #[serde(default)]
    pub tail: String,
}

impl CompletionNotice {
    pub fn builder() -> CompletionNoticeBuilder {
        <CompletionNoticeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompletionNoticeBuilder {
    error: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    omitted_before: Option<i64>,
    returncode: Option<i64>,
    status: Option<CompletionNoticeStatus>,
    tail: Option<String>,
}

impl CompletionNoticeBuilder {
    pub fn error(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.error = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn omitted_before(mut self, value: i64) -> Self {
        self.omitted_before = Some(value);
        self
    }

    pub fn returncode(mut self, value: i64) -> Self {
        self.returncode = Some(value);
        self
    }

    pub fn status(mut self, value: CompletionNoticeStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tail(mut self, value: impl Into<String>) -> Self {
        self.tail = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompletionNotice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CompletionNoticeBuilder::id)
    /// - [`omitted_before`](CompletionNoticeBuilder::omitted_before)
    /// - [`status`](CompletionNoticeBuilder::status)
    /// - [`tail`](CompletionNoticeBuilder::tail)
    pub fn build(self) -> Result<CompletionNotice, BuildError> {
        Ok(CompletionNotice {
            error: self.error,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            omitted_before: self.omitted_before.ok_or_else(|| BuildError::missing_field("omitted_before"))?,
            returncode: self.returncode,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            tail: self.tail.ok_or_else(|| BuildError::missing_field("tail"))?,
        })
    }
}
