pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckEnvironment {
    /// managed runs in the Sikaru sandbox.
    pub kind: CheckEnvironmentKind,
}

impl CheckEnvironment {
    pub fn builder() -> CheckEnvironmentBuilder {
        <CheckEnvironmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckEnvironmentBuilder {
    kind: Option<CheckEnvironmentKind>,
}

impl CheckEnvironmentBuilder {
    pub fn kind(mut self, value: CheckEnvironmentKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckEnvironment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](CheckEnvironmentBuilder::kind)
    pub fn build(self) -> Result<CheckEnvironment, BuildError> {
        Ok(CheckEnvironment {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
