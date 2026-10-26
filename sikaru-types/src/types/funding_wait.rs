pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FundingWait {
    #[serde(default)]
    pub count: i64,
    #[serde(rename = "firstAt")]
    #[serde(default)]
    pub first_at: String,
    #[serde(rename = "lastAt")]
    #[serde(default)]
    pub last_at: String,
    pub reason: FundingWaitReason,
}

impl FundingWait {
    pub fn builder() -> FundingWaitBuilder {
        <FundingWaitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FundingWaitBuilder {
    count: Option<i64>,
    first_at: Option<String>,
    last_at: Option<String>,
    reason: Option<FundingWaitReason>,
}

impl FundingWaitBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn first_at(mut self, value: impl Into<String>) -> Self {
        self.first_at = Some(value.into());
        self
    }

    pub fn last_at(mut self, value: impl Into<String>) -> Self {
        self.last_at = Some(value.into());
        self
    }

    pub fn reason(mut self, value: FundingWaitReason) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FundingWait`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](FundingWaitBuilder::count)
    /// - [`first_at`](FundingWaitBuilder::first_at)
    /// - [`last_at`](FundingWaitBuilder::last_at)
    /// - [`reason`](FundingWaitBuilder::reason)
    pub fn build(self) -> Result<FundingWait, BuildError> {
        Ok(FundingWait {
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            first_at: self.first_at.ok_or_else(|| BuildError::missing_field("first_at"))?,
            last_at: self.last_at.ok_or_else(|| BuildError::missing_field("last_at"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
