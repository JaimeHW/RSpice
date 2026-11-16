//! Tests for regression export and tolerance contracts.
//!
//! The cases are fail-closed: an empty matrix still exports a blocked case,
//! XML control characters are refused rather than escaped away, and a receipt
//! whose tolerance digest does not match its contract is rejected.

use super::raster::{sealed_run, seed_corner};
use super::*;
use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use crate::state::{AnalysisResult, AnalysisType, RetainedTrace, SimulationRun};
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::simulate::test_host::SimulateTestHost;

pub(crate) mod yield_histogram;

fn attributed(analysis: AnalysisResult) -> AnalysisResult {
    attributed_with_id(analysis, AnalysisInstanceId::new())
}

fn attributed_with_id(
    analysis: AnalysisResult,
    source_instance_id: AnalysisInstanceId,
) -> AnalysisResult {
    analysis.with_provenance(
        crate::state::AnalysisResultProvenance::new(
            source_instance_id,
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0x5a; 32]),
            Vec::new(),
        )
        .expect("test provenance is internally valid"),
    )
}

#[test]
fn golden_regression_authority_is_scoped_to_the_active_plan() {
    let owner = crate::product::SimulationPlanId::new();
    let other = crate::product::SimulationPlanId::new();
    let run = sealed_run(
        1,
        owner,
        AnalysisInstanceId::new(),
        2,
        AnalysisResult::new(1, AnalysisType::Ac, "AC"),
    );

    assert!(validate_regression_run_for_plan(&run, owner).is_ok());
    let error = validate_regression_run_for_plan(&run, other).unwrap_err();
    assert!(error.contains(&owner.to_string()));
    assert!(error.contains(&other.to_string()));
}

#[test]
fn selecting_a_golden_regression_baseline_pins_its_dataset_atomically() {
    let mut harness = SimulateTestHost::harness();
    let plan_id = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let source_id = AnalysisInstanceId::new();
    let baseline = sealed_run(
        1,
        plan_id,
        source_id,
        2,
        AnalysisResult::new(1, AnalysisType::Ac, "baseline")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)]),
    );
    let baseline_id = baseline.run_id;
    let candidate = sealed_run(
        2,
        plan_id,
        source_id,
        2,
        AnalysisResult::new(1, AnalysisType::Ac, "candidate")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)]),
    );
    harness.model.simulation.retained.runs = vec![candidate, baseline].into();
    assert!(harness.model.simulation.select_run(0));

    commit_regression_baseline(&mut harness.cx(), baseline_id).expect("baseline commits");

    assert!(
        harness
            .model
            .simulation
            .retained
            .run_by_stable_id(baseline_id)
            .expect("baseline remains retained")
            .retention()
            .is_pinned()
    );
    assert_eq!(
        harness
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .and_then(|payload| payload.regression_baseline_run),
        Some(baseline_id)
    );
}

fn retain_preflight_report(harness: &mut FeatureHarness<SimulateFeature>) {
    let (plan_id, plan_revision) = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| (plan.id(), plan.revision()))
        .expect("default plan");
    let (topology_root, topology_revision, topology_closure) =
        harness.model.configured_topology_revision();
    harness.runs.preflight = crate::workbench::app_state::run_preflight::PreflightDialogState {
        report: Some(crate::state::PreflightReport {
            project_revision: harness.model.workspace.content.project.revision().get(),
            topology_root,
            topology_revision,
            topology_closure,
            simulation_plan_id: Some(plan_id),
            simulation_plan_revision: Some(plan_revision),
            blockers: Vec::new(),
            advisories: Vec::new(),
            prepared: None,
        }),
        ..Default::default()
    };
}

#[test]
fn verify_plan_insertion_invalidates_retained_preflight_evidence() {
    let mut harness = SimulateTestHost::harness();
    retain_preflight_report(&mut harness);
    let source_revision = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .revision();

    open_analysis_configuration(
        &mut harness.cx(),
        crate::simulation::plan::AnalysisKind::Noise,
    )
    .expect("missing analysis inserts and opens");

    assert_eq!(
        harness
            .model
            .sim_setup
            .stable_analysis_plan()
            .expect("plan remains available")
            .revision(),
        source_revision.next().expect("revision advances")
    );
    assert!(harness.runs.preflight.report.is_none());
}

#[test]
fn soa_cross_probe_selects_the_exact_schematic_device() {
    let mut harness = SimulateTestHost::harness();
    harness
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            crate::state::Component::new(
                42,
                crate::state::ComponentType::Nmos,
                crate::state::Point::new(10, 20),
            )
            .with_name_value("M1", "NMOS"),
        );

    cross_probe_soa_device(&mut harness.cx(), "m1")
        .expect("device identity cross-probes case-insensitively");

    assert!(
        harness
            .model
            .schematic
            .session
            .editor
            .selection
            .has_component(42)
    );
    assert_eq!(
        harness.wb.workspace,
        super::super::super::state::Workspace::Design
    );
    assert!(cross_probe_soa_device(&mut harness.cx(), "M404").is_err());
}

#[test]
fn normalized_margin_is_positive_inside_and_negative_outside_bounds() {
    let spec = SpecEntry {
        measurement: "gain".to_owned(),
        expression: String::new(),
        min: Some(10.0),
        max: Some(20.0),
        unit: "dB".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };

    assert_eq!(verification_margin(&spec, 15.0), 5.0);
    assert_eq!(verification_margin(&spec, 8.0), -2.0);
    assert_eq!(verification_margin(&spec, 23.0), -3.0);
    assert_eq!(normalized_margin(&spec, 15.0), Some(0.5));
    assert_eq!(normalized_margin(&spec, 8.0), Some(-0.2));
}

#[test]
fn joint_sample_summary_requires_aligned_trails_and_ands_every_spec() {
    let make_result = |target: &str, trail: Vec<bool>| {
        let pass_count = trail.iter().filter(|passes| **passes).count();
        rspice_results::yield_analysis::YieldResult {
            spec: rspice_results::yield_analysis::YieldSpec::lower(target, 0.0, ""),
            total_runs: trail.len(),
            pass_count,
            fail_count: trail.len() - pass_count,
            yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
            stats: rspice_results::yield_analysis::DistributionStats::default(),
            samples: vec![1.0; trail.len()],
            trail,
        }
    };
    let aligned = [
        make_result("gain", vec![true, false, false]),
        make_result("bandwidth", vec![true, false, true]),
    ];
    let joint = joint_sample_summary(&aligned).expect("aligned joint trail");
    assert_eq!(joint.passing, 1);
    assert_eq!(joint.total, 3);
    assert_eq!(joint.specification_count, 2);
    assert!((joint.yield_percent() - 100.0 / 3.0).abs() < 1.0e-12);

    let misaligned = [
        make_result("gain", vec![true, true]),
        make_result("bandwidth", vec![true]),
    ];
    assert_eq!(joint_sample_summary(&misaligned), None);
}

#[test]
fn joint_headline_can_be_lower_than_every_individual_spec_yield() {
    let make_result = |target: &str, trail: Vec<bool>| {
        let pass_count = trail.iter().filter(|passes| **passes).count();
        rspice_results::yield_analysis::YieldResult {
            spec: rspice_results::yield_analysis::YieldSpec::lower(target, 0.0, ""),
            total_runs: trail.len(),
            pass_count,
            fail_count: trail.len() - pass_count,
            yield_percent: pass_count as f64 / trail.len() as f64 * 100.0,
            stats: rspice_results::yield_analysis::DistributionStats::default(),
            samples: vec![1.0; trail.len()],
            trail,
        }
    };
    let results = [
        make_result("gain", vec![true, true, false, true]),
        make_result("bandwidth", vec![true, false, true, true]),
    ];

    let joint = joint_sample_summary(&results).expect("aligned joint trail");
    assert_eq!((joint.passing, joint.total), (2, 4));
    assert_eq!(joint.yield_percent(), 50.0);
    assert!(
        results
            .iter()
            .all(|result| result.yield_percent > joint.yield_percent())
    );
}

#[test]
fn verification_requires_an_explicit_active_run_selection() {
    let mut harness = SimulateTestHost::harness();
    harness
        .model
        .simulation
        .retained
        .runs
        .push(SimulationRun::new(1));
    harness.model.simulation.view.active_run_idx = None;

    assert_eq!(verification_run_index(&harness.model), None);
    assert!(verification_run(&harness.model).is_none());
}

#[test]
fn regression_evidence_uses_only_aligned_finite_measurements() {
    let source_instance_id = AnalysisInstanceId::new();
    let mut baseline = SimulationRun::new(38);
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Ac, "baseline").with_measurements(vec![
            rspice_core::MeasureResult::success("gain", 40.0),
            rspice_core::MeasureResult::success("bandwidth", 100.0),
            rspice_core::MeasureResult::success("baseline_only", 1.0),
        ]),
        source_instance_id,
    ));
    let mut current = SimulationRun::new(41);
    current.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Ac, "candidate").with_measurements(vec![
            rspice_core::MeasureResult::success("GAIN", 40.4),
            rspice_core::MeasureResult::success("bandwidth", 103.0),
            rspice_core::MeasureResult::success("candidate_only", 2.0),
        ]),
        source_instance_id,
    ));

    let checks = derive_regression_checks(&baseline, &current);
    assert_eq!(checks.len(), 2);
    assert_eq!(checks[0].name, "bandwidth");
    assert!(checks[0].changed());
    assert_eq!(checks[1].name, "gain");
    assert!(checks[1].changed());
}

#[test]
fn regression_evidence_retains_finite_goal_misses() {
    let source_instance_id = AnalysisInstanceId::new();
    let mut baseline_measurement = rspice_core::MeasureResult::success("gain", 40.0);
    baseline_measurement.expected = Some(41.0);
    baseline_measurement.tolerance = Some(0.1);
    baseline_measurement.passed = false;
    baseline_measurement.error = Some("value misses GOAL".to_owned());
    let mut current_measurement = rspice_core::MeasureResult::success("gain", 40.2);
    current_measurement.expected = Some(41.0);
    current_measurement.tolerance = Some(0.1);
    current_measurement.passed = false;
    current_measurement.error = Some("value misses GOAL".to_owned());

    let mut baseline = SimulationRun::new(38);
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Ac, "baseline")
            .with_measurements(vec![baseline_measurement]),
        source_instance_id,
    ));
    let mut current = SimulationRun::new(41);
    current.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Ac, "candidate")
            .with_measurements(vec![current_measurement]),
        source_instance_id,
    ));

    let checks = derive_regression_checks(&baseline, &current);
    assert_eq!(checks.len(), 1);
    assert_eq!(checks[0].baseline, 40.0);
    assert_eq!(checks[0].current, 40.2);
}

#[test]
fn regression_waveforms_align_by_stable_source_and_signal_identity() {
    let source_instance_id = AnalysisInstanceId::new();
    let mut baseline = SimulationRun::new(38);
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(9, AnalysisType::Transient, "unrelated transient").with_waveforms(
            vec![RetainedTrace::new(
                "v(OUT)",
                vec![0.0, 2.0, 4.0],
                vec![0.0, 2.0, 4.0],
                "#808080",
            )],
        ),
        AnalysisInstanceId::new(),
    ));
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Transient, "baseline").with_waveforms(vec![
            RetainedTrace::new(
                "V(out)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 1.0, 2.0],
                "#ffffff",
            ),
        ]),
        source_instance_id,
    ));
    let mut current = SimulationRun::new(41);
    current.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Transient, "candidate").with_waveforms(vec![
            RetainedTrace::new(
                "v(OUT)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 1.1, 2.1],
                "#ffbf00",
            ),
            RetainedTrace::new(
                "V(other)",
                vec![0.0, 1.0, 2.0],
                vec![0.0, 0.5, 1.0],
                "#00ff00",
            ),
        ]),
        source_instance_id,
    ));

    let aligned = regression_waveform_pairs(&baseline, &current);
    assert_eq!(aligned.len(), 1);
    assert_eq!(aligned[0].baseline.name, "V(out)");

    current.analyses[0].waveforms[0].x = std::sync::Arc::new(vec![0.0, 1.0, 2.5]);
    assert_eq!(regression_waveform_pairs(&baseline, &current).len(), 1);
}

fn tolerance_rule(
    target: crate::state::RegressionTargetSelector,
    method: crate::state::RegressionComparisonMethod,
) -> crate::state::RegressionToleranceRule {
    crate::state::RegressionToleranceRule {
        target,
        method,
        absolute_tolerance: 0.0,
        relative_tolerance: 0.0,
        time_skew_allowance: 0.0,
        comparison_window: None,
    }
}

#[test]
fn scalar_regression_verdict_uses_persisted_absolute_and_relative_contract() {
    let source_instance_id = AnalysisInstanceId::new();
    let mut baseline = SimulationRun::new(1);
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Ac, "baseline")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 100.0)]),
        source_instance_id,
    ));
    let mut candidate = SimulationRun::new(2);
    candidate.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Ac, "candidate")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 101.5)]),
        source_instance_id,
    ));
    let check = derive_regression_checks(&baseline, &candidate)
        .into_iter()
        .next()
        .unwrap();
    let mut rule = tolerance_rule(
        check.target.clone(),
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope,
    );
    rule.absolute_tolerance = 0.5;
    rule.relative_tolerance = 0.01;
    assert!(evaluate_regression_check(&check, Some(&rule)).passed());

    rule.relative_tolerance = 0.005;
    assert!(evaluate_regression_check(&check, Some(&rule)).failed());

    rule.method = crate::state::RegressionComparisonMethod::PointwiseRelative;
    rule.absolute_tolerance = 2.0;
    rule.relative_tolerance = 0.01;
    assert!(evaluate_regression_check(&check, Some(&rule)).passed());

    rule.absolute_tolerance = 0.4;
    rule.relative_tolerance = 0.005;
    assert!(evaluate_regression_check(&check, Some(&rule)).failed());
    assert_eq!(
        evaluate_regression_check(&check, None),
        RegressionVerdict::NotConfigured
    );
}

#[test]
fn waveform_envelope_interpolates_within_skew_and_window() {
    let source_instance_id = AnalysisInstanceId::new();
    let mut baseline = SimulationRun::new(1);
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Transient, "baseline").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 0.0], "#fff"),
        ]),
        source_instance_id,
    ));
    let mut candidate = SimulationRun::new(2);
    candidate.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Transient, "candidate").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.1, 1.1, 2.1], vec![0.0, 1.0, 0.0], "#ff0"),
        ]),
        source_instance_id,
    ));
    let pairs = regression_waveform_pairs(&baseline, &candidate);
    let pair = &pairs[0];
    let mut rule = tolerance_rule(
        pair.target.clone(),
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope,
    );
    rule.time_skew_allowance = 0.100_000_1;
    rule.comparison_window = Some(crate::state::RegressionComparisonWindow {
        start: 0.5,
        end: 1.5,
    });
    assert!(evaluate_regression_waveform(pair, Some(&rule)).passed());

    rule.time_skew_allowance = 0.01;
    assert!(evaluate_regression_waveform(pair, Some(&rule)).failed());
}

#[test]
fn symmetric_waveform_envelope_rejects_candidate_only_spike_and_missing_coverage() {
    let target = crate::state::RegressionTargetSelector {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        source_instance_id: AnalysisInstanceId::new(),
        kind: crate::state::RegressionTargetKind::Waveform,
        name: "v(out)".to_owned(),
        occurrence: 0,
    };
    let baseline = RetainedTrace::new("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 0.0], "#fff");
    let spike = RetainedTrace::new(
        "V(out)",
        vec![0.0, 0.5, 1.0, 1.5, 2.0],
        vec![0.0, 0.0, 10.0, 0.0, 0.0],
        "#ff0",
    );
    let mut rule = tolerance_rule(
        target.clone(),
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope,
    );
    rule.absolute_tolerance = 0.1;
    rule.time_skew_allowance = 0.6;
    assert!(
        evaluate_regression_waveform(
            &RegressionWaveformPair {
                target: target.clone(),
                baseline: &baseline,
                current: &spike,
                analysis_type: AnalysisType::Transient,
            },
            Some(&rule),
        )
        .failed()
    );

    let truncated = RetainedTrace::new("V(out)", vec![0.5, 1.0, 1.5], vec![0.0, 0.0, 0.0], "#ff0");
    rule.time_skew_allowance = 0.1;
    assert!(
        evaluate_regression_waveform(
            &RegressionWaveformPair {
                target,
                baseline: &baseline,
                current: &truncated,
                analysis_type: AnalysisType::Transient,
            },
            Some(&rule),
        )
        .failed()
    );
}

#[test]
fn waveform_window_evaluates_interpolated_inclusive_boundaries() {
    let target = crate::state::RegressionTargetSelector {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        source_instance_id: AnalysisInstanceId::new(),
        kind: crate::state::RegressionTargetKind::Waveform,
        name: "v(out)".to_owned(),
        occurrence: 0,
    };
    let baseline = RetainedTrace::new("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 0.0], "#fff");
    let candidate = RetainedTrace::new("V(out)", vec![0.0, 1.0, 2.0], vec![2.0, 0.0, 2.0], "#ff0");
    let mut rule = tolerance_rule(
        target.clone(),
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope,
    );
    rule.comparison_window = Some(crate::state::RegressionComparisonWindow {
        start: 0.5,
        end: 1.5,
    });
    assert!(
        evaluate_regression_waveform(
            &RegressionWaveformPair {
                target,
                baseline: &baseline,
                current: &candidate,
                analysis_type: AnalysisType::Transient,
            },
            Some(&rule),
        )
        .failed(),
        "equal stored interior knots must not hide different interpolated window boundaries"
    );
}

#[test]
fn coverage_union_blocks_dropped_measurement_waveform_analysis_and_orphan_rule() {
    let source_id = AnalysisInstanceId::new();
    let dropped_analysis_id = AnalysisInstanceId::new();
    let orphan_id = AnalysisInstanceId::new();
    let mut baseline = SimulationRun::new(1);
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Transient, "baseline")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 10.0)])
            .with_waveforms(vec![RetainedTrace::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![0.0, 1.0],
                "#fff",
            )]),
        source_id,
    ));
    baseline.add_analysis(attributed_with_id(
        AnalysisResult::new(2, AnalysisType::DcOp, "dropped empty analysis"),
        dropped_analysis_id,
    ));
    let mut candidate = SimulationRun::new(2);
    candidate.add_analysis(attributed_with_id(
        AnalysisResult::new(1, AnalysisType::Transient, "candidate"),
        source_id,
    ));
    let orphan_target = crate::state::RegressionTargetSelector {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        source_instance_id: orphan_id,
        kind: crate::state::RegressionTargetKind::Measurement,
        name: "orphan".to_owned(),
        occurrence: 0,
    };
    let rules = vec![tolerance_rule(
        orphan_target.clone(),
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope,
    )];

    let issues = regression_coverage_issues(&baseline, &candidate, &rules);
    assert_eq!(issues.len(), 4);
    assert!(
        issues.iter().any(|issue| issue.label.contains("gain")
            && issue.detail.contains("missing from the candidate"))
    );
    assert!(issues.iter().any(|issue| issue.label.contains("v(out)")
        && issue.detail.contains("missing from the candidate")));
    assert!(issues.iter().any(
        |issue| issue.label.contains(&dropped_analysis_id.to_string())
            && issue.detail.contains("analysis is missing")
    ));
    assert!(
        issues.iter().any(
            |issue| issue.label.contains("orphan") && issue.detail.contains("absent from both")
        )
    );
    assert_eq!(orphaned_regression_targets(&issues), vec![orphan_target]);

    let limit_only = regression_coverage_issues_for_policy(
        &baseline,
        &candidate,
        &rules,
        crate::state::RegressionSpecificationPolicy::LimitOnly,
    );
    assert_eq!(limit_only.len(), 3);
    assert!(
        limit_only
            .iter()
            .all(|issue| !issue.label.contains("v(out)"))
    );
    assert!(!regression_requires_waveforms(
        crate::state::RegressionSpecificationPolicy::LimitOnly
    ));
    assert!(regression_requires_waveforms(
        crate::state::RegressionSpecificationPolicy::LimitAndWaveform
    ));
}

#[test]
fn every_supported_waveform_method_rejects_nonfinite_and_nonmonotonic_samples() {
    assert_eq!(crate::state::RegressionComparisonMethod::ALL.len(), 2);
    assert!(
        serde_json::from_str::<crate::state::RegressionComparisonMethod>("\"feature_landmarks\"")
            .is_err()
    );
    let target = crate::state::RegressionTargetSelector {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        source_instance_id: AnalysisInstanceId::new(),
        kind: crate::state::RegressionTargetKind::Waveform,
        name: "v(out)".to_owned(),
        occurrence: 0,
    };
    let baseline = RetainedTrace::new("V(out)", vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 0.0], "#fff");
    let invalid = RetainedTrace::new(
        "V(out)",
        vec![0.0, 2.0, 1.0],
        vec![0.0, f64::NAN, 0.0],
        "#ff0",
    );
    let pair = RegressionWaveformPair {
        target: target.clone(),
        baseline: &baseline,
        current: &invalid,
        analysis_type: AnalysisType::Transient,
    };
    for method in crate::state::RegressionComparisonMethod::ALL {
        let rule = tolerance_rule(target.clone(), method);
        assert!(matches!(
            evaluate_regression_waveform(&pair, Some(&rule)),
            RegressionVerdict::NotEvaluated(_)
        ));
    }
}

#[test]
fn legacy_and_incomplete_runs_are_not_eligible_regression_datasets() {
    let source_id = AnalysisInstanceId::new();
    let mut legacy = SimulationRun::new(1);
    legacy.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "legacy").with_provenance(
            crate::state::AnalysisResultProvenance::new_with_source_domain(
                crate::state::AnalysisResultSourceDomain::LegacyUnclassified,
                source_id,
                ObjectRevision::INITIAL,
                ContentDigest::from_bytes([0x31; 32]),
                Vec::new(),
            )
            .unwrap(),
        ),
    );
    legacy
        .restore_provenance(crate::state::SimulationRunProvenance::LegacyPreparedUnclassified)
        .unwrap();
    assert!(regression_run_seal(&legacy).is_err());

    let plan_id = crate::product::SimulationPlanId::new();
    let complete = sealed_run(
        2,
        plan_id,
        AnalysisInstanceId::new(),
        2,
        AnalysisResult::new(1, AnalysisType::Ac, "complete"),
    );
    let mut incomplete = complete.clone();
    incomplete.analyses.clear();
    assert!(
        regression_run_seal(&incomplete)
            .unwrap_err()
            .contains("incomplete")
    );
}

#[test]
fn ci_documents_are_deterministic_receipt_bound_and_cover_the_full_matrix() {
    let digest = |byte| ContentDigest::from_bytes([byte; 32]);
    let receipt = super::super::super::state::RegressionComparisonReceipt {
        plan_id: crate::product::SimulationPlanId::new(),
        plan_revision: ObjectRevision::INITIAL,
        tolerance_digest: digest(1),
        baseline_run: crate::product::RunId::new(),
        candidate_run: crate::product::RunId::new(),
        baseline_dataset: crate::product::DatasetId::new(),
        candidate_dataset: crate::product::DatasetId::new(),
        baseline_content_digest: digest(2),
        candidate_content_digest: digest(3),
        baseline_authority_digest: digest(4),
        candidate_authority_digest: digest(5),
        aligned_checks: 1,
        aligned_waveforms: 1,
        changed_checks: 1,
        passed_checks: 1,
        failed_checks: 0,
        passed_waveforms: 0,
        failed_waveforms: 1,
        unconfigured_targets: 0,
        unevaluated_targets: 1,
    };
    let cases = vec![
        RegressionExportCase {
            name: "measurement::gain".to_owned(),
            detail: "worst_delta=0".to_owned(),
            disposition: RegressionExportDisposition::Pass,
        },
        RegressionExportCase {
            name: "waveform::v(out)".to_owned(),
            detail: "envelope exceeded <limit>".to_owned(),
            disposition: RegressionExportDisposition::Failure,
        },
        RegressionExportCase {
            name: "coverage::analysis".to_owned(),
            detail: "candidate missing".to_owned(),
            disposition: RegressionExportDisposition::Blocked,
        },
    ];
    let first = regression_ci_documents(&receipt, &cases).unwrap();
    let second = regression_ci_documents(&receipt, &cases).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.0.matches("<testcase ").count(), 3);
    assert!(first.0.contains("tests=\"3\" failures=\"1\" errors=\"1\""));
    assert!(
        first
            .0
            .contains(&receipt.candidate_content_digest.to_string())
    );
    assert!(first.0.contains("envelope exceeded &lt;limit&gt;"));
    assert!(first.1.contains("1..3"));
    assert!(first.1.contains("not ok 2 - waveform::v(out)"));
    assert!(
        first
            .1
            .contains(&receipt.baseline_authority_digest.to_string())
    );
}

#[test]
fn empty_regression_matrix_exports_one_blocked_case_and_xml_controls_fail_closed() {
    let cases = regression_export_cases(&[], &[], &[], &[], &[]);
    assert_eq!(cases.len(), 1);
    assert_eq!(cases[0].name, "coverage::no_comparable_targets");
    assert_eq!(cases[0].disposition, RegressionExportDisposition::Blocked);
    let digest = |byte| ContentDigest::from_bytes([byte; 32]);
    let receipt = super::super::super::state::RegressionComparisonReceipt {
        plan_id: crate::product::SimulationPlanId::new(),
        plan_revision: ObjectRevision::INITIAL,
        tolerance_digest: digest(1),
        baseline_run: crate::product::RunId::new(),
        candidate_run: crate::product::RunId::new(),
        baseline_dataset: crate::product::DatasetId::new(),
        candidate_dataset: crate::product::DatasetId::new(),
        baseline_content_digest: digest(2),
        candidate_content_digest: digest(3),
        baseline_authority_digest: digest(4),
        candidate_authority_digest: digest(5),
        aligned_checks: 0,
        aligned_waveforms: 0,
        changed_checks: 0,
        passed_checks: 0,
        failed_checks: 0,
        passed_waveforms: 0,
        failed_waveforms: 0,
        unconfigured_targets: 0,
        unevaluated_targets: 1,
    };
    let (junit, tap) = regression_ci_documents(&receipt, &cases).unwrap();
    assert!(junit.contains("tests=\"1\" failures=\"0\" errors=\"1\""));
    assert!(tap.contains("not ok 1 - coverage::no_comparable_targets"));
    assert!(xml_escape("illegal\u{1}control").is_err());
    assert_eq!(xml_escape("safe < value").unwrap(), "safe &lt; value");

    let source = AnalysisInstanceId::new();
    let checks = [RegressionCheck {
        target: crate::state::RegressionTargetSelector {
            source_domain: crate::state::AnalysisResultSourceDomain::ManualDeck,
            source_instance_id: source,
            kind: crate::state::RegressionTargetKind::Measurement,
            name: "gain".to_owned(),
            occurrence: 2,
        },
        name: "gain".to_owned(),
        source_identity: source.to_string(),
        baseline: 1.0,
        current: 1.0,
    }];
    let cases = regression_export_cases(
        &checks,
        &[RegressionVerdict::Pass {
            worst_delta: 0.0,
            allowed_delta: 0.0,
        }],
        &[],
        &[],
        &[],
    );
    assert_eq!(
        cases[0].name,
        format!("measurement::manual_deck::{source}::gain[2]")
    );
}

#[test]
fn comparison_window_parser_is_unit_safe_and_fail_closed() {
    assert_eq!(
        parse_regression_window("1m … 20m").unwrap(),
        Some(crate::state::RegressionComparisonWindow {
            start: 1e-3,
            end: 20e-3,
        })
    );
    assert_eq!(parse_regression_window("full domain").unwrap(), None);
    assert!(parse_regression_window("20m … 1m").is_err());
    assert!(parse_regression_window("not a window").is_err());
}

#[test]
fn tolerance_digest_is_order_independent_and_receipts_fail_on_contract_mismatch() {
    let target = |name: &str| crate::state::RegressionTargetSelector {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        source_instance_id: AnalysisInstanceId::from_namespace(
            uuid::Uuid::NAMESPACE_OID,
            name.as_bytes(),
        ),
        kind: crate::state::RegressionTargetKind::Measurement,
        name: name.to_owned(),
        occurrence: 0,
    };
    let mut first = tolerance_rule(
        target("gain"),
        crate::state::RegressionComparisonMethod::AbsoluteRelativeEnvelope,
    );
    first.absolute_tolerance = 0.1;
    let mut second = tolerance_rule(
        target("settling"),
        crate::state::RegressionComparisonMethod::PointwiseRelative,
    );
    second.relative_tolerance = 0.02;
    let digest = regression_tolerance_digest(&[first.clone(), second.clone()]);
    assert_eq!(
        digest,
        regression_tolerance_digest(&[second.clone(), first.clone()])
    );
    second.relative_tolerance = 0.03;
    let changed_digest = regression_tolerance_digest(&[first, second]);
    assert_ne!(digest, changed_digest);

    let plan_id = crate::product::SimulationPlanId::new();
    let source_id = AnalysisInstanceId::new();
    let baseline = sealed_run(
        1,
        plan_id,
        source_id,
        2,
        AnalysisResult::new(1, AnalysisType::Ac, "baseline")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)]),
    );
    let mut candidate = sealed_run(
        2,
        plan_id,
        source_id,
        2,
        AnalysisResult::new(1, AnalysisType::Ac, "candidate")
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)]),
    );
    let baseline_seal = regression_run_seal(&baseline).unwrap();
    let candidate_seal = regression_run_seal(&candidate).unwrap();
    let receipt = super::super::super::state::RegressionComparisonReceipt {
        plan_id,
        plan_revision: ObjectRevision::INITIAL,
        tolerance_digest: digest,
        baseline_run: baseline.run_id,
        candidate_run: candidate.run_id,
        baseline_dataset: baseline.dataset_id,
        candidate_dataset: candidate.dataset_id,
        baseline_content_digest: baseline_seal.content_digest,
        candidate_content_digest: candidate_seal.content_digest,
        baseline_authority_digest: baseline_seal.authority_digest,
        candidate_authority_digest: candidate_seal.authority_digest,
        aligned_checks: 2,
        aligned_waveforms: 0,
        changed_checks: 1,
        passed_checks: 2,
        failed_checks: 0,
        passed_waveforms: 0,
        failed_waveforms: 0,
        unconfigured_targets: 0,
        unevaluated_targets: 0,
    };
    assert!(regression_receipt_matches_contract(
        &receipt,
        plan_id,
        ObjectRevision::INITIAL,
        digest,
        &baseline,
        &candidate,
    ));
    assert!(!regression_receipt_matches_contract(
        &receipt,
        crate::product::SimulationPlanId::new(),
        ObjectRevision::INITIAL,
        digest,
        &baseline,
        &candidate,
    ));
    assert!(!regression_receipt_matches_contract(
        &receipt,
        plan_id,
        ObjectRevision::new(2).unwrap(),
        digest,
        &baseline,
        &candidate,
    ));
    assert!(!regression_receipt_matches_contract(
        &receipt,
        plan_id,
        ObjectRevision::INITIAL,
        changed_digest,
        &baseline,
        &candidate,
    ));

    candidate.analyses[0].measurements[0].value = Some(1.25);
    assert!(!regression_receipt_matches_contract(
        &receipt,
        plan_id,
        ObjectRevision::INITIAL,
        digest,
        &baseline,
        &candidate,
    ));

    let mut session = super::super::super::state::VerificationSessionState::default();
    session.regression_comparison = Some(receipt);
    let serialized = serde_json::to_value(session).unwrap();
    assert!(serialized.get("regression_comparison").is_none());
}

/// A limit is answered by the worst of every attributed measurement of its
/// name, not the first one found. On a dataset holding several — a corner set,
/// or a family whose members measured — the first would report a
/// specification passing while a retained point fails it.
#[test]
fn the_worst_retained_point_answers_a_limit() {
    use crate::state::{
        AnalysisResultFamilyMetadata, FamilyMeasurementEvidence, FamilyMemberId,
        FamilyMemberMeasurements,
    };

    let spec = SpecEntry {
        measurement: "gain".to_owned(),
        expression: "param=gain".to_owned(),
        min: Some(10.0),
        max: None,
        unit: "dB".to_owned(),
        scope: crate::state::SpecPointScope::AllPoints,
    };

    let members = [(0, 25.0, 12.0), (1, 75.0, 7.25), (2, 125.0, 11.0)]
        .into_iter()
        .map(|(index, coordinate, value)| {
            FamilyMemberMeasurements::new(
                FamilyMemberId::SweepPoint {
                    index,
                    value: coordinate,
                },
                vec![FamilyMeasurementEvidence {
                    unit: None,
                    name: "gain".to_owned(),
                    value: Some(value),
                    passed: true,
                    error: None,
                }],
            )
        })
        .collect();

    let mut run = SimulationRun::new(7);
    run.add_analysis(attributed(
        AnalysisResult::new(1, AnalysisType::Parametric, "sweep").with_family_metadata(
            AnalysisResultFamilyMetadata::Parametric {
                target: "TEMP".to_owned(),
                sweep_values: vec![25.0, 75.0, 125.0],
                failed_points: 0,
                member_measurements: members,
            },
        ),
    ));

    assert_eq!(
        crate::workbench::surfaces::simulate::measurement_in_output_dataset_for_test(&run, &spec),
        Some(7.25),
        "the worst retained point answers the limit, not the first"
    );
}

/// Verify must not judge a dataset the studio refuses.
///
/// The run's own measured values stay on the page — it did measure them — but
/// a run another plan owns answers none of this plan's limits, so no limit
/// reaches the table and nothing in it is judged.
#[test]
fn verify_refuses_the_datasets_the_studio_refuses_and_names_why() {
    use crate::state::EvidenceDomain;

    assert!(EvidenceDomain::ThisPlan.answers_a_plan_limit());
    assert!(EvidenceDomain::Legacy.answers_a_plan_limit());
    for refused in [
        EvidenceDomain::AnotherPlan,
        EvidenceDomain::ManualDeck,
        EvidenceDomain::NoDataset,
    ] {
        assert!(
            !refused.answers_a_plan_limit(),
            "{refused:?} must not answer a plan's limit"
        );
        assert!(
            refused.refusal().is_some(),
            "{refused:?} must state why it cannot, or the surface goes silently blank"
        );
    }

    let mut harness = corner_app();
    assert_eq!(
        verification_evidence_domain(&harness.model),
        EvidenceDomain::Legacy,
        "the corner fixture's run predates receipts, which Verify reads"
    );
    assert!(
        !verification_specs(&harness.model).is_empty(),
        "and its four limits are this plan's"
    );

    // The same run, now owned by a plan that is not the one being read.
    let plan_id = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let other = crate::product::SimulationPlanId::new();
    let analysis = harness.model.simulation.retained.runs[0].analyses[0].clone();
    let source = analysis
        .provenance()
        .expect("the fixture attributes its analysis")
        .source_instance_id();
    harness.model.simulation.retained.runs =
        vec![super::raster::sealed_run(3, other, source, 19, analysis)].into();
    harness.model.simulation.view.active_run_idx = Some(0);

    assert_ne!(other, plan_id);
    assert_eq!(
        verification_evidence_domain(&harness.model),
        EvidenceDomain::AnotherPlan
    );
    assert!(
        verification_specs(&harness.model).is_empty(),
        "another plan's dataset answers none of this plan's limits"
    );
    let run = verification_run(&harness.model).expect("the run is still the one Verify reads");
    let family = latest_analysis(&harness.model, AnalysisType::Corner)
        .expect("and still holds its corner run");
    let matrix = corner_matrix(run, family, None, &verification_specs(&harness.model), None);
    assert!(
        !matrix.rows.is_empty(),
        "the values the run did measure stay on the page"
    );
    assert!(
        matrix.rows.iter().all(|row| row.verdict().is_none()),
        "and nothing in it is judged"
    );
}

// ---------------------------------------------------------------------------
// The SOA evidence gate
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// The page grammar: corners and SOA
// ---------------------------------------------------------------------------

use super::table::Verdict;

#[test]
fn corner_labels_read_back_into_their_point() {
    assert_eq!(
        parse_corner_label("SS_1.620000V_-40.000000C"),
        Some(CornerPoint {
            process: "SS".to_owned(),
            voltage: Some(1.62),
            temperature: -40.0,
        })
    );
    assert_eq!(
        parse_corner_label("TT_MC_1.00V_27C").map(|point| point.process),
        Some("TT_MC".to_owned())
    );
    assert_eq!(parse_corner_label("Point 3"), None);
    assert_eq!(parse_corner_label("TT_1.8_27C"), None);
}

fn fixture_corner_matrix() -> CornerMatrix {
    let harness = corner_app();
    let run = verification_run(&harness.model).expect("the fixture selects its run");
    let family = latest_analysis(&harness.model, AnalysisType::Corner)
        .expect("the fixture holds a corner run");
    corner_matrix(
        run,
        family,
        None,
        &verification_specs(&harness.model),
        Some(&ReferenceCorner {
            process: "TT".to_owned(),
            temperature: 27.0,
        }),
    )
}

#[test]
fn the_corner_matrix_reads_every_output_at_every_corner_from_the_run() {
    let matrix = fixture_corner_matrix();
    assert_eq!(
        matrix
            .columns
            .iter()
            .map(|column| column.head.as_str())
            .collect::<Vec<_>>(),
        [
            "TT 1.8 V 27 °C",
            "SS 1.62 V 125 °C",
            "SS 1.62 V −40 °C",
            "FF 1.98 V 125 °C",
            "FF 1.98 V −40 °C",
            "SF 1.8 V 27 °C",
            "FS 1.8 V 27 °C",
        ]
    );
    assert_eq!(matrix.columns[1].detail, "SS · 1.62 V · 125 °C");
    assert_eq!(
        matrix.nominal,
        Some(0),
        "TT at 27 °C is the reference corner"
    );
    assert_eq!(matrix.corner_indices(), [1, 2, 3, 4, 5, 6]);
    assert_eq!(
        matrix
            .rows
            .iter()
            .map(|row| row.output.as_str())
            .collect::<Vec<_>>(),
        ["gain", "bw", "pm", "idd", "V(out)", "V(bias)"],
        "limited outputs in specification order, then the corner waveforms"
    );
    let gain = &matrix.rows[0];
    assert_eq!(gain.values[1], Some(39.4));
    assert_eq!(gain.verdict_of(39.4), Some(Verdict::Fail));
    assert_eq!(gain.verdict(), Some(Verdict::Fail));
    assert_eq!(matrix.rows[2].verdict_of(47.0), Some(Verdict::Near));
    assert_eq!(matrix.rows[3].verdict(), Some(Verdict::Near));
    assert_eq!(matrix.rows[1].verdict(), Some(Verdict::Pass));
    assert_eq!(
        matrix.rows[4].verdict(),
        None,
        "a node voltage has no limit"
    );
    assert_eq!(matrix.rows[4].unit, "V");
    assert_eq!(
        matrix.summary(),
        super::toolbar::Summary::failing(
            "3 / 4 outputs pass across 7 corners · worst SS 1.62 V 125 °C"
        )
    );
}

#[test]
fn a_corner_run_without_its_reference_point_has_no_nominal_column() {
    let harness = corner_app();
    let run = verification_run(&harness.model).expect("run");
    let family = latest_analysis(&harness.model, AnalysisType::Corner).expect("corner run");
    let matrix = corner_matrix(
        run,
        family,
        None,
        &verification_specs(&harness.model),
        Some(&ReferenceCorner {
            process: "TT".to_owned(),
            temperature: 85.0,
        }),
    );
    assert_eq!(matrix.nominal, None);
    assert_eq!(matrix.corner_indices().len(), 7);
}

/// A page with nothing to show is its toolbar and one line saying what
/// produces data: no table, no tiles.
#[test]
fn empty_verify_pages_are_one_line_under_the_toolbar() {
    for page in VerificationPage::NAVIGATION {
        let geometry = super::raster::page_geometry(SimulateTestHost::harness(), page);
        let toolbar = geometry.one("toolbar").expect("toolbar");
        let empty = geometry.one("empty").expect("empty line");
        assert_eq!(empty.top(), toolbar.bottom(), "{page:?}");
        assert_eq!(empty.height(), super::toolbar::EMPTY_LINE_H, "{page:?}");
        assert!(geometry.one("table.head").is_none(), "{page:?}");
        assert!(geometry.one("toolbar.summary").is_none(), "{page:?}");
    }
}

/// Regression is one table of every compared target, and the selected
/// target's detail in a pane directly under it.
#[test]
fn the_regression_page_is_one_table_and_the_selected_targets_detail() {
    use super::raster::{PAGE_WIDTH, page_geometry};

    let geometry = page_geometry(regression_app(), VerificationPage::Regression);
    let toolbar = geometry.one("toolbar").expect("toolbar");
    assert!(geometry.one("toolbar.summary").is_some());
    let head = geometry.one("table.head").expect("checks table");
    assert_eq!(
        head.top(),
        toolbar.bottom(),
        "nothing between toolbar and table"
    );
    let rows = geometry.all("table.row");
    assert_eq!(rows.len(), 5, "four measurements and one waveform");
    assert!(rows.iter().all(|row| row.height() == 24.0));
    assert_eq!(rows[0].top(), head.bottom());
    let detail = geometry
        .one("split")
        .expect("the selected waveform's detail");
    let last = rows.last().expect("rows");
    assert_eq!(
        detail.top(),
        last.bottom() + 1.0,
        "the detail sits under the table"
    );
    assert_eq!(detail.left(), 0.0);
    assert_eq!(detail.right(), PAGE_WIDTH);
    for (column, edges) in geometry.right_edges() {
        assert_eq!(edges.len(), 1, "{column}: numbers end on one edge");
    }
    assert_eq!(geometry.outside_insets(PAGE_WIDTH), Vec::<String>::new());
}

// ---------------------------------------------------------------------------
// The fixtures only these cases build
// ---------------------------------------------------------------------------

/// The canonical execution tag a transient task carries, which the receipt
/// checks its result against.
const TRANSIENT_TASK_TAG: u8 = 5;

/// A second-order step response settling to 1 V.
fn step_response(damping: f64, natural_rad_s: f64, time: &[f64]) -> Vec<f64> {
    let root = (1.0 - damping * damping).sqrt();
    let damped = natural_rad_s * root;
    time.iter()
        .map(|t| {
            1.0 - (-damping * natural_rad_s * t).exp()
                * ((damped * t).cos() + damping / root * (damped * t).sin())
        })
        .collect()
}

/// Three microseconds of a step, at 12.5 ns.
fn step_time() -> Vec<f64> {
    (0..=240).map(|index| f64::from(index) * 12.5e-9).collect()
}

/// A transient step of the active plan's one analysis, with its measurements.
fn step_analysis(damping: f64, natural_rad_s: f64, measurements: &[(&str, f64)]) -> AnalysisResult {
    let time = step_time();
    AnalysisResult::new(1, AnalysisType::Transient, "Step")
        .with_waveforms(vec![
            RetainedTrace::new(
                "V(out)",
                time.clone(),
                step_response(damping, natural_rad_s, &time),
                "#4c9be8",
            )
            .with_unit("V"),
        ])
        .with_measurements(
            measurements
                .iter()
                .map(|(name, value)| rspice_core::MeasureResult::success(name, *value))
                .collect(),
        )
}

/// Run 4 of the active plan against its baseline, Run 3: four measurements and
/// the output waveform. Bandwidth breaks its 2 % rule, overshoot has no rule,
/// and the waveform is selected, so its comparison opens under the table.
fn regression_app() -> FeatureHarness<SimulateFeature> {
    use crate::state::{
        RegressionComparisonMethod, RegressionComparisonWindow, RegressionSpecificationPolicy,
        RegressionTargetSelector, RegressionToleranceRule,
    };

    let mut harness = SimulateTestHost::harness();
    let plan_id = harness
        .model
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let source = AnalysisInstanceId::new();
    let run = |number: u64, damping: f64, natural: f64, values: [f64; 4]| {
        sealed_run(
            number,
            plan_id,
            source,
            TRANSIENT_TASK_TAG,
            step_analysis(
                damping,
                natural,
                &[
                    ("gain", values[0]),
                    ("bw", values[1]),
                    ("tsettle", values[2]),
                    ("overshoot", values[3]),
                ],
            ),
        )
    };
    let baseline = run(3, 0.55, 4.49e6, [42.30, 18.2e6, 1.62e-6, 12.6]);
    let candidate = run(4, 0.56, 4.45e6, [42.28, 17.1e6, 1.64e-6, 12.1]);
    let baseline_id = baseline.run_id;
    let rule = |target: RegressionTargetSelector, absolute: f64, relative: f64| {
        let waveform = target.kind == crate::state::RegressionTargetKind::Waveform;
        RegressionToleranceRule {
            target,
            method: RegressionComparisonMethod::AbsoluteRelativeEnvelope,
            absolute_tolerance: absolute,
            relative_tolerance: relative,
            time_skew_allowance: if waveform { 20.0e-9 } else { 0.0 },
            comparison_window: waveform.then_some(RegressionComparisonWindow {
                start: 0.0,
                end: 3.0e-6,
            }),
        }
    };
    let mut rules = super::derive_regression_checks(&baseline, &candidate)
        .into_iter()
        .filter_map(|check| match check.name.as_str() {
            "gain" => Some(rule(check.target, 0.1, 0.005)),
            "bw" => Some(rule(check.target, 0.0, 0.02)),
            "tsettle" => Some(rule(check.target, 0.0, 0.05)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let waveform = super::regression_waveform_pairs(&baseline, &candidate)
        .first()
        .map(|pair| pair.target.clone())
        .expect("both runs hold V(out)");
    rules.push(rule(waveform.clone(), 0.02, 0.02));
    let payload = harness
        .model
        .workspace
        .content
        .ensure_active_plan_data(plan_id);
    payload.regression_tolerances = rules;
    payload.specification_policy.regression = RegressionSpecificationPolicy::LimitAndWaveform;
    harness.model.simulation.retained.runs = vec![baseline, candidate].into();
    assert!(harness.model.simulation.select_run(1));
    super::commit_regression_baseline(&mut harness.cx(), baseline_id)
        .expect("the baseline commits");
    harness.state.verification.regression_selected_target = Some(waveform);
    harness
}

/// A seven-corner run of four limited outputs and two node voltages: gain
/// fails at SS 125 °C, and phase margin and supply current sit near their
/// limits at FF 125 °C.
fn corner_app() -> FeatureHarness<SimulateFeature> {
    let mut harness = SimulateTestHost::harness();
    seed_corner(&mut harness.model);
    harness
}
