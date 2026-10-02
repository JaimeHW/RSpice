//! Population correspondence, units, requirement margins and descriptive statistics.

use super::*;
use crate::analysis_type::AnalysisType;
use crate::family_measurements::{FamilyMeasurementEvidence, FamilyMemberMeasurements};
use crate::family_metadata::MonteCarloVariableMetadata;

fn variable(name: &str, samples: Vec<f64>) -> MonteCarloVariableMetadata {
    let count = samples.len() as f64;
    let mean = samples.iter().sum::<f64>() / count;
    let variance = samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (count - 1.0);
    MonteCarloVariableMetadata {
        mean_confidence: None,
        name: name.to_owned(),
        mean,
        std_dev: variance.sqrt(),
        min: samples.iter().copied().fold(f64::INFINITY, f64::min),
        max: samples.iter().copied().fold(f64::NEG_INFINITY, f64::max),
        samples,
    }
}

fn trial(index: usize, gain: f64) -> FamilyMemberMeasurements {
    FamilyMemberMeasurements::new(
        FamilyMemberId::MonteCarloTrial {
            index,
            seed: 0x73a4 + index as u64,
        },
        vec![FamilyMeasurementEvidence {
            unit: None,
            name: "gain_dc".to_owned(),
            value: Some(gain),
            passed: true,
            error: None,
        }],
    )
}

fn monte_carlo(
    members: Vec<FamilyMemberMeasurements>,
    samples: Vec<f64>,
) -> AnalysisResult<RetainedWaveform> {
    let completed = members.len();
    AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC", 0.0).with_family_metadata(
        AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 0x73a4,
            runs_requested: completed,
            runs_completed: completed,
            failures: 0,
            all_converged: true,
            variables: vec![variable("RGAIN.r", samples)],
            member_measurements: members,
        },
    )
}

fn specs_with_limit(min: Option<f64>, max: Option<f64>) -> Vec<SpecEntry> {
    vec![SpecEntry {
        measurement: "gain_dc".to_owned(),
        expression: String::new(),
        min,
        max,
        unit: "dB".to_owned(),
        scope: SpecPointScope::AllPoints,
    }]
}

#[test]
fn a_complete_run_pairs_its_variables_with_its_measurements() {
    let analysis = monte_carlo(
        vec![trial(0, 40.0), trial(1, 39.0), trial(2, 41.0)],
        vec![1.0, 2.0, 3.0],
    );
    let plan = build(
        &analysis,
        PopulationRequirements::Legacy(&specs_with_limit(Some(39.5), None)),
    )
    .expect("the fixture retains a population");

    assert!(plan.variables_paired);
    assert_eq!(plan.trial_count(), 3);
    assert_eq!(
        plan.columns[plan.column_index("RGAIN.r").expect("sampled column")].values,
        [Some(1.0), Some(2.0), Some(3.0)]
    );
    assert_eq!(
        plan.status,
        [
            TrialStatus::Passing,
            TrialStatus::Failing,
            TrialStatus::Passing
        ]
    );
    assert_eq!(plan.failing_count(), 1);
    assert_eq!(
        plan.columns[plan.column_index("gain_dc").expect("measured column")].unit,
        "dB",
        "the unit comes from the requirement that bounds it"
    );
}

#[test]
fn a_run_that_dropped_a_trial_refuses_to_pair_variables_with_measurements() {
    // The driver requested four and retained trials 0, 1 and 3.
    let analysis = monte_carlo(
        vec![trial(0, 40.0), trial(1, 40.5), trial(3, 41.0)],
        vec![1.0, 2.0, 3.0],
    );
    let plan =
        build(&analysis, PopulationRequirements::Legacy(&[])).expect("a population is built");

    assert!(
        !plan.variables_paired,
        "trial 3 is not sample 2, and the sheet must not pretend it is"
    );
    assert!(
        plan.columns[plan
            .column_index("RGAIN.r")
            .expect("the sampled column is still listed")]
        .values
        .iter()
        .all(Option::is_none),
        "an unpairable sampled column carries no per-trial value"
    );
    let measured = &plan.columns[plan.column_index("gain_dc").expect("measured column")];
    let variable = &plan.columns[plan.column_index("RGAIN.r").expect("sampled column")];
    assert!(!plan.columns_are_paired(variable, measured));
    assert!(plan.columns_are_paired(measured, measured));
}

#[test]
fn an_unmeasured_trial_is_not_a_failing_trial() {
    let mut members = vec![trial(0, 40.0), trial(1, 40.0)];
    members[1].measurements[0].value = None;
    members[1].measurements[0].passed = false;
    let analysis = monte_carlo(members, vec![1.0, 2.0]);
    let plan = build(
        &analysis,
        PopulationRequirements::Legacy(&specs_with_limit(Some(39.5), None)),
    )
    .expect("a population is built");

    assert_eq!(plan.status, [TrialStatus::Passing, TrialStatus::Unmeasured]);
    assert_eq!(plan.failing_count(), 0);
}

#[test]
fn the_box_is_the_interpolated_quartiles_with_tukey_whiskers() {
    let values = sorted(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 100.0]);
    let statistics = box_statistics(&values, Whiskers::Tukey).expect("ten samples");

    assert!((statistics.q1 - 3.25).abs() < 1.0e-12, "{statistics:?}");
    assert!((statistics.median - 5.5).abs() < 1.0e-12);
    assert!((statistics.q3 - 7.75).abs() < 1.0e-12, "{statistics:?}");
    assert!(
        (statistics.whisker_high - 9.0).abs() < 1.0e-12,
        "{statistics:?}"
    );
    assert!((statistics.whisker_low - 1.0).abs() < 1.0e-12);
    assert!((statistics.maximum - 100.0).abs() < 1.0e-12);

    let extremes = box_statistics(&values, Whiskers::Extremes).expect("ten samples");
    assert!((extremes.whisker_high - 100.0).abs() < 1.0e-12);
}

#[test]
fn a_perfect_yield_still_carries_an_interval() {
    let (low, high) = wilson_interval(1_000, 1_000).expect("a thousand trials");
    assert!(low > 99.0 && low < 100.0, "{low}");
    assert!((high - 100.0).abs() < 1.0e-9, "{high}");

    let (low, high) = wilson_interval(986, 1_000).expect("a thousand trials");
    assert!(low > 97.6 && low < 98.7, "{low}");
    assert!(high > 98.5 && high < 99.3, "{high}");
}

#[test]
fn three_sigma_of_margin_is_a_capability_of_one() {
    let values: Vec<f64> = (0..1_001)
        .map(|index| (index as f64 - 500.0) / 500.0)
        .collect();
    let sigma = std_dev(&values).expect("a spread");
    let limit = PopulationLimit::for_test(Some(-3.0 * sigma), None);
    let capability = cpk(&values, &limit).expect("a capability");
    assert!((capability - 1.0).abs() < 1.0e-9, "{capability}");

    let two_sided = PopulationLimit::for_test(Some(-3.0 * sigma), Some(6.0 * sigma));
    let capability = cpk(&values, &two_sided).expect("a capability");
    assert!((capability - 1.0).abs() < 1.0e-9, "the tighter side wins");
}

#[test]
fn the_margin_percentage_is_measured_against_the_requirement_it_belongs_to() {
    let window = PopulationLimit::for_test(Some(-50.0), Some(50.0));
    assert_eq!(window.margin_percent(0.0), Some(100.0));
    assert_eq!(window.margin_percent(50.0), Some(0.0));
    assert_eq!(window.margin_percent(75.0), Some(-50.0));

    let floor = PopulationLimit::for_test(Some(95.0), None);
    assert_eq!(floor.margin_percent(95.0), Some(0.0));
    let above = floor.margin_percent(190.0).expect("a percentage");
    assert!((above - 100.0).abs() < 1.0e-12);
}

#[test]
fn the_fit_recovers_the_line_it_was_given() {
    let xs: Vec<f64> = (0..50).map(f64::from).collect();
    let ys: Vec<f64> = xs.iter().map(|x| 3.5 * x - 7.25).collect();
    assert!((pearson(&xs, &ys).expect("a correlation") - 1.0).abs() < 1.0e-12);
    let (slope, intercept) = least_squares(&xs, &ys).expect("a fit");
    assert!((slope - 3.5).abs() < 1.0e-12);
    assert!((intercept + 7.25).abs() < 1.0e-12);

    // A column with no spread has no correlation to report, and saying
    // "0" would read as "measured and uncorrelated".
    let flat = vec![2.0; 50];
    assert_eq!(pearson(&flat, &ys), None);
    assert_eq!(least_squares(&flat, &ys), None);
}

#[test]
fn the_kernel_density_is_a_density() {
    let values = sorted(
        &(0..201)
            .map(|i| f64::from(i) / 100.0 - 1.0)
            .collect::<Vec<_>>(),
    );
    let bandwidth = silverman_bandwidth(&values).expect("a bandwidth");
    let (low, high) = (-2.0, 2.0);
    let steps = 4_000;
    let step = (high - low) / f64::from(steps);
    let area: f64 = (0..steps)
        .map(|index| {
            let at = low + step * f64::from(index) + step / 2.0;
            kernel_density(&values, bandwidth, at) * step
        })
        .sum();
    assert!(
        (area - 1.0).abs() < 1.0e-3,
        "the density integrates to {area}"
    );
}

#[test]
fn monte_carlo_population_uses_exact_trial_evidence_across_failed_gaps() {
    let mut members = vec![trial(0, 40.0), trial(1, 0.0), trial(2, 42.0)];
    members[1].measurements[0].value = None;
    members[1].measurements[0].passed = false;
    members[1].measurements[0].error = Some("did not converge".into());
    let mut result = monte_carlo(members, vec![40.0, 42.0]);
    let Some(AnalysisResultFamilyMetadata::MonteCarlo {
        runs_completed,
        failures,
        all_converged,
        variables,
        ..
    }) = &mut result.family_metadata
    else {
        panic!("MC")
    };
    *runs_completed = 2;
    *failures = 1;
    *all_converged = false;
    variables[0].name = "gain_dc".into();
    let plan = build(
        &result,
        PopulationRequirements::Legacy(&specs_with_limit(Some(39.5), None)),
    )
    .unwrap();
    assert!(plan.variables_paired);
    assert_eq!(plan.columns[0].values, [Some(40.0), None, Some(42.0)]);
    assert_eq!(
        plan.status,
        [
            TrialStatus::Passing,
            TrialStatus::Unmeasured,
            TrialStatus::Passing
        ]
    );
    assert_eq!(plan.trial_count(), 3);
}

#[test]
fn measurement_units_scale_distribution_values_limits_and_failure_counts_together() {
    use rspice_core::analysis::MeasurementUnit;
    let mut members = vec![trial(0, 0.25), trial(1, 0.35), trial(2, 0.20)];
    for member in &mut members {
        member.measurements[0].unit = Some(MeasurementUnit::Known("V".into()));
    }
    let analysis = monte_carlo(members, vec![1.0, 2.0, 3.0]);
    let mut specs = specs_with_limit(Some(200.0), Some(300.0));
    specs[0].unit = "mV".into();
    let plan = build(&analysis, PopulationRequirements::Legacy(&specs)).unwrap();
    let column = &plan.columns[plan.column_index("gain_dc").unwrap()];
    assert_eq!(column.values, vec![Some(250.0), Some(350.0), Some(200.0)]);
    assert_eq!(column.unit, "mV");
    assert_eq!(plan.failing_count(), 1);
    specs[0].unit = "mA".into();
    let plan = build(&analysis, PopulationRequirements::Legacy(&specs)).unwrap();
    let column = &plan.columns[plan.column_index("gain_dc").unwrap()];
    assert!(column.values.iter().all(Option::is_none));
}
