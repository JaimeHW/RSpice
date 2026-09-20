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
    if base.objective_terms.is_empty() && base.measurements.len() != 1 {
        return Err(SimulationError::InvalidConfig(
            "Optimization requires exactly one selected objective measurement".into(),
        ));
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
    let source = services::splice_before_terminal_end_card(
        source,
        &format!("{}\n{}", base.analysis_line, base.numeric_options),
    );
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
    let environment = environment.map(|point| MonteCarloEnvironment {
        temperature_celsius: point.temperature_celsius,
        supply_voltage: point.supply_voltage,
        nominal_supply_voltage: point.nominal_supply_voltage,
        supply_source_names: point.supply_source_names,
    });
    if let Some(point) = &environment {
        if !point.temperature_celsius.is_finite()
            || point.temperature_celsius <= -273.15
            || point.supply_voltage.is_some() != point.nominal_supply_voltage.is_some()
        {
            return Err(SimulationError::InvalidConfig("Optimization Run Set requires a physical temperature and a complete supply/nominal pair".into()));
        }
    }
    let analysis = analysis_for_environment(base, environment.as_ref());
    let engine = rspice_core::Engine::default().resolved_for_netlist(&circuit);
    let mut limits = engine.config().resource_limits;
    let retained_objectives = base.objective_terms.len().saturating_mul(5);
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
    let evaluate = |variables: &std::collections::HashMap<String, f64>| {
        let candidate = services::materialize_optimization_candidate(
            &engine,
            &circuit,
            &config.variables,
            variables,
            environment.as_ref(),
            &signal,
        )?;
        let result =
            EngineBridge::run_materialized_with_abort(&engine, &analysis, &candidate, &signal)
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
                let value = observation.value.expect("observed objective value");
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
            return Ok((total, observations));
        }
        result
            .study_measurement(&base.measurements[0])
            .and_then(|observation| observation.value)
            .map(|value| (value, Vec::new()))
            .ok_or_else(|| {
                services::ServiceRunError::Failure(format!(
                    "Optimization measurement {:?} is unavailable or failed",
                    base.measurements[0]
                ))
            })
    };
    let response = if base.objective_terms.is_empty() {
        services::run_optimization_with_evaluator(config, limits, &signal, |variables| {
            evaluate(variables).map(|(value, _)| value)
        })
    } else {
        let target_cost = base
            .objective_terms
            .iter()
            .all(|term| {
                term.goal == crate::simulation::optimizer::OptimizationObjectiveGoal::Target
            })
            .then_some(0.0);
        services::run_optimization_with_cost_evaluator(
            config,
            limits,
            &signal,
            target_cost,
            |variables| {
                evaluate(variables)
                    .map(|(cost, objectives)| services::OptimizationEvaluation { cost, objectives })
            },
        )
    };
    if let Some(error) = fatal.into_inner().unwrap() {
        return Err(error);
    }
    let data = super::super::spec::run_abort_aware_service(abort, || response)?;
    crate::simulation::optimizer::validate_optimization_objectives(
        &data.best_objectives,
        data.best_cost,
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
            objective_terms: Vec::new(),
            instance_id: AnalysisInstanceId::new(),
            source_revision: ObjectRevision::INITIAL,
            analysis_line: analysis.to_spice(),
            analysis,
            numeric_options: ".OPTIONS RELTOL=1e-5".into(),
            measurements: vec![measurement.into()],
            histogram_bins: 20,
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
                    ..Default::default()
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
                search: Default::default(),
                variables: vec![OptimizationVariable {
                    name: "RLOAD".into(),
                    min: 500.0,
                    max: 1800.0,
                    initial: 1400.0,
                }],
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
                    goal: Goal::Target,
                    target: Some(1.0),
                    scale: 1.0,
                    weight: 1.0,
                },
                Term {
                    measurement: "scalar:V(b)".into(),
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
}
