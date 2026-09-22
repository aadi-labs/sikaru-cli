pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentSetupLink {
    #[serde(default)]
    pub url: String,
}

impl PaymentSetupLink {
    pub fn builder() -> PaymentSetupLinkBuilder {
        <PaymentSetupLinkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentSetupLinkBuilder {
    url: Option<String>,
}

impl PaymentSetupLinkBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentSetupLink`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](PaymentSetupLinkBuilder::url)
    pub fn build(self) -> Result<PaymentSetupLink, BuildError> {
        Ok(PaymentSetupLink {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
