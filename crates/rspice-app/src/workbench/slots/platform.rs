//! The platform services' runtime state: what opening, saving and recovering
//! a project keeps between frames, the recovery catalog included.

/// What the platform services keep open between frames.
#[derive(Debug, Clone)]
pub(crate) struct PlatformState {
    /// A saved-file open found a bound, eligible autosave checkpoint; the
    /// restore dialog resolves it before either exact byte snapshot is loaded.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) pending_autosave_restore:
        Option<crate::workbench::lifecycle::recovery_checkpoint::AutosaveRestoreCandidate>,
    /// Autosave checkpoint interval in minutes; 0 = off. Checkpoints write
    /// next to the project file and never touch it.
    pub(crate) autosave_minutes: u8,
    /// The last autosave checkpoint could not be written. A failure is said
    /// when it starts, not again at every interval it persists. Transient.
    pub(crate) autosave_failing: bool,
    /// What the last save of the session kept. Like `autosave_failing`, it
    /// makes a refusal said when it starts, not at every save. Transient.
    pub(crate) session_kept: crate::services::session::SessionKept,
    /// The scanned native checkpoint catalog the Project Launcher's recovery
    /// page lists. Runtime state, never project or user preferences.
    pub(crate) recovery_catalog: crate::workbench::lifecycle::recovery::RecoveryCatalog,
}

impl Default for PlatformState {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            pending_autosave_restore: None,
            // Crash recovery is not opt-in; Off stays available.
            autosave_minutes: 5,
            autosave_failing: false,
            session_kept: Default::default(),
            recovery_catalog: Default::default(),
        }
    }
}
