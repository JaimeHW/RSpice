//! The hardcopy feature's session: Page Setup, Print and Export Active View,
//! and the report documents being authored.

/// What the hardcopy feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct HardcopyState {
    /// Unified Page Setup / Print / Export Active View transaction. Draft
    /// controls and preview state are runtime-only; committed setups live in
    /// `ProjectWorkspace::hardcopy_setups`.
    pub(crate) hardcopy: crate::workbench::app::HardcopyDialogState,
    /// Session-local selection and editor drafts for the project-owned report
    /// documents.
    pub report_authoring: rspice_results_ui::report::session::ReportAuthoringState,
}
