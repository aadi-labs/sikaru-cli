pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BuiltInToolSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<BuiltInToolSettingPolicy>,
}

impl BuiltInToolSetting {
    pub fn builder() -> BuiltInToolSettingBuilder {
        <BuiltInToolSettingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BuiltInToolSettingBuilder {
    enabled: Option<bool>,
    policy: Option<BuiltInToolSettingPolicy>,
}

impl BuiltInToolSettingBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn policy(mut self, value: BuiltInToolSettingPolicy) -> Self {
        self.policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BuiltInToolSetting`].
    pub fn build(self) -> Result<BuiltInToolSetting, BuildError> {
        Ok(BuiltInToolSetting {
            enabled: self.enabled,
            policy: self.policy,
        })
    }
}
