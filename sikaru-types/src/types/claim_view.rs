pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ClaimView {
    #[serde(default)]
    pub attachment_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub lease_until: f64,
    #[serde(default)]
    pub owner_epoch: i64,
    #[serde(default)]
    pub owner_id: String,
    /// Initial launch bound anchored at the first claim request start; idempotent claim retries never restart this clock. Workers cannot renew a ready executor lease.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startup_ttl_seconds: Option<i64>,
    #[serde(default)]
    pub status: String,
}

impl ClaimView {
    pub fn builder() -> ClaimViewBuilder {
        <ClaimViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClaimViewBuilder {
    attachment_id: Option<String>,
    id: Option<String>,
    lease_until: Option<f64>,
    owner_epoch: Option<i64>,
    owner_id: Option<String>,
    startup_ttl_seconds: Option<i64>,
    status: Option<String>,
}

impl ClaimViewBuilder {
    pub fn attachment_id(mut self, value: impl Into<String>) -> Self {
        self.attachment_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn lease_until(mut self, value: f64) -> Self {
        self.lease_until = Some(value);
        self
    }

    pub fn owner_epoch(mut self, value: i64) -> Self {
        self.owner_epoch = Some(value);
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    pub fn startup_ttl_seconds(mut self, value: i64) -> Self {
        self.startup_ttl_seconds = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClaimView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attachment_id`](ClaimViewBuilder::attachment_id)
    /// - [`id`](ClaimViewBuilder::id)
    /// - [`lease_until`](ClaimViewBuilder::lease_until)
    /// - [`owner_epoch`](ClaimViewBuilder::owner_epoch)
    /// - [`owner_id`](ClaimViewBuilder::owner_id)
    /// - [`status`](ClaimViewBuilder::status)
    pub fn build(self) -> Result<ClaimView, BuildError> {
        Ok(ClaimView {
            attachment_id: self.attachment_id.ok_or_else(|| BuildError::missing_field("attachment_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            lease_until: self.lease_until.ok_or_else(|| BuildError::missing_field("lease_until"))?,
            owner_epoch: self.owner_epoch.ok_or_else(|| BuildError::missing_field("owner_epoch"))?,
            owner_id: self.owner_id.ok_or_else(|| BuildError::missing_field("owner_id"))?,
            startup_ttl_seconds: self.startup_ttl_seconds,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
