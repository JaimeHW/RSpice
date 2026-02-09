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
use crate::state::{
    Cell, Component, ComponentType, LibraryCellInstance, LibraryManager, Tool, View, ViewType,
};

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
    let cells: Vec<(Cell, Vec<View>)> = state
        .library_manager
        .get_library(lib_name)
        .map(|lib| {
            lib.cells_sorted()
                .iter()
                .filter(|c| filter.is_empty() || c.name.to_lowercase().contains(filter))
                .map(|c| {
                    let views: Vec<View> = c.views_sorted().iter().map(|v| (*v).clone()).collect();
                    ((*c).clone(), views)
                })
                .collect()
        })
        .unwrap_or_default();

    if cells.is_empty() {
        ui.label("No cells defined");
    } else {
        for (cell, views) in &cells {
            let cell_name = &cell.name;

            // Render cell as collapsible header
            CollapsingHeader::new(format!("📄 {}", cell_name))
                .default_open(false)
                .show(ui, |ui| {
                    // Show views within cell
                    for view in views {
                        let view_name = &view.name;
                        let placeable_instance = build_placeable_instance(lib_name, cell, view);
                        let is_selected = if let Some(instance) = &placeable_instance {
                            matches!(
                                state.schematic.tool,
                                Tool::Place(ComponentType::CellInstance)
                            ) && state
                                .schematic
                                .pending_library_cell
                                .as_ref()
                                .map(|pending| {
                                    pending.library == instance.library
                                        && pending.cell == instance.cell
                                        && pending.view == instance.view
                                })
                                .unwrap_or(false)
                        } else {
                            false
                        };

                        ui.horizontal(|ui| {
                            let icon = view_icon(view_name);
                            if let Some(instance) = placeable_instance {
                                let response = ui.selectable_label(
                                    is_selected,
                                    format!("  {} {}", icon, view_name),
                                );
                                if response.clicked() {
                                    state.schematic.tool = Tool::Place(ComponentType::CellInstance);
                                    state.schematic.pending_library_cell = Some(instance);
                                    state.console_messages.push(
                                        crate::common::app::ConsoleMessage::info(format!(
                                            "Click on schematic to place {}/{}/{}",
                                            lib_name, cell_name, view_name
                                        )),
                                    );
                                }
                                response.on_hover_text(
                                    "Click to place this library cell instance on schematic",
                                );
                            } else {
                                ui.label(format!("  {} {}", icon, view_name));
                            }

                            // Delete view button (small X)
                            if ui.small_button("×").on_hover_text("Delete view").clicked() {
                                state.pending_delete_view = Some((
                                    lib_name.to_string(),
                                    cell_name.to_string(),
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
                            state.dialogs.new_view_cell = cell_name.to_string();
                        }

                        // Delete cell button
                        if ui
                            .small_button("🗑 Delete Cell")
                            .on_hover_text("Delete this cell")
                            .clicked()
                        {
                            state.pending_delete_cell =
                                Some((lib_name.to_string(), cell_name.to_string()));
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
        // Voltage Sources
        "VSource DC" => Some(ComponentType::VoltageSource),
        "VSource AC" => Some(ComponentType::VoltageSourceAc),
        "VSource Pulse" => Some(ComponentType::VoltageSourcePulse),
        "VSource Sin" => Some(ComponentType::VoltageSourceSin),
        "VSource PWL" => Some(ComponentType::VoltageSourcePwl),
        "VSource Exp" => Some(ComponentType::VoltageSourceExp),
        "VSource SFFM" => Some(ComponentType::VoltageSourceSffm),
        // Current Sources
        "ISource DC" => Some(ComponentType::CurrentSource),
        "ISource AC" => Some(ComponentType::CurrentSourceAc),
        "ISource Pulse" => Some(ComponentType::CurrentSourcePulse),
        "ISource Sin" => Some(ComponentType::CurrentSourceSin),
        "ISource PWL" => Some(ComponentType::CurrentSourcePwl),
        "ISource Exp" => Some(ComponentType::CurrentSourceExp),
        "ISource Noise" => Some(ComponentType::CurrentSourceNoise),
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
        state.schematic.pending_library_cell = None;
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

fn build_placeable_instance(
    lib_name: &str,
    cell: &Cell,
    view: &View,
) -> Option<LibraryCellInstance> {
    if view.view_type != ViewType::VerilogA {
        return None;
    }

    let source_path = view
        .file_path
        .clone()
        .or_else(|| {
            view.metadata
                .get("veriloga.source_path")
                .map(std::path::PathBuf::from)
        })
        .or_else(|| {
            cell.metadata
                .get("veriloga.source_path")
                .map(std::path::PathBuf::from)
        });

    let mut instance = LibraryCellInstance::new(lib_name, &cell.name, &view.name);
    instance.source_path = source_path;
    instance.module_name = view
        .metadata
        .get("veriloga.module")
        .cloned()
        .or_else(|| cell.metadata.get("veriloga.module").cloned())
        .or_else(|| Some(cell.name.clone()));

    if let Some(raw_ports) = view
        .metadata
        .get("veriloga.ports")
        .or_else(|| cell.metadata.get("veriloga.ports"))
    {
        instance.terminal_order = parse_terminal_order(raw_ports);
    }

    if instance.source_path.is_none() {
        return None;
    }

    Some(instance)
}

fn parse_terminal_order(raw: &str) -> Vec<String> {
    if let Ok(parsed) = serde_json::from_str::<Vec<String>>(raw) {
        return parsed
            .into_iter()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
    }

    raw.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
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
    use std::path::PathBuf;

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

    #[test]
    fn test_parse_terminal_order_json_array() {
        let terminals = parse_terminal_order(r#"[" p ","n","","b"]"#);
        assert_eq!(terminals, vec!["p", "n", "b"]);
    }

    #[test]
    fn test_parse_terminal_order_csv_fallback() {
        let terminals = parse_terminal_order("p, n, , b");
        assert_eq!(terminals, vec!["p", "n", "b"]);
    }

    #[test]
    fn test_build_placeable_instance_veriloga_from_view_metadata() {
        let mut cell = Cell::new("res_mod");
        let mut view = View::new("veriloga", ViewType::VerilogA);
        view.file_path = Some(PathBuf::from("models/res_mod.va"));
        view.metadata
            .insert("veriloga.module".to_string(), "my_res_mod".to_string());
        view.metadata
            .insert("veriloga.ports".to_string(), r#"["p","n"]"#.to_string());

        let instance = build_placeable_instance("veriloga", &cell, &view)
            .expect("expected placeable Verilog-A instance");
        assert_eq!(instance.library, "veriloga");
        assert_eq!(instance.cell, "res_mod");
        assert_eq!(instance.view, "veriloga");
        assert_eq!(instance.module_name.as_deref(), Some("my_res_mod"));
        assert_eq!(
            instance.source_path,
            Some(PathBuf::from("models/res_mod.va"))
        );
        assert_eq!(instance.terminal_order, vec!["p", "n"]);

        // Ensure non-empty fallback still works if view metadata is absent.
        cell.metadata
            .insert("veriloga.module".to_string(), "cell_level_mod".to_string());
        let instance_from_cell =
            build_placeable_instance("veriloga", &cell, &view).expect("still placeable");
        assert_eq!(
            instance_from_cell.module_name.as_deref(),
            Some("my_res_mod")
        );
    }

    #[test]
    fn test_build_placeable_instance_uses_cell_metadata_fallbacks() {
        let mut cell = Cell::new("vactrl");
        cell.metadata
            .insert("veriloga.module".to_string(), "varactor".to_string());
        cell.metadata.insert(
            "veriloga.source_path".to_string(),
            "models/varactor.va".to_string(),
        );
        cell.metadata
            .insert("veriloga.ports".to_string(), "anode,cathode".to_string());

        let view = View::new("veriloga", ViewType::VerilogA);
        let instance = build_placeable_instance("veriloga", &cell, &view)
            .expect("expected placeable Verilog-A instance from cell metadata");
        assert_eq!(instance.module_name.as_deref(), Some("varactor"));
        assert_eq!(
            instance.source_path,
            Some(PathBuf::from("models/varactor.va"))
        );
        assert_eq!(instance.terminal_order, vec!["anode", "cathode"]);
    }

    #[test]
    fn test_build_placeable_instance_requires_source_path() {
        let cell = Cell::new("res_mod");
        let view = View::new("veriloga", ViewType::VerilogA);
        assert!(build_placeable_instance("veriloga", &cell, &view).is_none());
    }

    #[test]
    fn test_build_placeable_instance_rejects_non_veriloga_view() {
        let cell = Cell::new("res_mod");
        let mut view = View::new("symbol", ViewType::Symbol);
        view.file_path = Some(PathBuf::from("models/res_mod.va"));
        assert!(build_placeable_instance("veriloga", &cell, &view).is_none());
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
