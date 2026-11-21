//! A marker's number is never handed out again, through a save and a
//! reopen: the view that keeps the marker keeps the numbers it gave out.

use super::*;
use rspice_app_results::workbench::documents::result_document::{
    MarkerPlacement, place_marker, remove_marker, waves::source_waveform_anchor, working_view,
};
use rspice_app_services::workbench::lifecycle::project_lifecycle::{
    DestinationAuthority, SaveScope, has_unsaved_changes, save_native,
};
use rspice_results_ui::session::MarkerSelector;

fn retained_results() -> AppState {
    let mut state = AppState::default();
    let mut run = rspice_app_state::state::SimulationRun::new(1);
    run.add_analysis(
        rspice_app_state::state::AnalysisResult::new(
            1,
            rspice_app_state::state::AnalysisType::Transient,
            "Tran",
        )
        .with_waveforms(vec![rspice_app_state::state::RetainedTrace::new(
            "V(out)",
            vec![0.0, 1.0],
            vec![1.0, 2.0],
            "#ffffff",
        )]),
    );
    seal_legacy_unattributed(&mut run);
    run.mark_running().unwrap();
    run.finish_lifecycle(rspice_app_state::state::SimulationRunLifecycle::Completed)
        .unwrap();
    state.model.simulation.retained.runs.push(run);
    assert!(state.model.simulation.select_run(0));
    state
        .workbench
        .activate(rspice_app_state::workbench::workspace::Workspace::Results);
    state
}

/// Draw the quick view's frame around nothing: its run's working view is
/// made or followed, drawn, and kept.
fn quick_frame(state: &mut AppState) {
    let mut parts = state.results_parts();
    working_view::sync(&mut parts);
    working_view::capture(&mut parts);
}

fn mark(state: &mut AppState, x: f64) -> MarkerSelector {
    quick_frame(state);
    let (analysis, anchor) =
        source_waveform_anchor(&mut state.results_parts(), "V(out)").expect("V(out) is drawn");
    place_marker(
        &mut state.results_parts(),
        MarkerPlacement {
            analysis,
            anchor,
            trace_name: "V(out)".to_owned(),
            x,
            samples: &vec![0.0, 1.0].into(),
        },
    )
    .expect("the working view keeps the marker")
}

#[test]
fn a_removed_markers_number_is_not_handed_out_again_after_save_and_reopen() {
    let path = std::env::temp_dir().join(format!(
        "rspice-marker-numbers-{}.rspiceproj",
        uuid::Uuid::new_v4()
    ));
    let mut state = retained_results();
    mark(&mut state, 0.25);
    let highest = mark(&mut state, 0.5);
    remove_marker(&mut state.results_parts(), highest);
    save_native(
        &mut state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .unwrap();
    assert!(!has_unsaved_changes(&state));
    drop(state);

    let mut reopened = AppState::default();
    assert!(reopened.load_project_from_path(&path));
    assert!(reopened.model.simulation.select_run(0));
    quick_frame(&mut reopened);
    assert!(
        !has_unsaved_changes(&reopened),
        "drawing the reopened run's view changes nothing"
    );
    let next = mark(&mut reopened, 0.75);
    assert!(
        next.marker_id.get() > highest.marker_id.get(),
        "a removed marker's number was handed out again after a reopen"
    );
    assert_eq!(reopened.results.results.session.document_markers.len(), 2);
    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.into_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
}
