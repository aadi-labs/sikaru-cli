pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateReviewQueueItemRequestAction {
    ReviewQueue,
    NotifyReviewer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateReviewQueueItemRequestAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ReviewQueue => serializer.serialize_str("review_queue"),
            Self::NotifyReviewer => serializer.serialize_str("notify_reviewer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateReviewQueueItemRequestAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "review_queue" => Ok(Self::ReviewQueue),
            "notify_reviewer" => Ok(Self::NotifyReviewer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateReviewQueueItemRequestAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReviewQueue => write!(f, "review_queue"),
            Self::NotifyReviewer => write!(f, "notify_reviewer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
