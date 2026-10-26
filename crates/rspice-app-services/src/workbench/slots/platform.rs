//! The platform services' runtime state: what opening, saving and recovering
//! a project keeps between frames, the recovery catalog included.

use crate::services::recent_files::RecentKind;
use rspice_app_state::workbench::app_state::model::Model;

/// What the platform services keep open between frames.
#[derive(Debug, Clone)]
pub struct PlatformState {
    /// A saved-file open found a bound, eligible autosave checkpoint; the
    /// restore dialog resolves it before either exact byte snapshot is loaded.
    #[cfg(not(target_arch = "wasm32"))]
    pub pending_autosave_restore:
        Option<crate::workbench::lifecycle::recovery_checkpoint::AutosaveRestoreCandidate>,
    /// Autosave checkpoint interval in minutes; 0 = off. Checkpoints write
    /// next to the project file and never touch it.
    pub autosave_minutes: u8,
    /// The last autosave checkpoint could not be written. A failure is said
    /// when it starts, not again at every interval it persists. Transient.
    pub autosave_failing: bool,
    /// What the last save of the session kept. Like `autosave_failing`, it
    /// makes a refusal said when it starts, not at every save. Transient.
    pub session_kept: crate::services::session::SessionKept,
    /// The scanned native checkpoint catalog the Project Launcher's recovery
    /// page lists. Runtime state, never project or user preferences.
    pub recovery_catalog: crate::workbench::lifecycle::recovery::RecoveryCatalog,
    /// Transactional document registry, accepted project baseline, and
    /// canonical persistence identity. Runtime-only apart from whether a
    /// project is open; session recovery keeps the working set separately and
    /// reconstructs this boundary at startup.
    pub project_lifecycle:
        Box<crate::workbench::lifecycle::project_lifecycle::ProjectLifecycleState>,
    /// Recently opened/saved schematic and project files, most recent first
    /// (persisted across sessions). The projects are the start page's recent
    /// list; the schematics are what crash recovery scans.
    pub recent_files: Vec<crate::services::recent_files::RecentFile>,
    /// Browser-only suggested filename for the next schematic download.
    pub browser_schematic_save_name: Option<String>,
    /// Browser-only suggested filename for the next project download.
    pub browser_project_save_name: Option<String>,
    /// Exact native canonical-file authority saved with the working session.
    /// A restored pathname alone never grants ordinary Save authority.
    pub native_project_binding_receipt:
        Option<crate::workbench::lifecycle::project_lifecycle::NativeBindingReceipt>,
    /// Exact browser canonical-binding authority saved with the working-set
    /// session.  The binding UUID is opaque and intentionally independent of
    /// the logical project UUID. Runtime file handles are never serialized.
    pub browser_project_binding_receipt:
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

/// Drop the oldest recent projects beyond what Projects and files ▸ Recent
/// projects listed keeps, so lowering it shortens the start page's list at
/// once. The count is of projects, the only kind that list shows: schematics
/// are capped at the same count on their own, so opening them never pushes a
/// project off the page.
pub fn trim_recent_files(model: &Model, platform: &mut PlatformState) {
    let listed = model.preferences.recent_files_listed();
    let (mut projects, mut schematics) = (0, 0);
    platform.recent_files.retain(|recent| {
        let kept = match recent.kind {
            RecentKind::Project => &mut projects,
            RecentKind::Schematic => &mut schematics,
        };
        *kept += 1;
        *kept <= listed
    });
}
