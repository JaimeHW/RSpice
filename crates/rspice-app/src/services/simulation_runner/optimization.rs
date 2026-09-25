//! Optimization runs.
//!
//! Drives repeated analyses while varying design parameters toward a goal:
//! the variables and their bounds, the goal formulation, and the algorithm
//! that searches.

mod objective;
pub use objective::validate_optimization_expression;

use super::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically};
use super::{build_engine_config, is_ground_like, parse_runner_netlist_with_abort};
use crate::simulation::optimizer::{DesignVar, OptimizerEngine};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::Engine;
use rspice_results::optimization::OptimizationScore;
use rspice_simulation_contract::optimization_search::{OptimizerAlgo, OptimizerConfig};
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Optimization objective strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationGoalMode {
    /// Minimize objective value.
    Minimize,
    /// Maximize objective value.
    Maximize,
    /// Reach target value.
    Target,
}

/// Optimization algorithm mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationAlgorithmMode {
    /// Gradient-descent algorithm.
    GradientDescent,
    /// Pattern-search algorithm.
    PatternSearch,
    /// Simulated-annealing algorithm.
    SimulatedAnnealing,
}

/// Single optimization variable.
#[derive(Debug, Clone)]
pub struct OptimizationVariable {
    /// Parameter name (`.param <name>=...`).
    pub name: String,
    /// Lower bound.
    pub min: Value,
    /// Upper bound.
    pub max: Value,
    /// Initial value.
    pub initial: Value,
}

use rspice_simulation_contract::optimization_search::OptimizationSearchControls;

/// Optimization run configuration.
#[derive(Debug, Clone)]
pub struct OptimizationRunConfig {
    pub search: OptimizationSearchControls,
    /// Optimization variables.
    pub variables: Vec<OptimizationVariable>,
    /// Requested physical unit; blank keeps the producer value.
    pub objective_unit: String,
    /// Optional scalar operating-point expression; overrides the voltage objective.
    pub objective_expression: Option<String>,
    /// Objective node (V(node,ref)) when no expression is configured.
    pub objective_node: String,
    /// Objective reference node.
    pub objective_ref: String,
    /// Goal mode.
    pub goal: OptimizationGoalMode,
    /// Optional goal target value.
    pub target: Option<Value>,
    /// Algorithm selection.
    pub algorithm: OptimizationAlgorithmMode,
    /// Maximum iterations.
    pub max_iterations: usize,
    /// Cost tolerance.
    pub cost_tolerance: Value,
    /// Finite difference relative step.
    pub fd_step: Value,
    /// Initial step size.
    pub initial_step: Value,
    /// Minimum step size.
    pub min_step: Value,
}

impl Default for OptimizationRunConfig {
    fn default() -> Self {
        Self {
            search: OptimizationSearchControls::default(),
            variables: vec![OptimizationVariable {
                name: "RLOAD".to_string(),
                min: 500.0,
                max: 5000.0,
                initial: 1000.0,
            }],
            objective_unit: String::new(),
            objective_expression: None,
            objective_node: "out".to_string(),
            objective_ref: "0".to_string(),
            goal: OptimizationGoalMode::Target,
            target: Some(1.2),
            algorithm: OptimizationAlgorithmMode::PatternSearch,
            max_iterations: 120,
            cost_tolerance: 1e-8,
            fd_step: 1e-4,
            initial_step: 0.1,
            min_step: 1e-8,
        }
    }
}

impl OptimizationRunConfig {
    fn operating_point_objective_unit(&self) -> rspice_core::analysis::MeasurementUnit {
        self.objective_expression.as_ref().map_or_else(
            || rspice_core::analysis::MeasurementUnit::Known("V".into()),
            |expression| rspice_core::analysis::expression_unit(expression, &HashMap::new()),
        )
    }

    pub(crate) fn objective_observations(
        &self,
        name: &str,
        value: f64,
        cost: f64,
    ) -> Vec<crate::simulation::optimizer::OptimizationObjectiveObservation> {
        if self.objective_unit.trim().is_empty() {
            return Vec::new();
        }
        use crate::simulation::optimizer::{
            OptimizationObjectiveGoal as Goal, OptimizationObjectiveObservation,
            OptimizationObjectiveTerm,
        };
        vec![OptimizationObjectiveObservation {
            objective: OptimizationObjectiveTerm {
                measurement: name.into(),
                unit: self.objective_unit.clone(),
                goal: match self.goal {
                    OptimizationGoalMode::Minimize => Goal::Minimize,
                    OptimizationGoalMode::Maximize => Goal::Maximize,
                    OptimizationGoalMode::Target => Goal::Target,
                },
                target: self.target,
                scale: 1.0,
                weight: 1.0,
            },
            value,
            contribution: cost,
        }]
    }

    pub(super) fn validate(&self) -> Result<(), String> {
        self.search.validate()?;
        rspice_results::optimization::validate_requested_unit(&self.objective_unit)?;
        if self.variables.is_empty() {
            return Err("Optimization requires at least one variable".to_string());
        }
        if let Some(expression) = &self.objective_expression {
            crate::services::simulation_runner::validate_optimization_expression(expression)?;
        } else {
            if self.objective_node.trim().is_empty() {
                return Err("Optimization objective_node must not be empty".to_string());
            }
            if self.objective_ref.trim().is_empty() {
                return Err("Optimization objective_ref must not be empty".to_string());
            }
            if self
                .objective_node
                .eq_ignore_ascii_case(&self.objective_ref)
            {
                return Err("Optimization objective_node and objective_ref must differ".to_string());
            }
        }
        if self.max_iterations == 0 {
            return Err("Optimization max_iterations must be > 0".to_string());
        }
        if !self.cost_tolerance.is_finite() || self.cost_tolerance <= 0.0 {
            return Err("Optimization cost_tolerance must be finite and > 0".to_string());
        }
        if !self.fd_step.is_finite() || self.fd_step <= 0.0 {
            return Err("Optimization fd_step must be finite and > 0".to_string());
        }
        if !self.initial_step.is_finite() || self.initial_step <= 0.0 {
            return Err("Optimization initial_step must be finite and > 0".to_string());
        }
        if !self.min_step.is_finite() || self.min_step <= 0.0 {
            return Err("Optimization min_step must be finite and > 0".to_string());
        }
        if self.min_step > self.initial_step {
            return Err("Optimization min_step must be <= initial_step".to_string());
        }
        if self.goal == OptimizationGoalMode::Target {
            if self.target.is_none() || self.target.is_some_and(|v| !v.is_finite()) {
                return Err("Optimization target goal requires a finite target value".to_string());
            }
        } else if self.target.is_some_and(|v| !v.is_finite()) {
            return Err("Optimization target must be finite when provided".to_string());
        }

        let mut seen = HashSet::new();
        for var in &self.variables {
            if !is_valid_param_identifier(&var.name) {
                return Err(format!(
                    "Invalid optimization variable name '{}': expected [A-Za-z_][A-Za-z0-9_]*",
                    var.name
                ));
            }
            if !var.min.is_finite() || !var.max.is_finite() || !var.initial.is_finite() {
                return Err(format!(
                    "Optimization variable '{}' bounds/initial must be finite",
                    var.name
                ));
            }
            if var.max <= var.min {
                return Err(format!(
                    "Optimization variable '{}' requires max > min",
                    var.name
                ));
            }
            if var.initial < var.min || var.initial > var.max {
                return Err(format!(
                    "Optimization variable '{}' initial must be within [{}, {}]",
                    var.name, var.min, var.max
                ));
            }
            if !seen.insert(var.name.to_ascii_uppercase()) {
                return Err(format!(
                    "Optimization variable '{}' is defined more than once",
                    var.name
                ));
            }
        }
        self.search.validate_domains(
            self.variables.iter().map(|variable| {
                (
                    variable.name.as_str(),
                    variable.min,
                    variable.max,
                    variable.initial,
                )
            }),
            self.algorithm == OptimizationAlgorithmMode::GradientDescent,
        )?;
        Ok(())
    }
}

/// One evaluated cost and its optional per-objective evidence.
pub(crate) struct OptimizationEvaluation {
    pub cost: Value,
    pub objectives: Vec<crate::simulation::optimizer::OptimizationObjectiveObservation>,
    pub constraints: Vec<crate::simulation::optimizer::OptimizationConstraintObservation>,
}

/// Optimization output data.
#[derive(Debug, Clone)]
pub struct OptimizationData {
    /// Values and weighted contributions at the exact best candidate.
    pub best_objectives: Vec<crate::simulation::optimizer::OptimizationObjectiveObservation>,
    pub best_constraints: Vec<crate::simulation::optimizer::OptimizationConstraintObservation>,
    /// Iteration axis points.
    pub iterations: Vec<Value>,
    /// Cost history.
    pub costs: Vec<Value>,
    /// Variable traces by name.
    pub variable_traces: HashMap<String, Vec<Value>>,
    /// Best cost reached.
    pub best_cost: Value,
    /// Best variable values.
    pub best_variables: HashMap<String, Value>,
    /// Whether convergence criterion was met.
    pub converged: bool,
}

/// Run optimization analysis with default configuration and no source path.
///
/// Test-only. The shipping path is
/// [`run_optimization_analysis_with_config_and_source_path_and_abort`], which
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
pub(crate) fn run_optimization_analysis_with_environment_and_source_path_and_abort(
    netlist_text: &str,
    config: &OptimizationRunConfig,
    source_path: Option<&Path>,
    environment: Option<&rspice_core::engine::MonteCarloEnvironment>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<OptimizationData> {
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
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    for variable in &config.variables {
        if netlist.params.get(&variable.name).is_none() {
            return Err(ServiceRunError::Failure(format!(
                "Optimization parameter {:?} is not declared in this circuit",
                variable.name,
            )));
        }
    }
    let engine = Engine::new(build_engine_config(&netlist, None));
    run_optimization_with_evaluator(config, engine.config().resource_limits, abort, |vars| {
        let candidate = materialize_optimization_candidate(
            &engine,
            &netlist,
            &config.variables,
            vars,
            environment,
            abort,
        )?;
        let value = evaluate_optimization_objective(&candidate, config, abort)?;
        if config.objective_unit.trim().is_empty() {
            Ok(value)
        } else {
            objective_unit
                .convert_value(value, &config.objective_unit)
                .map_err(ServiceRunError::Failure)
        }
    })
}

/// Search a caller-provided configured analysis while retaining the standard
/// algorithms, histories, stopping rules and failure handling.
pub(crate) fn run_optimization_with_evaluator<F>(
    config: &OptimizationRunConfig,
    limits: rspice_core::ResourceLimits,
    abort: &dyn AbortSignal,
    mut evaluate: F,
) -> ServiceRunResult<OptimizationData>
where
    F: FnMut(&HashMap<String, Value>) -> ServiceRunResult<Value>,
{
    ensure_not_aborted(abort)?;
    let mut limits = limits;
    if !config.objective_unit.trim().is_empty() {
        if limits.max_result_values < 5 {
            return Err(ServiceRunError::resource_limit(
                rspice_core::ResourceKind::ResultValues,
                5,
                limits.max_result_values,
            ));
        }
        limits.max_result_values -= 5;
    }
    let name = config
        .objective_expression
        .clone()
        .unwrap_or_else(|| format!("V({},{})", config.objective_node, config.objective_ref));
    run_optimization_with_cost_evaluator(
        config,
        limits,
        abort,
        optimizer_target_cost(config.goal),
        |variables| {
            let value = evaluate(variables)?;
            let cost = objective_to_cost(value, config.goal, config.target)?;
            Ok(OptimizationEvaluation {
                cost,
                objectives: config.objective_observations(&name, value, cost),
                constraints: Vec::new(),
            })
        },
    )
}

/// Search an already combined cost without transforming or squaring it again.
pub(crate) fn run_optimization_with_cost_evaluator<F>(
    config: &OptimizationRunConfig,
    limits: rspice_core::ResourceLimits,
    abort: &dyn AbortSignal,
    target_cost: Option<Value>,
    mut evaluate: F,
) -> ServiceRunResult<OptimizationData>
where
    F: FnMut(&HashMap<String, Value>) -> ServiceRunResult<OptimizationEvaluation>,
{
    config.validate().map_err(ServiceRunError::Failure)?;
    ensure_not_aborted(abort)?;

    let retained = config
        .max_iterations
        .saturating_add(1)
        .saturating_mul(config.variables.len().saturating_add(2));
    if retained > limits.max_result_values {
        return Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::ResultValues,
            retained,
            limits.max_result_values,
        ));
    }
    let optimizer_config = OptimizerConfig {
        algorithm: match config.algorithm {
            OptimizationAlgorithmMode::GradientDescent => OptimizerAlgo::GradientDescent,
            OptimizationAlgorithmMode::PatternSearch => OptimizerAlgo::PatternSearch,
            OptimizationAlgorithmMode::SimulatedAnnealing => OptimizerAlgo::SimulatedAnnealing,
        },
        max_iterations: config.max_iterations,
        cost_tolerance: config.cost_tolerance,
        fd_step: config.fd_step,
        initial_step: config.initial_step,
        min_step: config.min_step,
        var_tolerance: config.search.var_tolerance,
        sa_initial_temp: config.search.sa_initial_temp,
        sa_cooling_rate: config.search.sa_cooling_rate,
        random_seed: config.search.random_seed,
    };

    let mut optimizer = OptimizerEngine::with_config(optimizer_config);
    let mut coordinates = Vec::with_capacity(config.variables.len());
    for (variable_index, var) in config.variables.iter().enumerate() {
        poll_periodically(abort, variable_index)?;
        let domain = config
            .search
            .variable_domains
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(&var.name))
            .map(|(_, domain)| domain)
            .cloned()
            .unwrap_or_default();
        let mapping = domain
            .coordinates(var.min, var.max, var.initial)
            .map_err(ServiceRunError::Failure)?;
        optimizer.add_var(
            DesignVar::new(var.name.clone(), mapping.initial, mapping.min, mapping.max)
                .with_quantum(mapping.quantum()),
        );
        coordinates.push((var.name.clone(), mapping));
    }
    let physical_vars = |vars: &HashMap<String, Value>| -> HashMap<String, Value> {
        coordinates
            .iter()
            .map(|(name, mapping)| (name.clone(), mapping.physical(vars[name])))
            .collect()
    };
    let mut variable_traces: HashMap<String, Vec<Value>> = HashMap::new();
    for (variable_index, var) in config.variables.iter().enumerate() {
        poll_periodically(abort, variable_index)?;
        variable_traces.insert(
            var.name.clone(),
            Vec::with_capacity(config.max_iterations + 1),
        );
    }
    let mut iterations = Vec::with_capacity(config.max_iterations + 1);
    let mut costs = Vec::with_capacity(config.max_iterations + 1);

    let evaluated_objectives = RefCell::new(Vec::new());
    let evaluated_constraints = RefCell::new(Vec::new());
    let eval_error: RefCell<Option<ServiceRunError>> = RefCell::new(None);
    let abort_seen = Cell::new(false);
    let fatal_error_seen = Cell::new(false);
    let mut evaluations = 0usize;
    let mut cost_fn = |vars: &HashMap<String, Value>| -> OptimizationScore {
        if abort_seen.get() || fatal_error_seen.get() {
            return Value::INFINITY.into();
        }
        let evaluation = if evaluations >= limits.max_batch_runs {
            Err(ServiceRunError::resource_limit(
                rspice_core::ResourceKind::BatchRuns,
                evaluations.saturating_add(1),
                limits.max_batch_runs,
            ))
        } else {
            evaluations += 1;
            evaluate(&physical_vars(vars)).and_then(|evaluation| {
                if evaluation.cost.is_finite() {
                    let violation =
                        crate::simulation::optimizer::validate_optimization_constraints(
                            &evaluation.constraints,
                        )
                        .map_err(ServiceRunError::Failure)?;
                    *evaluated_objectives.borrow_mut() = evaluation.objectives;
                    *evaluated_constraints.borrow_mut() = evaluation.constraints;
                    Ok(OptimizationScore {
                        cost: evaluation.cost,
                        violation,
                    })
                } else {
                    Err(ServiceRunError::Failure(
                        "Optimization cost must be finite".into(),
                    ))
                }
            })
        };
        match evaluation {
            Ok(cost) => cost,
            Err(ServiceRunError::Aborted) => {
                abort_seen.set(true);
                Value::INFINITY.into()
            }
            Err(error @ ServiceRunError::ResourceLimit(_)) => {
                fatal_error_seen.set(true);
                *eval_error.borrow_mut() = Some(error);
                Value::INFINITY.into()
            }
            Err(error @ ServiceRunError::Failure(_)) => {
                if eval_error.borrow().is_none() {
                    *eval_error.borrow_mut() = Some(error);
                }
                // A failed circuit or expression must never outrank a valid
                // finite objective, regardless of that objective's scale.
                Value::INFINITY.into()
            }
        }
    };

    let initial_vars = optimizer.current_vars();
    let initial_cost = cost_fn(&initial_vars);
    propagate_optimization_fatal_error(&fatal_error_seen, &eval_error)?;
    ensure_optimization_not_aborted(abort, &abort_seen)?;
    if !initial_cost.is_valid() {
        return Err(eval_error.borrow_mut().take().unwrap_or_else(|| {
            ServiceRunError::Failure(
                "Optimization requires a finite objective cost at the initial design".into(),
            )
        }));
    }
    let mut best_objectives = evaluated_objectives.take();
    let mut best_constraints = evaluated_constraints.take();
    let mut best_observed_score = initial_cost;
    optimizer.observe_candidate(&initial_vars, initial_cost);
    record_optimization_state(
        0.0,
        &physical_vars(&initial_vars),
        initial_cost.cost,
        &mut iterations,
        &mut costs,
        &mut variable_traces,
        abort,
    )?;

    while optimizer.current_iteration() < config.max_iterations
        && !optimizer.search_finished(target_cost)
    {
        ensure_optimization_not_aborted(abort, &abort_seen)?;
        optimizer.step(&mut cost_fn);
        propagate_optimization_fatal_error(&fatal_error_seen, &eval_error)?;
        ensure_optimization_not_aborted(abort, &abort_seen)?;
        let vars = optimizer.current_vars();
        let cost = cost_fn(&vars);
        propagate_optimization_fatal_error(&fatal_error_seen, &eval_error)?;
        ensure_optimization_not_aborted(abort, &abort_seen)?;
        // The last evaluation is this accepted iterate, not a gradient probe
        // or rejected line-search point. Retain components with the same
        // feasibility-first comparison as OptimizerEngine::observe_candidate.
        if cost.better_than(best_observed_score) {
            best_observed_score = cost;
            best_objectives = evaluated_objectives.take();
            best_constraints = evaluated_constraints.take();
        }
        record_optimization_state(
            optimizer.current_iteration() as Value,
            &physical_vars(&vars),
            cost.cost,
            &mut iterations,
            &mut costs,
            &mut variable_traces,
            abort,
        )?;
    }

    ensure_not_aborted(abort)?;
    let (best_vars, best_cost) = optimizer.best_result();
    Ok(OptimizationData {
        best_objectives,
        best_constraints,
        iterations,
        costs,
        variable_traces,
        best_cost,
        best_variables: physical_vars(best_vars),
        converged: optimizer.is_converged(target_cost),
    })
}

fn propagate_optimization_fatal_error(
    fatal_error_seen: &Cell<bool>,
    eval_error: &RefCell<Option<ServiceRunError>>,
) -> ServiceRunResult<()> {
    if fatal_error_seen.get() {
        Err(eval_error
            .borrow_mut()
            .take()
            .expect("fatal optimization error must retain its typed cause"))
    } else {
        Ok(())
    }
}

fn ensure_optimization_not_aborted(
    abort: &dyn AbortSignal,
    abort_seen: &Cell<bool>,
) -> ServiceRunResult<()> {
    if abort_seen.get() {
        Err(ServiceRunError::Aborted)
    } else {
        ensure_not_aborted(abort)
    }
}

fn record_optimization_state(
    iteration: Value,
    vars: &HashMap<String, Value>,
    cost: Value,
    iterations: &mut Vec<Value>,
    costs: &mut Vec<Value>,
    variable_traces: &mut HashMap<String, Vec<Value>>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    ensure_not_aborted(abort)?;
    if !cost.is_finite() || vars.values().any(|value| !value.is_finite()) {
        return Err(ServiceRunError::Failure(
            "Optimization cannot record a non-finite accepted design or cost".into(),
        ));
    }
    iterations.push(iteration);
    costs.push(cost);
    for (trace_index, (name, trace)) in variable_traces.iter_mut().enumerate() {
        poll_periodically(abort, trace_index)?;
        let value = vars.get(name).copied().ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "Optimization candidate is missing configured variable '{name}'"
            ))
        })?;
        trace.push(value);
    }
    ensure_not_aborted(abort)
}

pub(crate) fn objective_to_cost(
    objective: Value,
    goal: OptimizationGoalMode,
    target: Option<Value>,
) -> ServiceRunResult<Value> {
    let cost = match goal {
        OptimizationGoalMode::Minimize => objective,
        OptimizationGoalMode::Maximize => -objective,
        OptimizationGoalMode::Target => {
            let t = target.expect("validated target optimization must carry a target value");
            (objective - t).powi(2)
        }
    };
    if cost.is_finite() {
        Ok(cost)
    } else {
        Err(ServiceRunError::Failure(
            "Optimization objective cost is not finite; reduce the expression scale or change the target".into(),
        ))
    }
}

fn optimizer_target_cost(goal: OptimizationGoalMode) -> Option<Value> {
    (goal == OptimizationGoalMode::Target).then_some(0.0)
}

fn evaluate_optimization_objective(
    netlist: &rspice_core::Netlist,
    config: &OptimizationRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    ensure_not_aborted(abort)?;
    let engine = Engine::new(build_engine_config(netlist, None));
    let dc = engine
        .run_dc_op_with_abort(netlist, abort)
        .map_err(|error| {
            ServiceRunError::from_core("DC operating point failed during optimization", error)
        })?;

    if let Some(expression) = &config.objective_expression {
        return objective::evaluate(expression, netlist, &dc, abort);
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
        super::apply_voltage_corner(
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

fn is_valid_param_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
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

    #[test]
    fn signed_optimization_objectives_preserve_minimize_and_maximize_ordering() {
        assert_eq!(
            objective_to_cost(-2.0, OptimizationGoalMode::Minimize, None).unwrap(),
            -2.0
        );
        assert_eq!(
            objective_to_cost(-2.0, OptimizationGoalMode::Maximize, None).unwrap(),
            2.0
        );
        assert_eq!(
            objective_to_cost(2.0, OptimizationGoalMode::Maximize, None).unwrap(),
            -2.0
        );
        assert_eq!(
            objective_to_cost(2.0, OptimizationGoalMode::Target, Some(-1.0)).unwrap(),
            9.0
        );
    }

    #[test]
    fn initial_candidate_participates_in_best_result_and_pattern_search_does_not_fake_gradient_convergence()
     {
        let mut optimizer = OptimizerEngine::with_config(OptimizerConfig {
            algorithm: OptimizerAlgo::PatternSearch,
            ..OptimizerConfig::default()
        });
        optimizer.add_var(DesignVar::new("R", 1.0, 0.0, 2.0));
        let initial = optimizer.current_vars();
        optimizer.observe_candidate(&initial, -3.0);

        let (best, cost) = optimizer.best_result();
        assert_eq!(cost, -3.0);
        assert_eq!(best.get("R"), Some(&1.0));
        assert!(
            !optimizer.is_converged(None),
            "an uncomputed zero gradient must not stop derivative-free search"
        );
    }

    #[test]
    fn bounded_finite_difference_uses_the_realized_displacement() {
        let mut optimizer = OptimizerEngine::with_config(OptimizerConfig {
            fd_step: 0.1,
            ..OptimizerConfig::default()
        });
        optimizer.add_var(DesignVar::new("X", 0.0, 0.0, 10.0));
        let gradient = optimizer.compute_gradient(&mut |vars| vars["X"] * vars["X"]);

        assert_eq!(gradient, vec![1.0]);
    }

    #[test]
    fn largest_finite_cost_is_a_valid_initial_candidate() {
        let mut optimizer = OptimizerEngine::new();
        optimizer.add_var(DesignVar::new("X", 0.5, 0.0, 1.0));
        let initial = optimizer.current_vars();
        optimizer.observe_candidate(&initial, f64::INFINITY);
        assert!(optimizer.best_result().0.is_empty());
        optimizer.observe_candidate(&initial, f64::MAX);
        assert_eq!(optimizer.best_result(), (&initial, f64::MAX));
    }
}

#[cfg(test)]
mod configured_search_limits {
    use super::*;
    use rspice_core::abort_signal::NoAbort;

    #[test]
    fn configured_optimization_bounds_history_and_candidate_evaluations() {
        let mut limits = rspice_core::ResourceLimits::default();
        limits.max_result_values = 256;
        let config = OptimizationRunConfig {
            max_iterations: usize::MAX,
            ..Default::default()
        };
        let calls = Cell::new(0);
        let error = run_optimization_with_evaluator(&config, limits, &NoAbort, |_| {
            calls.set(calls.get() + 1);
            Ok(1.0)
        })
        .unwrap_err();
        assert!(matches!(error, ServiceRunError::ResourceLimit(_)));
        assert_eq!(calls.get(), 0);
        limits.max_batch_runs = 1;
        let config = OptimizationRunConfig {
            max_iterations: 10,
            ..Default::default()
        };
        let error = run_optimization_with_evaluator(&config, limits, &NoAbort, |_| {
            calls.set(calls.get() + 1);
            Ok(1.0)
        })
        .unwrap_err();
        assert!(
            matches!(error, ServiceRunError::ResourceLimit(error) if error.resource == rspice_core::ResourceKind::BatchRuns)
        );
        assert_eq!(calls.get(), 1);
    }
}

#[cfg(test)]
mod variable_domain_tests {
    use super::*;
    use crate::simulation::optimizer::OptimizationVariableDomain as Domain;
    use rspice_core::NoAbort;

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
