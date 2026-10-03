pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreationResumeId {
    #[serde(default)]
    pub resume_id: String,
}

impl CreationResumeId {
    pub fn builder() -> CreationResumeIdBuilder {
        <CreationResumeIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreationResumeIdBuilder {
    resume_id: Option<String>,
}

impl CreationResumeIdBuilder {
    pub fn resume_id(mut self, value: impl Into<String>) -> Self {
        self.resume_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreationResumeId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resume_id`](CreationResumeIdBuilder::resume_id)
    pub fn build(self) -> Result<CreationResumeId, BuildError> {
        Ok(CreationResumeId {
            resume_id: self.resume_id.ok_or_else(|| BuildError::missing_field("resume_id"))?,
        })
    }
}
