//! The hardcopy feature's session: Page Setup, Print and Export Active View.

/// What the hardcopy feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct HardcopyState {
    /// Unified Page Setup / Print / Export Active View transaction. Draft
    /// controls and preview state are runtime-only; committed setups live in
    /// `ProjectWorkspace::hardcopy_setups`.
    pub(crate) hardcopy: crate::workbench::app::HardcopyDialogState,
}
