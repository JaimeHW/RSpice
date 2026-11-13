//! Family measurements against closed forms on synthetic families whose
//! member values are known exactly.

use super::*;
use crate::analysis_type::AnalysisType;
use crate::calculator::parser;
use crate::family_measurements::{FamilyMeasurementEvidence, FamilyMemberMeasurements};
use crate::provenance::{
    AnalysisResultProvenance, AnalysisResultPvtPoint, AnalysisResultSourceDomain,
};
use rspice_vocabulary::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};

fn parse(expression: &str) -> CalculatorExpr {
    parser::try_parse(expression, |literal| {
        literal.parse::<f64>().map_err(|error| error.to_string())
    })
    .expect("the test expression parses")
}

/// A transient of `V(out)` that ramps from `start` to `end` over one second.
fn ramp(id: u64, label: &str, start: f64, end: f64) -> AnalysisResult {
    let x: Vec<f64> = (0..=100).map(|step| f64::from(step) / 100.0).collect();
    let y: Vec<f64> = x.iter().map(|t| start + (end - start) * t).collect();
    AnalysisResult::<RetainedWaveform>::new(id, AnalysisType::Transient, label, 0.0)
        .with_waveforms(vec![RetainedWaveform::new("V(out)", x, y)])
}

/// Points of one authored transient, each solved as its own task.
fn expanded(members: &[(f64, f64)]) -> Vec<AnalysisResult> {
    let authored = AnalysisInstanceId::new();
    members
        .iter()
        .enumerate()
        .map(|(index, (start, end))| {
            let provenance = AnalysisResultProvenance::new_with_authored_source_domain(
                AnalysisResultSourceDomain::SimulationPlan,
                AnalysisInstanceId::new(),
                authored,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([7; 32]),
                Vec::new(),
            )
            .expect("valid provenance")
            .with_pvt_point(Some(
                AnalysisResultPvtPoint::new("tt", None, 25.0 * index as f64, None, index == 0)
                    .expect("valid point"),
            ));
            ramp(index as u64, &format!("point {index}"), *start, *end).with_provenance(provenance)
        })
        .collect()
}

fn measure(analyses: &[AnalysisResult], expression: &str) -> FamilyMeasurement {
    let members = family_members(&analyses[0], analyses).expect("an expanded family");
    evaluate_members(&parse(expression), &members)
}

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1e-12 * expected.abs().max(1.0),
        "{actual} != {expected}"
    );
}

#[test]
fn each_member_of_an_expanded_family_is_measured_on_its_own_waveforms() {
    // avg of a ramp from a to b over the whole run is (a + b) / 2.
    let family = expanded(&[(0.0, 2.0), (1.0, 3.0), (2.0, 4.0), (3.0, 5.0), (4.0, 6.0)]);
    let measured = measure(&family, "avg(V(out))");
    let values: Vec<f64> = measured
        .members
        .iter()
        .map(|m| *m.value.as_ref().unwrap())
        .collect();
    for (value, expected) in values.iter().zip([1.0, 2.0, 3.0, 4.0, 5.0]) {
        close(*value, expected);
    }
    // Each member is named by the point it was solved at.
    assert!(
        measured.members[1].label.contains("25"),
        "{}",
        measured.members[1].label
    );

    // Mean 3; sample variance (4 + 1 + 0 + 1 + 4) / (5 - 1) = 2.5.
    let statistics = measured.statistics().expect("five values");
    assert_eq!((statistics.count, statistics.unmeasured), (5, 0));
    close(statistics.mean, 3.0);
    close(statistics.std_dev.unwrap(), 2.5_f64.sqrt());
    close(statistics.min, 1.0);
    close(statistics.max, 5.0);
}

#[test]
fn the_standard_deviation_is_the_sample_one() {
    // The textbook set: population sigma 2, sample sigma sqrt(32 / 7).
    let members = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    let measured = FamilyMeasurement {
        members: members
            .iter()
            .enumerate()
            .map(|(index, value)| MemberValue {
                index,
                label: format!("Trial {index}"),
                value: Ok(*value),
            })
            .collect(),
    };
    let statistics = measured.statistics().unwrap();
    close(statistics.mean, 5.0);
    close(statistics.std_dev.unwrap(), (32.0_f64 / 7.0).sqrt());

    // One value has no spread to state.
    let single = FamilyMeasurement {
        members: measured.members[..1].to_vec(),
    };
    assert_eq!(single.statistics().unwrap().std_dev, None);
}

#[test]
fn a_yield_counts_the_members_inside_the_limits_and_names_the_rest() {
    let family = expanded(&[(0.0, 2.0), (1.0, 3.0), (2.0, 4.0), (3.0, 5.0), (4.0, 6.0)]);
    let measured = measure(&family, "avg(V(out))");

    let window = measured.yield_against(Some(1.5), Some(4.5)).unwrap();
    assert_eq!((window.passed, window.total), (3, 5));
    assert_eq!(window.failing, vec![0, 4]);
    close(window.percent(), 60.0);

    // A bound is inclusive, as every specification in the product reads it.
    let lower = measured.yield_against(Some(2.0), None).unwrap();
    assert_eq!((lower.passed, lower.failing.clone()), (4, vec![0]));
    let upper = measured.yield_against(None, Some(2.0)).unwrap();
    assert_eq!((upper.passed, upper.failing), (2, vec![2, 3, 4]));

    assert_eq!(measured.yield_against(None, None), None);
    assert_eq!(measured.yield_against(Some(3.0), Some(1.0)), None);
}

#[test]
fn a_member_the_measurement_cannot_take_keeps_its_reason_and_fails_the_yield() {
    let mut family = expanded(&[(0.0, 2.0), (1.0, 3.0), (2.0, 4.0)]);
    family[1].waveforms[0].name = "V(other)".to_owned();
    let measured = measure(&family, "avg(V(out))");
    assert!(measured.members[1].value.is_err());

    let statistics = measured.statistics().unwrap();
    assert_eq!((statistics.count, statistics.unmeasured), (2, 1));
    close(statistics.mean, 2.0);

    let window = measured.yield_against(Some(0.0), Some(10.0)).unwrap();
    assert_eq!(
        (window.passed, window.total, window.failing),
        (2, 3, vec![1])
    );
}

#[test]
fn a_trace_is_not_a_member_value() {
    let family = expanded(&[(0.0, 2.0), (1.0, 3.0)]);
    let measured = measure(&family, "V(out) * 2");
    assert!(
        measured.members[0]
            .value
            .as_ref()
            .is_err_and(|reason| reason.contains("trace"))
    );
}

#[test]
fn a_lone_analysis_is_not_a_family() {
    let family = expanded(&[(0.0, 2.0)]);
    assert!(family_members(&family[0], &family).is_none());
    let plain = ramp(9, "plain", 0.0, 1.0);
    assert!(family_members(&plain, std::slice::from_ref(&plain)).is_none());
}

#[test]
fn a_sweep_member_is_one_row_of_the_shared_waveforms() {
    let sweep = AnalysisResult::<RetainedWaveform>::new(3, AnalysisType::Parametric, "Sweep", 0.0)
        .with_waveforms(vec![RetainedWaveform::new(
            "V(out)",
            vec![1.0, 2.0, 3.0],
            vec![10.0, 20.0, 40.0],
        )])
        .with_family_metadata(AnalysisResultFamilyMetadata::Parametric {
            target: "R1".to_owned(),
            sweep_values: vec![1.0, 2.0, 3.0],
            failed_points: 0,
            member_measurements: Vec::new(),
        });
    let members =
        family_members(&sweep, std::iter::empty::<&AnalysisResult>()).expect("a sweep family");
    assert_eq!(members.len(), 3);
    assert_eq!(members[2].label, "Point 3");
    let measured = evaluate_members(&parse("V(out) * 2"), &members);
    let values = measured.values();
    for (value, expected) in values.iter().zip([20.0, 40.0, 80.0]) {
        close(*value, expected);
    }
    // {20, 40, 80}: mean 140 / 3, squared deviations summing to 5600 / 3,
    // so a sample variance of 2800 / 3.
    let statistics = measured.statistics().unwrap();
    close(statistics.mean, 140.0 / 3.0);
    close(statistics.std_dev.unwrap(), (2800.0_f64 / 3.0).sqrt());
}

fn trial(index: usize, delay: Option<f64>) -> FamilyMemberMeasurements {
    FamilyMemberMeasurements::new(
        FamilyMemberId::MonteCarloTrial {
            index,
            seed: 100 + index as u64,
        },
        vec![FamilyMeasurementEvidence {
            unit: None,
            name: "tdelay".to_owned(),
            value: delay,
            passed: delay.is_some(),
            error: delay.is_none().then(|| "no crossing".to_owned()),
        }],
    )
}

#[test]
fn a_monte_carlo_trial_is_measured_from_its_retained_measurements() {
    let analysis = AnalysisResult::<RetainedWaveform>::new(4, AnalysisType::MonteCarlo, "MC", 0.0)
        .with_family_metadata(AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 1,
            runs_requested: 4,
            runs_completed: 4,
            failures: 0,
            all_converged: true,
            variables: Vec::new(),
            member_measurements: vec![
                trial(0, Some(1.0e-9)),
                trial(1, Some(2.0e-9)),
                trial(2, None),
                trial(3, Some(3.0e-9)),
            ],
        });
    let members = family_members(&analysis, std::iter::empty::<&AnalysisResult>())
        .expect("a trial population");
    assert_eq!(members[1].label, "Trial 1 \u{b7} seed 101");
    let measured = evaluate_members(&parse("TDELAY * 1e9"), &members);
    assert!(measured.members[2].value.is_err());
    let statistics = measured.statistics().unwrap();
    assert_eq!((statistics.count, statistics.unmeasured), (3, 1));
    close(statistics.mean, 2.0);
    close(statistics.std_dev.unwrap(), 1.0);
    let window = measured.yield_against(None, Some(2.5)).unwrap();
    assert_eq!(
        (window.passed, window.total, window.failing),
        (2, 4, vec![2, 3])
    );
}
