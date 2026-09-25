//! Optimizer candidates use the same frozen analysis and scalar selection as MC.

use super::*;

pub(crate) fn run_optimization(
    base: &StudyRunConfig,
    config: &services::OptimizationRunConfig,
    source: &str,
    source_path: Option<&Path>,
    environment: Option<AnalysisExecutionEnvironment>,
    abort: &dyn AbortSignal,
) -> Result<services::OptimizationData, SimulationError> {
    super::super::spec::ensure_not_aborted(abort)?;
    if base.objective_terms.is_empty() && base.measurements.is_empty() {
        return Err(SimulationError::InvalidConfig(
            "Optimization requires a selected objective measurement".into(),
        ));
    }
    for constraint in &base.constraints {
        constraint
            .validate()
            .map_err(SimulationError::InvalidConfig)?;
        if !base
            .measurements
            .iter()
            .any(|name| name.eq_ignore_ascii_case(&constraint.measurement))
        {
            return Err(SimulationError::InvalidConfig(
                "Constraint is absent from the study measurements".into(),
            ));
        }
    }
    for objective in &base.objective_terms {
        objective
            .validate()
            .map_err(SimulationError::InvalidConfig)?;
        if !base
            .measurements
            .iter()
            .any(|name| name.eq_ignore_ascii_case(&objective.measurement))
        {
            return Err(SimulationError::InvalidConfig(
                "Weighted objective is absent from the study measurements".into(),
            ));
        }
    }
    base.analysis
        .validate()
        .map_err(|errors| SimulationError::InvalidConfig(errors.join("; ")))?;
    let (analysis, environment) = resolved_study_environment(base, environment);
    let source = study_source_at_environment(base, source, environment.as_ref(), abort)?;
    let bridge = EngineBridge::new();
    let circuit = bridge.parse_netlist_with_abort_and_source_path(&source, source_path, abort)?;
    validate_base_measurements(base, &circuit)?;
    for variable in &config.variables {
        if circuit.params.get(&variable.name).is_none() {
            return Err(SimulationError::InvalidConfig(format!(
                "Optimization parameter {:?} is not declared in this circuit",
                variable.name
            )));
        }
    }
    if let Some(point) = &environment
        && (!point.temperature_celsius.is_finite()
            || point.temperature_celsius <= -273.15
            || point.supply_voltage.is_some() != point.nominal_supply_voltage.is_some())
    {
        return Err(SimulationError::InvalidConfig("Optimization Run Set requires a physical temperature and a complete supply/nominal pair".into()));
    }
    let engine = rspice_core::Engine::default().resolved_for_netlist(&circuit);
    let mut limits = engine.config().resource_limits;
    let retained_objectives = base
        .objective_terms
        .len()
        .max(usize::from(!config.objective_unit.trim().is_empty()))
        .saturating_mul(5)
        .saturating_add(base.constraints.len().saturating_mul(6));
    if retained_objectives > limits.max_result_values {
        return super::super::spec::run_abort_aware_service(abort, || {
            Err(services::ServiceRunError::resource_limit(
                rspice_core::ResourceKind::ResultValues,
                retained_objectives,
                limits.max_result_values,
            ))
        });
    }
    limits.max_result_values -= retained_objectives;
    let signal = StudyAbort {
        parent: abort,
        failed: AtomicBool::new(false),
    };
    let fatal = Mutex::new(None);
    let measurement_value = |observation: crate::state::FamilyMeasurementEvidence,
                             requested: &str| {
        // Reject a dimensional mismatch once; a finite candidate whose converted
        // magnitude overflows is still an ordinary failed candidate.
        if !requested.trim().is_empty() {
            observation
                .unit
                .as_ref()
                .unwrap_or(&rspice_core::analysis::MeasurementUnit::Unknown)
                .convert_value(0.0, requested)
                .map_err(|error| {
                    *fatal.lock().unwrap() = Some(SimulationError::InvalidConfig(format!(
                        "Optimization measurement {:?}: {error}",
                        observation.name
                    )));
                    signal.failed.store(true, Ordering::Release);
                    services::ServiceRunError::Aborted
                })?;
        }
        observation
            .value_in_unit(requested)
            .map_err(services::ServiceRunError::Failure)?
            .ok_or_else(|| {
                services::ServiceRunError::Failure(format!(
                    "Optimization measurement {:?} is unavailable or failed",
                    observation.name
                ))
            })
    };
    let evaluate = |variables: &std::collections::HashMap<String, f64>| {
        let candidate = services::materialize_optimization_candidate(
            &engine,
            &circuit,
            &config.variables,
            variables,
            environment.as_ref(),
            &signal,
        )?;
        let result = base
            .run_trial(&engine, &analysis, &candidate, &signal)
            .map_err(|error| match error {
                SimulationError::SolverError(_)
                | SimulationError::ConvergenceFailed { .. }
                | SimulationError::Attributed { .. }
                | SimulationError::CircuitError(_) => {
                    services::ServiceRunError::Failure(error.to_string())
                }
                other => {
                    *fatal.lock().unwrap() = Some(other);
                    signal.failed.store(true, Ordering::Release);
                    services::ServiceRunError::Aborted
                }
            })?;
        let mut constraints = Vec::with_capacity(base.constraints.len());
        for constraint in &base.constraints {
            let observation = result
                .study_measurement(&constraint.measurement)
                .ok_or_else(|| {
                    services::ServiceRunError::Failure(format!(
                        "Constraint measurement {:?} is unavailable or failed",
                        constraint.measurement
                    ))
                })?;
            let value = measurement_value(observation, &constraint.unit)?;
            constraints.push(
                crate::simulation::optimizer::OptimizationConstraintObservation {
                    constraint: constraint.clone(),
                    value,
                    violation: constraint
                        .violation(value)
                        .map_err(services::ServiceRunError::Failure)?,
                },
            );
        }
        if !base.objective_terms.is_empty() {
            let mut total = 0.0;
            let mut observations = Vec::with_capacity(base.objective_terms.len());
            for objective in &base.objective_terms {
                let observation = result
                    .study_measurement(&objective.measurement)
                    .ok_or_else(|| {
                        services::ServiceRunError::Failure(format!(
                            "Optimization measurement {:?} is unavailable or failed",
                            objective.measurement
                        ))
                    })?;
                let value = measurement_value(observation, &objective.unit)?;
                let contribution = objective
                    .contribution(value)
                    .map_err(services::ServiceRunError::Failure)?;
                total += contribution;
                observations.push(
                    crate::simulation::optimizer::OptimizationObjectiveObservation {
                        objective: objective.clone(),
                        value,
                        contribution,
                    },
                );
            }
            if !total.is_finite() {
                return Err(services::ServiceRunError::Failure(
                    "Combined optimization cost is non-finite".into(),
                ));
            }
            return Ok(services::OptimizationEvaluation {
                cost: total,
                objectives: observations,
                constraints,
            });
        }
        let observation = result
            .study_measurement(&base.measurements[0])
            .ok_or_else(|| {
                services::ServiceRunError::Failure(format!(
                    "Optimization measurement {:?} is unavailable or failed",
                    base.measurements[0]
                ))
            })?;
        let value = measurement_value(observation, &config.objective_unit)?;
        let cost = services::optimization_objective_cost(value, config.goal, config.target)?;
        Ok(services::OptimizationEvaluation {
            cost,
            objectives: config.objective_observations(&base.measurements[0], value, cost),
            constraints,
        })
    };
    let target_cost = if base.objective_terms.is_empty() {
        (config.goal == services::OptimizationGoalMode::Target).then_some(0.0)
    } else {
        base.objective_terms
            .iter()
            .all(|term| {
                term.goal == crate::simulation::optimizer::OptimizationObjectiveGoal::Target
            })
            .then_some(0.0)
    };
    let response = services::run_optimization_with_cost_evaluator(
        config,
        limits,
        &signal,
        target_cost,
        evaluate,
    );
    if let Some(error) = fatal.into_inner().unwrap() {
        return Err(error);
    }
    let data = super::super::spec::run_abort_aware_service(abort, || response)?;
    crate::simulation::optimizer::validate_optimization_objectives(
        &data.best_objectives,
        data.best_cost,
    )
    .map_err(SimulationError::InvalidConfig)?;
    crate::simulation::optimizer::validate_optimization_constraint_result(
        &data.best_constraints,
        data.converged,
    )
    .map_err(SimulationError::InvalidConfig)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::config::{AcAnalysisConfig, AcSweepType, TransientAnalysisConfig};
    use crate::simulation::multi_run::{
        AnalysisSpec, OptimizationAlgorithm, OptimizationGoal, OptimizationVariable,
    };
    use crate::simulation::runner::{
        SimulationRequest, SpecExecutionOptions, worker_contract::WorkerSimulationRequest,
    };
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

    fn base(analysis: AnalysisConfig, measurement: &str) -> StudyRunConfig {
        StudyRunConfig {
            postprocess: None,
            constraints: Vec::new(),
            objective_terms: Vec::new(),
            instance_id: AnalysisInstanceId::new(),
            source_revision: ObjectRevision::INITIAL,
            analysis_line: analysis.to_spice(),
            analysis: analysis.into(),
            numeric_options: ".OPTIONS RELTOL=1e-5".into(),
            measurements: vec![measurement.into()],
            histogram_bins: 20,
        }
    }

    #[test]
    fn optimization_units_convert_configured_objectives_and_constraints() {
        use crate::simulation::optimizer::{
            OptimizationConstraint, OptimizationObjectiveGoal, OptimizationObjectiveTerm,
        };
        let deck = "Units\n.param X=0.35\nV1 out 0 {X}\nR1 out 0 1k\n.end\n";
        for weighted in [false, true] {
            let mut selected = base(AnalysisConfig::dc_op(), "scalar:V(out)");
            selected.constraints = vec![OptimizationConstraint {
                measurement: "scalar:V(out)".into(),
                unit: "mV".into(),
                lower: Some(340.0),
                upper: Some(360.0),
                tolerance: 1.0,
                scale: 100.0,
            }];
            let config = services::OptimizationRunConfig {
                objective_unit: if weighted { String::new() } else { "mV".into() },
                target: Some(350.0),
                max_iterations: 4,
                variables: vec![services::OptimizationVariable {
                    name: "X".into(),
                    min: 0.1,
                    max: 0.5,
                    initial: 0.35,
                }],
                ..Default::default()
            };
            if weighted {
                selected.objective_terms.push(OptimizationObjectiveTerm {
                    measurement: "scalar:V(out)".into(),
                    unit: "mV".into(),
                    goal: OptimizationObjectiveGoal::Target,
                    target: Some(350.0),
                    scale: 100.0,
                    weight: 1.0,
                });
            }
            let result = run_optimization(&selected, &config, deck, None, None, &NoAbort).unwrap();
            assert!(result.best_cost < 1e-20, "{result:?}");
            assert_eq!(result.best_objectives[0].value, 350.0);
            assert_eq!(result.best_constraints[0].value, 350.0);
            assert_eq!(result.best_constraints[0].violation, 0.0);
            selected.constraints[0].unit = "ns".into();
            assert!(
                matches!(run_optimization(&selected, &config, deck, None, None, &NoAbort),
                Err(SimulationError::InvalidConfig(message)) if message.contains("incompatible"))
            );
            selected.constraints.clear();
            let mut invalid = config.clone();
            if weighted {
                selected.objective_terms[0].unit = "A".into();
            } else {
                invalid.objective_unit = "A".into();
            }
            assert!(
                matches!(run_optimization(&selected, &invalid, deck, None, None, &NoAbort),
                Err(SimulationError::InvalidConfig(message)) if message.contains("incompatible"))
            );
        }
    }

    #[test]
    fn configured_optimization_worker_finds_ac_and_transient_targets_with_dependent_parameters() {
        let deck = "Configured optimizer\n.param RLOAD=1400 RACTUAL={2*RLOAD}\nV1 in 0 DC 1 AC 1\nR1 in out {RACTUAL}\nC1 out 0 1u\n.ic V(out)=0\n.meas AC gain FIND VM(out) AT=1k\n.meas TRAN settled FIND V(out) AT=2m\n.end\n";
        for (analysis, name, target) in [
            (
                AnalysisConfig::Ac(AcAnalysisConfig {
                    start_freq: 1000.0,
                    stop_freq: 1000.0,
                    num_points: 1,
                    sweep_type: AcSweepType::Linear,
                }),
                "gain",
                1.0 / 1.0_f64.hypot(std::f64::consts::TAU * 2.0),
            ),
            (
                AnalysisConfig::Transient(TransientAnalysisConfig {
                    stop_time: 2e-3,
                    step_time: 2e-5,
                    start_time: 2e-4,
                    max_timestep: Some(2e-5),
                    uic: true,
                }),
                "settled",
                1.8 * (1.0 - (-1.0_f64).exp()),
            ),
        ] {
            let spec = AnalysisSpec::Optimization {
                search:
                    rspice_simulation_contract::optimization_search::OptimizationSearchControls {
                        variable_domains: std::collections::BTreeMap::from([(
                            "RLOAD".into(),
                            crate::simulation::optimizer::OptimizationVariableDomain::Logarithmic,
                        )]),
                        ..Default::default()
                    },
                variables: vec![OptimizationVariable {
                    name: "RLOAD".into(),
                    min: 500.0,
                    max: 1800.0,
                    initial: 1400.0,
                }],
                objective_unit: String::new(),
                objective_expression: None,
                objective_node: "out".into(),
                objective_ref: "0".into(),
                goal: OptimizationGoal::Target,
                target: Some(target),
                algorithm: OptimizationAlgorithm::PatternSearch,
                max_iterations: 48,
                cost_tolerance: 1e-8,
                fd_step: 1e-4,
                initial_step: 0.1,
                min_step: 1e-8,
            };
            let request = SimulationRequest::Spec {
                spec: Box::new(spec),
                options: Box::new(SpecExecutionOptions {
                    study_base: Some(base(analysis, name)),
                    ..Default::default()
                }),
            };
            let encoded =
                serde_json::to_string(&WorkerSimulationRequest::try_from(&request).unwrap())
                    .unwrap();
            let decoded: WorkerSimulationRequest = serde_json::from_str(&encoded).unwrap();
            let SimulationRequest::Spec { spec, options } = SimulationRequest::from(decoded) else {
                unreachable!()
            };
            let result = super::super::super::spec::run_spec_request_with_environment(
                &EngineBridge::new(),
                *spec,
                *options,
                deck,
                None,
                &crate::simulation::execution::ResolvedExecutionDependencies::default(),
                Some(AnalysisExecutionEnvironment {
                    temperature_celsius: 75.0,
                    supply_voltage: Some(1.8),
                    nominal_supply_voltage: Some(1.0),
                    supply_source_names: vec!["V1".into()],
                }),
                &NoAbort,
            )
            .unwrap();
            let crate::simulation::results::SimulationResult::Optimization {
                best_variables,
                best_cost,
                converged,
                ..
            } = result
            else {
                panic!("optimization result")
            };
            assert!(
                converged,
                "{name}: cost={best_cost} vars={best_variables:?}"
            );
            assert!(best_cost <= 1e-8, "{name}: {best_cost}");
            assert!(
                (best_variables["RLOAD"] - 1000.0).abs() < 3.0,
                "{name}: {best_variables:?}"
            );
        }
    }

    #[test]
    fn operating_point_optimization_applies_run_set_to_temperature_expressions_and_supply() {
        let deck = "Temperature objective\n.param RLOAD=1400 RACTUAL={RLOAD*(1+0.01*(TEMP-25))}\nV1 in 0 1\nVAUX aux 0 0.5\nR1 in 0 {RACTUAL}\nR2 aux 0 1k\n.end\n";
        let point = AnalysisExecutionEnvironment {
            temperature_celsius: 75.0,
            supply_voltage: Some(1.8),
            nominal_supply_voltage: Some(1.0),
            supply_source_names: vec!["V1".into()],
        };
        for (environment, expected) in [(None, 1000.0 / 1.02), (Some(point), 1200.0)] {
            let result = super::super::super::spec::run_spec_request_with_environment(
                &EngineBridge::new(),
                AnalysisSpec::Optimization {
                    search: Default::default(),
                    variables: vec![OptimizationVariable {
                        name: "RLOAD".into(),
                        min: 500.0,
                        max: 1800.0,
                        initial: 1400.0,
                    }],
                    // Also observe the unrelated supply: it must remain unscaled.
                    objective_unit: String::new(),
                    objective_expression: Some("I(V1)+I(VAUX)".into()),
                    objective_node: String::new(),
                    objective_ref: String::new(),
                    goal: OptimizationGoal::Target,
                    target: Some(-0.0015),
                    algorithm: OptimizationAlgorithm::PatternSearch,
                    max_iterations: 48,
                    cost_tolerance: 1e-16,
                    fd_step: 1e-4,
                    initial_step: 0.1,
                    min_step: 1e-8,
                },
                SpecExecutionOptions::default(),
                deck,
                None,
                &crate::simulation::execution::ResolvedExecutionDependencies::default(),
                environment,
                &NoAbort,
            )
            .unwrap();
            let crate::simulation::results::SimulationResult::Optimization {
                best_variables,
                best_cost,
                converged,
                ..
            } = result
            else {
                panic!("optimization result")
            };
            assert!(converged, "cost={best_cost} vars={best_variables:?}");
            assert!(
                (best_variables["RLOAD"] - expected).abs() < 0.1,
                "{best_variables:?}"
            );
        }
    }

    #[test]
    fn weighted_optimization_executes_scaled_goals_and_retains_best_components() {
        use crate::simulation::optimizer::{
            OptimizationObjectiveGoal as Goal, OptimizationObjectiveTerm as Term,
        };
        use crate::simulation::runner::worker_contract::WorkerSimulationResult;
        let deck = "Weighted targets\n.param X=0.8\nV1 a 0 {X}\nV2 b 0 {2*X}\nR1 a 0 1k\nR2 b 0 1k\n.end\n";
        for (weight, scale, expected, algorithm) in [
            (3.0, 2.0, 0.25, OptimizationAlgorithm::PatternSearch),
            (1.0, 2.0, 0.5, OptimizationAlgorithm::PatternSearch),
            (3.0, 4.0, 4.0 / 7.0, OptimizationAlgorithm::PatternSearch),
            (3.0, 2.0, 0.25, OptimizationAlgorithm::GradientDescent),
        ] {
            let mut selected = base(AnalysisConfig::dc_op(), "scalar:V(a)");
            selected.measurements.push("scalar:V(b)".into());
            selected.objective_terms = vec![
                Term {
                    measurement: "scalar:V(a)".into(),
                    unit: String::new(),
                    goal: Goal::Target,
                    target: Some(1.0),
                    scale: 1.0,
                    weight: 1.0,
                },
                Term {
                    measurement: "scalar:V(b)".into(),
                    unit: String::new(),
                    goal: Goal::Target,
                    target: Some(0.0),
                    scale,
                    weight,
                },
            ];
            let request = SimulationRequest::Spec {
                spec: Box::new(AnalysisSpec::Optimization {
                    search: Default::default(),
                    variables: vec![OptimizationVariable {
                        name: "X".into(),
                        min: 0.0,
                        max: 1.0,
                        initial: 0.8,
                    }],
                    objective_unit: String::new(),
                    objective_expression: None,
                    objective_node: "a".into(),
                    objective_ref: "0".into(),
                    goal: OptimizationGoal::Target,
                    target: Some(123.0), // inactive single objective
                    algorithm,
                    max_iterations: 60,
                    cost_tolerance: 1e-12,
                    fd_step: 1e-4,
                    initial_step: 0.1,
                    min_step: 1e-9,
                }),
                options: Box::new(SpecExecutionOptions {
                    study_base: Some(selected),
                    ..Default::default()
                }),
            };
            let wire = WorkerSimulationRequest::try_from(&request).unwrap();
            let restored: WorkerSimulationRequest =
                serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
            assert_eq!(wire, restored);
            let SimulationRequest::Spec { spec, options } = SimulationRequest::from(restored)
            else {
                unreachable!()
            };
            let result = super::super::super::spec::run_spec_request(
                &EngineBridge::new(),
                *spec,
                *options,
                deck,
                None,
                &crate::simulation::execution::ResolvedExecutionDependencies::default(),
                &NoAbort,
            )
            .unwrap();
            let wire = WorkerSimulationResult::try_from(result).unwrap();
            let restored: WorkerSimulationResult =
                serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
            assert_eq!(wire, restored);
            let crate::simulation::results::SimulationResult::Optimization {
                best_variables,
                best_cost,
                best_objectives,
                best_constraints,
                converged,
                ..
            } = crate::simulation::results::SimulationResult::from(restored)
            else {
                panic!("optimization result")
            };
            assert!(converged);
            assert!(
                (best_variables["X"] - expected).abs() < 1e-5,
                "{weight}/{scale}: {best_variables:?}"
            );
            assert!(best_constraints.is_empty());
            assert_eq!(best_objectives.len(), 2);
            assert_eq!(best_objectives[0].value, best_variables["X"]);
            assert_eq!(best_objectives[1].value, 2.0 * best_variables["X"]);
            let expected_cost =
                (expected - 1.0).powi(2) + weight * (2.0 * expected / scale).powi(2);
            assert!(
                (best_cost - expected_cost).abs() < 1e-10,
                "{best_cost} vs {expected_cost}"
            );
            crate::simulation::optimizer::validate_optimization_objectives(
                &best_objectives,
                best_cost,
            )
            .unwrap();
            let mut corrupted = best_objectives;
            corrupted[0].contribution += 1.0;
            assert!(
                crate::simulation::optimizer::validate_optimization_objectives(
                    &corrupted, best_cost
                )
                .is_err()
            );
        }
        let mut term = Term {
            measurement: "gain".into(),
            unit: String::new(),
            goal: Goal::Minimize,
            target: None,
            scale: 2.0,
            weight: 3.0,
        };
        assert_eq!(term.contribution(-4.0).unwrap(), -6.0);
        term.goal = Goal::Maximize;
        assert_eq!(term.contribution(-4.0).unwrap(), 6.0);
        term.scale = 0.0;
        assert!(term.validate().is_err());
        term.scale = 1.0;
        term.weight = f64::MAX;
        assert!(term.contribution(10.0).is_err());
    }

    #[test]
    fn configured_optimization_rejects_missing_targets_and_preserves_cancellation() {
        let deck = "Optimizer\n.param RLOAD=1k\nV1 out 0 1\nR1 out 0 {RLOAD}\n.end\n";
        let selected = base(AnalysisConfig::dc_op(), "scalar:V(out)");
        assert!(matches!(
            run_optimization(
                &selected,
                &Default::default(),
                deck,
                None,
                None,
                &ImmediateAbort
            ),
            Err(SimulationError::Aborted)
        ));
        let mut missing = selected.clone();
        missing.measurements = vec!["missing".into()];
        assert!(matches!(
            run_optimization(&missing, &Default::default(), deck, None, None, &NoAbort),
            Err(SimulationError::InvalidConfig(_))
        ));
        let config = services::OptimizationRunConfig {
            variables: vec![services::OptimizationVariable {
                name: "UNDECLARED".into(),
                min: 1.0,
                max: 2.0,
                initial: 1.5,
            }],
            ..Default::default()
        };
        assert!(
            matches!(run_optimization(&selected, &config, deck, None, None, &NoAbort), Err(SimulationError::InvalidConfig(ref message)) if message.contains("UNDECLARED"))
        );
    }
    #[test]
    fn constrained_optimization_recovers_feasibility_and_preserves_physical_objectives() {
        use crate::simulation::optimizer::{
            OptimizationConstraint as Constraint, OptimizationObjectiveGoal as Goal,
            OptimizationObjectiveTerm as Term, OptimizationVariableDomain as Domain,
        };
        use crate::simulation::runner::worker_contract::WorkerSimulationResult;
        let deck = "Constrained design\n.param X=0.8\nV1 a 0 {X}\nV2 b 0 {2*X}\nR1 a 0 1k\nR2 b 0 1k\n.end\n";
        for (algorithm, lower, upper, tolerance, expected, feasible, weighted) in [
            (
                OptimizationAlgorithm::PatternSearch,
                None,
                Some(0.8),
                0.0,
                0.4,
                true,
                true,
            ),
            (
                OptimizationAlgorithm::GradientDescent,
                None,
                Some(0.8),
                0.0,
                0.4,
                true,
                true,
            ),
            (
                OptimizationAlgorithm::SimulatedAnnealing,
                None,
                Some(0.8),
                0.0,
                0.4,
                true,
                true,
            ),
            (
                OptimizationAlgorithm::PatternSearch,
                Some(0.8),
                Some(0.8),
                0.02,
                0.41,
                true,
                false,
            ),
            (
                OptimizationAlgorithm::PatternSearch,
                Some(3.0),
                None,
                0.0,
                1.0,
                false,
                false,
            ),
        ] {
            let mut selected = base(AnalysisConfig::dc_op(), "scalar:V(a)");
            selected.measurements.push("scalar:V(b)".into());
            selected.constraints.push(Constraint {
                measurement: "scalar:V(b)".into(),
                unit: String::new(),
                lower,
                upper,
                tolerance,
                scale: 2.0,
            });
            if weighted {
                selected.objective_terms.push(Term {
                    measurement: "scalar:V(a)".into(),
                    unit: String::new(),
                    goal: Goal::Maximize,
                    target: None,
                    scale: 1.0,
                    weight: 1e30,
                });
            }
            let mut search = rspice_simulation_contract::optimization_search::OptimizationSearchControls::default();
            if algorithm == OptimizationAlgorithm::SimulatedAnnealing {
                search
                    .variable_domains
                    .insert("X".into(), Domain::Quantized { step: 0.1 });
            }
            let request = SimulationRequest::Spec {
                spec: Box::new(AnalysisSpec::Optimization {
                    search,
                    variables: vec![OptimizationVariable {
                        name: "X".into(),
                        min: 0.0,
                        max: 1.0,
                        initial: 0.8,
                    }],
                    objective_unit: String::new(),
                    objective_expression: None,
                    objective_node: "a".into(),
                    objective_ref: "0".into(),
                    goal: OptimizationGoal::Maximize,
                    target: None,
                    algorithm,
                    max_iterations: 100,
                    cost_tolerance: 1e-12,
                    fd_step: 1e-4,
                    initial_step: 0.1,
                    min_step: 1e-10,
                }),
                options: Box::new(SpecExecutionOptions {
                    study_base: Some(selected),
                    ..Default::default()
                }),
            };
            let wire = WorkerSimulationRequest::try_from(&request).unwrap();
            let wire: WorkerSimulationRequest =
                serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
            let SimulationRequest::Spec { spec, options } = SimulationRequest::from(wire) else {
                unreachable!()
            };
            let result = super::super::super::spec::run_spec_request(
                &EngineBridge::new(),
                *spec,
                *options,
                deck,
                None,
                &crate::simulation::execution::ResolvedExecutionDependencies::default(),
                &NoAbort,
            )
            .unwrap();
            let wire = WorkerSimulationResult::try_from(result).unwrap();
            let restored: WorkerSimulationResult =
                serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
            assert_eq!(wire, restored);
            let crate::simulation::results::SimulationResult::Optimization {
                best_variables,
                best_cost,
                best_constraints,
                converged,
                ..
            } = crate::simulation::results::SimulationResult::from(restored)
            else {
                unreachable!()
            };
            assert!(
                (best_variables["X"] - expected).abs() < 1e-5,
                "{algorithm:?}, {lower:?}/{upper:?}: {best_variables:?}"
            );
            assert_eq!(
                converged, feasible,
                "{algorithm:?}: {best_variables:?}, {best_constraints:?}"
            );
            assert_eq!(best_constraints.len(), 1);
            assert!((best_constraints[0].value - 2.0 * best_variables["X"]).abs() < 1e-12);
            assert_eq!(best_constraints[0].violation == 0.0, feasible);
            let expected_cost = -best_variables["X"] * if weighted { 1e30 } else { 1.0 };
            assert!((best_cost - expected_cost).abs() < 1e-12 * expected_cost.abs().max(1.0));
            crate::simulation::optimizer::validate_optimization_constraint_result(
                &best_constraints,
                converged,
            )
            .unwrap();
        }
        // A feasibility gradient at the old point cannot prove convergence
        // after a coordinate fallback moves into the feasible region.
        use crate::simulation::optimizer::{
            DesignVar, OptimizationScore, OptimizerAlgo, OptimizerConfig, OptimizerEngine,
        };
        let mut optimizer = OptimizerEngine::with_config(OptimizerConfig {
            algorithm: OptimizerAlgo::GradientDescent,
            ..Default::default()
        });
        optimizer.add_var(DesignVar::new("X", 0.5, 0.0, 1.0));
        let mut score = |vars: &std::collections::HashMap<String, f64>| OptimizationScore {
            cost: (vars["X"] - 0.9).powi(2),
            violation: if vars["X"] < 0.6 { 1.0 } else { 0.0 },
        };
        let initial = optimizer.current_vars();
        optimizer.observe_candidate(&initial, score(&initial));
        optimizer.step(&mut score);
        assert_eq!(optimizer.best_score().violation, 0.0);
        assert!(!optimizer.is_converged(None));
        let tiny = Constraint {
            measurement: "tiny".into(),
            unit: String::new(),
            lower: Some(1e-300),
            upper: None,
            tolerance: 0.0,
            scale: 1e300,
        };
        assert!(tiny.violation(0.0).unwrap() > 0.0);
        for term in [
            Constraint {
                measurement: "x".into(),
                unit: String::new(),
                lower: None,
                upper: None,
                tolerance: 0.0,
                scale: 1.0,
            },
            Constraint {
                measurement: "x".into(),
                unit: String::new(),
                lower: Some(2.0),
                upper: Some(1.0),
                tolerance: 0.0,
                scale: 1.0,
            },
            Constraint {
                measurement: "x".into(),
                unit: String::new(),
                lower: Some(0.0),
                upper: None,
                tolerance: -1.0,
                scale: 1.0,
            },
            Constraint {
                measurement: "x".into(),
                unit: String::new(),
                lower: Some(0.0),
                upper: None,
                tolerance: 0.0,
                scale: 0.0,
            },
        ] {
            assert!(term.validate().is_err());
        }
    }
}
