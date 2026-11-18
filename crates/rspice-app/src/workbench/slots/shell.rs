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
    /// Quitting asks whether to wait for the save copying its results to
    /// another volume, or to quit without saving.
    pub(crate) quit_while_saving: bool,
    /// Whether the focused source editor has keys of this frame still to
    /// apply, which it does as the page draws.
    pub(crate) editor_keys_pending: bool,
    /// The commands raised while the editor's keys were pending, carried out
    /// in the order they were raised once the page has drawn.
    pub(crate) held_commands: Vec<rspice_app_state::state::command_catalog::CommandKey>,
}
