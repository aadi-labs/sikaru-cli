pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueuePage {
    #[serde(default)]
    pub attachments: Vec<AttachmentView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_after_seconds: Option<i64>,
}

impl QueuePage {
    pub fn builder() -> QueuePageBuilder {
        <QueuePageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueuePageBuilder {
    attachments: Option<Vec<AttachmentView>>,
    poll_after_seconds: Option<i64>,
}

impl QueuePageBuilder {
    pub fn attachments(mut self, value: Vec<AttachmentView>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn poll_after_seconds(mut self, value: i64) -> Self {
        self.poll_after_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueuePage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attachments`](QueuePageBuilder::attachments)
    pub fn build(self) -> Result<QueuePage, BuildError> {
        Ok(QueuePage {
            attachments: self.attachments.ok_or_else(|| BuildError::missing_field("attachments"))?,
            poll_after_seconds: self.poll_after_seconds,
        })
    }
}
