pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CheckList {
    #[serde(default)]
    pub checks: Vec<Check>,
}

impl CheckList {
    pub fn builder() -> CheckListBuilder {
        <CheckListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckListBuilder {
    checks: Option<Vec<Check>>,
}

impl CheckListBuilder {
    pub fn checks(mut self, value: Vec<Check>) -> Self {
        self.checks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`checks`](CheckListBuilder::checks)
    pub fn build(self) -> Result<CheckList, BuildError> {
        Ok(CheckList {
            checks: self.checks.ok_or_else(|| BuildError::missing_field("checks"))?,
        })
    }
}
