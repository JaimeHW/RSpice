//! A plan's working view keeps what the reader made of its runs: through a
//! re-run, a restart, and a save and reopen. The runs keep only what they
//! were solved with.

use crate::workbench::app_state::AppState;
use rspice_app_results::workbench::documents::result_document::{
    AnalysisPresentationKey, MarkerKind, MarkerPlacement, WavePanePresentationKey,
    commit_marker_edit, place_marker, set_trace_color, waves::source_waveform_anchor, working_view,
};
use rspice_app_runs::simulation::controller::SimulationController;
use rspice_app_runs::simulation::controller::test_execution::state_with_current_clean_drc;
use rspice_app_state::results::visualization_document::{ResultDocumentTrackingMode, TraceColor};
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_results::feature_search::{Anchor, nearest};
use rspice_results::result_presentation::SourceWaveformPresentationKey;

mod large_runs;

const TIME: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];
const FIRST_OUT: [f64; 5] = [0.0, 0.4, 1.0, 0.4, 0.0];
const RERUN_OUT: [f64; 5] = [0.0, 0.2, 0.4, 1.0, 0.1];
const ORANGE: TraceColor = TraceColor::from_rgb(0xff, 0x88, 0x00);

/// Run the session's plan once, its transient answering `out`, and finish
/// the run.
fn run_plan(state: &mut AppState, out: [f64; 5]) {
    run_plan_with(
        state,
        vec![
            RetainedTrace::new("V(in)", TIME.to_vec(), vec![1.0; 5], "#ffffff"),
            RetainedTrace::new("V(out)", TIME.to_vec(), out.to_vec(), "#ffffff"),
        ],
    );
}

/// Run the session's plan once, its transient answering `waveforms`, and
/// finish the run.
pub(super) fn run_plan_with(state: &mut AppState, waveforms: Vec<RetainedTrace>) {
    let mut controller = SimulationController::new();
    controller
        .preflight_run_set(state)
        .expect("clean plan preflight");
    state.model.simulation.execution.request_simulate_run_set();
    controller.start_simulation(state);
    let provenance = controller
        .current_provenance
        .clone()
        .expect("the plan's prepared task");
    state
        .model
        .simulation
        .active_run_mut()
        .expect("the plan's run")
        .add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
                .with_provenance(provenance)
                .with_waveforms(waveforms),
        );
    controller.abort();
    state
        .model
        .simulation
        .active_run_mut()
        .expect("the plan's run")
        .finish_lifecycle(SimulationRunLifecycle::Completed)
        .expect("the run finishes");
}

/// The quick view's frame around nothing: the shown run's working view is
/// made or followed, drawn, and kept.
pub(super) fn quick_frame(state: &mut AppState) {
    let mut parts = state.results_parts();
    working_view::sync(&mut parts);
    working_view::capture(&mut parts);
    // The traces of a large gesture, made away from the frame, are taken
    // in before the test reads the view.
    working_view::settle(&mut parts);
}

/// Show the newest run, first in the history, as Results does when a run
/// finishes.
pub(super) fn show_newest(state: &mut AppState) -> AnalysisPresentationKey {
    assert!(state.model.simulation.select_run(0));
    let run = state.model.simulation.active_run().expect("the newest run");
    AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
}

/// The hidden input, the doubled output, the output's colour, the log axis
/// and the peak marker are all on the quick view of `analysis`; every run
/// keeps the colour it was solved with.
fn assert_kept(state: &AppState, analysis: AnalysisPresentationKey, phase: &str) {
    let session = &state.results.results.session;
    assert_eq!(
        session
            .waveform_visibility
            .get(&SourceWaveformPresentationKey::new(analysis, "V(in)")),
        Some(&false),
        "{phase}: the hidden trace stays hidden"
    );
    assert_eq!(
        session
            .waveform_colors
            .get(&SourceWaveformPresentationKey::new(analysis, "V(out)")),
        Some(&ORANGE),
        "{phase}: the output keeps the colour the reader gave it"
    );
    for run in state.model.simulation.retained.runs.iter() {
        for waveform in &run.analyses[0].waveforms {
            assert_eq!(waveform.color, "#ffffff", "{phase}: the run is as solved");
        }
    }
    assert_eq!(
        session
            .analysis_exprs
            .get(&analysis)
            .map(|expressions| expressions.iter().map(|e| e.text.as_str()).collect()),
        Some(vec!["V(out)*2"]),
        "{phase}: the expression stays"
    );
    assert!(
        session.log_y_panes.contains(&WavePanePresentationKey {
            analysis,
            unit: "V".to_owned(),
        }),
        "{phase}: the log axis stays"
    );
    let markers = session
        .document_markers
        .iter()
        .filter(|marker| marker.analysis == analysis)
        .collect::<Vec<_>>();
    assert_eq!(markers.len(), 1, "{phase}: the marker stays");
    assert_eq!(
        markers[0].kind,
        MarkerKind::Peak(Default::default()),
        "{phase}"
    );
    assert_eq!(
        nearest(&TIME, &RERUN_OUT, markers[0].x, Anchor::Peak { size: 0.0 }).map(|found| found.x),
        Some(0.75),
        "{phase}: the peak marker finds the re-run's peak"
    );
    let views = state
        .model
        .workspace
        .content
        .visualization_documents
        .iter()
        .filter(|document| document.working_view().is_some())
        .count();
    assert_eq!(views, 1, "{phase}: the plan has one working view");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_working_view_keeps_the_readers_changes_through_a_rerun_a_restart_and_a_reopen() {
    let host = state_with_current_clean_drc();
    let mut state = AppState::default();
    state.model = host.model;
    state.runs = host.runs;
    state.workbench.activate(Workspace::Results);

    run_plan(&mut state, FIRST_OUT);
    let first = show_newest(&mut state);
    quick_frame(&mut state);
    let session = &mut state.results.results.session;
    session
        .waveform_visibility
        .insert(SourceWaveformPresentationKey::new(first, "V(in)"), false);
    session.log_y_panes.insert(WavePanePresentationKey {
        analysis: first,
        unit: "V".to_owned(),
    });
    state
        .results
        .results
        .add_expression_trace(&state.model.simulation, first, "V(out)*2".to_owned())
        .expect("the expression plots");
    quick_frame(&mut state);
    let (analysis, anchor) =
        source_waveform_anchor(&mut state.results_parts(), "V(out)").expect("V(out) is drawn");
    assert_eq!(analysis, first);
    let marker = place_marker(
        &mut state.results_parts(),
        MarkerPlacement {
            analysis,
            anchor,
            trace_name: "V(out)".to_owned(),
            x: 0.5,
            samples: &TIME.to_vec().into(),
        },
    )
    .expect("the working view keeps the marker");
    commit_marker_edit(
        &mut state.results_parts(),
        marker,
        "V(out)",
        MarkerKind::Peak(Default::default()),
    )
    .expect("the marker becomes a peak marker");
    set_trace_color(&mut state.results_parts(), first, "V(out)", Some(ORANGE))
        .expect("the working view keeps the output's colour");
    quick_frame(&mut state);

    run_plan(&mut state, RERUN_OUT);
    let rerun = show_newest(&mut state);
    assert_ne!(rerun, first, "the re-run is another dataset");
    quick_frame(&mut state);
    assert_kept(&state, rerun, "after the re-run");

    let mut storage = super::two_windows::MemoryStorage::default();
    state.store_session(&mut storage);
    drop(state);
    let mut state = AppState::restore_eframe_session(&storage).expect("the session restores");
    let rerun = show_newest(&mut state);
    quick_frame(&mut state);
    assert_kept(&state, rerun, "after a restart");

    let path = std::env::temp_dir().join(format!(
        "rspice-working-view-{}.rspiceproj",
        uuid::Uuid::new_v4()
    ));
    assert!(state.save_project_to_path(&path), "the project saves");
    drop(state);
    let mut state = AppState::default();
    let reopened = state.load_project_from_path(&path);
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.into_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
    assert!(reopened, "the project reopens");
    let rerun = show_newest(&mut state);
    quick_frame(&mut state);
    assert_kept(&state, rerun, "after a save and reopen");
}

/// The two notes on the quick view of `analysis`: one on the output's curve
/// at X 0.5, wherever a re-run takes the curve, and one at a point of the
/// axes.
fn assert_notes(state: &AppState, analysis: AnalysisPresentationKey, phase: &str) {
    use rspice_results_ui::session::NoteAnchor;
    let notes: Vec<_> = state
        .results
        .results
        .session
        .document_notes
        .iter()
        .filter(|note| note.analysis == analysis)
        .map(|note| (note.text.as_str(), &note.anchor))
        .collect();
    assert_eq!(notes.len(), 2, "{phase}: both notes stay: {notes:?}");
    assert!(
        matches!(
            notes[0],
            ("Peak", NoteAnchor::Trace { trace_name, x, .. }) if trace_name == "V(out)" && *x == 0.5
        ),
        "{phase}: the note on the curve stays on it: {notes:?}"
    );
    assert_eq!(
        notes[1],
        (
            "Knee",
            &NoteAnchor::Data {
                unit: "V".to_owned(),
                x: 0.25,
                y: 0.75,
            }
        ),
        "{phase}: the note on the axes stays at its point"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_note_keeps_its_place_through_a_zoom_a_rerun_and_a_restart() {
    use rspice_app_results::workbench::documents::result_document::notes::edit_note;
    use rspice_results_ui::session::NoteAnchor;
    use rspice_results_ui::waves::notes::NoteEdit;

    let host = state_with_current_clean_drc();
    let mut state = AppState::default();
    state.model = host.model;
    state.runs = host.runs;
    state.workbench.activate(Workspace::Results);

    run_plan(&mut state, FIRST_OUT);
    let first = show_newest(&mut state);
    quick_frame(&mut state);
    let (analysis, anchor) =
        source_waveform_anchor(&mut state.results_parts(), "V(out)").expect("V(out) is drawn");
    assert_eq!(analysis, first);
    let on_curve = NoteAnchor::Trace {
        anchor,
        trace_name: "V(out)".to_owned(),
        x: 0.5,
    };
    let on_axes = NoteAnchor::Data {
        unit: "V".to_owned(),
        x: 0.25,
        y: 0.75,
    };
    for (anchor, text) in [(on_curve, "Peak"), (on_axes, "Knee")] {
        edit_note(
            &mut state.results_parts(),
            NoteEdit::Add {
                analysis,
                anchor,
                text: text.to_owned(),
            },
        );
    }
    quick_frame(&mut state);
    assert_notes(&state, first, "as written");

    rspice_results_ui::waves::stack::set_shared_x_view(
        &mut state.results.results.session,
        first,
        Some((0.375, 0.625)),
    );
    quick_frame(&mut state);
    assert_notes(&state, first, "after a zoom");

    run_plan(&mut state, RERUN_OUT);
    let rerun = show_newest(&mut state);
    quick_frame(&mut state);
    assert_notes(&state, rerun, "after the re-run");

    let mut storage = super::two_windows::MemoryStorage::default();
    state.store_session(&mut storage);
    drop(state);
    let mut state = AppState::restore_eframe_session(&storage).expect("the session restores");
    let rerun = show_newest(&mut state);
    quick_frame(&mut state);
    assert_notes(&state, rerun, "after a restart");
}

/// The view of `state`'s plan, which every run of it is drawn through.
fn plan_view(state: &AppState) -> rspice_app_state::product::ResultDocumentId {
    let run = state.model.simulation.active_run().expect("a shown run");
    working_view::view_of_run(state.results_reads(), run.dataset_id).expect("the plan's view")
}

/// Whether the shown analysis draws `V(in)`.
fn input_shown(state: &AppState, analysis: AnalysisPresentationKey) -> Option<bool> {
    state
        .results
        .results
        .session
        .waveform_visibility
        .get(&SourceWaveformPresentationKey::new(analysis, "V(in)"))
        .copied()
}

/// An older run is shown through the same view, which stays on it; a
/// document saved from the view then stays on that run too, while the view
/// goes back to following the plan's newest run.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_older_run_is_shown_through_the_same_view_and_back_to_latest() {
    let host = state_with_current_clean_drc();
    let mut state = AppState::default();
    state.model = host.model;
    state.runs = host.runs;
    state.workbench.activate(Workspace::Results);
    run_plan(&mut state, FIRST_OUT);
    let first_run = state.model.simulation.retained.runs[0].dataset_id;
    run_plan(&mut state, RERUN_OUT);
    show_newest(&mut state);
    quick_frame(&mut state);
    let view = plan_view(&state);
    let rerun = show_newest(&mut state);
    state
        .results
        .results
        .session
        .waveform_visibility
        .insert(SourceWaveformPresentationKey::new(rerun, "V(in)"), false);
    quick_frame(&mut state);

    working_view::choose_run(&mut state.results_parts(), view, Some(first_run))
        .expect("the older run is one of the plan's");
    quick_frame(&mut state);
    let shown = state.model.simulation.active_run().expect("a shown run");
    assert_eq!(shown.dataset_id, first_run, "the older run is shown");
    let first = AnalysisPresentationKey::new(shown.dataset_id, &shown.analyses[0]);
    assert_eq!(plan_view(&state), view, "through the same view");
    assert_eq!(
        input_shown(&state, first),
        Some(false),
        "the hidden trace stays hidden"
    );
    let document = |state: &AppState, id| {
        state
            .model
            .workspace
            .content
            .visualization_document(id)
            .expect("the document")
            .clone()
    };
    assert_eq!(
        document(&state, view).tracking().mode,
        ResultDocumentTrackingMode::Pinned
    );

    let saved = working_view::save_as_document(&mut state.results_parts(), view, "Older run")
        .expect("the view is saved as a document");
    let saved = document(&state, saved);
    assert_eq!(saved.working_view(), None);
    assert_eq!(saved.title(), "Older run");
    assert_eq!(saved.tracking().mode, ResultDocumentTrackingMode::Pinned);
    assert_eq!(saved.traces(), document(&state, view).traces());

    working_view::choose_run(&mut state.results_parts(), view, None)
        .expect("the view follows the plan again");
    quick_frame(&mut state);
    let shown = state.model.simulation.active_run().expect("a shown run");
    assert_ne!(shown.dataset_id, first_run, "the newest run is shown again");
    assert_eq!(
        document(&state, view).tracking().mode,
        ResultDocumentTrackingMode::Latest
    );
    assert_eq!(input_shown(&state, rerun), Some(false));
}

/// The signals the plan's view plots, and whether each is drawn.
pub(super) fn view_plots(state: &AppState) -> Vec<(String, bool)> {
    let view = plan_view(state);
    let mut plots: Vec<_> = state
        .model
        .workspace
        .content
        .visualization_document(view)
        .expect("the plan's view")
        .traces()
        .iter()
        .map(|trace| (trace.label.clone(), trace.visible))
        .collect();
    plots.sort();
    plots
}

/// The first run plots the outputs the plan marks Plot and leaves the rest
/// in the browser; a re-run that adds an output marked Plot plots it too,
/// while an output the reader took off stays off.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_run_plots_the_outputs_its_plan_marks_and_a_rerun_plots_a_new_one() {
    let output = |name: &str, plot: bool| {
        let mut waveform = RetainedTrace::new(name, TIME.to_vec(), FIRST_OUT.to_vec(), "#ffffff");
        waveform.visible = plot;
        waveform
    };
    let host = state_with_current_clean_drc();
    let mut state = AppState::default();
    state.model = host.model;
    state.runs = host.runs;
    state.workbench.activate(Workspace::Results);
    run_plan_with(
        &mut state,
        vec![
            output("V(in)", true),
            output("V(out)", true),
            output("I(R1)", false),
        ],
    );
    let first = show_newest(&mut state);
    quick_frame(&mut state);
    assert_eq!(
        view_plots(&state),
        [("V(in)".to_owned(), true), ("V(out)".to_owned(), true)],
        "the first run plots what the plan marks, and only that"
    );
    state
        .results
        .results
        .session
        .waveform_visibility
        .insert(SourceWaveformPresentationKey::new(first, "V(in)"), false);
    quick_frame(&mut state);

    run_plan_with(
        &mut state,
        vec![
            output("V(in)", true),
            output("V(out)", true),
            output("I(R1)", false),
            output("V(mid)", true),
        ],
    );
    show_newest(&mut state);
    quick_frame(&mut state);
    assert_eq!(
        view_plots(&state),
        [
            ("V(in)".to_owned(), false),
            ("V(mid)".to_owned(), true),
            ("V(out)".to_owned(), true)
        ],
        "the new output is plotted and the one taken off stays off"
    );
}

/// A plan's run opens on the sheet its plot arrives on: while it has written
/// nothing yet, a reader left on another analysis' sheet is taken to the
/// Waves sheet its transient draws on, where the sheet says the run is
/// starting.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_starting_run_opens_on_the_sheet_its_plot_arrives_on() {
    use rspice_app_results::workbench::documents::result_document::{
        ResultViewer, prepare_viewer_state,
    };
    let host = state_with_current_clean_drc();
    let mut state = AppState::default();
    state.model = host.model;
    state.runs = host.runs;
    state.workbench.activate(Workspace::Results);
    let mut controller = SimulationController::new();
    controller
        .preflight_run_set(&state)
        .expect("clean plan preflight");
    state.model.simulation.execution.request_simulate_run_set();
    controller.start_simulation(&mut state);
    let run = state
        .model
        .simulation
        .active_run()
        .expect("the starting run");
    assert!(run.analyses.is_empty(), "the run has written nothing yet");
    assert!(!run.lifecycle.is_terminal());
    state.results.results.session.viewer = ResultViewer::Bode;

    prepare_viewer_state(&mut state.results_parts());

    assert_eq!(state.results.results.session.viewer, ResultViewer::Waves);
    controller.abort();
}

/// A tick the reader makes is one step of the view to undo: undoing it puts
/// the view and the quick view back as they were before it, and redoing it
/// takes the tick again.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_undo_after_a_tick_puts_the_quick_view_back() {
    use rspice_app_workbench::workbench::framework::command_registry::WorkbenchCommand;
    let host = state_with_current_clean_drc();
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.model = host.model;
    app.state.runs = host.runs;
    app.state.workbench.activate(Workspace::Results);
    run_plan(&mut app.state, FIRST_OUT);
    // Shown from the data browser, as a reader shows a run.
    assert!(
        rspice_app_results::workbench::docks::navigator::results_actions::select_result_dataset(
            &mut app.state.results_parts(),
            0
        )
    );
    let analysis = {
        let run = app
            .state
            .model
            .simulation
            .active_run()
            .expect("the shown run");
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
    };
    quick_frame(&mut app.state);
    let before = view_plots(&app.state);
    assert_eq!(input_shown(&app.state, analysis), None);

    // Taken off as the data browser takes it off: the reader's own setting.
    app.state.results.results.session.waveform_visibility.set(
        SourceWaveformPresentationKey::new(analysis, "V(in)"),
        Some(false),
    );
    quick_frame(&mut app.state);
    assert!(
        view_plots(&app.state).contains(&("V(in)".to_owned(), false)),
        "the view takes the tick"
    );
    assert_eq!(input_shown(&app.state, analysis), Some(false));

    assert!(app.command_key_available(WorkbenchCommand::Undo.key()));
    app.execute_command_key(WorkbenchCommand::Undo.key());
    quick_frame(&mut app.state);
    assert_eq!(
        view_plots(&app.state),
        before,
        "the undo puts the view back"
    );
    assert_eq!(
        input_shown(&app.state, analysis),
        None,
        "the quick view draws the input as its run does again"
    );

    app.execute_command_key(WorkbenchCommand::Redo.key());
    quick_frame(&mut app.state);
    assert_eq!(
        input_shown(&app.state, analysis),
        Some(false),
        "the redo takes it off again"
    );
}

/// An edit of the view made outside the quick view, as another pane makes
/// one, is drawn on the quick view's next frame, and a frame after it
/// changes nothing more.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_edit_of_the_view_made_elsewhere_reaches_the_quick_view() {
    use rspice_app_state::results::visualization_document::DocumentEdit;
    let host = state_with_current_clean_drc();
    let mut state = AppState::default();
    state.model = host.model;
    state.runs = host.runs;
    state.workbench.activate(Workspace::Results);
    run_plan(&mut state, FIRST_OUT);
    let analysis = show_newest(&mut state);
    quick_frame(&mut state);

    let view = plan_view(&state);
    let content = &mut state.model.workspace.content;
    let document = content
        .visualization_document(view)
        .expect("the plan's view");
    let output = document
        .traces()
        .iter()
        .find(|trace| trace.label == "V(out)")
        .expect("the output is plotted");
    let edits = vec![
        DocumentEdit::SetSignalsPlotted {
            pane_id: output.pane_id,
            signals: vec!["V(in)".to_owned()],
            plotted: false,
        },
        DocumentEdit::SetTraceColor {
            trace_id: output.id,
            color: Some(ORANGE),
        },
    ];
    let revision = document.revision();
    content
        .transact_visualization_document(view, revision, edits)
        .expect("the edit commits");
    quick_frame(&mut state);

    let output = SourceWaveformPresentationKey::new(analysis, "V(out)");
    assert_eq!(
        input_shown(&state, analysis),
        Some(false),
        "the hide is drawn"
    );
    assert_eq!(
        state.results.results.session.waveform_colors.get(&output),
        Some(&ORANGE),
        "the colour is drawn"
    );
    let revision = state
        .model
        .workspace
        .content
        .visualization_document(view)
        .map(|document| document.revision());
    quick_frame(&mut state);
    assert_eq!(input_shown(&state, analysis), Some(false));
    assert_eq!(
        state
            .model
            .workspace
            .content
            .visualization_document(view)
            .map(|document| document.revision()),
        revision,
        "drawing the edit edits the view no further"
    );
}

/// A tick is unsaved work, and so is its undo: the undo puts the view's
/// traces back, but the view a save keeps moved on to a new revision, so the
/// project still holds work the save did not. The frame reads each from the
/// view's revision, without capturing the project again.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_tick_and_its_undo_are_unsaved_work() {
    use rspice_app_services::workbench::lifecycle::project_lifecycle::{
        has_unsaved_changes_fresh, snapshot, unsaved_work,
    };
    use rspice_app_workbench::workbench::framework::command_registry::WorkbenchCommand;
    let host = state_with_current_clean_drc();
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.model = host.model;
    app.state.runs = host.runs;
    app.state.workbench.activate(Workspace::Results);
    run_plan(&mut app.state, FIRST_OUT);
    // Shown from the data browser, as a reader shows a run.
    assert!(
        rspice_app_results::workbench::docks::navigator::results_actions::select_result_dataset(
            &mut app.state.results_parts(),
            0
        )
    );
    let analysis = {
        let run = app
            .state
            .model
            .simulation
            .active_run()
            .expect("the shown run");
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
    };
    quick_frame(&mut app.state);
    let saved = snapshot(&app.state).expect("the project as saved");
    crate::workbench::app_state::services_access::accept_loaded_project(
        &mut app.state,
        saved,
        None,
    );
    quick_frame(&mut app.state);
    let unsaved = |app: &crate::workbench::RSpiceApp| {
        let (unsaved, _) = unsaved_work(&app.state);
        assert_eq!(
            unsaved,
            has_unsaved_changes_fresh(&app.state),
            "the frame's answer is the one a capture gets"
        );
        unsaved
    };
    assert!(!unsaved(&app), "the project as saved holds no unsaved work");
    // The frame's answer, and the captures of the project it took.
    let read_by_the_frame = |app: &mut crate::workbench::RSpiceApp| {
        let captures = || rspice_project::registry::PROJECT_CAPTURES.with(std::cell::Cell::get);
        let before = captures();
        quick_frame(&mut app.state);
        let (unsaved, _) = unsaved_work(&app.state);
        (unsaved, captures() - before)
    };

    app.state.results.results.session.waveform_visibility.set(
        SourceWaveformPresentationKey::new(analysis, "V(in)"),
        Some(false),
    );
    assert_eq!(
        read_by_the_frame(&mut app),
        (true, 0),
        "the tick reads unsaved from the view's revision, with no capture"
    );
    assert_eq!(input_shown(&app.state, analysis), Some(false));
    assert!(unsaved(&app), "the tick is unsaved work");

    app.execute_command_key(WorkbenchCommand::Undo.key());
    assert_eq!(
        read_by_the_frame(&mut app),
        (true, 0),
        "the undo reads unsaved from the view's revision, with no capture"
    );
    assert_eq!(input_shown(&app.state, analysis), None, "the undo is drawn");
    assert!(
        unsaved(&app),
        "the undo leaves the view at a revision the save did not keep"
    );
}

/// In a project never saved, a tick is unsaved work, and so is its undo, as
/// they are against a save: the view moved on from the revision the project
/// started with.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_tick_in_a_project_never_saved_and_its_undo_are_unsaved_work() {
    use rspice_app_services::workbench::lifecycle::project_lifecycle::{
        has_unsaved_changes_fresh, mark_pristine, unsaved_work,
    };
    use rspice_app_workbench::workbench::framework::command_registry::WorkbenchCommand;
    let host = state_with_current_clean_drc();
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.model = host.model;
    app.state.runs = host.runs;
    app.state.workbench.activate(Workspace::Results);
    run_plan(&mut app.state, FIRST_OUT);
    assert!(
        rspice_app_results::workbench::docks::navigator::results_actions::select_result_dataset(
            &mut app.state.results_parts(),
            0
        )
    );
    let analysis = {
        let run = app
            .state
            .model
            .simulation
            .active_run()
            .expect("the shown run");
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
    };
    quick_frame(&mut app.state);
    // What the project started from: the run, shown.
    mark_pristine(&mut app.state);
    quick_frame(&mut app.state);
    let unsaved = |app: &crate::workbench::RSpiceApp| {
        let (unsaved, _) = unsaved_work(&app.state);
        assert_eq!(
            unsaved,
            has_unsaved_changes_fresh(&app.state),
            "the frame's answer is the one a capture gets"
        );
        unsaved
    };
    assert!(!unsaved(&app), "the project holds what it started from");

    app.state.results.results.session.waveform_visibility.set(
        SourceWaveformPresentationKey::new(analysis, "V(in)"),
        Some(false),
    );
    quick_frame(&mut app.state);
    assert_eq!(input_shown(&app.state, analysis), Some(false));
    assert!(unsaved(&app), "the tick is unsaved work");

    app.execute_command_key(WorkbenchCommand::Undo.key());
    quick_frame(&mut app.state);
    assert_eq!(input_shown(&app.state, analysis), None, "the undo is drawn");
    assert!(
        unsaved(&app),
        "the undo leaves the view at a revision the project did not start with"
    );
}
