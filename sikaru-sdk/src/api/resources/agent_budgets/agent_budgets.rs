use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AgentBudgetsClient {
    pub http_client: HttpClient,
}

impl AgentBudgetsClient {
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
    ///         .agent_budgets
    ///         .get(&"project_id".to_string(), &"harness_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        harness_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AgentBudget, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/harnesses/{}/budget", project_id, harness_id),
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
    ///         .agent_budgets
    ///         .configure_auto_reload(
    ///             &"project_id".to_string(),
    ///             &"harness_id".to_string(),
    ///             &ReloadInput {
    ///                 amount_usd: "amount_usd".to_string(),
    ///                 enabled: true,
    ///                 threshold_usd: "threshold_usd".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn configure_auto_reload(
        &self,
        project_id: &str,
        harness_id: &str,
        request: &ReloadInput,
        options: Option<RequestOptions>,
    ) -> Result<AgentBudget, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "v1/projects/{}/harnesses/{}/budget/auto-reload",
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
    ///         .agent_budgets
    ///         .add(
    ///             &"project_id".to_string(),
    ///             &"harness_id".to_string(),
    ///             &FundingInput {
    ///                 amount_usd: "amount_usd".to_string(),
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add(
        &self,
        project_id: &str,
        harness_id: &str,
        request: &FundingInput,
        options: Option<RequestOptions>,
    ) -> Result<FundingReceipt, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/harnesses/{}/budget/funding",
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
    ///         .agent_budgets
    ///         .setup_payment_method(
    ///             &"project_id".to_string(),
    ///             &"harness_id".to_string(),
    ///             &SetupInput {
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn setup_payment_method(
        &self,
        project_id: &str,
        harness_id: &str,
        request: &SetupInput,
        options: Option<RequestOptions>,
    ) -> Result<PaymentSetupLink, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/harnesses/{}/budget/payment-method",
                    project_id, harness_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
