//! Properties Panel
//!
//! Shows properties of selected components/wires for editing.

use egui::Ui;

use crate::egui_app::app::AppState;

/// Render the properties panel
pub fn render_properties_panel(ui: &mut Ui, state: &mut AppState) {
    ui.heading("Properties");
    ui.separator();

    let selection = &state.schematic.selection;
    let has_selection = !selection.is_empty();

    if !has_selection {
        ui.label("No selection");
        ui.add_space(8.0);
        ui.label("Select a component or wire to view and edit its properties.");
        return;
    }

    // Display component properties
    let selected_component_ids: Vec<u64> = selection.components.iter().copied().collect();

    if selected_component_ids.len() == 1 {
        let comp_id = selected_component_ids[0];

        // Find the component (immutable borrow first to get data)
        // Use correct field names: `kind` and `pos`
        let comp_data = state
            .schematic
            .components
            .iter()
            .find(|c| c.id == comp_id)
            .map(|c| {
                (
                    c.name.clone(),
                    c.value.clone(),
                    format!("{:?}", c.kind),
                    c.pos,
                    c.rotation.degrees(),
                )
            });

        if let Some((name, value, comp_type, position, rotation_deg)) = comp_data {
            ui.group(|ui| {
                ui.label(format!("📦 {}", comp_type));
                ui.separator();

                // Name field (read-only for now)
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.add_space(ui.available_width() - 100.0);
                    ui.label(&name);
                });

                // Value field (read-only for now)
                ui.horizontal(|ui| {
                    ui.label("Value:");
                    ui.add_space(ui.available_width() - 100.0);
                    ui.label(&value);
                });

                ui.separator();

                // Position
                ui.horizontal(|ui| {
                    ui.label("Position:");
                    ui.add_space(ui.available_width() - 120.0);
                    ui.label(format!("({}, {})", position.x, position.y));
                });

                // Rotation
                ui.horizontal(|ui| {
                    ui.label("Rotation:");
                    ui.add_space(ui.available_width() - 100.0);
                    ui.label(format!("{}°", rotation_deg));
                });
            });
        }
    } else if selected_component_ids.len() > 1 {
        // Multiple selection
        ui.label(format!(
            "{} components selected",
            selected_component_ids.len()
        ));
        ui.separator();
        ui.label("Multi-edit not yet supported");
    }

    // Display wire properties if selected
    let selected_wire_ids: Vec<u64> = selection.wires.iter().copied().collect();

    if !selected_wire_ids.is_empty() {
        ui.add_space(8.0);
        ui.separator();
        ui.label(format!("{} wire(s) selected", selected_wire_ids.len()));
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties_panel_no_selection() {
        let state = AppState::default();
        assert!(state.schematic.selection.is_empty());
    }

    #[test]
    fn test_properties_panel_with_component_selection() {
        let mut state = AppState::default();

        // Add a component using correct API
        use crate::state::{Component, ComponentType, Point};
        let comp = Component::new(1, ComponentType::Resistor, Point::new(100, 100))
            .with_name_value("R1", "1k");
        state.schematic.components.push(comp);

        // Select it
        state.schematic.selection.select_component(1);

        assert!(!state.schematic.selection.is_empty());
        assert!(state.schematic.selection.components.contains(&1));
    }
}
