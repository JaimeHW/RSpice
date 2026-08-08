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

use rspice_cloud_client::contract::{Entitlement, IssueLicenseLeaseRequest, LicenseLease};
use rspice_cloud_client::{
    BearerToken, ClientConfig, CloudClient, NativeLicenseVerifier, PageRequest,
    VerifiedNativeLicense,
};

use super::config::CloudAccountConfig;
use super::{
    CloudAccountCommand, CloudAccountEvent, CloudSessionPhase, CloudSessionSnapshot,
    EntitlementSummary, LeaseSummary, NativeLicenseSummary, PrincipalSummary, PublicationSummary,
    PublishState, WorkspaceSummary, native_login, oidc, publish, store,
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

/// Spawn the executor thread; it restores any stored session immediately.
pub(super) fn spawn(
    configuration: CloudAccountConfig,
    repaint: Option<egui::Context>,
) -> (Sender<CloudAccountCommand>, Receiver<CloudAccountEvent>) {
    let (command_sender, command_receiver) = std::sync::mpsc::channel();
    let (event_sender, event_receiver) = std::sync::mpsc::channel();
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
            );
        })
        .expect("spawning the cloud account executor thread");
    (command_sender, event_receiver)
}

fn run(
    configuration: CloudAccountConfig,
    repaint: Option<egui::Context>,
    commands: Receiver<CloudAccountCommand>,
    listener_commands: Sender<CloudAccountCommand>,
    events: Sender<CloudAccountEvent>,
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
            CloudAccountCommand::CompleteSignIn { code, state } => {
                self.complete_sign_in(&code, &state);
            }
            CloudAccountCommand::SignInFailed { reason } => {
                if self.pending_sign_in.take().is_some() {
                    self.set_signed_out(Some(reason));
                }
            }
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

    fn sign_out(&mut self) {
        if let Some(pending) = self.pending_sign_in.take() {
            pending.cancel.store(true, Ordering::Relaxed);
        }
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

        self.snapshot = CloudSessionSnapshot {
            phase: CloudSessionPhase::Active,
            principal: Some(reads.principal),
            entitlements: reads.entitlements,
            workspaces: reads.workspaces,
            native_license,
            device_leases,
            publish: self.snapshot.publish.clone(),
            publications: std::mem::take(&mut self.snapshot.publications),
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
