//! Every remote operation uses the generated SDK under a complete-future deadline.
use super::{config::Bootstrap, journal::Binding};
use anyhow::{bail, Result};
use sikaru_sdk::api::*;
use sikaru_sdk::{api::ApiClient, client::ApiClientBuilder, ApiError};
use std::{future::Future, sync::Arc, time::Duration};
use tokio::{sync::watch, time::Instant};
pub struct Transport {
    pub client: ApiClient,
    pub binding: Binding,
}
impl Transport {
    pub fn new(
        b: &Bootstrap,
        base_url: String,
        http: reqwest::Client,
        binding: Binding,
    ) -> Result<Arc<Self>> {
        let client = ApiClientBuilder::new(base_url)
            .token(b.token.clone())
            .max_retries(0)
            .reqwest_client(http)
            .build()
            .map_err(|_| anyhow::anyhow!("executor client configuration failed"))?;
        Ok(Arc::new(Self { client, binding }))
    }
    pub async fn status(&self, deadline: Instant) -> Result<AttachmentView> {
        bounded(
            deadline,
            self.client.compute_attachments.status(
                &self.binding.project_id,
                &self.binding.attachment_id,
                None,
            ),
        )
        .await
    }
    pub async fn poll(&self, deadline: Instant) -> Result<WorkPage> {
        bounded(
            deadline,
            self.client.compute_operations.poll(
                &self.binding.project_id,
                &self.binding.attachment_id,
                &ComputeOperationsPollQueryRequest {
                    wait_seconds: Some(1),
                    limit: Some(16),
                },
                None,
            ),
        )
        .await
    }
    pub async fn submit(&self, receipt: &ReceiptInput, deadline: Instant) -> Result<ReceiptView> {
        bounded(
            deadline,
            self.client.compute_operations.submit_receipt(
                &self.binding.project_id,
                &self.binding.attachment_id,
                receipt,
                None,
            ),
        )
        .await
    }
    pub fn validate(&self, a: &AttachmentView) -> Result<()> {
        let b = &self.binding;
        let same = a.id == b.attachment_id
            && a.project_id == b.project_id
            && a.session_id == b.session_id
            && a.journal_id == b.journal_id;
        if !same
            || a.owner_epoch != b.owner_epoch
            || a.workspace_generation != b.workspace_generation
            || a.workspace_provenance != b.workspace_provenance
        {
            bail!("attachment authority or workspace identity mismatch");
        }
        Ok(())
    }
    pub fn ready_input(&self, instance: &str) -> ReadyInput {
        ReadyInput {
            executor_instance_id: instance.into(),
            journal_id: self.binding.journal_id.clone(),
            workspace_provenance: self.binding.workspace_provenance.clone(),
            protocol_version: ReadyInputProtocolVersion::SikaruComputeV1,
            capabilities: vec![
                ReadyInputCapabilitiesItem::ComputeExecute,
                ReadyInputCapabilitiesItem::BashRun,
            ],
        }
    }
    pub async fn ready(&self, instance: &str, deadline: Instant) -> Result<Instant> {
        let start = Instant::now();
        let a = bounded(
            deadline,
            self.client.compute_attachments.ready(
                &self.binding.project_id,
                &self.binding.attachment_id,
                &self.ready_input(instance),
                None,
            ),
        )
        .await?;
        self.validate(&a)?;
        renewal_deadline(start, &a)
    }
    pub async fn connect(&self, instance: &str, deadline: Instant) -> Result<()> {
        let a = bounded(
            deadline,
            self.client.compute_attachments.connect(
                &self.binding.project_id,
                &self.binding.attachment_id,
                &self.ready_input(instance),
                None,
            ),
        )
        .await?;
        self.validate(&a)
    }
    pub async fn reconcile(
        &self,
        body: &ReconcileInput,
        deadline: Instant,
    ) -> Result<ReconcileView> {
        bounded(
            deadline,
            self.client.compute_attachments.reconcile(
                &self.binding.project_id,
                &self.binding.attachment_id,
                body,
                None,
            ),
        )
        .await
    }
    pub async fn heartbeat(&self, deadline: Instant) -> Result<Instant> {
        let start = Instant::now();
        let a = bounded(
            deadline,
            self.client.compute_attachments.heartbeat(
                &self.binding.project_id,
                &self.binding.attachment_id,
                None,
            ),
        )
        .await?;
        self.validate(&a)?;
        renewal_deadline(start, &a)
    }
    pub async fn cleanup(&self, confirmed: bool) -> bool {
        let body = CleanupInput {
            children_terminated: confirmed,
            evidence: if confirmed {
                "native owned process groups terminated"
            } else {
                "native cleanup could not be proven"
            }
            .into(),
        };
        bounded(
            Instant::now() + Duration::from_secs(5),
            self.client.compute_attachments.cleanup(
                &self.binding.project_id,
                &self.binding.attachment_id,
                &body,
                None,
            ),
        )
        .await
        .is_ok()
    }
    pub async fn stop(&self) -> bool {
        bounded(
            Instant::now() + Duration::from_secs(5),
            self.client.compute_attachments.stop(
                &self.binding.project_id,
                &self.binding.attachment_id,
                None,
            ),
        )
        .await
        .is_ok()
    }
}
pub async fn bounded<T>(
    deadline: Instant,
    future: impl Future<Output = Result<T, ApiError>>,
) -> Result<T> {
    let end = deadline.min(Instant::now() + Duration::from_secs(5));
    // Do not format ApiError: arbitrary error bodies may contain credentials.
    tokio::time::timeout_at(end, future)
        .await
        .map_err(|_| anyhow::Error::new(TransportFailure::Transient))?
        .map_err(classify)
}
fn renewal_deadline(start: Instant, a: &AttachmentView) -> Result<Instant> {
    let ttl = a
        .lease_ttl_seconds
        .filter(|n| *n > 0 && *n <= 180)
        .ok_or_else(|| anyhow::anyhow!("invalid authority lease TTL"))?;
    let end = start + Duration::from_secs(ttl as u64);
    if end <= Instant::now() {
        bail!("lease renewal arrived after its deadline");
    }
    Ok(end)
}
/// Classify without retaining arbitrary remote bodies or credential-bearing URLs.
#[derive(Debug, thiserror::Error)]
pub enum TransportFailure {
    #[error("transient_transport_failure")]
    Transient,
    #[error("permanent_transport_failure")]
    Permanent,
    #[error("request_rejected_{0}")]
    Rejected(u16),
}
pub fn classify(error: ApiError) -> anyhow::Error {
    if let Some(status) = rejected_status(&error) {
        return anyhow::Error::new(TransportFailure::Rejected(status));
    }
    let transient = match error {
        ApiError::Network(_) | ApiError::ServiceUnavailableError { .. } => true,
        ApiError::Http { status, .. } => status == 429 || (500..600).contains(&status),
        _ => false,
    };
    anyhow::Error::new(if transient {
        TransportFailure::Transient
    } else {
        TransportFailure::Permanent
    })
}
fn rejected_status(error: &ApiError) -> Option<u16> {
    match error {
        ApiError::UnauthorizedError { .. } => Some(401),
        ApiError::ForbiddenError { .. } => Some(403),
        ApiError::NotFoundError { .. } => Some(404),
        ApiError::Http { status, .. } if matches!(status, 401..=404) => Some(*status),
        _ => None,
    }
}
pub fn transient(error: &anyhow::Error) -> bool {
    matches!(
        error.downcast_ref::<TransportFailure>(),
        Some(TransportFailure::Transient)
    )
}
pub fn backoff(attempt: &mut u32) -> Duration {
    let millis = 100u64 * (1u64 << (*attempt).min(4));
    *attempt = attempt.saturating_add(1);
    Duration::from_millis(millis + rand::random::<u64>() % 100)
}
pub async fn maintain_lease(
    transport: Arc<Transport>,
    lease: watch::Sender<Instant>,
) -> Result<()> {
    let initial = *lease.borrow();
    tokio::try_join!(
        maintain_heartbeats(&transport, &lease),
        maintain_credentials(
            &transport.client,
            &transport.binding.project_id,
            Some(transport.binding.credential_id.clone()),
            initial,
            Some(lease.subscribe())
        )
    )?;
    Ok(())
}
async fn maintain_heartbeats(transport: &Transport, lease: &watch::Sender<Instant>) -> Result<()> {
    loop {
        let deadline = *lease.borrow();
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            bail!("lease_expired");
        }
        tokio::time::sleep((remaining / 3).min(Duration::from_secs(10))).await;
        match transport.heartbeat(deadline).await {
            Ok(end) => {
                lease.send_replace(end);
            }
            Err(error) if transient(&error) => {
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            Err(error) => return Err(error),
        }
    }
}
/// The initial bound is conservative until the first authenticated renewal establishes expiry.
/// This future runs independently of heartbeat, queue polling, and active launch tasks.
pub async fn maintain_credentials(
    client: &ApiClient,
    project: &str,
    mut identity: Option<String>,
    mut expires: Instant,
    lease: Option<watch::Receiver<Instant>>,
) -> Result<()> {
    let mut attempts = 0;
    loop {
        let deadline = lease
            .as_ref()
            .map(|v| *v.borrow())
            .unwrap_or(expires)
            .min(expires);
        if Instant::now() >= deadline {
            bail!("credential_or_lease_expired");
        }
        let started = Instant::now();
        let wall = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs_f64();
        let response = bounded(deadline, client.compute_credentials.renew(project, None)).await;
        let delay = match response {
            Ok(value) => {
                expires = credential_deadline(value, &mut identity, started, wall)?;
                attempts = 0;
                (expires.saturating_duration_since(Instant::now()) / 3).min(Duration::from_secs(30))
            }
            Err(error) if transient(&error) => backoff(&mut attempts),
            Err(error) => return Err(error),
        };
        tokio::time::sleep_until((Instant::now() + delay).min(expires)).await;
    }
}
fn credential_deadline(
    value: CredentialRenewed,
    identity: &mut Option<String>,
    started: Instant,
    wall: f64,
) -> Result<Instant> {
    if identity
        .as_ref()
        .is_some_and(|id| id != &value.credential_id)
    {
        bail!("credential renewal identity mismatch");
    }
    let seconds = value.expires_at - wall;
    if !seconds.is_finite() || seconds <= 0.0 {
        bail!("credential_expired");
    }
    let expires = started + Duration::from_secs_f64(seconds.min(86400.0));
    if expires <= Instant::now() {
        bail!("credential_expired");
    }
    *identity = Some(value.credential_id);
    Ok(expires)
}
