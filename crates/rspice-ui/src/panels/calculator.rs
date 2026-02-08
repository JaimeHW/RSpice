//! Calculator Panel
//!
//! Commercial-grade waveform calculator UI.
//! Features:
//! - Expression editor with history
//! - Scientific keypad
//! - Function browser
//! - Signal list (drag & drop target)

use crate::analysis::calculator::{evaluator, parser, CalcValue, SimulationContext};
use crate::state::SimulationState;
use egui::{Button, Color32, RichText, ScrollArea, TextEdit, Ui, Vec2};

#[derive(Default, Clone)]
pub struct CalculatorPanel {
    /// Current expression string
    expression: String,
    /// Expression history
    history: Vec<String>,
    /// Last result (formatted string for scalars)
    last_result: Option<String>,
    /// Error message if last eval failed
    error_msg: Option<String>,
    /// Selected function category in browser
    selected_category: FunctionCategory,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum FunctionCategory {
    #[default]
    Math,
    Signal,
    Measure,
}

impl CalculatorPanel {
    pub fn new() -> Self {
        Self {
            expression: String::new(),
            history: Vec::new(),
            last_result: None,
            error_msg: None,
            selected_category: FunctionCategory::Math,
        }
    }

    /// Show the calculator panel UI
    ///
    /// # Arguments
    /// * `ui` - The egui UI context
    /// * `simulation` - Reference to simulation state for waveform access
    pub fn show(&mut self, ui: &mut Ui, simulation: &SimulationState) {
        ui.vertical(|ui| {
            // 1. Display / Editor Area
            self.show_display_area(ui, simulation);

            ui.separator();

            // 2. Main Controls (Keypad + Browser)
            ui.horizontal(|ui| {
                // Left: Keypad
                ui.vertical(|ui| {
                    self.show_keypad(ui, simulation);
                });

                ui.separator();

                // Right: Function/Signal Browser
                ui.vertical(|ui| {
                    self.show_browser(ui);
                });
            });
        });
    }

    fn show_display_area(&mut self, ui: &mut Ui, simulation: &SimulationState) {
        ui.group(|ui| {
            ui.vertical(|ui| {
                // Expression Editor
                ui.label("Expression:");
                let response = ui.add(
                    TextEdit::multiline(&mut self.expression)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY)
                        .desired_rows(3),
                );

                if response.changed() {
                    self.error_msg = None;
                }

                // Error / Result Display
                if let Some(err) = &self.error_msg {
                    ui.colored_label(Color32::RED, format!("Error: {}", err));
                } else if let Some(res) = &self.last_result {
                    ui.colored_label(Color32::GREEN, format!("Result: {}", res));
                }

                ui.horizontal(|ui| {
                    if ui.button("Evaluate").clicked() {
                        self.evaluate(simulation);
                    }
                    if ui.button("Clear").clicked() {
                        self.expression.clear();
                        self.last_result = None;
                        self.error_msg = None;
                    }
                });
            });
        });
    }

    fn show_keypad(&mut self, ui: &mut Ui, _simulation: &SimulationState) {
        let button_size = Vec2::new(40.0, 30.0);

        // Row 1: Common Ops
        ui.horizontal(|ui| {
            if ui.add(Button::new("+").min_size(button_size)).clicked() {
                self.append(" + ");
            }
            if ui.add(Button::new("-").min_size(button_size)).clicked() {
                self.append(" - ");
            }
            if ui.add(Button::new("*").min_size(button_size)).clicked() {
                self.append(" * ");
            }
            if ui.add(Button::new("/").min_size(button_size)).clicked() {
                self.append(" / ");
            }
        });

        // Row 2: Numbers 7-9
        ui.horizontal(|ui| {
            if ui.add(Button::new("7").min_size(button_size)).clicked() {
                self.append("7");
            }
            if ui.add(Button::new("8").min_size(button_size)).clicked() {
                self.append("8");
            }
            if ui.add(Button::new("9").min_size(button_size)).clicked() {
                self.append("9");
            }
            if ui.add(Button::new("^").min_size(button_size)).clicked() {
                self.append("^");
            }
        });

        // Row 3: Numbers 4-6
        ui.horizontal(|ui| {
            if ui.add(Button::new("4").min_size(button_size)).clicked() {
                self.append("4");
            }
            if ui.add(Button::new("5").min_size(button_size)).clicked() {
                self.append("5");
            }
            if ui.add(Button::new("6").min_size(button_size)).clicked() {
                self.append("6");
            }
            if ui.add(Button::new("(").min_size(button_size)).clicked() {
                self.append("(");
            }
        });

        // Row 4: Numbers 1-3
        ui.horizontal(|ui| {
            if ui.add(Button::new("1").min_size(button_size)).clicked() {
                self.append("1");
            }
            if ui.add(Button::new("2").min_size(button_size)).clicked() {
                self.append("2");
            }
            if ui.add(Button::new("3").min_size(button_size)).clicked() {
                self.append("3");
            }
            if ui.add(Button::new(")").min_size(button_size)).clicked() {
                self.append(")");
            }
        });

        // Row 5: 0 .
        ui.horizontal(|ui| {
            if ui
                .add(Button::new("0").min_size(Vec2::new(88.0, 30.0)))
                .clicked()
            {
                self.append("0");
            }
            if ui.add(Button::new(".").min_size(button_size)).clicked() {
                self.append(".");
            }
            if ui.add(Button::new("Exp").min_size(button_size)).clicked() {
                self.append("e");
            }
        });
    }

    fn show_browser(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.selected_category, FunctionCategory::Math, "Math");
            ui.selectable_value(
                &mut self.selected_category,
                FunctionCategory::Signal,
                "Signal",
            );
            ui.selectable_value(
                &mut self.selected_category,
                FunctionCategory::Measure,
                "Measure",
            );
        });

        ui.separator();

        ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
            match self.selected_category {
                FunctionCategory::Math => {
                    self.func_item(ui, "abs(x)", "Absolute value");
                    self.func_item(ui, "sqrt(x)", "Square root");
                    self.func_item(ui, "log(x)", "Natural logarithm");
                    self.func_item(ui, "log10(x)", "Log base 10");
                    self.func_item(ui, "exp(x)", "Exponential e^x");
                }
                FunctionCategory::Signal => {
                    self.func_item(ui, "deriv(x)", "Derivative");
                    self.func_item(ui, "integ(x)", "Integral");
                    self.func_item(ui, "clip(x, min, max)", "Clip signal");
                }
                FunctionCategory::Measure => {
                    self.func_item(ui, "avg(x)", "Average value");
                    self.func_item(ui, "rms(x)", "RMS value");
                    // Future: rise_time, bandwidth, etc.
                }
            }
        });
    }

    fn func_item(&mut self, ui: &mut Ui, signature: &str, tooltip: &str) {
        let name = signature.split('(').next().unwrap_or(signature);
        if ui.button(signature).on_hover_text(tooltip).clicked() {
            // Logic to insert function with parens and move cursor?
            // Simple append for now
            self.append(&format!("{}(", name));
        }
    }

    fn append(&mut self, text: &str) {
        self.expression.push_str(text);
    }

    /// Evaluate the current expression against simulation data
    ///
    /// Uses the SimulationContext to resolve waveform references like V(out)
    /// and performs full expression evaluation with vector arithmetic.
    fn evaluate(&mut self, simulation: &SimulationState) {
        // Clear previous error
        self.error_msg = None;

        // Parse the expression
        let expr = parser::parse(&self.expression);

        // Create evaluation context from simulation state
        let ctx = SimulationContext::new(simulation);

        // Evaluate
        match evaluator::evaluate(&expr, &ctx) {
            Ok(value) => {
                // Add to history
                if !self.expression.is_empty() && !self.history.contains(&self.expression) {
                    self.history.push(self.expression.clone());
                    // Limit history size
                    if self.history.len() > 50 {
                        self.history.remove(0);
                    }
                }

                // Format result
                self.last_result = Some(match value {
                    CalcValue::Scalar(v) => {
                        // Use engineering notation for large/small values
                        if v.abs() >= 1e6 || (v.abs() < 1e-3 && v != 0.0) {
                            format!("{:.4e}", v)
                        } else {
                            format!("{:.6}", v)
                        }
                    }
                    CalcValue::Waveform(x, y) => {
                        let x_min = x.iter().copied().fold(f64::INFINITY, f64::min);
                        let x_max = x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        let y_min = y.iter().copied().fold(f64::INFINITY, f64::min);
                        let y_max = y.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        format!(
                            "Waveform ({} pts)\nX: [{:.3e}, {:.3e}]\nY: [{:.3e}, {:.3e}]",
                            x.len(),
                            x_min,
                            x_max,
                            y_min,
                            y_max
                        )
                    }
                });
            }
            Err(e) => {
                self.error_msg = Some(format!("Error: {}", e));
                self.last_result = None;
            }
        }
    }
}
