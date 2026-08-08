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
    /// Authorization code + state delivered by the loopback listener.
    #[cfg(not(target_arch = "wasm32"))]
    CompleteSignIn { code: String, state: String },
    /// The loopback listener failed or timed out.
    #[cfg(not(target_arch = "wasm32"))]
    SignInFailed { reason: String },
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
    #[cfg(not(target_arch = "wasm32"))]
    commands: Option<std::sync::mpsc::Sender<CloudAccountCommand>>,
    #[cfg(not(target_arch = "wasm32"))]
    events: Option<std::sync::mpsc::Receiver<CloudAccountEvent>>,
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
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            match config::CloudAccountConfig::resolve() {
                Some(configuration) => {
                    let (commands, events) = executor::spawn(configuration, repaint);
                    Self {
                        availability: CloudAccountAvailability::Native,
                        snapshot: CloudSessionSnapshot::default(),
                        pending_browser_url: None,
                        pending_publish_receipt: None,
                        commands: Some(commands),
                        events: Some(events),
                    }
                }
                None => Self {
                    availability: CloudAccountAvailability::UnconfiguredBuild,
                    snapshot: CloudSessionSnapshot::default(),
                    pending_browser_url: None,
                    pending_publish_receipt: None,
                    commands: None,
                    events: None,
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
            #[cfg(not(target_arch = "wasm32"))]
            commands: None,
            #[cfg(not(target_arch = "wasm32"))]
            events: None,
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
            let Some(events) = self.events.as_ref() else {
                return false;
            };
            let mut changed = false;
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

    /// The receipt of a just-committed publish, at most once. The caller
    /// persists it into the project descriptor.
    pub(crate) fn take_publish_receipt(&mut self) -> Option<PublishReceipt> {
        self.pending_publish_receipt.take()
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
