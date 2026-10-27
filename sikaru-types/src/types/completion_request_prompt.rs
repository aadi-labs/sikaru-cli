pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CompletionRequestPrompt {
        String(String),

        ValueList(Vec<serde_json::Value>),
}

impl CompletionRequestPrompt {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_value_list(&self) -> bool {
        matches!(self, Self::ValueList(_))
    }


    pub fn as_string(&self) -> Option<&str> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_value_list(&self) -> Option<&Vec<serde_json::Value>> {
        match self {
                    Self::ValueList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_value_list(self) -> Option<Vec<serde_json::Value>> {
        match self {
                    Self::ValueList(value) => Some(value),
                    _ => None,
                }
    }
}
