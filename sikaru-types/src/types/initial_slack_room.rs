pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InitialSlackRoom {
    #[serde(default)]
    pub destination_id: String,
    pub destination_kind: InitialSlackRoomDestinationKind,
    #[serde(default)]
    pub installation_id: String,
    pub transport: InitialSlackRoomTransport,
}

impl InitialSlackRoom {
    pub fn builder() -> InitialSlackRoomBuilder {
        <InitialSlackRoomBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitialSlackRoomBuilder {
    destination_id: Option<String>,
    destination_kind: Option<InitialSlackRoomDestinationKind>,
    installation_id: Option<String>,
    transport: Option<InitialSlackRoomTransport>,
}

impl InitialSlackRoomBuilder {
    pub fn destination_id(mut self, value: impl Into<String>) -> Self {
        self.destination_id = Some(value.into());
        self
    }

    pub fn destination_kind(mut self, value: InitialSlackRoomDestinationKind) -> Self {
        self.destination_kind = Some(value);
        self
    }

    pub fn installation_id(mut self, value: impl Into<String>) -> Self {
        self.installation_id = Some(value.into());
        self
    }

    pub fn transport(mut self, value: InitialSlackRoomTransport) -> Self {
        self.transport = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InitialSlackRoom`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination_id`](InitialSlackRoomBuilder::destination_id)
    /// - [`destination_kind`](InitialSlackRoomBuilder::destination_kind)
    /// - [`installation_id`](InitialSlackRoomBuilder::installation_id)
    /// - [`transport`](InitialSlackRoomBuilder::transport)
    pub fn build(self) -> Result<InitialSlackRoom, BuildError> {
        Ok(InitialSlackRoom {
            destination_id: self.destination_id.ok_or_else(|| BuildError::missing_field("destination_id"))?,
            destination_kind: self.destination_kind.ok_or_else(|| BuildError::missing_field("destination_kind"))?,
            installation_id: self.installation_id.ok_or_else(|| BuildError::missing_field("installation_id"))?,
            transport: self.transport.ok_or_else(|| BuildError::missing_field("transport"))?,
        })
    }
}
