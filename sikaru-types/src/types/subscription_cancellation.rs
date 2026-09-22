pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubscriptionCancellation {
    #[serde(default)]
    pub cancel_at: i64,
    pub state: SubscriptionCancellationState,
}

impl SubscriptionCancellation {
    pub fn builder() -> SubscriptionCancellationBuilder {
        <SubscriptionCancellationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionCancellationBuilder {
    cancel_at: Option<i64>,
    state: Option<SubscriptionCancellationState>,
}

impl SubscriptionCancellationBuilder {
    pub fn cancel_at(mut self, value: i64) -> Self {
        self.cancel_at = Some(value);
        self
    }

    pub fn state(mut self, value: SubscriptionCancellationState) -> Self {
        self.state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionCancellation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cancel_at`](SubscriptionCancellationBuilder::cancel_at)
    /// - [`state`](SubscriptionCancellationBuilder::state)
    pub fn build(self) -> Result<SubscriptionCancellation, BuildError> {
        Ok(SubscriptionCancellation {
            cancel_at: self.cancel_at.ok_or_else(|| BuildError::missing_field("cancel_at"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
