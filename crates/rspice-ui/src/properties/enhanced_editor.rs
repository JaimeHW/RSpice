//! Enhanced Property Editor
//!
//! Commercial-grade component property editing with professional styling,
//! type-aware editors, multi-component batch editing, and expression support.
//! Matches Cadence Virtuoso property editing experience.

use egui::{
    Align, Color32, Frame, Layout, Margin, Response, RichText, Rounding, Sense, Stroke, TextEdit,
    Ui, Vec2, Widget, Window,
};

use crate::state::property_types::{
    PropertyDefinition, PropertyRegistry, PropertySheet, PropertyType, PropertyValue,
};
use crate::state::ComponentType;

// =============================================================================
// Theme Constants for Professional Styling
// =============================================================================

/// Professional color scheme matching commercial EDA tools
mod theme {
    use egui::Color32;

    // Background colors
    pub const DIALOG_BG: Color32 = Color32::from_rgb(45, 45, 48);
    pub const HEADER_BG: Color32 = Color32::from_rgb(38, 38, 40);
    pub const CATEGORY_BG: Color32 = Color32::from_rgb(52, 52, 56);
    pub const INPUT_BG: Color32 = Color32::from_rgb(30, 30, 32);
    pub const INPUT_BG_FOCUSED: Color32 = Color32::from_rgb(35, 35, 38);

    // Text colors
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(220, 220, 220);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(160, 160, 160);
    pub const TEXT_LABEL: Color32 = Color32::from_rgb(180, 180, 185);
    pub const TEXT_UNIT: Color32 = Color32::from_rgb(100, 180, 220);

    // Accent colors
    pub const ACCENT_BLUE: Color32 = Color32::from_rgb(66, 133, 244);
    pub const ACCENT_GREEN: Color32 = Color32::from_rgb(52, 168, 83);
    pub const ACCENT_RED: Color32 = Color32::from_rgb(234, 67, 53);
    pub const ACCENT_ORANGE: Color32 = Color32::from_rgb(251, 188, 4);

    // Border colors
    pub const BORDER_NORMAL: Color32 = Color32::from_rgb(70, 70, 75);
    pub const BORDER_FOCUSED: Color32 = Color32::from_rgb(66, 133, 244);
    pub const BORDER_ERROR: Color32 = Color32::from_rgb(234, 67, 53);

    // Button colors
    pub const BUTTON_PRIMARY_BG: Color32 = Color32::from_rgb(66, 133, 244);
    pub const BUTTON_PRIMARY_HOVER: Color32 = Color32::from_rgb(90, 150, 250);
    pub const BUTTON_SECONDARY_BG: Color32 = Color32::from_rgb(60, 60, 65);
    pub const BUTTON_SECONDARY_HOVER: Color32 = Color32::from_rgb(75, 75, 80);
}

// =============================================================================
// Enhanced Property Editor State
// =============================================================================

/// State for the enhanced property editor dialog
#[derive(Debug, Clone, Default)]
pub struct EnhancedPropertyEditorState {
    /// Whether the dialog is currently open
    pub open: bool,

    /// IDs of components being edited (supports multi-select)
    pub component_ids: Vec<u64>,

    /// Component type (used to fetch property sheet)
    pub component_type: Option<ComponentType>,

    /// Current edited values as strings (keyed by property name)
    pub edited_values: std::collections::HashMap<String, String>,

    /// Original values for revert functionality
    pub original_values: std::collections::HashMap<String, String>,

    /// Validation errors for each property
    pub validation_errors: std::collections::HashMap<String, String>,

    /// Currently focused field
    pub focused_field: Option<String>,

    /// Collapsed categories
    pub collapsed_categories: std::collections::HashSet<String>,

    /// Whether the editor is in expression mode
    pub expression_mode_enabled: bool,

    /// Component name for display
    pub display_name: String,
}

impl EnhancedPropertyEditorState {
    /// Create a new property editor state
    pub fn new() -> Self {
        Self::default()
    }

    /// Open the dialog for a single component
    pub fn open_for_component(
        &mut self,
        component_id: u64,
        component_type: ComponentType,
        name: String,
        value: String,
        model: String,
    ) {
        self.open = true;
        self.component_ids = vec![component_id];
        self.component_type = Some(component_type);
        self.display_name = format!("{} - {}", component_type.display_name(), name);

        // Initialize edited values from component
        self.edited_values.clear();
        self.original_values.clear();
        self.validation_errors.clear();

        // Set core properties
        self.edited_values.insert("name".to_string(), name.clone());
        self.original_values.insert("name".to_string(), name);

        if !value.is_empty() {
            self.edited_values
                .insert("value".to_string(), value.clone());
            self.original_values.insert("value".to_string(), value);
        }

        if !model.is_empty() {
            self.edited_values
                .insert("model".to_string(), model.clone());
            self.original_values.insert("model".to_string(), model);
        }
    }

    /// Close the dialog
    pub fn close(&mut self) {
        self.open = false;
        self.component_ids.clear();
        self.component_type = None;
        self.edited_values.clear();
        self.original_values.clear();
        self.validation_errors.clear();
    }

    /// Check if there are unsaved changes
    pub fn has_changes(&self) -> bool {
        self.edited_values != self.original_values
    }

    /// Revert all changes
    pub fn revert(&mut self) {
        self.edited_values = self.original_values.clone();
        self.validation_errors.clear();
    }

    /// Get the edited value for a property
    pub fn get_value(&self, name: &str) -> Option<&String> {
        self.edited_values.get(name)
    }

    /// Set the edited value for a property
    pub fn set_value(&mut self, name: String, value: String) {
        self.edited_values.insert(name, value);
    }
}

// =============================================================================
// Dialog Result
// =============================================================================

/// Result of the enhanced property editor dialog
#[derive(Debug, Clone)]
pub enum EnhancedPropertyEditorResult {
    /// No action (dialog still open)
    None,

    /// Apply changes to the components
    Apply {
        component_ids: Vec<u64>,
        name: String,
        value: String,
        model: String,
    },

    /// Cancel without changes
    Cancel,
}

// =============================================================================
// Dialog Rendering
// =============================================================================

/// Render the enhanced property editor dialog
#[allow(dead_code)]
pub fn render_enhanced_property_editor(
    ctx: &egui::Context,
    state: &mut EnhancedPropertyEditorState,
    _registry: &PropertyRegistry,
) -> EnhancedPropertyEditorResult {
    if !state.open {
        return EnhancedPropertyEditorResult::None;
    }

    let mut result = EnhancedPropertyEditorResult::None;

    // Professional-looking dialog
    Window::new("Component Properties")
        .collapsible(false)
        .resizable(true)
        .default_width(420.0)
        .min_width(360.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .frame(Frame {
            inner_margin: Margin::same(0.0),
            outer_margin: Margin::same(0.0),
            rounding: Rounding::same(8.0),
            shadow: egui::epaint::Shadow {
                offset: Vec2::new(0.0, 4.0),
                blur: 16.0,
                spread: 4.0,
                color: Color32::from_black_alpha(80),
            },
            fill: theme::DIALOG_BG,
            stroke: Stroke::NONE,
        })
        .show(ctx, |ui| {
            // Header section with component info
            render_dialog_header(ui, state);

            ui.add_space(1.0);

            // Properties section
            Frame::default()
                .inner_margin(Margin::symmetric(16.0, 12.0))
                .show(ui, |ui| {
                    render_property_fields(ui, state);
                });

            // Validation errors section
            if !state.validation_errors.is_empty() {
                ui.add_space(4.0);
                render_validation_errors(ui, state);
            }

            ui.add_space(8.0);

            // Footer with buttons
            let button_result = render_dialog_footer(ui, state);
            if let Some(r) = button_result {
                result = r;
            }
        });

    result
}

/// Render the dialog header with component type and name
fn render_dialog_header(ui: &mut Ui, state: &EnhancedPropertyEditorState) {
    Frame::default()
        .fill(theme::HEADER_BG)
        .inner_margin(Margin::symmetric(16.0, 12.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Component icon placeholder (could use actual icons)
                let icon = match state.component_type {
                    Some(ComponentType::Resistor) => "⎯⊏⊐⎯",
                    Some(ComponentType::Capacitor) => "⎯⫾⎯",
                    Some(ComponentType::Inductor) => "⎯∿⎯",
                    Some(ComponentType::VoltageSource) => "⎓⚡",
                    Some(ComponentType::CurrentSource) => "⎓→",
                    Some(ComponentType::Nmos) => "▷M",
                    Some(ComponentType::Pmos) => "◁M",
                    Some(ComponentType::Diode) => "⎯▷|",
                    Some(ComponentType::NpnBjt) => "Q⤒",
                    Some(ComponentType::PnpBjt) => "Q⤓",
                    _ => "📦",
                };

                ui.label(RichText::new(icon).size(18.0).color(theme::ACCENT_BLUE));
                ui.add_space(8.0);

                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(&state.display_name)
                            .size(14.0)
                            .color(theme::TEXT_PRIMARY)
                            .strong(),
                    );

                    if state.component_ids.len() > 1 {
                        ui.label(
                            RichText::new(format!(
                                "{} components selected",
                                state.component_ids.len()
                            ))
                            .size(11.0)
                            .color(theme::TEXT_SECONDARY),
                        );
                    }
                });
            });
        });
}

/// Render the property input fields
fn render_property_fields(ui: &mut Ui, state: &mut EnhancedPropertyEditorState) {
    let label_width = 100.0;
    let has_model = state
        .component_type
        .as_ref()
        .map(|t| t.is_semiconductor())
        .unwrap_or(false);

    // Instance Name
    ui.horizontal(|ui| {
        render_field_label(ui, "Name", label_width, true);
        let name = state.edited_values.entry("name".to_string()).or_default();
        let response = render_text_input(
            ui,
            name,
            "Instance name (e.g., R1, M1)",
            state.validation_errors.get("name"),
        );
        if response.changed() {
            state.validation_errors.remove("name");
        }
    });

    ui.add_space(8.0);

    // Value (for applicable components)
    let shows_value = state
        .component_type
        .as_ref()
        .map(|t| t.is_passive() || t.is_source())
        .unwrap_or(true);

    if shows_value {
        ui.horizontal(|ui| {
            render_field_label(ui, "Value", label_width, false);
            let value = state.edited_values.entry("value".to_string()).or_default();
            let unit = match state.component_type {
                Some(ComponentType::Resistor) => Some("Ω"),
                Some(ComponentType::Capacitor) => Some("F"),
                Some(ComponentType::Inductor) => Some("H"),
                Some(ComponentType::VoltageSource) | Some(ComponentType::VoltageSourceAc) => {
                    Some("V")
                }
                Some(ComponentType::CurrentSource) => Some("A"),
                _ => None,
            };

            let response = render_value_input_with_unit(
                ui,
                value,
                "e.g., 1k, 10u, 3.3meg",
                unit,
                state.validation_errors.get("value"),
            );
            if response.changed() {
                state.validation_errors.remove("value");
            }
        });

        ui.add_space(8.0);
    }

    // Model (for semiconductors)
    if has_model {
        ui.horizontal(|ui| {
            render_field_label(ui, "Model", label_width, false);
            let model = state.edited_values.entry("model".to_string()).or_default();
            render_text_input(
                ui,
                model,
                "Model name",
                state.validation_errors.get("model"),
            );
        });
    }
}

/// Render a field label with consistent styling
fn render_field_label(ui: &mut Ui, text: &str, width: f32, required: bool) {
    ui.allocate_ui(Vec2::new(width, 20.0), |ui| {
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            ui.add_space(8.0);
            if required {
                ui.label(RichText::new("*").size(12.0).color(theme::ACCENT_RED));
            }
            ui.label(RichText::new(text).size(12.0).color(theme::TEXT_LABEL));
        });
    });
}

/// Render a styled text input
fn render_text_input(
    ui: &mut Ui,
    value: &mut String,
    hint: &str,
    error: Option<&String>,
) -> Response {
    let border_color = if error.is_some() {
        theme::BORDER_ERROR
    } else {
        theme::BORDER_NORMAL
    };

    Frame::default()
        .fill(theme::INPUT_BG)
        .rounding(Rounding::same(4.0))
        .stroke(Stroke::new(1.0, border_color))
        .inner_margin(Margin::symmetric(8.0, 4.0))
        .show(ui, |ui| {
            let edit = TextEdit::singleline(value)
                .hint_text(RichText::new(hint).color(Color32::from_rgb(100, 100, 105)))
                .text_color(theme::TEXT_PRIMARY)
                .frame(false)
                .desired_width(ui.available_width());
            ui.add(edit)
        })
        .inner
}

/// Render a value input with unit suffix
fn render_value_input_with_unit(
    ui: &mut Ui,
    value: &mut String,
    hint: &str,
    unit: Option<&str>,
    error: Option<&String>,
) -> Response {
    let border_color = if error.is_some() {
        theme::BORDER_ERROR
    } else {
        theme::BORDER_NORMAL
    };

    Frame::default()
        .fill(theme::INPUT_BG)
        .rounding(Rounding::same(4.0))
        .stroke(Stroke::new(1.0, border_color))
        .inner_margin(Margin::symmetric(8.0, 4.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let available = ui.available_width() - if unit.is_some() { 30.0 } else { 0.0 };
                let edit = TextEdit::singleline(value)
                    .hint_text(RichText::new(hint).color(Color32::from_rgb(100, 100, 105)))
                    .text_color(theme::TEXT_PRIMARY)
                    .frame(false)
                    .desired_width(available);
                let response = ui.add(edit);

                if let Some(u) = unit {
                    ui.label(RichText::new(u).size(12.0).color(theme::TEXT_UNIT));
                }

                response
            })
            .inner
        })
        .inner
}

/// Render validation errors
fn render_validation_errors(ui: &mut Ui, state: &EnhancedPropertyEditorState) {
    Frame::default()
        .fill(Color32::from_rgb(50, 30, 30))
        .rounding(Rounding::same(4.0))
        .inner_margin(Margin::symmetric(12.0, 8.0))
        .show(ui, |ui| {
            for error in state.validation_errors.values() {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("⚠").color(theme::ACCENT_RED));
                    ui.label(RichText::new(error).size(11.0).color(theme::ACCENT_RED));
                });
            }
        });
}

/// Render the dialog footer with action buttons
fn render_dialog_footer(
    ui: &mut Ui,
    state: &mut EnhancedPropertyEditorState,
) -> Option<EnhancedPropertyEditorResult> {
    let mut result = None;

    Frame::default()
        .fill(theme::HEADER_BG)
        .inner_margin(Margin::symmetric(16.0, 12.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Revert button (left side)
                ui.add_enabled_ui(state.has_changes(), |ui| {
                    if styled_button(
                        ui,
                        "Revert",
                        theme::BUTTON_SECONDARY_BG,
                        theme::BUTTON_SECONDARY_HOVER,
                    )
                    .clicked()
                    {
                        state.revert();
                    }
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    // Cancel button
                    if styled_button(
                        ui,
                        "Cancel",
                        theme::BUTTON_SECONDARY_BG,
                        theme::BUTTON_SECONDARY_HOVER,
                    )
                    .clicked()
                    {
                        state.close();
                        result = Some(EnhancedPropertyEditorResult::Cancel);
                    }

                    ui.add_space(8.0);

                    // OK button
                    if styled_button(
                        ui,
                        "OK",
                        theme::BUTTON_PRIMARY_BG,
                        theme::BUTTON_PRIMARY_HOVER,
                    )
                    .clicked()
                    {
                        // Validate before applying
                        let valid = validate_properties(state);
                        if valid {
                            result = Some(EnhancedPropertyEditorResult::Apply {
                                component_ids: state.component_ids.clone(),
                                name: state.edited_values.get("name").cloned().unwrap_or_default(),
                                value: state
                                    .edited_values
                                    .get("value")
                                    .cloned()
                                    .unwrap_or_default(),
                                model: state
                                    .edited_values
                                    .get("model")
                                    .cloned()
                                    .unwrap_or_default(),
                            });
                            state.close();
                        }
                    }
                });
            });
        });

    result
}

/// Render a styled button
fn styled_button(ui: &mut Ui, text: &str, bg: Color32, hover_bg: Color32) -> Response {
    let button_size = Vec2::new(80.0, 28.0);
    let (rect, response) = ui.allocate_exact_size(button_size, Sense::click());

    if ui.is_rect_visible(rect) {
        let fill = if response.hovered() { hover_bg } else { bg };
        let rounding = Rounding::same(4.0);

        ui.painter().rect_filled(rect, rounding, fill);

        let text_color = if bg == theme::BUTTON_PRIMARY_BG {
            Color32::WHITE
        } else {
            theme::TEXT_PRIMARY
        };

        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(12.0),
            text_color,
        );
    }

    response
}

/// Validate all property values
fn validate_properties(state: &mut EnhancedPropertyEditorState) -> bool {
    state.validation_errors.clear();
    let mut valid = true;

    // Validate name
    if let Some(name) = state.edited_values.get("name") {
        if name.is_empty() {
            state
                .validation_errors
                .insert("name".to_string(), "Instance name is required".to_string());
            valid = false;
        } else if !name
            .chars()
            .next()
            .map(|c| c.is_ascii_alphabetic())
            .unwrap_or(false)
        {
            state.validation_errors.insert(
                "name".to_string(),
                "Name must start with a letter".to_string(),
            );
            valid = false;
        }
    }

    // Validate value if present
    if let Some(value) = state.edited_values.get("value") {
        if !value.is_empty() {
            if let Err(e) = crate::properties::dialog::parse_engineering_value(value) {
                state.validation_errors.insert("value".to_string(), e);
                valid = false;
            }
        }
    }

    valid
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_editor_state_default() {
        let state = EnhancedPropertyEditorState::new();
        assert!(!state.open);
        assert!(state.component_ids.is_empty());
    }

    #[test]
    fn test_enhanced_editor_open_for_component() {
        let mut state = EnhancedPropertyEditorState::new();
        state.open_for_component(
            42,
            ComponentType::Resistor,
            "R1".to_string(),
            "1k".to_string(),
            String::new(),
        );

        assert!(state.open);
        assert_eq!(state.component_ids, vec![42]);
        assert_eq!(state.component_type, Some(ComponentType::Resistor));
        assert_eq!(state.get_value("name"), Some(&"R1".to_string()));
        assert_eq!(state.get_value("value"), Some(&"1k".to_string()));
    }

    #[test]
    fn test_enhanced_editor_has_changes() {
        let mut state = EnhancedPropertyEditorState::new();
        state.open_for_component(
            1,
            ComponentType::Resistor,
            "R1".to_string(),
            "1k".to_string(),
            String::new(),
        );

        assert!(!state.has_changes());

        state.set_value("value".to_string(), "2k".to_string());
        assert!(state.has_changes());
    }

    #[test]
    fn test_enhanced_editor_revert() {
        let mut state = EnhancedPropertyEditorState::new();
        state.open_for_component(
            1,
            ComponentType::Resistor,
            "R1".to_string(),
            "1k".to_string(),
            String::new(),
        );

        state.set_value("value".to_string(), "999k".to_string());
        assert!(state.has_changes());

        state.revert();
        assert!(!state.has_changes());
        assert_eq!(state.get_value("value"), Some(&"1k".to_string()));
    }

    #[test]
    fn test_enhanced_editor_close() {
        let mut state = EnhancedPropertyEditorState::new();
        state.open_for_component(
            1,
            ComponentType::Resistor,
            "R1".to_string(),
            "1k".to_string(),
            String::new(),
        );

        state.close();

        assert!(!state.open);
        assert!(state.component_ids.is_empty());
    }

    #[test]
    fn test_validate_empty_name_fails() {
        let mut state = EnhancedPropertyEditorState::new();
        state.set_value("name".to_string(), String::new());

        let valid = validate_properties(&mut state);

        assert!(!valid);
        assert!(state.validation_errors.contains_key("name"));
    }

    #[test]
    fn test_validate_invalid_name_start_fails() {
        let mut state = EnhancedPropertyEditorState::new();
        state.set_value("name".to_string(), "1R".to_string());

        let valid = validate_properties(&mut state);

        assert!(!valid);
        assert!(state
            .validation_errors
            .get("name")
            .unwrap()
            .contains("letter"));
    }

    #[test]
    fn test_validate_valid_name_passes() {
        let mut state = EnhancedPropertyEditorState::new();
        state.set_value("name".to_string(), "R1".to_string());
        state.set_value("value".to_string(), "1k".to_string());

        let valid = validate_properties(&mut state);

        assert!(valid);
        assert!(state.validation_errors.is_empty());
    }

    #[test]
    fn test_validate_invalid_value_fails() {
        let mut state = EnhancedPropertyEditorState::new();
        state.set_value("name".to_string(), "R1".to_string());
        state.set_value("value".to_string(), "invalid".to_string());

        let valid = validate_properties(&mut state);

        assert!(!valid);
        assert!(state.validation_errors.contains_key("value"));
    }
}
