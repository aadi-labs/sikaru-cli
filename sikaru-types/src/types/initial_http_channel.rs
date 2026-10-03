pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InitialHttpChannel {
    pub transport: InitialHttpChannelTransport,
}

impl InitialHttpChannel {
    pub fn builder() -> InitialHttpChannelBuilder {
        <InitialHttpChannelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InitialHttpChannelBuilder {
    transport: Option<InitialHttpChannelTransport>,
}

impl InitialHttpChannelBuilder {
    pub fn transport(mut self, value: InitialHttpChannelTransport) -> Self {
        self.transport = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InitialHttpChannel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transport`](InitialHttpChannelBuilder::transport)
    pub fn build(self) -> Result<InitialHttpChannel, BuildError> {
        Ok(InitialHttpChannel {
            transport: self.transport.ok_or_else(|| BuildError::missing_field("transport"))?,
        })
    }
}
