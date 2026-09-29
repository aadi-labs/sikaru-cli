//! Git smart-HTTP receive-pack over the CLI's own HTTP client: one ref, report-status only.
use anyhow::{bail, Result};
use base64::Engine;
use bytes::Bytes;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};

pub const ZERO: &str = "0000000000000000000000000000000000000000";
const CHUNK: usize = 256 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum GitHttpError {
    #[error("remote_unauthorized")]
    Unauthorized,
    #[error("remote_status_{0}")]
    Status(u16),
    #[error("remote_protocol")]
    Protocol,
}
#[derive(Debug, PartialEq, Eq)]
pub enum PushOutcome {
    Accepted,
    /// The remote did not move the ref: its head changed, or the push broke its policy.
    Refused,
}
#[derive(Debug)]
pub struct Advertisement {
    pub head: Option<String>,
    /// Commits the remote already has on other branches; packs may omit their objects.
    pub haves: Vec<String>,
}
pub struct Remote<'a> {
    pub http: &'a reqwest::Client,
    pub url: &'a str,
    pub username: &'a str,
    pub token: &'a str,
    pub refname: &'a str,
}
impl Remote<'_> {
    fn endpoint(&self, suffix: &str) -> String {
        format!("{}/{suffix}", self.url.trim_end_matches('/'))
    }
    fn authorization(&self) -> String {
        let pair = format!("{}:{}", self.username, self.token);
        format!(
            "Basic {}",
            base64::engine::general_purpose::STANDARD.encode(pair)
        )
    }
    pub async fn advertisement(&self) -> Result<Advertisement> {
        let response = self
            .http
            .get(self.endpoint("info/refs?service=git-receive-pack"))
            .header(AUTHORIZATION, self.authorization())
            .send()
            .await?;
        parse_advertisement(&checked(response).await?, self.refname)
    }
    pub async fn push(
        &self,
        old: Option<&str>,
        new: &str,
        pack: Bytes,
        progress: impl Fn(u64, u64) + Send + Sync + 'static,
    ) -> Result<PushOutcome> {
        let command = format!(
            "{} {new} {}\0report-status agent=sikaru\n",
            old.unwrap_or(ZERO),
            self.refname
        );
        let mut head = pkt_line(command.as_bytes());
        head.extend_from_slice(b"0000");
        let total = (head.len() + pack.len()) as u64;
        let response = self
            .http
            .post(self.endpoint("git-receive-pack"))
            .header(AUTHORIZATION, self.authorization())
            .header(CONTENT_TYPE, "application/x-git-receive-pack-request")
            .header(ACCEPT, "application/x-git-receive-pack-result")
            .header(CONTENT_LENGTH, total)
            .body(reqwest::Body::wrap_stream(chunks(
                Bytes::from(head),
                pack,
                progress,
            )))
            .send()
            .await?;
        if response.status().as_u16() == 409 {
            return Ok(PushOutcome::Refused);
        }
        parse_report(&checked(response).await?, self.refname)
    }
}
/// The request body in fixed chunks, reporting bytes handed to the connection.
fn chunks(
    head: Bytes,
    pack: Bytes,
    progress: impl Fn(u64, u64) + Send + Sync + 'static,
) -> impl futures_util::Stream<Item = std::io::Result<Bytes>> {
    let total = (head.len() + pack.len()) as u64;
    let mut sent = 0u64;
    let parts = std::iter::once(head).chain(
        (0..pack.len())
            .step_by(CHUNK)
            .map(move |start| pack.slice(start..(start + CHUNK).min(pack.len()))),
    );
    futures_util::stream::iter(parts.map(move |part| {
        sent += part.len() as u64;
        progress(sent, total);
        Ok(part)
    }))
}
/// Response bodies are never formatted into errors: they may echo credentials.
async fn checked(response: reqwest::Response) -> Result<Bytes> {
    match response.status().as_u16() {
        200 => Ok(response.bytes().await?),
        401 | 403 => bail!(GitHttpError::Unauthorized),
        code => bail!(GitHttpError::Status(code)),
    }
}
pub fn pkt_line(data: &[u8]) -> Vec<u8> {
    let mut out = format!("{:04x}", data.len() + 4).into_bytes();
    out.extend_from_slice(data);
    out
}
/// Split a pkt-line stream; `None` is a flush packet.
pub fn pkt_lines(mut data: &[u8]) -> Result<Vec<Option<&[u8]>>> {
    let mut lines = Vec::new();
    while !data.is_empty() {
        let size = data
            .get(..4)
            .and_then(|h| std::str::from_utf8(h).ok())
            .and_then(|h| usize::from_str_radix(h, 16).ok())
            .ok_or(GitHttpError::Protocol)?;
        if size == 0 {
            lines.push(None);
            data = &data[4..];
            continue;
        }
        if size < 4 || size > data.len() {
            bail!(GitHttpError::Protocol);
        }
        lines.push(Some(&data[4..size]));
        data = &data[size..];
    }
    Ok(lines)
}
fn line_text(line: &[u8]) -> Result<&str> {
    let line = line.split(|b| *b == 0).next().unwrap_or_default();
    Ok(std::str::from_utf8(line)
        .map_err(|_| GitHttpError::Protocol)?
        .trim_end_matches('\n'))
}
fn completed_lines(body: &[u8]) -> Result<Vec<Option<&[u8]>>> {
    let mut lines = pkt_lines(body)?;
    if lines.pop() != Some(None) {
        bail!(GitHttpError::Protocol);
    }
    Ok(lines)
}
fn advertised_lines(body: &[u8]) -> Result<Vec<Option<&[u8]>>> {
    let mut lines = completed_lines(body)?;
    if lines.first() == Some(&Some(&b"# service=git-receive-pack\n"[..])) {
        if lines.get(1) != Some(&None) {
            bail!(GitHttpError::Protocol);
        }
        lines.drain(..2);
    }
    if lines.is_empty() {
        bail!(GitHttpError::Protocol);
    }
    Ok(lines)
}
pub fn parse_advertisement(body: &[u8], refname: &str) -> Result<Advertisement> {
    let mut advertised = Advertisement {
        head: None,
        haves: Vec::new(),
    };
    for packet in advertised_lines(body)? {
        let text = line_text(packet.ok_or(GitHttpError::Protocol)?)?;
        let (oid, name) = text.split_once(' ').ok_or(GitHttpError::Protocol)?;
        if oid.len() != 40 || !oid.bytes().all(|b| b.is_ascii_hexdigit()) {
            bail!(GitHttpError::Protocol);
        }
        add_advertised_ref(&mut advertised, refname, oid, name)?;
    }
    Ok(advertised)
}
fn add_advertised_ref(
    advertised: &mut Advertisement,
    refname: &str,
    oid: &str,
    name: &str,
) -> Result<()> {
    if oid == ZERO {
        return Ok(());
    }
    if name == refname {
        if advertised.head.replace(oid.to_owned()).is_some() {
            bail!(GitHttpError::Protocol);
        }
    } else if name == ".have" {
        if advertised.haves.len() >= 256 {
            bail!(GitHttpError::Protocol);
        }
        advertised.haves.push(oid.to_owned());
    }
    Ok(())
}
pub fn parse_report(body: &[u8], refname: &str) -> Result<PushOutcome> {
    let lines = completed_lines(body)?;
    let [Some(unpack), Some(status)] = lines.as_slice() else {
        bail!(GitHttpError::Protocol);
    };
    if *unpack != b"unpack ok\n" {
        bail!(GitHttpError::Protocol);
    }
    let status = line_text(status)?;
    if status.strip_prefix("ok ") == Some(refname) {
        return Ok(PushOutcome::Accepted);
    }
    if status
        .strip_prefix("ng ")
        .is_some_and(|rest| rest.split(' ').next() == Some(refname))
    {
        return Ok(PushOutcome::Refused);
    }
    bail!(GitHttpError::Protocol)
}
