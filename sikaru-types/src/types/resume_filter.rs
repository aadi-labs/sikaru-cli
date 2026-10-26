pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResumeFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<ResumeFilterReason>,
}

impl ResumeFilter {
    pub fn builder() -> ResumeFilterBuilder {
        <ResumeFilterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResumeFilterBuilder {
    reason: Option<ResumeFilterReason>,
}

impl ResumeFilterBuilder {
    pub fn reason(mut self, value: ResumeFilterReason) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResumeFilter`].
    pub fn build(self) -> Result<ResumeFilter, BuildError> {
        Ok(ResumeFilter {
            reason: self.reason,
        })
    }
}

