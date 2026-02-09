//! Optimization analysis configuration dialog.
//!
//! Provides a typed UI surface for closed-loop parameter optimization.

use super::options::parse_si_value;
use egui::Ui;

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
    /// Variable set to optimize.
    pub variables: Vec<OptimizationVariableConfig>,
    /// Objective node (V(node,ref)).
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
        if self.variables.is_empty() {
            return Err("At least one optimization variable is required".to_string());
        }
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

    /// Serialize to SPICE-like line for logging.
    pub fn to_spice(&self) -> String {
        let vars = self
            .variables
            .iter()
            .map(|v| format!("{}:{:.6e}:{:.6e}:{:.6e}", v.name, v.min, v.max, v.initial))
            .collect::<Vec<_>>()
            .join(",");
        let mut line = format!(
            ".opt algo={} goal={} obj=V({},{}) maxiter={} ctol={:.6e} fd={:.6e} initstep={:.6e} minstep={:.6e} vars={}",
            self.algorithm.as_str(),
            self.goal_mode.as_str(),
            self.objective_node,
            self.objective_ref,
            self.max_iterations,
            self.cost_tolerance,
            self.fd_step,
            self.initial_step,
            self.min_step,
            vars
        );
        if let Some(target) = self.target_value {
            line.push_str(&format!(" target={:.6e}", target));
        }
        line
    }
}

/// UI state for optimization dialog tab.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OptimizationDialogState {
    /// Variables encoded as `name:min:max[:initial]`, separated by newline/comma.
    pub variables_text: String,
    /// Objective node.
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
            variables_text,
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
            variables,
            objective_node: self.objective_node.trim().to_string(),
            objective_ref: self.objective_ref.trim().to_string(),
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

    /// Render optimization options.
    pub fn render(&mut self, ui: &mut Ui) {
        self.ensure_initialized();
        ui.heading("Optimization");
        ui.label(
            egui::RichText::new("Closed-loop parameter tuning using design-variable constraints")
                .weak(),
        );
        ui.add_space(8.0);

        egui::Grid::new("optimization_grid")
            .num_columns(2)
            .spacing([20.0, 6.0])
            .show(ui, |ui| {
                ui.label("Objective Node:");
                ui.add(egui::TextEdit::singleline(&mut self.objective_node).desired_width(140.0));
                ui.end_row();

                ui.label("Objective Ref:");
                ui.add(egui::TextEdit::singleline(&mut self.objective_ref).desired_width(140.0));
                ui.end_row();

                ui.label("Goal:");
                egui::ComboBox::from_id_salt("opt_goal_mode")
                    .selected_text(match self.goal_mode {
                        0 => "Minimize",
                        1 => "Maximize",
                        _ => "Target",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.goal_mode, 0, "Minimize");
                        ui.selectable_value(&mut self.goal_mode, 1, "Maximize");
                        ui.selectable_value(&mut self.goal_mode, 2, "Target");
                    });
                ui.end_row();

                ui.label("Target:");
                let target_enabled = self.goal_mode == 2;
                ui.add_enabled(
                    target_enabled,
                    egui::TextEdit::singleline(&mut self.target_value)
                        .desired_width(140.0)
                        .hint_text("e.g. 1.2"),
                );
                ui.end_row();

                ui.label("Algorithm:");
                egui::ComboBox::from_id_salt("opt_algo")
                    .selected_text(match self.algorithm {
                        0 => "Gradient Descent",
                        1 => "Pattern Search",
                        _ => "Simulated Annealing",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.algorithm, 0, "Gradient Descent");
                        ui.selectable_value(&mut self.algorithm, 1, "Pattern Search");
                        ui.selectable_value(&mut self.algorithm, 2, "Simulated Annealing");
                    });
                ui.end_row();

                ui.label("Max Iterations:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.max_iterations)
                        .desired_width(100.0)
                        .hint_text("100"),
                );
                ui.end_row();

                ui.label("Cost Tolerance:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.cost_tolerance)
                        .desired_width(120.0)
                        .hint_text("1e-8"),
                );
                ui.end_row();

                ui.label("FD Step:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.fd_step)
                        .desired_width(120.0)
                        .hint_text("1e-4"),
                );
                ui.end_row();

                ui.label("Initial Step:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.initial_step)
                        .desired_width(120.0)
                        .hint_text("0.1"),
                );
                ui.end_row();

                ui.label("Minimum Step:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.min_step)
                        .desired_width(120.0)
                        .hint_text("1e-8"),
                );
                ui.end_row();
            });

        ui.add_space(6.0);
        ui.label("Variables (one per line: name:min:max[:initial])");
        ui.add(
            egui::TextEdit::multiline(&mut self.variables_text)
                .desired_rows(6)
                .desired_width(f32::INFINITY),
        );
    }
}

fn format_scalar(v: f64) -> String {
    if v.abs() >= 1e4 || (v.abs() > 0.0 && v.abs() < 1e-3) {
        format!("{:.6e}", v)
    } else {
        format!("{:.6}", v)
    }
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
    for token in input.split(|c: char| c == '\n' || c == ',' || c == ';') {
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
mod tests {
    use super::*;

    #[test]
    fn test_default_config_is_valid() {
        assert!(OptimizationConfig::default().validate().is_ok());
    }

    #[test]
    fn test_parse_variable_specs_supports_optional_initial() {
        let vars = parse_variable_specs("R1:1k:2k:1.5k\nR2:500:1500")
            .expect("variable specs should parse");
        assert_eq!(vars.len(), 2);
        assert!((vars[0].initial - 1500.0).abs() < 1e-12);
        assert!((vars[1].initial - 1000.0).abs() < 1e-12);
    }

    #[test]
    fn test_parse_variable_specs_rejects_invalid_name() {
        let err = parse_variable_specs("1BAD:1:2:1.5").expect_err("invalid name must fail");
        assert!(err.contains("Invalid variable name"));
    }

    #[test]
    fn test_config_requires_target_for_target_mode() {
        let mut cfg = OptimizationConfig::default();
        cfg.goal_mode = OptimizationGoalMode::Target;
        cfg.target_value = None;
        let err = cfg.validate().expect_err("missing target must fail");
        assert!(err.contains("Target mode"));
    }

    #[test]
    fn test_dialog_to_config_round_trip() {
        let state = OptimizationDialogState::from_config(&OptimizationConfig::default());
        let cfg = state.to_config().expect("state should convert to config");
        assert_eq!(cfg.goal_mode, OptimizationGoalMode::Target);
        assert_eq!(cfg.algorithm, OptimizationAlgorithmMode::PatternSearch);
        assert_eq!(cfg.max_iterations, 120);
    }

    #[test]
    fn test_to_spice_contains_key_fields() {
        let cfg = OptimizationConfig::default();
        let line = cfg.to_spice();
        assert!(line.contains(".opt"));
        assert!(line.contains("algo=pattern_search"));
        assert!(line.contains("goal=target"));
        assert!(line.contains("vars="));
    }
}
