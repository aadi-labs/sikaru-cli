pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateManagedAgentRequestInitialChannel {
        InitialHttpChannel(InitialHttpChannel),

        InitialSlackRoom(InitialSlackRoom),

        InitialSlackDm(InitialSlackDm),
}

impl CreateManagedAgentRequestInitialChannel {
    pub fn is_initial_http_channel(&self) -> bool {
        matches!(self, Self::InitialHttpChannel(_))
    }

    pub fn is_initial_slack_room(&self) -> bool {
        matches!(self, Self::InitialSlackRoom(_))
    }

    pub fn is_initial_slack_dm(&self) -> bool {
        matches!(self, Self::InitialSlackDm(_))
    }


    pub fn as_initial_http_channel(&self) -> Option<&InitialHttpChannel> {
        match self {
                    Self::InitialHttpChannel(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_initial_http_channel(self) -> Option<InitialHttpChannel> {
        match self {
                    Self::InitialHttpChannel(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_initial_slack_room(&self) -> Option<&InitialSlackRoom> {
        match self {
                    Self::InitialSlackRoom(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_initial_slack_room(self) -> Option<InitialSlackRoom> {
        match self {
                    Self::InitialSlackRoom(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_initial_slack_dm(&self) -> Option<&InitialSlackDm> {
        match self {
                    Self::InitialSlackDm(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_initial_slack_dm(self) -> Option<InitialSlackDm> {
        match self {
                    Self::InitialSlackDm(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CreateManagedAgentRequestInitialChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitialHttpChannel(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::InitialSlackRoom(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::InitialSlackDm(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
