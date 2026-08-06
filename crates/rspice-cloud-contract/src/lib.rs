#![forbid(unsafe_code)]

//! Transport- and runtime-independent RSpice Cloud HTTP contract.
//!
//! The API serializes these types and every Rust client deserializes the same
//! types. This crate deliberately contains no HTTP client, database, async
//! runtime, platform bindings, credentials, or secret storage.

use std::{
    collections::{BTreeMap, HashSet},
    fmt,
};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
pub use rspice_cloud_domain::{
    CURRENT_SIMULATION_EXECUTION_MANIFEST_VERSION, CURRENT_SIMULATION_REQUEST_DIGEST_VERSION,
    CircuitVisibility, EntitlementStatus, LEGACY_SIMULATION_EXECUTION_MANIFEST_VERSION,
    MAX_SIMULATION_ATTEMPTS, MAX_SIMULATION_EXECUTION_ARTIFACT_BYTES,
    MAX_SIMULATION_EXECUTION_ARTIFACTS, MAX_SIMULATION_EXECUTION_MANIFEST_BYTES, SharePermission,
    SimulationExecutionArtifact, SimulationExecutionEngine, SimulationExecutionManifest,
    SimulationExecutionRequest, SimulationExecutionRevision, SimulationExecutionRuntimeMode,
    SimulationRunStatus, VERIFIED_ADAPTER_SIMULATION_EXECUTION_MANIFEST_VERSION, WorkspaceRole,
    is_valid_simulation_execution_manifest,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
pub use uuid::Uuid;

pub const API_VERSION: &str = "v1";
pub const SERVICE_NAME: &str = "rspice-cloud-api";
/// Selected WebSocket subprotocol for compatible RSpice Automerge sessions.
pub const COLLABORATION_PROTOCOL: &str = "rspice.automerge.v1";
/// Prefix for the client-offered, never-selected collaboration ticket protocol.
pub const COLLABORATION_TICKET_PROTOCOL_PREFIX: &str = "rspice.ticket.";
/// Selected WebSocket subprotocol for ephemeral live-session relays (ADR 0082).
pub const LIVE_SESSION_PROTOCOL: &str = "rspice.live-session.v1";

/// Class byte prefixing every live-session relay frame. The relay enforces
/// participant capability by class alone and never interprets payload bytes:
/// viewers send presence and cursor frames, editors add document and
/// run-request frames, and run-status frames are host-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LiveSessionFrameClass {
    Presence = 0,
    Cursor = 1,
    Document = 2,
    RunRequest = 3,
    RunStatus = 4,
}

impl LiveSessionFrameClass {
    /// Decode a frame's leading class byte.
    #[must_use]
    pub const fn from_byte(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Presence),
            1 => Some(Self::Cursor),
            2 => Some(Self::Document),
            3 => Some(Self::RunRequest),
            4 => Some(Self::RunStatus),
            _ => None,
        }
    }
}
/// Random bytes represented by every RSpice capability token.
pub const CAPABILITY_TOKEN_BYTES: usize = 32;
/// Canonical unpadded base64url length of a 256-bit capability token.
pub const CAPABILITY_TOKEN_LENGTH: usize = 43;
const CAPABILITY_TOKEN_FINAL_CHARACTERS: &[u8; 16] = b"AEIMQUYcgkosw048";
/// Minimum RSA modulus accepted for PS256 native-license signing keys.
pub const MIN_LICENSE_RSA_MODULUS_BITS: usize = 2048;
/// Maximum RSA modulus accepted by the bounded native-license wire contract.
pub const MAX_LICENSE_RSA_MODULUS_BITS: usize = 8192;
/// Required RSA public exponent encoded as a canonical JWK Base64urlUInt.
pub const LICENSE_RSA_PUBLIC_EXPONENT: &str = "AQAB";

/// Returns whether a value is the exact canonical unpadded base64url
/// representation of a 256-bit RSpice capability token.
#[must_use]
pub fn is_canonical_capability_token(value: &str) -> bool {
    value.len() == CAPABILITY_TOKEN_LENGTH
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        && CAPABILITY_TOKEN_FINAL_CHARACTERS
            .contains(&value.as_bytes()[CAPABILITY_TOKEN_LENGTH - 1])
}

/// Returns whether a public native-license JWKS is bounded, canonical, and
/// suitable for the service's pinned PS256 policy.
///
/// This validates public-key representation and strength only. Native clients
/// must still cryptographically verify every lease token and pin the expected
/// issuer and audience before granting authority.
#[must_use]
pub fn is_valid_license_jwk_set(value: &LicenseJwkSet) -> bool {
    (1..=16).contains(&value.keys.len())
        && value.keys.iter().all(is_valid_license_jwk)
        && value
            .keys
            .iter()
            .map(|key| key.kid.as_str())
            .collect::<HashSet<_>>()
            .len()
            == value.keys.len()
}

/// Returns whether one native-license RSA JWK uses the product's canonical
/// identifier, modulus, and public-exponent policy.
#[must_use]
pub fn is_valid_license_jwk(value: &LicenseJwk) -> bool {
    is_valid_license_key_id(&value.kid) && is_valid_license_rsa_public_key(&value.n, &value.e)
}

/// Returns whether a native-license signing key ID is canonical on the wire.
#[must_use]
pub fn is_valid_license_key_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

/// Returns whether RSA public parameters are canonical Base64urlUInt values
/// with a 2048-8192-bit modulus and the fixed exponent 65537 (`AQAB`).
#[must_use]
pub fn is_valid_license_rsa_public_key(modulus: &str, exponent: &str) -> bool {
    let Some(modulus) = decode_base64url_uint(modulus, MAX_LICENSE_RSA_MODULUS_BITS / 8) else {
        return false;
    };
    let modulus_bits = modulus
        .len()
        .checked_mul(8)
        .and_then(|bits| bits.checked_sub(modulus[0].leading_zeros() as usize));
    modulus_bits.is_some_and(|bits| {
        (MIN_LICENSE_RSA_MODULUS_BITS..=MAX_LICENSE_RSA_MODULUS_BITS).contains(&bits)
    }) && exponent == LICENSE_RSA_PUBLIC_EXPONENT
        && decode_base64url_uint(exponent, 8).as_deref() == Some([1_u8, 0, 1].as_slice())
}

fn decode_base64url_uint(value: &str, maximum_bytes: usize) -> Option<Vec<u8>> {
    if value.is_empty() || value.len() > maximum_bytes.saturating_mul(4).div_ceil(3) {
        return None;
    }
    let decoded = URL_SAFE_NO_PAD.decode(value).ok()?;
    (!decoded.is_empty()
        && decoded.len() <= maximum_bytes
        && (decoded.len() == 1 || decoded[0] != 0)
        && URL_SAFE_NO_PAD.encode(&decoded) == value)
        .then_some(decoded)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BuildInfo {
    pub service: String,
    pub version: String,
    pub source_sha: String,
    pub api_version: String,
}

impl Default for BuildInfo {
    fn default() -> Self {
        Self {
            service: SERVICE_NAME.to_owned(),
            version: env!("CARGO_PKG_VERSION").to_owned(),
            source_sha: option_env!("RSPICE_BUILD_SHA")
                .unwrap_or("development")
                .to_owned(),
            api_version: API_VERSION.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub kind: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    pub instance: String,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct CurrentPrincipal {
    pub id: Uuid,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Entitlement {
    pub id: Uuid,
    pub principal_id: Option<Uuid>,
    pub workspace_id: Option<Uuid>,
    pub product: String,
    pub plan: String,
    pub status: EntitlementStatus,
    pub features: Value,
    pub valid_from: String,
    pub valid_until: Option<String>,
    /// Immutable creation time used by the newest-first pagination contract.
    pub created_at: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LicenseJwkSet {
    pub keys: Vec<LicenseJwk>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LicenseJwk {
    pub kty: LicenseKeyType,
    pub n: String,
    pub e: String,
    pub kid: String,
    #[serde(rename = "use")]
    pub key_use: LicenseKeyUse,
    pub alg: LicenseKeyAlgorithm,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LicenseKeyType {
    #[serde(rename = "RSA")]
    Rsa,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LicenseKeyUse {
    #[serde(rename = "sig")]
    Signature,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LicenseKeyAlgorithm {
    #[serde(rename = "PS256")]
    Ps256,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateWorkspaceRequest {
    pub slug: String,
    pub name: String,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateWorkspaceRequest {
    pub expected_row_version: i64,
    pub name: String,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Workspace {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub role: WorkspaceRole,
    pub row_version: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// Immutable administrator-visible evidence for one workspace mutation.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct AuditEvent {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub actor_principal_id: Option<Uuid>,
    pub action: String,
    pub target_type: String,
    pub target_id: Option<Uuid>,
    pub metadata: Value,
    pub occurred_at: String,
}

/// One principal's current membership in a workspace.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorkspaceMember {
    pub workspace_id: Uuid,
    pub principal_id: Uuid,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub role: WorkspaceRole,
    pub joined_at: String,
}

/// Adds an existing principal or changes one workspace member's role.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateWorkspaceMemberRequest {
    pub role: WorkspaceRole,
}

/// Creates a circuit and its initial immutable revision.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateCircuitRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<CircuitVisibility>,
    pub schema_version: u32,
    pub document: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_ids: Option<Vec<Uuid>>,
}

/// Applies an optimistic-concurrency update to mutable circuit metadata.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateCircuitRequest {
    pub expected_row_version: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<CircuitVisibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
}

/// Creates an immutable successor revision under optimistic concurrency.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateCircuitRevisionRequest {
    pub expected_row_version: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_revision_id: Option<Uuid>,
    pub schema_version: u32,
    pub document: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_ids: Option<Vec<Uuid>>,
}

/// Mutable circuit metadata and its current immutable head revision.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct Circuit {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub title: String,
    pub visibility: CircuitVisibility,
    pub head_revision_id: Option<Uuid>,
    pub row_version: i64,
    pub archived_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// One sealed, content-addressed circuit revision.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct CircuitRevision {
    pub id: Uuid,
    pub parent_revision_id: Option<Uuid>,
    pub schema_version: i32,
    pub content_digest_version: i16,
    pub document: Value,
    pub artifact_ids: Vec<Uuid>,
    pub content_sha256: String,
    pub created_at: String,
}

/// Requests a one-time live-collaboration ticket for one client instance.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateCollaborationTicketRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_instance_id: Option<Uuid>,
}

/// Durable live-document position associated with an issued ticket.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct CollaborationDocument {
    pub base_revision_id: Uuid,
    pub schema_version: i32,
    pub latest_sequence: i64,
}

/// Preferred request-target-safe live-collaboration ticket handoff.
///
/// `ticket_protocol` is a short-lived bearer credential for the WebSocket
/// protocol offer. It must never be selected, echoed, persisted, or logged.
#[derive(Deserialize, Eq, PartialEq, Serialize)]
pub struct CollaborationTicketProtocol {
    pub protocol: String,
    pub websocket_endpoint: String,
    pub ticket_protocol: String,
    pub client_instance_id: Uuid,
    pub expires_at: String,
    pub document: CollaborationDocument,
}

/// A live session's guest capability vocabulary (ADR 0082).
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveSessionCapability {
    Edit,
    View,
}

/// A participant's admission state in a live session.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveSessionAdmission {
    Pending,
    Admitted,
    Removed,
}

/// Host-declared policy for one live session, fixed at creation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveSessionPolicy {
    pub default_capability: LiveSessionCapability,
    pub approve_joins: bool,
    pub allow_save_copy: bool,
}

/// Creation request for a live session. The optional circuit reference is
/// provenance only; a session over a local project omits it.
#[derive(Deserialize, Eq, PartialEq, Serialize)]
pub struct CreateLiveSessionRequest {
    pub policy: LiveSessionPolicy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circuit_id: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_instance_id: Option<Uuid>,
}

/// Creation response. `join_code` is returned exactly once and never again;
/// persistence stores only its digest.
#[derive(Deserialize, Eq, PartialEq, Serialize)]
pub struct CreatedLiveSession {
    pub session: LiveSession,
    pub join_code: String,
    pub ticket: LiveSessionTicketProtocol,
}

/// One participant row as the roster reports it.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveSessionParticipant {
    pub principal_id: Uuid,
    pub display_name: String,
    pub capability: LiveSessionCapability,
    pub admission: LiveSessionAdmission,
    pub joined_at: String,
}

/// A live session as its host and participants see it.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveSession {
    pub id: Uuid,
    pub host_principal_id: Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circuit_id: Option<Uuid>,
    pub policy: LiveSessionPolicy,
    pub created_at: String,
    pub participants: Vec<LiveSessionParticipant>,
}

/// Join request: the human-relayable code, exactly as displayed, plus an
/// optional stable client instance identity.
#[derive(Deserialize, Eq, PartialEq, Serialize)]
pub struct JoinLiveSessionRequest {
    pub join_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_instance_id: Option<Uuid>,
}

/// Join response. A pending admission carries no ticket; the caller polls
/// the session until the host admits them and then joins again.
#[derive(Deserialize, Eq, PartialEq, Serialize)]
pub struct JoinedLiveSession {
    pub session: LiveSession,
    pub admission: LiveSessionAdmission,
    pub capability: LiveSessionCapability,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticket: Option<LiveSessionTicketProtocol>,
}

/// Connect credential for the live-session relay.
///
/// `ticket_protocol` is a short-lived bearer credential for the WebSocket
/// protocol offer. It must never be selected, echoed, persisted, or logged.
#[derive(Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveSessionTicketProtocol {
    pub protocol: String,
    pub websocket_endpoint: String,
    pub ticket_protocol: String,
    pub client_instance_id: Uuid,
    pub expires_at: String,
}

/// Artifact kinds visible through the customer control plane.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactKind {
    /// File attached to an immutable circuit revision.
    CircuitAttachment,
    /// Reusable device-model or process-model library.
    ModelLibrary,
    /// Result generated only by a lease-owning simulation worker.
    SimulationResult,
    /// Sealed publication-snapshot document rendered into a public page.
    PublicationSnapshot,
}

impl ArtifactKind {
    /// Returns the canonical serialized vocabulary value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CircuitAttachment => "circuit_attachment",
            Self::ModelLibrary => "model_library",
            Self::SimulationResult => "simulation_result",
            Self::PublicationSnapshot => "publication_snapshot",
        }
    }

    /// Parses one canonical serialized vocabulary value.
    #[must_use]
    pub const fn from_wire_name(value: &str) -> Option<Self> {
        match value.as_bytes() {
            b"circuit_attachment" => Some(Self::CircuitAttachment),
            b"model_library" => Some(Self::ModelLibrary),
            b"simulation_result" => Some(Self::SimulationResult),
            b"publication_snapshot" => Some(Self::PublicationSnapshot),
            _ => None,
        }
    }
}

/// Artifact kinds a customer client may upload directly.
///
/// Simulation results are intentionally absent because only a simulation
/// worker may create them after engine output verification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientUploadArtifactKind {
    /// File attached to an immutable circuit revision.
    CircuitAttachment,
    /// Reusable device-model or process-model library.
    ModelLibrary,
    /// Sealed publication-snapshot document rendered into a public page.
    PublicationSnapshot,
}

impl From<ClientUploadArtifactKind> for ArtifactKind {
    fn from(value: ClientUploadArtifactKind) -> Self {
        match value {
            ClientUploadArtifactKind::CircuitAttachment => Self::CircuitAttachment,
            ClientUploadArtifactKind::ModelLibrary => Self::ModelLibrary,
            ClientUploadArtifactKind::PublicationSnapshot => Self::PublicationSnapshot,
        }
    }
}

/// Durable artifact lifecycle state.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactState {
    /// Metadata exists and the checksum-bound object upload may be pending.
    Uploading,
    /// Object size and checksum have been independently verified.
    Available,
    /// Verification permanently rejected the uploaded object.
    Rejected,
    /// Metadata is unavailable and object deletion has been durably queued.
    Deleted,
}

impl ArtifactState {
    /// Parses one canonical serialized vocabulary value.
    #[must_use]
    pub const fn from_wire_name(value: &str) -> Option<Self> {
        match value.as_bytes() {
            b"uploading" => Some(Self::Uploading),
            b"available" => Some(Self::Available),
            b"rejected" => Some(Self::Rejected),
            b"deleted" => Some(Self::Deleted),
            _ => None,
        }
    }
}

/// Checksum-bound request to create a direct object-storage upload session.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateArtifactUploadRequest {
    pub kind: ClientUploadArtifactKind,
    pub file_name: Option<String>,
    pub content_type: String,
    pub content_length: u64,
    pub content_sha256: String,
}

/// Immutable artifact integrity and lifecycle metadata.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Artifact {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub kind: ArtifactKind,
    pub state: ArtifactState,
    pub file_name: Option<String>,
    pub content_type: String,
    pub content_length: u64,
    pub content_sha256: String,
    pub verified_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Artifact metadata plus an optional short-lived direct-upload capability.
///
/// The three upload fields are either all populated or all null. They may be
/// null on an exact replay after the immutable upload session has expired, the
/// artifact became terminal, or the caller lost upload authority.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct ArtifactUpload {
    pub artifact: Artifact,
    pub upload_url: Option<String>,
    pub upload_headers: Option<BTreeMap<String, String>>,
    pub upload_expires_at: Option<String>,
}

/// Queues one immutable-revision simulation request.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateSimulationRunRequest {
    /// Omit to resolve the circuit's sealed head revision transactionally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<Uuid>,
    /// Versioned solver analysis configuration interpreted by the engine.
    pub analysis: Value,
}

/// Durable customer-visible state and provenance for one remote simulation.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct SimulationRun {
    pub id: Uuid,
    pub circuit_id: Uuid,
    pub revision_id: Uuid,
    pub status: SimulationRunStatus,
    pub analysis: Value,
    pub request_digest_version: i16,
    pub request_sha256: String,
    pub queued_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub cancellation_requested_at: Option<String>,
    pub execution_manifest: Option<Value>,
    pub result_manifest: Option<Value>,
    pub result_artifact_ids: Vec<Uuid>,
    pub failure_code: Option<String>,
    pub failure_detail: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Creates one immutable public circuit and optional simulation snapshot.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreatePublicationRequest {
    /// Verified `publication_snapshot` artifact the public page renders from.
    pub snapshot_artifact_id: Uuid,
    /// Omit to pin the circuit's sealed head at first acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<Uuid>,
    /// Optional succeeded simulation for the selected revision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulation_run_id: Option<Uuid>,
    /// Omit to snapshot the circuit title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Revision inputs explicitly authorized for unauthenticated download.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_artifact_ids: Vec<Uuid>,
}

/// Lifecycle of one publication's rendered public page.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationPageStatus {
    /// The render task is queued or running; the page URL shows a placeholder.
    Preparing,
    /// The immutable page bundle is rendered and served at the page URL.
    Live,
    /// Rendering dead-lettered; the page URL answers not-found until an
    /// operator requeues the render.
    Failed,
}

/// Authenticated management projection for one immutable publication.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct Publication {
    pub id: Uuid,
    pub slug: String,
    pub url_path: String,
    pub circuit_id: Uuid,
    pub revision_id: Uuid,
    pub simulation_run_id: Option<Uuid>,
    pub snapshot_artifact_id: Uuid,
    pub input_artifact_ids: Vec<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub page_status: PublicationPageStatus,
    pub published_at: String,
    pub sealed_at: String,
    pub unpublished_at: Option<String>,
}

/// Immutable succeeded-simulation snapshot disclosed by a publication.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct PublishedSimulation {
    pub id: Uuid,
    pub request_digest_version: i16,
    pub request_sha256: String,
    pub execution_manifest: Value,
    pub result_manifest: Value,
    pub artifact_ids: Vec<Uuid>,
}

/// Provenance category for one downloadable publication artifact.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationArtifactCategory {
    Input,
    Result,
}

/// Immutable integrity metadata for a publicly downloadable artifact.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct PublishedArtifact {
    pub id: Uuid,
    pub category: PublicationArtifactCategory,
    pub kind: ArtifactKind,
    pub file_name: Option<String>,
    pub content_type: String,
    pub size_bytes: u64,
    pub sha256: String,
}

/// Unauthenticated immutable circuit, simulation, and artifact publication.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct PublicPublication {
    pub id: Uuid,
    pub slug: String,
    pub url_path: String,
    pub title: String,
    pub description: Option<String>,
    pub circuit_id: Uuid,
    pub revision: SharedCircuitRevision,
    pub simulation: Option<PublishedSimulation>,
    pub downloadable_artifacts: Vec<PublishedArtifact>,
    pub published_at: String,
    pub sealed_at: String,
}

/// Non-secret management metadata for one immutable bearer share.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct CircuitShare {
    pub id: Uuid,
    pub circuit_id: Uuid,
    pub permission: SharePermission,
    pub revision_id: Uuid,
    pub expires_at: Option<String>,
    pub sealed_at: String,
    pub revoked_at: Option<String>,
    pub created_at: String,
}

/// Bearer-share metadata returned after creation or an exact replay.
///
/// `token` is null for preferred client-committed creation. It is populated
/// exactly once only by the compatibility keyless endpoint behavior.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreatedCircuitShare {
    pub id: Uuid,
    pub circuit_id: Uuid,
    pub token: Option<String>,
    pub permission: SharePermission,
    pub revision_id: Uuid,
    pub expires_at: Option<String>,
    pub sealed_at: String,
    pub revoked_at: Option<String>,
    pub created_at: String,
}

/// Immutable revision projection disclosed by a read-only circuit snapshot.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct SharedCircuitRevision {
    pub id: Uuid,
    pub schema_version: i32,
    pub content_digest_version: i16,
    pub document: Value,
    pub artifact_ids: Vec<Uuid>,
    pub content_sha256: String,
    pub created_at: String,
}

/// Read-only circuit projection disclosed by a bearer share or public discovery view.
#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct SharedCircuit {
    pub circuit_id: Uuid,
    pub title: String,
    pub permission: SharePermission,
    pub revision: SharedCircuitRevision,
}

/// Short-lived, self-verifying object-storage download handoff.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct ArtifactDownload {
    /// Immutable artifact identifier bound to the download route.
    pub artifact_id: Uuid,
    /// Immutable artifact classification.
    pub kind: ArtifactKind,
    /// Original optional file name supplied for the artifact.
    pub file_name: Option<String>,
    /// Verified media type recorded for the artifact.
    pub content_type: String,
    /// Exact verified byte length expected from object storage.
    pub content_length: u64,
    /// Lowercase hexadecimal SHA-256 of the exact expected bytes.
    pub content_sha256: String,
    pub download_url: String,
    pub download_expires_at: String,
}

/// Invitation metadata returned after retry-safe workspace invitation creation.
///
/// `token` is null for preferred client-committed issuance. It is populated
/// exactly once only by the compatibility keyless endpoint behavior.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreatedWorkspaceInvitation {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub token: Option<String>,
    pub email: String,
    pub role: WorkspaceInvitationRole,
    pub expires_at: String,
    pub accepted_at: Option<String>,
    pub revoked_at: Option<String>,
    pub created_at: String,
}

/// Role that may be assigned by a workspace invitation.
///
/// Ownership is deliberately absent because it requires an explicit transfer
/// after the recipient has accepted a non-owner invitation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceInvitationRole {
    Administrator,
    Editor,
    Viewer,
}

/// Non-secret lifecycle metadata for one email-bound workspace invitation.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorkspaceInvitation {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub email: String,
    pub role: WorkspaceInvitationRole,
    pub expires_at: String,
    pub accepted_at: Option<String>,
    pub revoked_at: Option<String>,
    pub created_at: String,
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IssueLicenseLeaseRequest {
    /// Client-generated UUID used to make retries safe after a lost response.
    pub request_id: Uuid,
    /// Base64url SHA-256 digest of stable device-binding material. Treat this
    /// persistent identifier as sensitive even though it is not a secret.
    pub device_fingerprint_sha256: String,
    /// Omit for a personal entitlement; set for a workspace entitlement.
    pub workspace_id: Option<Uuid>,
}

/// Returns whether a native-license issuance command has non-nil identities
/// and an exact canonical unpadded base64url SHA-256 device binding.
#[must_use]
pub fn is_valid_issue_license_lease_request(value: &IssueLicenseLeaseRequest) -> bool {
    !value.request_id.is_nil()
        && value.workspace_id.is_none_or(|id| !id.is_nil())
        && URL_SAFE_NO_PAD
            .decode(&value.device_fingerprint_sha256)
            .is_ok_and(|decoded| {
                decoded.len() == 32
                    && URL_SAFE_NO_PAD.encode(decoded) == value.device_fingerprint_sha256
            })
}

impl fmt::Debug for IssueLicenseLeaseRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IssueLicenseLeaseRequest")
            .field("request_id", &self.request_id)
            .field("device_fingerprint_sha256", &"[REDACTED]")
            .field("workspace_id", &self.workspace_id)
            .finish()
    }
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct LicenseLease {
    pub id: Uuid,
    pub entitlement_id: Uuid,
    pub workspace_id: Option<Uuid>,
    pub product: String,
    pub plan: String,
    pub issued_at: String,
    pub expires_at: String,
    pub revoked_at: Option<String>,
    pub revocation_reason: Option<String>,
    pub signing_key_id: String,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct LicenseLeaseList {
    pub items: Vec<LicenseLease>,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
pub struct IssuedLicenseLease {
    #[serde(flatten)]
    pub lease: LicenseLease,
    /// Caller-generated retry identity echoed to bind this success to the
    /// exact durable issuance command.
    pub request_id: Uuid,
    /// Compact PS256 JWS returned only to the authenticated native client.
    pub lease_token: String,
}

impl fmt::Debug for IssuedLicenseLease {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IssuedLicenseLease")
            .field("lease_id", &self.lease.id)
            .field("request_id", &self.request_id)
            .field("lease_token", &"[REDACTED]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn lease() -> LicenseLease {
        LicenseLease {
            id: Uuid::from_u128(1),
            entitlement_id: Uuid::from_u128(2),
            workspace_id: Some(Uuid::from_u128(3)),
            product: "rspice".to_owned(),
            plan: "team".to_owned(),
            issued_at: "2026-07-18T00:00:00Z".to_owned(),
            expires_at: "2026-07-19T00:00:00Z".to_owned(),
            revoked_at: None,
            revocation_reason: None,
            signing_key_id: "lease-2026-01".to_owned(),
        }
    }

    #[test]
    fn problem_details_uses_the_rfc_type_field() {
        let problem = ProblemDetails {
            kind: "https://rspice.app/problems/conflict".to_owned(),
            title: "Conflicting change".to_owned(),
            status: 409,
            detail: "The request conflicts with durable state.".to_owned(),
            instance: "/api/v1/workspaces".to_owned(),
        };
        let value = serde_json::to_value(problem).expect("serialize problem");
        assert_eq!(value["type"], "https://rspice.app/problems/conflict");
        assert!(value.get("kind").is_none());
    }

    #[test]
    fn issued_license_lease_flattens_durable_fields_and_redacts_debug() {
        let issued = IssuedLicenseLease {
            lease: lease(),
            request_id: Uuid::from_u128(4),
            lease_token: "secret.compact.jws".to_owned(),
        };
        let value = serde_json::to_value(&issued).expect("serialize lease");
        assert_eq!(value["id"], Uuid::from_u128(1).to_string());
        assert_eq!(value["request_id"], Uuid::from_u128(4).to_string());
        assert_eq!(value["lease_token"], "secret.compact.jws");
        assert!(value.get("lease").is_none());
        let debug = format!("{issued:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains("secret.compact.jws"));
    }

    #[test]
    fn native_license_issuance_commands_require_canonical_nonnil_bindings() {
        let mut request = IssueLicenseLeaseRequest {
            request_id: Uuid::from_u128(1),
            device_fingerprint_sha256: URL_SAFE_NO_PAD.encode([7_u8; 32]),
            workspace_id: Some(Uuid::from_u128(2)),
        };
        assert!(is_valid_issue_license_lease_request(&request));

        request.request_id = Uuid::nil();
        assert!(!is_valid_issue_license_lease_request(&request));
        request.request_id = Uuid::from_u128(1);
        request.workspace_id = Some(Uuid::nil());
        assert!(!is_valid_issue_license_lease_request(&request));
        request.workspace_id = None;
        request.device_fingerprint_sha256.push('=');
        assert!(!is_valid_issue_license_lease_request(&request));
    }

    #[test]
    fn mutation_requests_reject_unknown_fields() {
        let unknown_workspace = serde_json::from_value::<CreateWorkspaceRequest>(json!({
            "slug": "engineering",
            "name": "Engineering",
            "unexpected": true
        }));
        assert!(unknown_workspace.is_err());

        let unknown_member_update = serde_json::from_value::<UpdateWorkspaceMemberRequest>(json!({
            "role": "editor",
            "unexpected": true
        }));
        assert!(unknown_member_update.is_err());

        let unknown_circuit = serde_json::from_value::<CreateCircuitRequest>(json!({
            "title": "Precision amplifier",
            "schema_version": 1,
            "document": {},
            "unexpected": true
        }));
        assert!(unknown_circuit.is_err());

        let unknown_artifact = serde_json::from_value::<CreateArtifactUploadRequest>(json!({
            "kind": "model_library",
            "file_name": "models.lib",
            "content_type": "text/plain",
            "content_length": 128,
            "content_sha256": "00".repeat(32),
            "unexpected": true
        }));
        assert!(unknown_artifact.is_err());

        let unknown_simulation = serde_json::from_value::<CreateSimulationRunRequest>(json!({
            "revision_id": Uuid::from_u128(10),
            "analysis": {"kind": "transient", "stop": "1ms"},
            "unexpected": true
        }));
        assert!(unknown_simulation.is_err());
        assert_eq!(
            serde_json::to_value(CreateSimulationRunRequest {
                revision_id: None,
                analysis: json!({"kind": "operating_point"}),
            })
            .expect("serialize simulation command"),
            json!({"analysis": {"kind": "operating_point"}})
        );

        let unknown_publication = serde_json::from_value::<CreatePublicationRequest>(json!({
            "revision_id": Uuid::from_u128(10),
            "unexpected": true
        }));
        assert!(unknown_publication.is_err());
        assert_eq!(
            serde_json::to_value(CreatePublicationRequest {
                snapshot_artifact_id: Uuid::from_u128(11),
                revision_id: None,
                simulation_run_id: None,
                title: None,
                description: None,
                input_artifact_ids: Vec::new(),
            })
            .expect("serialize publication command"),
            json!({"snapshot_artifact_id": Uuid::from_u128(11)})
        );

        let unknown_ticket = serde_json::from_value::<CreateCollaborationTicketRequest>(json!({
            "client_instance_id": Uuid::from_u128(13),
            "unexpected": true
        }));
        assert!(unknown_ticket.is_err());
        assert_eq!(
            serde_json::to_value(CreateCollaborationTicketRequest {
                client_instance_id: Some(Uuid::from_u128(13)),
            })
            .expect("serialize collaboration ticket command"),
            json!({"client_instance_id": Uuid::from_u128(13)})
        );

        let artifact = CreateArtifactUploadRequest {
            kind: ClientUploadArtifactKind::ModelLibrary,
            file_name: Some("models.lib".to_owned()),
            content_type: "text/plain".to_owned(),
            content_length: 128,
            content_sha256: "00".repeat(32),
        };
        assert_eq!(
            serde_json::to_value(artifact).expect("serialize artifact command"),
            json!({
                "kind": "model_library",
                "file_name": "models.lib",
                "content_type": "text/plain",
                "content_length": 128,
                "content_sha256": "00".repeat(32)
            })
        );

        let circuit = CreateCircuitRequest {
            title: "Precision amplifier".to_owned(),
            visibility: None,
            schema_version: 1,
            document: json!({"components": []}),
            artifact_ids: None,
        };
        assert_eq!(
            serde_json::to_value(circuit).expect("serialize circuit command"),
            json!({
                "title": "Precision amplifier",
                "schema_version": 1,
                "document": {"components": []}
            })
        );

        let revision = CreateCircuitRevisionRequest {
            expected_row_version: 2,
            parent_revision_id: None,
            schema_version: 1,
            document: json!({"components": []}),
            artifact_ids: None,
        };
        assert_eq!(
            serde_json::to_value(revision).expect("serialize revision command"),
            json!({
                "expected_row_version": 2,
                "schema_version": 1,
                "document": {"components": []}
            })
        );

        let update = UpdateCircuitRequest {
            expected_row_version: 3,
            title: None,
            visibility: Some(CircuitVisibility::Private),
            archived: None,
        };
        assert_eq!(
            serde_json::to_value(update).expect("serialize circuit update"),
            json!({
                "expected_row_version": 3,
                "visibility": "private"
            })
        );

        let request = IssueLicenseLeaseRequest {
            request_id: Uuid::from_u128(4),
            device_fingerprint_sha256: "device-evidence".to_owned(),
            workspace_id: None,
        };
        let debug = format!("{request:?}");
        assert!(debug.contains("[REDACTED]"));
        assert!(!debug.contains("device-evidence"));
    }

    #[test]
    fn response_types_tolerate_additive_v1_fields() {
        let principal = serde_json::from_value::<CurrentPrincipal>(json!({
            "id": Uuid::from_u128(5),
            "email": "engineer@rspice.test",
            "display_name": "Engineer",
            "created_at": "2026-07-18T00:00:00Z",
            "updated_at": "2026-07-18T00:00:00Z",
            "future_additive_field": {"ignored": true}
        }))
        .expect("additive response field remains compatible");
        assert_eq!(principal.id, Uuid::from_u128(5));

        let invitation = serde_json::from_value::<CreatedWorkspaceInvitation>(json!({
            "id": Uuid::from_u128(6),
            "workspace_id": Uuid::from_u128(1),
            "token": null,
            "email": "engineer@rspice.test",
            "role": "viewer",
            "expires_at": "2026-08-01T00:00:00Z",
            "accepted_at": null,
            "revoked_at": null,
            "created_at": "2026-07-19T00:00:00Z",
            "future_delivery_state": "queued"
        }))
        .expect("additive invitation response field remains compatible");
        assert_eq!(invitation.id, Uuid::from_u128(6));
        assert!(invitation.token.is_none());

        let member = serde_json::from_value::<WorkspaceMember>(json!({
            "workspace_id": Uuid::from_u128(1),
            "principal_id": Uuid::from_u128(7),
            "email": "engineer@rspice.test",
            "display_name": "Engineer",
            "role": "editor",
            "joined_at": "2026-07-19T00:00:00Z",
            "future_presence": "online"
        }))
        .expect("additive workspace-member response field remains compatible");
        assert_eq!(member.principal_id, Uuid::from_u128(7));
        assert_eq!(member.role, WorkspaceRole::Editor);

        let audit_event = serde_json::from_value::<AuditEvent>(json!({
            "id": Uuid::from_u128(12),
            "workspace_id": Uuid::from_u128(1),
            "actor_principal_id": Uuid::from_u128(7),
            "action": "workspace.member_added",
            "target_type": "workspace_member",
            "target_id": Uuid::from_u128(7),
            "metadata": {"role": "editor"},
            "occurred_at": "2026-07-19T00:00:00Z",
            "future_evidence_version": 2
        }))
        .expect("additive audit-event response fields remain compatible");
        assert_eq!(audit_event.workspace_id, Uuid::from_u128(1));

        let invitation_record = serde_json::from_value::<WorkspaceInvitation>(json!({
            "id": Uuid::from_u128(13),
            "workspace_id": Uuid::from_u128(1),
            "email": "reviewer@rspice.test",
            "role": "administrator",
            "expires_at": "2026-08-01T00:00:00Z",
            "accepted_at": null,
            "revoked_at": null,
            "created_at": "2026-07-19T00:00:00Z",
            "future_delivery_state": "sent"
        }))
        .expect("additive invitation-record response fields remain compatible");
        assert_eq!(
            invitation_record.role,
            WorkspaceInvitationRole::Administrator
        );

        let circuit = serde_json::from_value::<Circuit>(json!({
            "id": Uuid::from_u128(8),
            "workspace_id": Uuid::from_u128(9),
            "title": "Precision amplifier",
            "visibility": "private",
            "head_revision_id": Uuid::from_u128(10),
            "row_version": 3,
            "archived_at": null,
            "created_at": "2026-07-19T00:00:00Z",
            "updated_at": "2026-07-19T00:01:00Z",
            "future_lock_owner": null
        }))
        .expect("additive circuit response field remains compatible");
        assert_eq!(circuit.id, Uuid::from_u128(8));
        assert_eq!(circuit.visibility, CircuitVisibility::Private);

        let revision = serde_json::from_value::<CircuitRevision>(json!({
            "id": Uuid::from_u128(10),
            "parent_revision_id": null,
            "schema_version": 1,
            "content_digest_version": 2,
            "document": {"components": []},
            "artifact_ids": [],
            "content_sha256": "00".repeat(32),
            "created_at": "2026-07-19T00:00:00Z",
            "future_solver_compatibility": ["v2"]
        }))
        .expect("additive circuit-revision response field remains compatible");
        assert_eq!(revision.id, Uuid::from_u128(10));
        assert_eq!(revision.content_digest_version, 2);

        let collaboration = serde_json::from_value::<CollaborationTicketProtocol>(json!({
            "protocol": COLLABORATION_PROTOCOL,
            "websocket_endpoint": format!(
                "/api/v1/collaboration/{}/connect",
                Uuid::from_u128(8)
            ),
            "ticket_protocol": format!(
                "{COLLABORATION_TICKET_PROTOCOL_PREFIX}{}",
                "A".repeat(CAPABILITY_TOKEN_LENGTH)
            ),
            "client_instance_id": Uuid::from_u128(13),
            "expires_at": "2026-07-19T00:05:00Z",
            "document": {
                "base_revision_id": Uuid::from_u128(10),
                "schema_version": 1,
                "latest_sequence": 0,
                "future_compaction_sequence": 0
            },
            "future_replica": "central-1"
        }))
        .expect("additive collaboration-ticket response fields remain compatible");
        assert_eq!(collaboration.client_instance_id, Uuid::from_u128(13));
        assert_eq!(collaboration.document.base_revision_id, Uuid::from_u128(10));

        let simulation = serde_json::from_value::<SimulationRun>(json!({
            "id": Uuid::from_u128(14),
            "circuit_id": Uuid::from_u128(8),
            "revision_id": Uuid::from_u128(10),
            "status": "succeeded",
            "analysis": {"kind": "transient", "future_tolerance": "default"},
            "request_digest_version": CURRENT_SIMULATION_REQUEST_DIGEST_VERSION,
            "request_sha256": "0a".repeat(32),
            "queued_at": "2026-07-19T00:00:00Z",
            "started_at": "2026-07-19T00:00:01Z",
            "completed_at": "2026-07-19T00:00:02Z",
            "cancellation_requested_at": null,
            "execution_manifest": {
                "protocol_version": 3,
                "engine_protocol_version": 3,
                "attempt": 1,
                "worker_class": "shared",
                "engine": {
                    "name": "rspice-engine",
                    "build": "2026.07.20",
                    "runtime_mode": "self_contained",
                    "adapter_sha256": "ab".repeat(32),
                    "solver_sha256": null,
                    "model_library_sha256": null,
                },
                "revision": {"content_digest_version": 2},
                "request": {"digest_version": 1},
                "artifacts": [],
                "future_host": "isolated",
            },
            "result_manifest": {"format": "rspice-result-v1", "future_index": []},
            "result_artifact_ids": [Uuid::from_u128(15)],
            "failure_code": null,
            "failure_detail": null,
            "created_at": "2026-07-19T00:00:00Z",
            "updated_at": "2026-07-19T00:00:02Z",
            "future_queue_class": "precision",
        }))
        .expect("additive simulation response fields remain compatible");
        assert_eq!(simulation.id, Uuid::from_u128(14));
        assert_eq!(simulation.status, SimulationRunStatus::Succeeded);

        let publication = serde_json::from_value::<Publication>(json!({
            "id": Uuid::from_u128(16),
            "slug": "23456789abcdefghjkmn",
            "url_path": "/c/23456789abcdefghjkmn",
            "circuit_id": Uuid::from_u128(8),
            "revision_id": Uuid::from_u128(10),
            "simulation_run_id": Uuid::from_u128(14),
            "snapshot_artifact_id": Uuid::from_u128(17),
            "input_artifact_ids": [],
            "title": "Precision amplifier",
            "description": "Verified response",
            "page_status": "live",
            "published_at": "2026-07-19T00:00:00Z",
            "sealed_at": "2026-07-19T00:00:01Z",
            "unpublished_at": null,
            "future_moderation_state": "clear",
        }))
        .expect("additive publication management fields remain compatible");
        assert_eq!(publication.id, Uuid::from_u128(16));
        assert_eq!(publication.page_status, PublicationPageStatus::Live);

        let public_publication = serde_json::from_value::<PublicPublication>(json!({
            "id": Uuid::from_u128(16),
            "slug": "23456789abcdefghjkmn",
            "url_path": "/c/23456789abcdefghjkmn",
            "title": "Precision amplifier",
            "description": null,
            "circuit_id": Uuid::from_u128(8),
            "revision": {
                "id": Uuid::from_u128(10),
                "schema_version": 1,
                "content_digest_version": 2,
                "document": {"components": [], "future_schema_hint": 2},
                "artifact_ids": [],
                "content_sha256": "00".repeat(32),
                "created_at": "2026-07-19T00:00:00Z",
                "future_revision_field": true
            },
            "simulation": null,
            "downloadable_artifacts": [],
            "published_at": "2026-07-19T00:00:00Z",
            "sealed_at": "2026-07-19T00:00:01Z",
            "future_publication_field": {"ignored": true},
        }))
        .expect("additive public-publication fields remain compatible");
        assert_eq!(public_publication.revision.id, Uuid::from_u128(10));

        let artifact = serde_json::from_value::<ArtifactUpload>(json!({
            "artifact": {
                "id": Uuid::from_u128(12),
                "workspace_id": Uuid::from_u128(9),
                "kind": "model_library",
                "state": "uploading",
                "file_name": "models.lib",
                "content_type": "text/plain",
                "content_length": 128,
                "content_sha256": "00".repeat(32),
                "verified_at": null,
                "created_at": "2026-07-19T00:00:00Z",
                "updated_at": "2026-07-19T00:00:00Z",
                "future_scanner_state": "pending"
            },
            "upload_url": "https://objects.rspice.test/presigned",
            "upload_headers": {
                "content-type": "text/plain",
                "x-amz-checksum-sha256": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="
            },
            "upload_expires_at": "2026-07-19T00:05:00Z",
            "future_transport": "s3"
        }))
        .expect("additive artifact-upload response fields remain compatible");
        assert_eq!(artifact.artifact.id, Uuid::from_u128(12));
        assert_eq!(artifact.artifact.kind, ArtifactKind::ModelLibrary);
        assert_eq!(artifact.artifact.state, ArtifactState::Uploading);
        assert!(artifact.upload_url.is_some());

        assert_eq!(
            ArtifactKind::from_wire_name(ArtifactKind::SimulationResult.as_str()),
            Some(ArtifactKind::SimulationResult)
        );
        assert_eq!(ArtifactKind::from_wire_name("future_kind"), None);
        assert_eq!(
            ArtifactState::from_wire_name("available"),
            Some(ArtifactState::Available)
        );
        assert_eq!(ArtifactState::from_wire_name("future_state"), None);

        let share = serde_json::from_value::<CreatedCircuitShare>(json!({
            "id": Uuid::from_u128(11),
            "circuit_id": Uuid::from_u128(8),
            "token": null,
            "permission": "view",
            "revision_id": Uuid::from_u128(10),
            "expires_at": null,
            "revoked_at": null,
            "created_at": "2026-07-19T00:00:00Z",
            "sealed_at": "2026-07-19T00:00:00Z",
            "future_delivery_count": 1
        }))
        .expect("additive bearer-share response field remains compatible");
        assert_eq!(share.id, Uuid::from_u128(11));
        assert!(share.token.is_none());
        assert_eq!(share.permission, SharePermission::View);

        let shared = serde_json::from_value::<SharedCircuit>(json!({
            "circuit_id": Uuid::from_u128(8),
            "title": "Precision amplifier",
            "permission": "view",
            "revision": {
                "id": Uuid::from_u128(10),
                "schema_version": 1,
                "content_digest_version": 2,
                "document": {"components": []},
                "artifact_ids": [],
                "content_sha256": "00".repeat(32),
                "created_at": "2026-07-19T00:00:00Z",
                "future_integrity_version": 3
            },
            "future_comment_count": 0
        }))
        .expect("additive shared-circuit response fields remain compatible");
        assert_eq!(shared.circuit_id, Uuid::from_u128(8));
        assert_eq!(shared.revision.id, Uuid::from_u128(10));

        let download = serde_json::from_value::<ArtifactDownload>(json!({
            "artifact_id": Uuid::from_u128(12),
            "kind": "model_library",
            "file_name": "models.lib",
            "content_type": "text/plain",
            "content_length": 128,
            "content_sha256": "01".repeat(32),
            "download_url": "https://objects.rspice.test/presigned",
            "download_expires_at": "2026-07-19T00:05:00Z",
            "future_checksum": "ignored"
        }))
        .expect("additive artifact-download response field remains compatible");
        assert_eq!(download.artifact_id, Uuid::from_u128(12));
        assert_eq!(download.download_expires_at, "2026-07-19T00:05:00Z");

        assert!(
            serde_json::from_value::<CircuitShare>(json!({
                "id": Uuid::from_u128(11),
                "permission": "edit",
                "revision_id": Uuid::from_u128(10),
                "expires_at": null,
                "revoked_at": null,
                "created_at": "2026-07-19T00:00:00Z"
            }))
            .is_err()
        );

        let invalid_owner_invitation =
            serde_json::from_value::<CreatedWorkspaceInvitation>(json!({
                "id": Uuid::from_u128(6),
                "workspace_id": Uuid::from_u128(1),
                "token": null,
                "email": "engineer@rspice.test",
                "role": "owner",
                "expires_at": "2026-08-01T00:00:00Z",
                "accepted_at": null,
                "revoked_at": null,
                "created_at": "2026-07-19T00:00:00Z"
            }));
        assert!(
            invalid_owner_invitation.is_err(),
            "invitation responses cannot silently broaden into ownership"
        );

        let jwks = serde_json::from_value::<LicenseJwkSet>(json!({
            "keys": [{
                "kty": "RSA",
                "n": "modulus",
                "e": "AQAB",
                "kid": "lease-2026-01",
                "use": "sig",
                "alg": "PS256",
                "future_thumbprint": "ignored"
            }],
            "future_set_metadata": true
        }))
        .expect("additive JWK fields remain compatible");
        assert_eq!(jwks.keys[0].alg, LicenseKeyAlgorithm::Ps256);

        let wrong_algorithm = serde_json::from_value::<LicenseJwkSet>(json!({
            "keys": [{
                "kty": "RSA",
                "n": "modulus",
                "e": "AQAB",
                "kid": "lease-2026-01",
                "use": "sig",
                "alg": "RS256"
            }]
        }));
        assert!(wrong_algorithm.is_err());
    }

    #[test]
    fn capability_tokens_require_canonical_256_bit_base64url() {
        assert!(is_canonical_capability_token(
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
        ));
        assert!(!is_canonical_capability_token("short"));
        assert!(!is_canonical_capability_token(
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA/"
        ));
        assert!(!is_canonical_capability_token(
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAB"
        ));

        for final_character in CAPABILITY_TOKEN_FINAL_CHARACTERS {
            let token = format!(
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA{}",
                char::from(*final_character)
            );
            assert!(is_canonical_capability_token(&token));
        }
    }

    #[test]
    fn native_license_jwks_require_strong_canonical_rsa_keys() {
        let modulus = URL_SAFE_NO_PAD.encode(
            std::iter::once(0x80_u8)
                .chain(std::iter::repeat_n(0x5a, 255))
                .collect::<Vec<_>>(),
        );
        let key = LicenseJwk {
            kty: LicenseKeyType::Rsa,
            n: modulus,
            e: LICENSE_RSA_PUBLIC_EXPONENT.to_owned(),
            kid: "lease-2026-01".to_owned(),
            key_use: LicenseKeyUse::Signature,
            alg: LicenseKeyAlgorithm::Ps256,
        };
        assert!(is_valid_license_jwk(&key));
        assert!(is_valid_license_jwk_set(&LicenseJwkSet {
            keys: vec![key.clone()],
        }));

        let mut weak = key.clone();
        weak.n = URL_SAFE_NO_PAD.encode(
            std::iter::once(0x80_u8)
                .chain(std::iter::repeat_n(0x5a, 254))
                .collect::<Vec<_>>(),
        );
        assert!(!is_valid_license_jwk(&weak));

        let mut noncanonical = key.clone();
        noncanonical.n = URL_SAFE_NO_PAD.encode(
            std::iter::once(0_u8)
                .chain(URL_SAFE_NO_PAD.decode(&key.n).expect("test modulus"))
                .collect::<Vec<_>>(),
        );
        assert!(!is_valid_license_jwk(&noncanonical));

        let mut wrong_exponent = key.clone();
        wrong_exponent.e = "Aw".to_owned();
        assert!(!is_valid_license_jwk(&wrong_exponent));
        assert!(!is_valid_license_jwk_set(&LicenseJwkSet {
            keys: vec![key.clone(), key],
        }));
    }

    #[test]
    fn collaboration_protocol_constants_are_distinct_http_tokens() {
        assert_eq!(COLLABORATION_PROTOCOL, "rspice.automerge.v1");
        assert_eq!(COLLABORATION_TICKET_PROTOCOL_PREFIX, "rspice.ticket.");
        for value in [COLLABORATION_PROTOCOL, COLLABORATION_TICKET_PROTOCOL_PREFIX] {
            assert!(value.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_')
            }));
        }
    }
}
