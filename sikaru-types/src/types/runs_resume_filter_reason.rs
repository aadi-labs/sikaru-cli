pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResumeFilterReason {
    CreditsExhausted,
    SpendLimitReached,
    BillingUnavailable,
    PlanInactive,
    TrialEnded,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResumeFilterReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreditsExhausted => serializer.serialize_str("credits_exhausted"),
            Self::SpendLimitReached => serializer.serialize_str("spend_limit_reached"),
            Self::BillingUnavailable => serializer.serialize_str("billing_unavailable"),
            Self::PlanInactive => serializer.serialize_str("plan_inactive"),
            Self::TrialEnded => serializer.serialize_str("trial_ended"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResumeFilterReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "credits_exhausted" => Ok(Self::CreditsExhausted),
            "spend_limit_reached" => Ok(Self::SpendLimitReached),
            "billing_unavailable" => Ok(Self::BillingUnavailable),
            "plan_inactive" => Ok(Self::PlanInactive),
            "trial_ended" => Ok(Self::TrialEnded),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResumeFilterReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreditsExhausted => write!(f, "credits_exhausted"),
            Self::SpendLimitReached => write!(f, "spend_limit_reached"),
            Self::BillingUnavailable => write!(f, "billing_unavailable"),
            Self::PlanInactive => write!(f, "plan_inactive"),
            Self::TrialEnded => write!(f, "trial_ended"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
