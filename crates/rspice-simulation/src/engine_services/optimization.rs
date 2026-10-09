//! Circuit evaluation adapter for the bounded runtime optimizer.

mod objective;
use super::{ServiceContext, is_ground_like};
#[cfg(test)]
use super::{build_engine_config, parse_runner_netlist_with_abort};
use crate::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically};
use crate::optimization::run_optimization_with_evaluator;
pub use crate::optimization::{
    OptimizationAlgorithmMode, OptimizationData, OptimizationEvaluation, OptimizationGoalMode,
    OptimizationRunConfig, OptimizationVariable, objective_to_cost,
    run_optimization_with_cost_evaluator,
};
use rspice_core::{Value, abort_signal::AbortSignal, engine::Engine};
use std::collections::HashMap;
#[cfg(test)]
use std::path::Path;

/// Run optimization analysis with default configuration and no source path.
///
/// Test-only. The shipping path is
/// [`run_optimization_analysis_with_context`], which
/// the device spec calls with the configuration the user set.
#[cfg(test)]
pub fn run_optimization_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<OptimizationData> {
    run_optimization_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        &OptimizationRunConfig::default(),
        None,
        abort,
    )
}

/// Run explicitly configured optimization analysis with source-path
/// resolution and cooperative cancellation through every objective trial and
/// outer iteration.
#[cfg(test)]
pub fn run_optimization_analysis_with_config_and_source_path_and_abort(
    netlist_text: &str,
    config: &OptimizationRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<OptimizationData> {
    run_optimization_analysis_with_environment_and_source_path_and_abort(
        netlist_text,
        config,
        source_path,
        None,
        abort,
    )
}

/// Apply the Studio Run Set to every operating-point objective candidate.
#[cfg(test)]
pub(crate) fn run_optimization_analysis_with_environment_and_source_path_and_abort(
    netlist_text: &str,
    config: &OptimizationRunConfig,
    source_path: Option<&Path>,
    environment: Option<&rspice_core::engine::MonteCarloEnvironment>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<OptimizationData> {
    run_optimization_analysis_with_context(
        netlist_text,
        config,
        environment,
        ServiceContext::with_defaults(source_path, abort),
    )
}

/// Apply one captured execution policy to materialization and every candidate.
pub(crate) fn run_optimization_analysis_with_context(
    netlist_text: &str,
    config: &OptimizationRunConfig,
    environment: Option<&rspice_core::engine::MonteCarloEnvironment>,
    context: ServiceContext<'_>,
) -> ServiceRunResult<OptimizationData> {
    let abort = context.abort;
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;
    let objective_unit = config.operating_point_objective_unit();
    if !config.objective_unit.trim().is_empty() {
        objective_unit
            .convert_value(0.0, &config.objective_unit)
            .map_err(|error| {
                ServiceRunError::Failure(format!("Optimization objective unit: {error}"))
            })?;
    }
    if let Some(point) = environment
        && (!point.temperature_celsius.is_finite()
            || point.temperature_celsius <= -273.15
            || point.supply_voltage.is_some() != point.nominal_supply_voltage.is_some())
    {
        return Err(ServiceRunError::Failure(
                "Optimization Run Set requires a physical temperature and a complete supply/nominal pair".into(),
            ));
    }
    let netlist = context.parse(netlist_text)?;
    for variable in &config.variables {
        if netlist.params.get(&variable.name).is_none() {
            return Err(ServiceRunError::Failure(format!(
                "Optimization parameter {:?} is not declared in this circuit",
                variable.name,
            )));
        }
    }
    let engine = Engine::new(context.engine_config(&netlist));
    run_optimization_with_evaluator(config, engine.config().resource_limits, abort, |vars| {
        let candidate = materialize_optimization_candidate(
            &engine,
            &netlist,
            &config.variables,
            vars,
            environment,
            abort,
        )?;
        let value = evaluate_optimization_objective(&candidate, config, context)?;
        if config.objective_unit.trim().is_empty() {
            Ok(value)
        } else {
            objective_unit
                .convert_value(value, &config.objective_unit)
                .map_err(ServiceRunError::Failure)
        }
    })
}

fn evaluate_optimization_objective(
    netlist: &rspice_core::Netlist,
    config: &OptimizationRunConfig,
    context: ServiceContext<'_>,
) -> ServiceRunResult<Value> {
    let abort = context.abort;
    ensure_not_aborted(abort)?;
    let engine = Engine::new(context.engine_config(netlist));
    let dc = engine
        .run_dc_op_with_abort(netlist, abort)
        .map_err(|error| {
            ServiceRunError::from_core("DC operating point failed during optimization", error)
        })?;

    if let Some(expression) = &config.objective_expression {
        return objective::evaluate_with_context(expression, netlist, &dc, context);
    }
    let node_idx =
        resolve_node_index_case_insensitive(&dc.node_names, &config.objective_node, abort)?
            .ok_or_else(|| {
                ServiceRunError::Failure(format!(
                    "Optimization objective node '{}' not found",
                    config.objective_node
                ))
            })?;
    let ref_idx = if is_ground_like(&config.objective_ref) {
        Some(0usize)
    } else {
        resolve_node_index_case_insensitive(&dc.node_names, &config.objective_ref, abort)?
    }
    .ok_or_else(|| {
        ServiceRunError::Failure(format!(
            "Optimization objective reference node '{}' not found",
            config.objective_ref
        ))
    })?;

    // `try_voltage` answers `None` both for an index the result does not have
    // and for a net only the event domain resolves; the second is the one an
    // objective can be written against by mistake, so it says so.
    let event_only = |node: usize, role: &str| -> ServiceRunError {
        match dc.event_only_node_kind(node) {
            Some(kind) => ServiceRunError::Failure(
                rspice_core::analysis::transient::event_only_voltage_refusal(
                    dc.node_names.get(node).map_or(role, |name| name.as_str()),
                    kind,
                    rspice_core::analysis::transient::EventTraceSurface::SolvedPoint,
                ),
            ),
            None => {
                ServiceRunError::Failure(format!("Optimization {role} voltage index out of bounds"))
            }
        }
    };
    let node_v = dc
        .try_voltage(node_idx)
        .ok_or_else(|| event_only(node_idx, "node"))?;
    let ref_v = dc
        .try_voltage(ref_idx)
        .ok_or_else(|| event_only(ref_idx, "reference"))?;
    ensure_not_aborted(abort)?;
    let objective = node_v - ref_v;
    if !objective.is_finite() {
        return Err(ServiceRunError::Failure(format!(
            "Optimization objective V({},{}) is non-finite",
            config.objective_node, config.objective_ref
        )));
    }
    Ok(objective)
}

/// Materialize one bounded coordinate, with all variable and temperature
/// overrides applied before dependent parameters and models are evaluated.
pub(crate) fn materialize_optimization_candidate(
    engine: &Engine,
    circuit: &rspice_core::Netlist,
    configured: &[OptimizationVariable],
    variables: &HashMap<String, Value>,
    environment: Option<&rspice_core::engine::MonteCarloEnvironment>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<rspice_core::Netlist> {
    use rspice_core::netlist::{StepCommand, StepSweep, StepTarget};
    ensure_not_aborted(abort)?;
    let mut steps = configured
        .iter()
        .map(|variable| StepCommand {
            target: StepTarget::Param,
            name: variable.name.clone(),
            param_name: None,
            sweep: StepSweep::List(vec![variables[&variable.name]]),
        })
        .collect::<Vec<_>>();
    if let Some(point) = environment {
        steps.push(StepCommand {
            target: StepTarget::Temp,
            name: "TEMP".into(),
            param_name: None,
            sweep: StepSweep::List(vec![point.temperature_celsius]),
        });
    }
    let plan = engine
        .plan_step_commands_with_abort(
            circuit,
            &steps,
            rspice_core::engine::StepPlanLimits::from_resource_limits(
                engine.config().resource_limits,
            ),
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("Optimization candidate planning", error))?;
    let mut candidate = engine
        .materialize_step_run_with_abort(&plan, 0, abort)
        .map_err(|error| {
            ServiceRunError::from_core("Optimization candidate materialization", error)
        })?
        .into_parts()
        .1;
    if let Some(point) = environment
        && let (Some(supply), Some(nominal)) = (point.supply_voltage, point.nominal_supply_voltage)
    {
        crate::netlist_preparation::apply_voltage_corner(
            &mut candidate,
            supply,
            nominal,
            &point.supply_source_names,
            abort,
        )?;
    }
    Ok(candidate)
}

fn resolve_node_index_case_insensitive(
    node_names: &[String],
    target: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<usize>> {
    ensure_not_aborted(abort)?;
    let trimmed = target.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    for (index, name) in node_names.iter().enumerate() {
        poll_periodically(abort, index)?;
        if name.eq_ignore_ascii_case(trimmed) {
            return Ok(Some(index));
        }
    }
    ensure_not_aborted(abort)?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortOnPoll {
        fn new(abort_on: usize) -> Self {
            Self {
                abort_on,
                polls: AtomicUsize::new(0),
            }
        }
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    const OPTIMIZATION_DECK: &str = "\
Optimization cancellation
.param RLOAD=1k
V1 in 0 1
R1 in out {RLOAD}
R2 out 0 1k
.op
.end
";

    #[test]
    fn optimization_seed_temperature_cooling_and_gradient_threshold_govern_the_search() {
        let run = |config: &OptimizationRunConfig| {
            run_optimization_analysis_with_config_and_source_path_and_abort(
                OPTIMIZATION_DECK,
                config,
                None,
                &rspice_core::NoAbort,
            )
            .unwrap()
        };
        let mut config = OptimizationRunConfig {
            algorithm: OptimizationAlgorithmMode::SimulatedAnnealing,
            max_iterations: 25,
            cost_tolerance: 1e-30,
            ..Default::default()
        };
        config.search.random_seed = 42;
        let first = run(&config);
        assert_eq!(first.costs, run(&config).costs);
        config.search.random_seed = 43;
        assert_ne!(first.variable_traces, run(&config).variable_traces);
        config.search.random_seed = 42;
        config.search.sa_initial_temp = 1e-12;
        assert_ne!(first.costs, run(&config).costs);
        config.search.sa_initial_temp = 100.0;
        config.search.sa_cooling_rate = 0.01;
        assert_ne!(first.costs, run(&config).costs);
        config.algorithm = OptimizationAlgorithmMode::GradientDescent;
        config.search.var_tolerance = 1e9;
        let loose = run(&config);
        config.search.var_tolerance = 1e-14;
        let strict = run(&config);
        assert!(strict.iterations.len() > loose.iterations.len());
    }

    #[test]
    fn optimization_honors_early_abort_before_invalid_input() {
        let abort = AbortOnPoll::new(1);
        let result = run_optimization_analysis_with_abort("invalid", &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn optimization_honors_abort_during_objective_trials() {
        let abort = AbortOnPoll::new(12);
        let result = run_optimization_analysis_with_abort(OPTIMIZATION_DECK, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.polls.load(Ordering::Relaxed) >= 12);
    }
}

#[cfg(test)]
mod variable_domain_tests {
    use super::*;
    use rspice_core::NoAbort;
    use rspice_simulation_contract::optimization_search::OptimizationVariableDomain as Domain;

    #[test]
    fn optimization_variable_domains_drive_real_circuits_and_report_physical_values() {
        let deck = "Variable domains\n.param X=1\nV1 out 0 {X}\nR1 out 0 1k\n.end\n";
        for (domain, min, max, initial, target, scale, expected) in [
            (Domain::Logarithmic, 1e-12, 1e6, 1.0, 1.0, 1e6, 1e-6),
            (
                Domain::Quantized { step: 2.0 },
                0.0,
                10.0,
                0.0,
                6.4,
                1.0,
                6.0,
            ),
            (
                Domain::Discrete {
                    values: vec![100.0, 470.0, 2200.0],
                },
                100.0,
                2200.0,
                100.0,
                500.0,
                1.0,
                470.0,
            ),
        ] {
            let mut config = OptimizationRunConfig {
                variables: vec![OptimizationVariable {
                    name: "X".into(),
                    min,
                    max,
                    initial,
                }],
                objective_unit: String::new(),
                objective_expression: Some(format!("{scale}*V(out)")),
                target: Some(target),
                cost_tolerance: 1e-10,
                max_iterations: 90,
                ..Default::default()
            };
            config
                .search
                .variable_domains
                .insert("x".into(), domain.clone());
            let data = run_optimization_analysis_with_config_and_source_path_and_abort(
                deck, &config, None, &NoAbort,
            )
            .unwrap();
            assert!(data.converged, "{domain:?}: {data:?}");
            assert!(
                (data.best_variables["X"] - expected).abs() <= expected.abs().max(1e-12) * 1e-5,
                "{domain:?}: {:?}",
                data.best_variables
            );
            for value in &data.variable_traces["X"] {
                assert!(*value >= min && *value <= max);
                match &domain {
                    Domain::Quantized { step } => {
                        assert_eq!((value - min) / step, ((value - min) / step).round())
                    }
                    Domain::Discrete { values } => assert!(values.contains(value)),
                    _ => {}
                }
            }
            let expected_cost = (scale * data.best_variables["X"] - target).powi(2);
            assert!((data.best_cost - expected_cost).abs() < 1e-8 * expected_cost.abs().max(1.0));
            config.algorithm = OptimizationAlgorithmMode::GradientDescent;
            if domain.is_discrete() {
                assert!(config.validate().unwrap_err().contains("pattern search"));
            }
        }
        let mut config = OptimizationRunConfig::default();
        config
            .search
            .variable_domains
            .insert("missing".into(), Domain::Logarithmic);
        assert!(
            config
                .validate()
                .unwrap_err()
                .contains("no configured variable")
        );
    }
}
