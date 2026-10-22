//! The cloud feature's dialogs: license activation, web publication and live
//! sessions.

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
}
