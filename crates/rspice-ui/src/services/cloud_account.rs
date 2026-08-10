//! RSpice Cloud account boundary.
//!
//! This service owns everything the typed cloud client deliberately does not:
//! the OIDC authorization-code + PKCE sign-in, token refresh, protected
//! at-rest storage of the refresh token and native license lease, and the
//! startup/bootstrap sequence from `docs/client-integration.md` in
//! RSpice-Cloud. All network work runs on one background executor thread; the
//! UI reads an owned [`CloudSessionSnapshot`] and never sees a token, a cloud
//! crate type, or a partially validated response.
//!
//! Availability is explicit and fails closed:
//! - a build without release-pinned cloud endpoints reports
//!   [`CloudAccountAvailability::UnconfiguredBuild`] and exposes no actions;
//! - configured browser builds use authorization-code + PKCE, memory-only
//!   tokens, bounded Fetch, and the authenticated v2 WebSocket relay. Native
//!   offline license leases remain desktop-only.

#[cfg(target_arch = "wasm32")]
mod browser_executor;
#[cfg(target_arch = "wasm32")]
mod browser_live_relay;
pub(crate) mod config;
#[cfg(not(target_arch = "wasm32"))]
mod executor;
#[cfg(not(target_arch = "wasm32"))]
mod live_relay;
#[cfg(not(target_arch = "wasm32"))]
mod native_login;
mod oidc;
#[cfg(not(target_arch = "wasm32"))]
mod publish;
#[cfg(not(target_arch = "wasm32"))]
mod store;

/// Entitlement feature key the service requires before accepting a
/// publication. Must match the API's `require_cloud_publishing_entitlement`.
pub(crate) const CLOUD_PUBLISHING_FEATURE: &str = "cloud_publishing";

/// Entitlement feature key for hosting or joining live sessions. Must match
/// the API's live-collaboration entitlement checks; every participant —
/// host and guest alike — needs it.
pub(crate) const LIVE_COLLABORATION_FEATURE: &str = "live_collaboration";

/// Whether this build can operate a cloud account session at all.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CloudAccountAvailability {
    /// Native build with release-configured endpoints: full sign-in, refresh,
    /// entitlements, and offline license leases.
    Native,
    /// Browser build with release-pinned endpoints and callback URI. Account
    /// state is online-only and tokens live only in WebAssembly memory.
    ///
    /// Only a browser build ever constructs this, but every target matches on
    /// it: the availability tables answer what each host offers, and a desktop
    /// build still has to say that web publishing is the desktop's job.
    #[cfg_attr(
        not(target_arch = "wasm32"),
        expect(dead_code, reason = "constructed only by browser builds")
    )]
    Browser,
    /// This build was produced without pinned cloud endpoints, so every cloud
    /// account surface reports the boundary instead of offering actions.
    UnconfiguredBuild,
}

/// Lifecycle phase of the cloud session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CloudSessionPhase {
    /// No account session. `last_error` carries the most recent sign-in or
    /// refresh failure in presentation-safe form.
    SignedOut { last_error: Option<String> },
    /// The system browser is showing the identity provider's sign-in page.
    WaitingForBrowser,
    /// The authorization code arrived; tokens are being exchanged.
    ExchangingTokens,
    /// Tokens are held; principal, entitlements, workspaces, and the native
    /// license lease are being fetched.
    Bootstrapping,
    /// Fully established server-verified session.
    Active,
    /// The stored native license lease verified offline, but the service has
    /// not yet reached the server in this run.
    OfflineLicensed,
}

/// Server-owned identity facts about the signed-in principal.
///
/// Carries exactly what the console renders; richer projections arrive with
/// the surfaces that prove them.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct PrincipalSummary {
    pub email: Option<String>,
    pub display_name: Option<String>,
}

/// One entitlement row as the server projects it (history included).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct EntitlementSummary {
    pub status: String,
    /// Explicit Boolean feature grants (`true` values only, fail closed).
    pub granted_features: Vec<String>,
    pub valid_until: Option<String>,
}

/// The verified native license lease bound to this device.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NativeLicenseSummary {
    pub plan: String,
    pub product: String,
    /// Feature names granted `true` in the authenticated feature policy.
    pub granted_features: Vec<String>,
    pub expires_at_unix_seconds: i64,
    pub lease_id: String,
}

/// One issued license lease (a signed-in device) from the server's history.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LeaseSummary {
    pub id: String,
    pub plan: String,
    pub issued_at: String,
    pub expires_at: String,
    pub revoked_at: Option<String>,
    /// Whether this row is the lease this installation currently holds.
    pub this_device: bool,
}

/// A workspace the principal can publish into.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorkspaceSummary {
    pub id: String,
    pub name: String,
}

/// Where a publication's circuit lives, as the publish dialog resolved it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PublishTarget {
    /// This project already published before: reuse its bound circuit.
    ExistingCircuit {
        workspace_id: uuid::Uuid,
        circuit_id: uuid::Uuid,
    },
    /// First publication: create a circuit in this workspace.
    NewCircuit {
        workspace_id: uuid::Uuid,
        /// `true` publishes a listed, indexable page; `false` stays unlisted.
        public: bool,
    },
}

/// One publish command as the dialog confirmed it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PublishRequest {
    pub target: PublishTarget,
    pub title: String,
    pub description: String,
    /// Canonical publication snapshot bytes (`.rspicepub`).
    pub snapshot_bytes: Vec<u8>,
}

/// The receipt the UI persists into the project descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PublishReceipt {
    pub workspace_id: String,
    pub circuit_id: String,
    pub publication_id: String,
    pub slug: String,
    pub url_path: String,
    /// `preparing` | `live` | `failed` at creation time (always `preparing`).
    pub page_status: String,
}

/// Progress of the publish in flight, rendered by the dialog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PublishState {
    /// Uploading the snapshot and sealing the circuit revision.
    Publishing,
    /// The publication exists; the page render is still preparing.
    AwaitingPage { url_path: String },
    /// The page answered live.
    Live { url_path: String },
    /// The publish failed with a presentation-safe reason.
    Failed { message: String },
}

/// One publication of the bound circuit, for the Published band.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PublicationSummary {
    pub id: String,
    pub title: String,
    pub url_path: String,
    pub page_status: String,
    pub published_at: String,
    pub unpublished_at: Option<String>,
}

/// One roster row of the live session, rendered fields only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LiveParticipantSummary {
    pub principal_id: String,
    pub display_name: String,
    /// Whether this row is the session host (a fixed editor).
    pub is_host: bool,
    /// Whether this row is the signed-in principal.
    pub is_self: bool,
    /// Whether the participant may edit and trigger runs.
    pub editor: bool,
    /// Whether the participant still waits for the host's approval.
    pub pending: bool,
    pub joined_at: String,
}

/// Host-controlled session policy, as the dialog renders and applies it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveSessionPolicySummary {
    /// Guests joining by code become editors (`true`) or viewers (`false`).
    pub guests_edit: bool,
    /// Whether each join waits for the host's approval.
    pub approve_joins: bool,
    /// Whether participants may fork the session's work.
    pub allow_save_copy: bool,
}

/// The live session as the UI renders it. Carries no connect credential.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LiveSessionSummary {
    pub session_id: String,
    /// The human-relayable join code, held only while hosting: the server
    /// returns it exactly once per creation or rotation and stores only a
    /// digest.
    pub join_code: Option<String>,
    /// Whether this install hosts the session.
    pub hosting: bool,
    /// Whether the signed-in participant may edit and trigger runs.
    pub editor: bool,
    pub policy: LiveSessionPolicySummary,
    pub participants: Vec<LiveParticipantSummary>,
    pub started_at: String,
    /// Whether the relay socket is attached and streaming right now. False
    /// while connecting, reconnecting, or awaiting approval.
    pub relay_connected: bool,
    /// The most recent host action that failed, presentation-safe; cleared
    /// by the next successful action or roster refresh.
    pub notice: Option<String>,
}

/// Payload class of one live-relay frame, mirroring the relay's class byte.
/// The relay enforces capability by class alone: everyone sends presence and
/// cursor frames, editors add document and run-request frames, and run-status
/// frames are host-only.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LiveFrameClass {
    Presence,
    Cursor,
    Document,
    RunRequest,
    RunStatus,
}

impl LiveFrameClass {
    pub(crate) const fn as_byte(self) -> u8 {
        match self {
            Self::Presence => 0,
            Self::Cursor => 1,
            Self::Document => 2,
            Self::RunRequest => 3,
            Self::RunStatus => 4,
        }
    }

    pub(crate) const fn from_byte(value: u8) -> Option<Self> {
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

/// Hard per-frame bound: the relay closes the connection over anything
/// larger, so senders must chunk beneath it.
pub(crate) const MAX_LIVE_FRAME_BYTES: usize = 1024 * 1024;
/// Relay-delivered v2 frames add two authenticated UUIDs after the class
/// byte. This overhead is outside the peer-controlled frame-size allowance.
pub(crate) const LIVE_RELAY_AUTHENTICATED_SENDER_BYTES: usize = 32;
pub(crate) const MAX_LIVE_RELAY_WIRE_BYTES: usize =
    MAX_LIVE_FRAME_BYTES + LIVE_RELAY_AUTHENTICATED_SENDER_BYTES;
/// Finite queues bound retained document data when either the browser socket
/// or UI frame loop stalls. One 64 MiB document batch needs 128 chunks.
pub(crate) const LIVE_RELAY_OUTBOUND_QUEUE_CAPACITY: usize = 144;
pub(crate) const LIVE_RELAY_INBOUND_QUEUE_CAPACITY: usize = 144;

/// Per-client full-jitter exponential retry delay. It is deterministic
/// for one installation/run identity and attempt, but distinct client UUIDs
/// spread a replica or bus recovery over the whole backoff window without
/// relying on another runtime entropy source. Server-side admission remains
/// the final protection against an unhealthy or malicious client.
fn client_retry_delay(attempt: u32, client_instance_id: uuid::Uuid) -> std::time::Duration {
    const BASE_MILLISECONDS: u64 = 1_000;
    const MAX_MILLISECONDS: u64 = 30_000;
    let multiplier = 1_u64 << attempt.min(5);
    let ceiling = BASE_MILLISECONDS
        .saturating_mul(multiplier)
        .min(MAX_MILLISECONDS);
    let floor = ceiling / 2;

    let bytes = client_instance_id.as_bytes();
    let mut seed = u64::from_be_bytes(bytes[..8].try_into().expect("UUID half is eight bytes"))
        ^ u64::from_be_bytes(bytes[8..].try_into().expect("UUID half is eight bytes"))
        ^ u64::from(attempt).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    // SplitMix64 finalizer: stable, cheap diffusion for timing jitter only.
    seed ^= seed >> 30;
    seed = seed.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    seed ^= seed >> 27;
    seed = seed.wrapping_mul(0x94d0_49bb_1331_11eb);
    seed ^= seed >> 31;
    let milliseconds = floor + seed % (ceiling - floor + 1);
    std::time::Duration::from_millis(milliseconds)
}

/// Identity stamped by the relay from the consumed one-time ticket. Payload
/// identity fields are compatibility data only and never authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LiveRelayIdentity {
    pub principal_id: uuid::Uuid,
    pub client_instance_id: uuid::Uuid,
}

/// One live-relay frame. Outbound bytes contain class + payload; inbound v2
/// bytes contain class + the relay-authenticated sender + payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LiveFrame {
    pub class: LiveFrameClass,
    pub authenticated_sender: Option<LiveRelayIdentity>,
    pub payload: Vec<u8>,
}

impl LiveFrame {
    pub(crate) fn encode(&self) -> Vec<u8> {
        debug_assert!(
            self.authenticated_sender.is_none(),
            "relay-delivered frames must never be sent back as local authority"
        );
        let mut bytes = Vec::with_capacity(1 + self.payload.len());
        bytes.push(self.class.as_byte());
        bytes.extend_from_slice(&self.payload);
        bytes
    }

    pub(crate) fn decode_authenticated(bytes: &[u8]) -> Option<Self> {
        let (class_byte, stamped) = bytes.split_first()?;
        if stamped.len() < LIVE_RELAY_AUTHENTICATED_SENDER_BYTES {
            return None;
        }
        let principal_id = uuid::Uuid::from_slice(&stamped[..16]).ok()?;
        let client_instance_id = uuid::Uuid::from_slice(&stamped[16..32]).ok()?;
        Some(Self {
            class: LiveFrameClass::from_byte(*class_byte)?,
            authenticated_sender: Some(LiveRelayIdentity {
                principal_id,
                client_instance_id,
            }),
            payload: stamped[32..].to_vec(),
        })
    }
}

/// Duplex port to one relay connection, delivered when the socket attaches.
/// A reconnection delivers a fresh port that supersedes any earlier one;
/// sends into a superseded port are silently dropped with its dead socket.
pub(crate) struct LiveRelayPort {
    /// This install's stable per-run connection identity, echoed by the
    /// session tickets; payload protocols stamp it on outbound messages.
    pub client_instance_id: uuid::Uuid,
    /// Frames to broadcast to the other participants.
    pub outbound: tokio::sync::mpsc::Sender<LiveFrame>,
    /// Frames the other participants broadcast, in arrival order.
    pub inbound: std::sync::mpsc::Receiver<LiveFrame>,
}

/// Lifecycle of the live session, dialog- and chrome-rendered.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LiveSessionState {
    /// Going live: the creation is in flight.
    Starting,
    /// This install hosts the session.
    Hosting(LiveSessionSummary),
    /// A join by code is in flight.
    Joining,
    /// Joined, pending the host's approval; the roster poll watches for it.
    AwaitingApproval(LiveSessionSummary),
    /// Joined and admitted as a guest.
    Participating(LiveSessionSummary),
    /// The session could not be started or joined, or it ended from the
    /// server side; presentation-safe.
    Failed { message: String },
}

/// Everything the UI may render about the cloud session. Carries no secrets.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CloudSessionSnapshot {
    pub phase: CloudSessionPhase,
    pub principal: Option<PrincipalSummary>,
    pub entitlements: Vec<EntitlementSummary>,
    pub workspaces: Vec<WorkspaceSummary>,
    pub native_license: Option<NativeLicenseSummary>,
    pub device_leases: Vec<LeaseSummary>,
    /// The publish currently in flight or just finished, dialog-rendered.
    pub publish: Option<PublishState>,
    /// Publications of the project's bound circuit (Published band).
    pub publications: Vec<PublicationSummary>,
    /// The live session this install hosts or participates in, if any.
    pub live_session: Option<LiveSessionState>,
    /// RFC 3339 stamp of the last successful server contact this run.
    pub verified_at: Option<String>,
    /// Sign-in URL while [`CloudSessionPhase::WaitingForBrowser`], so the
    /// dialog can offer "open the sign-in page again".
    pub authorization_url: Option<String>,
}

impl Default for CloudSessionSnapshot {
    fn default() -> Self {
        Self {
            phase: CloudSessionPhase::SignedOut { last_error: None },
            principal: None,
            entitlements: Vec::new(),
            workspaces: Vec::new(),
            native_license: None,
            device_leases: Vec::new(),
            publish: None,
            publications: Vec::new(),
            live_session: None,
            verified_at: None,
            authorization_url: None,
        }
    }
}

impl CloudSessionSnapshot {
    /// Whether the active entitlements grant `feature` explicitly. Checks
    /// the server projection first, then the verified native lease (the
    /// offline authority). Absence is denial.
    pub(crate) fn cloud_feature_enabled(&self, feature: &str) -> bool {
        if !self.signed_in() {
            return false;
        }
        let now_active = |summary: &EntitlementSummary| {
            summary.status == "active" || summary.status == "grace_period"
        };
        if self
            .entitlements
            .iter()
            .filter(|summary| now_active(summary))
            .any(|summary| summary.granted_features.iter().any(|name| name == feature))
        {
            return true;
        }
        self.native_license
            .as_ref()
            .is_some_and(|license| license.granted_features.iter().any(|name| name == feature))
    }
}

impl CloudSessionSnapshot {
    /// Whether any server-verified or offline-verified session exists.
    pub(crate) fn signed_in(&self) -> bool {
        matches!(
            self.phase,
            CloudSessionPhase::Active | CloudSessionPhase::OfflineLicensed
        )
    }
}

/// Commands the UI can issue. Everything runs on the background executor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CloudAccountCommand {
    /// Begin interactive sign-in (discovery, loopback listener, browser).
    SignIn,
    /// Abandon a sign-in that is waiting on the browser.
    CancelSignIn,
    /// Drop the session: best-effort server-side revocation, then local wipe.
    SignOut,
    /// Re-run the bootstrap reads against the server.
    Refresh,
    /// Revoke one license lease (a device session) by its public ID.
    RevokeLease { lease_id: String },
    /// Publish a snapshot to a `/c/` page (dialog-confirmed).
    PublishSnapshot { request: Box<PublishRequest> },
    /// Re-read the bound circuit's publications (page status, band rows).
    RefreshPublications { circuit_id: String },
    /// Soft-unpublish one publication (tombstone; slug never reissued).
    Unpublish {
        circuit_id: String,
        publication_id: String,
    },
    /// Go live: create a session under the dialog-confirmed policy.
    StartLiveSession {
        policy: LiveSessionPolicySummary,
        /// Cloud circuit binding for provenance, when the project has one.
        circuit_id: Option<String>,
    },
    /// Rotate the join code. The old code dies for anyone who has not yet
    /// joined; current participants stay in the roster.
    RegenerateLiveSessionCode,
    /// Join a session by its human-relayable code.
    JoinLiveSession { code: String },
    /// Re-read the roster and admission state now.
    RefreshLiveSession,
    /// Apply a revised session policy (host only).
    ApplyLiveSessionPolicy { policy: LiveSessionPolicySummary },
    /// Admit a pending participant (host only).
    ApproveLiveSessionParticipant { principal_id: String },
    /// Make one participant an editor or a viewer (host only).
    SetLiveSessionParticipantEditor { principal_id: String, editor: bool },
    /// Remove one participant; the code never readmits them (host only).
    RemoveLiveSessionParticipant { principal_id: String },
    /// End the hosted session for everyone, instantly.
    EndLiveSession,
    /// Leave a joined session locally. Guests hold no end authority and the
    /// roster keeps their row; presence leaves with the relay connection.
    LeaveLiveSession,
    /// Authorization code + state delivered by the loopback listener.
    #[cfg(not(target_arch = "wasm32"))]
    CompleteSignIn { code: String, state: String },
    /// The loopback listener failed or timed out.
    #[cfg(not(target_arch = "wasm32"))]
    SignInFailed { reason: String },
    /// The relay socket attached (sent by the socket thread, never the UI).
    LiveRelayAttached { generation: u64 },
    /// The relay socket ended (sent by the socket thread, never the UI).
    LiveRelayClosed {
        generation: u64,
        closure: LiveRelayClosure,
    },
}

/// Why a relay socket ended, as the socket thread reports it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LiveRelayClosure {
    /// The executor asked the socket to stop; no state change follows.
    Local,
    /// The server declared the session over, with the presentation-safe
    /// reading of its close reason.
    SessionOver { message: String },
    /// The server refused a frame or broke the handshake contract — a
    /// defect, so reconnecting would loop. Fail visible instead.
    Rejected,
    /// The connection dropped (network, relay restart); reconnectable.
    Interrupted,
}

/// Events the executor posts back to the UI thread.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CloudAccountEvent {
    /// Replace the UI's snapshot wholesale.
    Snapshot(Box<CloudSessionSnapshot>),
    /// Ask the UI to open the sign-in page in the system browser.
    OpenBrowser(String),
    /// A publish committed: the UI persists this binding into the project.
    PublishCompleted(Box<PublishReceipt>),
}

/// Fixed-size, coalescing executor-to-UI mailbox. Only the newest value of
/// each event class can be useful to the application: snapshots supersede
/// snapshots, a later browser request replaces an unopened predecessor, and
/// the service itself exposes only the newest unconsumed publication receipt.
/// Keeping those three slots here prevents a paused/minimized UI from growing
/// an unbounded cross-thread queue.
#[derive(Default)]
struct CloudAccountEventSlots {
    snapshot: Option<Box<CloudSessionSnapshot>>,
    browser_url: Option<String>,
    publish_receipt: Option<Box<PublishReceipt>>,
}

#[derive(Clone)]
pub(super) struct CloudAccountEventSender {
    slots: std::sync::Arc<std::sync::Mutex<CloudAccountEventSlots>>,
}

pub(super) struct CloudAccountEventReceiver {
    slots: std::sync::Arc<std::sync::Mutex<CloudAccountEventSlots>>,
}

pub(super) fn cloud_account_event_mailbox() -> (CloudAccountEventSender, CloudAccountEventReceiver)
{
    let slots = std::sync::Arc::new(std::sync::Mutex::new(CloudAccountEventSlots::default()));
    (
        CloudAccountEventSender {
            slots: std::sync::Arc::clone(&slots),
        },
        CloudAccountEventReceiver { slots },
    )
}

impl CloudAccountEventSender {
    pub(super) fn send(&self, event: CloudAccountEvent) -> Result<(), ()> {
        let mut slots = self
            .slots
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        match event {
            CloudAccountEvent::Snapshot(snapshot) => slots.snapshot = Some(snapshot),
            CloudAccountEvent::OpenBrowser(url) => slots.browser_url = Some(url),
            CloudAccountEvent::PublishCompleted(receipt) => {
                slots.publish_receipt = Some(receipt);
            }
        }
        Ok(())
    }
}

impl CloudAccountEventReceiver {
    fn try_recv(&self) -> Result<CloudAccountEvent, std::sync::mpsc::TryRecvError> {
        let mut slots = self
            .slots
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(snapshot) = slots.snapshot.take() {
            return Ok(CloudAccountEvent::Snapshot(snapshot));
        }
        if let Some(url) = slots.browser_url.take() {
            return Ok(CloudAccountEvent::OpenBrowser(url));
        }
        if let Some(receipt) = slots.publish_receipt.take() {
            return Ok(CloudAccountEvent::PublishCompleted(receipt));
        }
        Err(std::sync::mpsc::TryRecvError::Empty)
    }
}

struct LiveRelayPortSlot {
    value: Option<LiveRelayPort>,
}

#[derive(Clone)]
pub(super) struct LiveRelayPortSender {
    slot: std::sync::Arc<std::sync::Mutex<LiveRelayPortSlot>>,
}

pub(super) struct LiveRelayPortReceiver {
    slot: std::sync::Arc<std::sync::Mutex<LiveRelayPortSlot>>,
}

pub(super) fn live_relay_port_mailbox() -> (LiveRelayPortSender, LiveRelayPortReceiver) {
    let slot = std::sync::Arc::new(std::sync::Mutex::new(LiveRelayPortSlot { value: None }));
    (
        LiveRelayPortSender {
            slot: std::sync::Arc::clone(&slot),
        },
        LiveRelayPortReceiver { slot },
    )
}

impl LiveRelayPortSender {
    pub(super) fn send(&self, port: LiveRelayPort) -> Result<(), ()> {
        self.slot
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .value = Some(port);
        Ok(())
    }
}

impl LiveRelayPortReceiver {
    fn try_recv(&self) -> Result<LiveRelayPort, std::sync::mpsc::TryRecvError> {
        self.slot
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .value
            .take()
            .ok_or(std::sync::mpsc::TryRecvError::Empty)
    }
}

const MAX_PENDING_CLOUD_COMMANDS: usize = 8;

/// The application-facing service. Owns the executor and the latest snapshot.
pub(crate) struct CloudAccountService {
    availability: CloudAccountAvailability,
    snapshot: CloudSessionSnapshot,
    pending_browser_url: Option<String>,
    pending_publish_receipt: Option<PublishReceipt>,
    /// The newest relay port awaiting pickup by the workbench.
    pending_live_relay: Option<LiveRelayPort>,
    /// A presentation-safe local service error awaiting the application
    /// message center. It never replaces a still-valid authenticated snapshot.
    pending_local_error: Option<String>,
    pending_commands: std::collections::VecDeque<CloudAccountCommand>,
    #[cfg(target_arch = "wasm32")]
    commands: Option<tokio::sync::mpsc::Sender<CloudAccountCommand>>,
    #[cfg(not(target_arch = "wasm32"))]
    commands: Option<std::sync::mpsc::SyncSender<CloudAccountCommand>>,
    events: Option<CloudAccountEventReceiver>,
    relay_ports: Option<LiveRelayPortReceiver>,
}

impl CloudAccountService {
    /// Resolve configuration and, when this build can operate a session,
    /// start the background executor (which restores any stored session).
    pub(crate) fn new(repaint: Option<egui::Context>) -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            match config::CloudAccountConfig::resolve() {
                Some(configuration) => {
                    let (commands, events, relay_ports) =
                        browser_executor::spawn(configuration, repaint);
                    Self {
                        availability: CloudAccountAvailability::Browser,
                        snapshot: CloudSessionSnapshot::default(),
                        pending_browser_url: None,
                        pending_publish_receipt: None,
                        pending_live_relay: None,
                        pending_local_error: None,
                        pending_commands: std::collections::VecDeque::new(),
                        commands: Some(commands),
                        events: Some(events),
                        relay_ports: Some(relay_ports),
                    }
                }
                None => Self {
                    availability: CloudAccountAvailability::UnconfiguredBuild,
                    snapshot: CloudSessionSnapshot::default(),
                    pending_browser_url: None,
                    pending_publish_receipt: None,
                    pending_live_relay: None,
                    pending_local_error: None,
                    pending_commands: std::collections::VecDeque::new(),
                    commands: None,
                    events: None,
                    relay_ports: None,
                },
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            match config::CloudAccountConfig::resolve() {
                Some(configuration) => {
                    let (commands, events, relay_ports) = executor::spawn(configuration, repaint);
                    Self {
                        availability: CloudAccountAvailability::Native,
                        snapshot: CloudSessionSnapshot::default(),
                        pending_browser_url: None,
                        pending_publish_receipt: None,
                        pending_live_relay: None,
                        pending_local_error: None,
                        pending_commands: std::collections::VecDeque::new(),
                        commands: Some(commands),
                        events: Some(events),
                        relay_ports: Some(relay_ports),
                    }
                }
                None => Self {
                    availability: CloudAccountAvailability::UnconfiguredBuild,
                    snapshot: CloudSessionSnapshot::default(),
                    pending_browser_url: None,
                    pending_publish_receipt: None,
                    pending_live_relay: None,
                    pending_local_error: None,
                    pending_commands: std::collections::VecDeque::new(),
                    commands: None,
                    events: None,
                    relay_ports: None,
                },
            }
        }
    }

    /// A service that never operates a session, so test instances start no
    /// thread and touch no network.
    #[cfg(test)]
    pub(crate) fn unconfigured() -> Self {
        Self {
            availability: CloudAccountAvailability::UnconfiguredBuild,
            snapshot: CloudSessionSnapshot::default(),
            pending_browser_url: None,
            pending_publish_receipt: None,
            pending_live_relay: None,
            pending_local_error: None,
            pending_commands: std::collections::VecDeque::new(),
            commands: None,
            events: None,
            relay_ports: None,
        }
    }

    pub(crate) fn availability(&self) -> CloudAccountAvailability {
        self.availability
    }

    pub(crate) fn snapshot(&self) -> &CloudSessionSnapshot {
        &self.snapshot
    }

    /// Drain executor events into the snapshot. Returns whether it changed.
    pub(crate) fn poll(&mut self) -> bool {
        let mut changed = false;
        self.flush_pending_commands();
        if let Some(ports) = self.relay_ports.as_ref() {
            // Newest wins: a reconnection's port supersedes its predecessor
            // before the workbench ever saw it.
            while let Ok(port) = ports.try_recv() {
                self.pending_live_relay = Some(port);
                changed = true;
            }
        }
        let Some(events) = self.events.as_ref() else {
            return changed;
        };
        while let Ok(event) = events.try_recv() {
            match event {
                CloudAccountEvent::Snapshot(snapshot) => {
                    self.snapshot = *snapshot;
                    changed = true;
                }
                CloudAccountEvent::OpenBrowser(url) => {
                    self.pending_browser_url = Some(url);
                    changed = true;
                }
                CloudAccountEvent::PublishCompleted(receipt) => {
                    self.pending_publish_receipt = Some(*receipt);
                    changed = true;
                }
            }
        }
        changed
    }

    /// A sign-in URL the UI should open in the system browser, at most once.
    pub(crate) fn take_browser_request(&mut self) -> Option<String> {
        self.pending_browser_url.take()
    }

    pub(crate) fn take_local_error(&mut self) -> Option<String> {
        self.pending_local_error.take()
    }

    pub(crate) fn sign_in(&mut self) {
        self.send(CloudAccountCommand::SignIn);
    }

    /// Queue the pending sign-in page to open again (lost browser window).
    pub(crate) fn reopen_sign_in_page(&mut self) {
        if let Some(url) = self.snapshot.authorization_url.clone() {
            self.pending_browser_url = Some(url);
        }
    }

    pub(crate) fn cancel_sign_in(&mut self) {
        self.send(CloudAccountCommand::CancelSignIn);
    }

    pub(crate) fn sign_out(&mut self) {
        self.send(CloudAccountCommand::SignOut);
    }

    pub(crate) fn refresh(&mut self) {
        self.send(CloudAccountCommand::Refresh);
    }

    pub(crate) fn revoke_lease(&mut self, lease_id: String) {
        self.send(CloudAccountCommand::RevokeLease { lease_id });
    }

    /// Publish a snapshot to a `/c/` page; progress arrives via the snapshot.
    pub(crate) fn publish_snapshot(&mut self, request: PublishRequest) {
        self.send(CloudAccountCommand::PublishSnapshot {
            request: Box::new(request),
        });
    }

    /// Re-read the bound circuit's publications (Published band, page status).
    pub(crate) fn refresh_publications(&mut self, circuit_id: String) {
        self.send(CloudAccountCommand::RefreshPublications { circuit_id });
    }

    /// Soft-unpublish one publication of the bound circuit.
    pub(crate) fn unpublish(&mut self, circuit_id: String, publication_id: String) {
        self.send(CloudAccountCommand::Unpublish {
            circuit_id,
            publication_id,
        });
    }

    /// Go live: create a hosted session under the dialog-confirmed policy.
    pub(crate) fn start_live_session(
        &mut self,
        policy: LiveSessionPolicySummary,
        circuit_id: Option<String>,
    ) {
        self.send(CloudAccountCommand::StartLiveSession { policy, circuit_id });
    }

    /// Rotate the join code; current participants stay connected.
    pub(crate) fn regenerate_live_session_code(&mut self) {
        self.send(CloudAccountCommand::RegenerateLiveSessionCode);
    }

    pub(crate) fn join_live_session(&mut self, code: String) {
        self.send(CloudAccountCommand::JoinLiveSession { code });
    }

    /// Re-read the roster; also the user's way back from a blocked relay.
    pub(crate) fn refresh_live_session(&mut self) {
        self.send(CloudAccountCommand::RefreshLiveSession);
    }

    pub(crate) fn apply_live_session_policy(&mut self, policy: LiveSessionPolicySummary) {
        self.send(CloudAccountCommand::ApplyLiveSessionPolicy { policy });
    }

    pub(crate) fn approve_live_session_participant(&mut self, principal_id: String) {
        self.send(CloudAccountCommand::ApproveLiveSessionParticipant { principal_id });
    }

    pub(crate) fn set_live_session_participant_editor(
        &mut self,
        principal_id: String,
        editor: bool,
    ) {
        self.send(CloudAccountCommand::SetLiveSessionParticipantEditor {
            principal_id,
            editor,
        });
    }

    pub(crate) fn remove_live_session_participant(&mut self, principal_id: String) {
        self.send(CloudAccountCommand::RemoveLiveSessionParticipant { principal_id });
    }

    /// End the hosted session for everyone, instantly.
    pub(crate) fn end_live_session(&mut self) {
        self.send(CloudAccountCommand::EndLiveSession);
    }

    /// Leave a joined session locally; the roster row remains until it ends.
    pub(crate) fn leave_live_session(&mut self) {
        self.send(CloudAccountCommand::LeaveLiveSession);
    }

    /// The receipt of a just-committed publish, at most once. The caller
    /// persists it into the project descriptor.
    pub(crate) fn take_publish_receipt(&mut self) -> Option<PublishReceipt> {
        self.pending_publish_receipt.take()
    }

    /// The duplex port of a just-attached relay connection, at most once per
    /// connection. The workbench pumps live frames through it and replaces
    /// any port it already holds: a fresh port means the old socket is dead.
    pub(crate) fn take_live_relay_port(&mut self) -> Option<LiveRelayPort> {
        self.pending_live_relay.take()
    }

    fn send(&mut self, command: CloudAccountCommand) {
        // Once one action is deferred, every later action must join the same
        // FIFO. Sending a newer action straight to newly available executor
        // capacity would let it overtake an earlier start/end, admission, or
        // policy mutation before the next UI poll flushes the backlog.
        if !self.pending_commands.is_empty() {
            self.defer_command(command);
            return;
        }
        #[cfg(target_arch = "wasm32")]
        if let Some(commands) = self.commands.as_ref() {
            match commands.try_send(command) {
                Ok(()) => {}
                Err(tokio::sync::mpsc::error::TrySendError::Full(command)) => {
                    self.defer_command(command);
                }
                Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => {
                    self.snapshot = CloudSessionSnapshot {
                        phase: CloudSessionPhase::SignedOut {
                            last_error: Some(
                                "The cloud session service stopped; reload RSpice to sign in."
                                    .to_owned(),
                            ),
                        },
                        ..CloudSessionSnapshot::default()
                    };
                    self.commands = None;
                    self.events = None;
                    self.relay_ports = None;
                    self.pending_commands.clear();
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(commands) = self.commands.as_ref() {
            match commands.try_send(command) {
                Ok(()) => {}
                Err(std::sync::mpsc::TrySendError::Full(command)) => {
                    self.defer_command(command);
                }
                Err(std::sync::mpsc::TrySendError::Disconnected(_)) => {
                    self.snapshot = CloudSessionSnapshot {
                        phase: CloudSessionPhase::SignedOut {
                            last_error: Some(
                                "The cloud session service stopped; restart RSpice to sign in."
                                    .to_owned(),
                            ),
                        },
                        ..CloudSessionSnapshot::default()
                    };
                    self.commands = None;
                    self.events = None;
                    self.relay_ports = None;
                    self.pending_commands.clear();
                }
            }
        }
    }

    fn defer_command(&mut self, command: CloudAccountCommand) {
        if self.pending_commands.len() < MAX_PENDING_CLOUD_COMMANDS {
            self.pending_commands.push_back(command);
        } else {
            self.pending_local_error = Some(
                "The cloud service is busy and could not accept that action. Wait a moment and retry."
                    .to_owned(),
            );
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn flush_pending_commands(&mut self) {
        let Some(commands) = self.commands.as_ref() else {
            self.pending_commands.clear();
            return;
        };
        while let Some(command) = self.pending_commands.pop_front() {
            match commands.try_send(command) {
                Ok(()) => {}
                Err(tokio::sync::mpsc::error::TrySendError::Full(command)) => {
                    self.pending_commands.push_front(command);
                    return;
                }
                Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => {
                    self.pending_commands.clear();
                    self.snapshot = CloudSessionSnapshot {
                        phase: CloudSessionPhase::SignedOut {
                            last_error: Some(
                                "The cloud session service stopped; reload RSpice to sign in."
                                    .to_owned(),
                            ),
                        },
                        ..CloudSessionSnapshot::default()
                    };
                    self.commands = None;
                    self.events = None;
                    self.relay_ports = None;
                    return;
                }
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn flush_pending_commands(&mut self) {
        let Some(commands) = self.commands.as_ref() else {
            self.pending_commands.clear();
            return;
        };
        while let Some(command) = self.pending_commands.pop_front() {
            match commands.try_send(command) {
                Ok(()) => {}
                Err(std::sync::mpsc::TrySendError::Full(command)) => {
                    self.pending_commands.push_front(command);
                    return;
                }
                Err(std::sync::mpsc::TrySendError::Disconnected(_)) => {
                    self.pending_commands.clear();
                    self.snapshot = CloudSessionSnapshot {
                        phase: CloudSessionPhase::SignedOut {
                            last_error: Some(
                                "The cloud session service stopped; restart RSpice to sign in."
                                    .to_owned(),
                            ),
                        },
                        ..CloudSessionSnapshot::default()
                    };
                    self.commands = None;
                    self.events = None;
                    self.relay_ports = None;
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executor_mailboxes_coalesce_to_finite_latest_state() {
        let (events, event_receiver) = cloud_account_event_mailbox();
        let mut first = CloudSessionSnapshot::default();
        first.phase = CloudSessionPhase::WaitingForBrowser;
        let mut latest = CloudSessionSnapshot::default();
        latest.phase = CloudSessionPhase::Active;
        events
            .send(CloudAccountEvent::Snapshot(Box::new(first)))
            .expect("mailbox remains available");
        events
            .send(CloudAccountEvent::Snapshot(Box::new(latest.clone())))
            .expect("mailbox remains available");
        events
            .send(CloudAccountEvent::OpenBrowser(
                "https://old.invalid".to_owned(),
            ))
            .expect("mailbox remains available");
        events
            .send(CloudAccountEvent::OpenBrowser(
                "https://identity.example/authorize".to_owned(),
            ))
            .expect("mailbox remains available");

        assert_eq!(
            event_receiver.try_recv(),
            Ok(CloudAccountEvent::Snapshot(Box::new(latest)))
        );
        assert_eq!(
            event_receiver.try_recv(),
            Ok(CloudAccountEvent::OpenBrowser(
                "https://identity.example/authorize".to_owned()
            ))
        );
        assert_eq!(
            event_receiver.try_recv(),
            Err(std::sync::mpsc::TryRecvError::Empty)
        );

        let (ports, port_receiver) = live_relay_port_mailbox();
        for client_instance_id in [uuid::Uuid::from_u128(1), uuid::Uuid::from_u128(2)] {
            let (outbound, _outbound_receiver) = tokio::sync::mpsc::channel(1);
            let (_inbound_sender, inbound) = std::sync::mpsc::channel();
            ports
                .send(LiveRelayPort {
                    client_instance_id,
                    outbound,
                    inbound,
                })
                .expect("mailbox remains available");
        }
        assert_eq!(
            port_receiver
                .try_recv()
                .expect("latest relay port")
                .client_instance_id,
            uuid::Uuid::from_u128(2)
        );
        assert!(port_receiver.try_recv().is_err());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_ui_command_backpressure_is_finite_and_nonblocking() {
        let (commands, receiver) = std::sync::mpsc::sync_channel(1);
        let mut service = CloudAccountService::unconfigured();
        service.commands = Some(commands);

        service.refresh();
        for _ in 0..MAX_PENDING_CLOUD_COMMANDS {
            service.refresh();
        }
        assert_eq!(service.pending_commands.len(), MAX_PENDING_CLOUD_COMMANDS);
        assert!(service.pending_local_error.is_none());

        service.refresh();
        assert_eq!(service.pending_commands.len(), MAX_PENDING_CLOUD_COMMANDS);
        assert!(service.pending_local_error.is_some());

        assert!(matches!(
            receiver.try_recv(),
            Ok(CloudAccountCommand::Refresh)
        ));
        service.sign_out();
        assert!(matches!(
            receiver.try_recv(),
            Err(std::sync::mpsc::TryRecvError::Empty)
        ));
        assert_eq!(service.pending_commands.len(), MAX_PENDING_CLOUD_COMMANDS);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn deferred_ui_commands_preserve_fifo_order_when_executor_capacity_reopens() {
        let (commands, receiver) = std::sync::mpsc::sync_channel(1);
        let mut service = CloudAccountService::unconfigured();
        service.commands = Some(commands);

        service.refresh();
        service.refresh();
        assert!(matches!(
            receiver.try_recv(),
            Ok(CloudAccountCommand::Refresh)
        ));

        // Capacity is available again, but SignOut must not jump ahead of the
        // Refresh already waiting in the service FIFO.
        service.sign_out();
        assert!(matches!(
            receiver.try_recv(),
            Err(std::sync::mpsc::TryRecvError::Empty)
        ));

        service.poll();
        assert!(matches!(
            receiver.try_recv(),
            Ok(CloudAccountCommand::Refresh)
        ));
        service.poll();
        assert!(matches!(
            receiver.try_recv(),
            Ok(CloudAccountCommand::SignOut)
        ));
    }

    #[test]
    fn signed_in_covers_exactly_the_verified_phases() {
        let mut snapshot = CloudSessionSnapshot::default();
        assert!(!snapshot.signed_in());
        snapshot.phase = CloudSessionPhase::Bootstrapping;
        assert!(!snapshot.signed_in());
        snapshot.phase = CloudSessionPhase::Active;
        assert!(snapshot.signed_in());
        snapshot.phase = CloudSessionPhase::OfflineLicensed;
        assert!(snapshot.signed_in());
    }

    #[test]
    fn live_frames_encode_outbound_and_decode_ticket_stamped_identity() {
        let principal_id = uuid::Uuid::from_u128(11);
        let client_instance_id = uuid::Uuid::from_u128(12);
        for (class, byte) in [
            (LiveFrameClass::Presence, 0u8),
            (LiveFrameClass::Cursor, 1),
            (LiveFrameClass::Document, 2),
            (LiveFrameClass::RunRequest, 3),
            (LiveFrameClass::RunStatus, 4),
        ] {
            let frame = LiveFrame {
                class,
                authenticated_sender: None,
                payload: vec![9, 8, 7],
            };
            let encoded = frame.encode();
            assert_eq!(encoded[0], byte);
            let mut delivered = vec![byte];
            delivered.extend_from_slice(principal_id.as_bytes());
            delivered.extend_from_slice(client_instance_id.as_bytes());
            delivered.extend_from_slice(&[9, 8, 7]);
            let decoded = LiveFrame::decode_authenticated(&delivered).expect("stamped frame");
            assert_eq!(decoded.class, class);
            assert_eq!(decoded.payload, vec![9, 8, 7]);
            assert_eq!(
                decoded.authenticated_sender,
                Some(LiveRelayIdentity {
                    principal_id,
                    client_instance_id,
                })
            );
        }
        assert_eq!(LiveFrame::decode_authenticated(&[]), None, "no class byte");
        assert_eq!(
            LiveFrame::decode_authenticated(&[0; 32]),
            None,
            "truncated identity stamp"
        );
    }

    #[test]
    fn client_retry_is_capped_deterministic_and_client_jittered() {
        let first = uuid::Uuid::from_u128(11);
        let second = uuid::Uuid::from_u128(12);
        for attempt in 0..16 {
            let delay = client_retry_delay(attempt, first);
            let ceiling = std::time::Duration::from_millis(
                1_000_u64
                    .saturating_mul(1_u64 << attempt.min(5))
                    .min(30_000),
            );
            assert!(delay >= ceiling / 2 && delay <= ceiling);
            assert_eq!(delay, client_retry_delay(attempt, first));
        }
        assert_ne!(
            client_retry_delay(4, first),
            client_retry_delay(4, second),
            "different clients should not synchronize their fourth reconnect"
        );
    }

    /// The UI-facing class enum must stay byte-for-byte the relay contract's.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn live_frame_classes_match_the_relay_contract() {
        use rspice_cloud_client::contract::LiveSessionFrameClass;
        for (ours, contract) in [
            (LiveFrameClass::Presence, LiveSessionFrameClass::Presence),
            (LiveFrameClass::Cursor, LiveSessionFrameClass::Cursor),
            (LiveFrameClass::Document, LiveSessionFrameClass::Document),
            (
                LiveFrameClass::RunRequest,
                LiveSessionFrameClass::RunRequest,
            ),
            (LiveFrameClass::RunStatus, LiveSessionFrameClass::RunStatus),
        ] {
            assert_eq!(ours.as_byte(), contract as u8);
            assert_eq!(LiveFrameClass::from_byte(contract as u8), Some(ours));
        }
        assert_eq!(LiveSessionFrameClass::from_byte(5), None);
        assert_eq!(LiveFrameClass::from_byte(5), None);
    }

    #[test]
    fn unconfigured_service_accepts_commands_without_a_session() {
        let mut service = CloudAccountService::unconfigured();
        service.sign_in();
        service.refresh();
        assert!(!service.poll());
        assert_eq!(
            service.availability(),
            CloudAccountAvailability::UnconfiguredBuild
        );
        assert_eq!(
            service.snapshot().phase,
            CloudSessionPhase::SignedOut { last_error: None }
        );
    }
}
