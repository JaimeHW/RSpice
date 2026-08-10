//! Single-threaded browser Cloud account and live-session executor.
//!
//! Commands are serialized through one bounded queue and executed with
//! `spawn_local`; no credential ever crosses into the UI snapshot. Access and
//! refresh tokens remain in WebAssembly memory. Only the short-lived PKCE
//! verifier/state pair survives the full-page authorization redirect, in the
//! current tab's `sessionStorage`, and it is deleted before code exchange.

use std::time::{Duration, Instant};

use futures_util::{FutureExt as _, pin_mut, select};
use rspice_cloud_client::contract::{
    Entitlement, LiveSession, LiveSessionAdmission, LiveSessionCapability, LiveSessionPolicy,
    LiveSessionTicketProtocol, UpdateLiveSessionParticipantRequest,
};
use rspice_cloud_client::{
    BearerToken, ClientConfig, CloudClient, CloudError, IdempotencyKey, PageRequest,
};

use super::config::CloudAccountConfig;
use super::{
    CloudAccountCommand, CloudAccountEvent, CloudAccountEventReceiver, CloudAccountEventSender,
    CloudSessionPhase, CloudSessionSnapshot, EntitlementSummary, LIVE_RELAY_INBOUND_QUEUE_CAPACITY,
    LIVE_RELAY_OUTBOUND_QUEUE_CAPACITY, LiveParticipantSummary, LiveRelayClosure, LiveRelayPort,
    LiveRelayPortReceiver, LiveRelayPortSender, LiveSessionPolicySummary, LiveSessionState,
    LiveSessionSummary, PrincipalSummary, PublishState, WorkspaceSummary, browser_live_relay,
    client_retry_delay, cloud_account_event_mailbox, live_relay_port_mailbox, oidc,
};

const COMMAND_CAPACITY: usize = 64;
/// One current socket plus its locally closing predecessor can each emit at
/// most attach + close. Keep lifecycle authority isolated from UI commands.
const RELAY_COMMAND_CAPACITY: usize = 8;
const PAGE_LIMIT: usize = 100;
const TICK_PERIOD: Duration = Duration::from_secs(1);
const LIVE_SESSION_POLL_PERIOD: Duration = Duration::from_secs(5);
const PKCE_RECORD_MAX_AGE_SECONDS: i64 = 10 * 60;
const MAX_PKCE_RECORD_BYTES: usize = 8 * 1024;
const PKCE_STORAGE_KEY: &str = "rspice.cloud.oidc.pending.v1";
const LIVE_SESSION_UNREACHABLE: &str = "The live-session service could not be reached.";

pub(super) fn spawn(
    configuration: CloudAccountConfig,
    repaint: Option<egui::Context>,
) -> (
    tokio::sync::mpsc::Sender<CloudAccountCommand>,
    CloudAccountEventReceiver,
    LiveRelayPortReceiver,
) {
    let (commands, command_receiver) = tokio::sync::mpsc::channel(COMMAND_CAPACITY);
    let (relay_commands, relay_command_receiver) =
        tokio::sync::mpsc::channel(RELAY_COMMAND_CAPACITY);
    let (events, event_receiver) = cloud_account_event_mailbox();
    let (ports, port_receiver) = live_relay_port_mailbox();
    wasm_bindgen_futures::spawn_local(async move {
        let client_config = build_client_config(&configuration);
        let (Ok(client_config), Ok(http)) = (client_config, oidc::identity_http_client()) else {
            let _ = events.send(CloudAccountEvent::Snapshot(Box::new(
                CloudSessionSnapshot {
                    phase: CloudSessionPhase::SignedOut {
                        last_error: Some("The browser cloud configuration is invalid.".to_owned()),
                    },
                    ..CloudSessionSnapshot::default()
                },
            )));
            return;
        };
        let Ok(cloud) = CloudClient::new(client_config) else {
            return;
        };
        let mut executor = BrowserExecutor {
            configuration,
            http,
            cloud,
            endpoints: None,
            access: None,
            refresh_token: None,
            auth_retry_seed: uuid::Uuid::now_v7(),
            auth_retry_attempt: 0,
            principal_id: None,
            live_session: None,
            relay: None,
            relay_generation: 0,
            relay_retry_at: None,
            relay_retry_attempt: 0,
            relay_blocked: false,
            last_roster_poll: None,
            snapshot: CloudSessionSnapshot::default(),
            events,
            relay_ports: ports,
            relay_commands,
            repaint,
        };
        executor.startup().await;
        executor.run(command_receiver, relay_command_receiver).await;
    });
    (commands, event_receiver, port_receiver)
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
    build.map_err(|_| "The browser cloud endpoints are invalid.".to_owned())
}

struct AccessToken {
    token: String,
    refresh_at: Instant,
}

struct LiveSessionRuntime {
    session_id: uuid::Uuid,
    hosting: bool,
    create_key: Option<String>,
    client_instance_id: uuid::Uuid,
    join_code: Option<String>,
}

struct BrowserExecutor {
    configuration: CloudAccountConfig,
    http: oidc::IdentityHttpClient,
    cloud: CloudClient,
    endpoints: Option<oidc::ProviderEndpoints>,
    access: Option<AccessToken>,
    refresh_token: Option<String>,
    auth_retry_seed: uuid::Uuid,
    auth_retry_attempt: u32,
    principal_id: Option<uuid::Uuid>,
    live_session: Option<LiveSessionRuntime>,
    relay: Option<browser_live_relay::BrowserRelayHandle>,
    relay_generation: u64,
    relay_retry_at: Option<Instant>,
    relay_retry_attempt: u32,
    relay_blocked: bool,
    last_roster_poll: Option<Instant>,
    snapshot: CloudSessionSnapshot,
    events: CloudAccountEventSender,
    relay_ports: LiveRelayPortSender,
    relay_commands: tokio::sync::mpsc::Sender<CloudAccountCommand>,
    repaint: Option<egui::Context>,
}

impl BrowserExecutor {
    async fn startup(&mut self) {
        match take_browser_callback(&self.configuration) {
            Ok(Some(callback)) => self.complete_callback(callback).await,
            Ok(None) => self.publish(),
            Err(message) => self.set_signed_out(Some(message)),
        }
    }

    async fn run(
        &mut self,
        mut commands: tokio::sync::mpsc::Receiver<CloudAccountCommand>,
        mut relay_commands: tokio::sync::mpsc::Receiver<CloudAccountCommand>,
    ) {
        loop {
            let command = commands.recv().fuse();
            let relay_command = relay_commands.recv().fuse();
            let tick = gloo_timers::future::TimeoutFuture::new(
                u32::try_from(TICK_PERIOD.as_millis()).expect("tick fits u32"),
            )
            .fuse();
            pin_mut!(command, relay_command, tick);
            select! {
                incoming = command => match incoming {
                    Some(command) => self.handle(command).await,
                    None => return,
                },
                incoming = relay_command => match incoming {
                    Some(command) => self.handle(command).await,
                    None => return,
                },
                () = tick => self.tick().await,
            }
        }
    }

    fn publish(&self) {
        let _ = self
            .events
            .send(CloudAccountEvent::Snapshot(Box::new(self.snapshot.clone())));
        if let Some(repaint) = &self.repaint {
            repaint.request_repaint();
        }
    }

    fn open_browser(&self, url: String) {
        let _ = self.events.send(CloudAccountEvent::OpenBrowser(url));
        if let Some(repaint) = &self.repaint {
            repaint.request_repaint();
        }
    }

    fn set_signed_out(&mut self, error: Option<String>) {
        self.stop_live_relay();
        self.access = None;
        self.refresh_token = None;
        self.auth_retry_attempt = 0;
        self.principal_id = None;
        self.live_session = None;
        self.relay_blocked = false;
        self.snapshot = CloudSessionSnapshot {
            phase: CloudSessionPhase::SignedOut { last_error: error },
            ..CloudSessionSnapshot::default()
        };
        self.publish();
    }

    async fn handle(&mut self, command: CloudAccountCommand) {
        match command {
            CloudAccountCommand::SignIn => self.begin_sign_in().await,
            CloudAccountCommand::CancelSignIn => {
                clear_pkce_record();
                self.set_signed_out(None);
            }
            CloudAccountCommand::SignOut => self.sign_out().await,
            CloudAccountCommand::Refresh => self.refresh_online().await,
            CloudAccountCommand::RevokeLease { .. } => {}
            CloudAccountCommand::PublishSnapshot { .. } => {
                self.snapshot.publish = Some(PublishState::Failed {
                    message: "Browser publishing is not enabled in this release.".to_owned(),
                });
                self.publish();
            }
            CloudAccountCommand::RefreshPublications { .. }
            | CloudAccountCommand::Unpublish { .. } => {}
            CloudAccountCommand::StartLiveSession { policy, circuit_id } => {
                self.start_live_session(policy, circuit_id.as_deref()).await;
            }
            CloudAccountCommand::RegenerateLiveSessionCode => {
                self.regenerate_live_session_code().await;
            }
            CloudAccountCommand::JoinLiveSession { code } => self.join_live_session(&code).await,
            CloudAccountCommand::RefreshLiveSession => {
                self.relay_blocked = false;
                self.relay_retry_at = None;
                self.relay_retry_attempt = 0;
                self.refresh_live_session().await;
            }
            CloudAccountCommand::ApplyLiveSessionPolicy { policy } => {
                self.apply_live_session_policy(policy).await;
            }
            CloudAccountCommand::ApproveLiveSessionParticipant { principal_id } => {
                self.manage_participant(&principal_id, ParticipantAction::Approve)
                    .await;
            }
            CloudAccountCommand::SetLiveSessionParticipantEditor {
                principal_id,
                editor,
            } => {
                self.manage_participant(&principal_id, ParticipantAction::SetEditor(editor))
                    .await;
            }
            CloudAccountCommand::RemoveLiveSessionParticipant { principal_id } => {
                self.manage_participant(&principal_id, ParticipantAction::Remove)
                    .await;
            }
            CloudAccountCommand::EndLiveSession => self.end_live_session().await,
            CloudAccountCommand::LeaveLiveSession => self.leave_live_session(),
            CloudAccountCommand::LiveRelayAttached { generation } => {
                if let Some(relay) = self
                    .relay
                    .as_mut()
                    .filter(|relay| relay.generation == generation)
                {
                    relay.attached = true;
                    self.relay_retry_at = None;
                    self.relay_retry_attempt = 0;
                    self.set_relay_connected(true);
                }
            }
            CloudAccountCommand::LiveRelayClosed {
                generation,
                closure,
            } => self.on_live_relay_closed(generation, closure),
        }
    }

    async fn tick(&mut self) {
        let refresh_due = self
            .access
            .as_ref()
            .is_some_and(|access| Instant::now() >= access.refresh_at);
        if refresh_due {
            self.refresh_grant().await;
        }
        let roster_due = self.live_session.is_some()
            && self.last_roster_poll.is_none_or(|last| {
                Instant::now().saturating_duration_since(last) >= LIVE_SESSION_POLL_PERIOD
            });
        if roster_due {
            self.refresh_live_session().await;
        }
        self.ensure_live_relay().await;
    }

    async fn provider_endpoints(&mut self) -> Result<oidc::ProviderEndpoints, oidc::IdentityError> {
        if let Some(endpoints) = &self.endpoints {
            return Ok(endpoints.clone());
        }
        let endpoints = oidc::discover(
            &self.http,
            &self.configuration.oidc_issuer,
            self.configuration.development,
        )
        .await?;
        self.endpoints = Some(endpoints.clone());
        Ok(endpoints)
    }

    async fn begin_sign_in(&mut self) {
        if !matches!(self.snapshot.phase, CloudSessionPhase::SignedOut { .. }) {
            return;
        }
        let endpoints = match self.provider_endpoints().await {
            Ok(endpoints) => endpoints,
            Err(error) => {
                self.set_signed_out(Some(error.to_string()));
                return;
            }
        };
        let (pkce, state) = match (oidc::pkce_pair(), oidc::state_value()) {
            (Ok(pkce), Ok(state)) => (pkce, state),
            _ => {
                self.set_signed_out(Some("No secure entropy source for sign-in.".to_owned()));
                return;
            }
        };
        let url = match oidc::authorization_url(
            &endpoints,
            &self.configuration.oidc_client_id,
            &self.configuration.browser_redirect_uri,
            &state,
            &pkce,
        ) {
            Ok(url) => url,
            Err(error) => {
                self.set_signed_out(Some(error.to_string()));
                return;
            }
        };
        let record = PendingPkceRecord {
            state,
            verifier: pkce.verifier,
            redirect_uri: self.configuration.browser_redirect_uri.clone(),
            created_at_unix_seconds: time::OffsetDateTime::now_utc().unix_timestamp(),
        };
        if store_pkce_record(&record).is_err() {
            self.set_signed_out(Some(
                "The browser blocked temporary sign-in storage; allow session storage and retry."
                    .to_owned(),
            ));
            return;
        }
        self.snapshot.phase = CloudSessionPhase::WaitingForBrowser;
        self.snapshot.authorization_url = Some(url.clone());
        self.publish();
        self.open_browser(url);
    }

    async fn complete_callback(&mut self, callback: BrowserCallback) {
        self.snapshot.phase = CloudSessionPhase::ExchangingTokens;
        self.snapshot.authorization_url = None;
        self.publish();
        let endpoints = match self.provider_endpoints().await {
            Ok(endpoints) => endpoints,
            Err(error) => {
                self.set_signed_out(Some(error.to_string()));
                return;
            }
        };
        match oidc::exchange_code(
            &self.http,
            &endpoints,
            &self.configuration.oidc_client_id,
            &callback.redirect_uri,
            &callback.code,
            &callback.verifier,
        )
        .await
        {
            Ok(grant) => {
                self.adopt_grant(grant);
                self.bootstrap().await;
            }
            Err(error) => self.set_signed_out(Some(error.to_string())),
        }
    }

    fn adopt_grant(&mut self, grant: oidc::TokenGrant) {
        self.auth_retry_attempt = 0;
        let lifetime = Duration::from_secs(grant.expires_in);
        self.access = Some(AccessToken {
            token: grant.access_token,
            refresh_at: Instant::now() + lifetime.mul_f32(0.6),
        });
        if let Some(refresh_token) = grant.refresh_token {
            self.refresh_token = Some(refresh_token);
        }
    }

    async fn refresh_grant(&mut self) {
        let Some(refresh_token) = self.refresh_token.clone() else {
            self.set_signed_out(Some(
                "Your browser session expired; sign in again.".to_owned(),
            ));
            return;
        };
        let endpoints = match self.provider_endpoints().await {
            Ok(endpoints) => endpoints,
            Err(oidc::IdentityError::Unreachable) => {
                self.schedule_auth_retry();
                return;
            }
            Err(error) => {
                self.set_signed_out(Some(error.to_string()));
                return;
            }
        };
        match oidc::refresh_grant(
            &self.http,
            &endpoints,
            &self.configuration.oidc_client_id,
            &refresh_token,
        )
        .await
        {
            Ok(grant) => self.adopt_grant(grant),
            Err(oidc::IdentityError::Unreachable) => self.schedule_auth_retry(),
            Err(oidc::IdentityError::Rejected) => {
                self.set_signed_out(Some(
                    "Your browser session expired; sign in again.".to_owned(),
                ));
            }
            Err(error) => self.set_signed_out(Some(error.to_string())),
        }
    }

    fn schedule_auth_retry(&mut self) {
        let delay = client_retry_delay(self.auth_retry_attempt, self.auth_retry_seed);
        self.auth_retry_attempt = self.auth_retry_attempt.saturating_add(1);
        if let Some(access) = self.access.as_mut() {
            access.refresh_at = Instant::now() + delay;
        }
    }

    async fn refresh_online(&mut self) {
        if self.access.is_none() {
            self.refresh_grant().await;
        }
        if self.access.is_some() {
            self.bootstrap().await;
        }
    }

    async fn bootstrap(&mut self) {
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            return;
        };
        self.snapshot.phase = CloudSessionPhase::Bootstrapping;
        self.publish();
        let reads = match bootstrap_reads(&self.cloud, &access).await {
            Ok(reads) => reads,
            Err(BootstrapError::Rejected) => {
                self.set_signed_out(Some(
                    "The cloud service no longer accepts this session; sign in again.".to_owned(),
                ));
                return;
            }
            Err(BootstrapError::Unreachable) => {
                self.snapshot.phase = CloudSessionPhase::Active;
                self.publish();
                return;
            }
            Err(BootstrapError::Contract) => {
                self.set_signed_out(Some(
                    "The cloud service response was invalid; try again later.".to_owned(),
                ));
                return;
            }
        };
        self.principal_id = Some(reads.principal_id);
        self.snapshot = CloudSessionSnapshot {
            phase: CloudSessionPhase::Active,
            principal: Some(reads.principal),
            entitlements: reads.entitlements,
            workspaces: reads.workspaces,
            native_license: None,
            device_leases: Vec::new(),
            publish: self.snapshot.publish.clone(),
            publications: std::mem::take(&mut self.snapshot.publications),
            live_session: self.snapshot.live_session.clone(),
            verified_at: now_rfc3339(),
            authorization_url: None,
        };
        self.publish();
    }

    async fn sign_out(&mut self) {
        clear_pkce_record();
        if self
            .live_session
            .as_ref()
            .is_some_and(|runtime| runtime.hosting)
        {
            self.end_live_session().await;
        }
        if let (Some(endpoints), Some(refresh_token)) =
            (self.endpoints.clone(), self.refresh_token.clone())
        {
            let _ = oidc::revoke_refresh_token(
                &self.http,
                &endpoints,
                &self.configuration.oidc_client_id,
                &refresh_token,
            )
            .await;
        }
        self.set_signed_out(None);
    }

    async fn start_live_session(
        &mut self,
        policy: LiveSessionPolicySummary,
        circuit_id: Option<&str>,
    ) {
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            self.fail_live("Sign in before going live.");
            return;
        };
        let circuit_id = circuit_id.and_then(|value| value.parse().ok());
        let create_key = uuid::Uuid::now_v7().to_string();
        let client_instance_id = uuid::Uuid::now_v7();
        self.stop_live_relay();
        self.relay_blocked = false;
        self.snapshot.live_session = Some(LiveSessionState::Starting);
        self.publish();
        match create_live_session(
            &self.cloud,
            &access,
            &create_key,
            contract_policy(policy),
            circuit_id,
            client_instance_id,
        )
        .await
        {
            Ok(created) => {
                self.live_session = Some(LiveSessionRuntime {
                    session_id: created.session.id,
                    hosting: true,
                    create_key: Some(create_key),
                    client_instance_id,
                    join_code: Some(created.join_code.clone()),
                });
                self.snapshot.live_session = Some(LiveSessionState::Hosting(live_session_summary(
                    &created.session,
                    self.principal_id,
                    true,
                    Some(created.join_code),
                )));
                self.last_roster_poll = Some(Instant::now());
                self.publish();
                self.attach_live_relay(&created.ticket);
            }
            Err(message) => self.fail_live(&message),
        }
    }

    async fn regenerate_live_session_code(&mut self) {
        let Some(runtime) = self.live_session.as_ref() else {
            return;
        };
        if !runtime.hosting {
            return;
        }
        let (Some(create_key), Some(access)) = (
            runtime.create_key.clone(),
            self.access.as_ref().map(|access| access.token.clone()),
        ) else {
            return;
        };
        let client_instance_id = runtime.client_instance_id;
        let policy = match &self.snapshot.live_session {
            Some(LiveSessionState::Hosting(summary)) => summary.policy,
            _ => return,
        };
        match create_live_session(
            &self.cloud,
            &access,
            &create_key,
            contract_policy(policy),
            None,
            client_instance_id,
        )
        .await
        {
            Ok(created) => {
                if let Some(runtime) = self.live_session.as_mut() {
                    runtime.session_id = created.session.id;
                    runtime.join_code = Some(created.join_code.clone());
                }
                self.snapshot.live_session = Some(LiveSessionState::Hosting(live_session_summary(
                    &created.session,
                    self.principal_id,
                    true,
                    Some(created.join_code),
                )));
                self.publish();
                self.attach_live_relay(&created.ticket);
            }
            Err(message) => self.set_live_notice(message),
        }
    }

    async fn join_live_session(&mut self, code: &str) {
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            self.fail_live("Sign in before joining a live session.");
            return;
        };
        let client_instance_id = uuid::Uuid::now_v7();
        self.stop_live_relay();
        self.relay_blocked = false;
        self.snapshot.live_session = Some(LiveSessionState::Joining);
        self.publish();
        match join_live_session(&self.cloud, &access, code, client_instance_id).await {
            Ok(joined) => {
                self.live_session = Some(LiveSessionRuntime {
                    session_id: joined.session.id,
                    hosting: false,
                    create_key: None,
                    client_instance_id,
                    join_code: None,
                });
                let summary = live_session_summary(&joined.session, self.principal_id, false, None);
                self.snapshot.live_session =
                    Some(if joined.admission == LiveSessionAdmission::Pending {
                        LiveSessionState::AwaitingApproval(summary)
                    } else {
                        LiveSessionState::Participating(summary)
                    });
                self.last_roster_poll = Some(Instant::now());
                self.publish();
                if let Some(ticket) = joined.ticket.as_ref() {
                    self.attach_live_relay(ticket);
                }
            }
            Err(message) => self.fail_live(&message),
        }
    }

    async fn refresh_live_session(&mut self) {
        let Some(runtime) = self.live_session.as_ref() else {
            return;
        };
        let (session_id, Some(access)) = (
            runtime.session_id,
            self.access.as_ref().map(|access| access.token.clone()),
        ) else {
            return;
        };
        self.last_roster_poll = Some(Instant::now());
        match read_live_session(&self.cloud, &access, session_id).await {
            Ok(session) => {
                self.publish_live_state_from(&session);
                self.ensure_live_relay().await;
            }
            Err(LiveSessionReadFailure::Gone) => {
                self.stop_live_relay();
                self.live_session = None;
                self.fail_live("This live session has ended.");
            }
            Err(LiveSessionReadFailure::Transient) => {}
        }
    }

    async fn apply_live_session_policy(&mut self, policy: LiveSessionPolicySummary) {
        let Some((session_id, access)) = self.host_access() else {
            return;
        };
        let token = match BearerToken::new(&access) {
            Ok(token) => token,
            Err(_) => return,
        };
        match self
            .cloud
            .update_live_session_policy(&token, session_id, contract_policy(policy))
            .await
        {
            Ok(response) => self.publish_live_state_from(&response.into_body()),
            Err(error) => self.set_live_notice(live_session_error_message(&error)),
        }
    }

    async fn manage_participant(&mut self, principal_id: &str, action: ParticipantAction) {
        let Some((session_id, access)) = self.host_access() else {
            return;
        };
        let (Ok(participant), Ok(token)) = (
            principal_id.parse::<uuid::Uuid>(),
            BearerToken::new(&access),
        ) else {
            return;
        };
        let outcome = match action {
            ParticipantAction::Approve => {
                self.cloud
                    .approve_live_session_participant(&token, session_id, participant)
                    .await
            }
            ParticipantAction::SetEditor(editor) => {
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
            ParticipantAction::Remove => {
                self.cloud
                    .remove_live_session_participant(&token, session_id, participant)
                    .await
            }
        };
        match outcome {
            Ok(_) => self.refresh_live_session().await,
            Err(error) => self.set_live_notice(live_session_error_message(&error)),
        }
    }

    async fn end_live_session(&mut self) {
        let Some((session_id, access)) = self.host_access() else {
            return;
        };
        let Ok(token) = BearerToken::new(&access) else {
            return;
        };
        match self.cloud.end_live_session(&token, session_id).await {
            Ok(_) => {
                self.stop_live_relay();
                self.live_session = None;
                self.snapshot.live_session = None;
                self.publish();
            }
            Err(error) => self.set_live_notice(live_session_error_message(&error)),
        }
    }

    fn leave_live_session(&mut self) {
        if self
            .live_session
            .as_ref()
            .is_none_or(|runtime| runtime.hosting)
        {
            return;
        }
        self.stop_live_relay();
        self.live_session = None;
        self.snapshot.live_session = None;
        self.publish();
    }

    fn host_access(&self) -> Option<(uuid::Uuid, String)> {
        let runtime = self.live_session.as_ref()?;
        runtime
            .hosting
            .then_some((runtime.session_id, self.access.as_ref()?.token.clone()))
    }

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
        let pending = self.principal_id.is_some_and(|own| {
            session.participants.iter().any(|participant| {
                participant.principal_id == own
                    && participant.admission == LiveSessionAdmission::Pending
            })
        });
        self.snapshot.live_session = Some(if runtime.hosting {
            LiveSessionState::Hosting(summary)
        } else if pending {
            LiveSessionState::AwaitingApproval(summary)
        } else {
            LiveSessionState::Participating(summary)
        });
        self.publish();
    }

    fn attach_live_relay(&mut self, ticket: &LiveSessionTicketProtocol) {
        if let Some(relay) = self.relay.take() {
            relay.stop();
        }
        self.relay_retry_at = None;
        let Some(client_instance_id) = self
            .live_session
            .as_ref()
            .map(|runtime| runtime.client_instance_id)
        else {
            return;
        };
        let Some(url) = browser_live_relay::relay_endpoint(
            &self.configuration.api_origin,
            &ticket.websocket_endpoint,
        ) else {
            self.set_live_notice("The live connection address is invalid.".to_owned());
            self.schedule_live_relay_retry();
            return;
        };
        self.relay_generation += 1;
        let generation = self.relay_generation;
        let (outbound, outbound_receiver) =
            tokio::sync::mpsc::channel(LIVE_RELAY_OUTBOUND_QUEUE_CAPACITY);
        let (inbound_sender, inbound) =
            std::sync::mpsc::sync_channel(LIVE_RELAY_INBOUND_QUEUE_CAPACITY);
        match browser_live_relay::spawn(
            browser_live_relay::BrowserRelayConnection {
                url,
                ticket_protocol: ticket.ticket_protocol.clone(),
                generation,
            },
            outbound_receiver,
            inbound_sender,
            self.relay_commands.clone(),
            self.repaint.clone(),
        ) {
            Ok(relay) => {
                let _ = self.relay_ports.send(LiveRelayPort {
                    client_instance_id,
                    outbound,
                    inbound,
                });
                self.relay = Some(relay);
                if let Some(repaint) = &self.repaint {
                    repaint.request_repaint();
                }
            }
            Err(()) => {
                self.schedule_live_relay_retry();
                self.set_relay_connected(false);
            }
        }
    }

    fn stop_live_relay(&mut self) {
        if let Some(relay) = self.relay.take() {
            relay.stop();
        }
        self.relay_retry_at = None;
        self.relay_retry_attempt = 0;
    }

    fn schedule_live_relay_retry(&mut self) {
        let Some(client_instance_id) = self
            .live_session
            .as_ref()
            .map(|runtime| runtime.client_instance_id)
        else {
            self.relay_retry_at = None;
            return;
        };
        let delay = client_retry_delay(self.relay_retry_attempt, client_instance_id);
        self.relay_retry_attempt = self.relay_retry_attempt.saturating_add(1);
        self.relay_retry_at = Some(Instant::now() + delay);
    }

    async fn ensure_live_relay(&mut self) {
        if self.relay.is_some() || self.relay_blocked {
            return;
        }
        if !matches!(
            self.snapshot.live_session,
            Some(LiveSessionState::Hosting(_) | LiveSessionState::Participating(_))
        ) {
            return;
        }
        if self
            .relay_retry_at
            .is_some_and(|retry_at| Instant::now() < retry_at)
        {
            return;
        }
        let Some(runtime) = self.live_session.as_ref() else {
            return;
        };
        let (session_id, client_instance_id) = (runtime.session_id, runtime.client_instance_id);
        let Some(access) = self.access.as_ref().map(|access| access.token.clone()) else {
            return;
        };
        self.relay_retry_at = Some(Instant::now() + LIVE_SESSION_POLL_PERIOD);
        match issue_live_session_ticket(&self.cloud, &access, session_id, client_instance_id).await
        {
            Ok(ticket) => self.attach_live_relay(&ticket),
            Err(_) => self.schedule_live_relay_retry(),
        }
    }

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
                self.relay_retry_attempt = 0;
                self.relay_blocked = false;
                self.live_session = None;
                self.snapshot.live_session = Some(LiveSessionState::Failed { message });
                self.publish();
            }
            LiveRelayClosure::Rejected => {
                self.relay_retry_attempt = 0;
                self.relay_blocked = true;
                self.set_relay_connected(false);
                self.set_live_notice("The live connection failed.".to_owned());
            }
            LiveRelayClosure::Interrupted => {
                self.set_relay_connected(false);
                self.schedule_live_relay_retry();
            }
        }
    }

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

    fn fail_live(&mut self, message: &str) {
        self.snapshot.live_session = Some(LiveSessionState::Failed {
            message: message.to_owned(),
        });
        self.publish();
    }
}

enum ParticipantAction {
    Approve,
    SetEditor(bool),
    Remove,
}

struct BootstrapReads {
    principal_id: uuid::Uuid,
    principal: PrincipalSummary,
    entitlements: Vec<EntitlementSummary>,
    workspaces: Vec<WorkspaceSummary>,
}

enum BootstrapError {
    Unreachable,
    Rejected,
    Contract,
}

fn classify_bootstrap(error: &CloudError) -> BootstrapError {
    match error.status() {
        Some(401) | Some(403) => BootstrapError::Rejected,
        Some(_) => BootstrapError::Contract,
        None if matches!(error, CloudError::Transport { .. }) => BootstrapError::Unreachable,
        None => BootstrapError::Contract,
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
        .map_err(|error| classify_bootstrap(&error))?
        .into_body();
    let mut entitlements: Vec<Entitlement> = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let request = match cursor.as_deref() {
            Some(cursor) => PageRequest::after(PAGE_LIMIT, cursor),
            None => PageRequest::first(PAGE_LIMIT),
        }
        .map_err(|_| BootstrapError::Contract)?;
        let page = cloud
            .list_entitlements(&token, request)
            .await
            .map_err(|error| classify_bootstrap(&error))?
            .into_body();
        entitlements.extend(page.items);
        cursor = page.next_cursor;
        if cursor.is_none() {
            break;
        }
    }
    let mut workspaces = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let request = match cursor.as_deref() {
            Some(cursor) => PageRequest::after(PAGE_LIMIT, cursor),
            None => PageRequest::first(PAGE_LIMIT),
        }
        .map_err(|_| BootstrapError::Contract)?;
        let page = cloud
            .list_workspaces(&token, request)
            .await
            .map_err(|error| classify_bootstrap(&error))?
            .into_body();
        workspaces.extend(page.items.into_iter().map(|workspace| WorkspaceSummary {
            id: workspace.id.to_string(),
            name: workspace.name,
        }));
        cursor = page.next_cursor;
        if cursor.is_none() {
            break;
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

async fn issue_live_session_ticket(
    cloud: &CloudClient,
    access: &str,
    session_id: uuid::Uuid,
    client_instance_id: uuid::Uuid,
) -> Result<LiveSessionTicketProtocol, String> {
    let token = BearerToken::new(access).map_err(|_| LIVE_SESSION_UNREACHABLE.to_owned())?;
    cloud
        .issue_live_session_ticket(&token, session_id, client_instance_id)
        .await
        .map(rspice_cloud_client::CloudResponse::into_body)
        .map_err(|error| live_session_error_message(&error))
}

enum LiveSessionReadFailure {
    Gone,
    Transient,
}

async fn read_live_session(
    cloud: &CloudClient,
    access: &str,
    session_id: uuid::Uuid,
) -> Result<LiveSession, LiveSessionReadFailure> {
    let token = BearerToken::new(access).map_err(|_| LiveSessionReadFailure::Transient)?;
    match cloud.get_live_session(&token, session_id).await {
        Ok(response) => Ok(response.into_body()),
        Err(CloudError::Problem { details, .. }) if matches!(details.status, 403 | 404 | 410) => {
            Err(LiveSessionReadFailure::Gone)
        }
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

fn live_session_error_message(error: &CloudError) -> String {
    match error {
        CloudError::Problem { details, .. } => details.title.clone(),
        _ => LIVE_SESSION_UNREACHABLE.to_owned(),
    }
}

fn entitlement_summary(entitlement: &Entitlement) -> EntitlementSummary {
    EntitlementSummary {
        status: entitlement.status.as_str().to_owned(),
        granted_features: granted_features(&entitlement.features),
        valid_until: entitlement.valid_until.clone(),
    }
}

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

fn now_rfc3339() -> Option<String> {
    time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .ok()
}

#[derive(serde::Deserialize, serde::Serialize)]
struct PendingPkceRecord {
    state: String,
    verifier: String,
    redirect_uri: String,
    created_at_unix_seconds: i64,
}

struct BrowserCallback {
    code: String,
    verifier: String,
    redirect_uri: String,
}

fn store_pkce_record(record: &PendingPkceRecord) -> Result<(), ()> {
    let storage = session_storage()?;
    let serialized = serde_json::to_string(record).map_err(|_| ())?;
    if serialized.len() > MAX_PKCE_RECORD_BYTES {
        return Err(());
    }
    storage
        .set_item(PKCE_STORAGE_KEY, &serialized)
        .map_err(|_| ())?;
    let reread = storage.get_item(PKCE_STORAGE_KEY).map_err(|_| ())?;
    if reread.as_deref() == Some(serialized.as_str()) {
        Ok(())
    } else {
        Err(())
    }
}

fn clear_pkce_record() {
    if let Ok(storage) = session_storage() {
        let _ = storage.remove_item(PKCE_STORAGE_KEY);
    }
}

fn session_storage() -> Result<web_sys::Storage, ()> {
    web_sys::window()
        .ok_or(())?
        .session_storage()
        .map_err(|_| ())?
        .ok_or(())
}

fn take_browser_callback(
    configuration: &CloudAccountConfig,
) -> Result<Option<BrowserCallback>, String> {
    let window =
        web_sys::window().ok_or_else(|| "The browser window is unavailable.".to_owned())?;
    let current = url::Url::parse(
        &window
            .location()
            .href()
            .map_err(|_| "The browser address is unavailable.".to_owned())?,
    )
    .map_err(|_| "The browser callback address is invalid.".to_owned())?;
    let redirect = url::Url::parse(&configuration.browser_redirect_uri)
        .map_err(|_| "The browser callback configuration is invalid.".to_owned())?;
    if current.origin() != redirect.origin() {
        clear_pkce_record();
        return Err("The browser callback origin does not match this RSpice build.".to_owned());
    }
    if !oidc::is_exact_browser_callback_route(&current, &redirect) {
        return Ok(None);
    }
    let query = oidc::parse_authorization_callback_parameters(current.query());
    let fragment = oidc::parse_authorization_callback_parameters(current.fragment());
    let query_is_oauth = query
        .as_ref()
        .is_ok_and(|parameters| parameters.has_oauth_parameter);
    let fragment_is_oauth = fragment
        .as_ref()
        .is_ok_and(|parameters| parameters.has_oauth_parameter);
    // RSpice navigation also uses query parameters and fragments. A normal
    // route on the registered callback path must remain untouched.
    if query.is_ok() && fragment.is_ok() && !query_is_oauth && !fragment_is_oauth {
        return Ok(None);
    }

    // Once a callback is OAuth-shaped (or too large to classify safely),
    // remove every query/fragment byte before validating it. Duplicate,
    // fragment-delivered, and malformed responses must not survive in browser
    // history merely because they are rejected.
    window
        .history()
        .map_err(|_| "The browser history API is unavailable.".to_owned())?
        .replace_state_with_url(
            &wasm_bindgen::JsValue::NULL,
            "",
            Some(configuration.browser_redirect_uri.as_str()),
        )
        .map_err(|_| {
            "The sign-in callback could not be cleared from browser history.".to_owned()
        })?;
    let (Ok(parameters), Ok(fragment)) = (query, fragment) else {
        clear_pkce_record();
        return Err("The sign-in callback parameters are invalid.".to_owned());
    };
    if fragment.has_oauth_parameter {
        clear_pkce_record();
        return Err("The sign-in callback used an unsupported response transport.".to_owned());
    }
    if parameters.has_duplicate {
        clear_pkce_record();
        return Err("The sign-in callback contained duplicate parameters.".to_owned());
    }

    let storage = session_storage()
        .map_err(|_| "The browser blocked temporary sign-in storage.".to_owned())?;
    let raw = storage
        .get_item(PKCE_STORAGE_KEY)
        .map_err(|_| "The browser blocked temporary sign-in storage.".to_owned())?
        .ok_or_else(|| "This sign-in callback has no matching request.".to_owned())?;
    let _ = storage.remove_item(PKCE_STORAGE_KEY);
    if raw.len() > MAX_PKCE_RECORD_BYTES {
        return Err("The pending sign-in record is invalid.".to_owned());
    }
    let pending: PendingPkceRecord = serde_json::from_str(&raw)
        .map_err(|_| "The pending sign-in record is invalid.".to_owned())?;
    let age = time::OffsetDateTime::now_utc()
        .unix_timestamp()
        .saturating_sub(pending.created_at_unix_seconds);
    let base64url = |value: &str| {
        value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    };
    if !(0..=PKCE_RECORD_MAX_AGE_SECONDS).contains(&age)
        || pending.redirect_uri != configuration.browser_redirect_uri
        || pending.state.len() != 43
        || !base64url(&pending.state)
        || pending.verifier.len() != 64
        || !base64url(&pending.verifier)
    {
        return Err("The pending sign-in request expired or does not match this build.".to_owned());
    }
    let returned_state = parameters
        .state
        .ok_or_else(|| "The sign-in callback omitted its state.".to_owned())?;
    if returned_state.len() > 256 || returned_state != pending.state {
        return Err("The sign-in response did not match this browser tab.".to_owned());
    }
    if parameters.has_error {
        return Err("Sign-in was cancelled in the browser.".to_owned());
    }
    let code = parameters
        .code
        .ok_or_else(|| "The sign-in callback omitted its authorization code.".to_owned())?;
    if code.is_empty() || code.len() > 4096 {
        return Err("The sign-in authorization code is invalid.".to_owned());
    }
    Ok(Some(BrowserCallback {
        code,
        verifier: pending.verifier,
        redirect_uri: pending.redirect_uri,
    }))
}
