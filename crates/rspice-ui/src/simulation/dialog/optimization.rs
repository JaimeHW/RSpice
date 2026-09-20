//! Optimization analysis configuration dialog.
//!
//! Provides a typed UI surface for closed-loop parameter optimization.

use super::options::parse_si_value;
use crate::services::simulation_runner::OptimizationSearchControls;

/// Optimization objective strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationGoalMode {
    /// Minimize objective value.
    Minimize,
    /// Maximize objective value.
    Maximize,
    /// Reach a specific objective value.
    Target,
}

impl OptimizationGoalMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Minimize => "minimize",
            Self::Maximize => "maximize",
            Self::Target => "target",
        }
    }
}

/// Optimization algorithm choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationAlgorithmMode {
    /// Gradient descent with line search.
    GradientDescent,
    /// Pattern search (direct search).
    PatternSearch,
    /// Simulated annealing.
    SimulatedAnnealing,
}

impl OptimizationAlgorithmMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::GradientDescent => "gradient_descent",
            Self::PatternSearch => "pattern_search",
            Self::SimulatedAnnealing => "simulated_annealing",
        }
    }
}

/// Design variable optimization bounds.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationVariableConfig {
    /// Parameter name from netlist `.param`.
    pub name: String,
    /// Lower bound.
    pub min: f64,
    /// Upper bound.
    pub max: f64,
    /// Initial value.
    pub initial: f64,
}

impl OptimizationVariableConfig {
    fn validate(&self) -> Result<(), String> {
        if !is_valid_identifier(&self.name) {
            return Err(format!(
                "Invalid variable name '{}': must start with a letter/underscore and contain only [A-Za-z0-9_]",
                self.name
            ));
        }
        if !self.min.is_finite() || !self.max.is_finite() || !self.initial.is_finite() {
            return Err(format!(
                "Variable '{}' bounds and initial value must be finite",
                self.name
            ));
        }
        if self.max <= self.min {
            return Err(format!(
                "Variable '{}' requires max > min (got {} <= {})",
                self.name, self.max, self.min
            ));
        }
        if self.initial < self.min || self.initial > self.max {
            return Err(format!(
                "Variable '{}' initial value {} is outside [{}, {}]",
                self.name, self.initial, self.min, self.max
            ));
        }
        Ok(())
    }
}

/// Typed optimization configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct OptimizationConfig {
    pub base_analysis: Option<crate::product::AnalysisInstanceId>,
    pub objective_measurement: String,
    pub search: OptimizationSearchControls,
    /// Variable set to optimize.
    pub variables: Vec<OptimizationVariableConfig>,
    /// Optional scalar operating-point expression; overrides the voltage objective.
    pub objective_expression: Option<String>,
    /// Objective node (V(node,ref)) when no expression is configured.
    pub objective_node: String,
    /// Objective reference node.
    pub objective_ref: String,
    /// Objective strategy.
    pub goal_mode: OptimizationGoalMode,
    /// Optional target value (required for target mode).
    pub target_value: Option<f64>,
    /// Algorithm.
    pub algorithm: OptimizationAlgorithmMode,
    /// Maximum iterations.
    pub max_iterations: usize,
    /// Cost tolerance.
    pub cost_tolerance: f64,
    /// Finite-difference step (relative).
    pub fd_step: f64,
    /// Initial algorithm step.
    pub initial_step: f64,
    /// Minimum step.
    pub min_step: f64,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            base_analysis: None,
            objective_measurement: String::new(),
            search: OptimizationSearchControls::default(),
            variables: vec![
                OptimizationVariableConfig {
                    name: "RLOAD".to_string(),
                    min: 500.0,
                    max: 5_000.0,
                    initial: 1_000.0,
                },
                OptimizationVariableConfig {
                    name: "VDD".to_string(),
                    min: 1.0,
                    max: 2.0,
                    initial: 1.2,
                },
            ],
            objective_expression: None,
            objective_node: "out".to_string(),
            objective_ref: "0".to_string(),
            goal_mode: OptimizationGoalMode::Target,
            target_value: Some(1.2),
            algorithm: OptimizationAlgorithmMode::PatternSearch,
            max_iterations: 120,
            cost_tolerance: 1e-8,
            fd_step: 1e-4,
            initial_step: 0.1,
            min_step: 1e-8,
        }
    }
}

impl OptimizationConfig {
    /// Validate optimization settings.
    pub fn validate(&self) -> Result<(), String> {
        self.search.validate()?;
        if self.variables.is_empty() {
            return Err("At least one optimization variable is required".to_string());
        }
        if self.base_analysis.is_some() {
            crate::simulation::runner::study::validate_measurements(std::slice::from_ref(
                &self.objective_measurement,
            ))?;
        } else if let Some(expression) = &self.objective_expression {
            crate::services::simulation_runner::validate_optimization_expression(expression)?;
        } else {
            if self.objective_node.trim().is_empty() {
                return Err("Objective node must not be empty".to_string());
            }
            if self.objective_ref.trim().is_empty() {
                return Err("Objective reference must not be empty".to_string());
            }
            if self
                .objective_node
                .eq_ignore_ascii_case(&self.objective_ref)
            {
                return Err("Objective node and reference must differ".to_string());
            }
        }
        if self.max_iterations == 0 {
            return Err("max_iterations must be > 0".to_string());
        }
        if !self.cost_tolerance.is_finite() || self.cost_tolerance <= 0.0 {
            return Err("cost_tolerance must be finite and > 0".to_string());
        }
        if !self.fd_step.is_finite() || self.fd_step <= 0.0 {
            return Err("fd_step must be finite and > 0".to_string());
        }
        if !self.initial_step.is_finite() || self.initial_step <= 0.0 {
            return Err("initial_step must be finite and > 0".to_string());
        }
        if !self.min_step.is_finite() || self.min_step <= 0.0 {
            return Err("min_step must be finite and > 0".to_string());
        }
        if self.min_step > self.initial_step {
            return Err("min_step must be <= initial_step".to_string());
        }
        match (self.goal_mode, self.target_value) {
            (OptimizationGoalMode::Target, Some(v)) if v.is_finite() => {}
            (OptimizationGoalMode::Target, _) => {
                return Err("Target mode requires a finite target value".to_string());
            }
            (_, Some(v)) if !v.is_finite() => {
                return Err("target_value must be finite when provided".to_string());
            }
            _ => {}
        }

        let mut seen = std::collections::HashSet::new();
        for var in &self.variables {
            var.validate()?;
            let key = var.name.to_ascii_uppercase();
            if !seen.insert(key) {
                return Err(format!("Duplicate optimization variable '{}'", var.name));
            }
        }
        Ok(())
    }

    /// The record this optimization writes into the deck.
    ///
    /// A comment, not a directive. No SPICE dialect spells an optimization
    /// card, and in this engine `.OPT` is an accepted alias for `.OPTIONS` —
    /// so the `.opt algo=...` line the studio used to write was not merely
    /// unread, it was handed to the solver-options parser, which refused it
    /// and with it the whole prepared deck. The run itself is driven from
    /// `AnalysisSpec::Optimization`, which `runner::spec::device` hands to
    /// `run_optimization` field by field; this line is the deck's readable
    /// record of what was asked for, spelled the way PSP, HBSP and HBNOISE
    /// spell theirs.
    pub fn to_spice(&self) -> String {
        let vars = self
            .variables
            .iter()
            .map(|v| format!("{}:{:.6e}:{:.6e}:{:.6e}", v.name, v.min, v.max, v.initial))
            .collect::<Vec<_>>()
            .join(",");
        let objective = if let Some(id) = self.base_analysis {
            format!("base={id} measurement={}", self.objective_measurement)
        } else {
            format!("obj=V({},{})", self.objective_node, self.objective_ref)
        };
        let mut line = format!(
            "* RSPICE OPT algo={} goal={} {} maxiter={} ctol={:.6e} fd={:.6e} initstep={:.6e} minstep={:.6e} vars={}",
            self.algorithm.as_str(),
            self.goal_mode.as_str(),
            objective,
            self.max_iterations,
            self.cost_tolerance,
            self.fd_step,
            self.initial_step,
            self.min_step,
            vars
        );
        line.push_str(&format!(
            " gtol={} temp={} cooling={} seed={}",
            self.search.var_tolerance,
            self.search.sa_initial_temp,
            self.search.sa_cooling_rate,
            self.search.random_seed
        ));
        if let Some(expression) = &self.objective_expression {
            line.push_str(&format!(" expr={{{expression}}}"));
        }
        if let Some(target) = self.target_value {
            line.push_str(&format!(" target={:.6e}", target));
        }
        line
    }
}

/// UI state for optimization dialog tab.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptimizationDialogState {
    #[serde(default)]
    pub base_analysis: Option<crate::product::AnalysisInstanceId>,
    #[serde(default)]
    pub objective_measurement: String,
    #[serde(default = "default_gradient_tolerance")]
    pub var_tolerance: String,
    #[serde(default = "default_annealing_temperature")]
    pub sa_initial_temp: String,
    #[serde(default = "default_cooling_rate")]
    pub sa_cooling_rate: String,
    #[serde(default = "default_seed")]
    pub random_seed: String,
    /// Variables encoded as `name:min:max[:initial]`, separated by newline/comma.
    pub variables_text: String,
    /// Optional scalar operating-point expression.
    #[serde(default)]
    pub objective_expression: String,
    pub objective_node: String,
    /// Objective reference node.
    pub objective_ref: String,
    /// Goal mode index (0=min, 1=max, 2=target).
    pub goal_mode: usize,
    /// Target value buffer.
    pub target_value: String,
    /// Algorithm index (0=gd, 1=pattern, 2=anneal).
    pub algorithm: usize,
    /// Max iterations.
    pub max_iterations: String,
    /// Cost tolerance.
    pub cost_tolerance: String,
    /// Finite difference step.
    pub fd_step: String,
    /// Initial step.
    pub initial_step: String,
    /// Minimum step.
    pub min_step: String,
    /// Lazy default initialization.
    #[serde(skip)]
    pub initialized: bool,
}

impl OptimizationDialogState {
    /// Build UI state from typed config.
    pub fn from_config(config: &OptimizationConfig) -> Self {
        let variables_text = config
            .variables
            .iter()
            .map(|v| {
                format!(
                    "{}:{}:{}:{}",
                    v.name,
                    format_scalar(v.min),
                    format_scalar(v.max),
                    format_scalar(v.initial)
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        Self {
            base_analysis: config.base_analysis,
            objective_measurement: config.objective_measurement.clone(),
            var_tolerance: config.search.var_tolerance.to_string(),
            sa_initial_temp: config.search.sa_initial_temp.to_string(),
            sa_cooling_rate: config.search.sa_cooling_rate.to_string(),
            random_seed: config.search.random_seed.to_string(),
            variables_text,
            objective_expression: config.objective_expression.clone().unwrap_or_default(),
            objective_node: config.objective_node.clone(),
            objective_ref: config.objective_ref.clone(),
            goal_mode: match config.goal_mode {
                OptimizationGoalMode::Minimize => 0,
                OptimizationGoalMode::Maximize => 1,
                OptimizationGoalMode::Target => 2,
            },
            target_value: config.target_value.map(format_scalar).unwrap_or_default(),
            algorithm: match config.algorithm {
                OptimizationAlgorithmMode::GradientDescent => 0,
                OptimizationAlgorithmMode::PatternSearch => 1,
                OptimizationAlgorithmMode::SimulatedAnnealing => 2,
            },
            max_iterations: config.max_iterations.to_string(),
            cost_tolerance: format_scalar(config.cost_tolerance),
            fd_step: format_scalar(config.fd_step),
            initial_step: format_scalar(config.initial_step),
            min_step: format_scalar(config.min_step),
            initialized: true,
        }
    }

    /// Convert UI state into typed config.
    pub fn to_config(&self) -> Result<OptimizationConfig, String> {
        let goal_mode = match self.goal_mode {
            0 => OptimizationGoalMode::Minimize,
            1 => OptimizationGoalMode::Maximize,
            _ => OptimizationGoalMode::Target,
        };
        let algorithm = match self.algorithm {
            0 => OptimizationAlgorithmMode::GradientDescent,
            1 => OptimizationAlgorithmMode::PatternSearch,
            _ => OptimizationAlgorithmMode::SimulatedAnnealing,
        };

        let target_value = if goal_mode == OptimizationGoalMode::Target {
            Some(
                parse_si_value(&self.target_value)
                    .map_err(|e| format!("Invalid optimization target value: {}", e))?,
            )
        } else if self.target_value.trim().is_empty() {
            None
        } else {
            Some(
                parse_si_value(&self.target_value)
                    .map_err(|e| format!("Invalid optimization target value: {}", e))?,
            )
        };

        let max_iterations = self
            .max_iterations
            .trim()
            .parse::<usize>()
            .map_err(|_| "Invalid max iterations".to_string())?;
        let variables = parse_variable_specs(&self.variables_text)?;
        let config = OptimizationConfig {
            base_analysis: self.base_analysis,
            objective_measurement: self.objective_measurement.trim().to_owned(),
            search: OptimizationSearchControls {
                var_tolerance: parse_si_value(&self.var_tolerance)
                    .map_err(|e| format!("Invalid gradient tolerance: {e}"))?,
                sa_initial_temp: parse_si_value(&self.sa_initial_temp)
                    .map_err(|e| format!("Invalid annealing temperature: {e}"))?,
                sa_cooling_rate: parse_si_value(&self.sa_cooling_rate)
                    .map_err(|e| format!("Invalid cooling rate: {e}"))?,
                random_seed: self
                    .random_seed
                    .trim()
                    .parse()
                    .map_err(|_| "Seed must be an unsigned 64-bit integer")?,
            },
            variables,
            // Inactive legacy objective buffers remain in the saved draft;
            // the bound measurement supplies the actual objective at execution.
            objective_expression: (self.base_analysis.is_none()
                && !self.objective_expression.trim().is_empty())
            .then(|| self.objective_expression.trim().to_string()),
            objective_node: if self.base_analysis.is_some() {
                OptimizationConfig::default().objective_node
            } else {
                self.objective_node.trim().to_string()
            },
            objective_ref: if self.base_analysis.is_some() {
                OptimizationConfig::default().objective_ref
            } else {
                self.objective_ref.trim().to_string()
            },
            goal_mode,
            target_value,
            algorithm,
            max_iterations,
            cost_tolerance: parse_si_value(&self.cost_tolerance)
                .map_err(|e| format!("Invalid cost tolerance: {}", e))?,
            fd_step: parse_si_value(&self.fd_step)
                .map_err(|e| format!("Invalid fd step: {}", e))?,
            initial_step: parse_si_value(&self.initial_step)
                .map_err(|e| format!("Invalid initial step: {}", e))?,
            min_step: parse_si_value(&self.min_step)
                .map_err(|e| format!("Invalid minimum step: {}", e))?,
        };
        config.validate()?;
        Ok(config)
    }

    /// Initialize with defaults once.
    pub fn ensure_initialized(&mut self) {
        if !self.initialized {
            *self = Self::from_config(&OptimizationConfig::default());
        }
    }
}

fn format_scalar(v: f64) -> String {
    v.to_string()
}
fn default_gradient_tolerance() -> String {
    OptimizationSearchControls::default()
        .var_tolerance
        .to_string()
}
fn default_annealing_temperature() -> String {
    OptimizationSearchControls::default()
        .sa_initial_temp
        .to_string()
}
fn default_cooling_rate() -> String {
    OptimizationSearchControls::default()
        .sa_cooling_rate
        .to_string()
}
fn default_seed() -> String {
    OptimizationSearchControls::default()
        .random_seed
        .to_string()
}

fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn parse_variable_specs(input: &str) -> Result<Vec<OptimizationVariableConfig>, String> {
    let mut variables = Vec::new();
    for token in input.split(['\n', ',', ';']) {
        let raw = token.trim();
        if raw.is_empty() {
            continue;
        }
        let parts: Vec<&str> = raw.split(':').map(str::trim).collect();
        if parts.len() < 3 || parts.len() > 4 {
            return Err(format!(
                "Invalid variable specification '{}'; expected name:min:max[:initial]",
                raw
            ));
        }

        let name = parts[0].to_string();
        let min = parse_si_value(parts[1])
            .map_err(|e| format!("Invalid min bound in '{}': {}", raw, e))?;
        let max = parse_si_value(parts[2])
            .map_err(|e| format!("Invalid max bound in '{}': {}", raw, e))?;
        let initial = if parts.len() == 4 {
            parse_si_value(parts[3])
                .map_err(|e| format!("Invalid initial value in '{}': {}", raw, e))?
        } else {
            0.5 * (min + max)
        };
        let var = OptimizationVariableConfig {
            name,
            min,
            max,
            initial,
        };
        var.validate()?;
        variables.push(var);
    }

    if variables.is_empty() {
        return Err("At least one optimization variable must be provided".to_string());
    }
    Ok(variables)
}

#[cfg(test)]
mod search_tests {
    use super::*;
    #[test]
    fn optimization_expression_survives_draft_restoration_and_legacy_omission() {
        let config = OptimizationConfig {
            objective_expression: Some("-V(in)*I(V1)".into()),
            ..Default::default()
        };
        let draft = OptimizationDialogState::from_config(&config);
        let json: OptimizationDialogState =
            serde_json::from_str(&serde_json::to_string(&draft).unwrap()).unwrap();
        let ron: OptimizationDialogState = ron::from_str(&ron::to_string(&draft).unwrap()).unwrap();
        assert_eq!(json.to_config().unwrap(), config);
        assert_eq!(ron.to_config().unwrap(), config);
        assert!(config.to_spice().contains("expr={-V(in)*I(V1)}"));
        let mut legacy = serde_json::to_value(&draft).unwrap();
        legacy
            .as_object_mut()
            .unwrap()
            .remove("objective_expression");
        let legacy: OptimizationDialogState = serde_json::from_value(legacy).unwrap();
        assert!(legacy.to_config().unwrap().objective_expression.is_none());
    }

    #[test]
    fn optimization_search_controls_survive_drafts_and_legacy_loading() {
        let mut config = OptimizationConfig::default();
        config.search = OptimizationSearchControls {
            var_tolerance: 7.123456789e-8,
            sa_initial_temp: 12.25,
            sa_cooling_rate: 0.875,
            random_seed: u64::MAX,
        };
        let draft = OptimizationDialogState::from_config(&config);
        let json = serde_json::to_string(&draft).unwrap();
        let restored: OptimizationDialogState = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.to_config().unwrap(), config);
        let mut old = serde_json::to_value(&draft).unwrap();
        for name in [
            "var_tolerance",
            "sa_initial_temp",
            "sa_cooling_rate",
            "random_seed",
        ] {
            old.as_object_mut().unwrap().remove(name);
        }
        let legacy: OptimizationDialogState = serde_json::from_value(old).unwrap();
        assert_eq!(
            legacy.to_config().unwrap().search,
            OptimizationSearchControls::default()
        );
    }
}
