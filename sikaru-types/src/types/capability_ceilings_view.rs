pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CapabilityCeilingsView {
    #[serde(rename = "canEdit")]
    #[serde(default)]
    pub can_edit: bool,
    #[serde(default)]
    pub ceilings: CapabilityCeilings,
}

impl CapabilityCeilingsView {
    pub fn builder() -> CapabilityCeilingsViewBuilder {
        <CapabilityCeilingsViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapabilityCeilingsViewBuilder {
    can_edit: Option<bool>,
    ceilings: Option<CapabilityCeilings>,
}

impl CapabilityCeilingsViewBuilder {
    pub fn can_edit(mut self, value: bool) -> Self {
        self.can_edit = Some(value);
        self
    }

    pub fn ceilings(mut self, value: CapabilityCeilings) -> Self {
        self.ceilings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapabilityCeilingsView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`can_edit`](CapabilityCeilingsViewBuilder::can_edit)
    /// - [`ceilings`](CapabilityCeilingsViewBuilder::ceilings)
    pub fn build(self) -> Result<CapabilityCeilingsView, BuildError> {
        Ok(CapabilityCeilingsView {
            can_edit: self.can_edit.ok_or_else(|| BuildError::missing_field("can_edit"))?,
            ceilings: self.ceilings.ok_or_else(|| BuildError::missing_field("ceilings"))?,
        })
    }
}
