//! A difference from another run, drawn on the result's strip.

use super::*;
use crate::state::{RetainedTrace, SimulationRun};
use crate::workbench::documents::result_document::document_edit::{
    commit_open_dialog,
    compare::{comparison_marks, open_compare, settle_comparison},
};
use rspice_results_ui::document_dialogs::ResultDocumentDialog;

/// Two retained runs of one transient on one time grid, the first shown and
/// compared with the second within 100 mV and 10 %: the first stands 1 V
/// above the second from 1 ms to 2 ms and 2 V above it at 5 ms.
fn compared_runs() -> ResultsTestHost {
    let mut state = ResultsTestHost::default();
    let time = vec![0.0, 1.0e-3, 2.0e-3, 3.0e-3, 4.0e-3, 5.0e-3];
    let mut runs = Vec::new();
    for (id, out) in [(1, vec![1.0, 2.0, 2.0, 1.0, 1.0, 3.0]), (2, vec![1.0; 6])] {
        let mut run = SimulationRun::new(id);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                RetainedTrace::new("V(out)", time.clone(), out, "#fff"),
            ]),
        );
        runs.push(run);
    }
    state.model.simulation.retained.runs = runs.into();
    assert!(state.model.simulation.select_run(0));
    assert!(state.model.simulation.select_analysis(0));
    state.workbench.workspace = crate::workbench::state::Workspace::Results;
    let mut parts = state.results_parts();
    open_compare(&mut parts);
    let Some(ResultDocumentDialog::Compare(draft)) = parts.result_document_dialog.as_mut() else {
        panic!("Compare datasets opens its form");
    };
    draft.absolute_tolerance = "100m".to_owned();
    draft.relative_tolerance = "100m".to_owned();
    commit_open_dialog(&mut parts);
    settle_comparison(&mut parts);
    state
}

/// A difference from the baseline is drawn on the result's own times, says
/// which run it reads and how, and carries the band of the tolerance the
/// comparison judged it by, with the places the runs part.
#[test]
fn a_difference_from_the_baseline_states_its_run_and_carries_its_tolerance() {
    let mut state = compared_runs();
    let run = state.model.simulation.active_run().unwrap();
    let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    state
        .results
        .results
        .add_expression_trace(
            &state.model.simulation,
            key,
            "V(out) - V(out)@baseline".to_owned(),
        )
        .unwrap();
    let tokens = Tokens::default();
    let mut models = build_models(
        &state.model.simulation,
        &mut DerivedSeries::default(),
        &tokens,
        false,
        ComplexNumberDisplay::MagnitudePhaseDegrees,
        Default::default(),
        None,
        &HashSet::new(),
    );
    let curves = resolve_strip_exprs(&mut state.results_parts(), &models[0], &tokens);
    assert_eq!(curves.len(), 1, "{:?}", state.prompt_text());
    assert_eq!(curves[0].y.as_slice(), &[0.0, 1.0, 1.0, 0.0, 0.0, 2.0]);
    assert!(
        curves[0].label.ends_with(" · Run 2, linear X"),
        "{}",
        curves[0].label
    );

    // The difference is drawn apart, on a volt pane of its own directly
    // below the pane of V(out), which keeps its own axis.
    let session = &state.results.results.session;
    route_expressions(
        &mut models,
        &state.model.simulation,
        &session.analysis_exprs,
        &session.expression_units,
        |_, _| false,
    );
    let panes = models[0].unit_panes();
    let shape: Vec<(&str, usize, Vec<usize>)> = panes
        .iter()
        .map(|pane| {
            (
                pane.axis_unit(),
                pane.traces.len(),
                pane.expressions.clone(),
            )
        })
        .collect();
    assert_eq!(shape, [("V", 1, vec![]), ("V", 0, vec![curves[0].slot])]);

    // Each pane is asked with the expressions it draws, as the stack asks.
    let mut marks = |pane, expressions: &[rspice_results_ui::waves::pane::ResolvedExpr]| {
        comparison_marks(
            &mut state.results_parts(),
            &models[0],
            pane,
            expressions,
            &tokens,
        )
    };
    let (signals, difference) = (marks(&panes[0], &[]), marks(&panes[1], &curves));
    let places = |marks: &rspice_results_ui::waves::pane::ComparisonMarks| -> Vec<(f64, f64)> {
        marks
            .places
            .iter()
            .flat_map(|places| places.iter().copied())
            .collect()
    };
    // The places the runs part are shaded on both panes.
    assert_eq!(places(&signals), [(1.0e-3, 2.0e-3), (5.0e-3, 5.0e-3)]);
    assert_eq!(places(&difference), places(&signals));
    // The band is the difference's: 100 mV, and a tenth of the 1 V baseline.
    assert!(signals.tolerance.is_none());
    let band = difference
        .tolerance
        .expect("the difference carries its band");
    assert_eq!(band.x, curves[0].x.as_slice());
    assert!(
        band.half_width
            .iter()
            .all(|width| (width - 0.2).abs() < 1e-12),
        "{:?}",
        band.half_width
    );
}
