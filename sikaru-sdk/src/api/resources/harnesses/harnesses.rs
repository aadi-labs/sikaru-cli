use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct HarnessesClient {
    pub http_client: HttpClient,
}

impl HarnessesClient {
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
    ///         .harnesses
    ///         .get_invoice_budget(&"project_id".to_string(), &"harness_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_invoice_budget(
        &self,
        project_id: &str,
        harness_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<InvoiceBudget, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/harnesses/{}/budget/invoice",
                    project_id, harness_id
                ),
                None,
                None,
                options,
            )
            .await
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
    ///         .harnesses
    ///         .get_subscription(&"project_id".to_string(), &"harness_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_subscription(
        &self,
        project_id: &str,
        harness_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionStatus, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/harnesses/{}/budget/subscription",
                    project_id, harness_id
                ),
                None,
                None,
                options,
            )
            .await
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
    ///         .harnesses
    ///         .subscribe(
    ///             &"project_id".to_string(),
    ///             &"harness_id".to_string(),
    ///             &SubscriptionInput {
    ///                 accepted_recurring_terms: true,
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 plan: SubscriptionInputPlan::Build,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn subscribe(
        &self,
        project_id: &str,
        harness_id: &str,
        request: &SubscriptionInput,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionSetup, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/harnesses/{}/budget/subscription",
                    project_id, harness_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
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
    ///         .harnesses
    ///         .cancel_subscription(&"project_id".to_string(), &"harness_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel_subscription(
        &self,
        project_id: &str,
        harness_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SubscriptionCancellation, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/harnesses/{}/budget/subscription/cancel",
                    project_id, harness_id
                ),
                None,
                None,
                options,
            )
            .await
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
    ///         .harnesses
    ///         .improvement_options(&"project_id".to_string(), &"harness_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn improvement_options(
        &self,
        project_id: &str,
        harness_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/harnesses/{}/improvement-options",
                    project_id, harness_id
                ),
                None,
                None,
                options,
            )
            .await
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
    ///         .harnesses
    ///         .list_improvements(
    ///             &"project_id".to_string(),
    ///             &"harness_id".to_string(),
    ///             &ListImprovementsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_improvements(
        &self,
        project_id: &str,
        harness_id: &str,
        request: &ListImprovementsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/harnesses/{}/improvements",
                    project_id, harness_id
                ),
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
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
    ///         .harnesses
    ///         .start_improvement(
    ///             &"project_id".to_string(),
    ///             &"harness_id".to_string(),
    ///             &ImprovementInput {
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 objective: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn start_improvement(
        &self,
        project_id: &str,
        harness_id: &str,
        request: &ImprovementInput,
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
                &format!(
                    "v1/projects/{}/harnesses/{}/improvements",
                    project_id, harness_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
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
    ///         .harnesses
    ///         .get_improvement(
    ///             &"project_id".to_string(),
    ///             &"harness_id".to_string(),
    ///             &"job_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_improvement(
        &self,
        project_id: &str,
        harness_id: &str,
        job_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/harnesses/{}/improvements/{}",
                    project_id, harness_id, job_id
                ),
                None,
                None,
                options,
            )
            .await
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
    ///         .harnesses
    ///         .resume_improvement(
    ///             &"project_id".to_string(),
    ///             &"harness_id".to_string(),
    ///             &"job_id".to_string(),
    ///             &Some(ResumeImprovementInput {
    ///                 ..Default::default()
    ///             }),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn resume_improvement(
        &self,
        project_id: &str,
        harness_id: &str,
        job_id: &str,
        request: &Option<ResumeImprovementInput>,
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
                &format!(
                    "v1/projects/{}/harnesses/{}/improvements/{}/resume",
                    project_id, harness_id, job_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Reserved, unavailable model-training step; no learning job is submitted.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
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
    ///         .harnesses
    ///         .train_model_stub(&"project_id".to_string(), &"harness_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn train_model_stub(
        &self,
        project_id: &str,
        harness_id: &str,
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
                &format!(
                    "v1/projects/{}/harnesses/{}/training",
                    project_id, harness_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
