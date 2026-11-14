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

/// One `.STEP` point's analysis of `kind`, naming `c` at its point.
fn point(
    id: u64,
    kind: AnalysisType,
    index: usize,
    c: f64,
    waveforms: Vec<RetainedTrace>,
) -> AnalysisResult<RetainedTrace> {
    use rspice_results::provenance::AnalysisResultStepCoordinate;
    let member = AnalysisResultFamilyMember::StepPoint {
        index,
        coordinate: vec![AnalysisResultStepCoordinate {
            name: "c".to_owned(),
            value: c,
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
    AnalysisResult::new(id, kind, format!("Sweep · {}", member.label()), 0.0)
        .with_waveforms(waveforms)
        .with_provenance(provenance)
}

#[test]
fn a_stepped_dc_sweep_draws_its_points_as_one_family_on_one_strip() {
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    for (index, scale) in [1.0, 2.0, 3.0].into_iter().enumerate() {
        run.analyses.push(point(
            index as u64 + 1,
            AnalysisType::DcSweep,
            index,
            scale,
            vec![RetainedTrace::new(
                "I(VD)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, scale, scale],
                "A",
            )],
        ));
    }
    let built = models(&run, &HashSet::new());
    let [strip] = built.as_slice() else {
        panic!("one strip for the family, got {}", built.len());
    };
    assert_eq!(strip.subtitle, "Sweep · 3 points");
    let names: Vec<_> = strip
        .traces
        .iter()
        .map(|trace| trace.name.as_str())
        .collect();
    assert_eq!(names, ["I(VD) · c = 1", "I(VD) · c = 2", "I(VD) · c = 3"]);
    for (index, trace) in strip.traces.iter().enumerate() {
        assert_eq!(trace.family_group_ordinal, Some(index));
        assert!(Arc::ptr_eq(&trace.y, &run.analyses[index].waveforms[0].y));
    }
}

#[test]
fn a_stepped_ac_sweep_draws_each_points_decibels_and_phase() {
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    for (index, (c, corner_magnitude)) in [(1e-9, 0.1), (1e-8, 0.01)].into_iter().enumerate() {
        run.analyses.push(point(
            index as u64 + 1,
            AnalysisType::Ac,
            index,
            c,
            vec![
                RetainedTrace::new("|V(out)|", vec![1.0, 1e3], vec![1.0, corner_magnitude], ""),
                RetainedTrace::new(
                    "phase(V(out))",
                    vec![1.0, 1e3],
                    vec![0.0, -45.0 * (index + 1) as f64],
                    "°",
                ),
            ],
        ));
    }
    // A member of another analysis kind is not a member of this family.
    run.analyses.push(point(
        3,
        AnalysisType::DcSweep,
        0,
        1.0,
        vec![RetainedTrace::new(
            "V(out)",
            vec![0.0, 1.0],
            vec![0.0, 1.0],
            "V",
        )],
    ));
    let built = models(&run, &HashSet::new());
    assert_eq!(built.len(), 2, "the AC family and the DC point");
    let decibels: Vec<_> = built[0]
        .traces
        .iter()
        .filter(|trace| trace.kind == TraceKind::MagnitudeDb)
        .map(|trace| (trace.name.as_str(), trace.y[1]))
        .collect();
    assert_eq!(
        decibels,
        [
            ("|V(out)| · c = 1 n", -20.0),
            ("|V(out)| · c = 10 n", -40.0)
        ]
    );
    let phases: Vec<_> = built[0]
        .traces
        .iter()
        .filter(|trace| trace.kind == TraceKind::PhaseDeg)
        .map(|trace| trace.y[1])
        .collect();
    assert_eq!(phases, [-45.0, -90.0]);
}

/// One run of an imported LTspice `stepped` rawfile: no prepared task, the
/// run the file states on its import source.
fn imported_run(index: usize, member: AnalysisResultFamilyMember) -> AnalysisResult<RetainedTrace> {
    use rspice_results::result_import::{ResultImportFormat, ResultImportSource};
    let mut analysis = AnalysisResult::new(
        index as u64 + 1,
        AnalysisType::Transient,
        member.titled("Imported · Transient Analysis"),
        0.0,
    )
    .with_waveforms(vec![RetainedTrace::new(
        "V(out)",
        vec![0.0, 1.0],
        vec![0.0, 1.0 / (index + 1) as f64],
        "",
    )]);
    analysis.import_source = Some(ResultImportSource {
        source_name: "rc_step.raw".to_owned(),
        format: ResultImportFormat::SpiceRaw,
        coordinate: None,
        family_member: Some(member),
    });
    analysis
}

#[test]
fn the_runs_of_an_imported_stepped_rawfile_draw_as_one_family() {
    use rspice_results::provenance::AnalysisResultStepCoordinate;
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    for (index, r) in [1000.0, 2000.0, 4000.0].into_iter().enumerate() {
        let member = AnalysisResultFamilyMember::StepPoint {
            index,
            coordinate: vec![AnalysisResultStepCoordinate {
                name: "r".to_owned(),
                value: r,
            }],
        };
        run.analyses.push(imported_run(index, member));
    }
    let built = models(&run, &HashSet::new());
    let [strip] = built.as_slice() else {
        panic!("one strip for the runs, got {}", built.len());
    };
    assert_eq!(strip.subtitle, "Imported · Transient Analysis · 3 points");
    let names: Vec<_> = strip
        .traces
        .iter()
        .map(|trace| trace.name.as_str())
        .collect();
    assert_eq!(
        names,
        ["V(out) · r = 1 k", "V(out) · r = 2 k", "V(out) · r = 4 k"]
    );
}

#[test]
fn runs_no_log_names_are_numbered_from_one_as_their_points() {
    let mut run = SimulationRun::new(1, 0.0, ExecutionTarget::LocalDesktop);
    for index in 0..2 {
        let member = AnalysisResultFamilyMember::StepPoint {
            index,
            coordinate: Vec::new(),
        };
        run.analyses.push(imported_run(index, member));
    }
    assert_eq!(
        run.analyses[0].label,
        "Imported · Transient Analysis · point 1"
    );
    let built = models(&run, &HashSet::new());
    let [strip] = built.as_slice() else {
        panic!("one strip for the runs, got {}", built.len());
    };
    let names: Vec<_> = strip
        .traces
        .iter()
        .map(|trace| trace.name.as_str())
        .collect();
    assert_eq!(names, ["V(out) · point 1", "V(out) · point 2"]);
}
