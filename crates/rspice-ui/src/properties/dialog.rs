//! Component Properties Dialog
//!
//! Commercial-grade component property editing dialog matching Cadence Virtuoso behavior.
//! Provides editing for:
//! - Component name (instance name)
//! - Component value (resistance, capacitance, etc.)
//! - Model reference (for semiconductor devices)
//! - Additional SPICE parameters

// =============================================================================
// Property Editor State
// =============================================================================

/// State for the component properties dialog
#[derive(Debug, Clone, Default)]
pub struct PropertyEditorState {
    /// Whether the dialog is currently open
    pub open: bool,

    /// ID of the component being edited (None if creating new)
    pub component_id: Option<u64>,

    /// Current edited values
    pub edited: EditedProperties,

    /// Original values (for cancel/revert)
    pub original: EditedProperties,

    /// Validation error message (if any)
    pub error_message: Option<String>,
}

/// Edited property values
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EditedProperties {
    /// Instance name (e.g., "R1", "M1")
    pub name: String,

    /// Value string (e.g., "1k", "10u", "1p")
    pub value: String,

    /// Model name for semiconductor devices
    pub model: String,

    /// Additional parameters as key-value pairs
    pub parameters: Vec<(String, String)>,
}

impl PropertyEditorState {
    /// Create a new property editor state
    pub fn new() -> Self {
        Self::default()
    }

    /// Open the dialog for editing an existing component
    pub fn open_for(&mut self, component_id: u64, props: EditedProperties) {
        self.open = true;
        self.component_id = Some(component_id);
        self.edited = props.clone();
        self.original = props;
        self.error_message = None;
    }

    /// Close the dialog without applying changes
    pub fn cancel(&mut self) {
        self.open = false;
        self.component_id = None;
        self.error_message = None;
    }

    /// Check if there are unsaved changes
    pub fn has_changes(&self) -> bool {
        self.edited != self.original
    }

    /// Validate the current properties
    pub fn validate(&mut self) -> bool {
        // Name validation
        if self.edited.name.is_empty() {
            self.error_message = Some("Instance name cannot be empty".to_string());
            return false;
        }

        // Name must start with a letter
        if !self
            .edited
            .name
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic())
        {
            self.error_message = Some("Instance name must start with a letter".to_string());
            return false;
        }

        // Value validation (if provided)
        if !self.edited.value.is_empty()
            && let Err(e) = parse_engineering_value(&self.edited.value)
        {
            self.error_message = Some(format!("Invalid value: {}", e));
            return false;
        }

        self.error_message = None;
        true
    }

    /// Revert to original values
    pub fn revert(&mut self) {
        self.edited = self.original.clone();
        self.error_message = None;
    }
}

// =============================================================================
// Engineering Value Parser
// =============================================================================

/// Parse an engineering notation value (e.g., "1k", "10u", "3.3meg")
///
/// Returns the numeric value or an error message.
pub fn parse_engineering_value(input: &str) -> Result<f64, String> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err("Empty value".to_string());
    }

    // Find where the number ends and the suffix begins
    let mut numeric_end = 0;
    let chars: Vec<char> = trimmed.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if c.is_ascii_digit() || *c == '.' || *c == '-' || *c == '+' || *c == 'e' || *c == 'E' {
            numeric_end = i + 1;
        } else {
            break;
        }
    }

    // Parse the numeric part
    let numeric_str = &trimmed[..trimmed.chars().take(numeric_end).collect::<String>().len()];
    let base_value: f64 = numeric_str
        .parse()
        .map_err(|_| format!("Cannot parse '{}' as number", numeric_str))?;

    // Parse the suffix - take the rest and compare lowercase/normalized
    let suffix: String = trimmed.chars().skip(numeric_end).collect();
    let suffix = suffix.trim();
    let suffix_lower = suffix.to_lowercase();

    // Normalize micro sign: U+00B5 (µ MICRO SIGN) and U+03BC (μ GREEK SMALL LETTER MU) both map to 'u'
    // Using explicit unicode escape sequences for reliability
    let normalized_suffix = suffix_lower.replace(['\u{00B5}', '\u{03BC}'], "u"); // GREEK SMALL LETTER MU

    let multiplier = match normalized_suffix.as_str() {
        "" => 1.0,
        "t" | "tera" => 1e12,
        "g" | "giga" => 1e9,
        "meg" | "mega" => 1e6,
        "k" | "kilo" => 1e3,
        "m" | "milli" => 1e-3,
        "u" | "micro" => 1e-6,
        "n" | "nano" => 1e-9,
        "p" | "pico" => 1e-12,
        "f" | "femto" => 1e-15,
        "a" | "atto" => 1e-18,
        _ => {
            return Err(format!(
                "Unknown suffix: '{}' (normalized: '{}')",
                suffix, normalized_suffix
            ));
        }
    };

    Ok(base_value * multiplier)
}

/// Format a value with engineering notation
pub fn format_engineering_value(value: f64) -> String {
    let abs_value = value.abs();

    // Special case: zero should never have a suffix
    if abs_value == 0.0 {
        return "0".to_string();
    }

    let (scaled, suffix) = if abs_value >= 1e12 {
        (value / 1e12, "T")
    } else if abs_value >= 1e9 {
        (value / 1e9, "G")
    } else if abs_value >= 1e6 {
        (value / 1e6, "Meg")
    } else if abs_value >= 1e3 {
        (value / 1e3, "k")
    } else if abs_value >= 1.0 {
        (value, "")
    } else if abs_value >= 1e-3 {
        (value * 1e3, "m")
    } else if abs_value >= 1e-6 {
        (value * 1e6, "u")
    } else if abs_value >= 1e-9 {
        (value * 1e9, "n")
    } else if abs_value >= 1e-12 {
        (value * 1e12, "p")
    } else if abs_value >= 1e-15 {
        (value * 1e15, "f")
    } else {
        (value * 1e18, "a")
    };

    // Format with appropriate precision, using epsilon for floating-point comparison
    let eps = 1e-9;
    let is_int = (scaled.round() - scaled).abs() < eps;
    let is_one_decimal = ((scaled * 10.0).round() - scaled * 10.0).abs() < eps;
    let is_two_decimal = ((scaled * 100.0).round() - scaled * 100.0).abs() < eps;

    if is_int {
        format!("{:.0}{}", scaled.round(), suffix)
    } else if is_one_decimal {
        format!("{:.1}{}", scaled, suffix)
    } else if is_two_decimal {
        format!("{:.2}{}", scaled, suffix)
    } else {
        format!("{:.3}{}", scaled, suffix)
    }
}

// =============================================================================
// Properties Dialog Result
// =============================================================================

/// Result of the properties dialog
#[derive(Debug, Clone, PartialEq)]
pub enum PropertiesDialogResult {
    /// No action (dialog still open or was cancelled)
    None,
    /// Apply changes
    Apply(u64, EditedProperties),
    /// Cancel without changes
    Cancel,
}

// =============================================================================
// egui Dialog Rendering
// =============================================================================

use egui::{Align, Color32, Layout, RichText, TextEdit, Window};

/// Render the properties dialog
///
/// Returns a PropertiesDialogResult indicating what action the user took.
pub fn render_properties_dialog(
    ctx: &egui::Context,
    state: &mut PropertyEditorState,
) -> PropertiesDialogResult {
    if !state.open {
        return PropertiesDialogResult::None;
    }

    let mut result = PropertiesDialogResult::None;

    Window::new("Component Properties")
        .collapsible(false)
        .resizable(true)
        .default_width(350.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Component ID header
                if let Some(id) = state.component_id {
                    ui.label(RichText::new(format!("Editing Component ID: {}", id)).weak());
                    ui.add_space(8.0);
                }

                // Instance Name
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.add_space(20.0);
                    let name_edit = TextEdit::singleline(&mut state.edited.name)
                        .hint_text("e.g., R1, M1, C2")
                        .desired_width(200.0);
                    ui.add(name_edit);
                });
                ui.add_space(4.0);

                // Value
                ui.horizontal(|ui| {
                    ui.label("Value:");
                    ui.add_space(16.0);
                    let value_edit = TextEdit::singleline(&mut state.edited.value)
                        .hint_text("e.g., 1k, 10u, 3.3meg")
                        .desired_width(200.0);
                    ui.add(value_edit);
                });
                ui.add_space(4.0);

                // Model (for semiconductor devices)
                ui.horizontal(|ui| {
                    ui.label("Model:");
                    ui.add_space(12.0);
                    let model_edit = TextEdit::singleline(&mut state.edited.model)
                        .hint_text("e.g., nmos4, 2N2222")
                        .desired_width(200.0);
                    ui.add(model_edit);
                });

                ui.add_space(16.0);
                ui.separator();

                // Error message
                if let Some(ref error) = state.error_message {
                    ui.add_space(4.0);
                    ui.label(RichText::new(error).color(Color32::RED));
                    ui.add_space(4.0);
                }

                // Buttons
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        // Cancel button
                        if ui.button("Cancel").clicked() {
                            state.cancel();
                            result = PropertiesDialogResult::Cancel;
                        }

                        // Revert button (only if changes)
                        ui.add_enabled_ui(state.has_changes(), |ui| {
                            if ui.button("Revert").clicked() {
                                state.revert();
                            }
                        });

                        // OK button
                        if ui.button("OK").clicked()
                            && state.validate()
                            && let Some(id) = state.component_id
                        {
                            result = PropertiesDialogResult::Apply(id, state.edited.clone());
                            state.open = false;
                            state.component_id = None;
                        }
                    });
                });
            });
        });

    result
}

// =============================================================================
// Tests
// =============================================================================
