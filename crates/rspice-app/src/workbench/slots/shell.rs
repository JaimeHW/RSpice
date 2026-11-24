//! The shell's own dialogs: About, the shortcut reference and the Help Center.

/// What the shell keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct ShellState {
    /// About dialog
    pub about: bool,
    /// Shortcuts help dialog
    pub shortcuts_help: bool,
    /// Shortcuts help filter text — nobody reads 45 unfiltered rows
    pub shortcuts_filter: String,
    /// Versioned help, migration, diagnostics, support, and notices manager.
    pub(crate) help_center: crate::workbench::app::dialogs::help::HelpCenterDialogState,
    /// Flag to signal that application exit has been requested (after confirmation)
    pub(crate) exit_requested: bool,
    /// The exit asked about the unsaved work and is closing the window, so
    /// that close goes through instead of asking again.
    pub(crate) exit_reviewed: bool,
    /// Quitting asks whether to wait for the save copying its results to
    /// another volume, or to quit without saving.
    pub(crate) quit_while_saving: bool,
    /// Closing waits while the session's results go into its folder, and
    /// closes once they are there.
    pub(crate) closing_for_session_results: bool,
    /// Whether the focused source editor has keys of this frame still to
    /// apply, which it does as the page draws.
    pub(crate) editor_keys_pending: bool,
    /// The commands raised while the editor's keys were pending, carried out
    /// in the order they were raised once the page has drawn.
    pub(crate) held_commands: Vec<rspice_app_state::state::command_catalog::CommandKey>,
    /// A deck file opened from disk while it is read away from the frame,
    /// its tab in front saying it is reading.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) opening_deck: Option<crate::workbench::session::deck_transactions::OpeningDeck>,
    /// The comparison of the open deck's text, edited back, with the text it
    /// was last saved with.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) saved_text: crate::workbench::session::deck_transactions::SavedTextCompare,
}
