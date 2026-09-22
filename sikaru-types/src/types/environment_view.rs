pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EnvironmentView {
    #[serde(default)]
    pub environment_slug: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub product_id: String,
    #[serde(default)]
    pub project_id: String,
    pub status: EnvironmentViewStatus,
}

impl EnvironmentView {
    pub fn builder() -> EnvironmentViewBuilder {
        <EnvironmentViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnvironmentViewBuilder {
    environment_slug: Option<String>,
    id: Option<String>,
    product_id: Option<String>,
    project_id: Option<String>,
    status: Option<EnvironmentViewStatus>,
}

impl EnvironmentViewBuilder {
    pub fn environment_slug(mut self, value: impl Into<String>) -> Self {
        self.environment_slug = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: EnvironmentViewStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnvironmentView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment_slug`](EnvironmentViewBuilder::environment_slug)
    /// - [`id`](EnvironmentViewBuilder::id)
    /// - [`product_id`](EnvironmentViewBuilder::product_id)
    /// - [`project_id`](EnvironmentViewBuilder::project_id)
    /// - [`status`](EnvironmentViewBuilder::status)
    pub fn build(self) -> Result<EnvironmentView, BuildError> {
        Ok(EnvironmentView {
            environment_slug: self.environment_slug.ok_or_else(|| BuildError::missing_field("environment_slug"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            product_id: self.product_id.ok_or_else(|| BuildError::missing_field("product_id"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
