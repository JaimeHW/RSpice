//! The cloud feature's dialogs: license activation, web publication and live
//! sessions, and the request to start mirroring a host.

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

    /// One-shot request raised by a completed close-to-mirror transaction;
    /// the live-session engine consumes it to start mirroring the host.
    pub(crate) live_mirror_entry_pending: bool,
}

impl CloudState {
    /// Raise the one-shot live-mirror entry request; the live-session engine
    /// consumes it on its next pump.
    pub(crate) fn request_live_mirror_entry(&mut self) {
        self.live_mirror_entry_pending = true;
    }

    pub(crate) fn take_live_mirror_entry(&mut self) -> bool {
        std::mem::take(&mut self.live_mirror_entry_pending)
    }
}
