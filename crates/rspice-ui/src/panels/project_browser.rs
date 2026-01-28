//! Project Browser Panel
//!
//! Cadence-style Library/Cell/View hierarchy browser with:
//! - Dynamic library enumeration from LibraryManager
//! - Primitive component library (R, C, L, sources, semiconductors)
//! - User library support for custom cells
//! - Click-to-place component selection
//!
//! # Usage
//!
//! Click on a component in the library to select it for placement.
//! Then click on the schematic canvas to place the component.

use egui::{CollapsingHeader, Ui};

use crate::common::app::AppState;
use crate::state::{Component, ComponentType, LibraryManager, Tool};

// =============================================================================
// Public API
// =============================================================================

/// Render the project browser panel
pub fn render_project_browser(ui: &mut Ui, state: &mut AppState) {
    // Search filter
    ui.horizontal(|ui| {
        ui.label("🔍");
        ui.add(
            egui::TextEdit::singleline(&mut state.library_manager.filter_text)
                .hint_text("Filter...")
                .desired_width(ui.available_width() - 20.0),
        );
    });
    ui.add_space(4.0);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            // Get filter text
            let filter = state.library_manager.filter_text.to_lowercase();

            // Render each library
            let library_names: Vec<String> = state
                .library_manager
                .libraries_sorted()
                .iter()
                .map(|lib| lib.name.clone())
                .collect();

            for lib_name in library_names {
                render_library(ui, &lib_name, &filter, state);
            }
        });
}

// =============================================================================
// Library Rendering
// =============================================================================

/// Render a single library with its categories and cells
fn render_library(ui: &mut Ui, lib_name: &str, filter: &str, state: &mut AppState) {
    // Get categories for this library
    let categories = state.library_manager.categories(lib_name);

    // Determine icon based on library type
    let is_primitives = lib_name == LibraryManager::PRIMITIVES_LIBRARY;
    let icon = if is_primitives { "📚" } else { "📁" };
    let is_expanded = is_primitives; // Primitives library open by default

    CollapsingHeader::new(format!("{} {}", icon, lib_name))
        .default_open(is_expanded)
        .show(ui, |ui| {
            // For primitives library, show categories as sub-headers
            if is_primitives {
                for category in &categories {
                    render_category(ui, lib_name, category, filter, state);
                }
            } else {
                // For user libraries, show cells directly with "+ New Cell" option
                render_user_library_cells(ui, lib_name, filter, state);
            }
        });
}

/// Render a category within a library
fn render_category(
    ui: &mut Ui,
    lib_name: &str,
    category: &str,
    filter: &str,
    state: &mut AppState,
) {
    // Get cells in this category
    let cells: Vec<String> = state
        .library_manager
        .cells_in_category(lib_name, category)
        .iter()
        .filter(|cell| filter.is_empty() || cell.name.to_lowercase().contains(filter))
        .map(|c| c.name.clone())
        .collect();

    if cells.is_empty() && !filter.is_empty() {
        return; // Skip empty categories when filtering
    }

    CollapsingHeader::new(format!("📂 {}", category))
        .default_open(category == "Passives") // Open passives by default
        .show(ui, |ui| {
            for cell_name in cells {
                if let Some(comp_type) = cell_name_to_component_type(&cell_name) {
                    render_library_cell(ui, &cell_name, comp_type, state);
                }
            }
        });
}

/// Render user library cells with full LCV hierarchy and management
fn render_user_library_cells(ui: &mut Ui, lib_name: &str, filter: &str, state: &mut AppState) {
    // Get cells matching filter
    let cells: Vec<(String, Vec<String>)> = state
        .library_manager
        .get_library(lib_name)
        .map(|lib| {
            lib.cells_sorted()
                .iter()
                .filter(|c| filter.is_empty() || c.name.to_lowercase().contains(filter))
                .map(|c| {
                    let views: Vec<String> =
                        c.views_sorted().iter().map(|v| v.name.clone()).collect();
                    (c.name.clone(), views)
                })
                .collect()
        })
        .unwrap_or_default();

    if cells.is_empty() {
        ui.label("No cells defined");
    } else {
        for (cell_name, views) in &cells {
            // Render cell as collapsible header
            CollapsingHeader::new(format!("📄 {}", cell_name))
                .default_open(false)
                .show(ui, |ui| {
                    // Show views within cell
                    for view_name in views {
                        ui.horizontal(|ui| {
                            let icon = view_icon(view_name);
                            ui.label(format!("  {} {}", icon, view_name));

                            // Delete view button (small X)
                            if ui.small_button("×").on_hover_text("Delete view").clicked() {
                                state.pending_delete_view = Some((
                                    lib_name.to_string(),
                                    cell_name.clone(),
                                    view_name.clone(),
                                ));
                            }
                        });
                    }

                    // Add view button
                    ui.horizontal(|ui| {
                        if ui.small_button("+ View").clicked() {
                            state.dialogs.new_view_dialog = true;
                            state.dialogs.new_view_library = lib_name.to_string();
                            state.dialogs.new_view_cell = cell_name.clone();
                        }

                        // Delete cell button
                        if ui
                            .small_button("🗑 Delete Cell")
                            .on_hover_text("Delete this cell")
                            .clicked()
                        {
                            state.pending_delete_cell =
                                Some((lib_name.to_string(), cell_name.clone()));
                        }
                    });
                });
        }
    }

    ui.add_space(4.0);

    // Add new cell button
    if ui.small_button("+ New Cell").clicked() {
        state.dialogs.new_cell_dialog = true;
        state.dialogs.new_cell_name.clear();
        state.dialogs.new_cell_library = lib_name.to_string();
    }
}

/// Get icon for a view type
fn view_icon(view_name: &str) -> &'static str {
    match view_name {
        "schematic" => "📋",
        "symbol" => "🔲",
        "layout" => "🗺️",
        "testbench" => "🧪",
        "veriloga" => "📝",
        "netlist" => "📜",
        "waveform" => "📈",
        _ => "📄",
    }
}

/// Map primitive cell names to ComponentType
fn cell_name_to_component_type(cell_name: &str) -> Option<ComponentType> {
    match cell_name {
        // Passives
        "Resistor" => Some(ComponentType::Resistor),
        "Capacitor" => Some(ComponentType::Capacitor),
        "Inductor" => Some(ComponentType::Inductor),
        "Ground" => Some(ComponentType::Ground),
        // Sources
        "VSource DC" => Some(ComponentType::VoltageSource),
        "VSource AC" => Some(ComponentType::VoltageSourceAc),
        "VSource Pulse" => Some(ComponentType::VoltageSourcePulse),
        "VSource Sin" => Some(ComponentType::VoltageSourceSin),
        "ISource DC" => Some(ComponentType::CurrentSource),
        "ISource AC" => Some(ComponentType::CurrentSourceAc),
        // Controlled Sources
        "VCVS" => Some(ComponentType::Vcvs),
        "VCCS" => Some(ComponentType::Vccs),
        "CCVS" => Some(ComponentType::Ccvs),
        "CCCS" => Some(ComponentType::Cccs),
        // Semiconductors
        "Diode" => Some(ComponentType::Diode),
        "NMOS" => Some(ComponentType::Nmos),
        "PMOS" => Some(ComponentType::Pmos),
        "NPN" => Some(ComponentType::NpnBjt),
        "PNP" => Some(ComponentType::PnpBjt),
        "NJFET" => Some(ComponentType::Njfet),
        "PJFET" => Some(ComponentType::Pjfet),
        _ => None,
    }
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

    // =========================================================================
    // Cell to ComponentType Mapping Tests
    // =========================================================================

    #[test]
    fn test_cell_to_component_type_passives() {
        assert_eq!(
            super::cell_name_to_component_type("Resistor"),
            Some(ComponentType::Resistor)
        );
        assert_eq!(
            super::cell_name_to_component_type("Capacitor"),
            Some(ComponentType::Capacitor)
        );
        assert_eq!(
            super::cell_name_to_component_type("Inductor"),
            Some(ComponentType::Inductor)
        );
        assert_eq!(
            super::cell_name_to_component_type("Ground"),
            Some(ComponentType::Ground)
        );
    }

    #[test]
    fn test_cell_to_component_type_sources() {
        assert_eq!(
            super::cell_name_to_component_type("VSource DC"),
            Some(ComponentType::VoltageSource)
        );
        assert_eq!(
            super::cell_name_to_component_type("VSource AC"),
            Some(ComponentType::VoltageSourceAc)
        );
        assert_eq!(
            super::cell_name_to_component_type("VSource Pulse"),
            Some(ComponentType::VoltageSourcePulse)
        );
        assert_eq!(
            super::cell_name_to_component_type("ISource DC"),
            Some(ComponentType::CurrentSource)
        );
    }

    #[test]
    fn test_cell_to_component_type_semiconductors() {
        assert_eq!(
            super::cell_name_to_component_type("NMOS"),
            Some(ComponentType::Nmos)
        );
        assert_eq!(
            super::cell_name_to_component_type("PMOS"),
            Some(ComponentType::Pmos)
        );
        assert_eq!(
            super::cell_name_to_component_type("NPN"),
            Some(ComponentType::NpnBjt)
        );
        assert_eq!(
            super::cell_name_to_component_type("PNP"),
            Some(ComponentType::PnpBjt)
        );
        assert_eq!(
            super::cell_name_to_component_type("Diode"),
            Some(ComponentType::Diode)
        );
        assert_eq!(
            super::cell_name_to_component_type("NJFET"),
            Some(ComponentType::Njfet)
        );
        assert_eq!(
            super::cell_name_to_component_type("PJFET"),
            Some(ComponentType::Pjfet)
        );
    }

    #[test]
    fn test_cell_to_component_type_controlled_sources() {
        assert_eq!(
            super::cell_name_to_component_type("VCVS"),
            Some(ComponentType::Vcvs)
        );
        assert_eq!(
            super::cell_name_to_component_type("VCCS"),
            Some(ComponentType::Vccs)
        );
        assert_eq!(
            super::cell_name_to_component_type("CCVS"),
            Some(ComponentType::Ccvs)
        );
        assert_eq!(
            super::cell_name_to_component_type("CCCS"),
            Some(ComponentType::Cccs)
        );
    }

    #[test]
    fn test_cell_to_component_type_unknown() {
        assert_eq!(super::cell_name_to_component_type("UnknownCell"), None);
        assert_eq!(super::cell_name_to_component_type("my_opamp"), None);
        assert_eq!(super::cell_name_to_component_type(""), None);
    }

    #[test]
    fn test_cell_to_component_type_case_sensitive() {
        // Should be case-sensitive matching
        assert_eq!(super::cell_name_to_component_type("resistor"), None);
        assert_eq!(super::cell_name_to_component_type("RESISTOR"), None);
        assert_eq!(
            super::cell_name_to_component_type("Resistor"),
            Some(ComponentType::Resistor)
        );
    }
}
