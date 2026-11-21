//! A plan's working view keeps what the reader made of its runs: through a
//! re-run, a restart, and a save and reopen.

use crate::workbench::app_state::AppState;
use rspice_app_results::workbench::documents::result_document::{
    AnalysisPresentationKey, MarkerKind, MarkerPlacement, WavePanePresentationKey,
    commit_marker_edit, place_marker, waves::source_waveform_anchor, working_view,
};
use rspice_app_runs::simulation::controller::SimulationController;
use rspice_app_runs::simulation::controller::test_execution::state_with_current_clean_drc;
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_results::feature_search::{Anchor, nearest};
use rspice_results::result_presentation::SourceWaveformPresentationKey;

const TIME: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];
const FIRST_OUT: [f64; 5] = [0.0, 0.4, 1.0, 0.4, 0.0];
const RERUN_OUT: [f64; 5] = [0.0, 0.2, 0.4, 1.0, 0.1];

/// Run the session's plan once, its transient answering `out`, and finish
/// the run.
fn run_plan(state: &mut AppState, out: [f64; 5]) {
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
                .with_waveforms(vec![
                    RetainedTrace::new("V(in)", TIME.to_vec(), vec![1.0; 5], "#ffffff"),
                    RetainedTrace::new("V(out)", TIME.to_vec(), out.to_vec(), "#ffffff"),
                ]),
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

/// The hidden input, the doubled output, the log axis and the peak marker
/// are all on the quick view of `analysis`.
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
