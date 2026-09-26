//! Every remote operation uses the generated SDK under a complete-future deadline.
//! The persistent executor channel carries the same work pages and receipts as frames
//! whose shapes come from the generated channel types.
use super::{config::Bootstrap, journal::Binding};
use anyhow::{bail, Context, Result};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use sikaru_sdk::api::*;
use sikaru_sdk::{api::ApiClient, client::ApiClientBuilder, ApiError};
use std::{future::Future, sync::Arc, time::Duration};
use tokio::{
    sync::{mpsc, watch},
    time::Instant,
};
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, http, Message};
pub struct Transport {
    pub client: ApiClient,
    pub binding: Binding,
    channel_url: Result<String, String>,
    token: String,
}
impl Transport {
    pub fn new(
        b: &Bootstrap,
        base_url: String,
        http: reqwest::Client,
        binding: Binding,
    ) -> Result<Arc<Self>> {
        let channel_url = channel_url(&base_url, &binding).map_err(|e| e.to_string());
        let client = ApiClientBuilder::new(base_url)
            .token(b.token.clone())
            .max_retries(0)
            .reqwest_client(http)
            .build()
            .map_err(|_| anyhow::anyhow!("executor client configuration failed"))?;
        Ok(Arc::new(Self {
            client,
            binding,
            channel_url,
            token: b.token.clone(),
        }))
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
    pub async fn workspace_checkpoint(
        &self,
        run_id: &str,
        deadline: Instant,
    ) -> Result<WorkspaceCheckpointView> {
        bounded(
            deadline,
            self.client.compute_workspaces.get(
                &self.binding.project_id,
                &self.binding.attachment_id,
                run_id,
                None,
            ),
        )
        .await
    }
    pub async fn workspace_blob(
        &self,
        run_id: &str,
        hash: &str,
        bytes: Vec<u8>,
        deadline: Instant,
    ) -> Result<WorkspaceBlobView> {
        bounded(
            deadline,
            self.client.compute_workspaces.put_blob(
                &self.binding.project_id,
                &self.binding.attachment_id,
                run_id,
                hash,
                &bytes,
                None,
            ),
        )
        .await
    }
    pub async fn workspace_tree(
        &self,
        run_id: &str,
        tree: &WorkspaceTreeInput,
        deadline: Instant,
    ) -> Result<WorkspaceCheckpointView> {
        bounded(
            deadline,
            self.client.compute_workspaces.commit_tree(
                &self.binding.project_id,
                &self.binding.attachment_id,
                run_id,
                tree,
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
                ReadyInputCapabilitiesItem::FilesystemCheckpointV1,
                ReadyInputCapabilitiesItem::ConditionWaitsV1,
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

// ---- Persistent executor channel -------------------------------------------------

type Socket =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;
const CHANNEL_OPEN_TIMEOUT: Duration = Duration::from_secs(5);
const RECEIPT_ACK_TIMEOUT: Duration = Duration::from_secs(15);
const PATH_SEGMENT: &percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');

/// Why a channel ended, which decides what the executor does next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelEnd {
    /// The turn selects polling, or the page needs the HTTP route: poll now.
    Poll,
    /// Credential, owner, generation, environment or stop: poll to surface the authority.
    Fenced,
    /// The connection failed: reconcile, then reconnect with jittered backoff.
    Lost,
}
pub enum ChannelEvent {
    Page(Box<WorkPage>),
    /// The receipt for this tool call was recorded.
    Accepted(String),
    /// A receipt was refused, for this tool call when named.
    Rejected(Option<String>),
    Ended(ChannelEnd),
}
#[derive(Debug, PartialEq, Eq)]
pub enum Delivery {
    Accepted,
    NotAccepted,
}
pub enum Opened {
    Channel(Channel),
    Ended(ChannelEnd),
}

/// One open channel. Socket I/O runs in two pump tasks that own only the socket
/// halves, so heartbeats continue while an operation executes; dropping the
/// channel aborts both.
pub struct Channel {
    outbound: Option<mpsc::Sender<String>>,
    inbound: mpsc::Receiver<ChannelEvent>,
    page: Option<Box<WorkPage>>,
    ended: Option<ChannelEnd>,
    writer: Option<tokio::task::JoinHandle<()>>,
    reader: tokio::task::JoinHandle<()>,
}
impl Drop for Channel {
    fn drop(&mut self) {
        self.reader.abort();
        if let Some(writer) = &self.writer {
            writer.abort();
        }
    }
}
impl Channel {
    pub fn ended(&self) -> Option<ChannelEnd> {
        self.ended
    }
    /// The newest work page, or how the channel ended.
    pub async fn next(&mut self) -> ChannelEvent {
        loop {
            if let Some(page) = self.page.take() {
                return ChannelEvent::Page(self.newest(page));
            }
            if let Some(end) = self.ended {
                return ChannelEvent::Ended(end);
            }
            match self.recv().await {
                ChannelEvent::Page(page) => self.page = Some(page),
                ChannelEvent::Ended(end) => self.ended = Some(end),
                _ => {}
            }
        }
    }
    /// Pages already queued supersede older ones: each is the full current work page.
    fn newest(&mut self, mut page: Box<WorkPage>) -> Box<WorkPage> {
        while let Ok(event) = self.inbound.try_recv() {
            match event {
                ChannelEvent::Page(next) => page = next,
                ChannelEvent::Ended(end) => self.ended = Some(end),
                _ => {}
            }
        }
        page
    }
    async fn recv(&mut self) -> ChannelEvent {
        self.inbound
            .recv()
            .await
            .unwrap_or(ChannelEvent::Ended(ChannelEnd::Lost))
    }
    /// Send one receipt and wait for its acceptance. Anything else leaves delivery to the HTTP route.
    pub async fn deliver(&mut self, receipt: &ReceiptInput, deadline: Instant) -> Delivery {
        if self.ended.is_some() || !self.send_receipt(receipt).await {
            return Delivery::NotAccepted;
        }
        let end = deadline.min(Instant::now() + RECEIPT_ACK_TIMEOUT);
        loop {
            let Ok(event) = tokio::time::timeout_at(end, self.recv()).await else {
                self.ended = Some(ChannelEnd::Lost);
                return Delivery::NotAccepted;
            };
            match event {
                ChannelEvent::Accepted(id) if id == receipt.tool_call_id => return Delivery::Accepted,
                ChannelEvent::Rejected(id) if id.as_ref().is_none_or(|id| *id == receipt.tool_call_id) => {
                    return Delivery::NotAccepted
                }
                ChannelEvent::Page(page) => self.page = Some(page),
                ChannelEvent::Ended(end) => {
                    self.ended = Some(end);
                    return Delivery::NotAccepted;
                }
                _ => {}
            }
        }
    }
    async fn send_receipt(&mut self, receipt: &ReceiptInput) -> bool {
        let frame = ClientFrame::receipt(receipt.clone());
        let (Ok(text), Some(outbound)) = (serde_json::to_string(&frame), &self.outbound) else {
            return false;
        };
        if outbound.send(text).await.is_err() {
            self.ended = Some(ChannelEnd::Lost);
            return false;
        }
        true
    }
    /// Close the socket gracefully, then stop both pumps.
    pub async fn close(mut self) {
        self.outbound.take();
        if let Some(writer) = self.writer.take() {
            let _ = tokio::time::timeout(Duration::from_secs(1), writer).await;
        }
    }
}

impl Transport {
    /// Upgrade with the restricted credential in the Authorization header only.
    pub async fn open_channel(&self) -> Opened {
        let Ok(request) = self.channel_request() else {
            return Opened::Ended(ChannelEnd::Lost);
        };
        let end = Instant::now() + CHANNEL_OPEN_TIMEOUT;
        let socket =
            match tokio::time::timeout_at(end, tokio_tungstenite::connect_async(request)).await {
                Ok(Ok((socket, _))) => socket,
                Ok(Err(error)) => return Opened::Ended(refusal(&error)),
                Err(_) => return Opened::Ended(ChannelEnd::Lost),
            };
        let (sink, mut stream) = socket.split();
        let handshake = match tokio::time::timeout_at(end, stream.next()).await {
            Ok(Some(Ok(Message::Text(text)))) => decode(&text).and_then(Handshake::from_frame),
            _ => None,
        };
        let Some(handshake) = handshake else {
            return Opened::Ended(ChannelEnd::Lost);
        };
        match self.admit(&handshake) {
            Ok(()) => Opened::Channel(Channel::start(sink, stream, &handshake)),
            Err(end) => Opened::Ended(end),
        }
    }
    fn channel_request(&self) -> Result<http::Request<()>> {
        let url = self
            .channel_url
            .as_deref()
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        let mut request = url.into_client_request()?;
        let bearer = http::HeaderValue::from_str(&format!("Bearer {}", self.token))?;
        request
            .headers_mut()
            .insert(http::header::AUTHORIZATION, bearer);
        Ok(request)
    }
    /// The handshake must name this attachment's authority and select the channel.
    fn admit(&self, h: &Handshake) -> Result<(), ChannelEnd> {
        let b = &self.binding;
        if h.attachment_id != b.attachment_id
            || h.owner_epoch != b.owner_epoch
            || h.workspace_generation != b.workspace_generation
            || !h.protocol_ok
        {
            return Err(ChannelEnd::Fenced);
        }
        if h.channel {
            Ok(())
        } else {
            Err(ChannelEnd::Poll)
        }
    }
}
fn refusal(error: &tokio_tungstenite::tungstenite::Error) -> ChannelEnd {
    match error {
        tokio_tungstenite::tungstenite::Error::Http(response)
            if matches!(response.status().as_u16(), 401 | 403 | 409) =>
        {
            ChannelEnd::Fenced
        }
        _ => ChannelEnd::Lost,
    }
}
pub fn channel_url(base_url: &str, b: &Binding) -> Result<String> {
    let base = base_url.trim_end_matches('/');
    let (scheme, rest) = if let Some(rest) = base.strip_prefix("https://") {
        ("wss://", rest)
    } else {
        (
            "ws://",
            base.strip_prefix("http://")
                .context("unsupported base URL scheme")?,
        )
    };
    let segment = |s: &str| percent_encoding::utf8_percent_encode(s, PATH_SEGMENT).to_string();
    Ok(format!(
        "{scheme}{rest}/v1/projects/{}/compute-attachments/{}/channel",
        segment(&b.project_id),
        segment(&b.attachment_id)
    ))
}

impl Channel {
    fn start(
        sink: SplitSink<Socket, Message>,
        stream: SplitStream<Socket>,
        h: &Handshake,
    ) -> Self {
        let (interval, silence) = heartbeat_bounds(h);
        let (events, inbound) = mpsc::channel(64);
        let (outbound, queue) = mpsc::channel(8);
        let reader = tokio::spawn(read_pump(stream, events.clone(), silence));
        let writer = tokio::spawn(write_pump(sink, queue, events, interval));
        Self {
            outbound: Some(outbound),
            inbound,
            page: None,
            ended: None,
            writer: Some(writer),
            reader,
        }
    }
}
/// The handshake fields the executor acts on, from the generated server frame.
struct Handshake {
    attachment_id: String,
    owner_epoch: i64,
    workspace_generation: String,
    protocol_ok: bool,
    channel: bool,
    heartbeat_interval_seconds: f64,
    heartbeat_timeout_seconds: f64,
}
impl Handshake {
    fn from_frame(frame: ServerFrame) -> Option<Self> {
        let ServerFrame::Handshake {
            attachment_id,
            owner_epoch,
            workspace_generation,
            protocol,
            transport,
            heartbeat_interval_seconds,
            heartbeat_timeout_seconds,
            ..
        } = frame
        else {
            return None;
        };
        Some(Self {
            attachment_id,
            owner_epoch,
            workspace_generation,
            protocol_ok: protocol
                .is_none_or(|p| p == HandshakeFrameProtocol::SikaruComputeChannelV1),
            channel: transport == HandshakeFrameTransport::Channel,
            heartbeat_interval_seconds,
            heartbeat_timeout_seconds,
        })
    }
}
/// Heartbeat cadence and silence limit from the handshake, kept within sane bounds.
fn heartbeat_bounds(h: &Handshake) -> (Duration, Duration) {
    let seconds = |v: f64, low: f64, high: f64| {
        Duration::from_secs_f64(if v.is_finite() {
            v.clamp(low, high)
        } else {
            high
        })
    };
    let interval = seconds(h.heartbeat_interval_seconds, 0.05, 300.0);
    let silence = seconds(h.heartbeat_timeout_seconds, 0.1, 600.0).max(interval);
    (interval, silence)
}
async fn read_pump(
    mut stream: SplitStream<Socket>,
    events: mpsc::Sender<ChannelEvent>,
    silence: Duration,
) {
    let end = loop {
        let message = match tokio::time::timeout(silence, stream.next()).await {
            Ok(Some(Ok(message))) => message,
            _ => break ChannelEnd::Lost,
        };
        match inbound(message) {
            Inbound::Skip => {}
            Inbound::Event(event) => {
                if events.send(event).await.is_err() {
                    return;
                }
            }
            Inbound::End(end) => break end,
        }
    };
    let _ = events.send(ChannelEvent::Ended(end)).await;
}
async fn write_pump(
    mut sink: SplitSink<Socket, Message>,
    mut queue: mpsc::Receiver<String>,
    events: mpsc::Sender<ChannelEvent>,
    interval: Duration,
) {
    let mut beat = tokio::time::interval_at(Instant::now() + interval, interval);
    loop {
        let text = tokio::select! {
            text = queue.recv() => match text {
                Some(text) => text,
                None => {
                    let _ = sink.close().await;
                    return;
                }
            },
            _ = beat.tick() => heartbeat_text(),
        };
        if sink.send(Message::Text(text)).await.is_err() {
            let _ = events.send(ChannelEvent::Ended(ChannelEnd::Lost)).await;
            return;
        }
    }
}

/// The contract's heartbeat frame, `{"type": "heartbeat"}`.
pub fn heartbeat_text() -> String {
    serde_json::to_string(&ClientFrame::heartbeat(HeartbeatFrame::default())).unwrap_or_default()
}

enum Inbound {
    Skip,
    Event(ChannelEvent),
    End(ChannelEnd),
}
/// Frames decode by their `type` tag into the generated server frame union.
fn decode(text: &str) -> Option<ServerFrame> {
    serde_json::from_str(text).ok()
}
fn inbound(message: Message) -> Inbound {
    let text = match message {
        Message::Text(text) => text,
        Message::Ping(_) | Message::Pong(_) => return Inbound::Skip,
        Message::Close(frame) => {
            return Inbound::End(frame.map_or(ChannelEnd::Lost, |f| close_code(f.code.into())))
        }
        _ => return Inbound::End(ChannelEnd::Lost),
    };
    decode(&text).map_or(Inbound::End(ChannelEnd::Lost), frame_event)
}
/// A handshake after the first frame, or an unknown frame, ends the channel.
fn frame_event(frame: ServerFrame) -> Inbound {
    match frame {
        ServerFrame::Heartbeat { .. } => Inbound::Skip,
        ServerFrame::Operations { page, .. } => Inbound::Event(ChannelEvent::Page(Box::new(page))),
        ServerFrame::ReceiptAccepted { receipt, .. } => {
            Inbound::Event(ChannelEvent::Accepted(receipt.tool_call_id))
        }
        ServerFrame::ReceiptRejected { tool_call_id, .. } => {
            Inbound::Event(ChannelEvent::Rejected(tool_call_id))
        }
        ServerFrame::Close { reason, .. } => Inbound::End(close_reason(&reason)),
        _ => Inbound::End(ChannelEnd::Lost),
    }
}
fn close_reason(reason: &CloseFrameReason) -> ChannelEnd {
    match reason {
        CloseFrameReason::TransportPoll => ChannelEnd::Poll,
        CloseFrameReason::CredentialInvalid
        | CloseFrameReason::EnvironmentDisabled
        | CloseFrameReason::StaleOwner
        | CloseFrameReason::WorkspaceGenerationChanged
        | CloseFrameReason::AttachmentStopped => ChannelEnd::Fenced,
        _ => ChannelEnd::Lost,
    }
}
/// A close without a close frame, by its WebSocket code.
fn close_code(code: u16) -> ChannelEnd {
    match code {
        1000 => ChannelEnd::Poll,
        4401 | 4403 | 4409 | 4410 | 4412 => ChannelEnd::Fenced,
        _ => ChannelEnd::Lost,
    }
}
/// Reconnect delay: exponential, spread uniformly over [base/2, 3·base/2] so many
/// executors dropped together do not reconnect together.
pub fn channel_backoff(attempt: u32) -> Duration {
    let base = 250u64 << attempt.min(5);
    Duration::from_millis(base / 2 + rand::random::<u64>() % (base + 1))
}

#[cfg(test)]
mod channel_tests {
    use super::*;
    use serde_json::{json, Value};

    fn receipt() -> ReceiptInput {
        serde_json::from_value(json!({"run_id":"run","tool_call_id":"call","tool_provider_id":"provider",
            "capability_name":"compute.execute","idempotency_key":"key","request_digest":"a".repeat(64),
            "status":"completed","payload":{"written":true}}))
        .unwrap()
    }
    fn receipt_frame() -> Value {
        serde_json::to_value(ClientFrame::receipt(receipt())).unwrap()
    }
    fn page() -> Value {
        json!({"attachment":{"id":"attachment","session_id":"session","project_id":"project","environment_id":"environment",
            "provider_id":"provider","workspace_generation":"generation","workspace_provenance":{"kind":"existing_directory","identity":"original"},
            "journal_id":"journal","status":"ready","owner_id":"worker","owner_epoch":1,"lease_until":1.0,"lease_ttl_seconds":60,
            "startup_deadline":1.0,"capabilities":[],"protocol_version":null,"cleanup_at":null,"cleanup_status":"unconfirmed",
            "uncertain_operations":[],"processes":[]},
            "execution_phase":"running","execution":{"run_id":"run","status":"running","terminal":false,"approval_required":false},"poll_after_seconds":1,"operations":[],"issued_operations":[],"live_handles":[],"transport":"channel"})
    }
    /// One complete sample of every server frame, keyed by its AsyncAPI schema name.
    fn server_frames() -> Vec<(&'static str, Value)> {
        vec![
            (
                "HandshakeFrame",
                json!({"type":"handshake","protocol":"sikaru-compute-channel-v1","attachment_id":"attachment",
                "owner_epoch":1,"workspace_generation":"generation","transport":"channel","heartbeat_interval_seconds":25.0,
                "heartbeat_timeout_seconds":60.0,"max_frame_bytes":262144,"inbound_frame_limit":2000,"inbound_frame_window_seconds":300.0}),
            ),
            ("OperationsFrame", json!({"type":"operations","page":page()})),
            (
                "ReceiptAcceptedFrame",
                json!({"type":"receipt_accepted","run_id":"run","receipt":{"tool_call_id":"call","created":true,"status":"accepted"}}),
            ),
            (
                "ReceiptRejectedFrame",
                json!({"type":"receipt_rejected","status":409,"detail":"conflict","run_id":"run","tool_call_id":"call"}),
            ),
            ("HeartbeatFrame", json!({"type":"heartbeat"})),
            (
                "CloseFrame",
                json!({"type":"close","reason":"transport_poll","detail":"poll"}),
            ),
        ]
    }
    fn event(sample: &Value) -> Inbound {
        decode(&sample.to_string()).map_or(Inbound::End(ChannelEnd::Lost), frame_event)
    }

    #[test]
    fn client_frames_carry_exactly_one_type_tag() {
        assert_eq!(heartbeat_text(), r#"{"type":"heartbeat"}"#);
        let frame = receipt_frame();
        assert_eq!(frame["type"], "receipt");
        assert_eq!(frame["receipt"]["tool_call_id"], "call");
        assert_eq!(frame.as_object().unwrap().len(), 2);
    }
    #[test]
    fn server_frames_decode_by_type_and_close_reasons_choose_the_next_transport() {
        let frames = server_frames();
        let handshake = decode(&frames[0].1.to_string()).and_then(Handshake::from_frame).unwrap();
        assert!(handshake.channel && handshake.protocol_ok && handshake.owner_epoch == 1);
        assert!(
            matches!(event(&frames[1].1), Inbound::Event(ChannelEvent::Page(p)) if p.transport == Some(WorkPageTransport::Channel))
        );
        assert!(
            matches!(event(&frames[2].1), Inbound::Event(ChannelEvent::Accepted(id)) if id == "call")
        );
        assert!(
            matches!(event(&frames[3].1), Inbound::Event(ChannelEvent::Rejected(Some(id))) if id == "call")
        );
        assert!(matches!(event(&frames[4].1), Inbound::Skip));
        // A second handshake or an unknown frame ends the channel.
        assert!(matches!(event(&frames[0].1), Inbound::End(ChannelEnd::Lost)));
        assert!(matches!(event(&json!({"type":"unknown"})), Inbound::End(ChannelEnd::Lost)));
        let end = |reason: &str| match event(&json!({"type":"close","reason":reason,"detail":""})) {
            Inbound::End(end) => end,
            _ => panic!("close frame"),
        };
        assert_eq!(end("transport_poll"), ChannelEnd::Poll);
        for fenced in [
            "credential_invalid",
            "environment_disabled",
            "stale_owner",
            "workspace_generation_changed",
            "attachment_stopped",
        ] {
            assert_eq!(end(fenced), ChannelEnd::Fenced, "{fenced}");
        }
        for lost in [
            "heartbeat_timeout",
            "rate_limited",
            "frame_too_large",
            "invalid_frame",
        ] {
            assert_eq!(end(lost), ChannelEnd::Lost, "{lost}");
        }
        assert_eq!(close_code(1000), ChannelEnd::Poll);
        assert_eq!(close_code(4409), ChannelEnd::Fenced);
        assert_eq!(close_code(1012), ChannelEnd::Lost);
    }
    #[test]
    fn a_handshake_for_another_protocol_or_transport_is_not_admitted() {
        let mut sample = server_frames()[0].1.clone();
        sample["protocol"] = json!("sikaru-compute-channel-v2");
        assert!(decode(&sample.to_string())
            .and_then(Handshake::from_frame)
            .is_none_or(|h| !h.protocol_ok));
        let mut sample = server_frames()[0].1.clone();
        sample["transport"] = json!("poll");
        assert!(!decode(&sample.to_string()).and_then(Handshake::from_frame).unwrap().channel);
    }
    #[test]
    fn reconnect_delays_are_spread_not_synchronized() {
        for attempt in 1..=3u32 {
            let base = 250u64 << attempt;
            let delays = (0..500)
                .map(|_| channel_backoff(attempt).as_millis() as u64)
                .collect::<Vec<_>>();
            let (low, high) = (*delays.iter().min().unwrap(), *delays.iter().max().unwrap());
            assert!(low >= base / 2 && high <= base * 3 / 2, "{low}..{high}");
            assert!(high - low >= base / 2, "delays cluster: {low}..{high}");
            let distinct = delays
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len();
            assert!(distinct > 100, "only {distinct} distinct delays");
        }
    }
    /// Set SIKARU_CHANNEL_ASYNCAPI to the channel AsyncAPI document to check these
    /// frames against it: each frame's tag selects its union variant, the rest of the
    /// frame carries exactly that variant's fields, and every frame survives the
    /// generated union unchanged.
    #[test]
    fn frames_round_trip_the_channel_asyncapi_schema() {
        let Ok(path) = std::env::var("SIKARU_CHANNEL_ASYNCAPI") else {
            return;
        };
        let document: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        let schemas = &document["components"]["schemas"];
        let heartbeat: Value = serde_json::from_str(&heartbeat_text()).unwrap();
        for (name, frame) in [
            ("HeartbeatFrame", heartbeat),
            ("ReceiptFrame", receipt_frame()),
        ] {
            conforms(schemas, "ClientFrame", name, &frame);
            let typed: ClientFrame = serde_json::from_value(frame.clone()).unwrap();
            assert_eq!(serde_json::to_value(typed).unwrap(), frame, "{name}");
        }
        for (name, sample) in server_frames() {
            conforms(schemas, "ServerFrame", name, &sample);
            let typed = serde_json::to_value(decode(&sample.to_string()).unwrap()).unwrap();
            // The page's own fields are checked against WorkPage below.
            for key in sample.as_object().unwrap().keys().filter(|k| *k != "page") {
                let same = match (typed[key].as_f64(), sample[key].as_f64()) {
                    (Some(a), Some(b)) => a == b,
                    _ => typed[key] == sample[key],
                };
                assert!(same, "{name}.{key} changed through the generated type");
            }
        }
        let page_fields = schemas["WorkPage"]["properties"].as_object().unwrap();
        let typed: WorkPage = serde_json::from_value(page()).unwrap();
        let mut typed = serde_json::to_value(typed).unwrap();
        typed["workspace_checkpoint"] = Value::Null;
        for key in page_fields.keys() {
            assert!(
                typed.get(key).is_some(),
                "WorkPage.{key} is missing from the generated type"
            );
        }
    }
    fn conforms(schemas: &Value, union: &str, name: &str, frame: &Value) {
        let tag = frame["type"].as_str().unwrap();
        let mapping = &schemas[union]["discriminator"]["mapping"];
        assert_eq!(
            mapping[tag].as_str(),
            Some(format!("#/components/schemas/{name}").as_str()),
            "{union} tag {tag}"
        );
        let schema = &schemas[name];
        let properties = schema["properties"]
            .as_object()
            .unwrap_or_else(|| panic!("{name} schema"));
        assert!(!properties.contains_key("type"), "{name} repeats its tag");
        let object = frame.as_object().unwrap();
        for key in object.keys().filter(|k| *k != "type") {
            assert!(properties.contains_key(key), "{name} has no field {key}");
        }
        for required in schema["required"].as_array().into_iter().flatten() {
            assert!(
                object.contains_key(required.as_str().unwrap()),
                "{name} misses {required}"
            );
        }
        for (key, property) in properties {
            if let (Some(values), Some(value)) = (property["enum"].as_array(), object.get(key)) {
                assert!(values.contains(value), "{name}.{key}={value}");
            }
        }
    }
}
