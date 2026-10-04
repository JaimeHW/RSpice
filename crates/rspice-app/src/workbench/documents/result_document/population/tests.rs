//! Population identity, retained requirement contracts, and statistical readouts.
//! Prepared fixtures also cross sealing and project restoration so a correct
//! live projection cannot conceal a different persisted verdict.

use super::*;
use crate::state::{
    AnalysisResultFamilyMetadata, AnalysisType, FamilyMeasurementEvidence, FamilyMemberId,
    FamilyMemberMeasurements, MonteCarloVariableMetadata, SpecPointScope,
};
use rspice_results::population::{TrialStatus, cpk};

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

fn monte_carlo(members: Vec<FamilyMemberMeasurements>, samples: Vec<f64>) -> AnalysisResult {
    let completed = members.len();
    AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC").with_family_metadata(
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

fn workspace_with_limit(min: Option<f64>, max: Option<f64>) -> ProjectWorkspace {
    let mut workspace = ProjectWorkspace::default();
    workspace.content.specs.push(SpecEntry {
        measurement: "gain_dc".to_owned(),
        expression: String::new(),
        min,
        max,
        unit: "dB".to_owned(),
        scope: SpecPointScope::AllPoints,
    });
    workspace
}

fn retained_population(count: usize) -> crate::state::SimulationState {
    use crate::state::{SimulationRunLifecycle, SimulationRunProvenance, SimulationState};

    let mut simulation = SimulationState::default();
    let run = simulation.start_run();
    run.add_analysis(monte_carlo(
        (0..count).map(|index| trial(index, index as f64)).collect(),
        (0..count).map(|index| index as f64).collect(),
    ));
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .unwrap();
    run.mark_running().unwrap();
    run.finish_lifecycle(SimulationRunLifecycle::Completed)
        .unwrap();
    simulation.complete_run();
    crate::io::simulation_state_from_results(crate::io::capture_simulation_results(&simulation))
        .unwrap()
}

fn cached_population(
    simulation: &crate::state::SimulationState,
    workspace: &ProjectWorkspace,
    results: &mut super::super::ResultsState,
) -> Option<Arc<PopulationPlan>> {
    plan(&mut SheetContext {
        simulation,
        workspace,
        results,
        policy: crate::quantity::QuantityPresentationPolicy::default(),
    })
}

#[test]
fn population_cache_refreshes_a_same_length_measurement_rename() {
    let simulation = retained_population(3);
    let mut workspace = workspace_with_limit(Some(1.0), None);
    let mut results = super::super::ResultsState::default();
    let original = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert_eq!(original.failing_count(), 1);

    workspace.content.specs[0].measurement = "gain_ac".to_owned();
    let renamed = cached_population(&simulation, &workspace, &mut results).unwrap();
    let column = &renamed.columns[renamed.column_index("gain_dc").unwrap()];
    assert!(
        column.limit.is_none(),
        "the requirement now names another measurement"
    );
    assert!(column.unit.is_empty());
    assert_eq!(renamed.failing_count(), 0);

    workspace.content.specs[0].measurement = "gain_dc".to_owned();
    assert_eq!(
        cached_population(&simulation, &workspace, &mut results)
            .unwrap()
            .failing_count(),
        1
    );
}

#[test]
fn population_cache_refreshes_requirement_units_and_limit_text() {
    let simulation = retained_population(3);
    let mut workspace = workspace_with_limit(Some(1.0), None);
    let mut results = super::super::ResultsState::default();
    let original = cached_population(&simulation, &workspace, &mut results).unwrap();
    workspace.content.specs[0].unit = "V".to_owned();
    let changed = cached_population(&simulation, &workspace, &mut results).unwrap();
    let column = &changed.columns[changed.column_index("gain_dc").unwrap()];
    assert_eq!(column.unit, "V");
    assert_eq!(
        column.limit.as_ref().unwrap().text,
        workspace.content.specs[0].limit_text()
    );
    assert_eq!(changed.status, original.status);
}

#[test]
fn population_cache_distinguishes_an_absent_bound_from_zero() {
    let simulation = retained_population(3);
    let mut workspace = workspace_with_limit(None, None);
    let mut results = super::super::ResultsState::default();
    let original = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert_eq!(original.failing_count(), 0);

    workspace.content.specs[0].max = Some(0.0);
    let bounded = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert_eq!(bounded.failing_count(), 2);
    assert_eq!(
        bounded.columns[bounded.column_index("gain_dc").unwrap()]
            .limit
            .as_ref()
            .unwrap()
            .max(),
        Some(0.0)
    );

    workspace.content.specs[0].max = None;
    let unbounded = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert!(
        unbounded.columns[unbounded.column_index("gain_dc").unwrap()]
            .limit
            .is_none()
    );
}

#[test]
fn population_cache_refreshes_restored_same_identity_content() {
    let mut simulation = retained_population(3);
    let workspace = workspace_with_limit(Some(1.0), None);
    let mut results = super::super::ResultsState::default();
    let original = cached_population(&simulation, &workspace, &mut results).unwrap();
    let version = simulation.view.data_version;
    let mut replacement = simulation.clone();
    replacement.retained.runs[0].analyses[0] =
        monte_carlo(vec![trial(0, 10.0), trial(1, 20.0)], vec![10.0, 20.0]);
    simulation = crate::io::simulation_state_from_results(crate::io::capture_simulation_results(
        &replacement,
    ))
    .unwrap();
    assert_eq!(simulation.view.data_version, version);
    let restored = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert_eq!(restored.analysis, original.analysis);
    assert_eq!(restored.trial_count(), 2);
    assert_eq!(
        restored.columns[restored.column_index("gain_dc").unwrap()].values,
        [Some(10.0), Some(20.0)]
    );
    assert_eq!(restored.failing_count(), 0);
    assert_eq!(
        original.trial_count(),
        3,
        "an older shared plan stays immutable"
    );
}

#[test]
fn population_cache_rejects_corrupted_evidence_and_accepts_repair_without_a_frame() {
    let mut simulation = retained_population(3);
    let workspace = workspace_with_limit(Some(1.0), None);
    let mut results = super::super::ResultsState::default();
    let original = cached_population(&simulation, &workspace, &mut results).unwrap();
    let version = simulation.view.data_version;
    let retained = simulation.retained.runs[0].analyses[0].clone();
    if let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) =
        simulation.retained.runs[0].analyses[0]
            .family_metadata
            .as_mut()
    {
        variables[0].samples[0] = f64::NAN;
    } else {
        panic!("Monte Carlo fixture");
    }
    assert!(cached_population(&simulation, &workspace, &mut results).is_none());

    simulation.retained.runs[0].analyses[0] = retained;
    let repaired = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert_eq!(repaired.status, original.status);
    assert_eq!(simulation.view.data_version, version);
}

#[test]
fn population_cache_reuses_unchanged_clones_and_isolates_nested_edits() {
    for count in [3, 100_000] {
        let simulation = retained_population(count);
        let workspace = workspace_with_limit(Some(1.0), None);
        let mut results = super::super::ResultsState::default();
        let original = cached_population(&simulation, &workspace, &mut results).unwrap();
        let mut cloned_simulation = simulation.clone();
        let mut cloned_results = results.clone();
        for _ in 0..12 {
            assert!(Arc::ptr_eq(
                &original,
                &cached_population(&simulation, &workspace, &mut results).unwrap()
            ));
            assert!(Arc::ptr_eq(
                &original,
                &cached_population(&cloned_simulation, &workspace, &mut cloned_results).unwrap()
            ));
        }
        if let Some(AnalysisResultFamilyMetadata::MonteCarlo {
            member_measurements,
            ..
        }) = cloned_simulation.retained.runs[0].analyses[0]
            .family_metadata
            .as_mut()
        {
            member_measurements[0].measurements[0].value = Some(10.0);
        } else {
            panic!("Monte Carlo fixture");
        }
        let changed =
            cached_population(&cloned_simulation, &workspace, &mut cloned_results).unwrap();
        assert_eq!(changed.failing_count(), 0);
        assert_eq!(original.failing_count(), 1);
        assert!(Arc::ptr_eq(
            &original,
            &cached_population(&simulation, &workspace, &mut results).unwrap()
        ));
    }
}

fn prepared_population(
    values: &[f64],
    configure: impl FnOnce(&mut crate::state::SpecificationDefinition),
) -> (crate::state::SimulationState, ProjectWorkspace) {
    prepared_population_contract(values, |spec| {
        configure(spec);
        true
    })
}

fn prepared_population_contract(
    values: &[f64],
    configure: impl FnOnce(&mut crate::state::SpecificationDefinition) -> bool,
) -> (crate::state::SimulationState, ProjectWorkspace) {
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision, SimulationPlanId};
    use crate::state::{
        AnalysisResultProvenance, AnalysisResultPvtPoint, AnalysisResultSourceDomain,
        PreparedRunReceipt, PreparedRunTaskReceipt, PreparedSourceCheckReceipt,
        PreparedSpecification, SimulationRun, SimulationState, SpecificationDefinition,
    };
    let producer = AnalysisInstanceId::new();
    let workspace = workspace_with_limit(Some(1.0), None);
    let mut requirement = SpecificationDefinition::from_legacy(
        SimulationPlanId::new(),
        0,
        &workspace.content.specs[0],
    );
    requirement.producing_analysis = Some(producer);
    let retain_requirement = configure(&mut requirement);
    let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(SimulationPlanId::new()),
        project_revision: ObjectRevision::INITIAL,
        prepared_snapshot_digest: ContentDigest::from_bytes([1; 32]),
        source_content_digest: ContentDigest::from_bytes([2; 32]),
        source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes(
            [3; 32],
        )),
        project_model_sources: Vec::new(),
        specifications: if retain_requirement {
            vec![PreparedSpecification::from_definition(requirement).unwrap()]
        } else {
            Vec::new()
        },
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![
            PreparedRunTaskReceipt::new(
                producer,
                ObjectRevision::INITIAL,
                Vec::new(),
                crate::state::CanonicalAnalysisKind::MonteCarlo.tag(),
                ContentDigest::from_bytes([4; 32]),
            )
            .unwrap(),
        ],
    })
    .unwrap();
    let analysis = monte_carlo(
        values
            .iter()
            .enumerate()
            .map(|(i, v)| trial(i, *v))
            .collect(),
        values.to_vec(),
    )
    .with_provenance(
        AnalysisResultProvenance::new(
            producer,
            ObjectRevision::INITIAL,
            receipt.prepared_snapshot_digest(),
            Vec::new(),
        )
        .unwrap()
        .with_pvt_point(Some(
            AnalysisResultPvtPoint::new("tt", None, 27.0, None, true).unwrap(),
        )),
    );
    let mut run = SimulationRun::new_prepared(1, receipt);
    run.add_analysis(analysis);
    let mut simulation = SimulationState::default();
    simulation.retained.runs = vec![run].into();
    assert!(simulation.select_run(0));
    assert!(simulation.select_analysis(0));
    (simulation, workspace)
}

#[test]
fn population_contract_an_empty_prepared_contract_never_borrows_live_bounds() {
    let (simulation, workspace) = prepared_population_contract(&[0.0, 1.0, 2.0], |_| false);
    let mut results = super::super::ResultsState::default();
    let plan = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert_eq!(plan.status, vec![TrialStatus::NotEvaluated; 3]);
    assert!(plan.columns.iter().all(|column| column.limit.is_none()));
    assert_eq!(
        plan.requirement_note,
        "Requirements retained with this run."
    );
}

#[test]
fn population_contract_nominal_requires_the_retained_point_attribution() {
    for nominal in [Some(true), Some(false), None] {
        let (mut simulation, workspace) = prepared_population(&[0.0, 1.0, 2.0], |spec| {
            spec.scope = SpecPointScope::Nominal
        });
        let analysis = &mut simulation.retained.runs[0].analyses[0];
        let provenance = analysis
            .provenance()
            .unwrap()
            .clone()
            .with_pvt_point(nominal.map(|nominal| {
                crate::state::AnalysisResultPvtPoint::new("tt", None, 27.0, None, nominal).unwrap()
            }));
        *analysis = analysis.clone().with_provenance(provenance);
        let mut results = super::super::ResultsState::default();
        let plan = cached_population(&simulation, &workspace, &mut results).unwrap();
        assert_eq!(
            plan.columns[plan.column_index("gain_dc").unwrap()]
                .limit
                .is_some(),
            nominal == Some(true)
        );
    }
}

#[test]
fn population_contract_uses_the_frozen_requirement_and_ignores_later_drafts() {
    let (mut simulation, mut workspace) =
        prepared_population(&[0.0, 1.0, 2.0], |spec| spec.guard_band = Some(0.25));
    workspace.content.specs[0].min = Some(-10.0);
    workspace.content.specs[0].unit = "V".into();
    let mut results = super::super::ResultsState::default();
    let plan = cached_population(&simulation, &workspace, &mut results).unwrap();
    let column = &plan.columns[plan.column_index("gain_dc").unwrap()];
    assert_eq!(column.unit, "dB");
    assert_eq!(plan.failing_count(), 2);
    assert_eq!(
        column.limit.as_ref().unwrap().signed_margin(1.0),
        Some(-0.25)
    );
    let run = simulation.active_run().unwrap();
    let verdict = rspice_results::specification_verdict::SpecificationVerdict::evaluate(
        run.prepared_receipt().unwrap().specifications(),
        run.analyses.iter().map(|analysis| &analysis.data),
    );
    assert_eq!(
        verdict[0].passing_evidence_count() as usize,
        plan.trial_count() - plan.failing_count()
    );

    workspace.content.specs[0].min = Some(f64::NAN);
    assert!(Arc::ptr_eq(
        &plan,
        &cached_population(&simulation, &workspace, &mut results).unwrap()
    ));
    workspace.content.specs.clear();
    assert!(Arc::ptr_eq(
        &plan,
        &cached_population(&simulation, &workspace, &mut results).unwrap()
    ));
    simulation.retained.runs[0].mark_running().unwrap();
    simulation.retained.runs[0]
        .finish_lifecycle(crate::state::SimulationRunLifecycle::Completed)
        .unwrap();
    simulation.complete_run();
    let sealed = simulation.retained.runs[0]
        .specification_verdicts()
        .unwrap();
    assert_eq!(sealed[0].passing_evidence_count(), 1);
    simulation = crate::io::simulation_state_from_results(crate::io::capture_simulation_results(
        &simulation,
    ))
    .unwrap();
    let restored = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert_eq!(restored.status, plan.status);
    assert_eq!(
        restored.columns[restored.column_index("gain_dc").unwrap()].unit,
        "dB"
    );
}

#[test]
fn population_contract_applies_guard_bands_to_every_bound_shape_and_capability() {
    use crate::state::SpecificationComparison;
    for (comparison, expected) in [
        (
            SpecificationComparison::Minimum { limit: 1.0 },
            [-1.25, -0.25, 0.75],
        ),
        (
            SpecificationComparison::Maximum { limit: 1.0 },
            [0.75, -0.25, -1.25],
        ),
        (
            SpecificationComparison::Range {
                minimum: 0.0,
                maximum: 2.0,
            },
            [-0.25, 0.75, -0.25],
        ),
        (
            SpecificationComparison::EqualWithin {
                target: 1.0,
                tolerance: 1.0,
            },
            [-0.25, 0.75, -0.25],
        ),
    ] {
        let (simulation, workspace) = prepared_population(&[0.0, 1.0, 2.0], |spec| {
            spec.comparison = comparison;
            spec.guard_band = Some(0.25);
        });
        let mut results = super::super::ResultsState::default();
        let plan = cached_population(&simulation, &workspace, &mut results).unwrap();
        let limit = plan.columns[plan.column_index("gain_dc").unwrap()]
            .limit
            .as_ref()
            .unwrap();
        for (value, margin) in expected.into_iter().enumerate() {
            assert_eq!(limit.signed_margin(value as f64), Some(margin));
        }
        assert_eq!(cpk(&[0.0, 1.0, 2.0], limit), Some(expected[1] / 3.0));
        assert!(limit.text.contains("guard band 250m dB"));
        assert_eq!(limit.normalized().signed_margin(0.0), Some(0.0));
        if expected[1] > 0.0 {
            assert_eq!(limit.margin_percent(1.0), Some(100.0));
            assert_eq!(limit.min(), Some(0.25));
            assert_eq!(limit.max(), Some(1.75));
        }
    }
}

#[test]
fn population_contract_rejects_another_producer_or_pvt_scope() {
    for selector in 0..3 {
        let (simulation, workspace) =
            prepared_population(&[0.0, 1.0, 2.0], |spec| match selector {
                0 => spec.producing_analysis = Some(crate::product::AnalysisInstanceId::new()),
                1 => {
                    spec.scope = SpecPointScope::SelectedCorners {
                        corners: vec!["ff".into()],
                    }
                }
                _ => {
                    spec.scope = SpecPointScope::SelectedCorners {
                        corners: vec!["TT".into()],
                    }
                }
            });
        let mut results = super::super::ResultsState::default();
        let plan = cached_population(&simulation, &workspace, &mut results).unwrap();
        let limit = &plan.columns[plan.column_index("gain_dc").unwrap()].limit;
        assert_eq!(limit.is_some(), selector == 2);
    }
}

#[test]
fn population_contract_guard_band_keeps_the_verdicts_exact_margin_order() {
    let (simulation, workspace) = prepared_population(&[1e16, 1e16 + 2.0], |spec| {
        spec.comparison = crate::state::SpecificationComparison::Minimum { limit: 1e16 };
        spec.guard_band = Some(0.1);
    });
    let mut results = super::super::ResultsState::default();
    let plan = cached_population(&simulation, &workspace, &mut results).unwrap();
    let limit = plan.columns[plan.column_index("gain_dc").unwrap()]
        .limit
        .as_ref()
        .unwrap();
    assert_eq!(limit.signed_margin(1e16), Some(-0.1));
    assert!(!limit.passes(1e16));
    assert!(limit.passes(1e16 + 2.0));
    assert_eq!(plan.failing_count(), 1);
}

#[test]
fn population_contract_keeps_missing_measurements_in_the_trial_verdict() {
    let mut simulation = retained_population(3);
    let mut workspace = workspace_with_limit(Some(-1.0), None);
    let mut missing = workspace.content.specs[0].clone();
    missing.measurement = "missing".into();
    workspace.content.specs.push(missing);
    let mut results = super::super::ResultsState::default();
    let plan = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert_eq!(plan.status, vec![TrialStatus::Unmeasured; 3]);
    let column = &plan.columns[plan.column_index("missing").unwrap()];
    assert!(column.values.iter().all(Option::is_none));
    workspace.content.specs.pop();
    if let Some(AnalysisResultFamilyMetadata::MonteCarlo {
        member_measurements,
        ..
    }) = simulation.retained.runs[0].analyses[0]
        .family_metadata
        .as_mut()
    {
        member_measurements[0].measurements[0].passed = false;
    }
    assert_eq!(
        cached_population(&simulation, &workspace, &mut results)
            .unwrap()
            .status[0],
        TrialStatus::Unmeasured
    );
}

#[test]
fn population_contract_never_accepts_invalid_or_duplicate_legacy_requirements() {
    let simulation = retained_population(3);
    for duplicate in [false, true] {
        let mut workspace = workspace_with_limit(Some(-1.0), None);
        if duplicate {
            workspace
                .content
                .specs
                .push(workspace.content.specs[0].clone());
        } else {
            workspace.content.specs[0].min = Some(f64::NAN);
        }
        let mut results = super::super::ResultsState::default();
        let plan = cached_population(&simulation, &workspace, &mut results).unwrap();
        assert!(
            plan.status
                .iter()
                .all(|status| *status != TrialStatus::Passing)
        );
        assert!(plan.columns.iter().all(|column| column.limit.is_none()));
        assert!(
            plan.requirement_note
                .starts_with("Requirements unavailable:")
        );
        workspace.content.specs = workspace_with_limit(Some(-1.0), None).content.specs;
        let repaired = cached_population(&simulation, &workspace, &mut results).unwrap();
        assert_eq!(repaired.status, vec![TrialStatus::Passing; 3]);
    }
}

#[test]
fn population_contract_legacy_scope_edits_cannot_reuse_an_earlier_binding() {
    let simulation = retained_population(3);
    let mut workspace = workspace_with_limit(Some(1.0), None);
    let mut results = super::super::ResultsState::default();
    assert_eq!(
        cached_population(&simulation, &workspace, &mut results)
            .unwrap()
            .failing_count(),
        1
    );
    workspace.content.specs[0].scope = SpecPointScope::Nominal;
    let narrowed = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert_eq!(narrowed.status, vec![TrialStatus::NotEvaluated; 3]);
    assert!(narrowed.columns.iter().all(|column| column.limit.is_none()));
    workspace.content.specs[0].scope = SpecPointScope::AllPoints;
    assert_eq!(
        cached_population(&simulation, &workspace, &mut results)
            .unwrap()
            .failing_count(),
        1
    );
}
