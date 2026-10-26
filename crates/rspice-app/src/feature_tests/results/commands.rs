//! Commands reaching the Results workspace and the viewers inside it.
//!
//! A results route is gated on materialized evidence, not on the workspace
//! being open: a viewer with no dataset behind it reports unavailable and does
//! not navigate. Addressability is the second claim — every registered sheet
//! is reachable by a stable id, so it can be bound to a shortcut or driven
//! from automation rather than only clicked.

use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::{self, command_registry};
use crate::workbench::framework::command_registry::{CommandAvailability, CommandFlags};
use crate::workbench::hardcopy::commands::HardcopyCommand;
use crate::workbench::results::commands::ResultsCommand;
use crate::workbench::state::Workspace;

#[test]
fn edit_specifications_opens_the_real_results_editor() {
    let mut app = RSpiceApp::test_instance();

    app.execute_command_key(ResultsCommand::EditSpecifications.key());

    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        app.state.results.results.session.viewer,
        crate::workbench::ResultViewer::Specs
    );
    assert!(app.state.results.results.session.spec_drafts.is_some());
}

#[test]
fn generic_results_command_opens_the_workspace_without_a_dataset() {
    let mut app = RSpiceApp::test_instance();
    let command = ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Waves).key();

    assert!(app.command_key_enabled(command));
    assert_eq!(
        app.command_availability(command),
        CommandAvailability::Available
    );
    app.execute_command_key(command);

    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        app.state.results.results.session.viewer,
        crate::workbench::ResultViewer::Waves
    );
}

#[test]
fn incompatible_result_viewer_command_is_disabled_and_cannot_navigate() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Project;
    let command = ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Bode).key();

    assert!(!app.command_key_enabled(command));
    // Bode admits the complete set of frequency-response families its
    // unit-aware renderer can present; ordinary-noise spectra remain on the
    // Noise viewer that owns them.
    assert_eq!(
        app.command_availability(command),
        CommandAvailability::Disabled(
            "Requires a usable AC, PAC, PXF, STB, or distortion response"
        )
    );
    app.execute_command_key(command);

    assert_eq!(app.state.workbench.workspace, Workspace::Project);
    assert_eq!(
        app.state.results.results.session.viewer,
        crate::workbench::ResultViewer::Waves
    );
    assert_eq!(
        app.state.prompt_text(),
        None,
        "a disabled command is silent"
    );
}

#[test]
fn exposed_results_calculator_opens_the_real_editor_dialog() {
    let mut app = RSpiceApp::test_instance();
    assert!(!app.state.results.waveform_calculator_dialog);

    app.execute_command_key(ResultsCommand::WaveformCalculator.key());

    assert!(app.state.results.waveform_calculator_dialog);
}

#[test]
fn truthful_results_menu_routes_keep_their_stable_dispatch_identities() {
    let registry = command_registry();
    for (command, stable_id) in [
        (
            ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Waves),
            "waveforms",
        ),
        (ResultsCommand::DatasetManifestBrowser, "dataset-browser"),
        (
            ResultsCommand::CreateResultDocument,
            "create-result-document",
        ),
        (ResultsCommand::CompareResultDatasets, "compare-datasets"),
        (ResultsCommand::ToggleLinkedCursors, "toggle-linked-cursors"),
        (ResultsCommand::WaveformCalculator, "calculator"),
        (ResultsCommand::MeasurementLibrary, "measurement-library"),
        (ResultsCommand::FamilySlicing, "family-slicing"),
        (
            ResultsCommand::VisualizationDocumentProperties,
            "plot-properties",
        ),
        (ResultsCommand::ImportResultDataset, "import-dataset"),
        (ResultsCommand::ExportWaveformsCsv, "export-results"),
    ] {
        let key = command.key();
        assert_eq!(key.as_str(), stable_id);
        assert_eq!(registry.resolve(stable_id), Some(key));
        assert!(registry.order().contains(&key));
        assert!(
            registry
                .ordered()
                .any(|spec| spec.key == key && spec.flags.contains(CommandFlags::PALETTE_VISIBLE))
        );
    }

    let report = HardcopyCommand::ReportAuthoring.key();
    assert_eq!(report.as_str(), "report-page-editor");
    assert_eq!(registry.resolve("report-page-editor"), Some(report));
    assert!(registry.order().contains(&report));

    for unavailable in HardcopyCommand::REPORT_TOOLBAR {
        let key = unavailable.key();
        assert!(!registry.order().contains(&key), "{unavailable:?}");
        assert_eq!(registry.resolve(key.as_str()), None, "{unavailable:?}");
    }
}

/// The Studio's commands went with it. The cursor manager's slots became
/// the linked-cursors toggle, so a shortcut saved against its id keeps
/// working; the others resolve to nothing, and a profile keeps their
/// bindings as unknown rather than refusing to load.
#[test]
fn the_studio_commands_resolve_to_their_result_viewer_successors() {
    let registry = vocabulary::command_registry();
    for id in ["cursor-manager", "visualization-cursor-manager"] {
        assert_eq!(
            registry.resolve(id),
            Some(ResultsCommand::ToggleLinkedCursors.key())
        );
    }
    for id in [
        "visualization-studio",
        "trace-manager",
        "visualization-trace-manager",
        "annotation-manager",
        "visualization-export-document",
    ] {
        assert_eq!(registry.resolve(id), None, "{id}");
    }
    for (id, command) in [
        (
            "visualization-add-pane",
            ResultsCommand::AddVisualizationPane,
        ),
        ("measurement-library", ResultsCommand::MeasurementLibrary),
        ("family-slicing", ResultsCommand::FamilySlicing),
        (
            "plot-properties",
            ResultsCommand::VisualizationDocumentProperties,
        ),
        (
            "visualization-document-properties",
            ResultsCommand::VisualizationDocumentProperties,
        ),
        ("compare-datasets", ResultsCommand::CompareResultDatasets),
    ] {
        assert_eq!(registry.resolve(id), Some(command.key()), "{id}");
    }
}

/// A sheet reachable only by clicking its tab is reachable only when the strip
/// happens to draw that tab. The registry is what puts a command in the
/// palette, in the shortcut editor, and within reach of `from_stable_id` — so a
/// viewer missing from it cannot be bound, searched, or restored from a saved
/// profile. Driven by `ResultViewer::all()` on purpose: the hand-written list
/// this replaced went stale for five of the sheets.
#[test]
fn every_result_sheet_is_registered_for_commands_and_shortcuts() {
    use crate::workbench::ResultViewer;

    let registry = vocabulary::command_registry();
    for viewer in ResultViewer::all() {
        let key = ResultsCommand::ResultViewer(viewer).key();
        assert!(
            registry.order().contains(&key),
            "Result sheet is absent from the command registry: {viewer:?}"
        );
        assert_eq!(registry.resolve(key.as_str()), Some(key));
    }
}

#[test]
fn split_results_is_truthfully_gated_by_context_and_materialized_evidence() {
    let mut app = RSpiceApp::test_instance();
    let command = ResultsCommand::ToggleResultsSplit.key();
    app.state.workbench.activate(Workspace::Design);

    assert_eq!(
        app.command_availability(command),
        CommandAvailability::Disabled("no retained result dataset is available")
    );
    app.state
        .model
        .simulation
        .start_run()
        .add_analysis(crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::Transient,
            "retained TRAN",
        ));
    assert_eq!(
        app.command_availability(command),
        CommandAvailability::Available
    );

    app.state.workbench.activate(Workspace::Results);
    assert_eq!(
        app.command_availability(command),
        CommandAvailability::Disabled("open Design, Netlist, or Simulation setup")
    );
}

#[test]
fn enabling_split_selects_latest_materialized_run_without_copying_results() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    let retained_dataset = {
        let run = app.state.model.simulation.start_run();
        run.add_analysis(crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::Transient,
            "retained TRAN",
        ));
        run.dataset_id
    };
    app.state.model.simulation.start_run();
    assert_eq!(app.state.model.simulation.view.active_run_idx, Some(0));
    let history_len = app.state.model.simulation.retained.runs.len();

    app.execute_command_key(ResultsCommand::ToggleResultsSplit.key());

    assert!(app.state.workbench.split_with_results);
    assert_eq!(app.state.model.simulation.view.active_run_idx, Some(1));
    assert_eq!(
        app.state
            .model
            .simulation
            .active_run()
            .map(|run| run.dataset_id),
        Some(retained_dataset)
    );
    assert_eq!(
        app.state.model.simulation.retained.runs.len(),
        history_len,
        "the split projects the canonical dataset instead of cloning it"
    );

    app.execute_command_key(ResultsCommand::ToggleResultsSplit.key());
    assert!(!app.state.workbench.split_with_results);
}

#[test]
fn result_dataset_import_waits_for_an_idle_open_project() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    assert!(app.command_key_enabled(ResultsCommand::ImportResultDataset.key()));
    app.state.model.simulation.execution.is_running = true;
    assert!(!app.command_key_enabled(ResultsCommand::ImportResultDataset.key()));
}

/// "Export dataset…" needs the document its export resolves from.
///
/// The export writes the *displayed* result view, and that resolution starts
/// at the active Results document. The command was offered whenever any
/// dataset was retained, so pressing it with no result document open produced
/// a console warning instead of a file.
#[test]
fn exporting_the_dataset_requires_an_open_result_document() {
    use crate::state::{AnalysisResult, AnalysisType, RetainedTrace, SimulationRun};
    use crate::workbench::state::WorkspaceDocumentId;

    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    assert!(!app.command_key_enabled(ResultsCommand::ExportWaveformsCsv.key()));
    assert_eq!(
        app.command_availability(ResultsCommand::ExportWaveformsCsv.key()),
        CommandAvailability::Disabled("no result dataset is available")
    );

    let mut run = SimulationRun::new(7);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#4f81bd"),
        ]),
    );
    let dataset_id = run.dataset_id;
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.model.simulation.view.active_run_idx = Some(0);
    app.state.model.simulation.view.active_analysis_idx = Some(0);

    // Retained, but nothing open to export from: the refusal names what is
    // missing rather than claiming there is no dataset.
    assert!(!app.command_key_enabled(ResultsCommand::ExportWaveformsCsv.key()));
    assert_eq!(
        app.command_availability(ResultsCommand::ExportWaveformsCsv.key()),
        CommandAvailability::Disabled("open a result document in Results")
    );

    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::ResultDataset(dataset_id));
    assert!(app.command_key_enabled(ResultsCommand::ExportWaveformsCsv.key()));
    assert_eq!(
        app.command_availability(ResultsCommand::ExportWaveformsCsv.key()),
        CommandAvailability::Available
    );
}
