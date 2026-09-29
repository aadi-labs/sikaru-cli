pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectionApp {
    #[serde(default)]
    pub auth_schemes: Vec<String>,
    #[serde(default)]
    pub categories: Vec<ToolkitCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
}

impl ConnectionApp {
    pub fn builder() -> ConnectionAppBuilder {
        <ConnectionAppBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionAppBuilder {
    auth_schemes: Option<Vec<String>>,
    categories: Option<Vec<ToolkitCategory>>,
    logo: Option<String>,
    name: Option<String>,
    slug: Option<String>,
}

impl ConnectionAppBuilder {
    pub fn auth_schemes(mut self, value: Vec<String>) -> Self {
        self.auth_schemes = Some(value);
        self
    }

    pub fn categories(mut self, value: Vec<ToolkitCategory>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn logo(mut self, value: impl Into<String>) -> Self {
        self.logo = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectionApp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auth_schemes`](ConnectionAppBuilder::auth_schemes)
    /// - [`categories`](ConnectionAppBuilder::categories)
    /// - [`name`](ConnectionAppBuilder::name)
    /// - [`slug`](ConnectionAppBuilder::slug)
    pub fn build(self) -> Result<ConnectionApp, BuildError> {
        Ok(ConnectionApp {
            auth_schemes: self.auth_schemes.ok_or_else(|| BuildError::missing_field("auth_schemes"))?,
            categories: self.categories.ok_or_else(|| BuildError::missing_field("categories"))?,
            logo: self.logo,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
