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
//! - browser builds report [`CloudAccountAvailability::BrowserPending`] until
//!   the hosted deployment carries the sign-in callback page — server-side
//!   entitlement enforcement already covers every browser operation, so no
//!   offline lease is missing there.

pub(crate) mod config;
#[cfg(not(target_arch = "wasm32"))]
mod executor;
#[cfg(not(target_arch = "wasm32"))]
mod live_relay;
#[cfg(not(target_arch = "wasm32"))]
mod native_login;
#[cfg(not(target_arch = "wasm32"))]
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
    /// This build was produced without pinned cloud endpoints, so every cloud
    /// account surface reports the boundary instead of offering actions.
    UnconfiguredBuild,
    /// Browser build: sign-in arrives with the hosted deployment's callback
    /// page; until then the boundary is reported explicitly.
    // Constructed only by the wasm arm of `CloudAccountService::new`; native
    // builds match on it without ever constructing it.
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    BrowserPending,
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

/// One live-relay frame: a class byte followed by an opaque payload the
/// relay never interprets. Payload protocols are peer-owned.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LiveFrame {
    pub class: LiveFrameClass,
    pub payload: Vec<u8>,
}

impl LiveFrame {
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(1 + self.payload.len());
        bytes.push(self.class.as_byte());
        bytes.extend_from_slice(&self.payload);
        bytes
    }

    pub(crate) fn decode(bytes: &[u8]) -> Option<Self> {
        let (class_byte, payload) = bytes.split_first()?;
        Some(Self {
            class: LiveFrameClass::from_byte(*class_byte)?,
            payload: payload.to_vec(),
        })
    }
}

/// Duplex port to one relay connection, delivered when the socket attaches.
/// A reconnection delivers a fresh port that supersedes any earlier one;
/// sends into a superseded port are silently dropped with its dead socket.
pub(crate) struct LiveRelayPort {
    /// Monotonic connection identity, so stale ports are recognizable.
    pub generation: u64,
    /// This install's stable per-run connection identity, echoed by the
    /// session tickets; payload protocols stamp it on outbound messages.
    pub client_instance_id: uuid::Uuid,
    /// Frames to broadcast to the other participants.
    pub outbound: tokio::sync::mpsc::UnboundedSender<LiveFrame>,
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
    #[cfg(not(target_arch = "wasm32"))]
    LiveRelayAttached { generation: u64 },
    /// The relay socket ended (sent by the socket thread, never the UI).
    #[cfg(not(target_arch = "wasm32"))]
    LiveRelayClosed {
        generation: u64,
        closure: LiveRelayClosure,
    },
}

/// Why a relay socket ended, as the socket thread reports it.
#[cfg(not(target_arch = "wasm32"))]
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

/// The application-facing service. Owns the executor and the latest snapshot.
pub(crate) struct CloudAccountService {
    availability: CloudAccountAvailability,
    snapshot: CloudSessionSnapshot,
    pending_browser_url: Option<String>,
    pending_publish_receipt: Option<PublishReceipt>,
    /// The newest relay port awaiting pickup by the workbench.
    pending_live_relay: Option<LiveRelayPort>,
    #[cfg(not(target_arch = "wasm32"))]
    commands: Option<std::sync::mpsc::Sender<CloudAccountCommand>>,
    #[cfg(not(target_arch = "wasm32"))]
    events: Option<std::sync::mpsc::Receiver<CloudAccountEvent>>,
    #[cfg(not(target_arch = "wasm32"))]
    relay_ports: Option<std::sync::mpsc::Receiver<LiveRelayPort>>,
}

impl CloudAccountService {
    /// Resolve configuration and, when this build can operate a session,
    /// start the background executor (which restores any stored session).
    pub(crate) fn new(repaint: Option<egui::Context>) -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = repaint;
            Self {
                availability: CloudAccountAvailability::BrowserPending,
                snapshot: CloudSessionSnapshot::default(),
                pending_browser_url: None,
                pending_publish_receipt: None,
                pending_live_relay: None,
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
            #[cfg(not(target_arch = "wasm32"))]
            commands: None,
            #[cfg(not(target_arch = "wasm32"))]
            events: None,
            #[cfg(not(target_arch = "wasm32"))]
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
        #[cfg(target_arch = "wasm32")]
        {
            false
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut changed = false;
            if let Some(ports) = self.relay_ports.as_ref() {
                // Newest wins: a reconnection's port supersedes its
                // predecessor before the workbench ever saw it.
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
    }

    /// A sign-in URL the UI should open in the system browser, at most once.
    pub(crate) fn take_browser_request(&mut self) -> Option<String> {
        self.pending_browser_url.take()
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

    #[allow(unused_variables)]
    fn send(&mut self, command: CloudAccountCommand) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(commands) = self.commands.as_ref() {
            // A send failure means the executor thread is gone; surface that
            // as a signed-out session rather than silently dropping input.
            if commands.send(command).is_err() {
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
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn live_frames_round_trip_their_class_byte() {
        for (class, byte) in [
            (LiveFrameClass::Presence, 0u8),
            (LiveFrameClass::Cursor, 1),
            (LiveFrameClass::Document, 2),
            (LiveFrameClass::RunRequest, 3),
            (LiveFrameClass::RunStatus, 4),
        ] {
            let frame = LiveFrame {
                class,
                payload: vec![9, 8, 7],
            };
            let encoded = frame.encode();
            assert_eq!(encoded[0], byte);
            assert_eq!(LiveFrame::decode(&encoded), Some(frame));
        }
        assert_eq!(LiveFrame::decode(&[]), None, "no class byte");
        assert_eq!(LiveFrame::decode(&[7]), None, "unknown class");
        let empty = LiveFrame::decode(&[3]).expect("class byte alone is a frame");
        assert_eq!(empty.class, LiveFrameClass::RunRequest);
        assert!(empty.payload.is_empty());
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
            (LiveFrameClass::RunRequest, LiveSessionFrameClass::RunRequest),
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
