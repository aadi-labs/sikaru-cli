pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReconcileView {
    pub attachment: AttachmentView,
    #[serde(default)]
    pub receipts: Vec<ReceiptView>,
}

impl ReconcileView {
    pub fn builder() -> ReconcileViewBuilder {
        <ReconcileViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReconcileViewBuilder {
    attachment: Option<AttachmentView>,
    receipts: Option<Vec<ReceiptView>>,
}

impl ReconcileViewBuilder {
    pub fn attachment(mut self, value: AttachmentView) -> Self {
        self.attachment = Some(value);
        self
    }

    pub fn receipts(mut self, value: Vec<ReceiptView>) -> Self {
        self.receipts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReconcileView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attachment`](ReconcileViewBuilder::attachment)
    /// - [`receipts`](ReconcileViewBuilder::receipts)
    pub fn build(self) -> Result<ReconcileView, BuildError> {
        Ok(ReconcileView {
            attachment: self.attachment.ok_or_else(|| BuildError::missing_field("attachment"))?,
            receipts: self.receipts.ok_or_else(|| BuildError::missing_field("receipts"))?,
        })
    }
}
