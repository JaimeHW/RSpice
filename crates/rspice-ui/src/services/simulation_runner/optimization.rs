//! Optimization runs.
//!
//! Drives repeated analyses while varying design parameters toward a goal:
//! the variables and their bounds, the goal formulation, and the algorithm
//! that searches.

mod objective;
pub use objective::validate_optimization_expression;

use super::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically};
use super::{build_engine_config, is_ground_like, parse_runner_netlist_with_abort};
use crate::simulation::optimizer::{DesignVar, OptimizerAlgo, OptimizerConfig, OptimizerEngine};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::Engine;
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

/// Stopping and stochastic-search controls shared by drafts, workers and runs.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct OptimizationSearchControls {
    pub var_tolerance: Value,
    pub sa_initial_temp: Value,
    pub sa_cooling_rate: Value,
    pub random_seed: u64,
}

impl Default for OptimizationSearchControls {
    fn default() -> Self {
        let defaults = OptimizerConfig::default();
        Self {
            var_tolerance: defaults.var_tolerance,
            sa_initial_temp: defaults.sa_initial_temp,
            sa_cooling_rate: defaults.sa_cooling_rate,
            random_seed: defaults.random_seed,
        }
    }
}

impl OptimizationSearchControls {
    pub(crate) fn validate(&self) -> Result<(), String> {
        if !self.var_tolerance.is_finite() || self.var_tolerance <= 0.0 {
            return Err("Optimization gradient tolerance must be finite and positive".into());
        }
        if !self.sa_initial_temp.is_finite() || self.sa_initial_temp <= 0.0 {
            return Err("Annealing temperature must be finite and positive".into());
        }
        if !self.sa_cooling_rate.is_finite()
            || !(0.0..1.0).contains(&self.sa_cooling_rate)
            || self.sa_cooling_rate == 0.0
        {
            return Err("Annealing cooling rate must be between zero and one".into());
        }
        Ok(())
    }
}

/// Optimization run configuration.
#[derive(Debug, Clone)]
pub struct OptimizationRunConfig {
    pub search: OptimizationSearchControls,
    /// Optimization variables.
    pub variables: Vec<OptimizationVariable>,
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
    pub(super) fn validate(&self) -> Result<(), String> {
        self.search.validate()?;
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
        Ok(())
    }
}

/// Optimization output data.
#[derive(Debug, Clone)]
pub struct OptimizationData {
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
    config.validate().map_err(ServiceRunError::Failure)?;
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let limits = build_engine_config(&netlist, None).resource_limits;
    run_optimization_with_evaluator(config, limits, abort, |vars| {
        evaluate_optimization_objective(netlist_text, vars, config, source_path, abort)
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
    for (variable_index, var) in config.variables.iter().enumerate() {
        poll_periodically(abort, variable_index)?;
        optimizer.add_var(DesignVar::new(
            var.name.clone(),
            var.initial,
            var.min,
            var.max,
        ));
    }
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

    let eval_error: RefCell<Option<ServiceRunError>> = RefCell::new(None);
    let abort_seen = Cell::new(false);
    let fatal_error_seen = Cell::new(false);
    let mut evaluations = 0usize;
    let mut cost_fn = |vars: &HashMap<String, Value>| -> Value {
        if abort_seen.get() || fatal_error_seen.get() {
            return Value::INFINITY;
        }
        let evaluation = if evaluations >= limits.max_batch_runs {
            Err(ServiceRunError::resource_limit(
                rspice_core::ResourceKind::BatchRuns,
                evaluations.saturating_add(1),
                limits.max_batch_runs,
            ))
        } else {
            evaluations += 1;
            evaluate(vars).and_then(|value| objective_to_cost(value, config.goal, config.target))
        };
        match evaluation {
            Ok(cost) => cost,
            Err(ServiceRunError::Aborted) => {
                abort_seen.set(true);
                Value::INFINITY
            }
            Err(error @ ServiceRunError::ResourceLimit(_)) => {
                fatal_error_seen.set(true);
                *eval_error.borrow_mut() = Some(error);
                Value::INFINITY
            }
            Err(error @ ServiceRunError::Failure(_)) => {
                if eval_error.borrow().is_none() {
                    *eval_error.borrow_mut() = Some(error);
                }
                // A failed circuit or expression must never outrank a valid
                // finite objective, regardless of that objective's scale.
                Value::INFINITY
            }
        }
    };

    let initial_vars = optimizer.current_vars();
    let initial_cost = cost_fn(&initial_vars);
    propagate_optimization_fatal_error(&fatal_error_seen, &eval_error)?;
    ensure_optimization_not_aborted(abort, &abort_seen)?;
    if !initial_cost.is_finite() {
        return Err(eval_error.borrow_mut().take().unwrap_or_else(|| {
            ServiceRunError::Failure(
                "Optimization requires a finite objective cost at the initial design".into(),
            )
        }));
    }
    optimizer.observe_candidate(&initial_vars, initial_cost);
    record_optimization_state(
        0.0,
        &initial_vars,
        initial_cost,
        &mut iterations,
        &mut costs,
        &mut variable_traces,
        abort,
    )?;

    while optimizer.current_iteration() < config.max_iterations
        && !optimizer.is_converged(optimizer_target_cost(config.goal))
    {
        ensure_optimization_not_aborted(abort, &abort_seen)?;
        optimizer.step(&mut cost_fn);
        propagate_optimization_fatal_error(&fatal_error_seen, &eval_error)?;
        ensure_optimization_not_aborted(abort, &abort_seen)?;
        let vars = optimizer.current_vars();
        let cost = cost_fn(&vars);
        propagate_optimization_fatal_error(&fatal_error_seen, &eval_error)?;
        ensure_optimization_not_aborted(abort, &abort_seen)?;
        record_optimization_state(
            optimizer.current_iteration() as Value,
            &vars,
            cost,
            &mut iterations,
            &mut costs,
            &mut variable_traces,
            abort,
        )?;
    }

    ensure_not_aborted(abort)?;
    let (best_vars, best_cost) = optimizer.best_result();
    Ok(OptimizationData {
        iterations,
        costs,
        variable_traces,
        best_cost,
        best_variables: best_vars.clone(),
        converged: optimizer.is_converged(optimizer_target_cost(config.goal)),
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

fn objective_to_cost(
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
    netlist_text: &str,
    vars: &HashMap<String, Value>,
    config: &OptimizationRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    ensure_not_aborted(abort)?;
    let overridden = inject_param_overrides(netlist_text, vars, abort)?;
    let netlist = parse_runner_netlist_with_abort(&overridden, source_path, abort)?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let dc = engine
        .run_dc_op_with_abort(&netlist, abort)
        .map_err(|error| {
            ServiceRunError::from_core("DC operating point failed during optimization", error)
        })?;

    if let Some(expression) = &config.objective_expression {
        return objective::evaluate(expression, &netlist, &dc, abort);
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

fn inject_param_overrides(
    netlist_text: &str,
    vars: &HashMap<String, Value>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<String> {
    ensure_not_aborted(abort)?;
    if vars.is_empty() {
        return Ok(netlist_text.to_string());
    }

    let mut entries = Vec::with_capacity(vars.len());
    for (index, (name, value)) in vars.iter().enumerate() {
        poll_periodically(abort, index)?;
        if is_valid_param_identifier(name) {
            entries.push((name.to_ascii_uppercase(), name.clone(), *value));
        }
    }
    ensure_not_aborted(abort)?;
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    if entries.is_empty() {
        return Ok(netlist_text.to_string());
    }

    let mut lines = Vec::new();
    for (line_index, line) in netlist_text.lines().enumerate() {
        poll_periodically(abort, line_index)?;
        lines.push(line.to_string());
    }
    if lines.is_empty() {
        let mut line = ".param".to_string();
        for (entry_index, (_, name, value)) in entries.iter().enumerate() {
            poll_periodically(abort, entry_index)?;
            line.push(' ');
            line.push_str(name);
            line.push('=');
            line.push_str(&format_param_override_value(*value));
        }
        ensure_not_aborted(abort)?;
        return Ok(format!("{}\n", line));
    }

    let mut overrides_found: HashSet<String> = HashSet::new();

    for (line_index, line) in lines.iter_mut().enumerate().skip(1) {
        poll_periodically(abort, line_index)?;
        if !is_param_directive_line(line) {
            continue;
        }

        let assigned = collect_param_assignment_names(line);
        let mut append_parts = Vec::new();
        for (entry_index, (upper, name, value)) in entries.iter().enumerate() {
            poll_periodically(abort, entry_index)?;
            if assigned.contains(upper) {
                overrides_found.insert(upper.clone());
                append_parts.push(format!("{}={}", name, format_param_override_value(*value)));
            }
        }

        if !append_parts.is_empty() {
            let suffix = append_parts.join(" ");
            if let Some(comment_idx) = line.find(';') {
                let (head, comment) = line.split_at(comment_idx);
                let mut rebuilt = head.trim_end().to_string();
                rebuilt.push(' ');
                rebuilt.push_str(&suffix);
                rebuilt.push(' ');
                rebuilt.push_str(comment.trim_start());
                *line = rebuilt;
            } else {
                line.push(' ');
                line.push_str(&suffix);
            }
        }
    }

    let mut missing = Vec::new();
    for (entry_index, (upper, name, value)) in entries.iter().enumerate() {
        poll_periodically(abort, entry_index)?;
        if !overrides_found.contains(upper) {
            missing.push((name.clone(), *value));
        }
    }

    if !missing.is_empty() {
        let mut line = ".param".to_string();
        for (entry_index, (name, value)) in missing.iter().enumerate() {
            poll_periodically(abort, entry_index)?;
            line.push(' ');
            line.push_str(name);
            line.push('=');
            line.push_str(&format_param_override_value(*value));
        }
        lines.insert(1, line);
    }

    let mut out = lines.join("\n");
    if netlist_text.ends_with('\n') {
        out.push('\n');
    }
    ensure_not_aborted(abort)?;
    Ok(out)
}

fn format_param_override_value(value: Value) -> String {
    let raw = format!("{:.16e}", value);
    let Some(exp_pos) = raw.find('e') else {
        return raw;
    };
    let mantissa = &raw[..exp_pos];
    let exponent = &raw[exp_pos + 1..];
    match exponent.parse::<i32>() {
        Ok(exp) => format!("{}e{:+03}", mantissa, exp),
        Err(_) => raw,
    }
}

fn is_param_directive_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    if !trimmed
        .get(..6)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(".param"))
    {
        return false;
    }
    trimmed
        .as_bytes()
        .get(6)
        .is_none_or(|ch| ch.is_ascii_whitespace())
}

fn collect_param_assignment_names(line: &str) -> HashSet<String> {
    let mut names = HashSet::new();
    let trimmed = line.trim_start();
    if !trimmed
        .get(..6)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(".param"))
    {
        return names;
    }
    let rest = trimmed[6..].split(';').next().unwrap_or("").trim();
    let bytes = rest.as_bytes();
    let len = bytes.len();
    let mut i = 0usize;

    while i < len {
        while i < len && (bytes[i].is_ascii_whitespace() || bytes[i] == b',') {
            i += 1;
        }
        if i >= len {
            break;
        }

        let start = i;
        let first = bytes[i];
        if !(first.is_ascii_alphabetic() || first == b'_') {
            i += 1;
            continue;
        }
        i += 1;
        while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
            i += 1;
        }
        let name = &rest[start..i];

        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i < len && bytes[i] == b'=' {
            names.insert(name.to_ascii_uppercase());
        }
    }

    names
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
