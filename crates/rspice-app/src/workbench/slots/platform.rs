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
    /// Transactional document registry, accepted project baseline, and
    /// canonical persistence identity. Runtime-only apart from whether a
    /// project is open; session recovery keeps the working set separately and
    /// reconstructs this boundary at startup.
    pub(crate) project_lifecycle:
        Box<crate::workbench::lifecycle::project_lifecycle::ProjectLifecycleState>,
    /// Recently opened/saved schematic and project files, most recent first
    /// (persisted across sessions). The projects are the start page's recent
    /// list; the schematics are what crash recovery scans.
    pub(crate) recent_files: Vec<crate::services::recent_files::RecentFile>,
    /// Browser-only suggested filename for the next schematic download.
    pub(crate) browser_schematic_save_name: Option<String>,
    /// Browser-only suggested filename for the next project download.
    pub(crate) browser_project_save_name: Option<String>,
    /// Exact native canonical-file authority saved with the working session.
    /// A restored pathname alone never grants ordinary Save authority.
    pub(crate) native_project_binding_receipt:
        Option<crate::workbench::lifecycle::project_lifecycle::NativeBindingReceipt>,
    /// Exact browser canonical-binding authority saved with the working-set
    /// session.  The binding UUID is opaque and intentionally independent of
    /// the logical project UUID. Runtime file handles are never serialized.
    pub(crate) browser_project_binding_receipt:
        Option<crate::workbench::lifecycle::project_lifecycle::BrowserBindingReceipt>,
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
            project_lifecycle: Box::default(),
            recent_files: Vec::new(),
            browser_schematic_save_name: None,
            browser_project_save_name: None,
            native_project_binding_receipt: None,
            browser_project_binding_receipt: None,
        }
    }
}
