pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScheduleNotice {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub recipient: String,
}

impl ScheduleNotice {
    pub fn builder() -> ScheduleNoticeBuilder {
        <ScheduleNoticeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleNoticeBuilder {
    created_at: Option<f64>,
    reason: Option<String>,
    recipient: Option<String>,
}

impl ScheduleNoticeBuilder {
    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn recipient(mut self, value: impl Into<String>) -> Self {
        self.recipient = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScheduleNotice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ScheduleNoticeBuilder::created_at)
    /// - [`reason`](ScheduleNoticeBuilder::reason)
    /// - [`recipient`](ScheduleNoticeBuilder::recipient)
    pub fn build(self) -> Result<ScheduleNotice, BuildError> {
        Ok(ScheduleNotice {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            recipient: self.recipient.ok_or_else(|| BuildError::missing_field("recipient"))?,
        })
    }
}
