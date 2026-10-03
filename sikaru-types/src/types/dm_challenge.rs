pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DmChallenge {
    #[serde(default)]
    pub bot_user_id: String,
    #[serde(default)]
    pub challenge: String,
    #[serde(default)]
    pub verification_id: String,
}

impl DmChallenge {
    pub fn builder() -> DmChallengeBuilder {
        <DmChallengeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DmChallengeBuilder {
    bot_user_id: Option<String>,
    challenge: Option<String>,
    verification_id: Option<String>,
}

impl DmChallengeBuilder {
    pub fn bot_user_id(mut self, value: impl Into<String>) -> Self {
        self.bot_user_id = Some(value.into());
        self
    }

    pub fn challenge(mut self, value: impl Into<String>) -> Self {
        self.challenge = Some(value.into());
        self
    }

    pub fn verification_id(mut self, value: impl Into<String>) -> Self {
        self.verification_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DmChallenge`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bot_user_id`](DmChallengeBuilder::bot_user_id)
    /// - [`challenge`](DmChallengeBuilder::challenge)
    /// - [`verification_id`](DmChallengeBuilder::verification_id)
    pub fn build(self) -> Result<DmChallenge, BuildError> {
        Ok(DmChallenge {
            bot_user_id: self.bot_user_id.ok_or_else(|| BuildError::missing_field("bot_user_id"))?,
            challenge: self.challenge.ok_or_else(|| BuildError::missing_field("challenge"))?,
            verification_id: self.verification_id.ok_or_else(|| BuildError::missing_field("verification_id"))?,
        })
    }
}
