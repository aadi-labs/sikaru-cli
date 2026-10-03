pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Availability {
    #[serde(default)]
    pub http: bool,
    #[serde(default)]
    pub imessage: bool,
    #[serde(default)]
    pub slack: bool,
}

impl Availability {
    pub fn builder() -> AvailabilityBuilder {
        <AvailabilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AvailabilityBuilder {
    http: Option<bool>,
    imessage: Option<bool>,
    slack: Option<bool>,
}

impl AvailabilityBuilder {
    pub fn http(mut self, value: bool) -> Self {
        self.http = Some(value);
        self
    }

    pub fn imessage(mut self, value: bool) -> Self {
        self.imessage = Some(value);
        self
    }

    pub fn slack(mut self, value: bool) -> Self {
        self.slack = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Availability`].
    /// This method will fail if any of the following fields are not set:
    /// - [`http`](AvailabilityBuilder::http)
    /// - [`imessage`](AvailabilityBuilder::imessage)
    /// - [`slack`](AvailabilityBuilder::slack)
    pub fn build(self) -> Result<Availability, BuildError> {
        Ok(Availability {
            http: self.http.ok_or_else(|| BuildError::missing_field("http"))?,
            imessage: self.imessage.ok_or_else(|| BuildError::missing_field("imessage"))?,
            slack: self.slack.ok_or_else(|| BuildError::missing_field("slack"))?,
        })
    }
}
