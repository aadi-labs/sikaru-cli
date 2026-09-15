use fern_cli_sdk::auth::no_auth_provider;
use fern_cli_sdk::http::HttpConfig;
use fern_cli_sdk::openapi::discovery::RetriesConfig;
use fern_cli_sdk::sdk_executor::{CliExecutor, SdkRequestExecutor};

#[tokio::test]
async fn mutations_are_not_replayed_even_with_an_idempotency_header() {
    for method in ["POST", "PUT", "PATCH", "DELETE"] {
        let server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method(method))
            .respond_with(wiremock::ResponseTemplate::new(503))
            .expect(1)
            .mount(&server)
            .await;
        let executor = CliExecutor::new(
            HttpConfig::new("sikaru").unwrap(),
            no_auth_provider(),
            vec![],
            None,
        )
        .with_retries(RetriesConfig {
            enabled: true,
            max_attempts: 3,
            base_delay_ms: 1,
            factor: 1.0,
            jitter: 0.0,
        });
        let request = reqwest::Client::new()
            .request(method.parse().unwrap(), server.uri())
            .header("Idempotency-Key", "server-does-not-guarantee-deduplication")
            .body("{}")
            .build()
            .unwrap();
        let _ = executor.execute(request).await;
        assert_eq!(server.received_requests().await.unwrap().len(), 1, "{method}");
    }
}
