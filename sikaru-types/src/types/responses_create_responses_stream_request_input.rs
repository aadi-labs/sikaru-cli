pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateResponsesStreamRequestInput {
        String(String),

        StringToValueMapList(Vec<HashMap<String, serde_json::Value>>),
}

impl CreateResponsesStreamRequestInput {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_string_to_value_map_list(&self) -> bool {
        matches!(self, Self::StringToValueMapList(_))
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

    pub fn as_string_to_value_map_list(&self) -> Option<&Vec<HashMap<String, serde_json::Value>>> {
        match self {
                    Self::StringToValueMapList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string_to_value_map_list(self) -> Option<Vec<HashMap<String, serde_json::Value>>> {
        match self {
                    Self::StringToValueMapList(value) => Some(value),
                    _ => None,
                }
    }
}
