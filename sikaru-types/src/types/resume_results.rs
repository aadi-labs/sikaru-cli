pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResumeResults {
    #[serde(default)]
    pub results: Vec<ResumeResult>,
}

impl ResumeResults {
    pub fn builder() -> ResumeResultsBuilder {
        <ResumeResultsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResumeResultsBuilder {
    results: Option<Vec<ResumeResult>>,
}

impl ResumeResultsBuilder {
    pub fn results(mut self, value: Vec<ResumeResult>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResumeResults`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](ResumeResultsBuilder::results)
    pub fn build(self) -> Result<ResumeResults, BuildError> {
        Ok(ResumeResults {
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
