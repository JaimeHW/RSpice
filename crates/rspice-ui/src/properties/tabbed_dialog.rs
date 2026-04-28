//! Tabbed Property Dialog
//!
//! Commercial-grade tabbed property editing dialog matching Cadence Virtuoso
//! Edit Instance Properties (q) dialog behavior. Provides:
//!
//! - Category-based tabbed organization
//! - Expression vs. literal value editing
//! - "Show on Schematic" toggle per property
//! - Default value highlighting
//! - Validation with error feedback
//! - Undo/revert support
//!
//! # Architecture
//!
//! The dialog uses a three-tier design:
//! 1. `TabbedPropertyDialogState` - Core state management
//! 2. Tab rendering with category-based grouping
//! 3. Type-specific property editors

use crate::properties::model_browser::ModelBrowserState;
use crate::properties::pwl_editor::PwlEditorState;
use crate::state::ComponentType;
use crate::state::property_types::{
    DisplayMode, PropertyDefinition, PropertyRegistry, PropertySheet, PropertyType, PropertyValue,
    VisibilityCondition, format_engineering,
};
use egui::{Color32, RichText, Ui, Window};
use std::collections::{HashMap, HashSet};

// =============================================================================
// Dialog State
// =============================================================================

/// State for the tabbed property dialog.
///
/// Manages the complete lifecycle of property editing including:
/// - Opening for a specific component
/// - Tracking modifications
/// - Validation and error reporting
/// - Apply/cancel/revert operations
#[derive(Debug, Clone, Default)]
pub struct TabbedPropertyDialogState {
    /// Whether the dialog is currently open
    pub open: bool,

    /// ID of the component being edited
    pub component_id: Option<u64>,

    /// Name of the component being edited (e.g., "R1", "V1")
    pub component_name: Option<String>,

    /// Type of the component being edited
    pub component_type: Option<ComponentType>,

    /// Currently active tab
    pub active_tab: String,

    /// Ordered list of tabs
    pub tabs: Vec<TabInfo>,

    /// Current property values being edited
    pub values: HashMap<String, PropertyValue>,

    /// Original values (for revert)
    pub original_values: HashMap<String, PropertyValue>,

    /// Set of property names that have been modified
    pub modified: HashSet<String>,

    /// Validation errors by property name
    pub validation_errors: HashMap<String, String>,

    /// Whether to show advanced properties
    pub show_advanced: bool,

    /// Global error message (e.g., "Cannot apply changes")
    pub global_error: Option<String>,

    /// PWL editor state (for PWL sources)
    pub pwl_editor: PwlEditorState,

    /// Model browser state (for semiconductor components)
    pub model_browser: ModelBrowserState,
}

/// Information about a category tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabInfo {
    /// Internal category name
    pub name: String,

    /// Display name for the tab
    pub display_name: String,

    /// Order for sorting (lower = first)
    pub order: i32,

    /// Number of properties in this tab
    pub property_count: usize,

    /// Number of modified properties in this tab
    pub modified_count: usize,
}

impl TabInfo {
    /// Create a new tab info
    pub fn new(name: impl Into<String>, order: i32) -> Self {
        let name = name.into();
        Self {
            display_name: name.clone(),
            name,
            order,
            property_count: 0,
            modified_count: 0,
        }
    }
}

// =============================================================================
// Dialog Result
// =============================================================================

/// Result of the tabbed property dialog interaction.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TabbedDialogResult {
    /// No action taken
    #[default]
    None,
    /// User clicked Apply - changes should be committed
    Applied,
    /// User clicked Cancel - changes discarded
    Cancelled,
    /// User clicked Revert - restore original values
    Reverted,
}

// =============================================================================
// State Implementation
// =============================================================================

impl TabbedPropertyDialogState {
    /// Create a new dialog state
    pub fn new() -> Self {
        Self::default()
    }

    /// Open the dialog for a specific component.
    ///
    /// Populates the dialog with the component's property sheet and current values.
    pub fn open_for_component(
        &mut self,
        component_id: u64,
        component_name: impl Into<String>,
        component_type: ComponentType,
        sheet: &PropertySheet,
        current_values: HashMap<String, PropertyValue>,
    ) {
        self.open = true;
        self.component_id = Some(component_id);
        self.component_name = Some(component_name.into());
        self.component_type = Some(component_type);
        self.values = current_values.clone();
        self.original_values = current_values;
        self.modified.clear();
        self.validation_errors.clear();
        self.global_error = None;
        self.show_advanced = false;

        // Build tabs from property categories
        self.tabs = self.build_tabs_from_sheet(sheet);

        // Set active tab to first tab
        if let Some(first) = self.tabs.first() {
            self.active_tab = first.name.clone();
        }
    }

    /// Close the dialog AND clear all state (for Cancel)
    pub fn close(&mut self) {
        self.close_visual();
        self.component_id = None;
        self.component_name = None;
        self.component_type = None;
        self.values.clear();
        self.original_values.clear();
        self.modified.clear();
        self.validation_errors.clear();
        self.tabs.clear();
        self.global_error = None;
    }

    /// Close the dialog visually but KEEP values for caller to apply.
    /// Use this when Apply is clicked so caller can access values.
    pub fn close_visual(&mut self) {
        self.open = false;
        // NOTE: Do NOT clear component_id, component_type, or values here!
        // The caller needs these to apply changes.
    }

    /// Clear dialog state after values have been applied.
    /// Call this after successfully applying changes.
    pub fn clear_after_apply(&mut self) {
        self.component_id = None;
        self.component_name = None;
        self.component_type = None;
        self.values.clear();
        self.original_values.clear();
        self.modified.clear();
        self.validation_errors.clear();
        self.tabs.clear();
        self.global_error = None;
    }

    /// Cancel editing and close the dialog
    pub fn cancel(&mut self) {
        self.close();
    }

    /// Revert all changes to original values
    pub fn revert(&mut self) {
        self.values = self.original_values.clone();
        self.modified.clear();
        self.validation_errors.clear();
        self.global_error = None;
    }

    /// Set a property value.
    ///
    /// Tracks modification status and validates the value.
    pub fn set_value(&mut self, name: &str, value: PropertyValue) {
        // Check if value differs from original
        let is_modified = self
            .original_values
            .get(name)
            .map(|orig| orig != &value)
            .unwrap_or(true);

        if is_modified {
            self.modified.insert(name.to_string());
        } else {
            self.modified.remove(name);
        }

        self.values.insert(name.to_string(), value);

        // Clear validation error for this property
        self.validation_errors.remove(name);
    }

    /// Get the current value of a property
    pub fn get_value(&self, name: &str) -> Option<&PropertyValue> {
        self.values.get(name)
    }

    /// Check if a property has been modified
    pub fn is_modified(&self, name: &str) -> bool {
        self.modified.contains(name)
    }

    /// Check if any properties have been modified
    pub fn has_modifications(&self) -> bool {
        !self.modified.is_empty()
    }

    /// Validate all properties against the sheet definitions.
    ///
    /// Returns true if all validations pass.
    pub fn validate_all(&mut self, sheet: &PropertySheet) -> bool {
        self.validation_errors.clear();
        self.global_error = None;

        for (name, value) in &self.values {
            if let Some(def) = sheet.get(name)
                && let Err(error) = def.validate(value)
            {
                self.validation_errors.insert(name.clone(), error);
            }
        }

        if !self.validation_errors.is_empty() {
            self.global_error = Some(format!(
                "{} validation error(s)",
                self.validation_errors.len()
            ));
            return false;
        }

        true
    }

    /// Get the list of tabs
    pub fn get_tabs(&self) -> &[TabInfo] {
        &self.tabs
    }

    /// Get properties for a specific tab/category.
    ///
    /// Returns property names that belong to the given category.
    pub fn get_properties_for_tab<'a>(
        &self,
        tab_name: &str,
        sheet: &'a PropertySheet,
    ) -> Vec<&'a PropertyDefinition> {
        let mut props: Vec<_> = sheet
            .iter()
            .filter(|def| def.category == tab_name)
            .filter(|def| self.should_show_property(def))
            .collect();

        // Sort by display order
        props.sort_by_key(|def| def.display_order);
        props
    }

    /// Check if a property should be shown based on display mode and visibility
    pub fn should_show_property(&self, def: &PropertyDefinition) -> bool {
        // Check display mode
        match def.display_mode {
            DisplayMode::Hidden => return false,
            DisplayMode::Advanced if !self.show_advanced => return false,
            _ => {}
        }

        // Check visibility condition
        match &def.visibility_condition {
            VisibilityCondition::Always => true,
            VisibilityCondition::WhenNonDefault => {
                if let Some(value) = self.values.get(&def.name) {
                    value != &def.default_value
                } else {
                    false
                }
            }
            VisibilityCondition::WhenPropertyEquals { property, value } => {
                if let Some(prop_value) = self.values.get(property) {
                    prop_value.display_string() == *value
                } else {
                    false
                }
            }
            VisibilityCondition::WhenPropertySet(property) => {
                if let Some(prop_value) = self.values.get(property) {
                    match prop_value {
                        PropertyValue::String(s) => !s.is_empty(),
                        PropertyValue::Number { value, .. } => !value.is_nan(),
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }

    /// Check if a value differs from its default
    pub fn is_non_default(&self, def: &PropertyDefinition) -> bool {
        self.values
            .get(&def.name)
            .map(|v| v != &def.default_value)
            .unwrap_or(false)
    }

    /// Build tabs from a property sheet
    fn build_tabs_from_sheet(&self, sheet: &PropertySheet) -> Vec<TabInfo> {
        let mut category_orders: HashMap<String, i32> = HashMap::new();
        let mut category_counts: HashMap<String, usize> = HashMap::new();

        // Collect categories and their minimum display orders
        for def in sheet.iter() {
            let order = category_orders
                .entry(def.category.clone())
                .or_insert(def.display_order);
            *order = (*order).min(def.display_order);

            *category_counts.entry(def.category.clone()).or_insert(0) += 1;
        }

        // Build tab infos
        let mut tabs: Vec<TabInfo> = category_orders
            .into_iter()
            .map(|(name, order)| {
                let mut tab = TabInfo::new(&name, order);
                tab.property_count = category_counts.get(&name).copied().unwrap_or(0);
                tab
            })
            .collect();

        // Sort by order
        tabs.sort_by_key(|t| t.order);
        tabs
    }

    /// Update modified counts for tabs
    pub fn update_tab_modified_counts(&mut self, sheet: &PropertySheet) {
        for tab in &mut self.tabs {
            tab.modified_count = sheet
                .iter()
                .filter(|def| def.category == tab.name)
                .filter(|def| self.modified.contains(&def.name))
                .count();
        }
    }
}

// =============================================================================
// Dialog Rendering
// =============================================================================

/// Render the tabbed property dialog.
///
/// Returns the dialog result indicating user action.
pub fn render_tabbed_property_dialog(
    ctx: &egui::Context,
    state: &mut TabbedPropertyDialogState,
    registry: &PropertyRegistry,
    model_library_manager: &crate::state::ModelLibraryManager,
) -> TabbedDialogResult {
    let mut result = TabbedDialogResult::None;

    if !state.open {
        return result;
    }

    let component_type = match state.component_type {
        Some(t) => t,
        None => return result,
    };

    let sheet = match registry.get(component_type) {
        Some(s) => s,
        None => return result,
    };

    // Clone data we need before the closure to avoid borrow conflicts
    let active_tab = state.active_tab.clone();

    // Update modified counts
    state.update_tab_modified_counts(sheet);

    let title = format!(
        "Edit Properties - {}",
        match (&state.component_name, &state.component_type) {
            (Some(name), Some(kind)) => format!("{} ({:?})", name, kind),
            (Some(name), None) => name.clone(),
            (None, Some(kind)) => format!("{:?}", kind),
            (None, None) => "Component".to_string(),
        }
    );

    let mut should_close = false;
    let mut open = true; // Track if window close button was clicked

    Window::new(&title)
        .id(egui::Id::new("rspice_property_dialog"))
        .open(&mut open)
        .resizable(true)
        .collapsible(false)
        .scroll(false) // Content has its own ScrollArea, window shouldn't scroll
        .default_size([450.0, 400.0])
        .min_width(350.0)
        .min_height(200.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            // Tab bar - use cloned tabs to avoid borrow issues
            ui.horizontal(|ui| {
                let tabs: Vec<TabInfo> = state.tabs.clone();
                for tab in tabs {
                    let is_active = active_tab == tab.name;
                    let label = if tab.modified_count > 0 {
                        format!("{}*", tab.display_name)
                    } else {
                        tab.display_name.clone()
                    };

                    if ui.selectable_label(is_active, &label).clicked() {
                        state.active_tab = tab.name.clone();
                    }
                }

                // Add PWL Data tab for PWL sources
                if component_type.is_pwl_source() {
                    let is_pwl_active = active_tab == "PWL Data";
                    let pwl_label = if state.pwl_editor.is_modified {
                        "PWL Data*"
                    } else {
                        "PWL Data"
                    };
                    if ui.selectable_label(is_pwl_active, pwl_label).clicked() {
                        state.active_tab = "PWL Data".to_string();
                    }
                }
            });

            ui.separator();

            // Check if we're on the PWL Data tab
            if active_tab == "PWL Data" && component_type.is_pwl_source() {
                // Render PWL editor
                use crate::properties::pwl_editor::render_pwl_editor;
                egui::ScrollArea::vertical()
                    .id_salt("tabbed_dialog_pwl_content")
                    .max_height(280.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        render_pwl_editor(ui, &mut state.pwl_editor);
                    });
            } else {
                // Get properties for active tab
                let props: Vec<PropertyDefinition> = sheet
                    .iter()
                    .filter(|def| def.category == active_tab)
                    .filter(|def| {
                        // Check display mode
                        match def.display_mode {
                            DisplayMode::Hidden => false,
                            DisplayMode::Advanced if !state.show_advanced => false,
                            _ => true,
                        }
                    })
                    .cloned()
                    .collect();

                // Property list for active tab
                egui::ScrollArea::vertical()
                    .id_salt("tabbed_dialog_props_content")
                    .max_height(280.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for def in &props {
                            render_property_row(ui, def, state);
                        }
                    });
            }

            ui.separator();

            // Advanced toggle
            ui.horizontal(|ui| {
                ui.checkbox(&mut state.show_advanced, "Show Advanced");

                if state.has_modifications() {
                    ui.label(
                        RichText::new(format!("{} modified", state.modified.len()))
                            .color(Color32::YELLOW),
                    );
                }
            });

            // Error display
            if let Some(error) = &state.global_error.clone() {
                ui.colored_label(Color32::RED, error);
            }

            ui.separator();

            // Button row
            ui.horizontal(|ui| {
                if ui.button("Apply").clicked() && state.validate_all(sheet) {
                    result = TabbedDialogResult::Applied;
                    should_close = true;
                }

                if ui.button("Cancel").clicked() {
                    result = TabbedDialogResult::Cancelled;
                    should_close = true;
                }

                let has_mods = state.has_modifications();
                if ui
                    .add_enabled(has_mods, egui::Button::new("Revert"))
                    .clicked()
                {
                    state.revert();
                    result = TabbedDialogResult::Reverted;
                }
            });
        });

    // Handle window close button
    if !open {
        should_close = true;
        result = TabbedDialogResult::Cancelled;
    }

    if should_close {
        match result {
            TabbedDialogResult::Applied => {
                // Only close visually - preserve values for caller to apply
                state.close_visual();
            }
            _ => {
                // Cancel/X button - clear everything
                state.close();
            }
        }
    }

    // Render Model Browser modal if open
    if state.model_browser.open {
        use crate::properties::model_browser::{ModelBrowserResult, render_model_browser};

        // render_model_browser creates its own egui::Window internally
        match render_model_browser(ctx, &mut state.model_browser, model_library_manager) {
            ModelBrowserResult::Selected { model, .. } => {
                // Update the model property with selected model
                state.set_value(
                    "model",
                    crate::state::property_types::PropertyValue::String(model),
                );
                state.model_browser.open = false;
            }
            ModelBrowserResult::Cancelled => {
                state.model_browser.open = false;
            }
            ModelBrowserResult::None => {
                // Dialog still open, continue
            }
        }
    }

    result
}

/// Render a single property row
fn render_property_row(
    ui: &mut Ui,
    def: &PropertyDefinition,
    state: &mut TabbedPropertyDialogState,
) {
    let is_modified = state.is_modified(&def.name);
    let is_non_default = state.is_non_default(def);
    let has_error = state.validation_errors.contains_key(&def.name);

    ui.horizontal(|ui| {
        // Property name with styling
        let name_text = if def.required {
            format!("{}*", def.display_name)
        } else {
            def.display_name.clone()
        };

        let name_color = if has_error {
            Color32::RED
        } else if is_modified {
            Color32::YELLOW
        } else {
            ui.visuals().text_color()
        };

        // Bold if non-default
        let name_richtext = if is_non_default {
            RichText::new(&name_text).color(name_color).strong()
        } else {
            RichText::new(&name_text).color(name_color)
        };

        ui.label(name_richtext);

        // Value editor
        let current_value = state
            .get_value(&def.name)
            .cloned()
            .unwrap_or_else(|| def.default_value.clone());
        if let Some(new_value) = render_value_editor(ui, def, &current_value) {
            state.set_value(&def.name, new_value);
        }

        // Unit label
        if let Some(unit) = &def.unit {
            ui.label(RichText::new(unit).weak());
        }

        // Model Browser button for "model" property on semiconductor components
        if def.name == "model"
            && let Some(comp_type) = state.component_type
            && comp_type.is_semiconductor()
            && ui.small_button("📖 Browse...").clicked()
        {
            state.model_browser.open = true;
        }

        // Expression indicator
        if current_value.is_expression() {
            ui.label(RichText::new("{E}").color(Color32::LIGHT_BLUE).small());
        }
    });

    // Show error below if present
    if let Some(error) = state.validation_errors.get(&def.name) {
        ui.colored_label(Color32::RED, format!("  ↳ {}", error));
    }

    // Tooltip with description
    if !def.description.is_empty() {
        ui.label(RichText::new(&def.description).weak().small());
    }
}

/// Render the appropriate value editor for a property type.
///
/// Returns Some(new_value) if the value was changed.
fn render_value_editor(
    ui: &mut Ui,
    def: &PropertyDefinition,
    current: &PropertyValue,
) -> Option<PropertyValue> {
    // Handle read-only
    if def.read_only || def.display_mode == DisplayMode::Readonly {
        ui.label(current.display_string());
        return None;
    }

    match def.prop_type {
        PropertyType::Number => render_number_editor(ui, def, current),
        PropertyType::String => render_string_editor(ui, current),
        PropertyType::Expression => render_expression_editor(ui, current),
        PropertyType::Enum => render_enum_editor(ui, current),
        PropertyType::Boolean => render_boolean_editor(ui, current),
    }
}

/// Number editor with engineering notation support
fn render_number_editor(
    ui: &mut Ui,
    _def: &PropertyDefinition,
    current: &PropertyValue,
) -> Option<PropertyValue> {
    let text = match current {
        PropertyValue::Number { value, .. } => format_engineering(*value),
        PropertyValue::Expression(e) => e.clone(),
        _ => current.display_string(),
    };

    let mut new_text = text.clone();
    let response = ui.text_edit_singleline(&mut new_text);

    if response.changed() && new_text != text {
        // Check if it's an expression (starts with { or contains operators)
        if new_text.starts_with('{') || new_text.contains('*') || new_text.contains('/') {
            let expr = new_text.trim_matches(|c| c == '{' || c == '}').to_string();
            return Some(PropertyValue::Expression(expr));
        }

        // Try to parse as engineering notation
        if let Ok(value) = crate::properties::parse_engineering_value(&new_text) {
            return Some(PropertyValue::number(value));
        }
    }

    None
}

/// String editor
fn render_string_editor(ui: &mut Ui, current: &PropertyValue) -> Option<PropertyValue> {
    let text = match current {
        PropertyValue::String(s) => s.clone(),
        _ => current.display_string(),
    };

    let mut new_text = text.clone();
    if ui.text_edit_singleline(&mut new_text).changed() && new_text != text {
        return Some(PropertyValue::String(new_text));
    }

    None
}

/// Expression editor
fn render_expression_editor(ui: &mut Ui, current: &PropertyValue) -> Option<PropertyValue> {
    let text = match current {
        PropertyValue::Expression(e) => e.clone(),
        PropertyValue::Number { value, .. } => value.to_string(),
        _ => current.display_string(),
    };

    let mut new_text = text.clone();
    let response = ui.text_edit_singleline(&mut new_text);

    if response.changed() && new_text != text {
        // Try parsing as number first
        if let Ok(value) = new_text.parse::<f64>() {
            return Some(PropertyValue::number(value));
        }
        // Otherwise treat as expression
        return Some(PropertyValue::Expression(new_text));
    }

    None
}

/// Enum editor (dropdown)
fn render_enum_editor(ui: &mut Ui, current: &PropertyValue) -> Option<PropertyValue> {
    let (selected, options) = match current {
        PropertyValue::Enum { selected, options } => (selected.clone(), options.clone()),
        _ => (current.display_string(), vec![current.display_string()]),
    };

    let mut new_selected = selected.clone();

    egui::ComboBox::from_id_salt(&selected)
        .selected_text(&new_selected)
        .show_ui(ui, |ui| {
            for option in &options {
                if ui.selectable_label(*option == selected, option).clicked() {
                    new_selected = option.clone();
                }
            }
        });

    if new_selected != selected {
        return Some(PropertyValue::enumeration(new_selected, options));
    }

    None
}

/// Boolean editor (checkbox)
fn render_boolean_editor(ui: &mut Ui, current: &PropertyValue) -> Option<PropertyValue> {
    let value = match current {
        PropertyValue::Boolean(b) => *b,
        _ => false,
    };

    let mut new_value = value;
    if ui.checkbox(&mut new_value, "").changed() {
        return Some(PropertyValue::Boolean(new_value));
    }

    None
}

// =============================================================================
// Tests
// =============================================================================
