use super::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted, poll_periodically};
use super::{build_engine_config, is_ground_like, parse_runner_netlist_with_abort};
use crate::simulation::optimizer::{
    DesignVar, OptimizationGoal, OptimizerAlgo, OptimizerConfig, OptimizerEngine,
};
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
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

/// Optimization run configuration.
#[derive(Debug, Clone)]
pub struct OptimizationRunConfig {
    /// Optimization variables.
    pub variables: Vec<OptimizationVariable>,
    /// Objective node.
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
            variables: vec![OptimizationVariable {
                name: "RLOAD".to_string(),
                min: 500.0,
                max: 5000.0,
                initial: 1000.0,
            }],
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
        if self.variables.is_empty() {
            return Err("Optimization requires at least one variable".to_string());
        }
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

/// Run optimization analysis with default configuration.
pub fn run_optimization_analysis(netlist_text: &str) -> Result<OptimizationData, String> {
    run_optimization_analysis_with_abort(netlist_text, &NoAbort).map_err(|error| error.to_string())
}

/// Run optimization analysis with cooperative cancellation.
pub fn run_optimization_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<OptimizationData> {
    run_optimization_analysis_with_source_path_and_abort(netlist_text, None, abort)
}

/// Run optimization analysis with default configuration and a source path used
/// to resolve relative includes and model file references.
pub fn run_optimization_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<OptimizationData, String> {
    run_optimization_analysis_with_source_path_and_abort(netlist_text, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run optimization analysis with source-path resolution and cooperative
/// cancellation.
pub fn run_optimization_analysis_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<OptimizationData> {
    run_optimization_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        &OptimizationRunConfig::default(),
        source_path,
        abort,
    )
}

/// Run optimization analysis with explicit configuration.
pub fn run_optimization_analysis_with_config(
    netlist_text: &str,
    config: &OptimizationRunConfig,
) -> Result<OptimizationData, String> {
    run_optimization_analysis_with_config_and_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run explicitly configured optimization analysis with cooperative
/// cancellation.
pub fn run_optimization_analysis_with_config_and_abort(
    netlist_text: &str,
    config: &OptimizationRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<OptimizationData> {
    run_optimization_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        config,
        None,
        abort,
    )
}

/// Run optimization analysis with explicit configuration and a source path used
/// to resolve relative includes and model file references.
pub fn run_optimization_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &OptimizationRunConfig,
    source_path: Option<&Path>,
) -> Result<OptimizationData, String> {
    run_optimization_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        config,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
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
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;
    ensure_not_aborted(abort)?;

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
        ..OptimizerConfig::default()
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
    let mut synthetic_goal = match config.goal {
        OptimizationGoalMode::Minimize => OptimizationGoal::minimize("__objective"),
        OptimizationGoalMode::Maximize => OptimizationGoal::maximize("__objective"),
        OptimizationGoalMode::Target => {
            OptimizationGoal::hit_target("__objective", config.target.unwrap_or_default())
        }
    };
    synthetic_goal.weight = 1.0;
    optimizer.add_goal(synthetic_goal);

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

    let eval_error: RefCell<Option<String>> = RefCell::new(None);
    let successful_evals = Cell::new(0usize);
    let abort_seen = Cell::new(false);
    let mut cost_fn = |vars: &HashMap<String, Value>| -> Value {
        if abort_seen.get() {
            return 1e30;
        }
        match evaluate_optimization_objective(netlist_text, vars, config, source_path, abort) {
            Ok(value) => {
                successful_evals.set(successful_evals.get().saturating_add(1));
                objective_to_cost(value, config.goal, config.target)
            }
            Err(ServiceRunError::Aborted) => {
                abort_seen.set(true);
                1e30
            }
            Err(ServiceRunError::Failure(error)) => {
                if eval_error.borrow().is_none() {
                    *eval_error.borrow_mut() = Some(error);
                }
                1e30
            }
        }
    };

    let initial_vars = optimizer.current_vars();
    let initial_cost = cost_fn(&initial_vars);
    ensure_optimization_not_aborted(abort, &abort_seen)?;
    record_optimization_state(
        0.0,
        &initial_vars,
        initial_cost,
        &mut iterations,
        &mut costs,
        &mut variable_traces,
        abort,
    )?;

    while optimizer.current_iteration() < config.max_iterations {
        ensure_optimization_not_aborted(abort, &abort_seen)?;
        optimizer.step(&mut cost_fn);
        ensure_optimization_not_aborted(abort, &abort_seen)?;
        let vars = optimizer.current_vars();
        let cost = cost_fn(&vars);
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
        if optimizer.is_converged() {
            break;
        }
    }

    if successful_evals.get() == 0 {
        return Err(ServiceRunError::Failure(
            eval_error.into_inner().unwrap_or_else(|| {
                "Optimization failed: objective evaluation returned no valid samples".to_string()
            }),
        ));
    }

    ensure_not_aborted(abort)?;
    let (best_vars, best_cost) = optimizer.best_result();
    Ok(OptimizationData {
        iterations,
        costs,
        variable_traces,
        best_cost,
        best_variables: best_vars.clone(),
        converged: optimizer.is_converged(),
    })
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

#[allow(clippy::too_many_arguments)]
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
    iterations.push(iteration);
    costs.push(cost);
    for (trace_index, (name, trace)) in variable_traces.iter_mut().enumerate() {
        poll_periodically(abort, trace_index)?;
        trace.push(vars.get(name).copied().unwrap_or(0.0));
    }
    ensure_not_aborted(abort)
}

fn objective_to_cost(objective: Value, goal: OptimizationGoalMode, target: Option<Value>) -> Value {
    match goal {
        OptimizationGoalMode::Minimize => objective.abs(),
        OptimizationGoalMode::Maximize => {
            if objective > 0.0 {
                1.0 / objective
            } else {
                1e30
            }
        }
        OptimizationGoalMode::Target => {
            let t = target.unwrap_or_default();
            (objective - t).powi(2)
        }
    }
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

    let node_v = *dc.node_voltages.get(node_idx).ok_or_else(|| {
        ServiceRunError::Failure("Optimization node voltage index out of bounds".to_string())
    })?;
    let ref_v = *dc.node_voltages.get(ref_idx).ok_or_else(|| {
        ServiceRunError::Failure("Optimization reference voltage index out of bounds".to_string())
    })?;
    ensure_not_aborted(abort)?;
    Ok(node_v - ref_v)
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
