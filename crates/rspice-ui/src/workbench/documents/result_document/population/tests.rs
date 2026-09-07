//! Population identity, retained requirement contracts, and statistical readouts.
//! Prepared fixtures also cross sealing and project restoration so a correct
//! live projection cannot conceal a different persisted verdict.

use super::*;
use crate::state::{
    AnalysisType, FamilyMeasurementEvidence, FamilyMemberMeasurements, MonteCarloVariableMetadata,
    SpecPointScope,
};

fn variable(name: &str, samples: Vec<f64>) -> MonteCarloVariableMetadata {
    let count = samples.len() as f64;
    let mean = samples.iter().sum::<f64>() / count;
    let variance = samples.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (count - 1.0);
    MonteCarloVariableMetadata {
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
    workspace.specs.push(SpecEntry {
        measurement: "gain_dc".to_owned(),
        expression: String::new(),
        min,
        max,
        unit: "dB".to_owned(),
        scope: SpecPointScope::AllPoints,
    });
    workspace
}

fn key() -> AnalysisPresentationKey {
    AnalysisPresentationKey::new(
        crate::product::DatasetId::new(),
        &AnalysisResult::new(1, AnalysisType::MonteCarlo, "MC"),
    )
}

fn retained_population(count: usize) -> crate::state::SimulationState {
    use crate::io::project_io::ProjectSimulationResults;
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
    ProjectSimulationResults::from_state(&simulation)
        .into_simulation_state()
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

    workspace.specs[0].measurement = "gain_ac".to_owned();
    let renamed = cached_population(&simulation, &workspace, &mut results).unwrap();
    let column = &renamed.columns[renamed.column_index("gain_dc").unwrap()];
    assert!(
        column.limit.is_none(),
        "the requirement now names another measurement"
    );
    assert!(column.unit.is_empty());
    assert_eq!(renamed.failing_count(), 0);

    workspace.specs[0].measurement = "gain_dc".to_owned();
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
    workspace.specs[0].unit = "V".to_owned();
    let changed = cached_population(&simulation, &workspace, &mut results).unwrap();
    let column = &changed.columns[changed.column_index("gain_dc").unwrap()];
    assert_eq!(column.unit, "V");
    assert_eq!(
        column.limit.as_ref().unwrap().text,
        workspace.specs[0].limit_text()
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

    workspace.specs[0].max = Some(0.0);
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

    workspace.specs[0].max = None;
    let unbounded = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert!(
        unbounded.columns[unbounded.column_index("gain_dc").unwrap()]
            .limit
            .is_none()
    );
}

#[test]
fn population_cache_refreshes_restored_same_identity_content() {
    use crate::io::project_io::ProjectSimulationResults;

    let mut simulation = retained_population(3);
    let workspace = workspace_with_limit(Some(1.0), None);
    let mut results = super::super::ResultsState::default();
    let original = cached_population(&simulation, &workspace, &mut results).unwrap();
    let version = simulation.data_version;
    let mut replacement = simulation.clone();
    replacement.runs[0].analyses[0] =
        monte_carlo(vec![trial(0, 10.0), trial(1, 20.0)], vec![10.0, 20.0]);
    simulation = ProjectSimulationResults::from_state(&replacement)
        .into_simulation_state()
        .unwrap();
    assert_eq!(simulation.data_version, version);
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
    let version = simulation.data_version;
    let retained = simulation.runs[0].analyses[0].clone();
    if let Some(AnalysisResultFamilyMetadata::MonteCarlo { variables, .. }) =
        simulation.runs[0].analyses[0].family_metadata.as_mut()
    {
        variables[0].samples[0] = f64::NAN;
    } else {
        panic!("Monte Carlo fixture");
    }
    assert!(cached_population(&simulation, &workspace, &mut results).is_none());

    simulation.runs[0].analyses[0] = retained;
    let repaired = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert_eq!(repaired.status, original.status);
    assert_eq!(simulation.data_version, version);
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
        }) = cloned_simulation.runs[0].analyses[0]
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
    let mut requirement =
        SpecificationDefinition::from_legacy(SimulationPlanId::new(), 0, &workspace.specs[0]);
    requirement.producing_analysis = Some(producer);
    let retain_requirement = configure(&mut requirement);
    let receipt = PreparedRunReceipt::new_with_project_model_sources_and_specifications(
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        ContentDigest::from_bytes([1; 32]),
        ContentDigest::from_bytes([2; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([3; 32])),
        Vec::new(),
        if retain_requirement {
            vec![PreparedSpecification::from_definition(requirement).unwrap()]
        } else {
            Vec::new()
        },
        vec![
            PreparedRunTaskReceipt::new(
                producer,
                ObjectRevision::INITIAL,
                Vec::new(),
                crate::state::CanonicalAnalysisKind::MonteCarlo.tag(),
                ContentDigest::from_bytes([4; 32]),
            )
            .unwrap(),
        ],
    )
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
    simulation.runs = vec![run].into();
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
        let analysis = &mut simulation.runs[0].analyses[0];
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
    workspace.specs[0].min = Some(-10.0);
    workspace.specs[0].unit = "V".into();
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
    let verdict = crate::state::SpecificationVerdict::evaluate(
        run.prepared_receipt().unwrap().specifications(),
        &run.analyses,
    );
    assert_eq!(
        verdict[0].passing_evidence_count() as usize,
        plan.trial_count() - plan.failing_count()
    );

    workspace.specs[0].min = Some(f64::NAN);
    assert!(Arc::ptr_eq(
        &plan,
        &cached_population(&simulation, &workspace, &mut results).unwrap()
    ));
    workspace.specs.clear();
    assert!(Arc::ptr_eq(
        &plan,
        &cached_population(&simulation, &workspace, &mut results).unwrap()
    ));
    simulation.runs[0].mark_running().unwrap();
    simulation.runs[0]
        .finish_lifecycle(crate::state::SimulationRunLifecycle::Completed)
        .unwrap();
    simulation.complete_run();
    let sealed = simulation.runs[0].specification_verdicts().unwrap();
    assert_eq!(sealed[0].passing_evidence_count(), 1);
    simulation = crate::io::project_io::ProjectSimulationResults::from_state(&simulation)
        .into_simulation_state()
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
    let mut missing = workspace.specs[0].clone();
    missing.measurement = "missing".into();
    workspace.specs.push(missing);
    let mut results = super::super::ResultsState::default();
    let plan = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert_eq!(plan.status, vec![TrialStatus::Unmeasured; 3]);
    let column = &plan.columns[plan.column_index("missing").unwrap()];
    assert!(column.values.iter().all(Option::is_none));
    workspace.specs.pop();
    if let Some(AnalysisResultFamilyMetadata::MonteCarlo {
        member_measurements,
        ..
    }) = simulation.runs[0].analyses[0].family_metadata.as_mut()
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
            workspace.specs.push(workspace.specs[0].clone());
        } else {
            workspace.specs[0].min = Some(f64::NAN);
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
        workspace.specs = workspace_with_limit(Some(-1.0), None).specs;
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
    workspace.specs[0].scope = SpecPointScope::Nominal;
    let narrowed = cached_population(&simulation, &workspace, &mut results).unwrap();
    assert_eq!(narrowed.status, vec![TrialStatus::NotEvaluated; 3]);
    assert!(narrowed.columns.iter().all(|column| column.limit.is_none()));
    workspace.specs[0].scope = SpecPointScope::AllPoints;
    assert_eq!(
        cached_population(&simulation, &workspace, &mut results)
            .unwrap()
            .failing_count(),
        1
    );
}

/// A complete run pairs its sampled variables with its measurements, and
/// the requirement decides which trials failed.
#[test]
fn a_complete_run_pairs_its_variables_with_its_measurements() {
    let analysis = monte_carlo(
        vec![trial(0, 40.0), trial(1, 39.0), trial(2, 41.0)],
        vec![1.0, 2.0, 3.0],
    );
    let plan = build(
        &analysis,
        key(),
        (crate::state::RunHistory::default().revision(), 1),
        &workspace_with_limit(Some(39.5), None),
        None,
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

/// A run that dropped a trial has no correspondence between its sampled
/// variables and its measurements, and must not invent one.
#[test]
fn a_run_that_dropped_a_trial_refuses_to_pair_variables_with_measurements() {
    // The driver requested four and retained trials 0, 1 and 3.
    let analysis = monte_carlo(
        vec![trial(0, 40.0), trial(1, 40.5), trial(3, 41.0)],
        vec![1.0, 2.0, 3.0],
    );
    let plan = build(
        &analysis,
        key(),
        (crate::state::RunHistory::default().revision(), 1),
        &ProjectWorkspace::default(),
        None,
    )
    .expect("a population is built");

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

/// A trial whose measurement produced no number is evidence the analysis
/// could not take — never a failure.
#[test]
fn an_unmeasured_trial_is_not_a_failing_trial() {
    let mut members = vec![trial(0, 40.0), trial(1, 40.0)];
    members[1].measurements[0].value = None;
    members[1].measurements[0].passed = false;
    let analysis = monte_carlo(members, vec![1.0, 2.0]);
    let plan = build(
        &analysis,
        key(),
        (crate::state::RunHistory::default().revision(), 1),
        &workspace_with_limit(Some(39.5), None),
        None,
    )
    .expect("a population is built");

    assert_eq!(plan.status, [TrialStatus::Passing, TrialStatus::Unmeasured]);
    assert_eq!(plan.failing_count(), 0);
}

/// The quartiles are the interpolated definition, and Tukey whiskers stop
/// at the furthest sample inside 1.5 IQR rather than at the fence itself.
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

/// A perfect yield keeps a confidence bound: the interval is Wilson's,
/// not the normal approximation that collapses to zero width there.
#[test]
fn a_perfect_yield_still_carries_an_interval() {
    let (low, high) = wilson_interval(1_000, 1_000).expect("a thousand trials");
    assert!(low > 99.0 && low < 100.0, "{low}");
    assert!((high - 100.0).abs() < 1.0e-9, "{high}");

    let (low, high) = wilson_interval(986, 1_000).expect("a thousand trials");
    assert!(low > 97.6 && low < 98.7, "{low}");
    assert!(high > 98.5 && high < 99.3, "{high}");
}

/// Three sigma of margin to the nearest bound is Cpk 1, in one direction
/// or in both.
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

/// The margin percentage is measured against the half-width of a
/// two-sided requirement and against the bound of a one-sided one.
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

/// A straight line correlates perfectly and its fit recovers the line.
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

/// The kernel density integrates to one over the sample range, which is
/// what makes one violin's width comparable with the next.
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
