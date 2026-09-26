pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum ConditionOutcomeCondition {
        #[serde(rename = "exit")]
        #[non_exhaustive]
        Exit {
            #[serde(flatten)]
            data: ExitCondition,
        },

        #[serde(rename = "http")]
        #[non_exhaustive]
        Http {
            #[serde(flatten)]
            data: HttpCondition,
        },

        #[serde(rename = "log")]
        #[non_exhaustive]
        Log {
            #[serde(flatten)]
            data: LogCondition,
        },

        #[serde(rename = "path")]
        #[non_exhaustive]
        Path {
            #[serde(flatten)]
            data: PathCondition,
        },

        #[serde(rename = "port")]
        #[non_exhaustive]
        Port {
            #[serde(flatten)]
            data: PortCondition,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ConditionOutcomeCondition {
    pub fn exit(data: ExitCondition) -> Self {
        Self::Exit { data }
    }

    pub fn http(data: HttpCondition) -> Self {
        Self::Http { data }
    }

    pub fn log(data: LogCondition) -> Self {
        Self::Log { data }
    }

    pub fn path(data: PathCondition) -> Self {
        Self::Path { data }
    }

    pub fn port(data: PortCondition) -> Self {
        Self::Port { data }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
