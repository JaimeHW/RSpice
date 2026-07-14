use std::path::Path;

use crate::common::app::{AppState, ConsoleMessage};
use crate::io::{ProjectExecutionContext, ProjectFile, ProjectIoError, ProjectSimulationResults};

#[derive(Debug, Clone, Copy)]
pub(crate) enum ProjectLoadOrigin<'a> {
    PersistentPath(&'a Path),
    #[cfg(any(test, target_arch = "wasm32"))]
    BrowserImport(&'a str),
}

impl<'a> ProjectLoadOrigin<'a> {
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
            Self::PersistentPath(_) => "Opened project",
            #[cfg(any(test, target_arch = "wasm32"))]
            Self::BrowserImport(_) => "Imported project",
        }
    }
}

pub(crate) fn create_new_project(state: &mut AppState) {
    let mut library_manager = crate::state::LibraryManager::with_primitives();
    let mut workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut library_manager);
    let schematic = workspace
        .active_schematic()
        .cloned()
        .unwrap_or_else(crate::state::SchematicState::default);
    workspace.save_active_schematic(&schematic);

    state.library_manager = library_manager;
    state.workspace = workspace;
    state.schematic = schematic;
    state.clear_design_execution_context();
    state.sim_setup = crate::common::app::SimSetupState::new();
    state.model_library_manager = crate::common::app::default_model_library_manager();
    state.push_user_message(ConsoleMessage::info("Created new project"));
}

pub(crate) fn save_project_to_path(state: &mut AppState, path: &Path) -> bool {
    state.sync_active_schematic_to_workspace();
    let saved_path_is_reopenable = saved_project_paths_are_reopenable();
    let mut workspace = state.workspace.clone();
    if saved_path_is_reopenable {
        workspace.project.set_path(path.to_path_buf());
    }
    let mut result_warning = None;
    let mut simulation_results = ProjectSimulationResults::from_state(&state.simulation);
    if let Err(error) = simulation_results.validate() {
        simulation_results = ProjectSimulationResults::default();
        result_warning = Some(format!(
            "Saved project without simulation results because result history is invalid: {error}"
        ));
    }

    let execution_context =
        match ProjectExecutionContext::from_state(&state.sim_setup, &state.model_library_manager) {
            Ok(context) => context,
            Err(error) => {
                state.push_user_message(ConsoleMessage::error(format!(
                    "Project save failed: execution inputs are invalid: {error}"
                )));
                return false;
            }
        };

    let file = ProjectFile::new_with_execution_context(
        workspace.clone(),
        state.library_manager.clone(),
        simulation_results,
        execution_context,
    );
    match crate::io::save_project_file(&file, path) {
        Ok(()) => {
            finish_successful_project_save(state, workspace, path, saved_path_is_reopenable);
            if let Some(warning) = result_warning {
                state.push_user_message(ConsoleMessage::warning(warning));
            }
            true
        }
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Project save failed: {}",
                error
            )));
            false
        }
    }
}

fn saved_project_paths_are_reopenable() -> bool {
    !cfg!(target_arch = "wasm32")
}

fn file_name_string(path: &Path) -> Option<String> {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .filter(|name| !name.trim().is_empty())
}

fn project_save_dialog_default_name(state: &AppState) -> String {
    state
        .workspace
        .project
        .path
        .as_deref()
        .and_then(file_name_string)
        .or_else(|| state.browser_project_save_name.clone())
        .unwrap_or_else(|| "untitled.rspiceproj".to_string())
}

fn finish_successful_project_save(
    state: &mut AppState,
    workspace: crate::state::ProjectWorkspace,
    path: &Path,
    saved_path_is_reopenable: bool,
) {
    state.workspace = workspace;
    if saved_path_is_reopenable {
        state.browser_project_save_name = None;
        state.workspace.mark_all_clean();
        state.schematic.is_dirty = false;
        state.remember_recent_file(crate::common::app::RecentKind::Project, path);
        state.push_user_message(ConsoleMessage::info(format!(
            "Saved project: {}",
            path.display()
        )));
    } else {
        state.workspace.project.path = None;
        state.browser_project_save_name = file_name_string(path);
        state.push_user_message(ConsoleMessage::info(format!(
            "Downloaded project copy: {}",
            path.display()
        )));
    }
}

pub(crate) fn save_project(state: &mut AppState) -> bool {
    if let Some(path) = state.workspace.project.path.clone() {
        save_project_to_path(state, &path)
    } else {
        save_project_as(state)
    }
}

pub(crate) fn save_project_as(state: &mut AppState) -> bool {
    let default_name = project_save_dialog_default_name(state);

    match crate::io::show_save_project_dialog(Some(default_name.as_str())) {
        Ok(path) => save_project_to_path(state, &path),
        Err(ProjectIoError::Cancelled) => false,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Project Save As failed: {}",
                error
            )));
            false
        }
    }
}

pub(crate) fn load_project_from_path(state: &mut AppState, path: &Path) -> bool {
    match crate::io::load_project_file(path) {
        Ok(project) => {
            apply_loaded_project(state, project, ProjectLoadOrigin::PersistentPath(path))
        }
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Project open failed: {}",
                error
            )));
            false
        }
    }
}

pub(crate) fn apply_loaded_project(
    state: &mut AppState,
    mut project: ProjectFile,
    origin: ProjectLoadOrigin<'_>,
) -> bool {
    let (simulation_plan, model_library_manager, execution_warnings) =
        match project.execution_context.take() {
            Some(context) => match context.into_state() {
                Ok(restored) => restored,
                Err(error) => {
                    state.push_user_message(ConsoleMessage::error(format!(
                        "Project open failed: persisted execution context is invalid: {error}"
                    )));
                    return false;
                }
            },
            None => (
                crate::common::app::SimSetupState::new(),
                crate::common::app::default_model_library_manager(),
                vec![
                    "This legacy project predates durable simulation plans; RSpice initialized the documented default Transient plan and built-in model catalog"
                        .to_owned(),
                ],
            ),
        };
    project
        .workspace
        .ensure_library_model(&mut project.libraries);
    match origin {
        ProjectLoadOrigin::PersistentPath(_) => {
            state.browser_project_save_name = None;
        }
        #[cfg(any(test, target_arch = "wasm32"))]
        ProjectLoadOrigin::BrowserImport(name) => {
            project.workspace.project.path = None;
            state.browser_project_save_name = Some(name.to_string());
        }
    }
    let simulation_results = project.simulation_results;
    let mut simulation_results_warning = project.simulation_results_warning;
    state.clear_design_execution_context();
    state.library_manager = project.libraries;
    state.workspace = project.workspace;
    state.sim_setup = simulation_plan;
    state.model_library_manager = model_library_manager;
    state.restore_active_schematic_from_workspace();
    state.simulation = crate::state::SimulationState::default();
    if let Err(error) = simulation_results.apply_to_state(&mut state.simulation)
        && simulation_results_warning.is_none()
    {
        simulation_results_warning = Some(format!(
            "Simulation results were not restored because their persisted data is invalid: {error}"
        ));
    }
    if let Some(path) = origin.recent_path() {
        state.remember_recent_file(crate::common::app::RecentKind::Project, path);
    }
    state.push_user_message(ConsoleMessage::info(format!(
        "{}: {}",
        origin.success_prefix(),
        origin.display_label()
    )));
    if let Some(warning) = simulation_results_warning {
        state.push_user_message(ConsoleMessage::warning(warning));
    }
    for warning in execution_warnings {
        state.push_user_message(ConsoleMessage::warning(warning));
    }
    true
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn open_project(state: &mut AppState) -> bool {
    match crate::io::show_open_project_dialog() {
        Ok(path) => load_project_from_path(state, &path),
        Err(ProjectIoError::Cancelled) => false,
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Project open failed: {}",
                error
            )));
            false
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn open_project(state: &mut AppState) -> bool {
    match start_browser_project_import() {
        Ok(()) => {
            state.push_user_message(ConsoleMessage::info(
                "Choose an RSpice project file to open",
            ));
            true
        }
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Project open failed: {}",
                error
            )));
            false
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug)]
enum BrowserProjectImportResult {
    Cancelled,
    Failed(String),
    Loaded(crate::common::browser_file_import::PickedTextFile),
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_PROJECT_IMPORT_RESULT: std::cell::RefCell<Option<BrowserProjectImportResult>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
fn start_browser_project_import() -> Result<(), String> {
    crate::common::browser_file_import::try_begin_text_import(
        crate::common::browser_file_import::BrowserTextImportKind::Project,
    )?;

    crate::common::browser_file_import::pick_text_file(
        crate::io::project_io::PROJECT_FILTER.0,
        crate::io::project_io::PROJECT_FILTER.1,
        |result| {
            let event = match result {
                Ok(Some(file)) => BrowserProjectImportResult::Loaded(file),
                Ok(None) => BrowserProjectImportResult::Cancelled,
                Err(error) => BrowserProjectImportResult::Failed(error),
            };
            BROWSER_PROJECT_IMPORT_RESULT.with(|slot| {
                *slot.borrow_mut() = Some(event);
            });
        },
    );
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn poll_browser_project_import(state: &mut AppState) -> bool {
    let event = BROWSER_PROJECT_IMPORT_RESULT.with(|slot| slot.borrow_mut().take());
    if event.is_some() {
        crate::common::browser_file_import::finish_text_import(
            crate::common::browser_file_import::BrowserTextImportKind::Project,
        );
    }
    match event {
        Some(BrowserProjectImportResult::Loaded(file)) => {
            match crate::io::project_io::load_project_text(&file.contents, None) {
                Ok(project) => apply_loaded_project(
                    state,
                    project,
                    ProjectLoadOrigin::BrowserImport(&file.name),
                ),
                Err(error) => {
                    state.push_user_message(ConsoleMessage::error(format!(
                        "Project open failed: {}",
                        error
                    )));
                    false
                }
            }
        }
        Some(BrowserProjectImportResult::Failed(error)) => {
            state.push_user_message(ConsoleMessage::error(format!(
                "Project open failed: {}",
                error
            )));
            false
        }
        Some(BrowserProjectImportResult::Cancelled) | None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::bode::{BodeData, FrequencyResponse};
    use crate::analysis::eye_diagram::{EyeData, EyeTrace};
    use crate::analysis::fft::{FftData, window::WindowFunction};
    use crate::analysis::histogram::HistogramBuilder;
    use crate::analysis::nyquist::NyquistData;
    use crate::analysis::pole_zero::PoleZeroData;
    use crate::common::app::ActiveViewer;

    fn project_named(path: &str) -> ProjectFile {
        let mut libraries = crate::state::LibraryManager::with_primitives();
        let mut workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut libraries);
        workspace.project.set_path(std::path::PathBuf::from(path));
        ProjectFile::new(workspace, libraries)
    }

    fn project_named_with_results(path: &str) -> ProjectFile {
        let mut libraries = crate::state::LibraryManager::with_primitives();
        let mut workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut libraries);
        workspace.project.set_path(std::path::PathBuf::from(path));

        let waveform = crate::state::WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![0.0, 0.5, 1.0],
            "#00aaff",
        );
        let mut run = crate::state::SimulationRun::new(4);
        run.label = "Run 4 (import fixture)".to_string();
        run.add_analysis(
            crate::state::AnalysisResult::new(2, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![waveform]),
        );
        let mut simulation = crate::state::SimulationState::default();
        simulation.runs = vec![run];
        simulation.next_run_id = 4;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);

        ProjectFile::new_with_simulation_results(
            workspace,
            libraries,
            crate::io::project_io::ProjectSimulationResults::from_state(&simulation),
        )
    }

    fn seed_specialized_viewer_caches(state: &mut AppState) {
        state
            .analysis
            .histogram_state
            .load_histogram(HistogramBuilder::new().build(&[1.0, 2.0, 3.0]));

        let mut bode = BodeData::new();
        bode.add_response(FrequencyResponse::from_arrays(
            "old bode",
            &[1.0, 10.0],
            &[1.0, 0.1],
            &[0.0, -1.0],
        ));
        state.analysis.bode_plot_state.load_data(bode);

        state
            .analysis
            .nyquist_state
            .load_data(NyquistData::from_arrays(
                "old nyquist",
                &[1.0, 10.0],
                &[1.0, -0.5],
                &[0.0, 0.25],
            ));

        state
            .analysis
            .smith_chart_state
            .load_sparam_data("S11", &[1.0], &[0.25], &[0.0]);

        let mut pz = PoleZeroData::new("old pz");
        pz.add_real_pole(-1.0);
        state.analysis.pole_zero_state.load_data(pz);

        let mut eye = EyeData::new(1e-9, 2);
        eye.add_trace(EyeTrace::new(vec![0.0, 0.5, 1.0], vec![0.0, 1.0, 0.0]));
        state.analysis.eye_diagram_state.load_data(eye);

        state
            .analysis
            .fft_state
            .load_data(FftData::from_time_domain(
                "old fft",
                &[0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0],
                8.0,
                WindowFunction::Rectangular,
            ));

        for viewer in [
            ActiveViewer::SmithChart,
            ActiveViewer::EyeDiagram,
            ActiveViewer::Histogram,
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::Fft,
            ActiveViewer::PoleZero,
        ] {
            assert!(
                state.viewer_is_available(viewer),
                "{} should be available before project switch",
                viewer.name()
            );
        }
    }

    fn assert_specialized_viewer_caches_cleared(state: &AppState) {
        for viewer in [
            ActiveViewer::SmithChart,
            ActiveViewer::EyeDiagram,
            ActiveViewer::Histogram,
            ActiveViewer::BodePlot,
            ActiveViewer::Nyquist,
            ActiveViewer::Fft,
            ActiveViewer::PoleZero,
        ] {
            assert!(
                !state.viewer_is_available(viewer),
                "{} should be unavailable after project switch",
                viewer.name()
            );
        }
    }

    #[test]
    fn browser_import_applies_project_clears_runs_and_skips_recents() {
        let mut state = AppState::default();
        state.simulation.start_run();
        assert!(state.simulation.has_results());

        let mut project = project_named("browser-import.rspiceproj");
        project.workspace.project.path = None;

        let imported = apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("browser-import.rspiceproj"),
        );

        assert!(imported);
        assert_eq!(state.workspace.project.display_name(), "browser-import");
        assert!(state.workspace.project.path.is_none());
        assert!(!state.simulation.has_results());
        assert!(state.recent_files.is_empty());
        assert!(state.log_buffer.entries().any(|entry| {
            entry
                .message
                .contains("Imported project: browser-import.rspiceproj")
        }));
    }

    #[test]
    fn browser_import_keeps_project_filename_as_save_suggestion_without_native_path() {
        let mut state = AppState::default();
        let project = project_named("stale-native-path.rspiceproj");

        let imported = apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("browser-import.rspiceproj"),
        );

        assert!(imported);
        assert!(state.workspace.project.path.is_none());
        assert_eq!(
            state.browser_project_save_name.as_deref(),
            Some("browser-import.rspiceproj")
        );
        assert!(state.recent_files.is_empty());
    }

    #[test]
    fn browser_import_filename_is_used_for_next_project_save_as_dialog() {
        let mut state = AppState::default();
        let project = project_named("stale-native-path.rspiceproj");

        assert!(apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("browser-import.rspiceproj"),
        ));

        assert_eq!(
            project_save_dialog_default_name(&state),
            "browser-import.rspiceproj"
        );
    }

    #[test]
    fn project_import_clears_stale_specialized_viewer_caches_without_results() {
        let mut state = AppState::default();
        seed_specialized_viewer_caches(&mut state);

        let project = project_named("browser-import.rspiceproj");

        assert!(apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("browser-import.rspiceproj"),
        ));

        assert_specialized_viewer_caches_cleared(&state);
    }

    #[test]
    fn create_new_project_clears_stale_specialized_viewer_caches() {
        let mut state = AppState::default();
        seed_specialized_viewer_caches(&mut state);

        create_new_project(&mut state);

        assert_specialized_viewer_caches_cleared(&state);
    }

    #[test]
    fn create_new_project_resets_project_owned_plan_and_model_context() {
        use crate::common::simulation_analysis_tabs::{TAB_NOISE, TAB_TRANSIENT};
        use crate::state::model_library::{ModelLibrary, ModelLibraryManager};

        let mut state = AppState::default();
        state.sim_setup.enabled.clear();
        state.sim_setup.enabled.insert(TAB_NOISE);
        state.sim_setup.analysis_order = vec![TAB_NOISE];
        let mut stale_models = ModelLibraryManager::new();
        stale_models.add_library(ModelLibrary::new("stale_project_models"));
        state.model_library_manager = stale_models;

        create_new_project(&mut state);

        assert_eq!(state.sim_setup.enabled.len(), 1);
        assert!(state.sim_setup.enabled.contains(&TAB_TRANSIENT));
        assert_eq!(state.sim_setup.analysis_order, vec![TAB_TRANSIENT]);
        assert!(
            state
                .model_library_manager
                .get_library("stale_project_models")
                .is_none()
        );
        assert!(state.model_library_manager.library_count() > 0);
    }

    #[test]
    fn project_import_restores_plan_order_solver_options_and_model_catalog() {
        use crate::common::simulation_analysis_tabs::{TAB_AC, TAB_NOISE, TAB_TRANSIENT};
        use crate::simulation::dialog::{IntegrationMethod, MatrixSolver};
        use crate::state::model_library::{ModelLibrary, ModelLibraryManager};

        let mut source = AppState::default();
        source.sim_setup.enabled.extend([TAB_AC, TAB_NOISE]);
        source.sim_setup.analysis_order = vec![TAB_NOISE, TAB_TRANSIENT, TAB_AC];
        source.sim_setup.listed.insert(TAB_NOISE);
        source.sim_setup.options.reltol = 2e-4;
        source.sim_setup.options.method = IntegrationMethod::Gear2Only;
        source.sim_setup.options.solver = MatrixSolver::SparseLu;
        source.sim_setup.options.verbose = true;
        let mut project_models = ModelLibraryManager::new();
        project_models.add_library(ModelLibrary::new("project_exact_models"));
        source.model_library_manager = project_models;

        let context =
            ProjectExecutionContext::from_state(&source.sim_setup, &source.model_library_manager)
                .expect("source context validates");
        let expected_plan =
            serde_json::to_value(&context.simulation_plan).expect("expected plan serializes");
        let mut design_libraries = crate::state::LibraryManager::with_primitives();
        let mut workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut design_libraries);
        workspace
            .project
            .set_path(std::path::PathBuf::from("context.rspiceproj"));
        let project = ProjectFile::new_with_execution_context(
            workspace,
            design_libraries,
            ProjectSimulationResults::default(),
            context,
        );

        let mut target = AppState::default();
        target.sim_setup.enabled.clear();
        target.model_library_manager.clear();
        let imported = apply_loaded_project(
            &mut target,
            project,
            ProjectLoadOrigin::BrowserImport("context.rspiceproj"),
        );

        assert!(imported);
        assert_eq!(
            serde_json::to_value(&target.sim_setup).expect("restored plan serializes"),
            expected_plan
        );
        assert_eq!(
            target.sim_setup.options_draft.reltol,
            crate::simulation::dialog::options::format_si_value(2e-4)
        );
        assert!(!target.sim_setup.options_open);
        assert!(target.sim_setup.options_errors.is_empty());
        assert_eq!(target.model_library_manager.library_count(), 1);
        assert!(
            target
                .model_library_manager
                .get_library("project_exact_models")
                .is_some()
        );
    }

    #[test]
    fn invalid_execution_context_does_not_partially_replace_open_project() {
        let mut project = project_named("invalid-context.rspiceproj");
        let mut context = ProjectExecutionContext::from_state(
            &crate::common::app::SimSetupState::new(),
            &crate::state::model_library::ModelLibraryManager::new(),
        )
        .expect("baseline context validates");
        context.simulation_plan.enabled.insert(99);
        context.simulation_plan.analysis_order.push(99);
        project.execution_context = Some(context);

        let mut state = AppState::default();
        let original_project_name = state.workspace.project.display_name().to_owned();
        let original_active_view = state.workspace.active_view.clone();

        let imported = apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("invalid-context.rspiceproj"),
        );

        assert!(!imported);
        assert_eq!(
            state.workspace.project.display_name(),
            original_project_name
        );
        assert_eq!(state.workspace.active_view, original_active_view);
        assert!(state.log_buffer.entries().any(|entry| {
            entry.message.contains(
                "persisted execution context is invalid: simulation_plan.enabled contains unsupported analysis index 99",
            )
        }));
    }

    #[test]
    fn browser_import_restores_project_simulation_results_and_skips_recents() {
        let mut state = AppState::default();
        state.simulation.start_run();
        assert!(state.simulation.has_results());

        let project = project_named_with_results("browser-import.rspiceproj");

        let imported = apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("browser-import.rspiceproj"),
        );

        assert!(imported);
        assert_eq!(state.workspace.project.display_name(), "browser-import");
        assert_eq!(state.simulation.run_count(), 1);
        assert_eq!(
            state
                .simulation
                .active_run()
                .expect("active imported run")
                .label,
            "Run 4 (import fixture)"
        );
        assert_eq!(state.simulation.waveforms.len(), 1);
        assert_eq!(state.simulation.waveforms[0].name, "V(out)");
        assert!(state.recent_files.is_empty());
    }

    #[test]
    fn project_import_resets_non_persisted_simulation_runtime_state() {
        let mut state = AppState::default();
        state.simulation.is_running = true;
        state.simulation.trigger_simulation = true;
        state.simulation.trigger_abort = true;
        state.simulation.progress = 0.75;
        state.simulation.status = "Running stale project".to_string();
        state.simulation.netlist_content = "stale netlist".to_string();
        state
            .simulation
            .node_to_waveform
            .insert("stale".to_string(), 99);
        state.simulation.ground_node = Some("OLD_GND".to_string());

        let project = project_named_with_results("browser-import.rspiceproj");

        assert!(apply_loaded_project(
            &mut state,
            project,
            ProjectLoadOrigin::BrowserImport("browser-import.rspiceproj"),
        ));

        assert!(!state.simulation.is_running);
        assert!(!state.simulation.trigger_simulation);
        assert!(!state.simulation.trigger_abort);
        assert_eq!(state.simulation.progress, 0.0);
        assert!(state.simulation.status.is_empty());
        assert!(state.simulation.netlist_content.is_empty());
        assert_eq!(state.simulation.node_to_waveform.get("stale"), None);
        assert_eq!(state.simulation.ground_node, None);
        assert_eq!(state.simulation.waveforms[0].name, "V(out)");
    }

    #[test]
    fn download_only_project_save_keeps_document_dirty_without_recent_entry() {
        let mut state = AppState::default();
        state.schematic.is_dirty = true;
        state.sync_active_schematic_to_workspace();
        let path = Path::new("browser-project.rspiceproj");
        let mut workspace = state.workspace.clone();
        workspace.project.set_path(path.to_path_buf());

        finish_successful_project_save(&mut state, workspace, path, false);

        assert!(state.workspace.project.path.is_none());
        assert_eq!(
            state.browser_project_save_name.as_deref(),
            Some("browser-project.rspiceproj")
        );
        assert!(state.workspace.any_dirty());
        assert!(state.schematic.is_dirty);
        assert!(state.recent_files.is_empty());
    }

    #[test]
    fn project_save_completion_does_not_warn_about_dropped_results() {
        let mut state = AppState::default();
        state.simulation.start_run();
        let path = Path::new("project-with-results.rspiceproj");
        let mut workspace = state.workspace.clone();
        workspace.project.set_path(path.to_path_buf());

        finish_successful_project_save(&mut state, workspace, path, false);

        assert!(!state.log_buffer.entries().any(|entry| {
            entry
                .message
                .contains("Simulation results are not saved with project files")
        }));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn save_project_to_path_writes_simulation_results() {
        let mut state = AppState::default();
        let waveform = crate::state::WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![0.0, 1.5, 3.0],
            "#00aaff",
        );
        let mut run = crate::state::SimulationRun::new(9);
        run.add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![waveform]),
        );
        state.simulation.runs = vec![run];
        state.simulation.next_run_id = 9;
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);

        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rspice-save-project-results-{}-{unique}.rspiceproj",
            std::process::id()
        ));

        let saved = save_project_to_path(&mut state, &path);
        let loaded = crate::io::load_project_file(&path).expect("saved project reloads");
        let _ = std::fs::remove_file(&path);

        assert!(saved);
        assert!(loaded.execution_context.is_some());
        assert_eq!(loaded.simulation_results.runs.len(), 1);
        assert_eq!(
            loaded.simulation_results.runs[0].analyses[0].waveforms[0].name,
            "V(out)"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn save_project_to_path_drops_invalid_simulation_results_without_failing_save() {
        let mut state = AppState::default();
        let waveform = crate::state::WaveformData::new(
            "V(out)",
            vec![0.0, 1.0, 2.0],
            vec![0.0, f64::NAN, 3.0],
            "#00aaff",
        );
        let mut run = crate::state::SimulationRun::new(10);
        run.add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![waveform]),
        );
        state.simulation.runs = vec![run];
        state.simulation.next_run_id = 10;
        state.simulation.active_run_idx = Some(0);
        state.simulation.active_analysis_idx = Some(0);

        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rspice-save-project-invalid-results-{}-{unique}.rspiceproj",
            std::process::id()
        ));

        let saved = save_project_to_path(&mut state, &path);
        let loaded = crate::io::load_project_file(&path).expect("saved project reloads");
        let _ = std::fs::remove_file(&path);

        assert!(saved);
        assert!(loaded.simulation_results.is_empty());
        assert!(state.log_buffer.entries().any(|entry| {
            entry
                .message
                .contains("Saved project without simulation results")
        }));
    }
}
