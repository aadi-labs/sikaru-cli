pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckEnvironmentView {
    pub kind: CheckEnvironmentViewKind,
}

impl CheckEnvironmentView {
    pub fn builder() -> CheckEnvironmentViewBuilder {
        <CheckEnvironmentViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckEnvironmentViewBuilder {
    kind: Option<CheckEnvironmentViewKind>,
}

impl CheckEnvironmentViewBuilder {
    pub fn kind(mut self, value: CheckEnvironmentViewKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckEnvironmentView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](CheckEnvironmentViewBuilder::kind)
    pub fn build(self) -> Result<CheckEnvironmentView, BuildError> {
        Ok(CheckEnvironmentView {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
