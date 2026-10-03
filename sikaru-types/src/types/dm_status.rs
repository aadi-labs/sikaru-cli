pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DmStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
    pub status: DmStatusStatus,
    #[serde(default)]
    pub verification_id: String,
}

impl DmStatus {
    pub fn builder() -> DmStatusBuilder {
        <DmStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DmStatusBuilder {
    destination_id: Option<String>,
    recipient_id: Option<String>,
    status: Option<DmStatusStatus>,
    verification_id: Option<String>,
}

impl DmStatusBuilder {
    pub fn destination_id(mut self, value: impl Into<String>) -> Self {
        self.destination_id = Some(value.into());
        self
    }

    pub fn recipient_id(mut self, value: impl Into<String>) -> Self {
        self.recipient_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: DmStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn verification_id(mut self, value: impl Into<String>) -> Self {
        self.verification_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DmStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](DmStatusBuilder::status)
    /// - [`verification_id`](DmStatusBuilder::verification_id)
    pub fn build(self) -> Result<DmStatus, BuildError> {
        Ok(DmStatus {
            destination_id: self.destination_id,
            recipient_id: self.recipient_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            verification_id: self.verification_id.ok_or_else(|| BuildError::missing_field("verification_id"))?,
        })
    }
}
