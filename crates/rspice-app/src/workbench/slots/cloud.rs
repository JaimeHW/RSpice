//! The cloud feature's dialogs: license activation, web publication and live
//! sessions; and what the cloud shows other features, from this state and
//! from the sessions it runs.

use crate::services::live_protocol::CursorLocus;
use crate::workbench::framework::feature::{CloudPort, CloudSessionPort, LiveCursor};
use crate::workbench::live_session::CloudHandles;

/// What the cloud feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct CloudState {
    /// License activation dialog state
    pub license_dialog: crate::workbench::app::dialogs::license::LicenseDialogState,

    /// Immutable cloud publication transaction.
    pub(crate) publish_web: crate::workbench::app::dialogs::publish_web::PublishWebDialogState,

    /// Live-session lifecycle controls (start review, host roster, guest view).
    pub(crate) live_session: crate::workbench::app::dialogs::live_session::LiveSessionDialogState,

    /// Owner-confirmed publication tombstone transaction.
    pub(crate) unpublish_web:
        crate::workbench::app::dialogs::unpublish_web::UnpublishWebDialogState,

    /// The activated license key as pasted (persisted; re-verified on load).
    pub(crate) license_key: Option<String>,
    /// The verified grant behind `license_key` (derived, never persisted).
    pub(crate) license: Option<crate::services::license::LicenseInfo>,
}

impl CloudState {
    /// Take the license kept on this computer when the session holds none:
    /// its key and the grant it verifies to.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn restore_stored_license(&mut self) {
        if self.license.is_none()
            && let Some((key, info)) = crate::services::license::load_stored()
        {
            self.license_key = Some(key);
            self.license = Some(info);
        }
    }
}

impl CloudPort for CloudState {
    fn license_dialog_open(&self) -> bool {
        self.license_dialog.open
    }

    fn license_holder(&self) -> Option<&str> {
        self.license
            .as_ref()
            .map(|license| license.licensed_to.as_str())
    }
}

impl CloudSessionPort for CloudHandles {
    fn review_actor(&self, cloud: &dyn CloudPort) -> String {
        crate::workbench::user_identity::user_display_name(
            self.account.snapshot(),
            cloud.license_holder(),
        )
    }

    fn account_holder(&self, cloud: &dyn CloudPort) -> Option<String> {
        crate::workbench::user_identity::account_holder(
            self.account.snapshot(),
            cloud.license_holder(),
        )
    }

    /// A peer's cursor counts when it sits on `doc` at a finite point.
    fn live_cursors(&self, doc: &str) -> Vec<LiveCursor> {
        self.live_session
            .peers()
            .filter_map(|peer| match peer.cursor.as_ref() {
                Some(CursorLocus::Canvas {
                    doc: peer_doc,
                    x,
                    y,
                }) if peer_doc == doc && x.is_finite() && y.is_finite() => Some(LiveCursor {
                    principal: peer.identity.principal_id.as_u128(),
                    name: self.live_session.display_name(&peer.identity),
                    x: *x,
                    y: *y,
                }),
                _ => None,
            })
            .collect()
    }
}
