//! The schematic file workflow carried out on the whole state: creating,
//! importing, opening and saving a schematic, and what each clears or
//! keeps.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::io::{FileWorkflowIo, NativeFileWorkflowIo, SchematicIoError};
use crate::state::{CellViewRef, SchematicState, View, ViewType};
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::file_workflow::SchematicLoadOrigin;

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

    fn show_save_dialog(&self, default_name: Option<&str>) -> Result<PathBuf, SchematicIoError> {
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
fn browser_import_applies_schematic_clears_runs_and_skips_recents() {
    let mut state = no_project_state();
    state.model.simulation.start_run();
    assert!(state.model.simulation.retained.has_results());

    let schematic = SchematicState::default();

    let imported = state.apply_loaded_schematic(
        schematic,
        SchematicLoadOrigin::BrowserImport("browser-filter.rsch"),
    );

    assert!(imported);
    assert!(state.model.schematic.session.current_file.is_none());
    assert!(!state.model.simulation.retained.has_results());
    assert!(state.platform.recent_files.is_empty());
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

    let imported = state.apply_loaded_schematic(
        schematic,
        SchematicLoadOrigin::BrowserImport("browser-filter.rsch"),
    );

    assert!(imported);
    assert!(state.model.schematic.session.current_file.is_none());
    assert_eq!(
        state.platform.browser_schematic_save_name.as_deref(),
        Some("browser-filter.rsch")
    );
    assert!(state.platform.recent_files.is_empty());
}

#[test]
fn browser_import_filename_is_used_for_next_schematic_save_as_dialog() {
    let mut state = no_project_state();
    let defaults = Rc::new(RefCell::new(Vec::new()));
    let io = TestFileWorkflowIo {
        saved_paths_are_reopenable: false,
        save_dialog_default_names: Rc::clone(&defaults),
    };

    let imported = state.apply_loaded_schematic(
        SchematicState::default(),
        SchematicLoadOrigin::BrowserImport("browser-filter.rsch"),
    );
    assert!(imported);

    let saved = state.save_schematic_as_with_io(&io);

    assert!(saved);
    assert_eq!(
        defaults.borrow().as_slice(),
        &[Some("browser-filter.rsch".to_string())]
    );
    assert!(state.model.schematic.session.current_file.is_none());
    assert_eq!(
        state.platform.browser_schematic_save_name.as_deref(),
        Some("browser-save.rsch")
    );
    assert!(state.platform.recent_files.is_empty());
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
    state.runs.manual_run.last_run_buffer = Some("last manual run\n.end\n".to_owned());
    state.runs.manual_run.pending_run_buffer = Some("pending manual run\n.end\n".to_owned());
    state.runs.manual_run.pending_manual_run_id = Some(1);
    state.runs.manual_run.rerun_queued = true;
    state.runs.manual_run.edited_lines.insert(3);
    state.results.analysis.histogram_state.selected = Some("old measurement".to_owned());
}

fn assert_design_execution_context_cleared(state: &AppState) {
    assert!(state.model.workspace.content.netlist_source.is_none());
    assert!(state.model.simulation.source.netlist_content.is_empty());
    assert!(!state.model.simulation.retained.has_results());
    assert!(state.runs.manual_run.last_run_buffer.is_none());
    assert!(state.runs.manual_run.pending_run_buffer.is_none());
    assert!(state.runs.manual_run.pending_manual_run_id.is_none());
    assert!(!state.runs.manual_run.rerun_queued);
    assert!(state.runs.manual_run.edited_lines.is_empty());
    assert_eq!(state.results.analysis.histogram_state.selected, None);
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

    state.create_new_schematic();

    assert_design_execution_context_cleared(&state);
}

#[test]
fn create_new_schematic_from_symbol_view_replaces_paired_schematic_buffer() {
    let mut state = AppState::default();
    let schematic_reference = open_default_symbol_view(&mut state);
    let symbol_key = state.model.workspace.content.active_key();
    install_stale_paired_schematic(&mut state, &schematic_reference);

    state.create_new_schematic();

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
    let loaded = state.apply_loaded_schematic(
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

    assert!(state.apply_loaded_schematic(file.clone(), SchematicLoadOrigin::PersistentPath(path),));
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

    assert!(state.apply_loaded_schematic(file, SchematicLoadOrigin::PersistentPath(path),));
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
    let loaded = state.apply_loaded_schematic(
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
    let saved = state.save_schematic_to_path_with_io(Path::new("browser-save.rsch"), true, &io);

    assert!(saved);
    assert!(state.model.schematic.session.current_file.is_none());
    assert_eq!(
        state.platform.browser_schematic_save_name.as_deref(),
        Some("browser-save.rsch")
    );
    assert!(state.model.schematic.session.is_dirty);
    assert!(state.model.workspace.any_dirty());
    assert!(state.platform.recent_files.is_empty());
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
            .model
            .preferences
            .set_toggle(crate::workbench::TogglePreference::KeepSaveBackup, keep);
        for _ in 0..2 {
            assert!(state.save_schematic_to_path_with_io(&path, true, &NativeFileWorkflowIo));
        }
        assert_eq!(path.with_extension("rsch.bak").exists(), keep);
    }
    let _ = std::fs::remove_dir_all(&folder);
}
