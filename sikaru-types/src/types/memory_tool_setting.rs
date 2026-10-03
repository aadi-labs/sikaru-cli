pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MemoryToolSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<MemoryToolSettingPolicy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<bool>,
}

impl MemoryToolSetting {
    pub fn builder() -> MemoryToolSettingBuilder {
        <MemoryToolSettingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemoryToolSettingBuilder {
    enabled: Option<bool>,
    policy: Option<MemoryToolSettingPolicy>,
    shared: Option<bool>,
    user: Option<bool>,
}

impl MemoryToolSettingBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn policy(mut self, value: MemoryToolSettingPolicy) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn shared(mut self, value: bool) -> Self {
        self.shared = Some(value);
        self
    }

    pub fn user(mut self, value: bool) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MemoryToolSetting`].
    pub fn build(self) -> Result<MemoryToolSetting, BuildError> {
        Ok(MemoryToolSetting {
            enabled: self.enabled,
            policy: self.policy,
            shared: self.shared,
            user: self.user,
        })
    }
}
