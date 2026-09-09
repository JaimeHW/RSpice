//! Physical probe semantics through prepared solver execution.
//! Ground has no solved unknown; numeric-looking node names do. AC differences
//! must subtract rectangular components before computing display magnitudes.

use super::fixtures::{output, run};
use super::*;
use crate::state::{OutputSelectionMode, SimulationRun};

const DECK: &str =
    "Grounded probes\nV1 pos 0 DC 2 AC 1 0\nV2 neg 0 DC -1 AC 1 180\nR1 pos neg 1k\n.end\n";

fn dc(nested: bool, retraced: bool) -> AnalysisSpec {
    AnalysisSpec::DcSweep {
        source_name: "V1".to_owned(),
        start: -1.0,
        stop: 1.0,
        step: 1.0,
        source2: nested.then(|| "V2".to_owned()),
        start2: nested.then_some(-1.0),
        stop2: nested.then_some(0.0),
        step2: nested.then_some(1.0),
        hysteresis: retraced,
    }
}

fn ac() -> AnalysisSpec {
    AnalysisSpec::Ac {
        start_freq: 1.0,
        stop_freq: 10.0,
        points_per_unit: 3,
        sweep: FrequencySweep::Linear,
    }
}

fn execute(spec: AnalysisSpec, outputs: &[SavedOutput]) -> SimulationRun {
    let line = match &spec {
        AnalysisSpec::DcOp { .. } => ".op",
        AnalysisSpec::Ac { .. } => ".ac lin 3 1 10",
        AnalysisSpec::Transient { .. } => ".tran 1u 10u",
        _ => ".dc V1 -1 1 1",
    };
    run(
        DECK,
        "Physical probes",
        spec,
        line,
        outputs,
        OutputSelectionMode::ExplicitOnly,
    )
}

fn materialized<'a>(run: &'a SimulationRun, name: &str, count: usize) -> Vec<&'a WaveformData> {
    run.validate_provenance().unwrap();
    assert!(run.success);
    assert_eq!(run.analyses.len(), 1);
    let analysis = &run.analyses[0];
    assert!(analysis.success, "{:?}", analysis.error_message);
    let receipt = analysis
        .saved_output_receipts
        .iter()
        .find(|r| r.name == name)
        .unwrap();
    let names = receipt
        .status
        .materialized_waveforms()
        .map(|(name, _)| name)
        .collect::<Vec<_>>();
    assert_eq!(names.len(), count, "{name}: {:?}", receipt.status);
    names
        .into_iter()
        .map(|name| analysis.waveforms.iter().find(|w| w.name == name).unwrap())
        .collect()
}

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-10, "{actual} != {expected}");
}

fn assert_ground(spec: AnalysisSpec, members: usize) {
    let is_ac = matches!(spec, AnalysisSpec::Ac { .. });
    let is_dc = matches!(spec, AnalysisSpec::DcSweep { .. });
    let outputs = [
        ("Positive", "V(pos,0)"),
        ("Negative", "V(0,pos)"),
        ("Ground", "V(0)"),
        ("Ground difference", "V(0,0)"),
    ]
    .map(|(name, expression)| output(SavedOutputKind::RawVoltageOrCurrent, name, expression));
    let run = execute(spec, &outputs);
    for (name, sign) in [
        ("Positive", 1.0),
        ("Negative", -1.0),
        ("Ground", 0.0),
        ("Ground difference", 0.0),
    ] {
        for wave in materialized(&run, name, members) {
            assert_eq!(wave.unit.as_deref(), Some("V"));
            for (&x, &y) in wave.x.iter().zip(wave.y.iter()) {
                assert_close(
                    y,
                    if is_ac {
                        f64::abs(sign)
                    } else {
                        sign * if is_dc { x } else { 2.0 }
                    },
                );
            }
            if is_ac {
                let complex = wave
                    .complex
                    .as_ref()
                    .expect("an AC ground reference retains phase");
                for (&real, &imag) in complex.real.iter().zip(complex.imag.iter()) {
                    assert_close(real, sign);
                    assert_close(imag, 0.0);
                }
            }
        }
    }
}

#[test]
fn operating_point_ground_probes_use_the_solved_scalar_basis() {
    assert_ground(AnalysisSpec::dc_op(), 1);
}

#[test]
fn dc_ground_probes_use_the_solved_primary_axis() {
    assert_ground(dc(false, false), 1);
}

#[test]
fn nested_ground_probes_retain_every_member() {
    assert_ground(dc(true, false), 2);
}

#[test]
fn retraced_ground_probes_retain_both_traversals() {
    assert_ground(dc(false, true), 2);
}

#[test]
fn transient_ground_probes_use_accepted_times() {
    assert_ground(
        AnalysisSpec::Transient {
            stop_time: 1e-5,
            step_time: 1e-6,
            start_time: 0.0,
            max_timestep: Some(1e-6),
            uic: false,
        },
        1,
    );
}

#[test]
fn ac_ground_probes_preserve_polarity_as_phase() {
    assert_ground(ac(), 1);
}

#[test]
fn ac_differential_magnitude_comes_from_the_complex_difference() {
    let run = execute(
        ac(),
        &[output(
            SavedOutputKind::RawVoltageOrCurrent,
            "Difference",
            "V(pos,neg)",
        )],
    );
    let wave = materialized(&run, "Difference", 1)[0];
    let complex = wave.complex.as_ref().unwrap();
    for ((&magnitude, &real), &imag) in wave
        .y
        .iter()
        .zip(complex.real.iter())
        .zip(complex.imag.iter())
    {
        assert_close(real, 2.0);
        assert_close(imag, 0.0);
        assert_close(magnitude, real.hypot(imag));
    }
}

#[test]
fn authoring_accepted_probe_spacing_executes_in_scalar_and_family_results() {
    for (spec, members) in [(AnalysisSpec::dc_op(), 1), (dc(true, false), 2)] {
        let run = execute(
            spec,
            &[output(
                SavedOutputKind::RawVoltageOrCurrent,
                "Spaced",
                " V ( pos ) ",
            )],
        );
        for wave in materialized(&run, "Spaced", members) {
            assert_eq!(wave.unit.as_deref(), Some("V"));
        }
    }
}

#[test]
fn derived_ground_probes_execute_in_scalar_and_family_results() {
    for (spec, members, is_dc) in [
        (AnalysisSpec::dc_op(), 1, false),
        (dc(true, false), 2, true),
    ] {
        let run = execute(
            spec,
            &[output(
                SavedOutputKind::DerivedExpression,
                "Derived",
                "V(pos) - V(0)",
            )],
        );
        for wave in materialized(&run, "Derived", members) {
            for (&x, &y) in wave.x.iter().zip(wave.y.iter()) {
                assert_close(y, if is_dc { x } else { 2.0 });
            }
        }
    }
}

#[test]
fn numeric_looking_node_names_are_not_numeric_values() {
    let deck = "Literal node identities\nV1 00 0 2\nV2 001 0 3\nV3 1k 0 4\nR1 00 0 1k\nR2 001 0 1k\nR3 1k 0 1k\n.end\n";
    let outputs = [
        output(SavedOutputKind::RawVoltageOrCurrent, "Raw", "V(00)"),
        output(
            SavedOutputKind::DerivedExpression,
            "Sum",
            "V(00) + V(001) + V(1k)",
        ),
    ];
    let run = run(
        deck,
        "Literal nodes",
        AnalysisSpec::dc_op(),
        ".op",
        &outputs,
        OutputSelectionMode::ExplicitOnly,
    );
    assert_close(materialized(&run, "Raw", 1)[0].y[0], 2.0);
    assert_close(materialized(&run, "Sum", 1)[0].y[0], 9.0);
}

#[test]
fn absent_non_ground_nodes_remain_unavailable() {
    for spec in [AnalysisSpec::dc_op(), dc(true, false), ac()] {
        let outputs = ["V(pos,missing)", "V(missing,0)", "V(missing)"]
            .map(|expression| output(SavedOutputKind::RawVoltageOrCurrent, expression, expression));
        let run = execute(spec, &outputs);
        run.validate_provenance().unwrap();
        assert!(run.success);
        assert_eq!(run.analyses[0].saved_output_receipts.len(), outputs.len());
        assert!(
            run.analyses[0]
                .saved_output_receipts
                .iter()
                .all(|receipt| matches!(
                    receipt.status,
                    SavedOutputMaterializationStatus::Unavailable { .. }
                ))
        );
    }
}

#[test]
fn incomplete_ac_phase_evidence_is_rejected_for_differential_arithmetic() {
    let plain = WaveformData::new("V(pos)", vec![1.0, 2.0], vec![1.0, 1.0], "#fff").with_unit("V");
    let complex = WaveformData::new("V(neg)", vec![1.0, 2.0], vec![1.0, 1.0], "#fff")
        .with_unit("V")
        .with_complex_components("V(neg)", vec![-1.0, -1.0], vec![0.0, 0.0]);
    for require_complex in [false, true] {
        assert!(
            resolve_raw_probe(
                "V(pos,neg)",
                &[plain.clone(), complex.clone()],
                "diff",
                require_complex
            )
            .is_err()
        );
    }
    assert!(resolve_raw_probe("V(0,pos)", std::slice::from_ref(&plain), "diff", true).is_err());
    let mut negative = plain.clone();
    negative.name = "V(neg)".to_owned();
    assert!(resolve_raw_probe("V(pos,neg)", &[plain.clone(), negative], "diff", true).is_err());
    let malformed = plain.with_complex_components("V(pos)", vec![1.0], vec![0.0]);
    assert!(resolve_raw_probe("V(pos,neg)", &[malformed, complex], "diff", true).is_err());
    assert!(resolve_raw_probe("V(0)", &[], "ground", true).is_err());
}

#[test]
fn ground_and_spaced_probes_evaluate_after_project_reload() {
    for spec in [ac(), dc(true, false)] {
        let is_ac = matches!(spec, AnalysisSpec::Ac { .. });
        let mut output = output(
            SavedOutputKind::RawVoltageOrCurrent,
            "Deferred ground",
            " V ( 0, pos ) ",
        );
        assert_eq!(output.inferred_unit(), "volts");
        output.save_policy = SavedOutputPolicy::OnDemandFromRetainedState;
        output.stored_precision = SavedOutputPrecision::DisplayCacheWithFullSourcePrecision;
        let run = execute(spec, &[output]);
        run.validate_provenance().unwrap();
        let mut state = crate::state::SimulationState::default();
        state.next_run_id = run.id;
        state.runs = vec![run].into();
        let snapshot = crate::io::project_io::ProjectSimulationResults::from_state(&state);
        snapshot.validate().unwrap();
        let restored: crate::io::project_io::ProjectSimulationResults =
            serde_json::from_slice(&serde_json::to_vec(&snapshot).unwrap()).unwrap();
        restored.apply_to_state(&mut state).unwrap();
        let mut analysis = state.runs[0].analyses[0].clone();
        materialize_deferred_saved_output(&mut analysis, 0).unwrap();
        analysis.validate_retained_evidence().unwrap();
        let names = analysis.saved_output_receipts[0]
            .status
            .materialized_waveforms()
            .collect::<Vec<_>>();
        assert_eq!(names.len(), if is_ac { 1 } else { 2 });
        for (name, _) in names {
            let wave = analysis
                .waveforms
                .iter()
                .find(|wave| wave.name == name)
                .unwrap();
            assert!(wave.display_cache.is_some());
            assert_eq!(wave.unit.as_deref(), Some("V"));
            for (&x, &y) in wave.x.iter().zip(wave.y.iter()) {
                assert_close(y, if is_ac { 1.0 } else { -x });
            }
            if is_ac {
                assert!(
                    wave.complex
                        .as_ref()
                        .unwrap()
                        .real
                        .iter()
                        .all(|&real| (real + 1.0).abs() < 1e-10)
                );
            }
        }
    }
}
