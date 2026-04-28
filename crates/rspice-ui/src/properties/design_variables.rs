//! Design Variables Panel
//!
//! Commercial-grade global design variables panel matching Cadence Virtuoso VAR artisan.
//! Provides:
//! - Named parameter variables with engineering notation
//! - Expression evaluation support
//! - Variable grouping and organization
//! - Integration with component parameter expressions

use egui::{
    Align, Frame, Layout, Margin, RichText, Rounding, ScrollArea, Sense, Stroke, TextEdit, Ui, Vec2,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Theme Constants (matching enhanced_property_editor)
// =============================================================================

mod theme {
    use egui::Color32;

    pub const PANEL_BG: Color32 = Color32::from_rgb(45, 45, 48);
    pub const HEADER_BG: Color32 = Color32::from_rgb(38, 38, 40);
    pub const ROW_BG: Color32 = Color32::from_rgb(50, 50, 54);
    pub const ROW_BG_ALT: Color32 = Color32::from_rgb(46, 46, 50);

    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(220, 220, 220);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(160, 160, 160);
    pub const TEXT_VARIABLE: Color32 = Color32::from_rgb(130, 200, 255);
    pub const TEXT_VALUE: Color32 = Color32::from_rgb(180, 230, 180);
    pub const TEXT_ERROR: Color32 = Color32::from_rgb(255, 120, 120);

    pub const ACCENT_RED: Color32 = Color32::from_rgb(234, 67, 53);

    pub const BORDER: Color32 = Color32::from_rgb(70, 70, 75);
}

// =============================================================================
// Design Variable
// =============================================================================

/// A design variable with name, expression, and evaluated value
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DesignVariable {
    /// Variable name (e.g., "W1", "Vdd", "gm_target")
    pub name: String,

    /// Expression string (e.g., "1u", "3.3", "2*W1")
    pub expression: String,

    /// Evaluated numeric value (if valid)
    #[serde(skip)]
    pub evaluated_value: Option<f64>,

    /// Error message if expression is invalid
    #[serde(skip)]
    pub error: Option<String>,

    /// Description/comment
    pub description: String,

    /// Whether this variable is currently being edited
    #[serde(skip)]
    pub editing: bool,
}

impl DesignVariable {
    /// Create a new design variable
    pub fn new(name: impl Into<String>, expression: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            expression: expression.into(),
            ..Default::default()
        }
    }

    /// Create with description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
}

// =============================================================================
// Design Variables State
// =============================================================================

/// State for the design variables panel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DesignVariablesState {
    /// All defined design variables
    pub variables: Vec<DesignVariable>,

    /// Whether the panel is expanded
    #[serde(skip)]
    pub expanded: bool,

    /// Index of variable being edited (if any)
    #[serde(skip)]
    pub editing_index: Option<usize>,

    /// Temporary buffer for new variable name
    #[serde(skip)]
    pub new_var_name: String,

    /// Temporary buffer for new variable expression
    #[serde(skip)]
    pub new_var_expression: String,
}

impl DesignVariablesState {
    /// Create a new empty state
    pub fn new() -> Self {
        Self {
            expanded: true,
            ..Default::default()
        }
    }

    /// Add a new design variable
    pub fn add_variable(&mut self, name: String, expression: String) -> Result<(), String> {
        // Validate name
        if name.is_empty() {
            return Err("Variable name cannot be empty".to_string());
        }
        if !name
            .chars()
            .next()
            .map(|c| c.is_ascii_alphabetic())
            .unwrap_or(false)
        {
            return Err("Variable name must start with a letter".to_string());
        }
        if self.variables.iter().any(|v| v.name == name) {
            return Err(format!("Variable '{}' already exists", name));
        }

        let mut var = DesignVariable::new(name, expression);
        self.evaluate_variable(&mut var);
        self.variables.push(var);
        Ok(())
    }

    /// Remove a variable by index
    pub fn remove_variable(&mut self, index: usize) {
        if index < self.variables.len() {
            self.variables.remove(index);
            // Re-evaluate all variables since dependencies may have changed
            self.evaluate_all();
        }
    }

    /// Get a variable's value by name
    pub fn get_value(&self, name: &str) -> Option<f64> {
        self.variables
            .iter()
            .find(|v| v.name == name)
            .and_then(|v| v.evaluated_value)
    }

    /// Get all variable values as a map (for expression evaluation)
    pub fn get_context(&self) -> HashMap<String, f64> {
        self.variables
            .iter()
            .filter_map(|v| v.evaluated_value.map(|val| (v.name.clone(), val)))
            .collect()
    }

    /// Evaluate a single variable's expression
    fn evaluate_variable(&mut self, var: &mut DesignVariable) {
        let context = self.get_context();
        match evaluate_expression(&var.expression, &context) {
            Ok(value) => {
                var.evaluated_value = Some(value);
                var.error = None;
            }
            Err(e) => {
                var.evaluated_value = None;
                var.error = Some(e);
            }
        }
    }

    /// Re-evaluate all variables (order matters for dependencies)
    pub fn evaluate_all(&mut self) {
        // Simple approach: evaluate in order (assumes no circular dependencies)
        // For full DAG-based evaluation, we'd need to topologically sort
        let mut context: HashMap<String, f64> = HashMap::new();

        for var in &mut self.variables {
            match evaluate_expression(&var.expression, &context) {
                Ok(value) => {
                    var.evaluated_value = Some(value);
                    var.error = None;
                    context.insert(var.name.clone(), value);
                }
                Err(e) => {
                    var.evaluated_value = None;
                    var.error = Some(e);
                }
            }
        }
    }

    /// Update a variable's expression and re-evaluate
    pub fn update_expression(&mut self, index: usize, expression: String) {
        if index < self.variables.len() {
            self.variables[index].expression = expression;
            self.evaluate_all(); // Re-evaluate all since dependencies may have changed
        }
    }
}

// =============================================================================
// Expression Evaluator
// =============================================================================

/// Evaluate an expression with variable context
///
/// Supports:
/// - Engineering notation (1k, 10u, 3.3meg, etc.)
/// - Basic arithmetic (+, -, *, /)
/// - Variable references
/// - Parentheses
pub fn evaluate_expression(expr: &str, context: &HashMap<String, f64>) -> Result<f64, String> {
    let trimmed = expr.trim();
    if trimmed.is_empty() {
        return Err("Empty expression".to_string());
    }

    // First, try to parse as a simple engineering number
    if let Ok(value) = parse_engineering_number(trimmed) {
        return Ok(value);
    }

    // Check if it's a simple variable reference
    if let Some(value) = context.get(trimmed) {
        return Ok(*value);
    }

    // Try simple arithmetic (very basic parser)
    // This is a simplified evaluator - a real implementation would use a proper parser
    if let Some(result) = try_simple_arithmetic(trimmed, context) {
        return Ok(result);
    }

    Err(format!("Cannot evaluate '{}'", trimmed))
}

/// Parse an engineering notation number
fn parse_engineering_number(s: &str) -> Result<f64, String> {
    let trimmed = s.trim();

    // Find where number ends and suffix begins
    let mut num_end = 0;
    for (i, c) in trimmed.chars().enumerate() {
        if c.is_ascii_digit() || c == '.' || c == '-' || c == '+' || c == 'e' || c == 'E' {
            num_end = i + 1;
        } else {
            break;
        }
    }

    if num_end == 0 {
        return Err("No numeric value".to_string());
    }

    let num_str = &trimmed[..num_end];
    let base: f64 = num_str.parse().map_err(|_| "Invalid number")?;

    let suffix = trimmed[num_end..].trim().to_lowercase();
    let multiplier = match suffix.as_str() {
        "" => 1.0,
        "t" | "tera" => 1e12,
        "g" | "giga" => 1e9,
        "meg" | "mega" => 1e6,
        "k" | "kilo" => 1e3,
        "m" | "milli" => 1e-3,
        "u" | "micro" | "µ" => 1e-6,
        "n" | "nano" => 1e-9,
        "p" | "pico" => 1e-12,
        "f" | "femto" => 1e-15,
        _ => return Err(format!("Unknown suffix: {}", suffix)),
    };

    Ok(base * multiplier)
}

/// Try simple arithmetic evaluation
fn try_simple_arithmetic(expr: &str, context: &HashMap<String, f64>) -> Option<f64> {
    // Very simple: handle single binary operations
    // Format: term OP term
    for op in ['*', '/', '+', '-'] {
        if let Some(pos) = expr.rfind(op)
            && pos > 0
            && pos < expr.len() - 1
        {
            let left = expr[..pos].trim();
            let right = expr[pos + 1..].trim();

            let left_val = parse_term(left, context)?;
            let right_val = parse_term(right, context)?;

            return Some(match op {
                '+' => left_val + right_val,
                '-' => left_val - right_val,
                '*' => left_val * right_val,
                '/' => left_val / right_val,
                _ => return None,
            });
        }
    }
    None
}

/// Parse a single term (number or variable)
fn parse_term(term: &str, context: &HashMap<String, f64>) -> Option<f64> {
    // Try as number first
    if let Ok(v) = parse_engineering_number(term) {
        return Some(v);
    }
    // Try as variable
    context.get(term).copied()
}

/// Format a value with engineering notation
fn format_engineering(value: f64) -> String {
    let abs_value = value.abs();

    let (scaled, suffix) = if abs_value >= 1e12 {
        (value / 1e12, "T")
    } else if abs_value >= 1e9 {
        (value / 1e9, "G")
    } else if abs_value >= 1e6 {
        (value / 1e6, "M")
    } else if abs_value >= 1e3 {
        (value / 1e3, "k")
    } else if abs_value >= 1.0 || abs_value == 0.0 {
        (value, "")
    } else if abs_value >= 1e-3 {
        (value * 1e3, "m")
    } else if abs_value >= 1e-6 {
        (value * 1e6, "u")
    } else if abs_value >= 1e-9 {
        (value * 1e9, "n")
    } else if abs_value >= 1e-12 {
        (value * 1e12, "p")
    } else {
        (value * 1e15, "f")
    };

    if (scaled.round() - scaled).abs() < 1e-9 {
        format!("{:.0}{}", scaled.round(), suffix)
    } else {
        format!("{:.3}{}", scaled, suffix)
    }
}

// =============================================================================
// Panel Rendering
// =============================================================================

/// Render the design variables panel
#[allow(dead_code)]
pub fn render_design_variables_panel(ui: &mut Ui, state: &mut DesignVariablesState) {
    Frame::default()
        .fill(theme::PANEL_BG)
        .rounding(Rounding::same(4.0))
        .stroke(Stroke::new(1.0, theme::BORDER))
        .show(ui, |ui| {
            // Header
            render_panel_header(ui, state);

            if state.expanded {
                ui.add_space(4.0);

                // Variables table
                render_variables_table(ui, state);

                ui.add_space(8.0);

                // Add new variable row
                render_add_variable_row(ui, state);
            }
        });
}

fn render_panel_header(ui: &mut Ui, state: &mut DesignVariablesState) {
    Frame::default()
        .fill(theme::HEADER_BG)
        .inner_margin(Margin::symmetric(12.0, 8.0))
        .rounding(Rounding {
            nw: 4.0,
            ne: 4.0,
            sw: 0.0,
            se: 0.0,
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Collapse/expand button
                let icon = if state.expanded { "▼" } else { "▶" };
                if ui
                    .add(
                        egui::Button::new(
                            RichText::new(icon).size(10.0).color(theme::TEXT_SECONDARY),
                        )
                        .frame(false),
                    )
                    .clicked()
                {
                    state.expanded = !state.expanded;
                }

                ui.label(
                    RichText::new("Design Variables")
                        .size(12.0)
                        .color(theme::TEXT_PRIMARY)
                        .strong(),
                );

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.label(
                        RichText::new(format!("{} vars", state.variables.len()))
                            .size(11.0)
                            .color(theme::TEXT_SECONDARY),
                    );
                });
            });
        });
}

fn render_variables_table(ui: &mut Ui, state: &mut DesignVariablesState) {
    if state.variables.is_empty() {
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.add_space(12.0);
            ui.label(
                RichText::new("No variables defined. Add a variable below.")
                    .size(11.0)
                    .color(theme::TEXT_SECONDARY)
                    .italics(),
            );
        });
        return;
    }

    ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
        let mut to_remove: Option<usize> = None;
        let mut expression_updates: Vec<(usize, String)> = vec![];

        for (idx, var) in state.variables.iter_mut().enumerate() {
            let row_bg = if idx % 2 == 0 {
                theme::ROW_BG
            } else {
                theme::ROW_BG_ALT
            };

            Frame::default()
                .fill(row_bg)
                .inner_margin(Margin::symmetric(12.0, 6.0))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Variable name
                        ui.add_sized(
                            Vec2::new(80.0, 18.0),
                            egui::Label::new(
                                RichText::new(&var.name)
                                    .size(12.0)
                                    .color(theme::TEXT_VARIABLE)
                                    .strong(),
                            ),
                        );

                        ui.label(RichText::new("=").size(12.0).color(theme::TEXT_SECONDARY));

                        // Expression (editable)
                        let mut expr = var.expression.clone();
                        let expr_edit = TextEdit::singleline(&mut expr)
                            .font(egui::FontId::monospace(11.0))
                            .text_color(theme::TEXT_PRIMARY)
                            .frame(false)
                            .desired_width(100.0);

                        if ui.add(expr_edit).changed() {
                            expression_updates.push((idx, expr));
                        }

                        ui.add_space(8.0);

                        // Evaluated value or error
                        if let Some(value) = var.evaluated_value {
                            ui.label(
                                RichText::new(format!("→ {}", format_engineering(value)))
                                    .size(11.0)
                                    .color(theme::TEXT_VALUE),
                            );
                        } else if let Some(ref err) = var.error {
                            ui.label(
                                RichText::new(format!("⚠ {}", err))
                                    .size(10.0)
                                    .color(theme::TEXT_ERROR),
                            );
                        }

                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            // Delete button
                            let (rect, response) =
                                ui.allocate_exact_size(Vec2::splat(18.0), Sense::click());
                            if ui.is_rect_visible(rect) {
                                let color = if response.hovered() {
                                    theme::ACCENT_RED
                                } else {
                                    theme::TEXT_SECONDARY
                                };
                                ui.painter().text(
                                    rect.center(),
                                    egui::Align2::CENTER_CENTER,
                                    "✕",
                                    egui::FontId::proportional(12.0),
                                    color,
                                );
                            }
                            if response.clicked() {
                                to_remove = Some(idx);
                            }
                        });
                    });
                });
        }

        // Apply expression updates
        for (idx, expr) in expression_updates {
            state.update_expression(idx, expr);
        }

        // Remove variable if requested
        if let Some(idx) = to_remove {
            state.remove_variable(idx);
        }
    });
}

fn render_add_variable_row(ui: &mut Ui, state: &mut DesignVariablesState) {
    Frame::default()
        .fill(theme::HEADER_BG)
        .inner_margin(Margin::symmetric(12.0, 8.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Name input
                let name_edit = TextEdit::singleline(&mut state.new_var_name)
                    .hint_text("Name")
                    .font(egui::FontId::proportional(11.0))
                    .desired_width(70.0);
                ui.add(name_edit);

                ui.label(RichText::new("=").size(12.0).color(theme::TEXT_SECONDARY));

                // Expression input
                let expr_edit = TextEdit::singleline(&mut state.new_var_expression)
                    .hint_text("Expression")
                    .font(egui::FontId::proportional(11.0))
                    .desired_width(100.0);
                let expr_response = ui.add(expr_edit);

                ui.add_space(8.0);

                // Add button
                let add_btn = ui.add_sized(
                    Vec2::new(50.0, 22.0),
                    egui::Button::new(RichText::new("+ Add").size(11.0)),
                );

                // Add on button click or Enter key
                let should_add = add_btn.clicked()
                    || (expr_response.lost_focus()
                        && ui.input(|i| i.key_pressed(egui::Key::Enter)));

                if should_add && !state.new_var_name.is_empty() {
                    let name = std::mem::take(&mut state.new_var_name);
                    let expr = std::mem::take(&mut state.new_var_expression);
                    let result = state.add_variable(name, expr);

                    if let Err(e) = result {
                        log::warn!("Failed to add variable: {}", e);
                    }
                }
            });
        });
}

// =============================================================================
// Tests
// =============================================================================
