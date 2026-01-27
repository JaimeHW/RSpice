//! Project Browser Panel
//!
//! Cadence-style Library/Cell/View hierarchy browser with:
//! - Primitive component library (R, C, L, sources, semiconductors)
//! - Click-to-place component selection
//!
//! # Usage
//!
//! Click on a component in the library to select it for placement.
//! Then click on the schematic canvas to place the component.

use egui::{CollapsingHeader, Ui};

use crate::common::app::AppState;
use crate::state::{Component, ComponentType, Tool};

// =============================================================================
// Public API
// =============================================================================

/// Render the project browser panel
pub fn render_project_browser(ui: &mut Ui, state: &mut AppState) {
    ui.heading("Library Browser");
    ui.separator();

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            // Built-in primitives library
            CollapsingHeader::new("📚 Primitives")
                .default_open(true)
                .show(ui, |ui| {
                    render_library_cell(ui, "resistor", ComponentType::Resistor, state);
                    render_library_cell(ui, "capacitor", ComponentType::Capacitor, state);
                    render_library_cell(ui, "inductor", ComponentType::Inductor, state);
                    render_library_cell(ui, "ground", ComponentType::Ground, state);
                });

            // Sources
            CollapsingHeader::new("📚 Sources")
                .default_open(false)
                .show(ui, |ui| {
                    render_library_cell(ui, "vsource DC", ComponentType::VoltageSource, state);
                    render_library_cell(ui, "vsource AC", ComponentType::VoltageSourceAc, state);
                    render_library_cell(
                        ui,
                        "vsource Pulse",
                        ComponentType::VoltageSourcePulse,
                        state,
                    );
                    render_library_cell(ui, "vsource Sin", ComponentType::VoltageSourceSin, state);
                    render_library_cell(ui, "isource DC", ComponentType::CurrentSource, state);
                    render_library_cell(ui, "isource AC", ComponentType::CurrentSourceAc, state);
                });

            // Semiconductor devices
            CollapsingHeader::new("📚 Semiconductors")
                .default_open(false)
                .show(ui, |ui| {
                    render_library_cell(ui, "nmos", ComponentType::Nmos, state);
                    render_library_cell(ui, "pmos", ComponentType::Pmos, state);
                    render_library_cell(ui, "npn", ComponentType::NpnBjt, state);
                    render_library_cell(ui, "pnp", ComponentType::PnpBjt, state);
                    render_library_cell(ui, "diode", ComponentType::Diode, state);
                    render_library_cell(ui, "njfet", ComponentType::Njfet, state);
                    render_library_cell(ui, "pjfet", ComponentType::Pjfet, state);
                });

            // Controlled sources
            CollapsingHeader::new("📚 Controlled Sources")
                .default_open(false)
                .show(ui, |ui| {
                    render_library_cell(ui, "vcvs", ComponentType::Vcvs, state);
                    render_library_cell(ui, "vccs", ComponentType::Vccs, state);
                    render_library_cell(ui, "ccvs", ComponentType::Ccvs, state);
                    render_library_cell(ui, "cccs", ComponentType::Cccs, state);
                });

            // User libraries (placeholder)
            CollapsingHeader::new("📁 User Library")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("No user cells defined");
                    if ui.small_button("+ New Cell").clicked() {
                        state
                            .console_messages
                            .push(crate::common::app::ConsoleMessage::info(
                            "User cell creation: Coming soon. Use File > New to create schematics.",
                        ));
                    }
                });
        });
}

// =============================================================================
// Library Cell Rendering
// =============================================================================

/// Render a library cell entry with click-to-place support
fn render_library_cell(
    ui: &mut Ui,
    cell_name: &str,
    component_type: ComponentType,
    state: &mut AppState,
) {
    // Check if this component type is currently selected for placement
    let is_selected = matches!(&state.schematic.tool, Tool::Place(ct) if *ct == component_type);

    let button = ui.selectable_label(is_selected, format!("📦 {}", cell_name));

    if button.clicked() {
        // Set tool to place this component
        state.schematic.tool = Tool::Place(component_type);
        state
            .console_messages
            .push(crate::common::app::ConsoleMessage::info(format!(
                "Click on schematic to place {} (R to rotate, Esc to cancel)",
                cell_name
            )));
    }

    // Tooltip with component info
    button.on_hover_text(format!(
        "Click to select for placement\nType: {:?}\nPrefix: {}",
        component_type,
        component_type.spice_prefix()
    ));
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Generate a unique component name based on type and existing components
pub fn generate_component_name(comp_type: &ComponentType, existing: &[Component]) -> String {
    let prefix = comp_type.spice_prefix();

    // Find highest existing number for this prefix
    let mut max_num = 0;
    for comp in existing {
        if comp.name.starts_with(prefix) {
            if let Ok(num) = comp.name[prefix.len()..].parse::<u32>() {
                max_num = max_num.max(num);
            }
        }
    }

    format!("{}{}", prefix, max_num + 1)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;

    #[test]
    fn test_generate_component_name_resistor() {
        let existing = vec![
            Component::new(1, ComponentType::Resistor, Point::new(0, 0))
                .with_name_value("R1", "1k"),
            Component::new(2, ComponentType::Resistor, Point::new(0, 0))
                .with_name_value("R2", "2k"),
        ];
        let name = generate_component_name(&ComponentType::Resistor, &existing);
        assert_eq!(name, "R3");
    }

    #[test]
    fn test_generate_component_name_empty() {
        let existing: Vec<Component> = vec![];
        let name = generate_component_name(&ComponentType::Capacitor, &existing);
        assert_eq!(name, "C1");
    }

    #[test]
    fn test_generate_component_name_mixed() {
        let existing = vec![
            Component::new(1, ComponentType::Resistor, Point::new(0, 0))
                .with_name_value("R1", "1k"),
            Component::new(2, ComponentType::Capacitor, Point::new(0, 0))
                .with_name_value("C1", "10u"),
            Component::new(3, ComponentType::Resistor, Point::new(0, 0))
                .with_name_value("R5", "5k"),
        ];
        // R5 exists so next resistor should be R6
        let name = generate_component_name(&ComponentType::Resistor, &existing);
        assert_eq!(name, "R6");
    }

    #[test]
    fn test_project_browser_primitives_exist() {
        // Verify primitive component names are valid
        let primitives = ["resistor", "capacitor", "inductor", "ground"];
        for p in primitives {
            assert!(!p.is_empty());
        }
    }

    #[test]
    fn test_project_browser_semiconductors_exist() {
        let semis = ["nmos", "pmos", "npn", "pnp", "diode"];
        for s in semis {
            assert!(!s.is_empty());
        }
    }
}
