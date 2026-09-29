pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WorkspaceRemoteView {
    #[serde(default)]
    pub branch: String,
    #[serde(default)]
    pub expires_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_sha: Option<String>,
    #[serde(default)]
    pub ignore_defaults: Vec<String>,
    #[serde(default)]
    pub max_push_bytes: i64,
    #[serde(default)]
    pub remote_url: String,
    /// Short-lived push credential for this session branch. Treat it as a secret.
    #[serde(default)]
    pub token: String,
    pub username: WorkspaceRemoteViewUsername,
}

impl WorkspaceRemoteView {
    pub fn builder() -> WorkspaceRemoteViewBuilder {
        <WorkspaceRemoteViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceRemoteViewBuilder {
    branch: Option<String>,
    expires_at: Option<String>,
    head_sha: Option<String>,
    ignore_defaults: Option<Vec<String>>,
    max_push_bytes: Option<i64>,
    remote_url: Option<String>,
    token: Option<String>,
    username: Option<WorkspaceRemoteViewUsername>,
}

impl WorkspaceRemoteViewBuilder {
    pub fn branch(mut self, value: impl Into<String>) -> Self {
        self.branch = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn head_sha(mut self, value: impl Into<String>) -> Self {
        self.head_sha = Some(value.into());
        self
    }

    pub fn ignore_defaults(mut self, value: Vec<String>) -> Self {
        self.ignore_defaults = Some(value);
        self
    }

    pub fn max_push_bytes(mut self, value: i64) -> Self {
        self.max_push_bytes = Some(value);
        self
    }

    pub fn remote_url(mut self, value: impl Into<String>) -> Self {
        self.remote_url = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn username(mut self, value: WorkspaceRemoteViewUsername) -> Self {
        self.username = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceRemoteView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`branch`](WorkspaceRemoteViewBuilder::branch)
    /// - [`expires_at`](WorkspaceRemoteViewBuilder::expires_at)
    /// - [`ignore_defaults`](WorkspaceRemoteViewBuilder::ignore_defaults)
    /// - [`max_push_bytes`](WorkspaceRemoteViewBuilder::max_push_bytes)
    /// - [`remote_url`](WorkspaceRemoteViewBuilder::remote_url)
    /// - [`token`](WorkspaceRemoteViewBuilder::token)
    /// - [`username`](WorkspaceRemoteViewBuilder::username)
    pub fn build(self) -> Result<WorkspaceRemoteView, BuildError> {
        Ok(WorkspaceRemoteView {
            branch: self.branch.ok_or_else(|| BuildError::missing_field("branch"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
            head_sha: self.head_sha,
            ignore_defaults: self.ignore_defaults.ok_or_else(|| BuildError::missing_field("ignore_defaults"))?,
            max_push_bytes: self.max_push_bytes.ok_or_else(|| BuildError::missing_field("max_push_bytes"))?,
            remote_url: self.remote_url.ok_or_else(|| BuildError::missing_field("remote_url"))?,
            token: self.token.ok_or_else(|| BuildError::missing_field("token"))?,
            username: self.username.ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
