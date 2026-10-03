pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HttpReceipt {
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<HttpOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    pub status: HttpReceiptStatus,
    #[serde(default)]
    pub status_url: String,
}

impl HttpReceipt {
    pub fn builder() -> HttpReceiptBuilder {
        <HttpReceiptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HttpReceiptBuilder {
    id: Option<String>,
    operator_url: Option<String>,
    output: Option<HttpOutput>,
    run_id: Option<String>,
    session_id: Option<String>,
    status: Option<HttpReceiptStatus>,
    status_url: Option<String>,
}

impl HttpReceiptBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn operator_url(mut self, value: impl Into<String>) -> Self {
        self.operator_url = Some(value.into());
        self
    }

    pub fn output(mut self, value: HttpOutput) -> Self {
        self.output = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: HttpReceiptStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_url(mut self, value: impl Into<String>) -> Self {
        self.status_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HttpReceipt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](HttpReceiptBuilder::id)
    /// - [`status`](HttpReceiptBuilder::status)
    /// - [`status_url`](HttpReceiptBuilder::status_url)
    pub fn build(self) -> Result<HttpReceipt, BuildError> {
        Ok(HttpReceipt {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            operator_url: self.operator_url,
            output: self.output,
            run_id: self.run_id,
            session_id: self.session_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            status_url: self.status_url.ok_or_else(|| BuildError::missing_field("status_url"))?,
        })
    }
}
