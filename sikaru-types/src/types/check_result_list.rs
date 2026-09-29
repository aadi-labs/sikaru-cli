pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CheckResultList {
    #[serde(default)]
    pub results: Vec<CheckResult>,
}

impl CheckResultList {
    pub fn builder() -> CheckResultListBuilder {
        <CheckResultListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckResultListBuilder {
    results: Option<Vec<CheckResult>>,
}

impl CheckResultListBuilder {
    pub fn results(mut self, value: Vec<CheckResult>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckResultList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](CheckResultListBuilder::results)
    pub fn build(self) -> Result<CheckResultList, BuildError> {
        Ok(CheckResultList {
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
