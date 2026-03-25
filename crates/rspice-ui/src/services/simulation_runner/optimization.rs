use super::{build_engine_config, is_ground_like, parse_runner_netlist};
use crate::simulation::optimizer::{
    DesignVar, OptimizationGoal, OptimizerAlgo, OptimizerConfig, OptimizerEngine,
};
use rspice_core::Value;
use rspice_core::engine::Engine;
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
    run_optimization_analysis_with_source_path(netlist_text, None)
}

/// Run optimization analysis with default configuration and a source path used
/// to resolve relative includes and model file references.
pub fn run_optimization_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<OptimizationData, String> {
    run_optimization_analysis_with_config_and_source_path(
        netlist_text,
        &OptimizationRunConfig::default(),
        source_path,
    )
}

/// Run optimization analysis with explicit configuration.
pub fn run_optimization_analysis_with_config(
    netlist_text: &str,
    config: &OptimizationRunConfig,
) -> Result<OptimizationData, String> {
    run_optimization_analysis_with_config_and_source_path(netlist_text, config, None)
}

/// Run optimization analysis with explicit configuration and a source path used
/// to resolve relative includes and model file references.
pub fn run_optimization_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &OptimizationRunConfig,
    source_path: Option<&Path>,
) -> Result<OptimizationData, String> {
    config.validate()?;

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
    for var in &config.variables {
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
    for var in &config.variables {
        variable_traces.insert(
            var.name.clone(),
            Vec::with_capacity(config.max_iterations + 1),
        );
    }
    let mut iterations = Vec::with_capacity(config.max_iterations + 1);
    let mut costs = Vec::with_capacity(config.max_iterations + 1);

    let mut eval_error: Option<String> = None;
    let mut successful_evals: usize = 0;
    let mut cost_fn = |vars: &HashMap<String, Value>| -> Value {
        match evaluate_optimization_objective(netlist_text, vars, config, source_path) {
            Ok(value) => {
                successful_evals += 1;
                objective_to_cost(value, config.goal, config.target)
            }
            Err(err) => {
                if eval_error.is_none() {
                    eval_error = Some(err);
                }
                1e30
            }
        }
    };

    let mut record_state = |iter: Value, vars: &HashMap<String, Value>, cost: Value| {
        iterations.push(iter);
        costs.push(cost);
        for (name, trace) in &mut variable_traces {
            trace.push(vars.get(name).copied().unwrap_or(0.0));
        }
    };

    let initial_vars = optimizer.current_vars();
    let initial_cost = cost_fn(&initial_vars);
    record_state(0.0, &initial_vars, initial_cost);

    while optimizer.current_iteration() < config.max_iterations {
        optimizer.step(&mut cost_fn);
        let vars = optimizer.current_vars();
        let cost = cost_fn(&vars);
        record_state(optimizer.current_iteration() as Value, &vars, cost);
        if optimizer.is_converged() {
            break;
        }
    }

    if successful_evals == 0 {
        return Err(eval_error.unwrap_or_else(|| {
            "Optimization failed: objective evaluation returned no valid samples".to_string()
        }));
    }

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
) -> Result<Value, String> {
    let overridden = inject_param_overrides(netlist_text, vars);
    let netlist = parse_runner_netlist(&overridden, source_path)?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let dc = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC operating point failed during optimization: {}", e))?;

    let node_idx = resolve_node_index_case_insensitive(&dc.node_names, &config.objective_node)
        .ok_or_else(|| {
            format!(
                "Optimization objective node '{}' not found",
                config.objective_node
            )
        })?;
    let ref_idx = if is_ground_like(&config.objective_ref) {
        Some(0usize)
    } else {
        resolve_node_index_case_insensitive(&dc.node_names, &config.objective_ref)
    }
    .ok_or_else(|| {
        format!(
            "Optimization objective reference node '{}' not found",
            config.objective_ref
        )
    })?;

    let node_v = *dc
        .node_voltages
        .get(node_idx)
        .ok_or_else(|| "Optimization node voltage index out of bounds".to_string())?;
    let ref_v = *dc
        .node_voltages
        .get(ref_idx)
        .ok_or_else(|| "Optimization reference voltage index out of bounds".to_string())?;
    Ok(node_v - ref_v)
}

fn inject_param_overrides(netlist_text: &str, vars: &HashMap<String, Value>) -> String {
    if vars.is_empty() {
        return netlist_text.to_string();
    }

    let mut entries: Vec<(String, String, Value)> = vars
        .iter()
        .filter_map(|(name, value)| {
            if is_valid_param_identifier(name) {
                Some((name.to_ascii_uppercase(), name.clone(), *value))
            } else {
                None
            }
        })
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    if entries.is_empty() {
        return netlist_text.to_string();
    }

    let mut lines: Vec<String> = netlist_text.lines().map(str::to_string).collect();
    if lines.is_empty() {
        let mut line = ".param".to_string();
        for (_, name, value) in &entries {
            line.push(' ');
            line.push_str(name);
            line.push('=');
            line.push_str(&format_param_override_value(*value));
        }
        return format!("{}\n", line);
    }

    let mut overrides_found: HashSet<String> = HashSet::new();

    for line in lines.iter_mut().skip(1) {
        if !is_param_directive_line(line) {
            continue;
        }

        let assigned = collect_param_assignment_names(line);
        let mut append_parts = Vec::new();
        for (upper, name, value) in &entries {
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

    let missing: Vec<(String, Value)> = entries
        .iter()
        .filter(|(upper, _, _)| !overrides_found.contains(upper))
        .map(|(_, name, value)| (name.clone(), *value))
        .collect();

    if !missing.is_empty() {
        let mut line = ".param".to_string();
        for (name, value) in &missing {
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
    out
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

fn resolve_node_index_case_insensitive(node_names: &[String], target: &str) -> Option<usize> {
    let trimmed = target.trim();
    if trimmed.is_empty() {
        return None;
    }
    node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(trimmed))
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
pub(super) fn inject_param_overrides_for_tests(
    netlist_text: &str,
    vars: &HashMap<String, Value>,
) -> String {
    inject_param_overrides(netlist_text, vars)
}

#[cfg(test)]
pub(super) fn format_param_override_value_for_tests(value: Value) -> String {
    format_param_override_value(value)
}
