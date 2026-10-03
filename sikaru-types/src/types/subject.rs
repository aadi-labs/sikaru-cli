pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Subject {
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub tenant_id: String,
}

impl Subject {
    pub fn builder() -> SubjectBuilder {
        <SubjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubjectBuilder {
    subject: Option<String>,
    tenant_id: Option<String>,
}

impl SubjectBuilder {
    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Subject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`subject`](SubjectBuilder::subject)
    /// - [`tenant_id`](SubjectBuilder::tenant_id)
    pub fn build(self) -> Result<Subject, BuildError> {
        Ok(Subject {
            subject: self.subject.ok_or_else(|| BuildError::missing_field("subject"))?,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
        })
    }
}

