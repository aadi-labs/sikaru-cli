pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PersonalSlackChallenge {
    #[serde(default)]
    pub challenge: String,
    #[serde(default)]
    pub verification_id: String,
}

impl PersonalSlackChallenge {
    pub fn builder() -> PersonalSlackChallengeBuilder {
        <PersonalSlackChallengeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonalSlackChallengeBuilder {
    challenge: Option<String>,
    verification_id: Option<String>,
}

impl PersonalSlackChallengeBuilder {
    pub fn challenge(mut self, value: impl Into<String>) -> Self {
        self.challenge = Some(value.into());
        self
    }

    pub fn verification_id(mut self, value: impl Into<String>) -> Self {
        self.verification_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PersonalSlackChallenge`].
    /// This method will fail if any of the following fields are not set:
    /// - [`challenge`](PersonalSlackChallengeBuilder::challenge)
    /// - [`verification_id`](PersonalSlackChallengeBuilder::verification_id)
    pub fn build(self) -> Result<PersonalSlackChallenge, BuildError> {
        Ok(PersonalSlackChallenge {
            challenge: self.challenge.ok_or_else(|| BuildError::missing_field("challenge"))?,
            verification_id: self.verification_id.ok_or_else(|| BuildError::missing_field("verification_id"))?,
        })
    }
}
