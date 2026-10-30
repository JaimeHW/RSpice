//! The settings feature's dialogs: Preferences, the workspace layout manager
//! and the keyboard shortcut workflows.

/// What the settings feature keeps open between frames.
#[derive(Debug, Clone, Default)]
pub struct SettingsState {
    /// Preferences dialog open
    pub preferences_open: bool,

    /// Device-local workspace layout manager owned by Preferences.
    pub workspace_layout_manager:
        crate::workbench::app::dialogs::preferences::workspace_layout_manager::WorkspaceLayoutManagerState,

    /// Transactional shortcut import/export workflows.
    pub shortcut_portability: crate::workbench::app::dialogs::preferences::shortcut_portability_dialogs::ShortcutPortabilityDialogsState,

    /// Transactional keyboard shortcut editor.
    pub shortcut_editor:
        crate::workbench::app::dialogs::preferences::shortcut_preferences::ShortcutEditorState,

    /// Browser-only persist-before-live policy candidate. Native publication
    /// completes in the initiating frame, while browser storage is async.
    pub shortcut_policy_candidate: Option<crate::state::shortcuts::ShortcutPreferences>,

    /// Browser-only opt-in for egui's Web Speech interaction feedback.
    /// Native assistive technology is negotiated automatically by AccessKit.
    pub browser_spoken_feedback: bool,

    /// Canonical device-local shortcut-library CAS authority. The recoverable
    /// eframe session copy never replaces this owner after startup.
    pub shortcut_library_persistence: crate::workbench::app_state::session::shortcut_library::ShortcutLibraryPersistenceRuntime,
    /// UI transaction paired with the single browser shortcut-library CAS.
    pub shortcut_library_publication_continuation: Option<
        crate::workbench::app_state::session::shortcut_library::ShortcutLibraryPublicationContinuation,
    >,
    /// Where the preferences file lives and what has been written to it.
    /// Unbound in a default state, so nothing but a launch touches the disk.
    pub preferences_file:
        Box<crate::workbench::app_state::session::preferences_file::PreferencesFileRuntime>,
}
