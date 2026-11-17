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

    current.analyses[0].waveforms[0].x = vec![0.0, 1.0, 2.5].into();
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
    let cases = regression_export_cases(&[], &[], &[], &[], &[], &[]);
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
        unit: String::new(),
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
    let refresh = |harness: &mut FeatureHarness<SimulateFeature>| {
        let model = &mut harness.model;
        model
            .simulation
            .refresh_verification(&model.sim_setup, &model.workspace.content);
    };
    refresh(&mut harness);
    assert_eq!(
        verification_evidence_domain(&harness.model),
        EvidenceDomain::Legacy,
        "the corner fixture's run predates receipts, which Verify reads"
    );
    let sheet = harness
        .model
        .simulation
        .verification_sheet()
        .expect("a sheet");
    assert!(
        sheet.is_judged() && sheet.rows.len() == 4,
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
    refresh(&mut harness);

    assert_ne!(other, plan_id);
    assert_eq!(
        verification_evidence_domain(&harness.model),
        EvidenceDomain::AnotherPlan
    );
    let sheet = harness
        .model
        .simulation
        .verification_sheet()
        .expect("the run is still the one Verify reads");
    assert!(
        !sheet.is_judged(),
        "another plan's dataset answers none of this plan's limits"
    );
    assert!(
        sheet.points.len() == 7
            && sheet
                .rows
                .iter()
                .all(|row| row.cells.iter().all(|cell| cell.value.is_some())),
        "the values the run did measure stay on the page"
    );
    assert!(
        sheet
            .rows
            .iter()
            .flat_map(|row| &row.cells)
            .all(|cell| { cell.status == rspice_results::verification::CellStatus::Pass }),
        "and nothing in it is judged"
    );
}

// ---------------------------------------------------------------------------
// The page grammar: corners and SOA
// ---------------------------------------------------------------------------

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

/// A seven-corner run of four limited outputs and two node voltages: gain
/// fails at SS 125 °C, and phase margin and supply current sit near their
/// limits at FF 125 °C.
fn corner_app() -> FeatureHarness<SimulateFeature> {
    let mut harness = SimulateTestHost::harness();
    seed_corner(&mut harness.model);
    harness
}

/// On a phone every table that states a verdict keeps it in view: it sits
/// directly after the name, inside the window, and stays put while the values
/// scroll sideways under it.
#[test]
fn a_phone_keeps_every_tables_verdict_in_view() {
    use super::raster::{page_pass, seed_reference, seed_soa};
    let phone = egui::vec2(390.0, 844.0);
    for page in [
        VerificationPage::Specifications,
        VerificationPage::Checks,
        VerificationPage::Reference,
    ] {
        let mut harness = SimulateTestHost::harness();
        match page {
            VerificationPage::Checks => seed_soa(&mut harness.model),
            VerificationPage::Reference => seed_reference(&mut harness.model),
            _ => seed_corner(&mut harness.model),
        }
        harness
            .wb
            .activate(crate::workbench::state::Workspace::Verify);
        harness.state.verification_page = page;
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let pass = |harness: &mut FeatureHarness<SimulateFeature>| {
            let mut geometry = super::raster::PageGeometry::default();
            for _ in 0..3 {
                let input = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, phone)),
                    ..Default::default()
                };
                geometry = page_pass(&ctx, input, |ui| {
                    super::raster::refresh_verification(&mut harness.model);
                    super::show(ui, &mut harness.cx());
                })
                .1;
            }
            geometry
        };
        let before = pass(&mut harness);
        let name = before.one("table.head.0").expect("the name's head");
        let verdict = before.one("table.pinned").expect("the verdict's head");
        assert_eq!(verdict.left(), name.right(), "{page:?}: after the name");
        assert!(
            verdict.right() <= phone.x,
            "{page:?}: the verdict is in view, {verdict:?}"
        );
        let table = match page {
            VerificationPage::Specifications => {
                &mut harness.state.verification.specifications_table
            }
            VerificationPage::Checks => &mut harness.state.verification.checks_table,
            _ => &mut harness.state.verification.reference_table,
        };
        table.scroll_x = 120.0;
        let after = pass(&mut harness);
        assert_eq!(
            after.one("table.pinned"),
            Some(verdict),
            "{page:?}: the verdict stays put as the values scroll"
        );
    }
}
