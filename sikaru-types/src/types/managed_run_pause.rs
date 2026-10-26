pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ManagedRunPause {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub requires_action: bool,
    #[serde(default)]
    pub resumable: bool,
}

impl ManagedRunPause {
    pub fn builder() -> ManagedRunPauseBuilder {
        <ManagedRunPauseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ManagedRunPauseBuilder {
    message: Option<String>,
    reason: Option<String>,
    requires_action: Option<bool>,
    resumable: Option<bool>,
}

impl ManagedRunPauseBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn requires_action(mut self, value: bool) -> Self {
        self.requires_action = Some(value);
        self
    }

    pub fn resumable(mut self, value: bool) -> Self {
        self.resumable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ManagedRunPause`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ManagedRunPauseBuilder::message)
    /// - [`reason`](ManagedRunPauseBuilder::reason)
    /// - [`requires_action`](ManagedRunPauseBuilder::requires_action)
    /// - [`resumable`](ManagedRunPauseBuilder::resumable)
    pub fn build(self) -> Result<ManagedRunPause, BuildError> {
        Ok(ManagedRunPause {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            requires_action: self.requires_action.ok_or_else(|| BuildError::missing_field("requires_action"))?,
            resumable: self.resumable.ok_or_else(|| BuildError::missing_field("resumable"))?,
        })
    }
}
