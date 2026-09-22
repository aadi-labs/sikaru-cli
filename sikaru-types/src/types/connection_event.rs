pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionEvent {
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub actor: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(default)]
    pub id: String,
}

impl ConnectionEvent {
    pub fn builder() -> ConnectionEventBuilder {
        <ConnectionEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionEventBuilder {
    action: Option<String>,
    actor: Option<String>,
    created_at: Option<f64>,
    id: Option<String>,
}

impl ConnectionEventBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn actor(mut self, value: impl Into<String>) -> Self {
        self.actor = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectionEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](ConnectionEventBuilder::action)
    /// - [`actor`](ConnectionEventBuilder::actor)
    /// - [`created_at`](ConnectionEventBuilder::created_at)
    /// - [`id`](ConnectionEventBuilder::id)
    pub fn build(self) -> Result<ConnectionEvent, BuildError> {
        Ok(ConnectionEvent {
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            actor: self.actor.ok_or_else(|| BuildError::missing_field("actor"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
