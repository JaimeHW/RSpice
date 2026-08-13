//! Live-session engine: the workbench side of the relay.
//!
//! Pumped once per frame after the cloud service, this engine turns the
//! relay's opaque frame stream into the host-authoritative session model:
//! the host broadcasts whole documents at every commit boundary and
//! arbitrates per-document write leases; guests mirror the host's documents
//! and edit only what they hold a lease on. The relay socket, tickets, and
//! reconnection all belong to the cloud service — this engine only ever
//! sees `LiveRelayPort` values and rendered session snapshots.
//!
//! Authority rules, enforced here and backed by the relay's class matrix:
//! document replaces are applied from the session host, or — host-side —
//! from the participant holding that document's lease; everything else on
//! the document class is dropped. A guest's own edits leave as proposals
//! the host applies and rebroadcasts, with content digests recognizing the
//! echo so the leaseholder never fights its own edits.

use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

use crate::diagnostics::ConsoleMessage;
use crate::io::schematic_io::{load_schematic_text, serialize_schematic_for_wire};
use crate::services::cloud_account::{
    CloudAccountService, LiveFrame, LiveFrameClass, LiveRelayPort, LiveSessionState,
};
use crate::services::live_protocol::{
    CompletedContent, ContentReassembler, CursorLocus, CursorPayload, DocumentMessage,
    LeaseDecision, LeaseTable, LiveMessage, LiveProtocolError, ManifestEntry, PeerIdentity,
    PresencePayload, RunIntent, RunPhase, RunRequestPayload, RunStatusMessage, RunStatusPayload,
    content_digest, replace_messages,
};
use crate::state::SchematicState;
use crate::workbench::AppState;
use crate::workbench::state::LiveWriteLocks;

/// Presence cadence; doubles as the relay keepalive under its idle timeout.
const PRESENCE_PERIOD: Duration = Duration::from_secs(15);
/// Cursor traffic is ephemeral and coalesced to at most 20 updates/second.
const CURSOR_PERIOD: Duration = Duration::from_millis(50);
/// Drop peers whose presence went quiet (they left or lost the relay).
const PEER_TIMEOUT: Duration = Duration::from_secs(60);
/// Minimum progress-fraction movement worth a run-status frame.
const RUN_PROGRESS_STEP: f64 = 0.01;
const MAX_PENDING_RUN_REQUESTS: usize = 32;

/// Wire content contract of a schematic cell-view buffer.
const SCHEMATIC_CONTENT_TYPE: &str = "rspice.schematic-view.v1";
/// Wire content contract of the project-owned netlist source.
const NETLIST_CONTENT_TYPE: &str = "rspice.netlist-source.v1";
/// Wire content contract of the whole-project snapshot.
const PROJECT_CONTENT_TYPE: &str = "rspice.project.v1";
/// Wire key of the project's single owned netlist document.
const NETLIST_DOC: &str = "netlist";
/// Wire key of the whole-project snapshot. It carries everything a mirror
/// cannot reconstruct from per-document replaces — libraries, hierarchy,
/// execution context — and is sent only on sync, reconnect, and structural
/// change, never from the per-frame scan (results are stripped; guests run
/// the mirrored design locally).
const PROJECT_DOC: &str = "project";
/// Wire-key prefix of schematic buffers; the remainder is the cell-view key.
const SCHEMATIC_DOC_PREFIX: &str = "schematic/";

fn schematic_doc_key(cell_view_key: &str) -> String {
    format!("{SCHEMATIC_DOC_PREFIX}{cell_view_key}")
}

/// One participant as the session currently sees them, for presence chrome
/// and remote-cursor overlays.
pub(crate) struct PeerPresence {
    pub identity: PeerIdentity,
    pub focused_doc: Option<String>,
    pub cursor: Option<CursorLocus>,
    pub seen_at: Instant,
}

/// Broadcast bookkeeping for one document the host serves.
struct TrackedDoc {
    content_type: &'static str,
    /// The document's own change counter as last observed (schematic
    /// `content_version`, netlist `ObjectRevision`).
    seen_version: u64,
    /// This host's wire revision, monotonic per document.
    revision: u64,
    /// Digest of the content as last broadcast.
    digest: String,
}

#[derive(Default)]
struct RunTracker {
    active: bool,
    run_id: u64,
    last_fraction: f64,
    last_status: String,
}

struct HostRole {
    session_id: String,
    leases: LeaseTable,
    reassembler: ContentReassembler,
    tracked: HashMap<String, TrackedDoc>,
    run: RunTracker,
}

impl HostRole {
    fn new(session_id: String) -> Self {
        Self {
            session_id,
            leases: LeaseTable::default(),
            reassembler: ContentReassembler::default(),
            tracked: HashMap::new(),
            run: RunTracker::default(),
        }
    }
}

/// A document this guest holds the write lease on: proposal bookkeeping.
struct HeldDoc {
    proposal_revision: u64,
    seen_version: u64,
    digest: String,
}

/// A host document as last applied on this mirror.
struct MirrorDoc {
    content_type: String,
    digest: String,
}

struct GuestRole {
    session_id: String,
    /// The host's asserted wire identity, pinned by the roster's host row.
    host: Option<PeerIdentity>,
    leases: LeaseTable,
    reassembler: ContentReassembler,
    mirror_docs: HashMap<String, MirrorDoc>,
    held: HashMap<String, HeldDoc>,
    /// This install adopted the host's project as its mirror; until then
    /// inbound documents are tracked but never applied.
    mirroring: bool,
    /// The host's project snapshot has applied at least once; only the
    /// first application moves the user onto the project surface.
    project_synced: bool,
    /// A project snapshot that could not apply yet because a local run or
    /// lifecycle transaction owns the project; retried every pump.
    pending_project: Option<CompletedContent>,
    /// Session policy: whether this mirror may be kept as a saved copy.
    save_copy_allowed: bool,
    /// The freshest host run report, for the session status chrome.
    run_status: Option<RunStatusPayload>,
    /// The most recent lease refusal aimed at this guest: (doc, holder).
    denied: Option<(String, PeerIdentity)>,
}

impl GuestRole {
    fn new(session_id: String, save_copy_allowed: bool) -> Self {
        Self {
            session_id,
            host: None,
            leases: LeaseTable::default(),
            reassembler: ContentReassembler::default(),
            mirror_docs: HashMap::new(),
            held: HashMap::new(),
            mirroring: false,
            project_synced: false,
            pending_project: None,
            save_copy_allowed,
            run_status: None,
            denied: None,
        }
    }
}

enum Role {
    Idle,
    Host(Box<HostRole>),
    Guest(Box<GuestRole>),
}

struct Connection {
    port: LiveRelayPort,
    identity: PeerIdentity,
    dead: bool,
}

struct PendingRunRequest {
    sender: PeerIdentity,
    requested_at: Instant,
}

/// The per-frame live-session pump. Owned by the application shell right
/// next to the cloud account service it drains.
pub(crate) struct LiveSessionEngine {
    connection: Option<Connection>,
    role: Role,
    peers: HashMap<PeerIdentity, PeerPresence>,
    /// Roster display names by principal, refreshed from the session
    /// snapshot; frames never carry names.
    roster: HashMap<uuid::Uuid, String>,
    editors: HashSet<uuid::Uuid>,
    host_principal: Option<uuid::Uuid>,
    pending_run_requests: Vec<PendingRunRequest>,
    presence_sent_at: Option<Instant>,
    cursor_sent_at: Option<Instant>,
    last_cursor_sent: Option<Option<CursorLocus>>,
    /// The active schematic's content version as last reconciled into the
    /// workspace buffer map.
    active_synced_version: Option<(String, u64)>,
    /// An incompatible peer build was heard once this session.
    incompatible_peer: bool,
    /// A policy-mandated mirror teardown waits for the local run to stop.
    mirror_discard_pending: bool,
}

impl Default for LiveSessionEngine {
    fn default() -> Self {
        Self {
            connection: None,
            role: Role::Idle,
            peers: HashMap::new(),
            roster: HashMap::new(),
            editors: HashSet::new(),
            host_principal: None,
            pending_run_requests: Vec::new(),
            presence_sent_at: None,
            cursor_sent_at: None,
            last_cursor_sent: None,
            active_synced_version: None,
            incompatible_peer: false,
            mirror_discard_pending: false,
        }
    }
}

impl LiveSessionEngine {
    /// Drive the session for one frame: adopt fresh relay ports, drain
    /// inbound frames, broadcast local changes, and project write locks.
    pub(crate) fn pump(&mut self, state: &mut AppState, cloud: &mut CloudAccountService) {
        self.reconcile_role(state, cloud);
        self.settle_mirror_discard(state);
        if matches!(self.role, Role::Idle) {
            if state.workbench.take_live_mirror_entry() {
                // The close-to-mirror transaction outlived its session.
                state.workbench.open_project_launcher();
                state.push_user_message(ConsoleMessage::warning(
                    "The live session ended before the host's project arrived.",
                ));
            }
            // Ports minted for a session that no longer renders are dead
            // credentials' leftovers; drop them.
            let _ = cloud.take_live_relay_port();
            return;
        }
        self.adopt_port(state, cloud);
        self.finish_mirror_entry(state);
        self.drain_inbound(state);
        if let Some((doc, holder)) = self.take_lease_refusal() {
            let doc = doc
                .strip_prefix(SCHEMATIC_DOC_PREFIX)
                .unwrap_or(&doc)
                .to_owned();
            state.push_user_message(ConsoleMessage::warning(format!(
                "{holder} holds the write lease on {doc}; ask again once it is released."
            )));
        }
        self.flush_pending_project(state);
        self.broadcast_local_changes(state);
        self.stream_run_status(state);
        self.send_presence_heartbeat(state);
        self.send_cursor_update(state);
        self.prune_peers();
        self.project_locks(state);
    }

    /// Participants as currently seen over the relay, for presence chrome.
    pub(crate) fn peers(&self) -> impl Iterator<Item = &PeerPresence> {
        self.peers.values()
    }

    /// Roster display name for a wire identity, falling back to a neutral
    /// label for identities the roster has not caught up with.
    pub(crate) fn display_name(&self, identity: &PeerIdentity) -> String {
        self.roster
            .get(&identity.principal_id)
            .cloned()
            .unwrap_or_else(|| "Another participant".to_owned())
    }

    /// Roster display name of the session host.
    fn display_name_of_host(&self) -> String {
        self.host_principal
            .and_then(|principal| self.roster.get(&principal).cloned())
            .unwrap_or_else(|| "The session host".to_owned())
    }

    /// The freshest host run report, guest-side.
    pub(crate) fn guest_run_status(&self) -> Option<&RunStatusPayload> {
        match &self.role {
            Role::Guest(guest) => guest.run_status.as_ref(),
            _ => None,
        }
    }

    /// The most recent lease refusal aimed at this guest, at most once.
    pub(crate) fn take_lease_refusal(&mut self) -> Option<(String, String)> {
        let Role::Guest(guest) = &mut self.role else {
            return None;
        };
        let (doc, holder) = guest.denied.take()?;
        let holder = self
            .roster
            .get(&holder.principal_id)
            .cloned()
            .unwrap_or_else(|| "Another participant".to_owned());
        Some((doc, holder))
    }

    /// Whether an incompatible peer build was heard this session.
    pub(crate) fn incompatible_peer_seen(&self) -> bool {
        self.incompatible_peer
    }

    /// Host-side run requests awaiting an explicit human decision.
    pub(crate) fn pending_run_requests(&self) -> Vec<(PeerIdentity, String)> {
        self.pending_run_requests
            .iter()
            .map(|request| (request.sender, self.display_name(&request.sender)))
            .collect()
    }

    /// Approve exactly one authenticated request on the host. The workbench
    /// owns preflight and dispatch, so this domain action returns whether an
    /// authenticated request was consumed instead of bypassing that gate.
    pub(crate) fn approve_run_request(&mut self, sender: PeerIdentity) -> bool {
        if !matches!(self.role, Role::Host(_)) {
            return false;
        }
        if let Some(index) = self
            .pending_run_requests
            .iter()
            .position(|request| request.sender == sender)
        {
            self.pending_run_requests.remove(index);
            return true;
        }
        false
    }

    pub(crate) fn deny_run_request(&mut self, sender: PeerIdentity) {
        self.pending_run_requests
            .retain(|request| request.sender != sender);
    }

    /// Guest affordance: ask the host for one document's write lease.
    /// Meaningful only on an adopted mirror — a lease licenses edits to the
    /// mirrored document, not to unrelated local work.
    pub(crate) fn request_lease(&mut self, doc: &str) {
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        if let Role::Guest(guest) = &self.role
            && guest.mirroring
        {
            self.send(&LiveMessage::Document(DocumentMessage::LeaseRequest {
                doc: doc.to_owned(),
                sender: identity,
            }));
        }
    }

    /// Guest affordance: hand a held lease back to the host.
    pub(crate) fn release_lease(&mut self, doc: &str) {
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        let Role::Guest(guest) = &mut self.role else {
            return;
        };
        if guest.held.remove(doc).is_some() {
            guest.leases.release(doc, identity);
            self.send(&LiveMessage::Document(DocumentMessage::LeaseRelease {
                doc: doc.to_owned(),
                holder: identity,
            }));
        }
    }

    /// Guest affordance: ask the host to run its active simulation set.
    pub(crate) fn request_run(&mut self) {
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        if let Role::Guest(guest) = &self.role
            && guest.mirroring
        {
            self.send(&LiveMessage::RunRequest(RunRequestPayload {
                sender: identity,
                run: RunIntent::Simulate,
            }));
        }
    }

    /// Guest lifecycle: begin adopting the host's project as this install's
    /// mirror. Open local work goes through the data-safe close review
    /// first; the engine completes adoption once the close transaction
    /// lands, so callers need no continuation of their own.
    pub(crate) fn request_mirror_entry(&mut self, state: &mut AppState) {
        let Role::Guest(guest) = &self.role else {
            return;
        };
        if guest.mirroring {
            return;
        }
        if state.project_lifecycle.project_open {
            state
                .workbench
                .begin_project_close(crate::workbench::state::ProjectCloseDestination::LiveMirror);
            if crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(state) {
                state.dialogs.project_review_dialog.show_close_project();
            } else if !crate::workbench::workflows::project_workflow::close_project_discard(state) {
                state.workbench.cancel_project_close();
            }
        } else {
            state.workbench.request_live_mirror_entry();
        }
    }

    /// Complete mirror adoption after the close-to-mirror transaction:
    /// from here every host document applies, starting with a full resync.
    fn finish_mirror_entry(&mut self, state: &mut AppState) {
        if !state.workbench.take_live_mirror_entry() {
            return;
        }
        let Role::Guest(guest) = &mut self.role else {
            return;
        };
        guest.mirroring = true;
        guest.project_synced = false;
        guest.pending_project = None;
        guest.reassembler = ContentReassembler::default();
        for mirror in guest.mirror_docs.values_mut() {
            mirror.digest.clear();
        }
        state.push_user_message(ConsoleMessage::info(
            "Joined the live session; the host's project opens when its snapshot arrives.",
        ));
        if let Some(identity) = self.connection.as_ref().map(|c| c.identity) {
            self.send(&LiveMessage::Document(DocumentMessage::SyncRequest {
                sender: identity,
                docs: None,
            }));
        }
    }

    /// Apply the freshest stashed project snapshot; it stays stashed while
    /// a local run or lifecycle transaction owns the project.
    fn flush_pending_project(&mut self, state: &mut AppState) {
        use crate::workbench::workflows::project_workflow::{
            LiveProjectApply, apply_live_project_snapshot,
        };
        let host_label = self.display_name_of_host();
        let Role::Guest(guest) = &mut self.role else {
            return;
        };
        let Some(pending) = guest.pending_project.take() else {
            return;
        };
        match apply_live_project_snapshot(state, &pending.bytes, &host_label) {
            LiveProjectApply::Applied => {
                // The snapshot replaced every document wholesale; wire
                // digests restart so the host's next per-document
                // broadcasts converge idempotently.
                for mirror in guest.mirror_docs.values_mut() {
                    mirror.digest.clear();
                }
                if let Some(mirror) = guest.mirror_docs.get_mut(PROJECT_DOC) {
                    mirror.digest = pending.digest;
                }
                // Re-baseline held leases so snapshot content is not
                // proposed straight back at the host.
                let held_docs: Vec<String> = guest.held.keys().cloned().collect();
                for doc in held_docs {
                    let version = guest_document_version(state, &doc).unwrap_or(0);
                    let digest = serialize_host_document(state, &doc)
                        .map(|bytes| content_digest(&bytes))
                        .unwrap_or_default();
                    if let Some(held) = guest.held.get_mut(&doc) {
                        held.seen_version = version;
                        held.digest = digest;
                    }
                }
                if !guest.project_synced {
                    guest.project_synced = true;
                    state
                        .workbench
                        .activate(crate::workbench::state::Workspace::Project);
                }
            }
            LiveProjectApply::RetryLater => {
                guest.pending_project = Some(pending);
            }
            LiveProjectApply::Rejected => {
                state.push_user_message(ConsoleMessage::warning(
                    "The host's project snapshot could not be mirrored on this build.",
                ));
            }
        }
    }

    /// Finish a policy-mandated mirror teardown that had to wait for the
    /// local run to stop.
    fn settle_mirror_discard(&mut self, state: &mut AppState) {
        if !self.mirror_discard_pending || state.simulation.has_active_execution() {
            return;
        }
        state
            .workbench
            .begin_project_close(crate::workbench::state::ProjectCloseDestination::Launcher);
        if crate::workbench::workflows::project_workflow::close_project_discard(state) {
            self.mirror_discard_pending = false;
        }
    }

    // -- role and port lifecycle ---------------------------------------------

    fn reconcile_role(&mut self, state: &mut AppState, cloud: &CloudAccountService) {
        let session = cloud.snapshot().live_session.clone();
        let (want_host, summary) = match &session {
            Some(LiveSessionState::Hosting(summary)) => (true, Some(summary)),
            Some(LiveSessionState::Participating(summary)) => (false, Some(summary)),
            _ => (false, None),
        };
        let Some(summary) = summary else {
            if !matches!(self.role, Role::Idle) {
                self.leave_session(state);
            }
            return;
        };

        self.roster = summary
            .participants
            .iter()
            .filter_map(|participant| {
                participant
                    .principal_id
                    .parse()
                    .ok()
                    .map(|id| (id, participant.display_name.clone()))
            })
            .collect();
        self.editors = summary
            .participants
            .iter()
            .filter(|participant| participant.editor && !participant.pending)
            .filter_map(|participant| participant.principal_id.parse().ok())
            .collect();
        self.host_principal = summary
            .participants
            .iter()
            .find(|participant| participant.is_host)
            .and_then(|participant| participant.principal_id.parse().ok());

        // Policy can move mid-session (the host applies a new one); the
        // roster poll is its authority here.
        if let Role::Guest(guest) = &mut self.role
            && !want_host
        {
            guest.save_copy_allowed = summary.policy.allow_save_copy;
        }

        let current = match &self.role {
            Role::Host(host) if want_host => Some(&host.session_id),
            Role::Guest(guest) if !want_host => Some(&guest.session_id),
            _ => None,
        };
        if current == Some(&summary.session_id) {
            return;
        }
        // A different session (or a superseded one) is a fresh relay room:
        // every per-session bookkeeping starts over.
        self.leave_session(state);
        self.role = if want_host {
            Role::Host(Box::new(HostRole::new(summary.session_id.clone())))
        } else {
            Role::Guest(Box::new(GuestRole::new(
                summary.session_id.clone(),
                summary.policy.allow_save_copy,
            )))
        };
    }

    fn leave_session(&mut self, state: &mut AppState) {
        let previous = std::mem::replace(&mut self.role, Role::Idle);
        if let Role::Guest(guest) = previous
            && guest.mirroring
        {
            self.retire_mirror(state, guest.save_copy_allowed);
        }
        self.connection = None;
        self.peers.clear();
        self.pending_run_requests.clear();
        self.presence_sent_at = None;
        self.cursor_sent_at = None;
        self.last_cursor_sent = None;
        self.incompatible_peer = false;
        state.workbench.live_write_locks = LiveWriteLocks::default();
        // The read-only flag is this engine's to own; clearing it never
        // unlocks library or safe-mode gates, which are separate.
        state.schematic.read_only = false;
    }

    /// The session is over: keep the mirror open as an unsaved copy when
    /// the policy allows, otherwise close it out of this install.
    fn retire_mirror(&mut self, state: &mut AppState, save_copy_allowed: bool) {
        if save_copy_allowed {
            state.push_user_message(ConsoleMessage::info(
                "The live session ended. The mirrored project stays open; save it to keep a copy.",
            ));
            return;
        }
        state.push_user_message(ConsoleMessage::info(
            "The live session ended; its policy does not allow keeping a copy, \
             so the mirrored project is closing.",
        ));
        if state.simulation.has_active_execution() {
            if state.simulation.can_request_abort_active_run()
                && let Err(error) = state.simulation.request_abort_active_run()
            {
                state.push_sim_message(ConsoleMessage::warning(error));
            }
            self.mirror_discard_pending = true;
            return;
        }
        state
            .workbench
            .begin_project_close(crate::workbench::state::ProjectCloseDestination::Launcher);
        if !crate::workbench::workflows::project_workflow::close_project_discard(state) {
            self.mirror_discard_pending = true;
        }
    }

    fn adopt_port(&mut self, state: &mut AppState, cloud: &mut CloudAccountService) {
        if self.connection.as_ref().is_some_and(|c| c.dead) {
            self.connection = None;
        }
        let Some(port) = cloud.take_live_relay_port() else {
            return;
        };
        let Some(principal) = self.self_principal(cloud) else {
            return;
        };
        let identity = PeerIdentity {
            principal_id: principal,
            client_instance_id: port.client_instance_id,
        };
        self.connection = Some(Connection {
            port,
            identity,
            dead: false,
        });
        self.presence_sent_at = None;
        self.cursor_sent_at = None;
        self.last_cursor_sent = None;
        match &mut self.role {
            Role::Host(_) => {
                // A (re)connected host reintroduces the whole document set:
                // guests deduplicate by digest, so this is idempotent.
                self.refresh_host_documents(state, true);
                self.broadcast_manifest();
                self.broadcast_all_documents(state);
            }
            Role::Guest(guest) => {
                guest.reassembler = ContentReassembler::default();
                self.send(&LiveMessage::Document(DocumentMessage::SyncRequest {
                    sender: identity,
                    docs: None,
                }));
            }
            Role::Idle => {}
        }
    }

    fn self_principal(&self, cloud: &CloudAccountService) -> Option<uuid::Uuid> {
        let session = cloud.snapshot().live_session.as_ref()?;
        let (LiveSessionState::Hosting(summary)
        | LiveSessionState::AwaitingApproval(summary)
        | LiveSessionState::Participating(summary)) = session
        else {
            return None;
        };
        summary
            .participants
            .iter()
            .find(|participant| participant.is_self)
            .and_then(|participant| participant.principal_id.parse().ok())
    }

    fn send(&mut self, message: &LiveMessage) {
        let Some(connection) = self.connection.as_mut() else {
            return;
        };
        let frame = message.encode();
        let disposable = matches!(
            frame.class,
            LiveFrameClass::Presence | LiveFrameClass::Cursor
        );
        match connection.port.outbound.try_send(frame) {
            Ok(()) => {}
            Err(tokio::sync::mpsc::error::TrySendError::Full(_)) if disposable => {}
            Err(_) => {
                // Losing authoritative traffic requires a fresh socket and
                // document resynchronization; never silently drop it.
                connection.dead = true;
            }
        }
    }

    // -- inbound ---------------------------------------------------------------

    fn drain_inbound(&mut self, state: &mut AppState) {
        loop {
            let Some(connection) = self.connection.as_ref() else {
                return;
            };
            let frame = match connection.port.inbound.try_recv() {
                Ok(frame) => frame,
                Err(_) => return,
            };
            match LiveMessage::decode(&frame) {
                Ok(message) if self.authorize_inbound(&frame, &message) => {
                    self.handle_message(state, message);
                }
                Ok(_) => {
                    log::warn!("live frame with a forged or unauthorized sender was dropped");
                }
                Err(LiveProtocolError::UnsupportedVersion(version)) => {
                    if !self.incompatible_peer {
                        self.incompatible_peer = true;
                        state.push_user_message(ConsoleMessage::warning(format!(
                            "A participant runs an incompatible RSpice build \
                             (live protocol {version}); their changes cannot be shared."
                        )));
                    }
                }
                Err(LiveProtocolError::Malformed) => {
                    log::debug!("malformed live frame dropped (class {:?})", frame.class);
                }
            }
        }
    }

    /// Bind every payload identity claim to the relay's ticket-authenticated
    /// v2 stamp. The relay capability matrix limits classes; this method
    /// supplies the finer host/guest semantics within each class.
    fn authorize_inbound(&self, frame: &LiveFrame, message: &LiveMessage) -> bool {
        let Some(stamp) = frame.authenticated_sender else {
            return false;
        };
        let sender = PeerIdentity {
            principal_id: stamp.principal_id,
            client_instance_id: stamp.client_instance_id,
        };
        if !self.roster.contains_key(&sender.principal_id) {
            return false;
        }
        match &self.role {
            Role::Idle => false,
            Role::Host(_) => match message {
                LiveMessage::Presence(payload) => payload.sender == sender,
                LiveMessage::Cursor(payload) => payload.sender == sender,
                LiveMessage::RunRequest(payload) => payload.sender == sender,
                LiveMessage::RunStatus(_) => false,
                LiveMessage::Document(DocumentMessage::Replace { header, .. }) => {
                    header.sender == sender
                }
                LiveMessage::Document(DocumentMessage::LeaseRequest { sender: claim, .. })
                | LiveMessage::Document(DocumentMessage::SyncRequest { sender: claim, .. }) => {
                    *claim == sender
                }
                LiveMessage::Document(DocumentMessage::LeaseRelease { holder, .. }) => {
                    *holder == sender
                }
                LiveMessage::Document(
                    DocumentMessage::LeaseGrant { .. }
                    | DocumentMessage::LeaseDeny { .. }
                    | DocumentMessage::Manifest { .. },
                ) => false,
            },
            Role::Guest(_) => {
                let from_host = self.host_principal == Some(sender.principal_id);
                match message {
                    LiveMessage::Presence(payload) => payload.sender == sender,
                    LiveMessage::Cursor(payload) => payload.sender == sender,
                    LiveMessage::RunRequest(payload) => payload.sender == sender,
                    LiveMessage::RunStatus(RunStatusMessage::Status(payload)) => {
                        from_host && payload.sender == sender
                    }
                    LiveMessage::RunStatus(RunStatusMessage::ResultChunk { header, .. }) => {
                        from_host && header.sender == sender
                    }
                    LiveMessage::Document(DocumentMessage::Replace { header, .. }) => {
                        from_host && header.sender == sender
                    }
                    LiveMessage::Document(DocumentMessage::Manifest { sender: claim, .. }) => {
                        from_host && *claim == sender
                    }
                    LiveMessage::Document(
                        DocumentMessage::LeaseGrant { .. }
                        | DocumentMessage::LeaseDeny { .. }
                        | DocumentMessage::LeaseRelease { .. },
                    ) => from_host,
                    LiveMessage::Document(
                        DocumentMessage::LeaseRequest { .. } | DocumentMessage::SyncRequest { .. },
                    ) => false,
                }
            }
        }
    }

    fn handle_message(&mut self, state: &mut AppState, message: LiveMessage) {
        match message {
            LiveMessage::Presence(payload) => {
                let entry = self
                    .peers
                    .entry(payload.sender)
                    .or_insert_with(|| PeerPresence {
                        identity: payload.sender,
                        focused_doc: None,
                        cursor: None,
                        seen_at: Instant::now(),
                    });
                entry.focused_doc = payload.focused_doc;
                entry.seen_at = Instant::now();
            }
            LiveMessage::Cursor(payload) => {
                let entry = self
                    .peers
                    .entry(payload.sender)
                    .or_insert_with(|| PeerPresence {
                        identity: payload.sender,
                        focused_doc: None,
                        cursor: None,
                        seen_at: Instant::now(),
                    });
                entry.cursor = payload.locus;
                entry.seen_at = Instant::now();
            }
            LiveMessage::Document(message) => match &self.role {
                Role::Host(_) => self.host_handle_document(state, message),
                Role::Guest(_) => self.guest_handle_document(state, message),
                Role::Idle => {}
            },
            LiveMessage::RunRequest(payload) => {
                if matches!(self.role, Role::Host(_))
                    && self.editors.contains(&payload.sender.principal_id)
                    && !self
                        .pending_run_requests
                        .iter()
                        .any(|request| request.sender == payload.sender)
                    && self.pending_run_requests.len() < MAX_PENDING_RUN_REQUESTS
                {
                    let requester = self.display_name(&payload.sender);
                    state.push_user_message(ConsoleMessage::info(format!(
                        "{requester} requested a run. Review it in the Live session dialog."
                    )));
                    self.pending_run_requests.push(PendingRunRequest {
                        sender: payload.sender,
                        requested_at: Instant::now(),
                    });
                }
            }
            LiveMessage::RunStatus(RunStatusMessage::Status(payload)) => {
                if let Role::Guest(guest) = &mut self.role {
                    let stale = guest
                        .run_status
                        .as_ref()
                        .is_some_and(|current| current.run_id > payload.run_id);
                    if !stale {
                        guest.run_status = Some(payload);
                    }
                }
            }
            // Result datasets are not streamed in this protocol revision:
            // every participant holds a full seat and can run the mirrored
            // design locally.
            LiveMessage::RunStatus(RunStatusMessage::ResultChunk { .. }) => {}
        }
    }

    // -- host ------------------------------------------------------------------

    fn host_handle_document(&mut self, state: &mut AppState, message: DocumentMessage) {
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        let Role::Host(host) = &mut self.role else {
            return;
        };
        match message {
            DocumentMessage::LeaseRequest { doc, sender } => {
                // The whole-project snapshot is never leased: structural
                // authority stays with the host.
                let reply = if doc == PROJECT_DOC || !host.tracked.contains_key(&doc) {
                    DocumentMessage::LeaseDeny {
                        doc,
                        requester: sender,
                        holder: identity,
                    }
                } else {
                    match host.leases.arbitrate(&doc, sender) {
                        LeaseDecision::Granted => DocumentMessage::LeaseGrant {
                            doc,
                            holder: sender,
                        },
                        LeaseDecision::Held(holder) => DocumentMessage::LeaseDeny {
                            doc,
                            requester: sender,
                            holder,
                        },
                    }
                };
                self.send(&LiveMessage::Document(reply));
            }
            DocumentMessage::LeaseRelease { doc, holder } => {
                if host.leases.release(&doc, holder) {
                    // Re-announce so every guest's mirror table agrees.
                    self.send(&LiveMessage::Document(DocumentMessage::LeaseRelease {
                        doc,
                        holder,
                    }));
                }
            }
            DocumentMessage::Replace { header, chunk } => {
                let authorized = host.leases.holder(&header.doc).is_some_and(|holder| {
                    holder.client_instance_id == header.sender.client_instance_id
                });
                if !authorized {
                    log::warn!(
                        "live replace for {} dropped: sender holds no lease",
                        header.doc
                    );
                    return;
                }
                match host.reassembler.accept(&header, &chunk) {
                    Ok(Some(completed)) => self.host_apply_proposal(state, completed),
                    Ok(None) => {}
                    Err(error) => {
                        log::warn!("live proposal for {} rejected: {error:?}", header.doc)
                    }
                }
            }
            DocumentMessage::SyncRequest { docs, .. } => {
                self.refresh_host_documents(state, true);
                self.broadcast_manifest();
                match docs {
                    None => self.broadcast_all_documents(state),
                    Some(requested) => {
                        for doc in requested {
                            self.broadcast_document(state, &doc, false);
                        }
                    }
                }
            }
            // Host-only announcements arriving at the host are protocol
            // misuse from a peer; the relay already restricts the class.
            DocumentMessage::LeaseGrant { .. }
            | DocumentMessage::LeaseDeny { .. }
            | DocumentMessage::Manifest { .. } => {}
        }
    }

    /// Apply a leaseholder's completed proposal onto the host working copy
    /// and rebroadcast it with host authority.
    fn host_apply_proposal(&mut self, state: &mut AppState, completed: CompletedContent) {
        let applied = if completed.doc == NETLIST_DOC {
            match String::from_utf8(completed.bytes.clone()) {
                Ok(source) => {
                    crate::workbench::documents::netlist_document::apply_live_owned_source(
                        state, source,
                    )
                }
                Err(_) => false,
            }
        } else if let Some(cell_key) = completed.doc.strip_prefix(SCHEMATIC_DOC_PREFIX) {
            match std::str::from_utf8(&completed.bytes)
                .ok()
                .and_then(|text| load_schematic_text(text, None).ok())
            {
                Some(prepared) => {
                    install_schematic_buffer(state, cell_key, prepared, true);
                    true
                }
                None => false,
            }
        } else {
            false
        };
        if !applied {
            log::warn!("live proposal for {} could not be applied", completed.doc);
            return;
        }

        let Role::Host(host) = &mut self.role else {
            return;
        };
        let seen_version = host_document_version(state, &completed.doc).unwrap_or(0);
        let entry = host
            .tracked
            .entry(completed.doc.clone())
            .or_insert_with(|| TrackedDoc {
                content_type: content_type_for(&completed.doc),
                seen_version: 0,
                revision: 0,
                digest: String::new(),
            });
        entry.seen_version = seen_version;
        entry.revision += 1;
        entry.digest = completed.digest.clone();
        let revision = entry.revision;
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        // Rebroadcast the exact accepted bytes with host authority so every
        // mirror converges on them, the proposer included.
        if let Some(messages) =
            replace_messages(identity, &completed.doc, revision, &completed.bytes)
        {
            for message in messages {
                self.send(&LiveMessage::Document(message));
            }
        }
    }

    /// Reconcile the host's tracked set with the workspace: sync the active
    /// buffer, admit new documents, and retire deleted ones.
    fn refresh_host_documents(&mut self, state: &mut AppState, announce_removals: bool) {
        self.sync_active_buffer(state);
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        let Role::Host(host) = &mut self.role else {
            return;
        };
        let mut current: Vec<String> = state
            .workspace
            .schematic_buffers
            .keys()
            .map(|key| schematic_doc_key(key))
            .collect();
        if state.workspace.netlist_document.is_some() {
            current.push(NETLIST_DOC.to_owned());
        }
        current.push(PROJECT_DOC.to_owned());
        for doc in &current {
            host.tracked
                .entry(doc.clone())
                .or_insert_with(|| TrackedDoc {
                    content_type: content_type_for(doc),
                    // A fresh document is behind every version, so the change
                    // scan below broadcasts it.
                    seen_version: u64::MAX,
                    revision: 0,
                    digest: String::new(),
                });
        }
        let removed: Vec<String> = host
            .tracked
            .keys()
            .filter(|doc| !current.contains(doc))
            .cloned()
            .collect();
        let mut releases = Vec::new();
        for doc in removed {
            host.tracked.remove(&doc);
            if let Some(holder) = host.leases.holder(&doc) {
                host.leases.release(&doc, holder);
                releases.push((doc, holder));
            }
            // Absence from the next manifest is the deletion signal.
        }
        let _ = identity;
        if announce_removals {
            for (doc, holder) in releases {
                self.send(&LiveMessage::Document(DocumentMessage::LeaseRelease {
                    doc,
                    holder,
                }));
            }
        }
    }

    /// Keep the workspace buffer map current with the active working copy,
    /// but only when its content actually moved.
    fn sync_active_buffer(&mut self, state: &mut AppState) {
        let key = state.workspace.active_key();
        let version = state.schematic.content_version();
        if self.active_synced_version != Some((key.clone(), version)) {
            state.sync_active_schematic_to_workspace();
            self.active_synced_version = Some((key, version));
        }
    }

    fn broadcast_manifest(&mut self) {
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        let Role::Host(host) = &self.role else {
            return;
        };
        let mut docs: Vec<ManifestEntry> = host
            .tracked
            .iter()
            .map(|(doc, tracked)| ManifestEntry {
                doc: doc.clone(),
                content_type: tracked.content_type.to_owned(),
                revision: tracked.revision,
                digest: tracked.digest.clone(),
            })
            .collect();
        docs.sort_by(|a, b| a.doc.cmp(&b.doc));
        self.send(&LiveMessage::Document(DocumentMessage::Manifest {
            sender: identity,
            docs,
        }));
    }

    fn broadcast_all_documents(&mut self, state: &mut AppState) {
        let docs: Vec<String> = match &self.role {
            Role::Host(host) => host.tracked.keys().cloned().collect(),
            _ => return,
        };
        for doc in docs {
            self.broadcast_document(state, &doc, false);
        }
    }

    /// Serialize one document and broadcast it when it moved (or always,
    /// for sync responses re-serving current content).
    fn broadcast_document(&mut self, state: &mut AppState, doc: &str, only_if_changed: bool) {
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        let Role::Host(host) = &mut self.role else {
            return;
        };
        let Some(tracked) = host.tracked.get(doc) else {
            return;
        };
        let version = host_document_version(state, doc);
        // Documents without a cheap change counter (the whole-project
        // snapshot) broadcast only on explicit occasions — sync responses,
        // reconnects, structural changes — never from the per-frame scan.
        if only_if_changed && (version.is_none() || version == Some(tracked.seen_version)) {
            return;
        }
        let Some(bytes) = serialize_host_document(state, doc) else {
            return;
        };
        let digest = content_digest(&bytes);
        let tracked = host.tracked.get_mut(doc).expect("present above");
        tracked.seen_version = version.unwrap_or(0);
        if digest == tracked.digest && tracked.revision > 0 && only_if_changed {
            // The counter moved but the content did not (e.g. an undo that
            // restored the broadcast state); nothing to send.
            return;
        }
        if digest != tracked.digest || tracked.revision == 0 {
            tracked.revision += 1;
        }
        tracked.digest = digest;
        let revision = tracked.revision;
        if let Some(messages) = replace_messages(identity, doc, revision, &bytes) {
            for message in messages {
                self.send(&LiveMessage::Document(message));
            }
        } else {
            log::warn!("live document {doc} exceeds the shareable size and was not sent");
        }
    }

    fn broadcast_local_changes(&mut self, state: &mut AppState) {
        match &self.role {
            Role::Host(_) => {
                let before: Vec<String> = match &self.role {
                    Role::Host(host) => host.tracked.keys().cloned().collect(),
                    _ => return,
                };
                self.refresh_host_documents(state, true);
                let after: Vec<String> = match &self.role {
                    Role::Host(host) => host.tracked.keys().cloned().collect(),
                    _ => return,
                };
                let set_changed =
                    before.len() != after.len() || !before.iter().all(|doc| after.contains(doc));
                if set_changed {
                    self.broadcast_manifest();
                    // Structural changes (cells created, deleted, renamed)
                    // resynchronize the whole project so mirrors keep
                    // libraries and hierarchy coherent with the catalog.
                    self.broadcast_document(state, PROJECT_DOC, false);
                }
                for doc in after {
                    self.broadcast_document(state, &doc, true);
                }
            }
            Role::Guest(_) => self.guest_send_proposals(state),
            Role::Idle => {}
        }
    }

    fn stream_run_status(&mut self, state: &mut AppState) {
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        let Role::Host(host) = &mut self.role else {
            return;
        };
        let running = state.simulation.has_active_execution();
        let fraction = state.simulation.progress.clamp(0.0, 1.0);
        let status = state.simulation.status.clone();
        let mut outgoing: Option<RunStatusPayload> = None;
        if running && !host.run.active {
            host.run.active = true;
            host.run.run_id += 1;
            host.run.last_fraction = fraction;
            host.run.last_status = status.clone();
            outgoing = Some(RunStatusPayload {
                sender: identity,
                run_id: host.run.run_id,
                phase: RunPhase::Started,
                progress: None,
            });
        } else if running
            && ((fraction - host.run.last_fraction).abs() >= RUN_PROGRESS_STEP
                || status != host.run.last_status)
        {
            host.run.last_fraction = fraction;
            host.run.last_status = status.clone();
            outgoing = Some(RunStatusPayload {
                sender: identity,
                run_id: host.run.run_id,
                phase: RunPhase::Progress,
                progress: Some(serde_json::json!({
                    "fraction": fraction,
                    "status": status,
                })),
            });
        } else if !running && host.run.active {
            host.run.active = false;
            outgoing = Some(RunStatusPayload {
                sender: identity,
                run_id: host.run.run_id,
                phase: RunPhase::Finished,
                progress: Some(serde_json::json!({
                    "fraction": fraction,
                    "status": status,
                })),
            });
        }
        if let Some(payload) = outgoing {
            self.send(&LiveMessage::RunStatus(RunStatusMessage::Status(payload)));
        }
    }

    // -- guest -------------------------------------------------------------------

    fn guest_handle_document(&mut self, state: &mut AppState, message: DocumentMessage) {
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        let host_principal = self.host_principal;
        let Role::Guest(guest) = &mut self.role else {
            return;
        };
        match message {
            DocumentMessage::Manifest { sender, docs } => {
                // The manifest pins the host's wire identity — verified
                // against the authenticated roster's host row.
                if host_principal != Some(sender.principal_id) {
                    log::warn!("live manifest from a non-host identity dropped");
                    return;
                }
                guest.host = Some(sender);
                let named: Vec<String> = docs.iter().map(|entry| entry.doc.clone()).collect();
                guest.mirror_docs.retain(|doc, _| named.contains(doc));
                for entry in docs {
                    guest
                        .mirror_docs
                        .entry(entry.doc)
                        .or_insert_with(|| MirrorDoc {
                            content_type: entry.content_type,
                            digest: String::new(),
                        });
                }
            }
            DocumentMessage::Replace { header, chunk } => {
                // Only the host's broadcasts are authoritative; other
                // leaseholders' proposals come back through the host.
                if guest.host.map(|h| h.client_instance_id)
                    != Some(header.sender.client_instance_id)
                {
                    return;
                }
                match guest.reassembler.accept(&header, &chunk) {
                    Ok(Some(completed)) => self.guest_apply_document(state, completed),
                    Ok(None) => {}
                    Err(error) => {
                        log::warn!("live document {} rejected: {error:?}", header.doc);
                    }
                }
            }
            DocumentMessage::LeaseGrant { doc, holder } => {
                guest.leases.apply_grant(&doc, holder);
                if holder.client_instance_id == identity.client_instance_id {
                    let seen_version = guest_document_version(state, &doc).unwrap_or(0);
                    let digest = guest
                        .mirror_docs
                        .get(&doc)
                        .map(|mirror| mirror.digest.clone())
                        .unwrap_or_default();
                    guest.held.entry(doc).or_insert(HeldDoc {
                        proposal_revision: 0,
                        seen_version,
                        digest,
                    });
                }
            }
            DocumentMessage::LeaseDeny {
                doc,
                requester,
                holder,
            } => {
                if requester.client_instance_id == identity.client_instance_id {
                    guest.denied = Some((doc, holder));
                }
            }
            DocumentMessage::LeaseRelease { doc, holder } => {
                guest.leases.apply_release(&doc);
                if holder.client_instance_id == identity.client_instance_id {
                    guest.held.remove(&doc);
                }
            }
            // Guests never arbitrate or serve documents.
            DocumentMessage::LeaseRequest { .. } | DocumentMessage::SyncRequest { .. } => {}
        }
    }

    fn guest_apply_document(&mut self, state: &mut AppState, completed: CompletedContent) {
        let Role::Guest(guest) = &mut self.role else {
            return;
        };
        let Some(mirror) = guest.mirror_docs.get_mut(&completed.doc) else {
            // Replaces may outrun the manifest on a fresh connection; the
            // sync response's manifest lands first in send order, so an
            // unknown document here is a stale or misordered stream.
            log::debug!(
                "live document {} not in the manifest; dropped",
                completed.doc
            );
            return;
        };
        if mirror.digest == completed.digest {
            return;
        }
        if mirror.content_type != content_type_for(&completed.doc) {
            // The manifest promised a content contract this key does not
            // imply; applying it would misinterpret the bytes.
            log::warn!(
                "live document {} arrived as {}, which this build does not apply",
                completed.doc,
                mirror.content_type
            );
            return;
        }
        if let Some(held) = guest.held.get(&completed.doc)
            && held.digest == completed.digest
        {
            // The host echoed this guest's own proposal back with authority;
            // adopt the digest without touching the local working copy.
            mirror.digest = completed.digest;
            return;
        }
        if !guest.mirroring {
            // Tracked but not applied: the user has not adopted the mirror.
            return;
        }
        if completed.doc == PROJECT_DOC {
            // The whole-project snapshot has its own retrying applier (the
            // pump flushes it); the digest is recorded there on success.
            guest.pending_project = Some(completed);
            return;
        }
        let applied = if completed.doc == NETLIST_DOC {
            match String::from_utf8(completed.bytes.clone()) {
                Ok(source) => {
                    crate::workbench::documents::netlist_document::apply_live_owned_source(
                        state, source,
                    )
                }
                Err(_) => false,
            }
        } else if let Some(cell_key) = completed.doc.strip_prefix(SCHEMATIC_DOC_PREFIX) {
            match std::str::from_utf8(&completed.bytes)
                .ok()
                .and_then(|text| load_schematic_text(text, None).ok())
            {
                Some(prepared) => {
                    install_schematic_buffer(state, cell_key, prepared, false);
                    true
                }
                None => false,
            }
        } else {
            // A content type this build does not render; keep tracking it.
            true
        };
        if applied {
            mirror.digest = completed.digest.clone();
            if let Some(held) = guest.held.get_mut(&completed.doc) {
                held.seen_version = guest_document_version(state, &completed.doc).unwrap_or(0);
                held.digest = completed.digest;
            }
        } else {
            log::warn!("live document {} could not be applied", completed.doc);
        }
    }

    /// Send this guest's edits to documents it holds leases on.
    fn guest_send_proposals(&mut self, state: &mut AppState) {
        let held_docs: Vec<String> = match &self.role {
            // Leases exist only on an adopted mirror; anything else here
            // would propose unrelated local work at the host.
            Role::Guest(guest) if guest.mirroring => guest.held.keys().cloned().collect(),
            _ => return,
        };
        if held_docs.is_empty() {
            return;
        }
        self.sync_active_buffer(state);
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        for doc in held_docs {
            let Some(version) = guest_document_version(state, &doc) else {
                continue;
            };
            let Role::Guest(guest) = &mut self.role else {
                return;
            };
            let Some(held) = guest.held.get_mut(&doc) else {
                continue;
            };
            if held.seen_version == version {
                continue;
            }
            let Some(bytes) = serialize_host_document(state, &doc) else {
                continue;
            };
            let digest = content_digest(&bytes);
            held.seen_version = version;
            if digest == held.digest {
                continue;
            }
            held.proposal_revision += 1;
            held.digest = digest;
            let revision = held.proposal_revision;
            if let Some(messages) = replace_messages(identity, &doc, revision, &bytes) {
                for message in messages {
                    self.send(&LiveMessage::Document(message));
                }
            }
        }
    }

    // -- shared frame upkeep -----------------------------------------------------

    fn send_presence_heartbeat(&mut self, state: &AppState) {
        let due = self
            .presence_sent_at
            .is_none_or(|sent| sent.elapsed() >= PRESENCE_PERIOD);
        if !due || self.connection.is_none() {
            return;
        }
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        let focused = schematic_doc_key(&state.workspace.active_key());
        self.send(&LiveMessage::Presence(PresencePayload {
            sender: identity,
            focused_doc: Some(focused),
        }));
        self.presence_sent_at = Some(Instant::now());
    }

    fn send_cursor_update(&mut self, state: &AppState) {
        if self.connection.is_none()
            || self
                .cursor_sent_at
                .is_some_and(|sent| sent.elapsed() < CURSOR_PERIOD)
        {
            return;
        }
        let locus = if state.workbench.workspace == crate::workbench::state::Workspace::Design {
            state.ui.canvas_hover.and_then(|(x, y)| {
                let (x, y) = (x as f32, y as f32);
                (x.is_finite() && y.is_finite()).then(|| CursorLocus::Canvas {
                    doc: schematic_doc_key(&state.workspace.active_key()),
                    x,
                    y,
                })
            })
        } else if state.workbench.workspace == crate::workbench::state::Workspace::Netlist
            && state.ui.netlist.active_document
                == crate::workbench::documents::netlist_document::ActiveNetlistDocument::OwnedSource
        {
            Some(CursorLocus::Netlist {
                doc: NETLIST_DOC.to_owned(),
                line: u32::try_from(state.ui.netlist.cursor_line.saturating_add(1))
                    .unwrap_or(u32::MAX),
            })
        } else {
            None
        };
        if self.last_cursor_sent.as_ref() == Some(&locus) {
            return;
        }
        let Some(identity) = self
            .connection
            .as_ref()
            .map(|connection| connection.identity)
        else {
            return;
        };
        self.send(&LiveMessage::Cursor(CursorPayload {
            sender: identity,
            locus: locus.clone(),
        }));
        self.last_cursor_sent = Some(locus);
        self.cursor_sent_at = Some(Instant::now());
    }

    /// Retire peers whose presence went quiet, reclaiming whatever they left
    /// half-sent. A departed peer that also held write leases frees them:
    /// otherwise a document stays locked behind someone who is gone.
    fn prune_peers(&mut self) {
        self.pending_run_requests.retain(|request| {
            request.requested_at.elapsed() < PEER_TIMEOUT
                && self.editors.contains(&request.sender.principal_id)
        });
        let departed: Vec<PeerIdentity> = self
            .peers
            .values()
            .filter(|presence| presence.seen_at.elapsed() >= PEER_TIMEOUT)
            .map(|presence| presence.identity)
            .collect();
        if departed.is_empty() {
            return;
        }
        for identity in &departed {
            self.peers.remove(identity);
        }
        let mut freed = Vec::new();
        match &mut self.role {
            Role::Host(host) => {
                for identity in &departed {
                    host.reassembler
                        .forget_instance(identity.client_instance_id);
                    // Only a principal with no remaining presence loses its
                    // leases: a second window of the same person still holds.
                    if self
                        .peers
                        .keys()
                        .any(|peer| peer.principal_id == identity.principal_id)
                    {
                        continue;
                    }
                    for doc in host.leases.revoke_principal(identity.principal_id) {
                        freed.push((doc, *identity));
                    }
                }
            }
            Role::Guest(guest) => {
                for identity in &departed {
                    guest
                        .reassembler
                        .forget_instance(identity.client_instance_id);
                }
            }
            Role::Idle => {}
        }
        // Announce host-side revocations so every mirror agrees.
        for (doc, holder) in freed {
            self.send(&LiveMessage::Document(DocumentMessage::LeaseRelease {
                doc,
                holder,
            }));
        }
    }

    /// Project lease state onto the UI's write gates.
    fn project_locks(&mut self, state: &mut AppState) {
        let identity = self.connection.as_ref().map(|c| c.identity);
        let mut locks = LiveWriteLocks::default();
        match &self.role {
            Role::Host(host) => {
                for (doc, holder) in host.leases.entries() {
                    if Some(holder.client_instance_id) != identity.map(|i| i.client_instance_id) {
                        let name = self.display_name(&holder);
                        if doc == NETLIST_DOC {
                            locks.netlist = Some(name);
                        } else if let Some(cell_key) = doc.strip_prefix(SCHEMATIC_DOC_PREFIX) {
                            locks.schematic_views.insert(cell_key.to_owned(), name);
                        }
                    }
                }
            }
            Role::Guest(guest) => {
                locks.mirror = guest.mirroring;
                locks.mirror_save_copy_allowed = guest.save_copy_allowed;
                if guest.mirroring {
                    let host_name = self.display_name_of_host();
                    for doc in guest.mirror_docs.keys() {
                        if guest.held.contains_key(doc) {
                            continue;
                        }
                        let owner = guest
                            .leases
                            .holder(doc)
                            .map(|holder| self.display_name(&holder))
                            .unwrap_or_else(|| host_name.clone());
                        if doc == NETLIST_DOC {
                            locks.netlist = Some(owner);
                        } else if let Some(cell_key) = doc.strip_prefix(SCHEMATIC_DOC_PREFIX) {
                            locks.schematic_views.insert(cell_key.to_owned(), owner);
                        }
                    }
                }
            }
            Role::Idle => {}
        }
        let active_locked = locks
            .schematic_views
            .contains_key(&state.workspace.active_key());
        state.workbench.live_write_locks = locks;
        state.schematic.read_only = active_locked;
    }
}

/// Install one prepared schematic buffer, preserving the local view
/// orientation when it is the active document. `remote_is_unsaved` marks
/// the buffer dirty (host adopting a proposal owns saving it); mirrors stay
/// clean because saving them is governed by the session policy.
fn install_schematic_buffer(
    state: &mut AppState,
    cell_key: &str,
    mut prepared: SchematicState,
    remote_is_unsaved: bool,
) {
    prepared.needs_fit = false;
    prepared.is_dirty = remote_is_unsaved;
    prepared.needs_history_reset = true;
    if state.workspace.active_key() == cell_key {
        prepared.zoom = state.schematic.zoom;
        prepared.pan = state.schematic.pan;
        prepared.read_only = state.schematic.read_only;
        state.schematic = prepared.clone();
    }
    state
        .workspace
        .schematic_buffers
        .insert(cell_key.to_owned(), prepared);
}

fn content_type_for(doc: &str) -> &'static str {
    if doc == PROJECT_DOC {
        PROJECT_CONTENT_TYPE
    } else if doc == NETLIST_DOC {
        NETLIST_CONTENT_TYPE
    } else {
        SCHEMATIC_CONTENT_TYPE
    }
}

/// The document's own change counter, host- and guest-side alike.
fn host_document_version(state: &AppState, doc: &str) -> Option<u64> {
    if doc == NETLIST_DOC {
        state
            .workspace
            .netlist_document
            .as_ref()
            .map(|document| document.revision().get())
    } else {
        let cell_key = doc.strip_prefix(SCHEMATIC_DOC_PREFIX)?;
        state
            .workspace
            .schematic_buffers
            .get(cell_key)
            .map(SchematicState::content_version)
    }
}

fn guest_document_version(state: &AppState, doc: &str) -> Option<u64> {
    host_document_version(state, doc)
}

/// Serialize one document's current content for the wire.
fn serialize_host_document(state: &AppState, doc: &str) -> Option<Vec<u8>> {
    if doc == PROJECT_DOC {
        let mut project = match crate::workbench::lifecycle::project_lifecycle::snapshot(state) {
            Ok(project) => project,
            Err(error) => {
                log::warn!("live project snapshot failed: {error}");
                return None;
            }
        };
        // Result datasets are deliberately not streamed — every participant
        // holds a full seat and runs the mirrored design locally — and the
        // host's on-disk path is machine-local, so neither crosses the wire.
        project.simulation_results = Default::default();
        project.simulation_results_warning = None;
        project.workspace.project.path = None;
        return match crate::io::project_io::serialize_project_file(&project) {
            Ok(text) => Some(text.into_bytes()),
            Err(error) => {
                log::warn!("live project snapshot failed: {error}");
                None
            }
        };
    }
    if doc == NETLIST_DOC {
        state
            .workspace
            .netlist_document
            .as_ref()
            .map(|document| document.source_bytes().to_vec())
    } else {
        let cell_key = doc.strip_prefix(SCHEMATIC_DOC_PREFIX)?;
        let buffer = state.workspace.schematic_buffers.get(cell_key)?;
        serialize_schematic_for_wire(buffer).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::cloud_account::LiveRelayIdentity;

    fn identity(principal: u128, client: u128) -> PeerIdentity {
        PeerIdentity {
            principal_id: uuid::Uuid::from_u128(principal),
            client_instance_id: uuid::Uuid::from_u128(client),
        }
    }

    fn stamped(message: LiveMessage, authenticated: PeerIdentity) -> LiveFrame {
        let mut frame = message.encode();
        frame.authenticated_sender = Some(LiveRelayIdentity {
            principal_id: authenticated.principal_id,
            client_instance_id: authenticated.client_instance_id,
        });
        frame
    }

    fn attach_for_test(
        engine: &mut LiveSessionEngine,
        local: PeerIdentity,
    ) -> std::sync::mpsc::SyncSender<LiveFrame> {
        let (inbound_tx, inbound) = std::sync::mpsc::sync_channel(4);
        let (outbound, _outbound_rx) = tokio::sync::mpsc::channel(4);
        engine.connection = Some(Connection {
            port: LiveRelayPort {
                client_instance_id: local.client_instance_id,
                outbound,
                inbound,
            },
            identity: local,
            dead: false,
        });
        inbound_tx
    }

    #[test]
    fn forged_run_request_identity_is_dropped() {
        let host = identity(1, 11);
        let editor = identity(2, 22);
        let attacker = identity(3, 33);
        let mut engine = LiveSessionEngine {
            role: Role::Host(Box::new(HostRole::new("session".to_owned()))),
            ..LiveSessionEngine::default()
        };
        engine
            .roster
            .insert(editor.principal_id, "Editor".to_owned());
        engine
            .roster
            .insert(attacker.principal_id, "Attacker".to_owned());
        engine.editors.insert(editor.principal_id);
        let inbound = attach_for_test(&mut engine, host);
        inbound
            .send(stamped(
                LiveMessage::RunRequest(RunRequestPayload {
                    sender: editor,
                    run: RunIntent::Simulate,
                }),
                attacker,
            ))
            .expect("test relay remains attached");

        let mut state = AppState::default();
        engine.drain_inbound(&mut state);

        assert!(engine.pending_run_requests.is_empty());
        assert!(!state.simulation.trigger_simulation);
    }

    #[test]
    fn authenticated_run_request_requires_explicit_host_approval() {
        let host = identity(1, 11);
        let editor = identity(2, 22);
        let mut engine = LiveSessionEngine {
            role: Role::Host(Box::new(HostRole::new("session".to_owned()))),
            ..LiveSessionEngine::default()
        };
        engine
            .roster
            .insert(editor.principal_id, "Editor".to_owned());
        engine.editors.insert(editor.principal_id);
        let inbound = attach_for_test(&mut engine, host);
        inbound
            .send(stamped(
                LiveMessage::RunRequest(RunRequestPayload {
                    sender: editor,
                    run: RunIntent::Simulate,
                }),
                editor,
            ))
            .expect("test relay remains attached");

        let mut state = AppState::default();
        engine.drain_inbound(&mut state);

        assert_eq!(engine.pending_run_requests.len(), 1);
        assert!(!state.simulation.trigger_simulation);

        let approved = engine.approve_run_request(editor);

        assert!(approved);
        assert!(engine.pending_run_requests.is_empty());
        assert!(!state.simulation.trigger_simulation);
    }

    #[test]
    fn guest_rejects_host_authoritative_frames_from_non_host_stamp() {
        let guest = identity(1, 11);
        let host = identity(2, 22);
        let attacker = identity(3, 33);
        let mut engine = LiveSessionEngine {
            role: Role::Guest(Box::new(GuestRole::new("session".to_owned(), false))),
            host_principal: Some(host.principal_id),
            ..LiveSessionEngine::default()
        };
        engine.roster.insert(host.principal_id, "Host".to_owned());
        engine
            .roster
            .insert(attacker.principal_id, "Attacker".to_owned());
        let inbound = attach_for_test(&mut engine, guest);
        inbound
            .send(stamped(
                LiveMessage::RunStatus(RunStatusMessage::Status(RunStatusPayload {
                    sender: attacker,
                    run_id: 1,
                    phase: RunPhase::Started,
                    progress: None,
                })),
                attacker,
            ))
            .expect("test relay remains attached");

        let mut state = AppState::default();
        engine.drain_inbound(&mut state);

        assert!(engine.guest_run_status().is_none());
    }
}
