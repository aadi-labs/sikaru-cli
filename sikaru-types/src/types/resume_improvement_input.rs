pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResumeImprovementInput {
}

impl ResumeImprovementInput {
    pub fn builder() -> ResumeImprovementInputBuilder {
        <ResumeImprovementInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResumeImprovementInputBuilder {
}

impl ResumeImprovementInputBuilder {

    /// Consumes the builder and constructs a [`ResumeImprovementInput`].
    pub fn build(self) -> Result<ResumeImprovementInput, BuildError> {
        Ok(ResumeImprovementInput {
        })
    }
}
