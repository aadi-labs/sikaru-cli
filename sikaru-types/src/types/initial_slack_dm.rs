pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InitialSlackDm {
    #[serde(default)]
    pub destination_id: String,
    pub destination_kind: InitialSlackDmDestinationKind,
    #[serde(default)]
    pub installation_id: String,
    #[serde(default)]
    pub recipient_id: String,
    pub transport: InitialSlackDmTransport,
    #[serde(default)]
    pub verification_id: String,
}

impl InitialSlackDm {
    pub fn builder() -> InitialSlackDmBuilder {
        <InitialSlackDmBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitialSlackDmBuilder {
    destination_id: Option<String>,
    destination_kind: Option<InitialSlackDmDestinationKind>,
    installation_id: Option<String>,
    recipient_id: Option<String>,
    transport: Option<InitialSlackDmTransport>,
    verification_id: Option<String>,
}

impl InitialSlackDmBuilder {
    pub fn destination_id(mut self, value: impl Into<String>) -> Self {
        self.destination_id = Some(value.into());
        self
    }

    pub fn destination_kind(mut self, value: InitialSlackDmDestinationKind) -> Self {
        self.destination_kind = Some(value);
        self
    }

    pub fn installation_id(mut self, value: impl Into<String>) -> Self {
        self.installation_id = Some(value.into());
        self
    }

    pub fn recipient_id(mut self, value: impl Into<String>) -> Self {
        self.recipient_id = Some(value.into());
        self
    }

    pub fn transport(mut self, value: InitialSlackDmTransport) -> Self {
        self.transport = Some(value);
        self
    }

    pub fn verification_id(mut self, value: impl Into<String>) -> Self {
        self.verification_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InitialSlackDm`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination_id`](InitialSlackDmBuilder::destination_id)
    /// - [`destination_kind`](InitialSlackDmBuilder::destination_kind)
    /// - [`installation_id`](InitialSlackDmBuilder::installation_id)
    /// - [`recipient_id`](InitialSlackDmBuilder::recipient_id)
    /// - [`transport`](InitialSlackDmBuilder::transport)
    /// - [`verification_id`](InitialSlackDmBuilder::verification_id)
    pub fn build(self) -> Result<InitialSlackDm, BuildError> {
        Ok(InitialSlackDm {
            destination_id: self.destination_id.ok_or_else(|| BuildError::missing_field("destination_id"))?,
            destination_kind: self.destination_kind.ok_or_else(|| BuildError::missing_field("destination_kind"))?,
            installation_id: self.installation_id.ok_or_else(|| BuildError::missing_field("installation_id"))?,
            recipient_id: self.recipient_id.ok_or_else(|| BuildError::missing_field("recipient_id"))?,
            transport: self.transport.ok_or_else(|| BuildError::missing_field("transport"))?,
            verification_id: self.verification_id.ok_or_else(|| BuildError::missing_field("verification_id"))?,
        })
    }
}
