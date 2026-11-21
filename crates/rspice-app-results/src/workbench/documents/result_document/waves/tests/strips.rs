//! The strips the reader arranges a waveform stack into: kept by the run's
//! view, across a re-run of its plan and a restart, and stepped back by Undo.

use super::*;
use crate::workbench::documents::result_document::ResultsTestHost;
use crate::workbench::documents::result_document::test_fixtures::{completed_prepared_run, digest};
use crate::workbench::documents::result_document::view_edits;
use rspice_results_ui::waves::strip_layout::{self, Curve};

/// A run of `plan` holding a 5 V rail and the 2 mV ripple on it, solved
/// for the plan's analysis `analysis`.
fn planned_run(
    id: u64,
    plan: crate::product::SimulationPlanId,
    analysis: crate::product::AnalysisInstanceId,
) -> crate::state::SimulationRun {
    let mut run =
        completed_prepared_run(plan, crate::product::ObjectRevision::INITIAL, digest(0x40));
    run.id = id;
    let x = vec![0.0, 0.5e-3, 1.0e-3];
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![
                RetainedTrace::new("V(rail)", x.clone(), vec![5.0, 5.0, 5.0], "#ffffff"),
                RetainedTrace::new("V(ripple)", x, vec![0.0, 2.0e-3, -2.0e-3], "#ffffff"),
            ])
            .with_provenance(
                crate::state::AnalysisResultProvenance::new(
                    analysis,
                    crate::product::ObjectRevision::INITIAL,
                    digest(0x07),
                    Vec::new(),
                )
                .expect("analysis provenance"),
            ),
    );
    run
}

fn models(state: &mut ResultsTestHost) -> Arc<Vec<StripModel>> {
    let presentation = state.model.preferences.result_presentation_policy();
    cached_models(
        &state.model.simulation,
        &mut state.results.results,
        presentation.complex_number_display(),
        &Tokens::default(),
    )
}

/// The trace names each strip of the shown analysis draws.
fn strips_drawn(state: &mut ResultsTestHost) -> Vec<Vec<String>> {
    let models = models(state);
    let model = &models[0];
    model
        .unit_panes()
        .iter()
        .map(|pane| {
            pane.all_traces()
                .map(|index| model.traces[index].name.clone())
                .collect()
        })
        .collect()
}

/// Drop the ripple into the gap under the rail's strip, as one drag does.
fn drag_ripple_apart(state: &mut ResultsTestHost) {
    let models = models(state);
    let model = &models[0];
    let ripple = model
        .traces
        .iter()
        .position(|trace| trace.name == "V(ripple)")
        .expect("the ripple");
    let strips = strip_layout::move_to_gap(model, Curve::Trace(ripple), 1).expect("one drag");
    state
        .results
        .results
        .session
        .strip_layouts
        .insert(model.analysis_key, strips);
}

#[test]
fn strips_survive_a_rerun_and_a_restart_and_undo_puts_the_last_ones_back() {
    let plan = crate::product::SimulationPlanId::new();
    let analysis = crate::product::AnalysisInstanceId::new();
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![planned_run(1, plan, analysis)].into();
    assert!(state.model.simulation.select_run(0));
    state.quick_frame();
    assert_eq!(strips_drawn(&mut state), [["V(rail)", "V(ripple)"]]);

    drag_ripple_apart(&mut state);
    state.quick_frame();
    let kept: usize = state
        .working_view()
        .expect("the run's view")
        .panes()
        .iter()
        .map(|pane| pane.strips.len())
        .sum();
    assert_eq!(kept, 2, "the view keeps both strips");
    assert_eq!(strips_drawn(&mut state), [["V(rail)"], ["V(ripple)"]]);

    // The plan runs again: its view draws the new run in the same strips.
    state
        .model
        .simulation
        .retained
        .runs
        .insert(0, planned_run(2, plan, analysis));
    // The history lists the newest first, so the shown index is the new run.
    state.model.simulation.select_run(0);
    state.quick_frame();
    assert_eq!(strips_drawn(&mut state), [["V(rail)"], ["V(ripple)"]]);

    let mut reopened = state.reopened();
    reopened.quick_frame();
    assert_eq!(
        strips_drawn(&mut reopened),
        [["V(rail)"], ["V(ripple)"]],
        "the strips come back with the project"
    );

    let view = state.working_view().expect("the run's view").id();
    view_edits::step(&mut state.results_parts(), view, view_edits::Step::Undo)
        .expect("the drag is undone");
    state.quick_frame();
    assert_eq!(strips_drawn(&mut state), [["V(rail)", "V(ripple)"]]);
}
