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

use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::diagnostics::ConsoleMessage;
use crate::io::schematic_io::{load_schematic_text, serialize_schematic_for_wire};
use crate::services::cloud_account::{CloudAccountService, LiveRelayPort, LiveSessionState};
use crate::services::live_protocol::{
    CompletedContent, ContentReassembler, CursorLocus, DocumentMessage, LeaseDecision, LeaseTable,
    LiveMessage, LiveProtocolError, ManifestEntry, PeerIdentity, PresencePayload, RunIntent,
    RunPhase, RunRequestPayload, RunStatusMessage, RunStatusPayload, content_digest,
    replace_messages,
};
use crate::state::SchematicState;
use crate::workbench::AppState;
use crate::workbench::state::LiveWriteLocks;

/// Presence cadence; doubles as the relay keepalive under its idle timeout.
const PRESENCE_PERIOD: Duration = Duration::from_secs(15);
/// Drop peers whose presence went quiet (they left or lost the relay).
const PEER_TIMEOUT: Duration = Duration::from_secs(60);
/// Minimum progress-fraction movement worth a run-status frame.
const RUN_PROGRESS_STEP: f64 = 0.01;

/// Wire content contract of a schematic cell-view buffer.
const SCHEMATIC_CONTENT_TYPE: &str = "rspice.schematic-view.v1";
/// Wire content contract of the project-owned netlist source.
const NETLIST_CONTENT_TYPE: &str = "rspice.netlist-source.v1";
/// Wire key of the project's single owned netlist document.
const NETLIST_DOC: &str = "netlist";
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
    /// The freshest host run report, for the session status chrome.
    run_status: Option<RunStatusPayload>,
    /// The most recent lease refusal aimed at this guest: (doc, holder).
    denied: Option<(String, PeerIdentity)>,
}

impl GuestRole {
    fn new(session_id: String) -> Self {
        Self {
            session_id,
            host: None,
            leases: LeaseTable::default(),
            reassembler: ContentReassembler::default(),
            mirror_docs: HashMap::new(),
            held: HashMap::new(),
            mirroring: false,
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

/// The per-frame live-session pump. Owned by the application shell right
/// next to the cloud account service it drains.
pub(crate) struct LiveSessionEngine {
    connection: Option<Connection>,
    role: Role,
    peers: HashMap<PeerIdentity, PeerPresence>,
    /// Roster display names by principal, refreshed from the session
    /// snapshot; frames never carry names.
    roster: HashMap<uuid::Uuid, String>,
    host_principal: Option<uuid::Uuid>,
    presence_sent_at: Option<Instant>,
    /// The active schematic's content version as last reconciled into the
    /// workspace buffer map.
    active_synced_version: Option<(String, u64)>,
    /// An incompatible peer build was heard once this session.
    incompatible_peer: bool,
}

impl Default for LiveSessionEngine {
    fn default() -> Self {
        Self {
            connection: None,
            role: Role::Idle,
            peers: HashMap::new(),
            roster: HashMap::new(),
            host_principal: None,
            presence_sent_at: None,
            active_synced_version: None,
            incompatible_peer: false,
        }
    }
}

impl LiveSessionEngine {
    /// Drive the session for one frame: adopt fresh relay ports, drain
    /// inbound frames, broadcast local changes, and project write locks.
    pub(crate) fn pump(&mut self, state: &mut AppState, cloud: &mut CloudAccountService) {
        self.reconcile_role(state, cloud);
        if matches!(self.role, Role::Idle) {
            // Ports minted for a session that no longer renders are dead
            // credentials' leftovers; drop them.
            let _ = cloud.take_live_relay_port();
            return;
        }
        self.adopt_port(state, cloud);
        self.drain_inbound(state);
        self.broadcast_local_changes(state);
        self.stream_run_status(state);
        self.send_presence_heartbeat(state);
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

    /// Guest affordance: ask the host for one document's write lease.
    pub(crate) fn request_lease(&mut self, doc: &str) {
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        if matches!(self.role, Role::Guest(_)) {
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
        if matches!(self.role, Role::Guest(_)) {
            self.send(&LiveMessage::RunRequest(RunRequestPayload {
                sender: identity,
                run: RunIntent::Simulate,
            }));
        }
    }

    /// Guest lifecycle: adopt the host's incoming documents as this
    /// install's mirror. Called by the join flow once the user confirmed
    /// that joining replaces the open project.
    pub(crate) fn enter_mirror(&mut self) {
        if let Role::Guest(guest) = &mut self.role {
            guest.mirroring = true;
            // Ask for everything again so adoption starts complete.
            if let Some(identity) = self.connection.as_ref().map(|c| c.identity) {
                self.send(&LiveMessage::Document(DocumentMessage::SyncRequest {
                    sender: identity,
                    docs: None,
                }));
            }
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
        self.host_principal = summary
            .participants
            .iter()
            .find(|participant| participant.is_host)
            .and_then(|participant| participant.principal_id.parse().ok());

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
            Role::Guest(Box::new(GuestRole::new(summary.session_id.clone())))
        };
    }

    fn leave_session(&mut self, state: &mut AppState) {
        self.role = Role::Idle;
        self.connection = None;
        self.peers.clear();
        self.presence_sent_at = None;
        self.incompatible_peer = false;
        state.workbench.live_write_locks = LiveWriteLocks::default();
        // The read-only flag is this engine's to own; clearing it never
        // unlocks library or safe-mode gates, which are separate.
        state.schematic.read_only = false;
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
        if connection.port.outbound.send(message.encode()).is_err() {
            // The socket died; the cloud service reconnects and delivers a
            // fresh port, whose adoption rebroadcasts anything missed.
            connection.dead = true;
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
                Ok(message) => self.handle_message(state, message),
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
                if matches!(self.role, Role::Host(_)) {
                    let requester = self.display_name(&payload.sender);
                    state.push_user_message(ConsoleMessage::info(format!(
                        "{requester} requested a run from the live session."
                    )));
                    state.request_run_set_simulation();
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
                let reply = if !host.tracked.contains_key(&doc) {
                    DocumentMessage::LeaseDeny {
                        doc,
                        requester: sender,
                        holder: identity,
                    }
                } else {
                    match host.leases.arbitrate(&doc, sender) {
                        LeaseDecision::Granted => DocumentMessage::LeaseGrant { doc, holder: sender },
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
                let authorized = host
                    .leases
                    .holder(&header.doc)
                    .is_some_and(|holder| {
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
                    Err(error) => log::warn!(
                        "live proposal for {} rejected: {error:?}",
                        header.doc
                    ),
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
        for doc in &current {
            host.tracked.entry(doc.clone()).or_insert_with(|| TrackedDoc {
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
        if only_if_changed
            && host_document_version(state, doc) == Some(tracked.seen_version)
        {
            return;
        }
        let Some(bytes) = serialize_host_document(state, doc) else {
            return;
        };
        let digest = content_digest(&bytes);
        let version = host_document_version(state, doc).unwrap_or(0);
        let tracked = host.tracked.get_mut(doc).expect("present above");
        tracked.seen_version = version;
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
                let set_changed = before.len() != after.len()
                    || !before.iter().all(|doc| after.contains(doc));
                if set_changed {
                    self.broadcast_manifest();
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
        let running = state.simulation.is_running;
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
                guest
                    .mirror_docs
                    .retain(|doc, _| named.contains(doc));
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
            log::debug!("live document {} not in the manifest; dropped", completed.doc);
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
        self.sync_active_buffer(state);
        let Some(identity) = self.connection.as_ref().map(|c| c.identity) else {
            return;
        };
        let held_docs: Vec<String> = match &self.role {
            Role::Guest(guest) => guest.held.keys().cloned().collect(),
            _ => return,
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

    fn prune_peers(&mut self) {
        self.peers
            .retain(|_, presence| presence.seen_at.elapsed() < PEER_TIMEOUT);
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
                if guest.mirroring {
                    let host_name = self
                        .host_principal
                        .and_then(|principal| self.roster.get(&principal).cloned())
                        .unwrap_or_else(|| "The host".to_owned());
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
    if doc == NETLIST_DOC {
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
