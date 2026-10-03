pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BindingStatus {
    pub status: BindingStatusStatus,
}

impl BindingStatus {
    pub fn builder() -> BindingStatusBuilder {
        <BindingStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BindingStatusBuilder {
    status: Option<BindingStatusStatus>,
}

impl BindingStatusBuilder {
    pub fn status(mut self, value: BindingStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BindingStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](BindingStatusBuilder::status)
    pub fn build(self) -> Result<BindingStatus, BuildError> {
        Ok(BindingStatus {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

