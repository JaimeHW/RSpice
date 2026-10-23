//! The hardcopy feature's session: Page Setup, Print and Export Active View,
//! and the report documents being authored.

/// What the hardcopy feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct HardcopyState {
    /// Unified Page Setup / Print / Export Active View transaction. Draft
    /// controls and preview state are runtime-only; committed setups live in
    /// `ProjectWorkspace::hardcopy_setups`.
    pub(crate) hardcopy: crate::workbench::app::HardcopyDialogState,
    /// One-shot request to export the active view as a publication figure.
    ///
    /// This used to capture the window and crop it, which handed the reader
    /// back the pixels they already had at whatever resolution the display
    /// happened to be. It opens the hardcopy export workflow instead — the
    /// one pipeline that renders a sheet as PDF/A, PDF, SVG or PNG at a
    /// chosen resolution, and the same one Print already used.
    pub export_figure_requested: bool,
    /// Session-local selection and editor drafts for the project-owned report
    /// documents.
    pub report_authoring: rspice_results_ui::report::session::ReportAuthoringState,
}
