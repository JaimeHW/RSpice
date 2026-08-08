//! Background executor for the cloud account session (native).
//!
//! One thread owns every credential and network interaction: the OIDC tokens,
//! the typed cloud client, the protected session record, and the
//! client-integration startup sequence (principal → entitlements → workspaces
//! → native license lease). The UI thread only ever receives owned
//! [`CloudSessionSnapshot`] values, so no token or cloud DTO can leak into
//! rendered or persisted application state.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};

use rspice_cloud_client::contract::{
    Entitlement, IssueLicenseLeaseRequest, LicenseLease, LiveSession, LiveSessionAdmission,
    LiveSessionCapability, LiveSessionPolicy, LiveSessionTicketProtocol,
    UpdateLiveSessionParticipantRequest,
};
use rspice_cloud_client::{
    BearerToken, ClientConfig, CloudClient, CloudError, IdempotencyKey, NativeLicenseVerifier,
    PageRequest, VerifiedNativeLicense,
};

use super::config::CloudAccountConfig;
use super::{
    CloudAccountCommand, CloudAccountEvent, CloudSessionPhase, CloudSessionSnapshot,
    EntitlementSummary, LeaseSummary, LiveParticipantSummary, LiveRelayClosure, LiveRelayPort,
    LiveSessionPolicySummary, LiveSessionState, LiveSessionSummary, NativeLicenseSummary,
    PrincipalSummary, PublicationSummary, PublishState, WorkspaceSummary, live_relay,
    native_login, oidc, publish, store,
};

/// Collection page size for bootstrap reads.
const PAGE_LIMIT: usize = 100;
/// Allowed verifier clock skew, release policy per the integration contract.
const LEASE_CLOCK_SKEW: std::time::Duration = std::time::Duration::from_secs(60);
/// A fresh lease must be usable at least this long after verification.
const LEASE_MINIMUM_VALIDITY: std::time::Duration = std::time::Duration::from_secs(5 * 60);
/// Renew the lease once less than this much validity remains.
const LEASE_RENEWAL_WINDOW_SECONDS: i64 = 3 * 24 * 60 * 60;
/// Idle loop period when nothing is scheduled.
const IDLE_PERIOD: std::time::Duration = std::time::Duration::from_secs(3600);
/// Retry period after an unreachable-network refresh failure.
const OFFLINE_RETRY_PERIOD: std::time::Duration = std::time::Duration::from_secs(120);
/// Poll cadence while a just-published page is still preparing (the server's
/// own placeholder advertises `Retry-After: 5`).
const PAGE_POLL_PERIOD: std::time::Duration = std::time::Duration::from_secs(5);
/// Roster poll cadence while a live session is hosted or joined. Presence
/// and streaming arrive over the relay connection; this poll carries only
/// membership, admissions, and policy.
const LIVE_SESSION_POLL_PERIOD: std::time::Duration = std::time::Duration::from_secs(5);
/// Pause before re-establishing an interrupted relay connection. Every
/// reconnect re-mints a single-use ticket over HTTP, so this keeps a flapping
/// network from turning into a join-request loop.
const RELAY_RESUME_DELAY: std::time::Duration = std::time::Duration::from_secs(2);

/// Spawn the executor thread; it restores any stored session immediately.
pub(super) fn spawn(
    configuration: CloudAccountConfig,
    repaint: Option<egui::Context>,
) -> (
    Sender<CloudAccountCommand>,
    Receiver<CloudAccountEvent>,
    Receiver<LiveRelayPort>,
) {
    let (command_sender, command_receiver) = std::sync::mpsc::channel();
    let (event_sender, event_receiver) = std::sync::mpsc::channel();
    let (port_sender, port_receiver) = std::sync::mpsc::channel();
    let listener_commands = command_sender.clone();
    std::thread::Builder::new()
        .name("rspice-cloud-account".to_owned())
        .spawn(move || {
            run(
                configuration,
                repaint,
                command_receiver,
                listener_commands,
                event_sender,
                port_sender,
            );
        })
        .expect("spawning the cloud account executor thread");
    (command_sender, event_receiver, port_receiver)
}

fn run(
    configuration: CloudAccountConfig,
    repaint: Option<egui::Context>,
    commands: Receiver<CloudAccountCommand>,
    listener_commands: Sender<CloudAccountCommand>,
    events: Sender<CloudAccountEvent>,
    relay_ports: Sender<LiveRelayPort>,
) {
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(runtime) => runtime,
        Err(_) => return,
    };
    let client_config = match build_client_config(&configuration) {
        Ok(client_config) => client_config,
        Err(message) => {
            let snapshot = CloudSessionSnapshot {
                phase: CloudSessionPhase::SignedOut {
                    last_error: Some(message),
                },
                ..CloudSessionSnapshot::default()
            };
            let _ = events.send(CloudAccountEvent::Snapshot(Box::new(snapshot)));
            return;
        }
    };
    let Ok(cloud) = CloudClient::new(client_config) else {
        return;
    };
    let Ok(http) = oidc::identity_http_client() else {
        return;
    };

    let mut executor = Executor {
        configuration,
        http,
        cloud,
        endpoints: None,
        record: store::load(),
        access: None,
        pending_sign_in: None,
        page_poll: None,
        principal_id: None,
        live_session: None,
        relay: None,
        relay_generation: 0,
        relay_retry_at: None,
        relay_blocked: false,
        relay_ports,
        snapshot: CloudSessionSnapshot::default(),
        next_deadline: None,
        listener_commands,
        events,
        repaint,
        runtime,
    };
    executor.restore();
    loop {
        let timeout = executor
            .next_deadline
            .map(|deadline| deadline.saturating_duration_since(std::time::Instant::now()))
            .unwrap_or(IDLE_PERIOD)
            .max(std::time::Duration::from_millis(250));
        match commands.recv_timeout(timeout) {
            Ok(command) => executor.handle(command),
            Err(RecvTimeoutError::Timeout) => executor.on_deadline(),
            Err(RecvTimeoutError::Disconnected) => return,
        }
    }
}

fn build_client_config(configuration: &CloudAccountConfig) -> Result<ClientConfig, String> {
    let build = if configuration.development {
        ClientConfig::loopback_development(
            &configuration.api_origin,
            &configuration.object_storage_origin,
        )
    } else {
        ClientConfig::production(
            &configuration.api_origin,
            &configuration.object_storage_origin,
        )
    };
    build.map_err(|_| "The cloud endpoints this build carries are invalid.".to_owned())
}

struct AccessToken {
    token: String,
    refresh_at: std::time::Instant,
}

struct PendingSignIn {
    state: String,
    verifier: String,
    redirect_uri: String,
    cancel: Arc<AtomicBool>,
}

/// Executor-held live-session facts that must never reach the UI: the
/// replay handle, the presented join code, and the connection identity.
struct LiveSessionRuntime {
    session_id: uuid::Uuid,
    hosting: bool,
    /// Idempotency key of this hosting run: replaying it rotates the join
    /// code, so it lives exactly as long as the hosted session.
    create_key: Option<String>,
    /// The code this guest presented, held for the admitted re-join that
    /// mints a connect ticket when the relay connection is established.
    presented_code: Option<String>,
    /// Stable per-run connection identity echoed by every issued ticket.
    client_instance_id: uuid::Uuid,
    /// The display-form join code, held only while hosting.
    join_code: Option<String>,
}

/// One spawned relay socket, identified by its generation for the lifetime
/// notifications the socket thread sends back.
struct RelayRuntime {
    generation: u64,
    /// Whether the socket finished its handshake and is streaming.
    attached: bool,
    /// Dropping or firing this asks the socket thread to close and exit.
    stop: tokio::sync::oneshot::Sender<()>,
}

struct Executor {
    configuration: CloudAccountConfig,
    http: reqwest::Client,
    cloud: CloudClient,
    endpoints: Option<oidc::ProviderEndpoints>,
    record: Option<store::CloudSessionRecord>,
    access: Option<AccessToken>,
    pending_sign_in: Option<PendingSignIn>,
    /// A published page whose render is still preparing: (circuit, url_path).
    page_poll: Option<(uuid::Uuid, String)>,
    /// The signed-in principal, for marking the roster's self and host rows.
    principal_id: Option<uuid::Uuid>,
    live_session: Option<LiveSessionRuntime>,
    /// The relay socket of the current session, when one is spawned.
    relay: Option<RelayRuntime>,
    /// Monotonic socket counter; stale socket notifications carry old values.
    relay_generation: u64,
    /// No relay (re)connection attempt before this instant.
    relay_retry_at: Option<std::time::Instant>,
    /// A relay closure blamed this client (protocol defect); reconnecting
    /// would loop, so attempts pause until the next explicit user action.
    relay_blocked: bool,
    /// Hands each attached socket's duplex port to the UI thread.
    relay_ports: Sender<LiveRelayPort>,
    snapshot: CloudSessionSnapshot,
    next_deadline: Option<std::time::Instant>,
    listener_commands: Sender<CloudAccountCommand>,
    events: Sender<CloudAccountEvent>,
    repaint: Option<egui::Context>,
    runtime: tokio::runtime::Runtime,
}

impl Executor {
    // -- event plumbing ----------------------------------------------------

    fn publish(&mut self) {
        let _ = self
            .events
            .send(CloudAccountEvent::Snapshot(Box::new(self.snapshot.clone())));
        if let Some(repaint) = &self.repaint {
            repaint.request_repaint();
        }
    }

    fn open_browser(&mut self, url: String) {
        let _ = self.events.send(CloudAccountEvent::OpenBrowser(url));
        if let Some(repaint) = &self.repaint {
            repaint.request_repaint();
        }
    }

    fn set_signed_out(&mut self, error: Option<String>) {
        self.access = None;
        self.principal_id = None;
        self.stop_live_relay();
        self.relay_blocked = false;
        self.live_session = None;
        self.snapshot = CloudSessionSnapshot {
            phase: CloudSessionPhase::SignedOut { last_error: error },
            ..CloudSessionSnapshot::default()
        };
        self.next_deadline = None;
        self.publish();
    }

    // -- startup -----------------------------------------------------------

    fn restore(&mut self) {
        // Offline lease first: it needs no network and settles what this
        // installation may do before the first server contact.
        if let Some(license) = self.verify_stored_lease() {
            let record = self.record.as_ref().expect("lease implies a record");
            self.snapshot = CloudSessionSnapshot {
                phase: CloudSessionPhase::OfflineLicensed,
                principal: Some(PrincipalSummary {
                    email: record.principal_email.clone(),
                    display_name: record.principal_display_name.clone(),
                }),
                native_license: Some(native_license_summary(&license)),
                ..CloudSessionSnapshot::default()
            };
            self.publish();
        }
        let has_refresh_token = self
            .record
            .as_ref()
            .is_some_and(|record| record.refresh_token.is_some());
        if has_refresh_token {
            self.establish_online();
        } else if self.snapshot.phase == (CloudSessionPhase::SignedOut { last_error: None }) {
            self.publish();
        }
    }

    fn verify_stored_lease(&mut self) -> Option<VerifiedNativeLicense> {
        let record = self.record.as_ref()?;
        let token = record.lease_token.clone()?;
        let jwks_json = record.license_jwks.clone()?;
        let fingerprint = record.device_fingerprint_sha256()?;
        let jwks = serde_json::from_str(&jwks_json).ok()?;
        let verifier = NativeLicenseVerifier::new(
            &jwks,
            &self.configuration.license_issuer,
            &self.configuration.license_audience,
            &self.configuration.license_product,
            LEASE_CLOCK_SKEW,
        )
        .ok()?;
        match verifier.verify_token(&token, &fingerprint, std::time::Duration::ZERO) {
            Ok(license) => Some(license),
            Err(_) => {
                // An unverifiable stored lease is dead weight; drop it so the
                // next online bootstrap issues a fresh one.
                if let Some(record) = self.record.as_mut() {
                    record.lease_token = None;
                    let _ = store::save(record);
                }
                None
            }
        }
    }

    // -- sign-in -----------------------------------------------------------

    fn handle(&mut self, command: CloudAccountCommand) {
        match command {
            CloudAccountCommand::SignIn => self.begin_sign_in(),
            CloudAccountCommand::CancelSignIn => self.cancel_sign_in(),
            CloudAccountCommand::SignOut => self.sign_out(),
            CloudAccountCommand::Refresh => self.establish_online(),
            CloudAccountCommand::RevokeLease { lease_id } => self.revoke_lease(&lease_id),
            CloudAccountCommand::PublishSnapshot { request } => self.publish_snapshot(&request),
            CloudAccountCommand::RefreshPublications { circuit_id } => {
                if let Ok(parsed) = circuit_id.parse() {
                    self.refresh_publications(parsed);
                }
            }
            CloudAccountCommand::Unpublish {
                circuit_id,
                publication_id,
            } => self.unpublish(&circuit_id, &publication_id),
            CloudAccountCommand::StartLiveSession { policy, circuit_id } => {
                self.start_live_session(policy, circuit_id.as_deref());
            }
            CloudAccountCommand::RegenerateLiveSessionCode => self.regenerate_live_session_code(),
            CloudAccountCommand::JoinLiveSession { code } => self.join_live_session(&code),
            CloudAccountCommand::RefreshLiveSession => {
                // An explicit refresh is the user's way back from a blocked
                // relay: lift the pause, then let the roster read reattach.
                self.relay_blocked = false;
                self.relay_retry_at = None;
                self.refresh_live_session();
            }
            CloudAccountCommand::ApplyLiveSessionPolicy { policy } => {
                self.apply_live_session_policy(policy);
            }
            CloudAccountCommand::ApproveLiveSessionParticipant { principal_id } => {
                self.approve_live_session_participant(&principal_id);
            }
            CloudAccountCommand::SetLiveSessionParticipantEditor {
                principal_id,
                editor,
            } => self.set_live_session_participant_editor(&principal_id, editor),
            CloudAccountCommand::RemoveLiveSessionParticipant { principal_id } => {
                self.remove_live_session_participant(&principal_id);
            }
            CloudAccountCommand::EndLiveSession => self.end_live_session(),
            CloudAccountCommand::LeaveLiveSession => self.leave_live_session(),
            CloudAccountCommand::CompleteSignIn { code, state } => {
                self.complete_sign_in(&code, &state);
            }
            CloudAccountCommand::SignInFailed { reason } => {
                if self.pending_sign_in.take().is_some() {
                    self.set_signed_out(Some(reason));
                }
            }
            CloudAccountCommand::LiveRelayAttached { generation } => {
                if let Some(relay) = self
                    .relay
                    .as_mut()
                    .filter(|relay| relay.generation == generation)
                {
                    relay.attached = true;
                    self.relay_retry_at = None;
                    self.set_relay_connected(true);
                }
            }
            CloudAccountCommand::LiveRelayClosed {
                generation,
                closure,
            } => self.on_live_relay_closed(generation, closure),
        }
    }

    fn begin_sign_in(&mut self) {
        if matches!(
            self.snapshot.phase,
            CloudSessionPhase::WaitingForBrowser
                | CloudSessionPhase::ExchangingTokens
                | CloudSessionPhase::Bootstrapping
        ) {
            return;
        }
        let endpoints = match self.provider_endpoints() {
            Ok(endpoints) => endpoints,
            Err(error) => {
                self.set_signed_out(Some(error.to_string()));
                return;
            }
        };
        let (pkce, state) = match (oidc::pkce_pair(), oidc::state_value()) {
            (Ok(pkce), Ok(state)) => (pkce, state),
            _ => {
                self.set_signed_out(Some("No entropy source for sign-in.".to_owned()));
                return;
            }
        };
        let listener =
            match native_login::LoopbackListener::bind(&self.configuration.loopback_ports) {
                Ok(listener) => listener,
                Err(_) => {
                    self.set_signed_out(Some(
                        "No local sign-in port is free; close other RSpice sign-ins and retry."
                            .to_owned(),
                    ));
                    return;
                }
            };
        let redirect_uri = listener.redirect_uri();
        let url = match oidc::authorization_url(
            &endpoints,
            &self.configuration.oidc_client_id,
            &redirect_uri,
            &state,
            &pkce,
        ) {
            Ok(url) => url,
            Err(error) => {
                self.set_signed_out(Some(error.to_string()));
                return;
            }
        };

        let cancel = Arc::new(AtomicBool::new(false));
        {
            let cancel = Arc::clone(&cancel);
            let commands = self.listener_commands.clone();
            let expected_state = state.clone();
            std::thread::Builder::new()
                .name("rspice-cloud-signin".to_owned())
                .spawn(move || listener.run(expected_state, cancel, commands))
                .expect("spawning the sign-in listener thread");
        }
        self.pending_sign_in = Some(PendingSignIn {
            state,
            verifier: pkce.verifier,
            redirect_uri,
            cancel,
        });
        self.snapshot.phase = CloudSessionPhase::WaitingForBrowser;
        self.snapshot.authorization_url = Some(url.clone());
        self.publish();
        self.open_browser(url);
    }

    fn cancel_sign_in(&mut self) {
        if let Some(pending) = self.pending_sign_in.take() {
            pending.cancel.store(true, Ordering::Relaxed);
            self.set_signed_out(None);
        }
    }

    fn complete_sign_in(&mut self, code: &str, state: &str) {
        let Some(pending) = self.pending_sign_in.take() else {
            return;
        };
        pending.cancel.store(true, Ordering::Relaxed);
        if pending.state != state {
            self.set_signed_out(Some("The sign-in response did not match.".to_owned()));
            return;
        }
        self.snapshot.phase = CloudSessionPhase::ExchangingTokens;
        self.snapshot.authorization_url = None;
        self.publish();

        let endpoints = match self.provider_endpoints() {
            Ok(endpoints) => endpoints,
            Err(error) => {
                self.set_signed_out(Some(error.to_string()));
                return;
            }
        };
        let grant = self.runtime.block_on(oidc::exchange_code(
            &self.http,
            &endpoints,
            &self.configuration.oidc_client_id,
            &pending.redirect_uri,
            code,
            &pending.verifier,
        ));
        match grant {
            Ok(grant) => {
                self.adopt_grant(grant);
                self.bootstrap();
            }
            Err(error) => self.set_signed_out(Some(error.to_string())),
        }
    }

    fn provider_endpoints(&mut self) -> Result<oidc::ProviderEndpoints, oidc::IdentityError> {
        if let Some(endpoints) = &self.endpoints {
            return Ok(endpoints.clone());
        }
        let endpoints = self.runtime.block_on(oidc::discover(
            &self.http,
            &self.configuration.oidc_issuer,
            self.configuration.development,
        ))?;
        self.endpoints = Some(endpoints.clone());
        Ok(endpoints)
    }

    /// Hold the grant in memory and persist the rotated refresh token.
    fn adopt_grant(&mut self, grant: oidc::TokenGrant) {
        let lifetime = std::time::Duration::from_secs(grant.expires_in);
        self.access = Some(AccessToken {
            token: grant.access_token,
            refresh_at: std::time::Instant::now() + lifetime.mul_f32(0.6),
        });
        if let Some(refresh_token) = grant.refresh_token {
            let record = self.record.get_or_insert_with(|| {
                store::CloudSessionRecord::new_with_binding_secret()
                    .expect("entropy for the device binding secret")
            });
            record.refresh_token = Some(refresh_token);
            if let Err(error) = store::save(record) {
                log::warn!("cloud session record save failed: {error}");
            }
        }
        self.next_deadline = self.access.as_ref().map(|access| access.refresh_at);
    }

    // -- session maintenance -----------------------------------------------

    fn establish_online(&mut self) {
        let Some(refresh_token) = self
            .record
            .as_ref()
            .and_then(|record| record.refresh_token.clone())
        else {
            return;
        };
        let endpoints = match self.provider_endpoints() {
            Ok(endpoints) => endpoints,
            Err(oidc::IdentityError::Unreachable) => {
                self.schedule_offline_retry();
                return;
            }
            Err(error) => {
                self.set_signed_out(Some(error.to_string()));
                return;
            }
        };
        let grant = self.runtime.block_on(oidc::refresh_grant(
            &self.http,
            &endpoints,
            &self.configuration.oidc_client_id,
            &refresh_token,
        ));
        match grant {
            Ok(grant) => {
                self.adopt_grant(grant);
                self.bootstrap();
            }
            Err(oidc::IdentityError::Unreachable) => self.schedule_offline_retry(),
            Err(oidc::IdentityError::Rejected) => {
                // The provider revoked the session: wipe tokens, keep the
                // device binding secret so this install's identity is stable.
                if let Some(record) = self.record.as_mut() {
                    record.refresh_token = None;
                    record.lease_token = None;
                    let _ = store::save(record);
                }
                self.set_signed_out(Some(
                    "Your session has expired; sign in again to reconnect.".to_owned(),
                ));
            }
            Err(error) => self.set_signed_out(Some(error.to_string())),
        }
    }

    fn schedule_offline_retry(&mut self) {
        self.next_deadline = Some(std::time::Instant::now() + OFFLINE_RETRY_PERIOD);
        // OfflineLicensed (or a prior Active snapshot) stays presented as-is.
    }

    fn on_deadline(&mut self) {
        self.next_deadline = None;
        if let Some((circuit_id, _)) = self.page_poll {
            self.refresh_publications(circuit_id);
        }
        if self.live_session.is_some() {
            self.refresh_live_session();
        }
        let due = self
            .access
            .as_ref()
            .is_none_or(|access| std::time::Instant::now() >= access.refresh_at);
        if due {
            self.establish_online();
        } else {
            self.next_deadline = self.access.as_ref().map(|access| access.refresh_at);
        }
        self.arm_page_poll();
        self.arm_live_session_poll();
        self.arm_live_relay_retry();
    }

    /// Keep the wake-up early enough for a pending page-status poll.
    fn arm_page_poll(&mut self) {
        if self.page_poll.is_some() {
            let poll_at = std::time::Instant::now() + PAGE_POLL_PERIOD;
            self.next_deadline = Some(match self.next_deadline {
                Some(existing) => existing.min(poll_at),
                None => poll_at,
            });
        }
    }

    /// Keep the wake-up early enough for the live-session roster poll.
    fn arm_live_session_poll(&mut self) {
        if self.live_session.is_some() {
            let poll_at = std::time::Instant::now() + LIVE_SESSION_POLL_PERIOD;
            self.next_deadline = Some(match self.next_deadline {
                Some(existing) => existing.min(poll_at),
                None => poll_at,
            });
        }
    }

    /// Keep the wake-up early enough for a pending relay reconnection.
    fn arm_live_relay_retry(&mut self) {
        if self.live_session.is_some()
            && self.relay.is_none()
            && !self.relay_blocked
            && let Some(retry_at) = self.relay_retry_at
        {
            self.next_deadline = Some(match self.next_deadline {
                Some(existing) => existing.min(retry_at),
                None => retry_at,
            });
        }
    }

    // -- publishing ----------------------------------------------------------

    fn publish_snapshot(&mut self, request: &super::PublishRequest) {
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            self.snapshot.publish = Some(PublishState::Failed {
                message: "Sign in before publishing.".to_owned(),
            });
            self.publish();
            return;
        };
        self.snapshot.publish = Some(PublishState::Publishing);
        self.publish();
        let outcome = self
            .runtime
            .block_on(publish::publish(&self.cloud, &access, request));
        match outcome {
            Ok(receipt) => {
                self.snapshot.publish = Some(PublishState::AwaitingPage {
                    url_path: receipt.url_path.clone(),
                });
                if let Ok(circuit_id) = receipt.circuit_id.parse() {
                    self.page_poll = Some((circuit_id, receipt.url_path.clone()));
                }
                let _ = self
                    .events
                    .send(CloudAccountEvent::PublishCompleted(Box::new(receipt)));
                self.publish();
                if let Some((circuit_id, _)) = self.page_poll {
                    self.refresh_publications(circuit_id);
                }
                self.arm_page_poll();
            }
            Err(error) => {
                self.snapshot.publish = Some(PublishState::Failed {
                    message: error.to_string(),
                });
                self.publish();
            }
        }
    }

    fn refresh_publications(&mut self, circuit_id: uuid::Uuid) {
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            return;
        };
        let Some(rows) = self
            .runtime
            .block_on(read_publications(&self.cloud, &access, circuit_id))
        else {
            return;
        };
        if let Some((_, url_path)) = self.page_poll.clone() {
            let status = rows
                .iter()
                .find(|publication| publication.url_path == url_path)
                .map(|publication| publication.page_status.clone());
            match status.as_deref() {
                Some("live") => {
                    self.snapshot.publish = Some(PublishState::Live { url_path });
                    self.page_poll = None;
                }
                Some("failed") => {
                    self.snapshot.publish = Some(PublishState::Failed {
                        message: "The page could not be rendered; the publication was not \
                                  taken live."
                            .to_owned(),
                    });
                    self.page_poll = None;
                }
                _ => {}
            }
        }
        self.snapshot.publications = rows;
        self.publish();
    }

    fn unpublish(&mut self, circuit_id: &str, publication_id: &str) {
        let (Ok(circuit), Ok(publication)) = (
            circuit_id.parse::<uuid::Uuid>(),
            publication_id.parse::<uuid::Uuid>(),
        ) else {
            return;
        };
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            return;
        };
        let revoked = self.runtime.block_on(async {
            let token = BearerToken::new(&access).map_err(|_| ())?;
            self.cloud
                .unpublish_circuit_publication(&token, circuit, publication)
                .await
                .map_err(|_| ())
        });
        if revoked.is_ok() {
            self.refresh_publications(circuit);
        }
    }

    // -- live sessions -------------------------------------------------------

    fn start_live_session(&mut self, policy: LiveSessionPolicySummary, circuit_id: Option<&str>) {
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            self.snapshot.live_session = Some(LiveSessionState::Failed {
                message: "Sign in before going live.".to_owned(),
            });
            self.publish();
            return;
        };
        // The binding is provenance only and grants nothing; a malformed one
        // must not block going live on a local project.
        let circuit = circuit_id.and_then(|raw| raw.parse().ok());
        let create_key = uuid::Uuid::now_v7().to_string();
        let client_instance_id = uuid::Uuid::now_v7();
        self.stop_live_relay();
        self.relay_blocked = false;
        self.snapshot.live_session = Some(LiveSessionState::Starting);
        self.publish();
        let outcome = self.runtime.block_on(create_live_session(
            &self.cloud,
            &access,
            &create_key,
            contract_policy(policy),
            circuit,
            client_instance_id,
        ));
        match outcome {
            Ok(created) => {
                self.live_session = Some(LiveSessionRuntime {
                    session_id: created.session.id,
                    hosting: true,
                    create_key: Some(create_key),
                    presented_code: None,
                    client_instance_id,
                    join_code: Some(created.join_code.clone()),
                });
                let summary = live_session_summary(
                    &created.session,
                    self.principal_id,
                    true,
                    Some(created.join_code),
                );
                self.snapshot.live_session = Some(LiveSessionState::Hosting(summary));
                self.publish();
                self.arm_live_session_poll();
                // The creation ticket is single-use and short-lived, minted
                // exactly for this moment: attach the relay with it now.
                self.attach_live_relay(&created.ticket);
            }
            Err(message) => {
                self.snapshot.live_session = Some(LiveSessionState::Failed { message });
                self.publish();
            }
        }
    }

    /// Replay the hosting run's idempotency key: the server rotates the join
    /// code, and an over-age session is superseded by a fresh one instead.
    fn regenerate_live_session_code(&mut self) {
        let Some(runtime) = self.live_session.as_ref() else {
            return;
        };
        if !runtime.hosting {
            return;
        }
        let Some(create_key) = runtime.create_key.clone() else {
            return;
        };
        let client_instance_id = runtime.client_instance_id;
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            return;
        };
        let policy = match &self.snapshot.live_session {
            Some(
                LiveSessionState::Hosting(summary)
                | LiveSessionState::AwaitingApproval(summary)
                | LiveSessionState::Participating(summary),
            ) => summary.policy,
            _ => return,
        };
        self.relay_blocked = false;
        let outcome = self.runtime.block_on(create_live_session(
            &self.cloud,
            &access,
            &create_key,
            contract_policy(policy),
            None,
            client_instance_id,
        ));
        match outcome {
            Ok(created) => {
                let superseded = self
                    .live_session
                    .as_ref()
                    .is_some_and(|runtime| runtime.session_id != created.session.id);
                if let Some(runtime) = self.live_session.as_mut() {
                    runtime.session_id = created.session.id;
                    runtime.join_code = Some(created.join_code.clone());
                }
                let mut summary = live_session_summary(
                    &created.session,
                    self.principal_id,
                    true,
                    Some(created.join_code),
                );
                summary.relay_connected =
                    self.relay.as_ref().is_some_and(|relay| relay.attached);
                self.snapshot.live_session = Some(LiveSessionState::Hosting(summary));
                self.publish();
                // A rotation keeps current participants connected, but an
                // over-age session is superseded by a fresh one whose relay
                // room is new; the replay ticket exists for exactly that.
                if superseded || self.relay.is_none() {
                    self.attach_live_relay(&created.ticket);
                }
            }
            Err(message) => self.set_live_notice(message),
        }
    }

    fn join_live_session(&mut self, code: &str) {
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            self.snapshot.live_session = Some(LiveSessionState::Failed {
                message: "Sign in before joining a live session.".to_owned(),
            });
            self.publish();
            return;
        };
        let client_instance_id = uuid::Uuid::now_v7();
        self.stop_live_relay();
        self.relay_blocked = false;
        self.snapshot.live_session = Some(LiveSessionState::Joining);
        self.publish();
        let outcome = self.runtime.block_on(join_live_session(
            &self.cloud,
            &access,
            code,
            client_instance_id,
        ));
        match outcome {
            Ok(joined) => {
                self.live_session = Some(LiveSessionRuntime {
                    session_id: joined.session.id,
                    hosting: false,
                    create_key: None,
                    presented_code: Some(code.to_owned()),
                    client_instance_id,
                    join_code: None,
                });
                let summary = live_session_summary(&joined.session, self.principal_id, false, None);
                let state = if joined.admission == LiveSessionAdmission::Pending {
                    LiveSessionState::AwaitingApproval(summary)
                } else {
                    LiveSessionState::Participating(summary)
                };
                self.snapshot.live_session = Some(state);
                self.publish();
                self.arm_live_session_poll();
                // An admitted join carries its connect ticket; a pending one
                // attaches later, when the roster poll sees the approval.
                if let Some(ticket) = joined.ticket.as_ref() {
                    self.attach_live_relay(ticket);
                }
            }
            Err(message) => {
                self.snapshot.live_session = Some(LiveSessionState::Failed { message });
                self.publish();
            }
        }
    }

    fn refresh_live_session(&mut self) {
        let Some(runtime) = self.live_session.as_ref() else {
            return;
        };
        let session_id = runtime.session_id;
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            return;
        };
        let outcome = self
            .runtime
            .block_on(read_live_session(&self.cloud, &access, session_id));
        match outcome {
            Ok(session) => self.publish_live_state_from(&session),
            Err(LiveSessionReadFailure::Gone) => {
                // The server no longer admits us: the session ended, aged
                // out, or we were removed — indistinguishable by design.
                self.stop_live_relay();
                self.relay_blocked = false;
                self.live_session = None;
                self.snapshot.live_session = Some(LiveSessionState::Failed {
                    message: "This live session has ended.".to_owned(),
                });
                self.publish();
            }
            Err(LiveSessionReadFailure::Transient) => {}
        }
    }

    fn apply_live_session_policy(&mut self, policy: LiveSessionPolicySummary) {
        let Some(runtime) = self.live_session.as_ref() else {
            return;
        };
        if !runtime.hosting {
            return;
        }
        let session_id = runtime.session_id;
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            return;
        };
        let outcome = self.runtime.block_on(async {
            let token =
                BearerToken::new(&access).map_err(|_| LIVE_SESSION_UNREACHABLE.to_owned())?;
            self.cloud
                .update_live_session_policy(&token, session_id, contract_policy(policy))
                .await
                .map(rspice_cloud_client::CloudResponse::into_body)
                .map_err(|error| live_session_error_message(&error))
        });
        match outcome {
            Ok(session) => self.publish_live_state_from(&session),
            Err(message) => self.set_live_notice(message),
        }
    }

    fn approve_live_session_participant(&mut self, principal_id: &str) {
        self.manage_live_participant(principal_id, LiveParticipantAction::Approve);
    }

    fn set_live_session_participant_editor(&mut self, principal_id: &str, editor: bool) {
        self.manage_live_participant(principal_id, LiveParticipantAction::SetEditor(editor));
    }

    fn remove_live_session_participant(&mut self, principal_id: &str) {
        self.manage_live_participant(principal_id, LiveParticipantAction::Remove);
    }

    fn manage_live_participant(&mut self, principal_id: &str, action: LiveParticipantAction) {
        let Some(runtime) = self.live_session.as_ref() else {
            return;
        };
        if !runtime.hosting {
            return;
        }
        let session_id = runtime.session_id;
        let Ok(participant) = principal_id.parse::<uuid::Uuid>() else {
            return;
        };
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            return;
        };
        let outcome = self.runtime.block_on(async {
            let token =
                BearerToken::new(&access).map_err(|_| LIVE_SESSION_UNREACHABLE.to_owned())?;
            let result = match action {
                LiveParticipantAction::Approve => {
                    self.cloud
                        .approve_live_session_participant(&token, session_id, participant)
                        .await
                }
                LiveParticipantAction::SetEditor(editor) => {
                    self.cloud
                        .update_live_session_participant(
                            &token,
                            session_id,
                            participant,
                            &UpdateLiveSessionParticipantRequest {
                                capability: if editor {
                                    LiveSessionCapability::Edit
                                } else {
                                    LiveSessionCapability::View
                                },
                            },
                        )
                        .await
                }
                LiveParticipantAction::Remove => {
                    self.cloud
                        .remove_live_session_participant(&token, session_id, participant)
                        .await
                }
            };
            result
                .map(|_| ())
                .map_err(|error| live_session_error_message(&error))
        });
        match outcome {
            Ok(()) => self.refresh_live_session(),
            Err(message) => self.set_live_notice(message),
        }
    }

    fn end_live_session(&mut self) {
        let Some(runtime) = self.live_session.as_ref() else {
            return;
        };
        if !runtime.hosting {
            return;
        }
        let session_id = runtime.session_id;
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            return;
        };
        let outcome = self.runtime.block_on(async {
            let token =
                BearerToken::new(&access).map_err(|_| LIVE_SESSION_UNREACHABLE.to_owned())?;
            self.cloud
                .end_live_session(&token, session_id)
                .await
                .map(|_| ())
                .map_err(|error| live_session_error_message(&error))
        });
        match outcome {
            Ok(()) => {
                self.stop_live_relay();
                self.relay_blocked = false;
                self.live_session = None;
                self.snapshot.live_session = None;
                self.publish();
            }
            Err(message) => self.set_live_notice(message),
        }
    }

    fn leave_live_session(&mut self) {
        let Some(runtime) = self.live_session.as_ref() else {
            return;
        };
        if runtime.hosting {
            return;
        }
        // Guests hold no end authority and the service has no leave route by
        // design: membership is the roster's record, presence leaves with
        // the relay connection.
        self.stop_live_relay();
        self.relay_blocked = false;
        self.live_session = None;
        self.snapshot.live_session = None;
        self.publish();
    }

    /// Rebuild the rendered state from a fresh server projection.
    fn publish_live_state_from(&mut self, session: &LiveSession) {
        let Some(runtime) = self.live_session.as_ref() else {
            return;
        };
        let mut summary = live_session_summary(
            session,
            self.principal_id,
            runtime.hosting,
            runtime.join_code.clone(),
        );
        summary.relay_connected = self.relay.as_ref().is_some_and(|relay| relay.attached);
        let self_pending = self.principal_id.is_some_and(|own| {
            session.participants.iter().any(|participant| {
                participant.principal_id == own
                    && participant.admission == LiveSessionAdmission::Pending
            })
        });
        let state = if runtime.hosting {
            LiveSessionState::Hosting(summary)
        } else if self_pending {
            LiveSessionState::AwaitingApproval(summary)
        } else {
            LiveSessionState::Participating(summary)
        };
        self.snapshot.live_session = Some(state);
        self.publish();
        // Every fresh projection is a chance to (re)attach: a just-approved
        // guest, a healed network, or an elapsed retry pause all pass here.
        self.ensure_live_relay();
    }

    // -- live relay ----------------------------------------------------------

    /// Spawn the relay socket for a freshly minted connect ticket and hand
    /// the workbench its duplex port. Tickets are single-use and short-lived,
    /// so this runs immediately after minting, and the credential travels
    /// only inside the socket thread's subprotocol offer.
    fn attach_live_relay(&mut self, ticket: &LiveSessionTicketProtocol) {
        self.stop_live_relay();
        let Some((url, origin)) =
            live_relay::relay_endpoint(&self.configuration.api_origin, &ticket.websocket_endpoint)
        else {
            // Unreachable with a validated ticket and release-pinned origin.
            log::warn!("live relay endpoint could not be derived from the release origin");
            return;
        };
        self.relay_generation += 1;
        let generation = self.relay_generation;
        let (stop, stop_receiver) = tokio::sync::oneshot::channel();
        let (outbound, outbound_receiver) = tokio::sync::mpsc::unbounded_channel();
        let (inbound_sender, inbound) = std::sync::mpsc::channel();
        live_relay::spawn(
            live_relay::LiveRelayConnection {
                url,
                origin,
                protocols: live_relay::subprotocol_offer(&ticket.ticket_protocol),
                generation,
            },
            outbound_receiver,
            inbound_sender,
            stop_receiver,
            self.listener_commands.clone(),
            self.repaint.clone(),
        );
        let _ = self.relay_ports.send(LiveRelayPort {
            generation,
            outbound,
            inbound,
        });
        self.relay = Some(RelayRuntime {
            generation,
            attached: false,
            stop,
        });
        if let Some(repaint) = &self.repaint {
            // Wake the UI so it picks the fresh port up promptly.
            repaint.request_repaint();
        }
    }

    /// Tear down any live socket. The socket thread answers with a `Local`
    /// closure that the generation bookkeeping then ignores.
    fn stop_live_relay(&mut self) {
        if let Some(relay) = self.relay.take() {
            let _ = relay.stop.send(());
        }
        self.relay_retry_at = None;
    }

    /// Attach the relay when the session is admitted and no socket is live.
    /// Reconnections re-mint a single-use ticket by re-joining with the code
    /// this runtime already holds — the host's own code or the one the guest
    /// presented — and failures retry on the roster-poll cadence.
    fn ensure_live_relay(&mut self) {
        if self.relay.is_some() || self.relay_blocked {
            return;
        }
        if !matches!(
            self.snapshot.live_session,
            Some(LiveSessionState::Hosting(_) | LiveSessionState::Participating(_))
        ) {
            return;
        }
        if let Some(retry_at) = self.relay_retry_at
            && std::time::Instant::now() < retry_at
        {
            self.arm_live_relay_retry();
            return;
        }
        let Some(runtime) = self.live_session.as_ref() else {
            return;
        };
        let Some(code) = runtime
            .join_code
            .clone()
            .or_else(|| runtime.presented_code.clone())
        else {
            return;
        };
        let client_instance_id = runtime.client_instance_id;
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            return;
        };
        // Rate-limit before attempting: every attempt is a join round-trip.
        self.relay_retry_at = Some(std::time::Instant::now() + LIVE_SESSION_POLL_PERIOD);
        let outcome = self.runtime.block_on(join_live_session(
            &self.cloud,
            &access,
            &code,
            client_instance_id,
        ));
        match outcome {
            Ok(joined) => {
                if let Some(ticket) = joined.ticket.as_ref() {
                    self.attach_live_relay(ticket);
                }
            }
            // A session the server no longer admits surfaces through the
            // roster poll; transient failures retry on the same cadence.
            Err(_) => self.arm_live_relay_retry(),
        }
    }

    /// A socket thread reported its end; only the current socket's report
    /// changes anything.
    fn on_live_relay_closed(&mut self, generation: u64, closure: LiveRelayClosure) {
        if self
            .relay
            .as_ref()
            .is_none_or(|relay| relay.generation != generation)
        {
            return;
        }
        self.relay = None;
        match closure {
            LiveRelayClosure::Local => {}
            LiveRelayClosure::SessionOver { message } => {
                self.relay_retry_at = None;
                self.relay_blocked = false;
                self.live_session = None;
                self.snapshot.live_session = Some(LiveSessionState::Failed { message });
                self.publish();
            }
            LiveRelayClosure::Rejected => {
                self.relay_blocked = true;
                self.set_relay_connected(false);
                self.set_live_notice("The live connection failed.".to_owned());
            }
            LiveRelayClosure::Interrupted => {
                self.relay_retry_at = Some(std::time::Instant::now() + RELAY_RESUME_DELAY);
                self.set_relay_connected(false);
                self.arm_live_relay_retry();
            }
        }
    }

    /// Reflect the socket's health on the rendered summary.
    fn set_relay_connected(&mut self, connected: bool) {
        if let Some(
            LiveSessionState::Hosting(summary)
            | LiveSessionState::AwaitingApproval(summary)
            | LiveSessionState::Participating(summary),
        ) = self.snapshot.live_session.as_mut()
            && summary.relay_connected != connected
        {
            summary.relay_connected = connected;
            self.publish();
        }
    }

    /// Surface a failed host action on the current summary without losing
    /// the session view.
    fn set_live_notice(&mut self, message: String) {
        if let Some(
            LiveSessionState::Hosting(summary)
            | LiveSessionState::AwaitingApproval(summary)
            | LiveSessionState::Participating(summary),
        ) = self.snapshot.live_session.as_mut()
        {
            summary.notice = Some(message);
            self.publish();
        }
    }

    fn sign_out(&mut self) {
        if let Some(pending) = self.pending_sign_in.take() {
            pending.cancel.store(true, Ordering::Relaxed);
        }
        // A hosted session dies with the account session that owns it.
        self.end_live_session();
        // Best-effort server-side cleanup before wiping local state.
        if let Some(lease_id) = self
            .snapshot
            .native_license
            .as_ref()
            .and_then(|license| license.lease_id.parse().ok())
            && let Some(access) = self.access.as_ref()
            && let Ok(token) = BearerToken::new(&access.token)
        {
            let _ = self
                .runtime
                .block_on(self.cloud.revoke_license_lease(&token, lease_id));
        }
        if let (Some(endpoints), Some(refresh_token)) = (
            self.endpoints.clone(),
            self.record
                .as_ref()
                .and_then(|record| record.refresh_token.clone()),
        ) {
            let _ = self.runtime.block_on(oidc::revoke_refresh_token(
                &self.http,
                &endpoints,
                &self.configuration.oidc_client_id,
                &refresh_token,
            ));
        }
        if let Some(record) = self.record.as_mut() {
            record.refresh_token = None;
            record.lease_token = None;
            record.lease_request_id = None;
            record.principal_email = None;
            record.principal_display_name = None;
            if let Err(error) = store::save(record) {
                log::warn!("cloud session record save failed: {error}");
            }
        }
        self.set_signed_out(None);
    }

    fn revoke_lease(&mut self, lease_id: &str) {
        let Some(access) = self.access.as_ref() else {
            return;
        };
        let Ok(parsed) = lease_id.parse() else {
            return;
        };
        let Ok(token) = BearerToken::new(&access.token) else {
            return;
        };
        let revoked = self
            .runtime
            .block_on(self.cloud.revoke_license_lease(&token, parsed));
        if revoked.is_ok() {
            let revoked_own_lease = self
                .snapshot
                .native_license
                .as_ref()
                .is_some_and(|license| license.lease_id == lease_id);
            if revoked_own_lease && let Some(record) = self.record.as_mut() {
                record.lease_token = None;
                record.lease_request_id = None;
                let _ = store::save(record);
            }
        }
        self.bootstrap();
    }

    // -- bootstrap ---------------------------------------------------------

    /// The client-integration startup sequence, mapped into the snapshot.
    fn bootstrap(&mut self) {
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            return;
        };
        self.snapshot.phase = CloudSessionPhase::Bootstrapping;
        self.publish();

        let outcome = self.runtime.block_on(bootstrap_reads(&self.cloud, &access));
        let reads = match outcome {
            Ok(reads) => reads,
            Err(BootstrapError::SessionRejected) => {
                if let Some(record) = self.record.as_mut() {
                    record.refresh_token = None;
                    let _ = store::save(record);
                }
                self.set_signed_out(Some(
                    "The cloud service no longer accepts this session; sign in again.".to_owned(),
                ));
                return;
            }
            Err(BootstrapError::Unreachable) => {
                self.schedule_offline_retry();
                return;
            }
            Err(BootstrapError::Contract) => {
                self.set_signed_out(Some(
                    "The cloud service response was invalid; try again later.".to_owned(),
                ));
                return;
            }
        };

        // Identity cache lets the console name the account while offline.
        if let Some(record) = self.record.as_mut() {
            record.principal_email = reads.principal.email.clone();
            record.principal_display_name = reads.principal.display_name.clone();
            let _ = store::save(record);
        }

        let native_license = self.ensure_native_lease(&access);
        let device_leases = self
            .runtime
            .block_on(read_leases(&self.cloud, &access))
            .unwrap_or_default()
            .into_iter()
            .map(|lease| lease_summary(&lease, native_license.as_ref()))
            .collect();

        self.principal_id = Some(reads.principal_id);
        self.snapshot = CloudSessionSnapshot {
            phase: CloudSessionPhase::Active,
            principal: Some(reads.principal),
            entitlements: reads.entitlements,
            workspaces: reads.workspaces,
            native_license,
            device_leases,
            publish: self.snapshot.publish.clone(),
            publications: std::mem::take(&mut self.snapshot.publications),
            live_session: self.snapshot.live_session.clone(),
            verified_at: now_rfc3339(),
            authorization_url: None,
        };
        self.next_deadline = self.access.as_ref().map(|access| access.refresh_at);
        self.publish();
    }

    /// Verify the stored lease or issue a fresh one. Soft-fails to `None`:
    /// server-side entitlement checks still govern every online operation,
    /// and the console reports the missing offline license explicitly.
    fn ensure_native_lease(&mut self, access: &str) -> Option<NativeLicenseSummary> {
        let jwks = match self.runtime.block_on(self.cloud.get_license_jwks()) {
            Ok(response) => response.into_body(),
            Err(error) => {
                log::warn!("license JWKS unavailable: {error:?}");
                return self
                    .verify_stored_lease()
                    .map(|l| native_license_summary(&l));
            }
        };
        let verifier = NativeLicenseVerifier::new(
            &jwks,
            &self.configuration.license_issuer,
            &self.configuration.license_audience,
            &self.configuration.license_product,
            LEASE_CLOCK_SKEW,
        )
        .ok()?;

        let record = self.record.get_or_insert_with(|| {
            store::CloudSessionRecord::new_with_binding_secret()
                .expect("entropy for the device binding secret")
        });
        let fingerprint = record.device_fingerprint_sha256()?;

        // A stored lease that verifies against the *current* keys and is not
        // near expiry needs no round trip.
        if let Some(token) = record.lease_token.clone()
            && let Ok(license) =
                verifier.verify_token(&token, &fingerprint, std::time::Duration::ZERO)
            && license.expires_at_unix_seconds() - now_unix_seconds() > LEASE_RENEWAL_WINDOW_SECONDS
        {
            return Some(native_license_summary(&license));
        }

        // Persist the request UUID before issuing so a lost response retries
        // with the same identity.
        let request_id = record
            .lease_request_id
            .as_deref()
            .and_then(|value| value.parse().ok())
            .unwrap_or_else(uuid::Uuid::new_v4);
        record.lease_request_id = Some(request_id.to_string());
        if let Err(error) = store::save(record) {
            log::warn!("cloud session record save failed: {error}");
        }
        let request = IssueLicenseLeaseRequest {
            request_id,
            device_fingerprint_sha256: fingerprint,
            workspace_id: None,
        };
        let issued = match self.runtime.block_on(async {
            let token = BearerToken::new(access).map_err(|_| ())?;
            self.cloud
                .issue_license_lease(&token, &request)
                .await
                .map_err(|_| ())
        }) {
            Ok(response) => response.into_body(),
            Err(()) => {
                log::warn!("license lease issuance failed");
                return None;
            }
        };
        let license = match verifier.verify_issued(&issued, &request, LEASE_MINIMUM_VALIDITY) {
            Ok(license) => license,
            Err(error) => {
                log::warn!("license lease verification failed: {error}");
                return None;
            }
        };
        let record = self.record.as_mut().expect("record created above");
        record.lease_token = Some(issued.lease_token.clone());
        record.lease_request_id = None;
        record.license_jwks = serde_json::to_string(&jwks).ok();
        if let Err(error) = store::save(record) {
            log::warn!("cloud session record save failed: {error}");
        }
        Some(native_license_summary(&license))
    }
}

// -- bootstrap reads (pure client calls) ------------------------------------

struct BootstrapReads {
    principal_id: uuid::Uuid,
    principal: PrincipalSummary,
    entitlements: Vec<EntitlementSummary>,
    workspaces: Vec<WorkspaceSummary>,
}

enum BootstrapError {
    Unreachable,
    SessionRejected,
    Contract,
}

fn classify(error: &rspice_cloud_client::CloudError) -> BootstrapError {
    match error.status() {
        Some(401) | Some(403) => BootstrapError::SessionRejected,
        Some(_) => BootstrapError::Contract,
        None => match error {
            rspice_cloud_client::CloudError::Transport { .. } => BootstrapError::Unreachable,
            _ => BootstrapError::Contract,
        },
    }
}

async fn bootstrap_reads(
    cloud: &CloudClient,
    access: &str,
) -> Result<BootstrapReads, BootstrapError> {
    let token = BearerToken::new(access).map_err(|_| BootstrapError::Contract)?;

    let principal = cloud
        .get_current_principal(&token)
        .await
        .map_err(|error| classify(&error))?
        .into_body();

    let mut entitlements: Vec<Entitlement> = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let request = match cursor.as_deref() {
            Some(value) => PageRequest::after(PAGE_LIMIT, value),
            None => PageRequest::first(PAGE_LIMIT),
        }
        .map_err(|_| BootstrapError::Contract)?;
        let page = cloud
            .list_entitlements(&token, request)
            .await
            .map_err(|error| classify(&error))?
            .into_body();
        entitlements.extend(page.items);
        match page.next_cursor {
            Some(next) => cursor = Some(next),
            None => break,
        }
    }

    let mut workspaces = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let request = match cursor.as_deref() {
            Some(value) => PageRequest::after(PAGE_LIMIT, value),
            None => PageRequest::first(PAGE_LIMIT),
        }
        .map_err(|_| BootstrapError::Contract)?;
        let page = cloud
            .list_workspaces(&token, request)
            .await
            .map_err(|error| classify(&error))?
            .into_body();
        workspaces.extend(page.items.into_iter().map(|workspace| WorkspaceSummary {
            id: workspace.id.to_string(),
            name: workspace.name,
        }));
        match page.next_cursor {
            Some(next) => cursor = Some(next),
            None => break,
        }
    }

    Ok(BootstrapReads {
        principal_id: principal.id,
        principal: PrincipalSummary {
            email: principal.email,
            display_name: principal.display_name,
        },
        entitlements: entitlements.iter().map(entitlement_summary).collect(),
        workspaces,
    })
}

async fn read_publications(
    cloud: &CloudClient,
    access: &str,
    circuit_id: uuid::Uuid,
) -> Option<Vec<PublicationSummary>> {
    let token = BearerToken::new(access).ok()?;
    let mut publications = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let request = match cursor.as_deref() {
            Some(value) => PageRequest::after(PAGE_LIMIT, value).ok()?,
            None => PageRequest::first(PAGE_LIMIT).ok()?,
        };
        let page = cloud
            .list_circuit_publications(&token, circuit_id, request)
            .await
            .ok()?
            .into_body();
        publications.extend(
            page.items
                .into_iter()
                .map(|publication| PublicationSummary {
                    id: publication.id.to_string(),
                    title: publication.title,
                    url_path: publication.url_path,
                    page_status: publish::page_status_str(publication.page_status).to_owned(),
                    published_at: publication.published_at,
                    unpublished_at: publication.unpublished_at,
                }),
        );
        match page.next_cursor {
            Some(next) => cursor = Some(next),
            None => break,
        }
    }
    Some(publications)
}

async fn read_leases(cloud: &CloudClient, access: &str) -> Option<Vec<LicenseLease>> {
    let token = BearerToken::new(access).ok()?;
    let mut leases: Vec<LicenseLease> = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let request = match cursor.as_deref() {
            Some(value) => PageRequest::after(PAGE_LIMIT, value).ok()?,
            None => PageRequest::first(PAGE_LIMIT).ok()?,
        };
        let list = cloud
            .list_license_leases(&token, request)
            .await
            .ok()?
            .into_body();
        leases.extend(list.items);
        match list.next_cursor {
            Some(next) => cursor = Some(next),
            None => break,
        }
    }
    Some(leases)
}

// -- live sessions -------------------------------------------------------------

/// Presentation fallback when the service cannot be reached at all.
const LIVE_SESSION_UNREACHABLE: &str = "The live-session service could not be reached.";

enum LiveParticipantAction {
    Approve,
    SetEditor(bool),
    Remove,
}

enum LiveSessionReadFailure {
    /// The server refused the read: ended, aged out, or removed —
    /// indistinguishable by design.
    Gone,
    /// Transport or token trouble; the next poll retries.
    Transient,
}

async fn create_live_session(
    cloud: &CloudClient,
    access: &str,
    create_key: &str,
    policy: LiveSessionPolicy,
    circuit_id: Option<uuid::Uuid>,
    client_instance_id: uuid::Uuid,
) -> Result<rspice_cloud_client::contract::CreatedLiveSession, String> {
    let token = BearerToken::new(access).map_err(|_| LIVE_SESSION_UNREACHABLE.to_owned())?;
    let key = IdempotencyKey::new(create_key).map_err(|_| LIVE_SESSION_UNREACHABLE.to_owned())?;
    cloud
        .create_live_session_idempotent(&token, &key, policy, circuit_id, client_instance_id)
        .await
        .map(rspice_cloud_client::CloudResponse::into_body)
        .map_err(|error| live_session_error_message(&error))
}

async fn join_live_session(
    cloud: &CloudClient,
    access: &str,
    code: &str,
    client_instance_id: uuid::Uuid,
) -> Result<rspice_cloud_client::contract::JoinedLiveSession, String> {
    let token = BearerToken::new(access).map_err(|_| LIVE_SESSION_UNREACHABLE.to_owned())?;
    cloud
        .join_live_session(&token, code, client_instance_id)
        .await
        .map(rspice_cloud_client::CloudResponse::into_body)
        .map_err(|error| live_session_error_message(&error))
}

async fn read_live_session(
    cloud: &CloudClient,
    access: &str,
    session_id: uuid::Uuid,
) -> Result<LiveSession, LiveSessionReadFailure> {
    let token = BearerToken::new(access).map_err(|_| LiveSessionReadFailure::Transient)?;
    match cloud.get_live_session(&token, session_id).await {
        Ok(response) => Ok(response.into_body()),
        Err(CloudError::Problem { .. }) => Err(LiveSessionReadFailure::Gone),
        Err(_) => Err(LiveSessionReadFailure::Transient),
    }
}

fn contract_policy(policy: LiveSessionPolicySummary) -> LiveSessionPolicy {
    LiveSessionPolicy {
        default_capability: if policy.guests_edit {
            LiveSessionCapability::Edit
        } else {
            LiveSessionCapability::View
        },
        approve_joins: policy.approve_joins,
        allow_save_copy: policy.allow_save_copy,
    }
}

fn live_session_summary(
    session: &LiveSession,
    self_principal: Option<uuid::Uuid>,
    hosting: bool,
    join_code: Option<String>,
) -> LiveSessionSummary {
    let participants: Vec<LiveParticipantSummary> = session
        .participants
        .iter()
        .map(|participant| {
            let is_host = participant.principal_id == session.host_principal_id;
            LiveParticipantSummary {
                principal_id: participant.principal_id.to_string(),
                display_name: participant.display_name.clone(),
                is_host,
                is_self: self_principal.is_some_and(|own| own == participant.principal_id),
                editor: is_host || participant.capability == LiveSessionCapability::Edit,
                pending: participant.admission == LiveSessionAdmission::Pending,
                joined_at: participant.joined_at.clone(),
            }
        })
        .collect();
    let editor = hosting
        || participants
            .iter()
            .any(|participant| participant.is_self && participant.editor);
    LiveSessionSummary {
        session_id: session.id.to_string(),
        join_code,
        hosting,
        editor,
        policy: LiveSessionPolicySummary {
            guests_edit: session.policy.default_capability == LiveSessionCapability::Edit,
            approve_joins: session.policy.approve_joins,
            allow_save_copy: session.policy.allow_save_copy,
        },
        participants,
        started_at: session.created_at.clone(),
        relay_connected: false,
        notice: None,
    }
}

/// Presentation-safe failure text: the service's own problem title when the
/// server answered deliberately, a fixed line otherwise.
fn live_session_error_message(error: &CloudError) -> String {
    match error {
        CloudError::Problem { details, .. } => details.title.clone(),
        _ => LIVE_SESSION_UNREACHABLE.to_owned(),
    }
}

// -- mapping -----------------------------------------------------------------

fn entitlement_summary(entitlement: &Entitlement) -> EntitlementSummary {
    EntitlementSummary {
        status: entitlement.status.as_str().to_owned(),
        granted_features: granted_features(&entitlement.features),
        valid_until: entitlement.valid_until.clone(),
    }
}

/// Explicit `true` grants only; everything else is denial.
fn granted_features(features: &serde_json::Value) -> Vec<String> {
    let Some(map) = features.as_object() else {
        return Vec::new();
    };
    let mut granted: Vec<String> = map
        .iter()
        .filter(|(_, value)| value.as_bool() == Some(true))
        .map(|(name, _)| name.clone())
        .collect();
    granted.sort();
    granted
}

fn native_license_summary(license: &VerifiedNativeLicense) -> NativeLicenseSummary {
    NativeLicenseSummary {
        plan: license.plan().to_owned(),
        product: license.product().to_owned(),
        granted_features: granted_features(license.features()),
        expires_at_unix_seconds: license.expires_at_unix_seconds(),
        lease_id: license.lease_id().to_string(),
    }
}

fn lease_summary(lease: &LicenseLease, own: Option<&NativeLicenseSummary>) -> LeaseSummary {
    let id = lease.id.to_string();
    LeaseSummary {
        this_device: own.is_some_and(|license| license.lease_id == id),
        id,
        plan: lease.plan.clone(),
        issued_at: lease.issued_at.clone(),
        expires_at: lease.expires_at.clone(),
        revoked_at: lease.revoked_at.clone(),
    }
}

fn now_rfc3339() -> Option<String> {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .ok()
}

fn now_unix_seconds() -> i64 {
    time::OffsetDateTime::now_utc().unix_timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn granted_features_admits_only_explicit_true() {
        let features = serde_json::json!({
            "cloud_publishing": true,
            "live_collaboration": false,
            "max_devices": 3,
            "native_license": "true",
        });
        assert_eq!(granted_features(&features), vec!["cloud_publishing"]);
        assert!(granted_features(&serde_json::json!([])).is_empty());
    }

    #[test]
    fn live_session_summaries_mark_host_self_pending_and_editors() {
        use rspice_cloud_client::contract::LiveSessionParticipant;
        let host = uuid::Uuid::from_u128(1);
        let guest = uuid::Uuid::from_u128(2);
        let session = LiveSession {
            id: uuid::Uuid::from_u128(9),
            host_principal_id: host,
            circuit_id: None,
            policy: LiveSessionPolicy {
                default_capability: LiveSessionCapability::View,
                approve_joins: true,
                allow_save_copy: false,
            },
            created_at: "2026-08-08T00:00:00Z".to_owned(),
            participants: vec![
                LiveSessionParticipant {
                    principal_id: host,
                    display_name: "Host".to_owned(),
                    capability: LiveSessionCapability::Edit,
                    admission: LiveSessionAdmission::Admitted,
                    joined_at: "2026-08-08T00:00:00Z".to_owned(),
                },
                LiveSessionParticipant {
                    principal_id: guest,
                    display_name: "Guest".to_owned(),
                    capability: LiveSessionCapability::View,
                    admission: LiveSessionAdmission::Pending,
                    joined_at: "2026-08-08T00:01:00Z".to_owned(),
                },
            ],
        };

        let summary = live_session_summary(&session, Some(guest), false, None);
        assert!(!summary.hosting);
        assert!(!summary.editor, "a viewer guest is not an editor");
        assert!(!summary.policy.guests_edit);
        assert!(summary.policy.approve_joins);
        assert!(summary.join_code.is_none());
        let host_row = &summary.participants[0];
        assert!(host_row.is_host && host_row.editor && !host_row.is_self);
        let guest_row = &summary.participants[1];
        assert!(guest_row.is_self && guest_row.pending && !guest_row.editor);

        let hosted =
            live_session_summary(&session, Some(host), true, Some("ABCDE-FGHJK".to_owned()));
        assert!(hosted.hosting && hosted.editor);
        assert_eq!(hosted.join_code.as_deref(), Some("ABCDE-FGHJK"));
    }

    #[test]
    fn live_session_policy_maps_editors_to_the_contract_capability() {
        let editors = contract_policy(LiveSessionPolicySummary {
            guests_edit: true,
            approve_joins: false,
            allow_save_copy: true,
        });
        assert_eq!(editors.default_capability, LiveSessionCapability::Edit);
        assert!(!editors.approve_joins);
        assert!(editors.allow_save_copy);
        let viewers = contract_policy(LiveSessionPolicySummary {
            guests_edit: false,
            approve_joins: true,
            allow_save_copy: false,
        });
        assert_eq!(viewers.default_capability, LiveSessionCapability::View);
    }

    #[test]
    fn live_session_errors_surface_only_deliberate_problem_titles() {
        let details: rspice_cloud_client::contract::ProblemDetails =
            serde_json::from_value(serde_json::json!({
                "type": "about:blank",
                "title": "Live collaboration requires an active entitlement.",
                "status": 403,
                "detail": "internal wording that stays out of the summary line",
                "instance": "about:blank",
            }))
            .expect("problem details");
        let problem = CloudError::Problem {
            details: Box::new(details),
            metadata: rspice_cloud_client::ResponseMetadata::default(),
        };
        assert_eq!(
            live_session_error_message(&problem),
            "Live collaboration requires an active entitlement."
        );
    }

    #[test]
    fn lease_rows_mark_this_device_by_lease_id() {
        let lease = LicenseLease {
            id: uuid::Uuid::nil(),
            entitlement_id: uuid::Uuid::nil(),
            workspace_id: None,
            product: "rspice".to_owned(),
            plan: "professional".to_owned(),
            issued_at: "2026-08-01T00:00:00Z".to_owned(),
            expires_at: "2026-08-08T00:00:00Z".to_owned(),
            revoked_at: None,
            revocation_reason: None,
            signing_key_id: "test-key".to_owned(),
        };
        let own = NativeLicenseSummary {
            plan: "professional".to_owned(),
            product: "rspice".to_owned(),
            granted_features: Vec::new(),
            expires_at_unix_seconds: 0,
            lease_id: uuid::Uuid::nil().to_string(),
        };
        assert!(lease_summary(&lease, Some(&own)).this_device);
        assert!(!lease_summary(&lease, None).this_device);
    }
}
