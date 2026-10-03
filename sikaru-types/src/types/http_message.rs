pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HttpMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    /// Text content, at most 16 KiB encoded as UTF-8.
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub message_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy: Option<HttpMessagePrivacy>,
}

impl HttpMessage {
    pub fn builder() -> HttpMessageBuilder {
        <HttpMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HttpMessageBuilder {
    actor: Option<String>,
    content: Option<String>,
    conversation_id: Option<String>,
    message_id: Option<String>,
    privacy: Option<HttpMessagePrivacy>,
}

impl HttpMessageBuilder {
    pub fn actor(mut self, value: impl Into<String>) -> Self {
        self.actor = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn privacy(mut self, value: HttpMessagePrivacy) -> Self {
        self.privacy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HttpMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](HttpMessageBuilder::content)
    /// - [`conversation_id`](HttpMessageBuilder::conversation_id)
    /// - [`message_id`](HttpMessageBuilder::message_id)
    pub fn build(self) -> Result<HttpMessage, BuildError> {
        Ok(HttpMessage {
            actor: self.actor,
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
            privacy: self.privacy,
        })
    }
}

