//! Live-session frame payload protocol (peer-owned, relay-opaque).
//!
//! The relay validates only the leading class byte and enforces capability
//! by class; everything inside the payload is a contract between RSpice
//! peers, defined here. The model is host-authoritative state sync at
//! document granularity: the host broadcasts whole documents (chunked under
//! the relay's frame cap), guests mirror them, and per-document write leases
//! decide who may edit what. Payloads carry a version byte so incompatible
//! builds fail loudly instead of corrupting a session.
//!
//! Envelope, identical for every class:
//! `[version u8][kind u8][header_len u32 BE][JSON header][body bytes]`.
//! Headers are JSON for tolerant evolution; document content stays raw bytes
//! so large documents never pay a base64 or escaping tax.
//!
//! Trust model: the relay authenticates every participant and enforces
//! write authority by frame class, but it does not stamp sender identity on
//! relayed frames — the `sender` fields here are peer-asserted, which every
//! participant of an approved, seat-holding session is trusted for. Frames
//! that fail to decode are dropped by the consumer, never fatal.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::cloud_account::{LiveFrame, LiveFrameClass, MAX_LIVE_FRAME_BYTES};

/// Wire version of every payload this build produces and accepts.
pub(crate) const LIVE_PROTOCOL_VERSION: u8 = 1;

/// Body bytes per document chunk, comfortably under the relay's frame cap
/// with headroom for the envelope and header.
pub(crate) const DOCUMENT_CHUNK_BYTES: usize = 512 * 1024;

/// Largest complete document a peer will send or reassemble.
pub(crate) const MAX_DOCUMENT_BYTES: usize = 64 * 1024 * 1024;

/// Total bytes of partial documents a reassembler holds before it refuses
/// more, bounding what a misbehaving peer can make this client buffer.
const REASSEMBLY_BUDGET_BYTES: usize = 64 * 1024 * 1024;

/// Concurrent partial documents a reassembler tracks.
const MAX_PARTIAL_DOCUMENTS: usize = 32;

/// The two identifiers a peer asserts on every message. Display names
/// resolve through the authenticated roster, never through frames.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub(crate) struct PeerIdentity {
    pub principal_id: uuid::Uuid,
    pub client_instance_id: uuid::Uuid,
}

/// One decoded live message, classified exactly as the relay classified the
/// frame that carried it.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum LiveMessage {
    Presence(PresencePayload),
    Cursor(CursorPayload),
    Document(DocumentMessage),
    RunRequest(RunRequestPayload),
    RunStatus(RunStatusMessage),
}

/// Liveness plus focus. Presence doubles as the connection keepalive, so
/// hosts and guests send it on a fixed cadence as well as on focus changes.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct PresencePayload {
    pub sender: PeerIdentity,
    /// The document the participant is looking at, when any.
    pub focused_doc: Option<String>,
}

/// A pointer or caret position inside one document.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "surface")]
pub(crate) enum CursorLocus {
    /// Schematic canvas position in sheet coordinates.
    Canvas { doc: String, x: f32, y: f32 },
    /// Netlist caret line (one-based, as rendered).
    Netlist { doc: String, line: u32 },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct CursorPayload {
    pub sender: PeerIdentity,
    /// `None` announces the cursor left every shared surface.
    pub locus: Option<CursorLocus>,
}

/// What an editor asks the host to run. A closed enum so unknown intents
/// from newer builds fail decoding instead of doing something unintended.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "intent")]
pub(crate) enum RunIntent {
    /// Run the host's active simulation set on the host's working copy.
    Simulate,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct RunRequestPayload {
    pub sender: PeerIdentity,
    pub run: RunIntent,
}

/// Host-only report of a run's life. Progress detail is the simulation
/// runner's own serializable delta, opaque to the protocol.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "phase")]
pub(crate) enum RunPhase {
    Started,
    Progress,
    Finished,
    Failed { message: String },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct RunStatusPayload {
    pub sender: PeerIdentity,
    /// Host-scoped monotonic run identity, so late guests can discard
    /// status for runs they never saw start.
    pub run_id: u64,
    #[serde(flatten)]
    pub phase: RunPhase,
    /// The runner's progress delta, present on `Progress` reports.
    pub progress: Option<serde_json::Value>,
}

/// Messages on the run-status class. Result data travels here — not on the
/// document class — because results are host-produced run artifacts, and
/// the relay already restricts this class to the host.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum RunStatusMessage {
    Status(RunStatusPayload),
    /// One chunk of a result dataset, using the same chunk contract as
    /// document replaces (`doc` names the result, e.g. `results/<run_id>`).
    ResultChunk { header: ChunkHeader, chunk: Vec<u8> },
}

/// Header of one document (or result) chunk. `revision` is monotonic per
/// sender and document: the host's revisions are authoritative document
/// states, a leaseholder's are proposals the host applies and rebroadcasts.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct ChunkHeader {
    pub sender: PeerIdentity,
    pub doc: String,
    pub revision: u64,
    /// Lowercase hex SHA-256 of the complete content, verified on
    /// reassembly and used to recognize echoes of one's own edits.
    pub digest: String,
    pub total_bytes: u64,
    pub chunk_index: u32,
    pub chunk_count: u32,
}

/// One document the host announces in its manifest.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub(crate) struct ManifestEntry {
    pub doc: String,
    /// App-defined content contract of the document bytes, e.g.
    /// `rspice.schematic-sheet.v1`; guests skip entries they cannot render.
    pub content_type: String,
    pub revision: u64,
    pub digest: String,
}

/// Messages on the document class: whole-document replaces plus the
/// host-arbitrated write-lease conversation.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum DocumentMessage {
    /// One chunk of a complete document state.
    Replace { header: ChunkHeader, chunk: Vec<u8> },
    /// An editor asks the host for the write lease on one document.
    LeaseRequest { doc: String, sender: PeerIdentity },
    /// The host names a document's current leaseholder.
    LeaseGrant { doc: String, holder: PeerIdentity },
    /// The host refuses a lease; the document stays with its holder.
    LeaseDeny {
        doc: String,
        requester: PeerIdentity,
        holder: PeerIdentity,
    },
    /// The lease returns to the host. Guests send it to give a lease back
    /// (`holder` = themselves, verified against the table); the host sends
    /// it to announce any release, voluntary or revoked.
    LeaseRelease { doc: String, holder: PeerIdentity },
    /// A guest asks the host to resend its manifest and the named
    /// documents (`None` = everything), e.g. after (re)attaching.
    SyncRequest {
        sender: PeerIdentity,
        docs: Option<Vec<String>>,
    },
    /// The host's document inventory. Documents absent from a fresh
    /// manifest no longer exist on the host.
    Manifest {
        sender: PeerIdentity,
        docs: Vec<ManifestEntry>,
    },
}

/// Why a payload was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LiveProtocolError {
    /// The peer runs an incompatible build; surface it, don't guess.
    UnsupportedVersion(u8),
    /// Truncated, unknown kind, or a header that does not parse.
    Malformed,
}

// -- envelope -----------------------------------------------------------------

const ENVELOPE_PREFIX_BYTES: usize = 1 + 1 + 4;

fn encode_envelope(class: LiveFrameClass, kind: u8, header: &[u8], body: &[u8]) -> LiveFrame {
    let mut payload = Vec::with_capacity(ENVELOPE_PREFIX_BYTES + header.len() + body.len());
    payload.push(LIVE_PROTOCOL_VERSION);
    payload.push(kind);
    payload.extend_from_slice(&u32::try_from(header.len()).expect("bounded header").to_be_bytes());
    payload.extend_from_slice(header);
    payload.extend_from_slice(body);
    LiveFrame { class, payload }
}

fn decode_envelope(payload: &[u8]) -> Result<(u8, &[u8], &[u8]), LiveProtocolError> {
    if payload.len() < ENVELOPE_PREFIX_BYTES {
        return Err(LiveProtocolError::Malformed);
    }
    if payload[0] != LIVE_PROTOCOL_VERSION {
        return Err(LiveProtocolError::UnsupportedVersion(payload[0]));
    }
    let kind = payload[1];
    let header_length = u32::from_be_bytes(
        payload[2..ENVELOPE_PREFIX_BYTES]
            .try_into()
            .expect("four length bytes"),
    ) as usize;
    let rest = &payload[ENVELOPE_PREFIX_BYTES..];
    if rest.len() < header_length {
        return Err(LiveProtocolError::Malformed);
    }
    let (header, body) = rest.split_at(header_length);
    Ok((kind, header, body))
}

fn json_frame<T: Serialize>(class: LiveFrameClass, kind: u8, header: &T) -> LiveFrame {
    let header = serde_json::to_vec(header).expect("protocol headers serialize");
    encode_envelope(class, kind, &header, &[])
}

fn parse_header<'de, T: Deserialize<'de>>(header: &'de [u8]) -> Result<T, LiveProtocolError> {
    serde_json::from_slice(header).map_err(|_| LiveProtocolError::Malformed)
}

// -- document-class kinds -------------------------------------------------------

const KIND_ONLY: u8 = 0;

const DOCUMENT_REPLACE: u8 = 0;
const DOCUMENT_LEASE_REQUEST: u8 = 1;
const DOCUMENT_LEASE_GRANT: u8 = 2;
const DOCUMENT_LEASE_DENY: u8 = 3;
const DOCUMENT_LEASE_RELEASE: u8 = 4;
const DOCUMENT_SYNC_REQUEST: u8 = 5;
const DOCUMENT_MANIFEST: u8 = 6;

const RUN_STATUS_REPORT: u8 = 0;
const RUN_STATUS_RESULT_CHUNK: u8 = 1;

#[derive(Deserialize, Serialize)]
struct LeaseRequestHeader {
    doc: String,
    sender: PeerIdentity,
}

#[derive(Deserialize, Serialize)]
struct LeaseGrantHeader {
    doc: String,
    holder: PeerIdentity,
}

#[derive(Deserialize, Serialize)]
struct LeaseDenyHeader {
    doc: String,
    requester: PeerIdentity,
    holder: PeerIdentity,
}

#[derive(Deserialize, Serialize)]
struct LeaseReleaseHeader {
    doc: String,
    holder: PeerIdentity,
}

#[derive(Deserialize, Serialize)]
struct SyncRequestHeader {
    sender: PeerIdentity,
    docs: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize)]
struct ManifestHeader {
    sender: PeerIdentity,
    docs: Vec<ManifestEntry>,
}

impl LiveMessage {
    /// Encode onto the relay frame whose class carries this message.
    pub(crate) fn encode(&self) -> LiveFrame {
        match self {
            Self::Presence(payload) => json_frame(LiveFrameClass::Presence, KIND_ONLY, payload),
            Self::Cursor(payload) => json_frame(LiveFrameClass::Cursor, KIND_ONLY, payload),
            Self::RunRequest(payload) => {
                json_frame(LiveFrameClass::RunRequest, KIND_ONLY, payload)
            }
            Self::RunStatus(RunStatusMessage::Status(payload)) => {
                json_frame(LiveFrameClass::RunStatus, RUN_STATUS_REPORT, payload)
            }
            Self::RunStatus(RunStatusMessage::ResultChunk { header, chunk }) => encode_envelope(
                LiveFrameClass::RunStatus,
                RUN_STATUS_RESULT_CHUNK,
                &serde_json::to_vec(header).expect("protocol headers serialize"),
                chunk,
            ),
            Self::Document(message) => message.encode(),
        }
    }

    /// Decode one relay frame. The frame's class is trusted (the relay
    /// enforced it); everything inside is validated here.
    pub(crate) fn decode(frame: &LiveFrame) -> Result<Self, LiveProtocolError> {
        let (kind, header, body) = decode_envelope(&frame.payload)?;
        match frame.class {
            LiveFrameClass::Presence if kind == KIND_ONLY => {
                Ok(Self::Presence(parse_header(header)?))
            }
            LiveFrameClass::Cursor if kind == KIND_ONLY => Ok(Self::Cursor(parse_header(header)?)),
            LiveFrameClass::RunRequest if kind == KIND_ONLY => {
                Ok(Self::RunRequest(parse_header(header)?))
            }
            LiveFrameClass::RunStatus if kind == RUN_STATUS_REPORT => Ok(Self::RunStatus(
                RunStatusMessage::Status(parse_header(header)?),
            )),
            LiveFrameClass::RunStatus if kind == RUN_STATUS_RESULT_CHUNK => {
                Ok(Self::RunStatus(RunStatusMessage::ResultChunk {
                    header: parse_header(header)?,
                    chunk: body.to_vec(),
                }))
            }
            LiveFrameClass::Document => Ok(Self::Document(DocumentMessage::decode_parts(
                kind, header, body,
            )?)),
            _ => Err(LiveProtocolError::Malformed),
        }
    }
}

impl DocumentMessage {
    fn encode(&self) -> LiveFrame {
        let class = LiveFrameClass::Document;
        match self {
            Self::Replace { header, chunk } => encode_envelope(
                class,
                DOCUMENT_REPLACE,
                &serde_json::to_vec(header).expect("protocol headers serialize"),
                chunk,
            ),
            Self::LeaseRequest { doc, sender } => json_frame(
                class,
                DOCUMENT_LEASE_REQUEST,
                &LeaseRequestHeader {
                    doc: doc.clone(),
                    sender: *sender,
                },
            ),
            Self::LeaseGrant { doc, holder } => json_frame(
                class,
                DOCUMENT_LEASE_GRANT,
                &LeaseGrantHeader {
                    doc: doc.clone(),
                    holder: *holder,
                },
            ),
            Self::LeaseDeny {
                doc,
                requester,
                holder,
            } => json_frame(
                class,
                DOCUMENT_LEASE_DENY,
                &LeaseDenyHeader {
                    doc: doc.clone(),
                    requester: *requester,
                    holder: *holder,
                },
            ),
            Self::LeaseRelease { doc, holder } => json_frame(
                class,
                DOCUMENT_LEASE_RELEASE,
                &LeaseReleaseHeader {
                    doc: doc.clone(),
                    holder: *holder,
                },
            ),
            Self::SyncRequest { sender, docs } => json_frame(
                class,
                DOCUMENT_SYNC_REQUEST,
                &SyncRequestHeader {
                    sender: *sender,
                    docs: docs.clone(),
                },
            ),
            Self::Manifest { sender, docs } => json_frame(
                class,
                DOCUMENT_MANIFEST,
                &ManifestHeader {
                    sender: *sender,
                    docs: docs.clone(),
                },
            ),
        }
    }

    fn decode_parts(kind: u8, header: &[u8], body: &[u8]) -> Result<Self, LiveProtocolError> {
        match kind {
            DOCUMENT_REPLACE => Ok(Self::Replace {
                header: parse_header(header)?,
                chunk: body.to_vec(),
            }),
            DOCUMENT_LEASE_REQUEST => {
                let parsed: LeaseRequestHeader = parse_header(header)?;
                Ok(Self::LeaseRequest {
                    doc: parsed.doc,
                    sender: parsed.sender,
                })
            }
            DOCUMENT_LEASE_GRANT => {
                let parsed: LeaseGrantHeader = parse_header(header)?;
                Ok(Self::LeaseGrant {
                    doc: parsed.doc,
                    holder: parsed.holder,
                })
            }
            DOCUMENT_LEASE_DENY => {
                let parsed: LeaseDenyHeader = parse_header(header)?;
                Ok(Self::LeaseDeny {
                    doc: parsed.doc,
                    requester: parsed.requester,
                    holder: parsed.holder,
                })
            }
            DOCUMENT_LEASE_RELEASE => {
                let parsed: LeaseReleaseHeader = parse_header(header)?;
                Ok(Self::LeaseRelease {
                    doc: parsed.doc,
                    holder: parsed.holder,
                })
            }
            DOCUMENT_SYNC_REQUEST => {
                let parsed: SyncRequestHeader = parse_header(header)?;
                Ok(Self::SyncRequest {
                    sender: parsed.sender,
                    docs: parsed.docs,
                })
            }
            DOCUMENT_MANIFEST => {
                let parsed: ManifestHeader = parse_header(header)?;
                Ok(Self::Manifest {
                    sender: parsed.sender,
                    docs: parsed.docs,
                })
            }
            _ => Err(LiveProtocolError::Malformed),
        }
    }
}

// -- chunking -------------------------------------------------------------------

/// Lowercase hex SHA-256 of complete content, the digest form every chunk
/// header carries.
pub(crate) fn content_digest(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut text = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write;
        write!(text, "{byte:02x}").expect("writing to a string");
    }
    text
}

/// Split one complete content into chunk headers and bodies. Returns `None`
/// when the content exceeds what peers reassemble.
fn chunk_parts(
    sender: PeerIdentity,
    doc: &str,
    revision: u64,
    bytes: &[u8],
) -> Option<Vec<(ChunkHeader, Vec<u8>)>> {
    if bytes.len() > MAX_DOCUMENT_BYTES {
        return None;
    }
    let digest = content_digest(bytes);
    let chunk_count = bytes.len().div_ceil(DOCUMENT_CHUNK_BYTES).max(1);
    let mut parts = Vec::with_capacity(chunk_count);
    for index in 0..chunk_count {
        let start = index * DOCUMENT_CHUNK_BYTES;
        let end = (start + DOCUMENT_CHUNK_BYTES).min(bytes.len());
        parts.push((
            ChunkHeader {
                sender,
                doc: doc.to_owned(),
                revision,
                digest: digest.clone(),
                total_bytes: bytes.len() as u64,
                chunk_index: u32::try_from(index).expect("bounded by MAX_DOCUMENT_BYTES"),
                chunk_count: u32::try_from(chunk_count).expect("bounded by MAX_DOCUMENT_BYTES"),
            },
            bytes[start..end].to_vec(),
        ));
    }
    Some(parts)
}

/// A complete document state as document-class replace messages.
pub(crate) fn replace_messages(
    sender: PeerIdentity,
    doc: &str,
    revision: u64,
    bytes: &[u8],
) -> Option<Vec<DocumentMessage>> {
    Some(
        chunk_parts(sender, doc, revision, bytes)?
            .into_iter()
            .map(|(header, chunk)| DocumentMessage::Replace { header, chunk })
            .collect(),
    )
}

/// A complete result dataset as run-status-class chunk messages.
pub(crate) fn result_messages(
    sender: PeerIdentity,
    doc: &str,
    revision: u64,
    bytes: &[u8],
) -> Option<Vec<RunStatusMessage>> {
    Some(
        chunk_parts(sender, doc, revision, bytes)?
            .into_iter()
            .map(|(header, chunk)| RunStatusMessage::ResultChunk { header, chunk })
            .collect(),
    )
}

/// Why a chunk was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReassemblyError {
    /// The header claims more than [`MAX_DOCUMENT_BYTES`] or an impossible
    /// chunk layout.
    Inconsistent,
    /// The finished content did not match its declared digest or size.
    DigestMismatch,
    /// Accepting the chunk would exceed the reassembly memory budget.
    BudgetExceeded,
}

/// One finished reassembly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CompletedContent {
    pub sender: PeerIdentity,
    pub doc: String,
    pub revision: u64,
    pub digest: String,
    pub bytes: Vec<u8>,
}

struct PartialContent {
    revision: u64,
    digest: String,
    total_bytes: u64,
    chunk_count: u32,
    chunks: Vec<Option<Vec<u8>>>,
    buffered: usize,
}

/// Reassembles chunked content per sender and document. A newer revision
/// from the same sender evicts its predecessor mid-flight, so an
/// interrupted broadcast never wedges a document.
#[derive(Default)]
pub(crate) struct ContentReassembler {
    partials: HashMap<(uuid::Uuid, String), PartialContent>,
    buffered_bytes: usize,
}

impl ContentReassembler {
    /// Feed one chunk; a completed, digest-verified content comes back on
    /// the chunk that finishes it.
    pub(crate) fn accept(
        &mut self,
        header: &ChunkHeader,
        chunk: &[u8],
    ) -> Result<Option<CompletedContent>, ReassemblyError> {
        let claimed_total = usize::try_from(header.total_bytes)
            .ok()
            .filter(|total| *total <= MAX_DOCUMENT_BYTES)
            .ok_or(ReassemblyError::Inconsistent)?;
        let expected_chunks = claimed_total.div_ceil(DOCUMENT_CHUNK_BYTES).max(1);
        if header.chunk_count as usize != expected_chunks
            || header.chunk_index >= header.chunk_count
            || chunk.len() > MAX_LIVE_FRAME_BYTES
        {
            return Err(ReassemblyError::Inconsistent);
        }

        let key = (header.sender.client_instance_id, header.doc.clone());
        match self.partials.get(&key) {
            Some(partial) if partial.revision > header.revision => return Ok(None),
            Some(partial)
                if partial.revision < header.revision
                    || partial.digest != header.digest
                    || partial.total_bytes != header.total_bytes =>
            {
                self.evict(&key);
            }
            Some(_) | None => {}
        }
        if !self.partials.contains_key(&key) {
            if self.partials.len() >= MAX_PARTIAL_DOCUMENTS {
                return Err(ReassemblyError::BudgetExceeded);
            }
            self.partials.insert(
                key.clone(),
                PartialContent {
                    revision: header.revision,
                    digest: header.digest.clone(),
                    total_bytes: header.total_bytes,
                    chunk_count: header.chunk_count,
                    chunks: vec![None; header.chunk_count as usize],
                    buffered: 0,
                },
            );
        }

        if self.buffered_bytes + chunk.len() > REASSEMBLY_BUDGET_BYTES {
            self.evict(&key);
            return Err(ReassemblyError::BudgetExceeded);
        }
        let partial = self.partials.get_mut(&key).expect("inserted above");
        let slot = &mut partial.chunks[header.chunk_index as usize];
        if let Some(existing) = slot.take() {
            partial.buffered -= existing.len();
            self.buffered_bytes -= existing.len();
        }
        partial.buffered += chunk.len();
        self.buffered_bytes += chunk.len();
        *slot = Some(chunk.to_vec());

        if partial.chunks.iter().any(Option::is_none) {
            return Ok(None);
        }
        let finished = self.partials.remove(&key).expect("present");
        self.buffered_bytes -= finished.buffered;
        let mut bytes = Vec::with_capacity(finished.buffered);
        for piece in finished.chunks.into_iter().flatten() {
            bytes.extend_from_slice(&piece);
        }
        if bytes.len() as u64 != finished.total_bytes || content_digest(&bytes) != finished.digest
        {
            return Err(ReassemblyError::DigestMismatch);
        }
        let _ = finished.chunk_count;
        Ok(Some(CompletedContent {
            sender: header.sender,
            doc: key.1,
            revision: finished.revision,
            digest: finished.digest,
            bytes,
        }))
    }

    /// Drop every partial from one client instance (it left or reconnected).
    pub(crate) fn forget_instance(&mut self, client_instance_id: uuid::Uuid) {
        let keys: Vec<_> = self
            .partials
            .keys()
            .filter(|(instance, _)| *instance == client_instance_id)
            .cloned()
            .collect();
        for key in keys {
            self.evict(&key);
        }
    }

    fn evict(&mut self, key: &(uuid::Uuid, String)) {
        if let Some(partial) = self.partials.remove(key) {
            self.buffered_bytes -= partial.buffered;
        }
    }
}

// -- write leases ---------------------------------------------------------------

/// The host's ruling on a lease request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LeaseDecision {
    /// The requester holds the lease now (or already did).
    Granted,
    /// Another participant holds it; the request is refused.
    Held(PeerIdentity),
}

/// Who may write which document. The host owns the authoritative table and
/// arbitrates requests; guests mirror it from grant and release messages.
/// Documents without a holder are the host's to edit.
#[derive(Default)]
pub(crate) struct LeaseTable {
    held: HashMap<String, PeerIdentity>,
}

impl LeaseTable {
    pub(crate) fn holder(&self, doc: &str) -> Option<PeerIdentity> {
        self.held.get(doc).copied()
    }

    /// Host-side arbitration: first holder wins, re-requests by the current
    /// holder stay granted.
    pub(crate) fn arbitrate(&mut self, doc: &str, requester: PeerIdentity) -> LeaseDecision {
        match self.held.get(doc) {
            Some(holder) if holder.client_instance_id != requester.client_instance_id => {
                LeaseDecision::Held(*holder)
            }
            _ => {
                self.held.insert(doc.to_owned(), requester);
                LeaseDecision::Granted
            }
        }
    }

    /// Release by its holder. Anyone else's release is ignored, so a stale
    /// or malicious release cannot free someone else's lease.
    pub(crate) fn release(&mut self, doc: &str, holder: PeerIdentity) -> bool {
        match self.held.get(doc) {
            Some(current) if current.client_instance_id == holder.client_instance_id => {
                self.held.remove(doc);
                true
            }
            _ => false,
        }
    }

    /// Free every lease a participant held (removed, left, or dropped).
    /// Returns the freed documents so the host can announce the releases.
    pub(crate) fn revoke_principal(&mut self, principal_id: uuid::Uuid) -> Vec<String> {
        let mut freed: Vec<String> = self
            .held
            .iter()
            .filter(|(_, holder)| holder.principal_id == principal_id)
            .map(|(doc, _)| doc.clone())
            .collect();
        freed.sort();
        for doc in &freed {
            self.held.remove(doc);
        }
        freed
    }

    /// Guest-side mirror of the host's announcements.
    pub(crate) fn apply_grant(&mut self, doc: &str, holder: PeerIdentity) {
        self.held.insert(doc.to_owned(), holder);
    }

    /// Guest-side mirror of a release announcement.
    pub(crate) fn apply_release(&mut self, doc: &str) {
        self.held.remove(doc);
    }

    pub(crate) fn clear(&mut self) {
        self.held.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn peer(seed: u128) -> PeerIdentity {
        PeerIdentity {
            principal_id: uuid::Uuid::from_u128(seed),
            client_instance_id: uuid::Uuid::from_u128(seed + 1000),
        }
    }

    #[test]
    fn every_message_round_trips_through_its_frame() {
        let sender = peer(1);
        let messages = vec![
            LiveMessage::Presence(PresencePayload {
                sender,
                focused_doc: Some("sheet/abc".to_owned()),
            }),
            LiveMessage::Cursor(CursorPayload {
                sender,
                locus: Some(CursorLocus::Canvas {
                    doc: "sheet/abc".to_owned(),
                    x: 12.5,
                    y: -3.0,
                }),
            }),
            LiveMessage::Cursor(CursorPayload {
                sender,
                locus: Some(CursorLocus::Netlist {
                    doc: "netlist/main".to_owned(),
                    line: 42,
                }),
            }),
            LiveMessage::RunRequest(RunRequestPayload {
                sender,
                run: RunIntent::Simulate,
            }),
            LiveMessage::RunStatus(RunStatusMessage::Status(RunStatusPayload {
                sender,
                run_id: 7,
                phase: RunPhase::Progress,
                progress: Some(serde_json::json!({"points": 128})),
            })),
            LiveMessage::RunStatus(RunStatusMessage::Status(RunStatusPayload {
                sender,
                run_id: 7,
                phase: RunPhase::Failed {
                    message: "convergence".to_owned(),
                },
                progress: None,
            })),
            LiveMessage::Document(DocumentMessage::LeaseRequest {
                doc: "sheet/abc".to_owned(),
                sender,
            }),
            LiveMessage::Document(DocumentMessage::LeaseGrant {
                doc: "sheet/abc".to_owned(),
                holder: sender,
            }),
            LiveMessage::Document(DocumentMessage::LeaseDeny {
                doc: "sheet/abc".to_owned(),
                requester: peer(2),
                holder: sender,
            }),
            LiveMessage::Document(DocumentMessage::LeaseRelease {
                doc: "sheet/abc".to_owned(),
                holder: sender,
            }),
            LiveMessage::Document(DocumentMessage::SyncRequest {
                sender,
                docs: None,
            }),
            LiveMessage::Document(DocumentMessage::Manifest {
                sender,
                docs: vec![ManifestEntry {
                    doc: "sheet/abc".to_owned(),
                    content_type: "rspice.schematic-sheet.v1".to_owned(),
                    revision: 3,
                    digest: content_digest(b"content"),
                }],
            }),
        ];
        for message in messages {
            let frame = message.encode();
            assert_eq!(
                LiveMessage::decode(&frame).expect("round trip"),
                message,
                "class {:?}",
                frame.class
            );
        }
    }

    #[test]
    fn replace_and_result_chunks_round_trip_with_bodies() {
        let sender = peer(3);
        let content = vec![7u8; DOCUMENT_CHUNK_BYTES + 17];
        let replaces =
            replace_messages(sender, "sheet/abc", 5, &content).expect("bounded content");
        assert_eq!(replaces.len(), 2);
        for message in replaces {
            let frame = LiveMessage::Document(message.clone()).encode();
            assert_eq!(frame.class, LiveFrameClass::Document);
            assert_eq!(
                LiveMessage::decode(&frame).expect("round trip"),
                LiveMessage::Document(message)
            );
        }
        let results = result_messages(sender, "results/7", 1, b"dataset").expect("bounded");
        assert_eq!(results.len(), 1);
        let frame = LiveMessage::RunStatus(results[0].clone()).encode();
        assert_eq!(frame.class, LiveFrameClass::RunStatus);
        assert_eq!(
            LiveMessage::decode(&frame).expect("round trip"),
            LiveMessage::RunStatus(results[0].clone())
        );
    }

    #[test]
    fn decoding_rejects_foreign_versions_kinds_and_truncation() {
        let sender = peer(4);
        let mut frame = LiveMessage::Presence(PresencePayload {
            sender,
            focused_doc: None,
        })
        .encode();
        frame.payload[0] = 2;
        assert_eq!(
            LiveMessage::decode(&frame),
            Err(LiveProtocolError::UnsupportedVersion(2))
        );

        let mut wrong_kind = LiveMessage::Presence(PresencePayload {
            sender,
            focused_doc: None,
        })
        .encode();
        wrong_kind.payload[1] = 9;
        assert_eq!(
            LiveMessage::decode(&wrong_kind),
            Err(LiveProtocolError::Malformed)
        );

        assert_eq!(
            LiveMessage::decode(&LiveFrame {
                class: LiveFrameClass::Presence,
                payload: vec![LIVE_PROTOCOL_VERSION, 0],
            }),
            Err(LiveProtocolError::Malformed),
            "truncated envelope"
        );

        let mut truncated_header = LiveMessage::Cursor(CursorPayload {
            sender,
            locus: None,
        })
        .encode();
        truncated_header.payload.truncate(8);
        assert_eq!(
            LiveMessage::decode(&truncated_header),
            Err(LiveProtocolError::Malformed)
        );

        // A document kind on a class the relay would never carry it on.
        let misclassed = LiveFrame {
            class: LiveFrameClass::Cursor,
            payload: LiveMessage::Document(DocumentMessage::LeaseRelease {
                doc: "sheet/abc".to_owned(),
                holder: sender,
            })
            .encode()
            .payload,
        };
        assert_eq!(
            LiveMessage::decode(&misclassed),
            Err(LiveProtocolError::Malformed)
        );
    }

    #[test]
    fn reassembly_completes_verifies_and_orders_chunks() {
        let sender = peer(5);
        let mut content = Vec::new();
        for index in 0..(DOCUMENT_CHUNK_BYTES * 2 + 300) {
            content.push((index % 251) as u8);
        }
        let parts = chunk_parts(sender, "sheet/abc", 9, &content).expect("bounded");
        assert_eq!(parts.len(), 3);

        let mut reassembler = ContentReassembler::default();
        // Deliver out of order: 2, 0, 1.
        assert_eq!(
            reassembler
                .accept(&parts[2].0, &parts[2].1)
                .expect("accepted"),
            None
        );
        assert_eq!(
            reassembler
                .accept(&parts[0].0, &parts[0].1)
                .expect("accepted"),
            None
        );
        let completed = reassembler
            .accept(&parts[1].0, &parts[1].1)
            .expect("accepted")
            .expect("finished");
        assert_eq!(completed.bytes, content);
        assert_eq!(completed.revision, 9);
        assert_eq!(completed.doc, "sheet/abc");
        assert_eq!(completed.digest, content_digest(&content));
        assert_eq!(reassembler.buffered_bytes, 0, "budget fully returned");
    }

    #[test]
    fn reassembly_evicts_stale_revisions_and_rejects_corruption() {
        let sender = peer(6);
        let old = vec![1u8; DOCUMENT_CHUNK_BYTES + 1];
        let new = vec![2u8; DOCUMENT_CHUNK_BYTES + 1];
        let old_parts = chunk_parts(sender, "sheet/abc", 1, &old).expect("bounded");
        let new_parts = chunk_parts(sender, "sheet/abc", 2, &new).expect("bounded");

        let mut reassembler = ContentReassembler::default();
        assert_eq!(
            reassembler
                .accept(&old_parts[0].0, &old_parts[0].1)
                .expect("accepted"),
            None
        );
        // The newer revision supersedes the unfinished older one.
        assert_eq!(
            reassembler
                .accept(&new_parts[0].0, &new_parts[0].1)
                .expect("accepted"),
            None
        );
        // A late chunk of the superseded revision is ignored, not fatal.
        assert_eq!(
            reassembler
                .accept(&old_parts[1].0, &old_parts[1].1)
                .expect("stale chunk tolerated"),
            None
        );
        let completed = reassembler
            .accept(&new_parts[1].0, &new_parts[1].1)
            .expect("accepted")
            .expect("finished");
        assert_eq!(completed.bytes, new);

        // Corrupted body: right layout, wrong digest.
        let mut corrupted = ContentReassembler::default();
        let parts = chunk_parts(sender, "sheet/abc", 3, b"expected").expect("bounded");
        let mut wrong = parts[0].1.clone();
        wrong[0] ^= 0xff;
        assert_eq!(
            corrupted.accept(&parts[0].0, &wrong),
            Err(ReassemblyError::DigestMismatch)
        );

        // Inconsistent claims are refused outright.
        let mut header = parts[0].0.clone();
        header.total_bytes = (MAX_DOCUMENT_BYTES as u64) + 1;
        assert_eq!(
            ContentReassembler::default().accept(&header, b"x"),
            Err(ReassemblyError::Inconsistent)
        );
        let mut layout = parts[0].0.clone();
        layout.chunk_count = 4;
        assert_eq!(
            ContentReassembler::default().accept(&layout, b"x"),
            Err(ReassemblyError::Inconsistent)
        );
    }

    #[test]
    fn reassembly_forgets_instances_and_returns_their_budget() {
        let sender = peer(7);
        let content = vec![9u8; DOCUMENT_CHUNK_BYTES + 1];
        let parts = chunk_parts(sender, "sheet/abc", 1, &content).expect("bounded");
        let mut reassembler = ContentReassembler::default();
        assert_eq!(
            reassembler
                .accept(&parts[0].0, &parts[0].1)
                .expect("accepted"),
            None
        );
        assert!(reassembler.buffered_bytes > 0);
        reassembler.forget_instance(sender.client_instance_id);
        assert_eq!(reassembler.buffered_bytes, 0);
        assert!(reassembler.partials.is_empty());
    }

    #[test]
    fn lease_table_arbitrates_releases_and_revokes() {
        let editor = peer(8);
        let rival = peer(9);
        let mut table = LeaseTable::default();

        assert_eq!(table.arbitrate("sheet/a", editor), LeaseDecision::Granted);
        assert_eq!(
            table.arbitrate("sheet/a", editor),
            LeaseDecision::Granted,
            "re-request by the holder stays granted"
        );
        assert_eq!(
            table.arbitrate("sheet/a", rival),
            LeaseDecision::Held(editor)
        );
        assert_eq!(table.holder("sheet/a"), Some(editor));

        assert!(!table.release("sheet/a", rival), "only the holder releases");
        assert!(table.release("sheet/a", editor));
        assert_eq!(table.holder("sheet/a"), None);
        assert_eq!(table.arbitrate("sheet/a", rival), LeaseDecision::Granted);

        assert_eq!(table.arbitrate("sheet/b", rival), LeaseDecision::Granted);
        assert_eq!(
            table.revoke_principal(rival.principal_id),
            vec!["sheet/a".to_owned(), "sheet/b".to_owned()]
        );
        assert_eq!(table.holder("sheet/a"), None);

        let mut mirror = LeaseTable::default();
        mirror.apply_grant("sheet/c", editor);
        assert_eq!(mirror.holder("sheet/c"), Some(editor));
        mirror.apply_release("sheet/c");
        assert_eq!(mirror.holder("sheet/c"), None);
        mirror.apply_grant("sheet/c", editor);
        mirror.clear();
        assert_eq!(mirror.holder("sheet/c"), None);
    }
}
