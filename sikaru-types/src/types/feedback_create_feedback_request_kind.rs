pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateFeedbackRequestKind {
    ThumbsUp,
    ThumbsDown,
    Annotation,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateFeedbackRequestKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ThumbsUp => serializer.serialize_str("thumbs_up"),
            Self::ThumbsDown => serializer.serialize_str("thumbs_down"),
            Self::Annotation => serializer.serialize_str("annotation"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateFeedbackRequestKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "thumbs_up" => Ok(Self::ThumbsUp),
            "thumbs_down" => Ok(Self::ThumbsDown),
            "annotation" => Ok(Self::Annotation),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateFeedbackRequestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThumbsUp => write!(f, "thumbs_up"),
            Self::ThumbsDown => write!(f, "thumbs_down"),
            Self::Annotation => write!(f, "annotation"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
