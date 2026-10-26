pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspacePolicyView {
    #[serde(default)]
    pub credentials: Vec<String>,
    #[serde(default)]
    pub ignore: Vec<String>,
}

impl WorkspacePolicyView {
    pub fn builder() -> WorkspacePolicyViewBuilder {
        <WorkspacePolicyViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspacePolicyViewBuilder {
    credentials: Option<Vec<String>>,
    ignore: Option<Vec<String>>,
}

impl WorkspacePolicyViewBuilder {
    pub fn credentials(mut self, value: Vec<String>) -> Self {
        self.credentials = Some(value);
        self
    }

    pub fn ignore(mut self, value: Vec<String>) -> Self {
        self.ignore = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspacePolicyView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`credentials`](WorkspacePolicyViewBuilder::credentials)
    /// - [`ignore`](WorkspacePolicyViewBuilder::ignore)
    pub fn build(self) -> Result<WorkspacePolicyView, BuildError> {
        Ok(WorkspacePolicyView {
            credentials: self.credentials.ok_or_else(|| BuildError::missing_field("credentials"))?,
            ignore: self.ignore.ok_or_else(|| BuildError::missing_field("ignore"))?,
        })
    }
}
