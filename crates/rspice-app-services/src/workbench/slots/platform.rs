//! The platform services' runtime state: what opening, saving and recovering
//! a project keeps between frames, the recovery catalog included.

use crate::services::recent_files::{RecentFile, RecentKind};
use rspice_app_state::workbench::app_state::model::Model;

/// What the platform services keep open between frames.
#[derive(Debug, Clone)]
pub struct PlatformState {
    /// A saved-file open found a bound, eligible autosave checkpoint; the
    /// restore dialog resolves it before either exact byte snapshot is loaded.
    #[cfg(not(target_arch = "wasm32"))]
    pub pending_autosave_restore:
        Option<crate::workbench::lifecycle::recovery_checkpoint::AutosaveRestoreCandidate>,
    /// The open project's autosave: its timer, the checkpoint being written
    /// and the recovered work an open found. Transient.
    pub project_autosave: Box<crate::workbench::app_state::session::autosave::ProjectAutosave>,
    /// Autosave checkpoint interval in minutes; 0 = off. Checkpoints write
    /// next to the project file and never touch it.
    pub autosave_minutes: u8,
    /// The last autosave checkpoint could not be written. A failure is said
    /// when it starts, not again at every interval it persists. Transient.
    pub autosave_failing: bool,
    /// What the last save of the session kept. Like `autosave_failing`, it
    /// makes a refusal said when it starts, not at every save. Transient.
    pub session_kept: crate::services::session::SessionKept,
    /// The runs the browser keeps for the session, written only when they
    /// changed. Transient.
    pub session_runs: crate::workbench::app_state::session::browser_session::KeptRuns,
    /// The scanned native checkpoint catalog the Project Launcher's recovery
    /// page lists. Runtime state, never project or user preferences.
    pub recovery_catalog: Box<crate::workbench::lifecycle::recovery::RecoveryCatalog>,
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
    /// A save keeping its results beside its file before it writes the file.
    #[cfg(not(target_arch = "wasm32"))]
    pub results_save: Option<crate::workbench::lifecycle::project_lifecycle::ResultsSave>,
    /// The first checks of kept segments a project open or a save left to run beside them.
    #[cfg(not(target_arch = "wasm32"))]
    pub segment_checks: Vec<crate::io::result_segments::SegmentCheck>,
    /// A pack of the project with its results being written, or one being opened.
    #[cfg(not(target_arch = "wasm32"))]
    pub project_pack: Option<crate::io::project_pack::ProjectPackJob>,
}

impl Default for PlatformState {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            pending_autosave_restore: None,
            project_autosave: Default::default(),
            // Crash recovery is not opt-in; Off stays available.
            autosave_minutes: 5,
            autosave_failing: false,
            session_kept: Default::default(),
            session_runs: Default::default(),
            recovery_catalog: Default::default(),
            project_lifecycle: Box::default(),
            recent_files: Vec::new(),
            browser_schematic_save_name: None,
            browser_project_save_name: None,
            native_project_binding_receipt: None,
            browser_project_binding_receipt: None,
            #[cfg(not(target_arch = "wasm32"))]
            results_save: None,
            #[cfg(not(target_arch = "wasm32"))]
            segment_checks: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            project_pack: None,
        }
    }
}

/// Record a file in the recent-files list (most recent first, deduped,
/// capped at Projects and files ▸ Recent projects listed). A file already
/// listed keeps its pin.
pub fn remember_recent_file(
    model: &Model,
    platform: &mut PlatformState,
    kind: RecentKind,
    path: &std::path::Path,
) {
    let previous = platform
        .recent_files
        .iter()
        .find(|recent| recent.path == path)
        .cloned();
    let entry = RecentFile {
        kind,
        path: path.to_path_buf(),
        opened_at_unix_ms: crate::time_compat::unix_epoch()
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX),
        pinned: previous.is_some_and(|recent| recent.pinned),
        missing: false,
    };
    platform
        .recent_files
        .retain(|recent| recent.path != entry.path);
    platform.recent_files.insert(0, entry);
    trim_recent_files(model, platform);
}

/// Drop the oldest recent projects beyond what Projects and files ▸ Recent
/// projects listed keeps, so lowering it shortens the start page's list at
/// once. The count is of projects, the only kind that list shows: schematics
/// are capped at the same count on their own, so opening them never pushes a
/// project off the page. A pinned entry is the reader's to remove: it is
/// kept and not counted.
pub fn trim_recent_files(model: &Model, platform: &mut PlatformState) {
    let listed = model.preferences.recent_files_listed();
    let (mut projects, mut schematics) = (0, 0);
    platform.recent_files.retain(|recent| {
        if recent.pinned {
            return true;
        }
        let kept = match recent.kind {
            RecentKind::Project => &mut projects,
            RecentKind::Schematic => &mut schematics,
        };
        *kept += 1;
        *kept <= listed
    });
}

/// Why the LTspice or QSPICE schematic import cannot run now, if it cannot:
/// natively, what the model and the platform state refuse; in the browser,
/// always, since the source program's symbol library is read from this
/// computer's disk. With no project open the import makes one, so that
/// refuses nothing.
pub fn foreign_import_refusal_in(model: &Model, platform: &PlatformState) -> Option<&'static str> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        if platform.project_lifecycle.is_open() && model.safe_mode.project_read_only() {
            Some("the project is open read-only")
        } else if model.simulation.execution.has_active_execution() {
            Some("an active simulation execution still owns the project")
        } else {
            None
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (model, platform);
        Some("the source program's symbol library is read from this computer's disk")
    }
}
