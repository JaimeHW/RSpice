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
}
