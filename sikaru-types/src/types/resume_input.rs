pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResumeInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_run_id: Option<String>,
}

impl ResumeInput {
    pub fn builder() -> ResumeInputBuilder {
        <ResumeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResumeInputBuilder {
    source_run_id: Option<String>,
}

impl ResumeInputBuilder {
    pub fn source_run_id(mut self, value: impl Into<String>) -> Self {
        self.source_run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResumeInput`].
    pub fn build(self) -> Result<ResumeInput, BuildError> {
        Ok(ResumeInput {
            source_run_id: self.source_run_id,
        })
    }
}
