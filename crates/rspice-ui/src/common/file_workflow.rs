use std::path::{Path, PathBuf};

use crate::common::app::{AppState, ConsoleMessage};
use crate::io::SchematicIoError;
use crate::state::{SchematicState, ViewType};

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

    fn recent_path(self) -> Option<&'a Path> {
        match self {
            Self::PersistentPath(path) => Some(path),
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(_) => None,
        }
    }

    fn success_prefix(self) -> &'static str {
        match self {
            Self::PersistentPath(_) => "Opened",
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
    fn save_schematic(
        &self,
        schematic: &SchematicState,
        path: &Path,
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
    ) -> Result<(), SchematicIoError> {
        crate::io::save_schematic(schematic, path)
    }

    fn saved_paths_are_reopenable(&self) -> bool {
        !cfg!(target_arch = "wasm32")
    }
}

/// Reset the current schematic to a new empty document.
pub(crate) fn create_new_schematic(state: &mut AppState) {
    replace_active_schematic_document(state, crate::state::SchematicState::default());
    state.clear_design_execution_context();
    state.push_user_message(ConsoleMessage::info("Created new schematic"));
}

fn replace_active_schematic_document(state: &mut AppState, schematic: SchematicState) {
    state.schematic = schematic;
    match state.workspace.active_view_type() {
        ViewType::Schematic | ViewType::Testbench => {
            state.sync_active_schematic_to_workspace();
        }
        _ => {
            let reference = state.workspace.active_schematic_reference();
            let dirty = state.schematic.is_dirty;
            state
                .workspace
                .schematic_buffers
                .insert(reference.key(), state.schematic.clone());
            if let Some(open_view) = state
                .workspace
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

/// Whether a newer autosave checkpoint shadows this file (native only —
/// the web build has no checkpoint files).
#[cfg(not(target_arch = "wasm32"))]
fn newer_checkpoint(path: &Path) -> Option<PathBuf> {
    let checkpoint = autosave_checkpoint_path(path);
    let checkpoint_modified = std::fs::metadata(&checkpoint)
        .and_then(|meta| meta.modified())
        .ok()?;
    let file_modified = std::fs::metadata(path)
        .and_then(|meta| meta.modified())
        .ok()?;
    (checkpoint_modified > file_modified).then_some(checkpoint)
}

pub(crate) fn load_schematic_from_path_with_io(
    state: &mut AppState,
    path: &Path,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    // A newer checkpoint means the last session ended without a save —
    // ask before silently opening the older file (volta-app-dialogs §02).
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(checkpoint) = newer_checkpoint(path) {
        state.dialogs.pending_autosave_restore = Some((path.to_path_buf(), checkpoint));
        return true;
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
            state.push_user_message(ConsoleMessage::error(format!("Failed to open: {}", e)));
            false
        }
    }
}

pub(crate) fn apply_loaded_schematic(
    state: &mut AppState,
    schematic: SchematicState,
    origin: SchematicLoadOrigin<'_>,
) -> bool {
    let schematic = prepare_loaded_schematic(state, schematic, origin);
    replace_active_schematic_document(state, schematic);
    state.clear_design_execution_context();
    if let Some(path) = origin.recent_path() {
        state.remember_recent_file(crate::common::app::RecentKind::Schematic, path);
    }
    state.push_user_message(ConsoleMessage::info(format!(
        "{}: {}",
        origin.success_prefix(),
        origin.display_label()
    )));
    true
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
            schematic.current_file = None;
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
            Ok(()) => {
                state.push_user_message(ConsoleMessage::info(
                    "Choose an RSpice schematic file to open",
                ));
            }
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!("Open failed: {}", error)));
            }
        }
        return;
    }

    #[cfg(not(target_arch = "wasm32"))]
    match io.show_open_dialog() {
        Ok(path) => {
            let _ = load_schematic_from_path_with_io(state, &path, io);
        }
        Err(SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state.push_user_message(ConsoleMessage::error(format!("Open failed: {}", e)));
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
        .schematic
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
    Loaded(crate::common::browser_file_import::PickedTextFile),
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_SCHEMATIC_IMPORT_RESULT: std::cell::RefCell<Option<BrowserSchematicImportResult>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn start_browser_schematic_import() -> Result<(), String> {
    crate::common::browser_file_import::try_begin_text_import(
        crate::common::browser_file_import::BrowserTextImportKind::Schematic,
    )?;

    crate::common::browser_file_import::pick_text_file(
        crate::io::schematic_io::SCHEMATIC_FILTER.0,
        crate::io::schematic_io::SCHEMATIC_FILTER.1,
        |result| {
            let event = match result {
                Ok(Some(file)) => BrowserSchematicImportResult::Loaded(file),
                Ok(None) => BrowserSchematicImportResult::Cancelled,
                Err(error) => BrowserSchematicImportResult::Failed(error),
            };
            BROWSER_SCHEMATIC_IMPORT_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(event);
            });
        },
    );
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_schematic_import(state: &mut AppState) -> bool {
    let event = BROWSER_SCHEMATIC_IMPORT_RESULT.with(|slot| slot.borrow_mut().take());
    if event.is_some() {
        crate::common::browser_file_import::finish_text_import(
            crate::common::browser_file_import::BrowserTextImportKind::Schematic,
        );
    }
    match event {
        Some(BrowserSchematicImportResult::Loaded(file)) => {
            match crate::io::schematic_io::load_schematic_text(&file.contents, None) {
                Ok(schematic) => apply_loaded_schematic(
                    state,
                    schematic,
                    SchematicLoadOrigin::BrowserImport(&file.name),
                ),
                Err(error) => {
                    state.push_user_message(ConsoleMessage::error(format!(
                        "Open failed: {}",
                        error
                    )));
                    false
                }
            }
        }
        Some(BrowserSchematicImportResult::Failed(error)) => {
            state.push_user_message(ConsoleMessage::error(format!("Open failed: {}", error)));
            false
        }
        Some(BrowserSchematicImportResult::Cancelled) | None => false,
    }
}

pub(crate) fn save_schematic_to_path_with_io(
    state: &mut AppState,
    path: &Path,
    update_current_file: bool,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    match io.save_schematic(&state.schematic, path) {
        Ok(()) => {
            let saved_path_is_reopenable = io.saved_paths_are_reopenable();
            if saved_path_is_reopenable && update_current_file {
                state.schematic.current_file = Some(path.to_path_buf());
            }
            if saved_path_is_reopenable {
                state.browser_schematic_save_name = None;
                state.schematic.is_dirty = false;
                // A clean save is the truth again — a leftover checkpoint
                // would only shadow it with something older next open.
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = std::fs::remove_file(autosave_checkpoint_path(path));
                }
            }
            state.sync_active_schematic_to_workspace();
            if saved_path_is_reopenable {
                state.remember_recent_file(crate::common::app::RecentKind::Schematic, path);
                state.push_user_message(ConsoleMessage::info(format!("Saved: {}", path.display())));
            } else {
                state.browser_schematic_save_name = file_name_string(path);
                state.push_user_message(ConsoleMessage::info(format!(
                    "Downloaded schematic copy: {}",
                    path.display()
                )));
            }
            true
        }
        Err(e) => {
            state.push_user_message(ConsoleMessage::error(format!("Save failed: {}", e)));
            false
        }
    }
}

pub(crate) fn save_schematic_with_io(
    state: &mut AppState,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    if let Some(path) = state.schematic.current_file.clone() {
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
        Err(SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
            false
        }
        Err(e) => {
            state.push_user_message(ConsoleMessage::error(format!("Save As failed: {}", e)));
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::histogram::HistogramBuilder;
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
            Err(SchematicIoError::Cancelled)
        }

        fn save_schematic(
            &self,
            _schematic: &SchematicState,
            _path: &Path,
        ) -> Result<(), SchematicIoError> {
            Ok(())
        }

        fn saved_paths_are_reopenable(&self) -> bool {
            self.saved_paths_are_reopenable
        }
    }

    #[test]
    fn browser_import_applies_schematic_clears_runs_and_skips_recents() {
        let mut state = AppState::default();
        state.simulation.start_run();
        assert!(state.simulation.has_results());

        let schematic = SchematicState::default();

        let imported = apply_loaded_schematic(
            &mut state,
            schematic,
            SchematicLoadOrigin::BrowserImport("browser-filter.rsch"),
        );

        assert!(imported);
        assert!(state.schematic.current_file.is_none());
        assert!(!state.simulation.has_results());
        assert!(state.recent_files.is_empty());
        assert!(state.log_buffer.entries().any(|entry| {
            entry
                .message
                .contains("Imported schematic: browser-filter.rsch")
        }));
    }

    #[test]
    fn browser_import_keeps_filename_as_save_suggestion_without_native_path() {
        let mut state = AppState::default();
        let mut schematic = SchematicState::default();
        schematic.current_file = Some(PathBuf::from("stale-native-path.rsch"));

        let imported = apply_loaded_schematic(
            &mut state,
            schematic,
            SchematicLoadOrigin::BrowserImport("browser-filter.rsch"),
        );

        assert!(imported);
        assert!(state.schematic.current_file.is_none());
        assert_eq!(
            state.browser_schematic_save_name.as_deref(),
            Some("browser-filter.rsch")
        );
        assert!(state.recent_files.is_empty());
    }

    #[test]
    fn browser_import_filename_is_used_for_next_schematic_save_as_dialog() {
        let mut state = AppState::default();
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
        assert!(state.schematic.current_file.is_none());
        assert_eq!(
            state.browser_schematic_save_name.as_deref(),
            Some("browser-save.rsch")
        );
        assert!(state.recent_files.is_empty());
    }

    fn seed_stale_design_execution_context(state: &mut AppState) {
        state.workspace.netlist_source = Some("old manual deck\n.end\n".to_owned());
        state.simulation.netlist_content = "old generated deck\n.end\n".to_owned();
        state
            .simulation
            .runs
            .push(crate::state::SimulationRun::new(1));
        state.simulation.active_run_idx = Some(0);
        state.ui.netlist.last_run_buffer = Some("last manual run\n.end\n".to_owned());
        state.ui.netlist.pending_run_buffer = Some("pending manual run\n.end\n".to_owned());
        state.ui.netlist.pending_manual_run_id = Some(1);
        state.ui.netlist.rerun_queued = true;
        state.ui.netlist.edited_lines.insert(3);
        let histogram = HistogramBuilder::new()
            .name("old monte carlo")
            .bin_count(4)
            .build(&[1.0, 2.0, 3.0]);
        state.analysis.histogram_state.load_histogram(histogram);
    }

    fn assert_design_execution_context_cleared(state: &AppState) {
        assert!(state.workspace.netlist_source.is_none());
        assert!(state.simulation.netlist_content.is_empty());
        assert!(!state.simulation.has_results());
        assert!(state.ui.netlist.last_run_buffer.is_none());
        assert!(state.ui.netlist.pending_run_buffer.is_none());
        assert!(state.ui.netlist.pending_manual_run_id.is_none());
        assert!(!state.ui.netlist.rerun_queued);
        assert!(state.ui.netlist.edited_lines.is_empty());
        assert!(state.analysis.histogram_state.is_empty());
    }

    fn open_default_symbol_view(state: &mut AppState) -> CellViewRef {
        let schematic_reference = state.workspace.active_schematic_reference();
        let symbol_reference = CellViewRef::new(
            &schematic_reference.library,
            &schematic_reference.cell,
            "symbol",
        );
        let library = state
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
        assert_eq!(state.workspace.active_view_type(), ViewType::Symbol);
        schematic_reference
    }

    fn install_stale_paired_schematic(state: &mut AppState, reference: &CellViewRef) {
        let mut stale = SchematicState::default();
        stale.current_file = Some(PathBuf::from("stale-symbol-context.rsch"));
        stale.is_dirty = true;
        state.schematic = stale.clone();
        state
            .workspace
            .schematic_buffers
            .insert(reference.key(), stale);
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
        let symbol_key = state.workspace.active_key();
        install_stale_paired_schematic(&mut state, &schematic_reference);

        create_new_schematic(&mut state);

        let buffer = state
            .workspace
            .schematic_buffers
            .get(&schematic_reference.key())
            .expect("paired schematic buffer exists");
        assert!(state.schematic.current_file.is_none());
        assert!(buffer.current_file.is_none());
        assert!(!buffer.is_dirty);
        assert!(
            !state.workspace.schematic_buffers.contains_key(&symbol_key),
            "symbol view must not gain a schematic buffer"
        );
    }

    #[test]
    fn apply_loaded_schematic_resets_stale_netlist_and_result_context() {
        let mut state = AppState::default();
        seed_stale_design_execution_context(&mut state);

        let mut schematic = SchematicState::default();
        schematic.current_file = Some(PathBuf::from("fresh.rsch"));
        let loaded = apply_loaded_schematic(
            &mut state,
            schematic,
            SchematicLoadOrigin::PersistentPath(Path::new("fresh.rsch")),
        );

        assert!(loaded);
        assert_design_execution_context_cleared(&state);
        assert_eq!(
            state.schematic.current_file.as_deref(),
            Some(Path::new("fresh.rsch"))
        );
    }

    #[test]
    fn load_schematic_from_symbol_view_replaces_paired_schematic_buffer() {
        let mut state = AppState::default();
        let schematic_reference = open_default_symbol_view(&mut state);
        let symbol_key = state.workspace.active_key();
        install_stale_paired_schematic(&mut state, &schematic_reference);

        let mut schematic = SchematicState::default();
        schematic.current_file = Some(PathBuf::from("fresh-symbol-context.rsch"));
        let loaded = apply_loaded_schematic(
            &mut state,
            schematic,
            SchematicLoadOrigin::PersistentPath(Path::new("fresh-symbol-context.rsch")),
        );

        assert!(loaded);
        assert_eq!(
            state.schematic.current_file.as_deref(),
            Some(Path::new("fresh-symbol-context.rsch"))
        );
        let buffer = state
            .workspace
            .schematic_buffers
            .get(&schematic_reference.key())
            .expect("paired schematic buffer exists");
        assert_eq!(
            buffer.current_file.as_deref(),
            Some(Path::new("fresh-symbol-context.rsch"))
        );
        assert!(
            !state.workspace.schematic_buffers.contains_key(&symbol_key),
            "symbol view must not gain a schematic buffer"
        );
    }

    #[test]
    fn download_only_schematic_save_keeps_document_dirty_without_recent_entry() {
        let mut state = AppState::default();
        state.schematic.is_dirty = true;

        let io = TestFileWorkflowIo {
            saved_paths_are_reopenable: false,
            save_dialog_default_names: Rc::new(RefCell::new(Vec::new())),
        };
        let saved =
            save_schematic_to_path_with_io(&mut state, Path::new("browser-save.rsch"), true, &io);

        assert!(saved);
        assert!(state.schematic.current_file.is_none());
        assert_eq!(
            state.browser_schematic_save_name.as_deref(),
            Some("browser-save.rsch")
        );
        assert!(state.schematic.is_dirty);
        assert!(state.workspace.any_dirty());
        assert!(state.recent_files.is_empty());
    }
}
