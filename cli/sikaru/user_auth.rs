//! First-party user login, preserving explicit project-key precedence.
use std::sync::Arc;
use base64::Engine;
use fern_cli_sdk::{app::CliApp, error::CliError};
use fern_cli_sdk::auth::{login::{LoginContext, LoginFlow}, keyring_store::active_store,
    oauth_login::DeviceCodeLoginFlow, provider::{AuthProvider, DynAuthProvider, EndpointAuthMetadata}};

const SCHEME: &str = "BearerAuth";
const TOKEN: &str = "https://api.workos.com/user_management/authenticate";

pub fn install(app: CliApp) -> CliApp { app.login_flow(UserLogin) }

#[derive(Debug)]
struct UserLogin;

fn configured_flow() -> Result<DeviceCodeLoginFlow, CliError> {
    let client_id = match std::env::var("SIKARU_AUTH_CLIENT_ID") {
        Ok(value) if !value.is_empty() => value,
        _ => tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(client_id()))?,
    };
    Ok(DeviceCodeLoginFlow::new(SCHEME).client_id(client_id)
        .device_authorization_url("https://api.workos.com/user_management/authorize/device")
        .token_url(TOKEN))
}

async fn client_id() -> Result<String, CliError> {
    let base = std::env::var("SIKARU_BASE_URL").unwrap_or_else(|_| "https://api.sikaru.ai".into());
    let url = reqwest::Url::parse(&base).map_err(|_| CliError::Auth("Invalid Sikaru API URL".into()))?;
    if url.scheme() != "https" && !matches!(url.host_str(), Some("localhost" | "127.0.0.1")) {
        return Err(CliError::Auth("Sign-in configuration requires HTTPS".into()));
    }
    let response = reqwest::Client::builder().redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(10)).build()
        .map_err(|_| CliError::Auth("Sign-in configuration unavailable".into()))?
        .get(format!("{}/v1/auth/device/config", base.trim_end_matches('/'))).send().await
        .map_err(|_| CliError::Auth("Sign-in configuration unavailable".into()))?;
    if !response.status().is_success() { return Err(CliError::Auth("Sign-in configuration unavailable".into())); }
    let value: serde_json::Value = response.json().await.map_err(|_| CliError::Auth("Invalid sign-in configuration".into()))?;
    value["clientId"].as_str().filter(|s| !s.is_empty()).map(str::to_string)
        .ok_or_else(|| CliError::Auth("Sign-in is not configured".into()))
}

impl LoginFlow for UserLogin {
    fn flow_type(&self) -> &'static str { "device-code" }
    fn scheme_name(&self) -> &str { SCHEME }
    fn run(&self, ctx: &LoginContext) -> Result<(), CliError> {
        configured_flow()?.run(ctx)?;
        normalize_expiry(&ctx.cli_name)
    }
    fn build_auth_provider(&self, cli_name: &str) -> Option<DynAuthProvider> {
        Some(Arc::new(UserAuth { cli_name: cli_name.into(), flow: None }))
    }
}

/// The exchange response is trusted only for client refresh scheduling. The API
/// independently verifies signature, issuer, application and current membership.
fn normalize_expiry(cli_name: &str) -> Result<(), CliError> {
    let store = active_store();
    let Some(raw) = store.get(cli_name, SCHEME)? else { return Ok(()); };
    let Ok(mut bundle) = serde_json::from_str::<serde_json::Value>(&raw) else { return Ok(()); };
    if !bundle["expires_at"].is_null() { return Ok(()); }
    let expiry = bundle["access_token"].as_str().and_then(jwt_expiry)
        .ok_or_else(|| CliError::Auth("Sign-in token has no expiry; sign in again".into()))?;
    bundle["expires_at"] = expiry.into();
    store.set(cli_name, SCHEME, &bundle.to_string())
}

fn jwt_expiry(token: &str) -> Option<u64> {
    let encoded = token.split('.').nth(1)?;
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(encoded).ok()?;
    serde_json::from_slice::<serde_json::Value>(&bytes).ok()?["exp"].as_u64()
}

#[derive(Debug)]
struct UserAuth { cli_name: String, flow: Option<DeviceCodeLoginFlow> }
impl AuthProvider for UserAuth {
    fn name(&self) -> &str { SCHEME }
    fn has_credentials(&self) -> bool {
        std::env::var("SIKARU_API_KEY").is_ok_and(|s| !s.is_empty()) ||
            active_store().get(&self.cli_name, SCHEME).ok().flatten().is_some()
    }
    fn apply(&self, request: reqwest::RequestBuilder, endpoint: &EndpointAuthMetadata) -> Result<reqwest::RequestBuilder, CliError> {
        if endpoint.is_explicit_anonymous() { return Ok(request); }
        if let Ok(key) = std::env::var("SIKARU_API_KEY") {
            if !key.is_empty() { return Ok(request.bearer_auth(key)); }
        }
        let Some(raw) = active_store().get(&self.cli_name, SCHEME)? else { return Ok(request); };
        if serde_json::from_str::<serde_json::Value>(&raw).is_err() { return Ok(request.bearer_auth(raw)); }
        normalize_expiry(&self.cli_name)?;
        let flow = self.flow.clone().map(Ok).unwrap_or_else(configured_flow)?;
        let provider = flow.build_auth_provider(&self.cli_name)
            .ok_or_else(|| CliError::Auth("Sign-in provider unavailable".into()))?;
        let result = provider.apply(request, endpoint)?;
        normalize_expiry(&self.cli_name)?;
        Ok(result)
    }
    fn credential_hints(&self) -> Vec<String> { vec!["SIKARU_API_KEY or auth login".into()] }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fern_cli_sdk::auth::keyring_store::{set_active_store, FileKeyringStore};
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers::{method, path, body_string_contains}};
    fn token(exp: u64) -> String {
        format!("header.{}.signature", base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(
            serde_json::json!({"exp": exp}).to_string()))
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn public_device_exchange_refresh_rotation_and_project_key_precedence() {
        let server = MockServer::start().await;
        let directory = tempfile::tempdir().unwrap();
        set_active_store(Arc::new(FileKeyringStore::at_root(directory.path().to_path_buf())));
        let flow = DeviceCodeLoginFlow::new(SCHEME).client_id("client_public")
            .device_authorization_url(format!("{}/device", server.uri()))
            .token_url(format!("{}/token", server.uri()));
        Mock::given(method("POST")).and(path("/device"))
            .and(body_string_contains("client_id=client_public"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "device_code":"private-device-code", "user_code":"ABCD-EFGH",
                "verification_uri":"https://signin.example/device", "expires_in":60, "interval":1
            }))).expect(1).mount(&server).await;
        Mock::given(method("POST")).and(path("/token"))
            .and(body_string_contains("device_code=private-device-code"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token":token(1), "refresh_token":"first-refresh"
            }))).expect(1).mount(&server).await;
        Mock::given(method("POST")).and(path("/token"))
            .and(body_string_contains("grant_type=refresh_token"))
            .and(body_string_contains("refresh_token=first-refresh"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token":token(4102444800), "refresh_token":"rotated-refresh"
            }))).expect(1).mount(&server).await;
        flow.run(&LoginContext {cli_name:"fixture".into(), no_browser:true}).unwrap();
        normalize_expiry("fixture").unwrap();
        let auth = UserAuth {cli_name:"fixture".into(), flow:Some(flow)};
        let client = reqwest::Client::new();
        std::env::remove_var("SIKARU_API_KEY");
        let request = auth.apply(client.get("https://api.example"), &EndpointAuthMetadata::unspecified()).unwrap().build().unwrap();
        assert_eq!(request.headers()["authorization"], format!("Bearer {}", token(4102444800)));
        let stored: serde_json::Value = serde_json::from_str(&active_store().get("fixture", SCHEME).unwrap().unwrap()).unwrap();
        assert_eq!(stored["refresh_token"], "rotated-refresh");
        assert_eq!(stored["expires_at"], 4102444800u64);
        std::env::set_var("SIKARU_API_KEY", "project-key-fixture");
        let keyed = auth.apply(client.get("https://api.example"), &EndpointAuthMetadata::unspecified()).unwrap().build().unwrap();
        assert_eq!(keyed.headers()["authorization"], "Bearer project-key-fixture");
        std::env::remove_var("SIKARU_API_KEY");
        for request in server.received_requests().await.unwrap() {
            let form = String::from_utf8(request.body).unwrap();
            assert!(!form.contains("client_secret"));
            assert!(!form.contains("api_key"));
            assert!(request.headers["content-type"].to_str().unwrap().starts_with("application/x-www-form-urlencoded"));
        }
    }
}
