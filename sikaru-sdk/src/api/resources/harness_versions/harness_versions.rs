use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct HarnessVersionsClient {
    pub http_client: HttpClient,
}

impl HarnessVersionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client
    ///         .harness_versions
    ///         .create_harness_version(
    ///             &"project_id".to_string(),
    ///             &CreateHarnessVersionRequest {
    ///                 base_harness_profile_id: "baseHarnessProfileId".to_string(),
    ///                 compatibility_profile_id: "compatibilityProfileId".to_string(),
    ///                 display_name: "displayName".to_string(),
    ///                 harness_id: "harnessId".to_string(),
    ///                 source_artifact_id: "sourceArtifactId".to_string(),
    ///                 base_harness_version_id: None,
    ///                 status: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_harness_version(
        &self,
        project_id: &str,
        request: &CreateHarnessVersionRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/harness-versions", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
