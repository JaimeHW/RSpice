//! Commands reaching the Results workspace and the viewers inside it.
//!
//! A results route is gated on materialized evidence, not on the workspace
//! being open: a viewer with no dataset behind it reports unavailable and does
//! not navigate. Addressability is the second claim — every registered sheet
//! is reachable by a stable id, so it can be bound to a shortcut or driven
//! from automation rather than only clicked.

use super::*;

#[test]
fn edit_specifications_opens_the_real_results_editor() {
    let mut app = RSpiceApp::test_instance();

    Command::EditSpecifications.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        app.state.ui.results.viewer,
        crate::workbench::ResultViewer::Specs
    );
    assert!(app.state.ui.results.spec_drafts.is_some());
}

#[test]
fn generic_results_command_opens_the_workspace_without_a_dataset() {
    let mut app = RSpiceApp::test_instance();
    let command = Command::ResultViewer(crate::workbench::ResultViewer::Waves);

    assert!(command.is_enabled(&app));
    assert_eq!(command.availability(&app), CommandAvailability::Available);
    command.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        app.state.ui.results.viewer,
        crate::workbench::ResultViewer::Waves
    );
}

#[test]
fn incompatible_result_viewer_command_is_disabled_and_cannot_navigate() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Project;
    let command = Command::ResultViewer(crate::workbench::ResultViewer::Bode);

    assert!(!command.is_enabled(&app));
    // Bode admits the complete set of frequency-response families its
    // unit-aware renderer can present; ordinary-noise spectra remain on the
    // Noise viewer that owns them.
    assert_eq!(
        command.availability(&app),
        CommandAvailability::Disabled(
            "Requires a usable AC, PAC, PXF, STB, or distortion response"
        )
    );
    command.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Project);
    assert_eq!(
        app.state.ui.results.viewer,
        crate::workbench::ResultViewer::Waves
    );
    assert!(
        app.state
            .log_buffer
            .entries()
            .any(|message| message.message.contains("cannot be opened"))
    );
}

#[test]
fn exposed_results_calculator_opens_the_real_editor_dialog() {
    let mut app = RSpiceApp::test_instance();
    assert!(!app.state.dialogs.waveform_calculator_dialog);

    Command::WaveformCalculator.execute(&mut app);

    assert!(app.state.dialogs.waveform_calculator_dialog);
}

#[test]
fn truthful_results_menu_routes_keep_their_stable_dispatch_identities() {
    for (command, stable_id) in [
        (
            Command::ResultViewer(crate::workbench::ResultViewer::Waves),
            "waveforms",
        ),
        (Command::DatasetManifestBrowser, "dataset-browser"),
        (Command::CreateResultDocument, "create-result-document"),
        (Command::CompareResultDatasets, "compare-datasets"),
        (Command::VisualizationTraceManager, "trace-manager"),
        (Command::VisualizationCursorManager, "cursor-manager"),
        (Command::ReviewNotes, "annotation-manager"),
        (Command::WaveformCalculator, "calculator"),
        (Command::MeasurementLibrary, "measurement-library"),
        (Command::FamilySlicing, "family-slicing"),
        (Command::VisualizationDocumentProperties, "plot-properties"),
        (Command::ImportResultDataset, "import-dataset"),
        (Command::ExportWaveformsCsv, "export-results"),
        (Command::ReportAuthoring, "report-page-editor"),
    ] {
        assert_eq!(command.stable_id(), stable_id);
        assert_eq!(Command::from_stable_id(stable_id), Some(command));
        assert!(vocabulary::COMMAND_REGISTRY.contains(&command));
        assert!(command_catalog().any(|candidate| candidate == command));
    }

    for unavailable in [
        Command::SaveReportDocument,
        Command::AddReportPage,
        Command::ReportPageProperties,
    ] {
        assert!(!command_catalog().any(|candidate| candidate == unavailable));
    }
}

/// A sheet reachable only by clicking its tab is reachable only when the strip
/// happens to draw that tab. The registry is what puts a command in the
/// palette, in the shortcut editor, and within reach of `from_stable_id` — so a
/// viewer missing from it cannot be bound, searched, or restored from a saved
/// profile. Driven by `ResultViewer::every()` on purpose: the hand-written list
/// this replaced went stale for five of the sheets.
#[test]
fn every_result_sheet_is_registered_for_commands_and_shortcuts() {
    use crate::workbench::ResultViewer;

    for viewer in ResultViewer::every() {
        let command = Command::ResultViewer(viewer);
        assert!(
            vocabulary::COMMAND_REGISTRY.contains(&command),
            "Result sheet is absent from the command registry: {viewer:?}"
        );
        assert_eq!(Command::from_stable_id(command.stable_id()), Some(command));
    }
}

#[test]
fn split_results_is_truthfully_gated_by_context_and_materialized_evidence() {
    let mut app = RSpiceApp::test_instance();
    let command = Command::ToggleResultsSplit;
    app.state.workbench.activate(Workspace::Design);

    assert_eq!(
        command.availability(&app),
        CommandAvailability::Disabled("no retained result dataset is available")
    );
    app.state
        .simulation
        .start_run()
        .add_analysis(crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::Transient,
            "retained TRAN",
        ));
    assert_eq!(command.availability(&app), CommandAvailability::Available);

    app.state.workbench.activate(Workspace::Results);
    assert_eq!(
        command.availability(&app),
        CommandAvailability::Disabled("open Design, Netlist, or Simulation setup")
    );
}

#[test]
fn enabling_split_selects_latest_materialized_run_without_copying_results() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    let retained_dataset = {
        let run = app.state.simulation.start_run();
        run.add_analysis(crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::Transient,
            "retained TRAN",
        ));
        run.dataset_id
    };
    app.state.simulation.start_run();
    assert_eq!(app.state.simulation.active_run_idx, Some(0));
    let history_len = app.state.simulation.runs.len();

    Command::ToggleResultsSplit.execute(&mut app);

    assert!(app.state.workbench.split_with_results);
    assert_eq!(app.state.simulation.active_run_idx, Some(1));
    assert_eq!(
        app.state.simulation.active_run().map(|run| run.dataset_id),
        Some(retained_dataset)
    );
    assert_eq!(
        app.state.simulation.runs.len(),
        history_len,
        "the split projects the canonical dataset instead of cloning it"
    );

    Command::ToggleResultsSplit.execute(&mut app);
    assert!(!app.state.workbench.split_with_results);
}

#[test]
fn result_dataset_import_has_mockup_authoritative_command_identity_and_gates() {
    assert_eq!(Command::ImportResultDataset.stable_id(), "import-dataset");
    assert_eq!(
        Command::ImportResultDataset.spec().label,
        "Import result dataset…"
    );
    assert_eq!(Command::ImportResultDataset.spec().group, "Results");
    assert!(vocabulary::COMMAND_REGISTRY.contains(&Command::ImportResultDataset));

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    assert!(Command::ImportResultDataset.is_enabled(&app));
    app.state.simulation.is_running = true;
    assert!(!Command::ImportResultDataset.is_enabled(&app));
}
