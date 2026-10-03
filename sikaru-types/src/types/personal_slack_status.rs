pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PersonalSlackStatus {
    pub status: PersonalSlackStatusStatus,
    #[serde(default)]
    pub verification_id: String,
}

impl PersonalSlackStatus {
    pub fn builder() -> PersonalSlackStatusBuilder {
        <PersonalSlackStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonalSlackStatusBuilder {
    status: Option<PersonalSlackStatusStatus>,
    verification_id: Option<String>,
}

impl PersonalSlackStatusBuilder {
    pub fn status(mut self, value: PersonalSlackStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn verification_id(mut self, value: impl Into<String>) -> Self {
        self.verification_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PersonalSlackStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](PersonalSlackStatusBuilder::status)
    /// - [`verification_id`](PersonalSlackStatusBuilder::verification_id)
    pub fn build(self) -> Result<PersonalSlackStatus, BuildError> {
        Ok(PersonalSlackStatus {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            verification_id: self.verification_id.ok_or_else(|| BuildError::missing_field("verification_id"))?,
        })
    }
}
