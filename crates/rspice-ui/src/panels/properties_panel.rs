//! Properties Panel
//!
//! Right-side panel for displaying and editing component properties.
//! Integrates with the PropertyRegistry for schema-driven property editing.

use crate::common::app::AppState;
use crate::properties::{TabbedDialogResult, render_tabbed_property_dialog};
use egui::Ui;

/// Render the properties panel in the right sidebar
pub fn render_properties_panel(ui: &mut Ui, state: &mut AppState) {
    // Check if we have a selected component
    let selected_comp = state.schematic.selection.components.first().copied();

    if let Some(comp_id) = selected_comp {
        // Find the selected component
        if let Some(component) = state.schematic.components.iter().find(|c| c.id == comp_id) {
            let component_name = &component.name;
            let component_type = component.kind;

            ui.label(format!("{} ({:?})", component_name, component_type));
            ui.separator();

            // If tabbed dialog is open, show instructions
            if state.tabbed_property_dialog.open {
                ui.label("Editing properties in dialog...");
                ui.label("");
                ui.label("Adjust values in the property dialog window.");
                ui.label("Click Apply to save changes.");
            } else {
                // Show basic property summary
                ui.label("Double-click component to edit properties");
                ui.separator();

                // Use property bridge to collect all properties for display
                let properties =
                    crate::properties::property_bridge::collect_properties_from_component(
                        component,
                        &state.property_registry,
                    );

                // Show current property values in read-only mode
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        // Get property sheet for ordered display
                        if let Some(sheet) = state.property_registry.get(component_type) {
                            // Display properties in sheet order
                            for def in sheet.iter() {
                                // Get value from component, or fall back to default
                                let display_value = if let Some(value) = properties.get(&def.name) {
                                    value.display_string()
                                } else {
                                    // Use default value from the property definition
                                    def.default_value.display_string()
                                };

                                // Skip empty values
                                if display_value.is_empty() {
                                    continue;
                                }

                                ui.horizontal(|ui| {
                                    ui.label(format!("{}:", def.display_name));
                                    ui.label(&display_value);
                                });
                            }
                        } else {
                            // Fallback for components without registry entry
                            ui.horizontal(|ui| {
                                ui.label("Value:");
                                ui.label(&component.value);
                            });
                            if !component.params.is_empty() {
                                ui.horizontal(|ui| {
                                    ui.label("Params:");
                                    ui.label(&component.params);
                                });
                            }
                        }
                    });
            }
        } else {
            ui.label("Component not found");
        }
    } else {
        // No component selected
        ui.label("Select a component to view its properties.");
        ui.separator();
        ui.label("");
        ui.label("Tip: Double-click a component to edit properties.");
    }
}

/// Render the floating tabbed property dialog window
/// Call this from the main app update loop
pub fn render_property_dialog(ctx: &egui::Context, state: &mut AppState) -> TabbedDialogResult {
    let result = render_tabbed_property_dialog(
        ctx,
        &mut state.tabbed_property_dialog,
        &state.property_registry,
        &state.model_library_manager,
    );

    // Handle dialog result - apply changes back to component
    if matches!(result, TabbedDialogResult::Applied)
        && let Some(comp_id) = state.tabbed_property_dialog.component_id
    {
        // Clone the values to avoid borrow conflict
        let values = state.tabbed_property_dialog.values.clone();

        // Find the component and update its properties
        if let Some(component) = state
            .schematic
            .components
            .iter_mut()
            .find(|c| c.id == comp_id)
        {
            // Use property bridge for comprehensive property application
            // This properly serializes all properties including secondary params
            crate::properties::property_bridge::apply_properties_to_component(
                component,
                &values,
                &state.property_registry,
            );
            log::info!("Applied property changes to component {}", comp_id);
        }

        // Clear dialog state AFTER applying (values were preserved for this)
        state.tabbed_property_dialog.clear_after_apply();
    }

    result
}

