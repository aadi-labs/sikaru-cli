pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Installation {
    #[serde(default)]
    pub app_id: String,
    #[serde(default)]
    pub bot_user_id: String,
    #[serde(default)]
    pub generation: i64,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub team_id: String,
}

impl Installation {
    pub fn builder() -> InstallationBuilder {
        <InstallationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InstallationBuilder {
    app_id: Option<String>,
    bot_user_id: Option<String>,
    generation: Option<i64>,
    id: Option<String>,
    scopes: Option<Vec<String>>,
    status: Option<String>,
    team_id: Option<String>,
}

impl InstallationBuilder {
    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn bot_user_id(mut self, value: impl Into<String>) -> Self {
        self.bot_user_id = Some(value.into());
        self
    }

    pub fn generation(mut self, value: i64) -> Self {
        self.generation = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn team_id(mut self, value: impl Into<String>) -> Self {
        self.team_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Installation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`app_id`](InstallationBuilder::app_id)
    /// - [`bot_user_id`](InstallationBuilder::bot_user_id)
    /// - [`generation`](InstallationBuilder::generation)
    /// - [`id`](InstallationBuilder::id)
    /// - [`scopes`](InstallationBuilder::scopes)
    /// - [`status`](InstallationBuilder::status)
    /// - [`team_id`](InstallationBuilder::team_id)
    pub fn build(self) -> Result<Installation, BuildError> {
        Ok(Installation {
            app_id: self.app_id.ok_or_else(|| BuildError::missing_field("app_id"))?,
            bot_user_id: self.bot_user_id.ok_or_else(|| BuildError::missing_field("bot_user_id"))?,
            generation: self.generation.ok_or_else(|| BuildError::missing_field("generation"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            scopes: self.scopes.ok_or_else(|| BuildError::missing_field("scopes"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            team_id: self.team_id.ok_or_else(|| BuildError::missing_field("team_id"))?,
        })
    }
}
