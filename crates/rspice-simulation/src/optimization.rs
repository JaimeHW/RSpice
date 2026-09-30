//! Bounded optimization over a caller-provided circuit evaluator.
//!
//! The caller retains circuit preparation and execution authority. Search
//! owns candidate selection, resource limits, cancellation, and result history.

mod search;
use crate::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically};
use rspice_core::{Value, abort_signal::AbortSignal};
use rspice_results::optimization::OptimizationScore;
use rspice_simulation_contract::optimization_search::{OptimizerAlgo, OptimizerConfig};
use search::{DesignVar, OptimizerEngine};
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};

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
    pub fn operating_point_objective_unit(&self) -> rspice_core::analysis::MeasurementUnit {
        self.objective_expression.as_ref().map_or_else(
            || rspice_core::analysis::MeasurementUnit::Known("V".into()),
            |expression| rspice_core::analysis::expression_unit(expression, &HashMap::new()),
        )
    }

    pub fn objective_observations(
        &self,
        name: &str,
        value: f64,
        cost: f64,
    ) -> Vec<rspice_results::optimization::OptimizationObjectiveObservation> {
        if self.objective_unit.trim().is_empty() {
            return Vec::new();
        }
        use rspice_results::optimization::{
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

    pub fn validate(&self) -> Result<(), String> {
        self.search.validate()?;
        rspice_results::optimization::validate_requested_unit(&self.objective_unit)?;
        if self.variables.is_empty() {
            return Err("Optimization requires at least one variable".to_string());
        }
        if let Some(expression) = &self.objective_expression {
            rspice_simulation_contract::optimization_expression::validate_optimization_expression(
                expression,
            )?;
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
pub struct OptimizationEvaluation {
    pub cost: Value,
    pub objectives: Vec<rspice_results::optimization::OptimizationObjectiveObservation>,
    pub constraints: Vec<rspice_results::optimization::OptimizationConstraintObservation>,
}

/// Optimization output data.
#[derive(Debug, Clone)]
pub struct OptimizationData {
    /// Values and weighted contributions at the exact best candidate.
    pub best_objectives: Vec<rspice_results::optimization::OptimizationObjectiveObservation>,
    pub best_constraints: Vec<rspice_results::optimization::OptimizationConstraintObservation>,
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

/// Search a caller-provided configured analysis while retaining the standard
/// algorithms, histories, stopping rules and failure handling.
pub fn run_optimization_with_evaluator<F>(
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
pub fn run_optimization_with_cost_evaluator<F>(
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
                        rspice_results::optimization::validate_optimization_constraints(
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

pub fn objective_to_cost(
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
    use super::*;

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

    #[test]
    fn feasibility_fallback_does_not_reuse_the_previous_gradient() {
        // A feasibility gradient at the old point cannot prove convergence
        // after a coordinate fallback moves into the feasible region.
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
