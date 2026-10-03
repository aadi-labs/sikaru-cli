use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PersonalChannelsClient {
    pub http_client: HttpClient,
}

impl PersonalChannelsClient {
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
    ///         .personal_channels
    ///         .get_message(&"binding_id".to_string(), &"receipt_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_message(
        &self,
        binding_id: &str,
        receipt_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PersonalChannelReceipt, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/personal-channel-bindings/{}/messages/{}",
                    binding_id, receipt_id
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
    ///         .personal_channels
    ///         .authorize_connection(
    ///             &"binding_id".to_string(),
    ///             &"receipt_id".to_string(),
    ///             &"connection_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn authorize_connection(
        &self,
        binding_id: &str,
        receipt_id: &str,
        connection_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConnectionAuthorization, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/personal-channel-bindings/{}/messages/{}/connections/{}/authorize",
                    binding_id, receipt_id, connection_id
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
    ///         .personal_channels
    ///         .complete_connection(
    ///             &"binding_id".to_string(),
    ///             &"receipt_id".to_string(),
    ///             &"connection_id".to_string(),
    ///             &CompleteAuthorization {
    ///                 state: "state".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn complete_connection(
        &self,
        binding_id: &str,
        receipt_id: &str,
        connection_id: &str,
        request: &CompleteAuthorization,
        options: Option<RequestOptions>,
    ) -> Result<PersonalChannelConnection, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/personal-channel-bindings/{}/messages/{}/connections/{}/complete",
                    binding_id, receipt_id, connection_id
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
    ///         .personal_channels
    ///         .replace_connection_credentials(
    ///             &"binding_id".to_string(),
    ///             &"receipt_id".to_string(),
    ///             &"connection_id".to_string(),
    ///             &ReplaceCredentials {
    ///                 credentials: ConnectionCredentials {
    ///                     ..Default::default()
    ///                 },
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn replace_connection_credentials(
        &self,
        binding_id: &str,
        receipt_id: &str,
        connection_id: &str,
        request: &ReplaceCredentials,
        options: Option<RequestOptions>,
    ) -> Result<PersonalChannelConnection, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PUT,
                &format!(
                    "v1/personal-channel-bindings/{}/messages/{}/connections/{}/credentials",
                    binding_id, receipt_id, connection_id
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
    ///         .personal_channels
    ///         .list_files(&"binding_id".to_string(), &"receipt_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_files(
        &self,
        binding_id: &str,
        receipt_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PersonalChannelFiles, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/personal-channel-bindings/{}/messages/{}/files",
                    binding_id, receipt_id
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
    ///         .personal_channels
    ///         .download_file(
    ///             &"binding_id".to_string(),
    ///             &"receipt_id".to_string(),
    ///             &"file_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn download_file(
        &self,
        binding_id: &str,
        receipt_id: &str,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!(
                    "v1/personal-channel-bindings/{}/messages/{}/files/{}/content",
                    binding_id, receipt_id, file_id
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
    ///         .personal_channels
    ///         .decide_approval(
    ///             &"binding_id".to_string(),
    ///             &"receipt_id".to_string(),
    ///             &"tool_call_id".to_string(),
    ///             &ApprovalInput {
    ///                 decision: ApprovalInputDecision::Approved,
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn decide_approval(
        &self,
        binding_id: &str,
        receipt_id: &str,
        tool_call_id: &str,
        request: &ApprovalInput,
        options: Option<RequestOptions>,
    ) -> Result<PersonalChannelApproval, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/personal-channel-bindings/{}/messages/{}/tool-calls/{}/approval",
                    binding_id, receipt_id, tool_call_id
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
    ///         .personal_channels
    ///         .get_slack_link(
    ///             &"binding_id".to_string(),
    ///             &"verification_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_slack_link(
        &self,
        binding_id: &str,
        verification_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PersonalSlackStatus, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/personal-channel-bindings/{}/slack-link-verifications/{}",
                    binding_id, verification_id
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
    ///         .personal_channels
    ///         .start_slack_link(
    ///             &"binding_id".to_string(),
    ///             &SlackLink {
    ///                 installation_id: "installation_id".to_string(),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn start_slack_link(
        &self,
        binding_id: &str,
        request: &SlackLink,
        options: Option<RequestOptions>,
    ) -> Result<PersonalSlackChallenge, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/personal-channel-bindings/{}/slack-links", binding_id),
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
    ///         .personal_channels
    ///         .unlink_slack_identity(
    ///             &"binding_id".to_string(),
    ///             &"installation_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn unlink_slack_identity(
        &self,
        binding_id: &str,
        installation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/personal-channel-bindings/{}/slack-links/{}",
                    binding_id, installation_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
