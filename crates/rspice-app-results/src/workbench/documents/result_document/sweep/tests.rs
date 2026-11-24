//! The Sweep sheet's plan over a retained family: what it reads from a
//! stepped run, and the analyses it refuses as no sweep.

use super::*;
use crate::state::{AnalysisResult, AnalysisType, RetainedTrace, SimulationRun};
use crate::workbench::documents::result_document::test_host::ResultsTestHost;
use rspice_results::provenance::{
    AnalysisResultFamilyMember, AnalysisResultProvenance, AnalysisResultStepCoordinate,
};
use rspice_vocabulary::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};

/// The gain of a divider whose lower leg is 1 kΩ and upper leg `r`.
fn divider_gain(r: f64) -> f64 {
    1e3 / (r + 1e3)
}

/// One `.STEP` point of a transient over `r`: `V(out)` settles at the
/// divider's gain, which `.MEAS gain` also states.
fn point(index: usize, r: f64) -> AnalysisResult {
    let member = AnalysisResultFamilyMember::StepPoint {
        index,
        coordinate: vec![AnalysisResultStepCoordinate {
            name: "r".to_owned(),
            value: r,
            unit: String::new(),
        }],
    };
    let provenance = AnalysisResultProvenance::new(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([7; 32]),
        Vec::new(),
    )
    .unwrap()
    .with_family_member(Some(member.clone()))
    .unwrap();
    let gain = divider_gain(r);
    AnalysisResult::new(
        index as u64 + 1,
        AnalysisType::Transient,
        member.titled("Transient"),
    )
    .with_waveforms(vec![RetainedTrace::new(
        "V(out)",
        vec![0.0, 1e-3, 2e-3],
        vec![0.0, gain, gain],
        "#0af",
    )])
    .with_measurements(vec![rspice_core::MeasureResult {
        name: "gain".to_owned(),
        value: Some(gain),
        raw_value: Some(gain),
        error: None,
        passed: true,
        expected: None,
        tolerance: None,
        failure_limit: None,
        failure_limit_exceeded: false,
        event_axis: None,
        units: None,
    }])
    .with_provenance(provenance)
}

fn state_with(analyses: Vec<AnalysisResult>) -> ResultsTestHost {
    let mut run = SimulationRun::new(1);
    for analysis in analyses {
        run.add_analysis(analysis);
    }
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));
    state.model.simulation.view.active_analysis_idx = Some(1);
    state
}

/// A stepped transient's points are a sweep: the sheet reads each point's
/// `.MEAS` against its stepped value, and a signal at the end of each
/// point's own record.
#[test]
fn a_stepped_transient_reads_its_gain_against_the_stepped_value() {
    let mut state = state_with(vec![point(0, 1e3), point(1, 2e3), point(2, 4e3)]);
    assert!(shown_is_sweep(state.results_reads()));
    let mut parts = state.results_parts();
    let mut context = SheetContext::of(&mut parts);
    let plan = plan(&mut context).expect("the family as a table");
    assert_eq!(plan.table.rows, ["r=1k", "r=2k", "r=4k"]);
    let gains = drawn_values(
        context.simulation,
        &plan.table,
        &context.results.session.sweep,
    );
    let signal = drawn_values(
        context.simulation,
        &plan.table,
        &view::SweepSheetState {
            output: Some("V(out)".to_owned()),
            ..Default::default()
        },
    );
    for ((gain, end), r) in gains.iter().zip(&signal).zip([1e3, 2e3, 4e3]) {
        assert_eq!(
            gain.map(|gain| format!("{gain:.6}")),
            Some(format!("{:.6}", divider_gain(r)))
        );
        assert_eq!(*end, *gain);
    }
    // The table is read once per retained generation.
    let again = super::plan(&mut context).expect("the same table");
    assert!(Arc::ptr_eq(&plan, &again));
}

/// A lone transient is no sweep, so the sheet is not offered for it.
#[test]
fn a_lone_point_is_no_sweep() {
    let state = state_with(vec![
        AnalysisResult::new(1, AnalysisType::Transient, "Transient"),
        point(0, 1e3),
    ]);
    assert!(!shown_is_sweep(state.results_reads()));
}
