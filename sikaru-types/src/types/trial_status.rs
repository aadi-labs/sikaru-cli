pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TrialStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<i64>,
    #[serde(default)]
    pub remaining_usd: String,
    pub state: TrialStatusState,
}

impl TrialStatus {
    pub fn builder() -> TrialStatusBuilder {
        <TrialStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TrialStatusBuilder {
    ends_at: Option<i64>,
    remaining_usd: Option<String>,
    state: Option<TrialStatusState>,
}

impl TrialStatusBuilder {
    pub fn ends_at(mut self, value: i64) -> Self {
        self.ends_at = Some(value);
        self
    }

    pub fn remaining_usd(mut self, value: impl Into<String>) -> Self {
        self.remaining_usd = Some(value.into());
        self
    }

    pub fn state(mut self, value: TrialStatusState) -> Self {
        self.state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TrialStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`remaining_usd`](TrialStatusBuilder::remaining_usd)
    /// - [`state`](TrialStatusBuilder::state)
    pub fn build(self) -> Result<TrialStatus, BuildError> {
        Ok(TrialStatus {
            ends_at: self.ends_at,
            remaining_usd: self.remaining_usd.ok_or_else(|| BuildError::missing_field("remaining_usd"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
