//! Which project transitions authorize a destructive action.
//!
//! Browser copy-import and canonical save are distinct origins here because
//! conflating them overwrites the operator's file without a picker. The
//! remaining cases hold the reverse: only a verified canonical completion may
//! act immediately, and a live mirror snapshot waits out a local run rather
//! than landing on top of it.

use super::*;
use crate::analysis::bode::BodeData;
use crate::analysis::eye_diagram::{EyeData, EyeTrace};
use crate::analysis::fft::{FftData, window::WindowFunction};
use crate::analysis::histogram::HistogramBuilder;
use crate::analysis::nyquist::NyquistData;
use crate::analysis::pole_zero::PoleZeroData;
use crate::io::{ProjectExecutionContext, ProjectSimulationResults};
use crate::workbench::app_state::ActiveViewer;

fn seal_legacy_unattributed(run: &mut crate::state::SimulationRun) {
    run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
        .expect("synthetic historical run has valid unattributed legacy provenance");
}

fn project_named(path: &str) -> ProjectFile {
    let mut libraries = crate::state::LibraryManager::with_primitives();
    let mut workspace = crate::state::ProjectWorkspace::new_bootstrapped(&mut libraries);
    workspace.project.set_path(std::path::PathBuf::from(path));
    ProjectFile::new(workspace, libraries)
}

fn assert_active_grid_pitch_contract(state: &AppState, pitch: crate::state::SchematicGridPitch) {
    let expected = pitch.canvas_grid_size();
    assert_eq!(state.schematic.document_policy.grid_pitch, pitch);
    assert_eq!(state.schematic.grid_size, expected);
    assert_eq!(state.schematic.snap_engine.grid_size, expected);
    assert_eq!(state.ui.schematic_snap.grid_size, expected);
}

#[test]
fn new_and_closed_project_installs_reconcile_every_grid_pitch_owner() {
    use crate::workbench::ChoicePreference;

    let mut state = AppState::default();
    state.ui.schematic_snap.snap_radius = 8;
    state.ui.schematic_snap.snap_to_grid = false;
    state.ui.schematic_snap.grid_size = 555;
    state
        .ui
        .preferences
        .set_choice(ChoicePreference::SchematicGrid, 1)
        .unwrap();

    create_new_project(&mut state);

    assert_active_grid_pitch_contract(&state, crate::state::SchematicGridPitch::Mil25);
    assert_eq!(state.schematic.snap_engine.snap_radius, 8);
    assert!(!state.schematic.snap_engine.snap_to_grid);

    state
        .ui
        .preferences
        .set_choice(ChoicePreference::SchematicGrid, 2)
        .unwrap();
    state.ui.schematic_snap.grid_size = 777;
    state
        .workbench
        .begin_project_close(ProjectCloseDestination::EmptyWorkbench);

    assert!(close_project_discard(&mut state));
    assert_active_grid_pitch_contract(&state, crate::state::SchematicGridPitch::Metric);
    assert_eq!(state.schematic.snap_engine.snap_radius, 8);
    assert!(!state.schematic.snap_engine.snap_to_grid);
}

#[test]
fn only_verified_canonical_completion_authorizes_an_immediate_destructive_action() {
    assert!(SaveRequestOutcome::CanonicalComplete.authorizes_immediate_destructive_action());
    assert!(
        !SaveRequestOutcome::CanonicalPending(crate::product::TransactionId::new())
            .authorizes_immediate_destructive_action()
    );
    assert!(!SaveRequestOutcome::CopyOnly.authorizes_immediate_destructive_action());
    assert!(!SaveRequestOutcome::CopyPending.authorizes_immediate_destructive_action());
    assert!(!SaveRequestOutcome::Cancelled.authorizes_immediate_destructive_action());
    assert!(
        !SaveRequestOutcome::Failed("disk unavailable".to_owned())
            .authorizes_immediate_destructive_action()
    );
}

#[test]
fn browser_canonical_origin_is_distinct_from_copy_import_semantics() {
    let canonical = ProjectLoadOrigin::BrowserCanonical("sensor-afe.rspiceproj");
    let imported = ProjectLoadOrigin::BrowserImport("sensor-afe.rspiceproj");

    assert_eq!(canonical.display_label(), "sensor-afe.rspiceproj");
    assert_eq!(canonical.success_prefix(), "Opened project");
    assert_eq!(imported.success_prefix(), "Imported project");
    assert!(canonical.recent_path().is_none());
}

#[test]
fn browser_project_copy_never_opens_a_save_picker_over_a_canonical_handle() {
    assert!(browser_save_picker_is_safe(true, None, true));
    assert!(!browser_save_picker_is_safe(true, Some(7), true));
    assert!(!browser_save_picker_is_safe(true, Some(7), false));
    assert!(browser_save_picker_is_safe(false, Some(7), true));
}

#[test]
fn browser_project_copy_never_changes_canonical_save_name() {
    let mut canonical_name = Some("source-project.rspiceproj".to_owned());
    accept_browser_canonical_display_name(&mut canonical_name, true, "independent-copy.rspiceproj");
    assert_eq!(canonical_name.as_deref(), Some("source-project.rspiceproj"));

    accept_browser_canonical_display_name(
        &mut canonical_name,
        false,
        "canonical-project.rspiceproj",
    );
    assert_eq!(
        canonical_name.as_deref(),
        Some("canonical-project.rspiceproj")
    );
}

#[test]
fn launcher_continue_closes_through_the_reviewed_lifecycle_into_empty_workbench() {
    let mut state = AppState::default();
    state.workbench.open_project_launcher();

    assert!(request_close_project_to_empty_workbench(&mut state));
    assert!(matches!(
        state.dialogs.project_review_dialog.request,
        Some(crate::workbench::app::ProjectReviewRequest::CloseProject)
    ));

    state.dialogs.project_review_dialog.close();
    assert!(close_project_discard(&mut state));
    assert!(!state.project_lifecycle.project_open);
    assert!(!state.workbench.project_launcher_open);
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Project
    );
}

#[test]
fn close_to_live_mirror_raises_the_one_shot_engine_entry_request() {
    let mut state = AppState::default();
    state
        .workbench
        .begin_project_close(ProjectCloseDestination::LiveMirror);

    assert!(close_project_discard(&mut state));
    assert!(!state.project_lifecycle.project_open);
    assert!(!state.workbench.project_launcher_open);
    assert_eq!(
        state.workbench.workspace,
        crate::workbench::state::Workspace::Project
    );
    assert!(state.workbench.take_live_mirror_entry());
    assert!(!state.workbench.take_live_mirror_entry());
}

#[test]
fn live_project_snapshot_applies_wholesale_and_never_keeps_the_host_path() {
    let mut host = AppState::default();
    host.workspace
        .project
        .set_path(std::path::PathBuf::from("C:/host-only/design.rspiceproj"));
    let snapshot = crate::workbench::lifecycle::project_lifecycle::snapshot(&host)
        .expect("host state snapshots");
    let text =
        crate::io::project_io::serialize_project_file(&snapshot).expect("snapshot serializes");

    let mut guest = AppState::default();
    assert!(matches!(
        apply_live_project_snapshot(&mut guest, text.as_bytes(), "Jaime"),
        LiveProjectApply::Applied
    ));
    assert!(guest.project_lifecycle.project_open);
    assert_eq!(guest.workspace.project.id(), host.workspace.project.id());
    // The host's on-disk location must never become a guest save target.
    assert!(guest.workspace.project.path.is_none());
    assert!(
        crate::workbench::lifecycle::project_lifecycle::canonical_native_path(&guest).is_none()
    );
}

#[test]
fn live_project_snapshot_waits_out_a_local_run_and_rejects_garbage() {
    let host = AppState::default();
    let snapshot = crate::workbench::lifecycle::project_lifecycle::snapshot(&host)
        .expect("host state snapshots");
    let text =
        crate::io::project_io::serialize_project_file(&snapshot).expect("snapshot serializes");

    let mut guest = AppState::default();
    guest.simulation.is_running = true;
    assert!(matches!(
        apply_live_project_snapshot(&mut guest, text.as_bytes(), "Jaime"),
        LiveProjectApply::RetryLater
    ));

    guest.simulation.is_running = false;
    assert!(matches!(
        apply_live_project_snapshot(&mut guest, b"not a project", "Jaime"),
        LiveProjectApply::Rejected
    ));
}

#[test]
fn mirror_save_copy_policy_gates_every_project_persistence_path() {
    let mut state = AppState::default();
    state.workbench.live_write_locks.mirror = true;
    state.workbench.live_write_locks.mirror_save_copy_allowed = false;

    assert!(!save_project(&mut state));
    assert!(!save_all(&mut state));
    assert!(!save_project_as(&mut state));
    assert!(matches!(
        save_all_for_continuation(&mut state),
        SaveRequestOutcome::Failed(_)
    ));
}

#[test]
fn save_copy_permission_never_turns_a_live_mirror_into_a_canonical_project() {
    let mut state = AppState::default();
    state.workbench.live_write_locks.mirror = true;
    state.workbench.live_write_locks.mirror_save_copy_allowed = true;

    assert_eq!(
        live_mirror_save_block(&state, false),
        Some("A live mirror cannot be saved in place. Use Save As to create an independent copy.")
    );
    assert_eq!(live_mirror_save_block(&state, true), None);
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
    seal_legacy_unattributed(&mut run);
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
    let waveform = crate::state::WaveformData::new(
        "V(out)",
        vec![0.0, 1.0, 2.0],
        vec![0.0, 0.5, 1.0],
        "#00aaff",
    );
    let mut run = crate::state::SimulationRun::new(1);
    run.add_analysis(
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![waveform]),
    );
    seal_legacy_unattributed(&mut run);
    state.simulation.runs = vec![run];
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    state.simulation.next_run_id = 2;

    state
        .analysis
        .histogram_state
        .load_histogram(HistogramBuilder::new().build(&[1.0, 2.0, 3.0]));

    let mut bode = BodeData::new();
    bode.add_response();
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
        .load_sparam_data("S11", &[1.0], &[0.25], &[0.0], Some(50.0))
        .expect("valid Smith fixture");

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

    let provenance = state
        .active_specialized_viewer_cache_provenance()
        .expect("default test project has an active retained analysis");
    for viewer in [
        ActiveViewer::BodePlot,
        ActiveViewer::Nyquist,
        ActiveViewer::SmithChart,
        ActiveViewer::Histogram,
        ActiveViewer::Fft,
        ActiveViewer::EyeDiagram,
    ] {
        state.bind_specialized_viewer_cache(viewer, provenance);
    }

    for viewer in [
        ActiveViewer::SmithChart,
        ActiveViewer::EyeDiagram,
        ActiveViewer::Histogram,
        ActiveViewer::BodePlot,
        ActiveViewer::Nyquist,
        ActiveViewer::Fft,
    ] {
        assert!(
            state.viewer_is_available(viewer),
            "{} should be available before project switch",
            viewer.name()
        );
    }
}

fn assert_specialized_viewer_caches_cleared(state: &AppState) {
    assert!(
        state.analysis.pole_zero_state.is_empty(),
        "legacy pole-zero presentation cache should be cleared"
    );
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
    seal_legacy_unattributed(state.simulation.start_run());
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
    state.browser_project_save_name = Some("previous-project.rspiceproj".to_owned());

    create_new_project(&mut state);

    assert_specialized_viewer_caches_cleared(&state);
    assert!(
        state.browser_project_save_name.is_none(),
        "a new browser project must not inherit the previous canonical suggestion"
    );
}

#[test]
fn create_new_project_resets_project_owned_plan_and_model_context() {
    use crate::simulation::plan::AnalysisKind;
    use crate::state::model_library::{ModelLibrary, ModelLibraryManager};

    let mut state = AppState::default();
    let stale_plan = state
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("current project owns a stable plan");
    let transient_id = stale_plan.instances()[0].id();
    stale_plan
        .remove(transient_id, Vec::new())
        .expect("default transient removes");
    stale_plan
        .insert(AnalysisKind::Noise)
        .expect("stale noise analysis inserts");
    let mut stale_models = ModelLibraryManager::new();
    stale_models.add_library(ModelLibrary::new("stale_project_models"));
    state.model_library_manager = stale_models;

    create_new_project(&mut state);

    let reset_plan = state
        .sim_setup
        .stable_analysis_plan()
        .expect("new project owns a stable plan");
    assert_eq!(reset_plan.instances().len(), 1);
    assert_eq!(reset_plan.instances()[0].kind(), AnalysisKind::Transient);
    assert!(reset_plan.instances()[0].enabled());
    assert!(
        state
            .model_library_manager
            .get_library("stale_project_models")
            .is_none()
    );
    assert!(state.model_library_manager.library_count() > 0);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn new_and_closed_projects_restore_configured_sources_without_leaking_project_libraries() {
    use crate::state::model_library::ModelLibrary;

    let root = std::env::temp_dir().join(format!(
        "rspice-project-lifecycle-pdk-{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&root).expect("create configured PDK root");
    std::fs::write(
        root.join("configured.lib"),
        ".model configured_n NMOS (LEVEL=1)\n",
    )
    .expect("write configured PDK source");

    let mut state = AppState::default();
    state.pdk_config = crate::state::pdk_config::PdkConfig::new();
    state
        .pdk_config
        .add_library_path(root.to_string_lossy().into_owned());
    state
        .model_library_manager
        .add_library(ModelLibrary::new("previous_project_only"));

    create_new_project(&mut state);

    assert!(
        state
            .model_library_manager
            .get_library("configured")
            .is_some(),
        "a new project must rebuild the configured application PDK catalog"
    );
    assert!(
        state
            .model_library_manager
            .get_library("previous_project_only")
            .is_none(),
        "project-owned catalog state must not cross the new-project boundary"
    );

    state
        .model_library_manager
        .add_library(ModelLibrary::new("closing_project_only"));
    state.project_lifecycle.project_open = true;
    assert!(close_project_discard(&mut state));
    assert!(
        state
            .model_library_manager
            .get_library("configured")
            .is_some(),
        "closing a project must return to the configured application catalog"
    );
    assert!(
        state
            .model_library_manager
            .get_library("closing_project_only")
            .is_none(),
        "closing a project must discard its catalog additions"
    );

    std::fs::remove_dir_all(root).expect("remove configured PDK root");
}

#[test]
fn create_new_project_copies_the_retained_solver_default_into_the_plan() {
    use crate::simulation::dialog::IntegrationMethod;
    use crate::workbench::ChoicePreference;

    let mut state = AppState::default();
    state
        .ui
        .preferences
        .set_choice(ChoicePreference::DefaultSolverPreset, 3)
        .expect("Robust is a valid solver preset");

    create_new_project(&mut state);

    assert_eq!(state.sim_setup.options.itl1, 200);
    assert_eq!(state.sim_setup.options.itl4, 20);
    assert!(state.sim_setup.options.arc_length);
    assert_eq!(state.sim_setup.options.method, IntegrationMethod::Gear2);
    assert_eq!(state.sim_setup.options.temp, 27.0);
}

#[test]
fn create_new_project_captures_the_personal_drawing_sheet_default() {
    use crate::state::{
        DrawingSheetInheritance, DrawingSheetStandard, SchematicPageOrientation,
        SchematicSheetFormat,
    };

    let mut state = AppState::default();
    let mut personal = state.ui.preferences.drawing_sheet_personal_preferences();
    personal.default_format = SchematicSheetFormat::from_standard(
        DrawingSheetStandard::AnsiC,
        SchematicPageOrientation::Portrait,
    )
    .try_update(|draft| {
        draft.inheritance = DrawingSheetInheritance::UserDefault;
    })
    .expect("the personal default is valid");
    state
        .ui
        .preferences
        .set_drawing_sheet_personal_preferences(personal)
        .expect("the personal default persists");

    create_new_project(&mut state);

    let project_default = state
        .workspace
        .design_management
        .drawing_sheet_settings()
        .default_format
        .clone();
    assert!(
        matches!(
            &project_default.authored_size,
            crate::state::AuthoredDrawingSheetSize::Standard {
                standard: DrawingSheetStandard::AnsiC
            }
        ),
        "the personal physical format becomes the new project's default"
    );
    assert_eq!(
        project_default.orientation,
        SchematicPageOrientation::Portrait
    );
    assert_eq!(
        project_default.inheritance,
        DrawingSheetInheritance::ProjectDefault
    );
    assert!(crate::workbench::app::open_drawing_sheet_setup_for_state(
        &mut state
    ));
    let page_setup_format = state
        .dialogs
        .drawing_sheet_setup
        .draft
        .validate()
        .expect("the initial Page Setup draft is valid")
        .page_format;
    assert_eq!(
        page_setup_format.as_drawing_sheet_default(),
        project_default,
        "the initial Page Setup draft resolves the project default without storing a concrete sheet title in the reusable template"
    );
    assert_eq!(
        page_setup_format.title_block.fields[&crate::state::DrawingSheetTitleFieldId::SheetTitle]
            .value,
        "top",
        "the governed sheet keeps its own title while inheriting the project format"
    );
}

#[test]
fn new_project_custom_default_is_exact_and_has_no_personal_preset_dependency() {
    use crate::state::{
        AuthoredDrawingSheetSize, DrawingSheetInheritance, SchematicPageOrientation,
        SchematicSheetFormat,
    };

    let mut state = AppState::default();
    let mut personal = state.ui.preferences.drawing_sheet_personal_preferences();
    personal.default_format = SchematicSheetFormat::try_custom(
        "Personal lab panel",
        250_001,
        400_003,
        SchematicPageOrientation::Portrait,
    )
    .unwrap()
    .try_update(|draft| {
        draft.inheritance = DrawingSheetInheritance::UserDefault;
        let AuthoredDrawingSheetSize::Custom { snapshot } = &mut draft.authored_size else {
            unreachable!("the test starts with a custom size");
        };
        snapshot.preset_id = Some("personal-lab-panel".to_owned());
    })
    .unwrap();
    state
        .ui
        .preferences
        .set_drawing_sheet_personal_preferences(personal.clone())
        .unwrap();
    let retained_personal = state.ui.preferences.drawing_sheet_personal_preferences();

    create_new_project(&mut state);

    let project_settings = state.workspace.design_management.drawing_sheet_settings();
    assert_eq!(
        project_settings.default_format.portrait_dimensions_um(),
        (250_001, 400_003)
    );
    let AuthoredDrawingSheetSize::Custom { snapshot } =
        &project_settings.default_format.authored_size
    else {
        panic!("the exact custom physical format must be retained");
    };
    assert!(snapshot.preset_id.is_none());
    assert!(!snapshot.source_preset_unavailable);
    assert!(project_settings.presets.is_empty());
    assert_eq!(
        state.ui.preferences.drawing_sheet_personal_preferences(),
        retained_personal,
        "seeding a project must not mutate personal preferences"
    );
}

#[test]
fn project_import_restores_plan_order_solver_options_and_model_catalog() {
    use crate::simulation::dialog::{IntegrationMethod, MatrixSolver};
    use crate::simulation::plan::{AnalysisKind, AnalysisLifecycleState};
    use crate::state::model_library::{ModelLibrary, ModelLibraryManager};

    let mut source = AppState::default();
    let source_plan = source
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("current project owns a stable plan");
    let transient_id = source_plan.instances()[0].id();
    let (op_id, _) = source_plan
        .insert_at(AnalysisKind::OperatingPoint, 0)
        .expect("OP inserts first");
    let (ac_id, _) = source_plan
        .insert_at(AnalysisKind::Ac, 1)
        .expect("AC inserts second");
    source_plan
        .bind_dependency(ac_id, AnalysisKind::OperatingPoint, op_id)
        .expect("AC binds exact OP");
    let (noise_id, _) = source_plan
        .insert_at(AnalysisKind::Noise, 2)
        .expect("noise inserts third");
    source_plan
        .bind_dependency(noise_id, AnalysisKind::OperatingPoint, op_id)
        .expect("noise binds exact OP");
    source.sim_setup.options.reltol = 2e-4;
    source.sim_setup.options.method = IntegrationMethod::Gear2;
    source.sim_setup.options.solver = MatrixSolver::SparseLu;
    source.sim_setup.options.arc_length = true;
    let mut project_models = ModelLibraryManager::new();
    project_models.add_library(ModelLibrary::new("project_exact_models"));
    source.model_library_manager = project_models;

    let context = ProjectExecutionContext::from_state(
        source.workspace.project.id(),
        &source.sim_setup,
        &source.model_library_manager,
    )
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
    target
        .sim_setup
        .analysis_plan
        .as_mut()
        .expect("target owns a stable plan")
        .insert(AnalysisKind::DcSweep)
        .expect("target stale analysis inserts");
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
    let restored = target
        .sim_setup
        .stable_analysis_plan()
        .expect("import restores stable plan");
    assert_eq!(
        restored
            .instances()
            .iter()
            .map(|instance| instance.id())
            .collect::<Vec<_>>(),
        vec![op_id, ac_id, noise_id, transient_id]
    );
    assert_eq!(
        restored
            .instance(ac_id)
            .expect("AC restored")
            .dependencies()[0]
            .target(),
        op_id
    );
    assert_eq!(
        restored
            .instance(noise_id)
            .expect("noise restored")
            .dependencies()[0]
            .target(),
        op_id
    );
    assert!(restored.instances().iter().all(|instance| {
        instance.enabled() && instance.lifecycle() == AnalysisLifecycleState::Draft
    }));
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
    let context = ProjectExecutionContext::from_state(
        project.workspace.project.id(),
        &crate::workbench::app_state::SimSetupState::new(),
        &crate::state::model_library::ModelLibraryManager::new(),
    )
    .expect("baseline context validates");
    let mut value = serde_json::to_value(context).expect("context serializes");
    let instances = value["simulation_plan"]["analysis_plan"]["instances"]
        .as_array_mut()
        .expect("v4 instances are an array");
    let duplicate = instances[0].clone();
    instances.push(duplicate);
    let context: ProjectExecutionContext =
        serde_json::from_value(value).expect("corrupt structure deserializes for validation");
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
        entry
            .message
            .contains("persisted execution context is invalid")
            && entry.message.contains("appears more than once")
    }));
}

#[test]
fn browser_import_restores_project_simulation_results_and_skips_recents() {
    let mut state = AppState::default();
    seal_legacy_unattributed(state.simulation.start_run());
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
    // An actually running local simulation deliberately blocks project
    // replacement. These are stale, non-persisted controls from an
    // already-finished run and must be reset by the accepted import.
    state.simulation.is_running = false;
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
    state.ui.code_workspace.page =
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation;
    state.ui.code_workspace.automation.debug.watches.push(
        crate::workbench::documents::code_workspace::AutomationWatch {
            expression: "old_project.signal".to_owned(),
            value: "1.25".to_owned(),
            error: None,
        },
    );
    state.ui.netlist.diagnostics = std::sync::Arc::new(
        crate::workbench::documents::netlist_document::NetlistDiagnosticCollection::try_new(
            vec![
                crate::workbench::documents::netlist_document::Diagnostic::error(
                    "old project diagnostic",
                ),
            ],
            "",
        )
        .unwrap(),
    );
    state.log_buffer.warning(
        crate::diagnostics::LogSource::Simulation,
        "old project warning",
    );
    state.script_console.input_buffer = "old project command".to_owned();
    state.script_console.history.push(
        crate::workbench::app_state::session::script_console::ConsoleHistoryItem {
            command: "old".to_owned(),
            output: Default::default(),
        },
    );

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
    assert_eq!(
        state.ui.code_workspace.page,
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Netlist
    );
    assert!(state.ui.code_workspace.automation.debug.watches.is_empty());
    assert_eq!(state.ui.netlist.diagnostics.summary().total(), 0);
    assert!(state.script_console.input_buffer.is_empty());
    assert!(state.script_console.history.is_empty());
    assert!(
        !state
            .log_buffer
            .entries()
            .any(|entry| entry.message == "old project warning")
    );
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
    seal_legacy_unattributed(&mut run);
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

/// Markers are the reader's own annotation of a result. Losing them on
/// close is data loss, so they are written beside the retained datasets
/// and re-attach to the analysis they named.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn save_project_to_path_round_trips_result_markers() {
    use crate::workbench::documents::result_document::AnalysisPresentationKey;

    let mut state = AppState::default();
    let waveform = crate::state::WaveformData::new(
        "V(out)",
        vec![0.0, 1.0, 2.0],
        vec![0.0, 1.5, 3.0],
        "#00aaff",
    );
    let mut run = crate::state::SimulationRun::new(11);
    run.add_analysis(
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![waveform]),
    );
    seal_legacy_unattributed(&mut run);
    state.simulation.runs = vec![run];
    state.simulation.next_run_id = 11;
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);

    let active = state.simulation.active_run().expect("active retained run");
    let analysis = AnalysisPresentationKey::new(active.dataset_id, &active.analyses[0]);
    let anchor = state
        .ui
        .results
        .markers
        .first()
        .map(|marker| marker.anchor.clone());
    assert!(anchor.is_none(), "a fresh workspace carries no markers");
    let id = {
        let waveform_anchor =
            crate::workbench::documents::result_document::marker_anchor_for(analysis, "V(out)");
        state
            .ui
            .results
            .add_marker(analysis, waveform_anchor, "V(out)".to_owned(), 1.0)
    };
    if let Some(marker) = state.ui.results.marker_mut(id) {
        marker.note = "settling point".to_owned();
    }

    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rspice-save-project-markers-{}-{unique}.rspiceproj",
        std::process::id()
    ));

    let saved = save_project_to_path(&mut state, &path);
    let loaded = crate::io::load_project_file(&path).expect("saved project reloads");
    let _ = std::fs::remove_file(&path);

    assert!(saved);
    assert_eq!(loaded.result_markers.len(), 1);
    assert_eq!(loaded.result_markers[0].note, "settling point");

    let mut reopened = AppState::default();
    reopened.simulation = state.simulation.clone();
    crate::workbench::documents::result_document::restore_markers(
        &mut reopened,
        loaded.result_markers,
    );
    assert_eq!(reopened.ui.results.markers.len(), 1);
    assert_eq!(reopened.ui.results.markers[0].note, "settling point");
}

/// A logarithmic Y axis is a decision about a dataset, like a marker.
///
/// It used to live in egui's persisted memory under a hand-built id,
/// which put it outside every owner that knows what a project is: closing
/// one could not clear it, saving one could not carry it, and it grew an
/// entry for every dataset ever opened.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn save_project_to_path_round_trips_logarithmic_panes() {
    use crate::workbench::documents::result_document::{
        AnalysisPresentationKey, WavePanePresentationKey,
    };

    let mut state = AppState::default();
    let mut run = crate::state::SimulationRun::new(12);
    run.add_analysis(
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Ac, "AC").with_waveforms(
            vec![crate::state::WaveformData::new(
                "V(out)",
                vec![1.0, 10.0, 100.0],
                vec![1.0, 0.5, 0.1],
                "#00aaff",
            )],
        ),
    );
    seal_legacy_unattributed(&mut run);
    state.simulation.runs = vec![run];
    state.simulation.next_run_id = 12;
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);

    let active = state.simulation.active_run().expect("active retained run");
    let analysis = AnalysisPresentationKey::new(active.dataset_id, &active.analyses[0]);
    let pane = WavePanePresentationKey {
        analysis,
        unit: "V".to_owned(),
    };
    // A pane whose analysis this project does not retain must not come
    // back — it would put an axis in log space for nothing on screen.
    let orphan = WavePanePresentationKey {
        analysis: AnalysisPresentationKey::new(
            crate::product::DatasetId::new(),
            &crate::state::AnalysisResult::new(9, crate::state::AnalysisType::Transient, "gone"),
        ),
        unit: "A".to_owned(),
    };
    state.ui.results.log_y_panes.insert(pane.clone());
    state.ui.results.log_y_panes.insert(orphan);

    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rspice-save-project-logy-{}-{unique}.rspiceproj",
        std::process::id()
    ));

    let saved = save_project_to_path(&mut state, &path);
    let loaded = crate::io::load_project_file(&path).expect("saved project reloads");
    let _ = std::fs::remove_file(&path);

    assert!(saved);
    assert_eq!(loaded.result_log_y_panes.len(), 2);

    let mut reopened = AppState::default();
    reopened.simulation = state.simulation.clone();
    crate::workbench::documents::result_document::restore_log_y_panes(
        &mut reopened,
        loaded.result_log_y_panes,
    );
    assert_eq!(
        reopened.ui.results.log_y_panes.iter().collect::<Vec<_>>(),
        vec![&pane],
        "only the pane whose analysis is still retained comes back"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn save_project_to_path_round_trips_stable_expression_traces() {
    let mut state = AppState::default();
    let mut run = crate::state::SimulationRun::new(13);
    run.add_analysis(
        crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![crate::state::WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 0.5, 1.0],
                "#00aaff",
            )]),
    );
    seal_legacy_unattributed(&mut run);
    state.simulation.runs = vec![run];
    state.simulation.next_run_id = 13;
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    let expression_owner =
        crate::workbench::documents::result_document::AnalysisPresentationKey::new(
            state.simulation.runs[0].dataset_id,
            &state.simulation.runs[0].analyses[0],
        );
    assert!(
        state
            .ui
            .results
            .add_expression_trace(&state.simulation, expression_owner, "V(out) * 2".to_owned(),)
            .expect("retained expression owner")
    );
    state.workspace.visualization_documents_dirty = true;

    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rspice-save-project-expressions-{}-{unique}.rspiceproj",
        std::process::id()
    ));

    let saved = save_project_to_path(&mut state, &path);
    let loaded = crate::io::load_project_file(&path).expect("saved project reloads");
    let _ = std::fs::remove_file(&path);

    assert!(saved);
    assert_eq!(loaded.result_expression_groups.len(), 1);
    assert_eq!(
        loaded.result_expression_groups[0].traces,
        vec![crate::workbench::documents::result_document::ExprTrace {
            text: "V(out) * 2".to_owned(),
            visible: true,
        }]
    );

    let mut reopened = AppState::default();
    reopened.simulation = state.simulation.clone();
    crate::workbench::documents::result_document::restore_expression_groups(
        &mut reopened,
        loaded.result_expression_groups,
    );
    assert_eq!(
        reopened
            .ui
            .results
            .project_expression_groups(&reopened.simulation)[0]
            .traces[0]
            .text,
        "V(out) * 2"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn save_project_to_path_round_trips_canonical_result_document_entities() {
    use crate::results::visualization_document::{
        AnnotationAnchor, DocumentEdit, PaneDataBinding, TypedValue, VisualizationDocument,
        VisualizationPresentationPolicy,
    };

    let mut state = AppState::default();
    let mut run = crate::state::SimulationRun::new(21);
    run.add_analysis(
        crate::state::AnalysisResult::new(4, crate::state::AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![crate::state::WaveformData::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 0.5, 1.0],
                "#00aaff",
            )]),
    );
    seal_legacy_unattributed(&mut run);
    state.simulation.runs = vec![run];
    state.simulation.next_run_id = 21;
    state.simulation.active_run_idx = Some(0);
    state.simulation.active_analysis_idx = Some(0);
    let run = &state.simulation.runs[0];
    let analysis = &run.analyses[0];
    let source =
        crate::workbench::documents::result_document::visualization_source_dataset(run, analysis)
            .expect("source projection");
    let analysis_id = analysis.provenance().map_or_else(
        || {
            let name = format!("legacy-analysis-v1/{}", analysis.id);
            crate::product::AnalysisInstanceId::from_namespace(
                run.dataset_id.as_uuid(),
                name.as_bytes(),
            )
        },
        |provenance| provenance.source_instance_id(),
    );
    let mut document =
        VisualizationDocument::new("Saved review", vec![source.clone()]).expect("result document");
    let pane_id = document.panes()[0].id;
    let receipt = document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::SetPaneSource {
                    pane_id,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding: Some(PaneDataBinding {
                        analysis_id,
                        dataset: source.binding(),
                    }),
                },
                DocumentEdit::SetPresentation(VisualizationPresentationPolicy {
                    significant_digits: 13,
                    phase_continuous: true,
                }),
            ],
        )
        .expect("pane and presentation commit");
    let trace_id = receipt
        .created
        .iter()
        .find_map(|entity| match entity {
            crate::results::visualization_document::EntityRef::Trace(id) => Some(*id),
            _ => None,
        })
        .expect("bound pane provisions a trace");
    let cursor_axis_id = document
        .traces()
        .iter()
        .find(|trace| trace.id == trace_id)
        .expect("bound trace")
        .x_axis_id;
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::AddTypedMarker {
                    pane_id,
                    trace_id,
                    coordinate: TypedValue::Real(1.0),
                    label: "M1".to_owned(),
                    kind: crate::results::visualization_document::PlotMarkerKind::PointNote,
                    scope: crate::results::visualization_document::PlotMarkerScope::Document,
                    source_specification: None,
                },
                DocumentEdit::AddScalarMeasurement {
                    pane_id,
                    trace_ids: vec![trace_id],
                    expression: "rms(V(out))".to_owned(),
                    value: 0.6454972243679028,
                },
                DocumentEdit::AddAnnotation {
                    pane_id,
                    anchor: AnnotationAnchor::Trace {
                        trace_id,
                        coordinate: TypedValue::Real(1.0),
                    },
                    text: "Retained review note".to_owned(),
                },
                DocumentEdit::AddCursor {
                    pane_id,
                    axis_id: cursor_axis_id,
                    position: TypedValue::Real(1.0),
                    label: "A".to_owned(),
                },
            ],
        )
        .expect("authored entities commit");
    let document_id = state
        .workspace
        .insert_visualization_document(document)
        .expect("workspace owns result document");

    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rspice-save-project-result-document-{}-{unique}.rspiceproj",
        std::process::id()
    ));
    let saved = save_project_to_path(&mut state, &path);
    let loaded = crate::io::load_project_file(&path).expect("saved project reloads");
    let _ = std::fs::remove_file(&path);

    assert!(saved);
    let restored = loaded
        .workspace
        .visualization_document(document_id)
        .expect("result document round trips");
    assert_eq!(restored.presentation().significant_digits, 13);
    assert!(restored.presentation().phase_continuous);
    assert_eq!(restored.traces().len(), 1);
    assert_eq!(restored.markers().len(), 1);
    assert_eq!(restored.measurements().len(), 1);
    assert_eq!(restored.annotations().len(), 1);
    assert_eq!(restored.cursors().len(), 1);
    assert_eq!(restored.cursors()[0].label, "A");
    assert_eq!(
        restored.measurements()[0].expression.as_deref(),
        Some("rms(V(out))")
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn save_project_to_path_rejects_invalid_simulation_results_without_publishing() {
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
    let published = path.exists();
    let _ = std::fs::remove_file(&path);

    assert!(!saved);
    assert!(!published);
    assert!(
        state
            .log_buffer
            .entries()
            .any(|entry| { entry.message.contains("Project save failed") })
    );
}
