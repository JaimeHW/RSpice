//! Schematic file workflows.
//!
//! New, open, save, and save-as for a standalone schematic, behind the same
//! I/O trait split as the export workflows.

use std::path::{Path, PathBuf};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::io::SchematicIoError;
use crate::state::{SchematicState, ViewType};
use crate::workbench::app_state::AppState;

#[derive(Debug, Clone, Copy)]
pub(crate) enum SchematicLoadOrigin<'a> {
    PersistentPath(&'a Path),
    #[cfg(any(test, target_arch = "wasm32"))]
    BrowserImport(&'a str),
}

impl<'a> SchematicLoadOrigin<'a> {
    fn display_label(self) -> String {
        match self {
            Self::PersistentPath(path) => path.display().to_string(),
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(name) => name.to_string(),
        }
    }

    /// The file's name without its extension.
    fn file_stem(self) -> &'a str {
        match self {
            Self::PersistentPath(path) => path
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default(),
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(name) => Path::new(name)
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default(),
        }
    }

    fn recent_path(self) -> Option<&'a Path> {
        match self {
            Self::PersistentPath(path) => Some(path),
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(_) => None,
        }
    }

    fn success_prefix(self) -> &'static str {
        match self {
            Self::PersistentPath(_) => "Opened schematic",
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(_) => "Imported schematic",
        }
    }
}

/// IO abstraction for schematic file workflows.
///
/// This allows open/save behavior to be fully unit tested without invoking
/// native dialogs.
pub(crate) trait FileWorkflowIo {
    #[cfg(not(target_arch = "wasm32"))]
    fn show_open_dialog(&self) -> Result<PathBuf, SchematicIoError>;
    fn show_save_dialog(&self, default_name: Option<&str>) -> Result<PathBuf, SchematicIoError>;
    fn load_schematic(&self, path: &Path) -> Result<SchematicState, SchematicIoError>;
    /// Write `schematic` to `path`, first copying the previous file to a
    /// backup when `keep_backup` asks.
    fn save_schematic(
        &self,
        schematic: &SchematicState,
        path: &Path,
        keep_backup: bool,
    ) -> Result<(), SchematicIoError>;
    fn saved_paths_are_reopenable(&self) -> bool {
        true
    }
}

/// Production IO backend using native dialogs and filesystem operations.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct NativeFileWorkflowIo;

impl FileWorkflowIo for NativeFileWorkflowIo {
    #[cfg(not(target_arch = "wasm32"))]
    fn show_open_dialog(&self) -> Result<PathBuf, SchematicIoError> {
        crate::io::show_open_dialog()
    }

    fn show_save_dialog(&self, default_name: Option<&str>) -> Result<PathBuf, SchematicIoError> {
        crate::io::show_save_dialog(default_name)
    }

    fn load_schematic(&self, path: &Path) -> Result<SchematicState, SchematicIoError> {
        crate::io::load_schematic(path)
    }

    fn save_schematic(
        &self,
        schematic: &SchematicState,
        path: &Path,
        keep_backup: bool,
    ) -> Result<(), SchematicIoError> {
        crate::io::save_schematic(schematic, path, keep_backup)
    }

    fn saved_paths_are_reopenable(&self) -> bool {
        !cfg!(target_arch = "wasm32")
    }
}

/// Reset the current schematic to a new empty document.
#[cfg(test)]
pub(crate) fn create_new_schematic(state: &mut AppState) {
    let schematic = state.new_schematic_document();
    replace_active_schematic_document(state, schematic);
    state.clear_design_execution_context();
    state.record(
        LogSeverity::Info,
        LogSource::Studio,
        "Created a new schematic.",
    );
}

fn replace_active_schematic_document(state: &mut AppState, mut schematic: SchematicState) {
    // Loaded document data owns the grid pitch; the active device session owns
    // snap targets and radius. Install both authorities before publishing the
    // new active buffer, then mirror the derived pitch back into the session.
    schematic.session.editor.snap_engine = state.ui.schematic_snap.clone();
    schematic.reconcile_grid_pitch_runtime();
    state.model.schematic = schematic;
    state.ui.schematic_snap = state.model.schematic.session.editor.snap_engine.clone();
    state.bump_active_schematic_epoch();
    match state.model.workspace.content.active_view_type() {
        ViewType::Schematic | ViewType::Testbench => {
            state.sync_active_schematic_to_workspace();
        }
        _ => {
            let reference = state.model.workspace.content.active_schematic_reference();
            let dirty = state.model.schematic.session.is_dirty;
            state
                .model
                .workspace
                .insert_schematic_editor(reference.key(), state.model.schematic.clone());
            if let Some(open_view) = state
                .model
                .workspace
                .content
                .open_views
                .iter_mut()
                .find(|open_view| open_view.reference == reference)
            {
                open_view.dirty = dirty;
            }
        }
    }
}

/// Sibling checkpoint path for a schematic file: `design.rsp` →
/// `design.rsp.autosave`. Native only — the web build has no checkpoints.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn autosave_checkpoint_path(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(".autosave");
    PathBuf::from(name)
}

/// Open a schematic, first offering any checkpoint that is eligible under its
/// exact saved-source binding (native only; the web build has no checkpoint
/// files).
pub(crate) fn load_schematic_from_path_with_io(
    state: &mut AppState,
    path: &Path,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    // A bound checkpoint from an interrupted session is reviewed before the
    // saved source. Legacy or changed-source checkpoints only interrupt this
    // open when they are strictly newer; stale evidence remains in Recovery.
    #[cfg(not(target_arch = "wasm32"))]
    match crate::workbench::lifecycle::recovery_checkpoint::autosave_restore_candidate(path) {
        Ok(Some(candidate)) => {
            state.dialogs.pending_autosave_restore = Some(candidate);
            return true;
        }
        Ok(None) => {}
        Err(error) => {
            state.record(LogSeverity::Warning, LogSource::Studio, error);
        }
    }
    load_schematic_bypassing_autosave(state, path, io)
}

/// The actual load, with no checkpoint interception — the restore
/// dialog's own paths must not re-trigger it.
pub(crate) fn load_schematic_bypassing_autosave(
    state: &mut AppState,
    path: &Path,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    match io.load_schematic(path) {
        Ok(schematic) => {
            apply_loaded_schematic(state, schematic, SchematicLoadOrigin::PersistentPath(path))
        }
        Err(e) => {
            state.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Schematic open failed: {e}"),
            );
            false
        }
    }
}

pub(crate) fn apply_loaded_schematic(
    state: &mut AppState,
    schematic: SchematicState,
    origin: SchematicLoadOrigin<'_>,
) -> bool {
    // Inside a project a schematic file is added to it, never poured over
    // the cell view the user has open.
    if state.project_lifecycle.is_open() {
        return match import_loaded_schematic_as_cell_view(state, schematic, origin) {
            Ok(reference) => {
                state.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!(
                        "Imported {} as {}.",
                        origin.display_label(),
                        reference.display_path()
                    ),
                );
                true
            }
            Err(error) => {
                state.record(
                    LogSeverity::Error,
                    LogSource::Studio,
                    format!("Schematic import failed: {error}"),
                );
                false
            }
        };
    }
    let schematic = prepare_loaded_schematic(state, schematic, origin);
    replace_active_schematic_document(state, schematic);
    state.clear_design_execution_context();
    if let Some(path) = origin.recent_path() {
        state.remember_recent_file(crate::services::recent_files::RecentKind::Schematic, path);
    }
    state.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!("{} {}.", origin.success_prefix(), origin.display_label()),
    );
    true
}

/// Add a loaded schematic to the open project as a new cell, and open it.
///
/// The cell is named after the file (its stem, with anything but letters,
/// digits and underscores replaced), made unique in the library of the
/// active view, and given one schematic view holding the file's drawing.
/// The active cell view is not touched. The drawing now belongs to the
/// project, so it keeps no file binding and is unsaved until the project is.
pub(crate) fn import_loaded_schematic_as_cell_view(
    state: &mut AppState,
    mut schematic: SchematicState,
    origin: SchematicLoadOrigin<'_>,
) -> Result<crate::state::CellViewRef, String> {
    use crate::state::{Cell, CellViewRef, ProjectLibraryMutation, View};

    let library = state.model.workspace.content.active_view.library.clone();
    let Some(existing) = state.model.library_manager.get_library(&library) else {
        return Err(format!("library `{library}` is not in this project"));
    };
    if existing.read_only {
        return Err(format!("library `{library}` is read only"));
    }
    let name = unique_cell_name(existing, &cell_name_from_file(origin.file_stem()));
    let mut cell = Cell::new(&name);
    cell.add_view(View::new("schematic", ViewType::Schematic));
    let mut candidate_libraries = state.model.library_manager.clone();
    let Some(mut candidate_library) = candidate_libraries.edit_library(&library) else {
        return Err(format!("library `{library}` is not in this project"));
    };
    candidate_library.add_cell(cell);
    let mutation =
        state.preflight_project_library_mutation(ProjectLibraryMutation::CreateCell {
            library: library.clone(),
            cell: name.clone(),
        })?;

    schematic.session.current_file = None;
    schematic.session.is_dirty = true;
    schematic.session.editor.snap_engine = state.ui.schematic_snap.clone();
    schematic.reconcile_grid_pitch_runtime();
    state.model.library_manager = candidate_libraries;
    let reference = CellViewRef::new(&library, &name, "schematic");
    state
        .model
        .workspace
        .insert_schematic_editor(reference.key(), schematic);
    state.publish_project_library_mutation(mutation);
    state.open_workspace_view(reference.clone());
    Ok(reference)
}

/// A cell name from a file stem: letters, digits and underscores kept, every
/// other run of characters one underscore, never empty.
fn cell_name_from_file(stem: &str) -> String {
    let mut name = String::new();
    for character in stem.chars() {
        if character.is_alphanumeric() || character == '_' {
            name.push(character);
        } else if !name.ends_with('_') {
            name.push('_');
        }
    }
    let name = name.trim_matches('_');
    if name.is_empty() {
        "imported_schematic".to_owned()
    } else {
        name.to_owned()
    }
}

/// `base`, or `base_2`, `base_3`, … : the first the library does not hold,
/// compared as cell identities are (ignoring letter case).
fn unique_cell_name(library: &crate::state::Library, base: &str) -> String {
    let taken = |candidate: &str| {
        let identity = crate::state::canonical_cell_view_owner_key(&library.name, candidate, "");
        library.cells.values().any(|cell| {
            crate::state::canonical_cell_view_owner_key(&library.name, &cell.name, "") == identity
        })
    };
    if !taken(base) {
        return base.to_owned();
    }
    (2..)
        .map(|suffix| format!("{base}_{suffix}"))
        .find(|candidate| !taken(candidate))
        .expect("an unbounded suffix range finds a free name")
}

fn prepare_loaded_schematic(
    state: &mut AppState,
    schematic: SchematicState,
    origin: SchematicLoadOrigin<'_>,
) -> SchematicState {
    match origin {
        SchematicLoadOrigin::PersistentPath(_) => {
            state.browser_schematic_save_name = None;
            schematic
        }
        #[cfg(any(test, target_arch = "wasm32"))]
        SchematicLoadOrigin::BrowserImport(name) => {
            let mut schematic = schematic;
            schematic.session.current_file = None;
            state.browser_schematic_save_name = Some(name.to_string());
            schematic
        }
    }
}

pub(crate) fn open_schematic_from_dialog_with_io(
    state: &mut AppState,
    io: &(impl FileWorkflowIo + ?Sized),
) {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = io;
        match start_browser_schematic_import() {
            Ok(()) => state.prompt("Choose an RSpice schematic file to open."),
            Err(error) => {
                state.record(
                    LogSeverity::Error,
                    LogSource::Studio,
                    format!("Schematic open failed: {error}"),
                );
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    match io.show_open_dialog() {
        Ok(path) => {
            let _ = load_schematic_from_path_with_io(state, &path, io);
        }
        #[cfg(not(target_arch = "wasm32"))]
        Err(SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Schematic open failed: {e}"),
            );
        }
    }
}

fn file_name_string(path: &Path) -> Option<String> {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .filter(|name| !name.trim().is_empty())
}

fn schematic_save_dialog_default_name(state: &AppState) -> Option<String> {
    state
        .model
        .schematic
        .session
        .current_file
        .as_deref()
        .and_then(file_name_string)
        .or_else(|| state.browser_schematic_save_name.clone())
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
enum BrowserSchematicImportResult {
    Cancelled,
    Failed(String),
    Loaded(crate::workbench::browser::file_import::PickedTextFile),
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
struct BrowserSchematicImportCompletion {
    token: crate::workbench::browser::file_import::TextImportToken,
    result: BrowserSchematicImportResult,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_SCHEMATIC_IMPORT_RESULT: std::cell::RefCell<Option<BrowserSchematicImportCompletion>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn start_browser_schematic_import() -> Result<(), String> {
    let token = crate::workbench::browser::file_import::try_begin_text_import(
        crate::workbench::browser::file_import::BrowserTextImportKind::Schematic,
    )?;

    crate::workbench::browser::file_import::pick_text_file(
        crate::io::schematic_io::SCHEMATIC_FILTER.0,
        crate::io::schematic_io::SCHEMATIC_FILTER.1,
        move |result| {
            if !crate::workbench::browser::file_import::text_import_is_current(token) {
                return;
            }
            let event = match result {
                Ok(Some(file)) => BrowserSchematicImportResult::Loaded(file),
                Ok(None) => BrowserSchematicImportResult::Cancelled,
                Err(error) => BrowserSchematicImportResult::Failed(error),
            };
            BROWSER_SCHEMATIC_IMPORT_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(BrowserSchematicImportCompletion {
                    token,
                    result: event,
                });
            });
        },
    );
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_schematic_import(state: &mut AppState) -> bool {
    let Some(completion) = BROWSER_SCHEMATIC_IMPORT_RESULT.with(|slot| slot.borrow_mut().take())
    else {
        return false;
    };
    if !crate::workbench::browser::file_import::finish_text_import(completion.token) {
        return false;
    }
    match completion.result {
        BrowserSchematicImportResult::Loaded(file) => {
            match crate::io::schematic_io::load_schematic_text(&file.contents, None) {
                Ok(schematic) => apply_loaded_schematic(
                    state,
                    schematic,
                    SchematicLoadOrigin::BrowserImport(&file.name),
                ),
                Err(error) => {
                    state.record(
                        LogSeverity::Error,
                        LogSource::Studio,
                        format!("Schematic open failed: {error}"),
                    );
                    false
                }
            }
        }
        BrowserSchematicImportResult::Failed(error) => {
            state.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Schematic open failed: {error}"),
            );
            false
        }
        BrowserSchematicImportResult::Cancelled => false,
    }
}

pub(crate) fn save_schematic_to_path_with_io(
    state: &mut AppState,
    path: &Path,
    update_current_file: bool,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    let keep_backup = state
        .ui
        .preferences
        .toggle(crate::workbench::TogglePreference::KeepSaveBackup);
    match io.save_schematic(&state.model.schematic, path, keep_backup) {
        Ok(()) => {
            let saved_path_is_reopenable = io.saved_paths_are_reopenable();
            if saved_path_is_reopenable && update_current_file {
                state.model.schematic.session.current_file = Some(path.to_path_buf());
            }
            if saved_path_is_reopenable {
                state.browser_schematic_save_name = None;
                state.model.schematic.session.is_dirty = false;
                // Retire only the checkpoint whose writer lease and committed
                // identity prove this process owns these exact bytes. Foreign,
                // legacy, or replaced recovery evidence is retained.
                #[cfg(not(target_arch = "wasm32"))]
                {
                    if let Err(error) =
                        crate::workbench::lifecycle::recovery_checkpoint::cleanup_checkpoint(path)
                    {
                        state.record(
                            LogSeverity::Warning,
                            LogSource::Studio,
                            format!(
                                "Saved {}, but its recovery checkpoint could not be removed: \
                                 {error}",
                                path.display()
                            ),
                        );
                    }
                }
            }
            state.sync_active_schematic_to_workspace();
            if saved_path_is_reopenable {
                state.remember_recent_file(
                    crate::services::recent_files::RecentKind::Schematic,
                    path,
                );
                state.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!("Saved schematic {}.", path.display()),
                );
            } else {
                state.browser_schematic_save_name = file_name_string(path);
                state.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!("Downloaded a copy of the schematic: {}.", path.display()),
                );
            }
            true
        }
        Err(e) => {
            state.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Schematic save failed: {e}"),
            );
            false
        }
    }
}

pub(crate) fn save_schematic_with_io(
    state: &mut AppState,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    if let Some(path) = state.model.schematic.session.current_file.clone() {
        save_schematic_to_path_with_io(state, &path, false, io)
    } else {
        save_schematic_as_with_io(state, io)
    }
}

pub(crate) fn save_schematic_as_with_io(
    state: &mut AppState,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    let default_name = schematic_save_dialog_default_name(state);

    match io.show_save_dialog(default_name.as_deref()) {
        Ok(path) => save_schematic_to_path_with_io(state, &path, true, io),
        #[cfg(not(target_arch = "wasm32"))]
        Err(SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
            false
        }
        Err(e) => {
            state.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Schematic Save As failed: {e}"),
            );
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{CellViewRef, View, ViewType};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, Clone)]
    struct TestFileWorkflowIo {
        saved_paths_are_reopenable: bool,
        save_dialog_default_names: Rc<RefCell<Vec<Option<String>>>>,
    }

    impl FileWorkflowIo for TestFileWorkflowIo {
        #[cfg(not(target_arch = "wasm32"))]
        fn show_open_dialog(&self) -> Result<PathBuf, SchematicIoError> {
            Err(SchematicIoError::Cancelled)
        }

        fn show_save_dialog(
            &self,
            default_name: Option<&str>,
        ) -> Result<PathBuf, SchematicIoError> {
            self.save_dialog_default_names
                .borrow_mut()
                .push(default_name.map(str::to_string));
            Ok(PathBuf::from("browser-save.rsch"))
        }

        fn load_schematic(&self, _path: &Path) -> Result<SchematicState, SchematicIoError> {
            #[cfg(not(target_arch = "wasm32"))]
            return Err(SchematicIoError::Cancelled);
            #[cfg(target_arch = "wasm32")]
            Err(SchematicIoError::NotFound(_path.to_owned()))
        }

        fn save_schematic(
            &self,
            _schematic: &SchematicState,
            _path: &Path,
            _keep_backup: bool,
        ) -> Result<(), SchematicIoError> {
            Ok(())
        }

        fn saved_paths_are_reopenable(&self) -> bool {
            self.saved_paths_are_reopenable
        }
    }

    /// The no-project screen, the one place opening a schematic file replaces
    /// the drawing; inside a project the file is imported as a new cell view.
    fn no_project_state() -> AppState {
        let mut state = AppState::default();
        state.close_project_session();
        state
    }

    #[test]
    fn replacing_the_active_document_reconciles_document_pitch_and_session_snap_preferences() {
        let mut state = AppState::default();
        state.ui.schematic_snap.snap_radius = 8;
        state.ui.schematic_snap.snap_to_grid = false;
        state.ui.schematic_snap.snap_to_wire_segments = false;
        state.ui.schematic_snap.grid_size = 999;
        let mut loaded = SchematicState::default();
        loaded.document_mut_for_test().document_policy.grid_pitch =
            crate::state::SchematicGridPitch::Metric;
        loaded.document_mut_for_test().grid_size = 123;
        loaded.session.editor.snap_engine = crate::state::SnapEngine::default();

        replace_active_schematic_document(&mut state, loaded);

        let expected = crate::state::SchematicGridPitch::Metric.canvas_grid_size();
        assert_eq!(
            state.model.schematic.document().document_policy.grid_pitch,
            crate::state::SchematicGridPitch::Metric
        );
        assert_eq!(state.model.schematic.document().grid_size, expected);
        assert_eq!(
            state.model.schematic.session.editor.snap_engine.grid_size,
            expected
        );
        assert_eq!(
            state.model.schematic.session.editor.snap_engine.snap_radius,
            8
        );
        assert!(
            !state
                .model
                .schematic
                .session
                .editor
                .snap_engine
                .snap_to_grid
        );
        assert!(
            !state
                .model
                .schematic
                .session
                .editor
                .snap_engine
                .snap_to_wire_segments
        );
        assert_eq!(
            state.ui.schematic_snap,
            state.model.schematic.session.editor.snap_engine
        );
    }

    #[test]
    fn browser_import_applies_schematic_clears_runs_and_skips_recents() {
        let mut state = no_project_state();
        state.model.simulation.start_run();
        assert!(state.model.simulation.retained.has_results());

        let schematic = SchematicState::default();

        let imported = apply_loaded_schematic(
            &mut state,
            schematic,
            SchematicLoadOrigin::BrowserImport("browser-filter.rsch"),
        );

        assert!(imported);
        assert!(state.model.schematic.session.current_file.is_none());
        assert!(!state.model.simulation.retained.has_results());
        assert!(state.recent_files.is_empty());
        assert!(state.model.log_buffer.entries().any(|entry| {
            entry
                .message
                .contains("Imported schematic browser-filter.rsch.")
        }));
    }

    #[test]
    fn browser_import_keeps_filename_as_save_suggestion_without_native_path() {
        let mut state = no_project_state();
        let mut schematic = SchematicState::default();
        schematic.session.current_file = Some(PathBuf::from("stale-native-path.rsch"));

        let imported = apply_loaded_schematic(
            &mut state,
            schematic,
            SchematicLoadOrigin::BrowserImport("browser-filter.rsch"),
        );

        assert!(imported);
        assert!(state.model.schematic.session.current_file.is_none());
        assert_eq!(
            state.browser_schematic_save_name.as_deref(),
            Some("browser-filter.rsch")
        );
        assert!(state.recent_files.is_empty());
    }

    #[test]
    fn browser_import_filename_is_used_for_next_schematic_save_as_dialog() {
        let mut state = no_project_state();
        let defaults = Rc::new(RefCell::new(Vec::new()));
        let io = TestFileWorkflowIo {
            saved_paths_are_reopenable: false,
            save_dialog_default_names: Rc::clone(&defaults),
        };

        let imported = apply_loaded_schematic(
            &mut state,
            SchematicState::default(),
            SchematicLoadOrigin::BrowserImport("browser-filter.rsch"),
        );
        assert!(imported);

        let saved = save_schematic_as_with_io(&mut state, &io);

        assert!(saved);
        assert_eq!(
            defaults.borrow().as_slice(),
            &[Some("browser-filter.rsch".to_string())]
        );
        assert!(state.model.schematic.session.current_file.is_none());
        assert_eq!(
            state.browser_schematic_save_name.as_deref(),
            Some("browser-save.rsch")
        );
        assert!(state.recent_files.is_empty());
    }

    fn seed_stale_design_execution_context(state: &mut AppState) {
        state.model.workspace.content.netlist_source = Some("old manual deck\n.end\n".to_owned());
        state.model.simulation.source.netlist_content = "old generated deck\n.end\n".to_owned();
        state
            .model
            .simulation
            .retained
            .runs
            .push(crate::state::SimulationRun::new(1));
        state.model.simulation.view.active_run_idx = Some(0);
        state.manual_run.last_run_buffer = Some("last manual run\n.end\n".to_owned());
        state.manual_run.pending_run_buffer = Some("pending manual run\n.end\n".to_owned());
        state.manual_run.pending_manual_run_id = Some(1);
        state.ui.netlist.rerun_queued = true;
        state.manual_run.edited_lines.insert(3);
        state.analysis.histogram_state.selected = Some("old measurement".to_owned());
    }

    fn assert_design_execution_context_cleared(state: &AppState) {
        assert!(state.model.workspace.content.netlist_source.is_none());
        assert!(state.model.simulation.source.netlist_content.is_empty());
        assert!(!state.model.simulation.retained.has_results());
        assert!(state.manual_run.last_run_buffer.is_none());
        assert!(state.manual_run.pending_run_buffer.is_none());
        assert!(state.manual_run.pending_manual_run_id.is_none());
        assert!(!state.ui.netlist.rerun_queued);
        assert!(state.manual_run.edited_lines.is_empty());
        assert_eq!(state.analysis.histogram_state.selected, None);
    }

    fn open_default_symbol_view(state: &mut AppState) -> CellViewRef {
        let schematic_reference = state.model.workspace.content.active_schematic_reference();
        let symbol_reference = CellViewRef::new(
            &schematic_reference.library,
            &schematic_reference.cell,
            "symbol",
        );
        let library = state
            .model
            .library_manager
            .get_library_mut(&symbol_reference.library)
            .expect("default project library exists");
        let cell = library
            .get_cell_mut(&symbol_reference.cell)
            .expect("default top cell exists");
        if cell.get_view("symbol").is_none() {
            cell.add_view(View::new("symbol", ViewType::Symbol));
        }
        state.open_workspace_view(symbol_reference);
        assert_eq!(
            state.model.workspace.content.active_view_type(),
            ViewType::Symbol
        );
        schematic_reference
    }

    fn install_stale_paired_schematic(state: &mut AppState, reference: &CellViewRef) {
        let mut stale = SchematicState::default();
        stale.session.current_file = Some(PathBuf::from("stale-symbol-context.rsch"));
        stale.session.is_dirty = true;
        state.model.schematic = stale.clone();
        state
            .model
            .workspace
            .insert_schematic_editor(reference.key(), stale);
    }

    #[test]
    fn create_new_schematic_resets_stale_netlist_and_result_context() {
        let mut state = AppState::default();
        seed_stale_design_execution_context(&mut state);

        create_new_schematic(&mut state);

        assert_design_execution_context_cleared(&state);
    }

    #[test]
    fn create_new_schematic_from_symbol_view_replaces_paired_schematic_buffer() {
        let mut state = AppState::default();
        let schematic_reference = open_default_symbol_view(&mut state);
        let symbol_key = state.model.workspace.content.active_key();
        install_stale_paired_schematic(&mut state, &schematic_reference);

        create_new_schematic(&mut state);

        let buffer = state
            .model
            .workspace
            .session
            .schematic_sessions
            .get(&schematic_reference.key())
            .expect("paired schematic buffer exists");
        assert!(state.model.schematic.session.current_file.is_none());
        assert!(buffer.current_file.is_none());
        assert!(!buffer.is_dirty);
        assert!(
            !state
                .model
                .workspace
                .content
                .schematic_buffers
                .contains_key(&symbol_key),
            "symbol view must not gain a schematic buffer"
        );
    }

    #[test]
    fn apply_loaded_schematic_resets_stale_netlist_and_result_context() {
        let mut state = no_project_state();
        seed_stale_design_execution_context(&mut state);

        let mut schematic = SchematicState::default();
        schematic.session.current_file = Some(PathBuf::from("fresh.rsch"));
        let loaded = apply_loaded_schematic(
            &mut state,
            schematic,
            SchematicLoadOrigin::PersistentPath(Path::new("fresh.rsch")),
        );

        assert!(loaded);
        assert_design_execution_context_cleared(&state);
        assert_eq!(
            state.model.schematic.session.current_file.as_deref(),
            Some(Path::new("fresh.rsch"))
        );
    }

    /// Inside a project a schematic file becomes a new cell view named after
    /// the file, and the cell view that was open keeps its drawing.
    #[test]
    fn opening_a_schematic_inside_a_project_imports_it_as_a_new_cell_view() {
        use crate::state::{ComponentType, Point};

        let mut state = AppState::default();
        state.open_project_session();
        let active = state.model.workspace.content.active_view.clone();
        state
            .model
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        state.sync_active_schematic_to_workspace();
        let mut file = SchematicState::default();
        file.add_component(ComponentType::Capacitor, Point::origin());
        file.add_component(ComponentType::Capacitor, Point::new(40, 0));
        let path = Path::new("designs/RC low-pass.rsch");

        assert!(apply_loaded_schematic(
            &mut state,
            file.clone(),
            SchematicLoadOrigin::PersistentPath(path),
        ));
        let imported = CellViewRef::new(&active.library, "RC_low_pass", "schematic");
        assert_eq!(state.model.workspace.content.active_view, imported);
        assert_eq!(state.model.schematic.document().components.len(), 2);
        assert_eq!(state.model.schematic.session.current_file, None);
        let original = state
            .model
            .workspace
            .clone_schematic_editor(&active.key())
            .expect("the open cell view keeps its buffer");
        assert_eq!(
            original.document().components.len(),
            1,
            "nothing was replaced"
        );

        assert!(apply_loaded_schematic(
            &mut state,
            file,
            SchematicLoadOrigin::PersistentPath(path),
        ));
        assert_eq!(
            state.model.workspace.content.active_view.cell,
            "RC_low_pass_2"
        );
    }

    #[test]
    fn load_schematic_from_symbol_view_replaces_paired_schematic_buffer() {
        let mut state = no_project_state();
        let schematic_reference = open_default_symbol_view(&mut state);
        let symbol_key = state.model.workspace.content.active_key();
        install_stale_paired_schematic(&mut state, &schematic_reference);

        let mut schematic = SchematicState::default();
        schematic.session.current_file = Some(PathBuf::from("fresh-symbol-context.rsch"));
        let loaded = apply_loaded_schematic(
            &mut state,
            schematic,
            SchematicLoadOrigin::PersistentPath(Path::new("fresh-symbol-context.rsch")),
        );

        assert!(loaded);
        assert_eq!(
            state.model.schematic.session.current_file.as_deref(),
            Some(Path::new("fresh-symbol-context.rsch"))
        );
        let buffer = state
            .model
            .workspace
            .session
            .schematic_sessions
            .get(&schematic_reference.key())
            .expect("paired schematic buffer exists");
        assert_eq!(
            buffer.current_file.as_deref(),
            Some(Path::new("fresh-symbol-context.rsch"))
        );
        assert!(
            !state
                .model
                .workspace
                .content
                .schematic_buffers
                .contains_key(&symbol_key),
            "symbol view must not gain a schematic buffer"
        );
    }

    #[test]
    fn download_only_schematic_save_keeps_document_dirty_without_recent_entry() {
        let mut state = AppState::default();
        state.model.schematic.session.is_dirty = true;

        let io = TestFileWorkflowIo {
            saved_paths_are_reopenable: false,
            save_dialog_default_names: Rc::new(RefCell::new(Vec::new())),
        };
        let saved =
            save_schematic_to_path_with_io(&mut state, Path::new("browser-save.rsch"), true, &io);

        assert!(saved);
        assert!(state.model.schematic.session.current_file.is_none());
        assert_eq!(
            state.browser_schematic_save_name.as_deref(),
            Some("browser-save.rsch")
        );
        assert!(state.model.schematic.session.is_dirty);
        assert!(state.model.workspace.any_dirty());
        assert!(state.recent_files.is_empty());
    }

    /// Projects and files ▸ Keep a backup when saving: a second save copies
    /// the first file to `.rsch.bak` only while the setting is on.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_schematic_save_keeps_the_previous_file_only_when_asked() {
        let folder = std::env::temp_dir().join(format!(
            "rspice-save-backup-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system time after epoch")
                .as_nanos()
        ));
        std::fs::create_dir_all(&folder).expect("create the test folder");
        for keep in [true, false] {
            let path = folder.join(format!("design-{keep}.rsch"));
            let mut state = AppState::default();
            state
                .ui
                .preferences
                .set_toggle(crate::workbench::TogglePreference::KeepSaveBackup, keep);
            for _ in 0..2 {
                assert!(save_schematic_to_path_with_io(
                    &mut state,
                    &path,
                    true,
                    &NativeFileWorkflowIo
                ));
            }
            assert_eq!(path.with_extension("rsch.bak").exists(), keep);
        }
        let _ = std::fs::remove_dir_all(&folder);
    }
}
