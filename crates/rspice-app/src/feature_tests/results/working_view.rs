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
fn run_plan_with(state: &mut AppState, waveforms: Vec<RetainedTrace>) {
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
fn quick_frame(state: &mut AppState) {
    let mut parts = state.results_parts();
    working_view::sync(&mut parts);
    working_view::capture(&mut parts);
}

/// Show the newest run, first in the history, as Results does when a run
/// finishes.
fn show_newest(state: &mut AppState) -> AnalysisPresentationKey {
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
    assert_eq!(markers[0].kind, MarkerKind::Peak, "{phase}");
    assert_eq!(
        nearest(&TIME, &RERUN_OUT, markers[0].x, Anchor::Peak).map(|found| found.x),
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
        MarkerKind::Peak,
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
fn view_plots(state: &AppState) -> Vec<(String, bool)> {
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
