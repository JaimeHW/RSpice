use crate::properties::model_browser::ModelBrowserState;
use crate::properties::pwl_editor::PwlEditorState;
use crate::state::ComponentType;
use crate::state::property_types::{
    DisplayMode, PropertyDefinition, PropertySheet, PropertyValue, VisibilityCondition,
};
use std::collections::{HashMap, HashSet};

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

        self.tabs = self.build_tabs_from_sheet(sheet);

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
    pub fn close_visual(&mut self) {
        self.open = false;
    }

    /// Clear dialog state after values have been applied.
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

    /// Mark the current values as applied while staying open: the draft
    /// becomes the new baseline.
    pub fn mark_applied(&mut self) {
        self.original_values = self.values.clone();
        self.modified.clear();
        self.validation_errors.clear();
        self.global_error = None;
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

        props.sort_by_key(|def| def.display_order);
        props
    }

    /// Check if a property should be shown based on display mode and visibility
    pub fn should_show_property(&self, def: &PropertyDefinition) -> bool {
        match def.display_mode {
            DisplayMode::Hidden => return false,
            DisplayMode::Advanced if !self.show_advanced => return false,
            _ => {}
        }

        match &def.visibility_condition {
            VisibilityCondition::Always => true,
            VisibilityCondition::WhenNonDefault => self
                .values
                .get(&def.name)
                .map(|value| value != &def.default_value)
                .unwrap_or(false),
            VisibilityCondition::WhenPropertyEquals { property, value } => self
                .values
                .get(property)
                .map(|prop_value| prop_value.display_string() == *value)
                .unwrap_or(false),
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

    /// Build tabs from a property sheet
    fn build_tabs_from_sheet(&self, sheet: &PropertySheet) -> Vec<TabInfo> {
        let mut category_orders: HashMap<String, i32> = HashMap::new();
        let mut category_counts: HashMap<String, usize> = HashMap::new();

        for def in sheet.iter() {
            let order = category_orders
                .entry(def.category.clone())
                .or_insert(def.display_order);
            *order = (*order).min(def.display_order);

            *category_counts.entry(def.category.clone()).or_insert(0) += 1;
        }

        let mut tabs: Vec<TabInfo> = category_orders
            .into_iter()
            .map(|(name, order)| {
                let mut tab = TabInfo::new(&name, order);
                tab.property_count = category_counts.get(&name).copied().unwrap_or(0);
                tab
            })
            .collect();

        tabs.sort_by_key(|t| t.order);
        tabs
    }
}
