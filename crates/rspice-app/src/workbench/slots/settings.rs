//! The settings feature's dialogs: Preferences, the workspace layout manager
//! and the keyboard shortcut workflows.

/// What the settings feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct SettingsState {
    /// Preferences dialog open
    pub preferences_open: bool,

    /// Device-local workspace layout manager owned by Preferences.
    pub(crate) workspace_layout_manager:
        crate::workbench::app::dialogs::preferences::workspace_layout_manager::WorkspaceLayoutManagerState,

    /// Transactional shortcut import/export workflows.
    pub(crate) shortcut_portability: crate::workbench::app::dialogs::preferences::shortcut_portability_dialogs::ShortcutPortabilityDialogsState,

    /// Transactional keyboard shortcut editor.
    pub(crate) shortcut_editor:
        crate::workbench::app::dialogs::preferences::shortcut_preferences::ShortcutEditorState,

    /// Browser-only persist-before-live policy candidate. Native publication
    /// completes in the initiating frame, while browser storage is async.
    pub(crate) shortcut_policy_candidate: Option<crate::state::shortcuts::ShortcutPreferences>,

    /// Browser-only opt-in for egui's Web Speech interaction feedback.
    /// Native assistive technology is negotiated automatically by AccessKit.
    pub browser_spoken_feedback: bool,
}
