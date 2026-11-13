use std::collections::HashSet;

use rspice_results::analysis_type::AnalysisType;
use rspice_results::executed_deck::ExecutedDeckArchive;
use rspice_results::provenance::AnalysisResultProvenance;
use rspice_results::run::{ExecutionTarget, SimulationRun};
use rspice_ui_kit::tokens::Tokens;
use rspice_vocabulary::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};

use super::*;
use crate::waves::{ProjectionOptions, StripModel, build_models};

/// One trial's transient: its own time grid and its own `V(out)`.
fn trial(id: u64, index: usize, time: Vec<f64>, out: Vec<f64>) -> AnalysisResult<RetainedTrace> {
    let provenance = AnalysisResultProvenance::new(
        AnalysisInstanceId::new(),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([7; 32]),
        Vec::new(),
    )
    .unwrap()
    .with_family_member(Some(AnalysisResultFamilyMember::MonteCarloTrial { index }))
    .unwrap();
    AnalysisResult::new(
        id,
        AnalysisType::Transient,
        format!("Transient · trial {index}"),
        0.0,
    )
    .with_waveforms(vec![RetainedTrace::new("V(out)", time, out, "")])
    .with_provenance(provenance)
}

fn models(
    run: &SimulationRun<AnalysisResult<RetainedTrace>>,
    hidden: &HashSet<FamilyTraceVisibilityKey>,
) -> Vec<StripModel> {
    build_models(
        &[run],
        &ExecutedDeckArchive::default(),
        &mut DerivedSeries::default(),
        &Tokens::default(),
        ProjectionOptions {
            phase_continuous: false,
            complex_display: Default::default(),
            ac: Default::default(),
            selection: None,
            hidden_family_traces: hidden,
        },
        |_| None,
    )
}

#[test]
fn a_monte_carlo_transient_draws_its_trials_as_one_family_on_one_strip() {
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    // Retained out of trial order, on different grids, as members finish.
    run.analyses
        .push(trial(1, 1, vec![0.0, 0.5, 1.0], vec![0.0, 0.6, 1.1]));
    run.analyses
        .push(trial(2, 0, vec![0.0, 1.0], vec![0.0, 1.0]));
    run.analyses
        .push(trial(3, 2, vec![0.0, 0.25, 1.0], vec![0.0, 0.2, 0.9]));
    let built = models(&run, &HashSet::new());

    let [strip] = built.as_slice() else {
        panic!("one strip for the family, got {}", built.len());
    };
    assert_eq!(strip.analysis_index, 1, "hosted by the first trial");
    assert_eq!(strip.subtitle, "Transient · 3 trials");
    let names: Vec<_> = strip
        .traces
        .iter()
        .map(|trace| trace.name.as_str())
        .collect();
    assert_eq!(
        names,
        ["V(out) · trial 0", "V(out) · trial 1", "V(out) · trial 2"]
    );
    for (ordinal, (trace, analysis)) in strip.traces.iter().zip([1, 0, 2]).enumerate() {
        assert_eq!(trace.family_group_ordinal, Some(ordinal));
        assert!(trace.visible && !trace.overlay);
        // Each curve is the member's own retained samples, never a copy.
        let source = &run.analyses[analysis].waveforms[0];
        assert!(Arc::ptr_eq(&trace.x, &source.x) && Arc::ptr_eq(&trace.y, &source.y));
    }
    let keys: HashSet<_> = strip
        .traces
        .iter()
        .map(|trace| trace.presentation_key)
        .collect();
    assert_eq!(keys.len(), 3, "every trial is its own family group");

    // The strip's per-member control hides one trial and only that one.
    let hidden = HashSet::from([strip.traces[1].family_visibility_key.unwrap()]);
    let built = models(&run, &hidden);
    let visible: Vec<_> = built[0].traces.iter().map(|trace| trace.visible).collect();
    assert_eq!(visible, [true, false, true]);
}

#[test]
fn a_transient_that_names_no_member_keeps_its_own_strip() {
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    run.analyses
        .push(trial(1, 0, vec![0.0, 1.0], vec![0.0, 1.0]));
    run.analyses.push(
        AnalysisResult::new(2, AnalysisType::Transient, "Transient", 0.0).with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![1.0, 2.0], ""),
        ]),
    );
    let built = models(&run, &HashSet::new());
    assert_eq!(built.len(), 2);
    assert_eq!(built[1].traces[0].family_group_ordinal, None);
    assert_eq!(built[0].subtitle, "Transient · 1 trial");
}
