use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DatasetsClient {
    pub http_client: HttpClient,
}

impl DatasetsClient {
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
    ///         .datasets
    ///         .list_datasets(
    ///             &"project_id".to_string(),
    ///             &ListDatasetsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_datasets(
        &self,
        project_id: &str,
        request: &ListDatasetsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetList, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/datasets", project_id),
                None,
                QueryBuilder::new()
                    .serialize("purpose", request.purpose.clone())
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
    ///         .datasets
    ///         .create_dataset(
    ///             &"project_id".to_string(),
    ///             &CreateDataset {
    ///                 name: "name".to_string(),
    ///                 purpose: CreateDatasetPurpose::Eval,
    ///                 description: None,
    ///                 idempotency_key: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_dataset(
        &self,
        project_id: &str,
        request: &CreateDataset,
        options: Option<RequestOptions>,
    ) -> Result<DatasetResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/datasets", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates the named dataset and adds the runs to it; a retry of the key reuses both.
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
    ///         .datasets
    ///         .capture_into_new_dataset(
    ///             &"project_id".to_string(),
    ///             &CaptureIntoNewRequest {
    ///                 dataset: NewDataset {
    ///                     name: "name".to_string(),
    ///                     purpose: NewDatasetPurpose::Eval,
    ///                 },
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 items: vec![CaptureItem {
    ///                     run_id: "run_id".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn capture_into_new_dataset(
        &self,
        project_id: &str,
        request: &CaptureIntoNewRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchResult, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/datasets/capture", project_id),
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
    ///         .datasets
    ///         .get_dataset(&"project_id".to_string(), &"dataset_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_dataset(
        &self,
        project_id: &str,
        dataset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DatasetResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/datasets/{}", project_id, dataset_id),
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
    ///         .datasets
    ///         .delete_dataset(&"project_id".to_string(), &"dataset_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_dataset(
        &self,
        project_id: &str,
        dataset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("v1/projects/{}/datasets/{}", project_id, dataset_id),
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
    ///         .datasets
    ///         .update_dataset(
    ///             &"project_id".to_string(),
    ///             &"dataset_id".to_string(),
    ///             &UpdateDataset {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_dataset(
        &self,
        project_id: &str,
        dataset_id: &str,
        request: &UpdateDataset,
        options: Option<RequestOptions>,
    ) -> Result<DatasetResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/projects/{}/datasets/{}", project_id, dataset_id),
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
    ///         .datasets
    ///         .capture_into_dataset(
    ///             &"project_id".to_string(),
    ///             &"dataset_id".to_string(),
    ///             &CaptureRequest {
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 items: vec![CaptureItem {
    ///                     run_id: "run_id".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn capture_into_dataset(
        &self,
        project_id: &str,
        dataset_id: &str,
        request: &CaptureRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchResult, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/datasets/{}/capture", project_id, dataset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// One Check per eligible example, each started as a production run of the agent's live version.
    ///
    /// Credits are admitted before anything is created. A retry of the same key
    /// returns the same checks and starts only what did not start before.
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
    ///         .datasets
    ///         .start_dataset_checks(
    ///             &"project_id".to_string(),
    ///             &"dataset_id".to_string(),
    ///             &StartDatasetChecks {
    ///                 agent_slug: "agent_slug".to_string(),
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 dataset_version: None,
    ///                 version: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn start_dataset_checks(
        &self,
        project_id: &str,
        dataset_id: &str,
        request: &StartDatasetChecks,
        options: Option<RequestOptions>,
    ) -> Result<DatasetChecksStarted, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/datasets/{}/checks", project_id, dataset_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// How many examples can run as Checks, why the rest cannot, and the estimated model cost on ``agent``.
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
    ///         .datasets
    ///         .preview_dataset_checks(
    ///             &"project_id".to_string(),
    ///             &"dataset_id".to_string(),
    ///             &PreviewDatasetChecksQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn preview_dataset_checks(
        &self,
        project_id: &str,
        dataset_id: &str,
        request: &PreviewDatasetChecksQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetChecksPreview, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/datasets/{}/checks/preview",
                    project_id, dataset_id
                ),
                None,
                QueryBuilder::new()
                    .serialize("agent", request.agent.clone())
                    .serialize("version", request.version.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Pass rate per agent version for each dataset version run as Checks; failures name their example and run.
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
    ///         .datasets
    ///         .list_dataset_check_results(
    ///             &"project_id".to_string(),
    ///             &"dataset_id".to_string(),
    ///             &ListDatasetCheckResultsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_dataset_check_results(
        &self,
        project_id: &str,
        dataset_id: &str,
        request: &ListDatasetCheckResultsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetCheckResults, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/datasets/{}/checks/results",
                    project_id, dataset_id
                ),
                None,
                QueryBuilder::new()
                    .serialize("version", request.version.clone())
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
    ///         .datasets
    ///         .list_examples(
    ///             &"project_id".to_string(),
    ///             &"dataset_id".to_string(),
    ///             &ListExamplesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_examples(
        &self,
        project_id: &str,
        dataset_id: &str,
        request: &ListExamplesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DatasetExamplePage, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/datasets/{}/examples",
                    project_id, dataset_id
                ),
                None,
                QueryBuilder::new()
                    .serialize("version", request.version.clone())
                    .int("limit", request.limit.clone())
                    .int("offset", request.offset.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Permanently removes every copy of the example, including from older versions.
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
    ///         .datasets
    ///         .delete_example(
    ///             &"project_id".to_string(),
    ///             &"dataset_id".to_string(),
    ///             &"example_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_example(
        &self,
        project_id: &str,
        dataset_id: &str,
        example_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DatasetVersionResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/projects/{}/datasets/{}/examples/{}",
                    project_id, dataset_id, example_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Editing the expected answer or tags creates a version; edited fields are human-written.
    ///
    /// Sikaru's own tags survive a tag edit unless it comes from the signed-in dashboard, which shows them.
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
    ///         .datasets
    ///         .update_example(
    ///             &"project_id".to_string(),
    ///             &"dataset_id".to_string(),
    ///             &"example_id".to_string(),
    ///             &UpdateExample {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_example(
        &self,
        project_id: &str,
        dataset_id: &str,
        example_id: &str,
        request: &UpdateExample,
        options: Option<RequestOptions>,
    ) -> Result<DatasetExampleResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/projects/{}/datasets/{}/examples/{}",
                    project_id, dataset_id, example_id
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
    ///         .datasets
    ///         .export_dataset(
    ///             &"project_id".to_string(),
    ///             &"dataset_id".to_string(),
    ///             &ExportDatasetQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_dataset(
        &self,
        project_id: &str,
        dataset_id: &str,
        request: &ExportDatasetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("v1/projects/{}/datasets/{}/export", project_id, dataset_id),
                None,
                QueryBuilder::new()
                    .serialize("version", request.version.clone())
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
    ///         .datasets
    ///         .upload_examples(
    ///             &"project_id".to_string(),
    ///             &"dataset_id".to_string(),
    ///             &UploadRequest {
    ///                 content: "content".to_string(),
    ///                 format: UploadRequestFormat::Csv,
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upload_examples(
        &self,
        project_id: &str,
        dataset_id: &str,
        request: &UploadRequest,
        options: Option<RequestOptions>,
    ) -> Result<BatchResult, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/datasets/{}/uploads", project_id, dataset_id),
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
    ///         .datasets
    ///         .list_versions(&"project_id".to_string(), &"dataset_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_versions(
        &self,
        project_id: &str,
        dataset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DatasetVersionList, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/datasets/{}/versions",
                    project_id, dataset_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
