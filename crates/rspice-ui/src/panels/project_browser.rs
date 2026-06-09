//! Library Browser Panel
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
    Cell, CellViewRef, Component, ComponentType, LibraryCellInstance, LibraryManager, Tool, View,
    ViewType,
};

// =============================================================================
// Public API
// =============================================================================

/// Render the library browser panel.
pub fn render_library_browser(ui: &mut Ui, state: &mut AppState) {
    // Search filter
    ui.horizontal(|ui| {
        ui.label("Filter");
        ui.add(
            egui::TextEdit::singleline(&mut state.library_manager.filter_text)
                .hint_text("Filter...")
                .desired_width((ui.available_width() - 48.0).max(80.0)),
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

    let is_primitives = lib_name == LibraryManager::PRIMITIVES_LIBRARY;
    let is_expanded = is_primitives; // Primitives library open by default

    CollapsingHeader::new(lib_name)
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

    CollapsingHeader::new(category)
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
            CollapsingHeader::new(cell_name)
                .default_open(false)
                .show(ui, |ui| {
                    // Show views within cell
                    for view in views {
                        let view_name = &view.name;
                        let placeable_instance_result =
                            build_placeable_instance_with_reason(lib_name, cell, view);
                        let is_selected = if let Ok(instance) = &placeable_instance_result {
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
                            let kind = view_kind_label(view.view_type);
                            let reference = CellViewRef::new(
                                lib_name.to_string(),
                                cell_name.clone(),
                                view_name.clone(),
                            );
                            let is_active = state.workspace.active_view == reference;
                            let view_label = format!("  {:<3} {}", kind, view_name);
                            let response = ui.selectable_label(is_active, view_label);
                            if response.double_clicked() {
                                state.open_workspace_view(reference.clone());
                            }
                            response.on_hover_text(
                                "Double-click to open this view. Use Place to instantiate it.",
                            );

                            if ui.small_button("Open").clicked() {
                                state.open_workspace_view(reference.clone());
                            }

                            match placeable_instance_result {
                                Ok(instance) => {
                                    let place_label = if is_selected { "Placing" } else { "Place" };
                                    let response = ui.small_button(place_label);
                                    if response.clicked() {
                                        state.schematic.tool =
                                            Tool::Place(ComponentType::CellInstance);
                                        state.schematic.pending_library_cell = Some(instance);
                                        state.push_user_message(
                                            crate::common::app::ConsoleMessage::info(format!(
                                                "Click on schematic to place {}/{}/{}",
                                                lib_name, cell_name, view_name
                                            )),
                                        );
                                    }
                                    response.on_hover_text(
                                        "Click to place this library cell instance on schematic",
                                    );
                                }
                                Err(reason) => {
                                    ui.label(
                                        egui::RichText::new("Not placeable").weak().size(10.0),
                                    )
                                    .on_hover_text(format!("Not placeable: {}", reason));
                                }
                            }

                            // Delete view button (small X)
                            if ui.small_button("x").on_hover_text("Delete view").clicked() {
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
                            .small_button("Delete Cell")
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
/// Compact view kind label for dense Library/Cell/View rows.
fn view_kind_label(view_type: ViewType) -> &'static str {
    match view_type {
        ViewType::Schematic => "SCH",
        ViewType::Symbol => "SYM",
        ViewType::Layout => "LAY",
        ViewType::Testbench => "TB",
        ViewType::Verilog => "V",
        ViewType::VerilogA => "VA",
        ViewType::Spice => "SP",
        ViewType::Document => "DOC",
        ViewType::Extracted => "EXT",
        ViewType::Abstract => "ABS",
        ViewType::Config => "CFG",
        ViewType::Custom => "USR",
    }
}

/// Map primitive cell names to ComponentType
fn cell_name_to_component_type(cell_name: &str) -> Option<ComponentType> {
    match cell_name {
        // Passives
        "Resistor" => Some(ComponentType::Resistor),
        "Capacitor" => Some(ComponentType::Capacitor),
        "Inductor" => Some(ComponentType::Inductor),
        "Transformer" => Some(ComponentType::Transformer),
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

    let button = ui.selectable_label(is_selected, cell_name);

    if button.clicked() {
        // Set tool to place this component
        state.schematic.tool = Tool::Place(component_type);
        state.schematic.pending_library_cell = None;
        state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
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

fn build_placeable_instance_with_reason(
    lib_name: &str,
    cell: &Cell,
    view: &View,
) -> Result<LibraryCellInstance, String> {
    if !is_netlistable_view(view.view_type) {
        return Err(format!(
            "view type '{}' is not netlistable",
            view.view_type.display_name()
        ));
    }

    let source_path = resolve_view_source_path(cell, view);
    let source_path = source_path.ok_or_else(|| "missing source path metadata".to_string())?;

    let mut instance = LibraryCellInstance::new(lib_name, &cell.name, &view.name);
    instance.source_path = Some(source_path);
    instance.module_name = resolve_master_name(cell, view);
    instance.terminal_order = resolve_terminal_order(cell, view);

    if instance
        .module_name
        .as_ref()
        .map(|s| s.trim().is_empty())
        .unwrap_or(true)
    {
        return Err("missing netlist master/module binding".to_string());
    }

    if instance.terminal_order.is_empty() {
        return Err(
            "missing terminal order metadata (netlist.ports/netlist.terminals)".to_string(),
        );
    }

    Ok(instance)
}

fn is_netlistable_view(view_type: ViewType) -> bool {
    !matches!(
        view_type,
        ViewType::Layout | ViewType::Document | ViewType::Abstract | ViewType::Config
    )
}

fn resolve_view_source_path(cell: &Cell, view: &View) -> Option<std::path::PathBuf> {
    if let Some(path) = &view.file_path {
        return Some(path.clone());
    }

    metadata_source_path(&view.metadata, view)
        .or_else(|| metadata_source_path(&cell.metadata, view))
        .or_else(|| {
            // Backward-compatible Verilog-A metadata keys.
            view.metadata
                .get("veriloga.source_path")
                .map(std::path::PathBuf::from)
        })
        .or_else(|| {
            cell.metadata
                .get("veriloga.source_path")
                .map(std::path::PathBuf::from)
        })
}

fn metadata_source_path(
    metadata: &std::collections::HashMap<String, String>,
    view: &View,
) -> Option<std::path::PathBuf> {
    let view_key = view.name.to_ascii_lowercase();
    let keys = [
        "netlist.source_path".to_string(),
        "source_path".to_string(),
        format!("{}.source_path", view_key),
    ];
    for key in keys {
        if let Some(value) = metadata.get(&key) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(std::path::PathBuf::from(trimmed));
            }
        }
    }
    None
}

fn resolve_master_name(cell: &Cell, view: &View) -> Option<String> {
    if view.view_type == ViewType::VerilogA {
        return view
            .metadata
            .get("veriloga.module")
            .cloned()
            .or_else(|| cell.metadata.get("veriloga.module").cloned())
            .or_else(|| Some(cell.name.clone()));
    }

    metadata_master_name(&view.metadata, view)
        .or_else(|| metadata_master_name(&cell.metadata, view))
        .or_else(|| Some(cell.name.clone()))
}

fn metadata_master_name(
    metadata: &std::collections::HashMap<String, String>,
    view: &View,
) -> Option<String> {
    let view_key = view.name.to_ascii_lowercase();
    let keys = [
        "netlist.master".to_string(),
        "master".to_string(),
        "module".to_string(),
        format!("{}.master", view_key),
    ];
    for key in keys {
        if let Some(value) = metadata.get(&key) {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

fn resolve_terminal_order(cell: &Cell, view: &View) -> Vec<String> {
    if let Some(raw_ports) = view
        .metadata
        .get("netlist.ports")
        .or_else(|| view.metadata.get("netlist.terminals"))
        .or_else(|| cell.metadata.get("netlist.ports"))
        .or_else(|| cell.metadata.get("netlist.terminals"))
        .or_else(|| {
            // Backward-compatible Verilog-A metadata keys.
            view.metadata
                .get("veriloga.ports")
                .or_else(|| cell.metadata.get("veriloga.ports"))
        })
    {
        return parse_terminal_order(raw_ports);
    }
    Vec::new()
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
        if comp.name.starts_with(prefix)
            && let Ok(num) = comp.name[prefix.len()..].parse::<u32>()
        {
            max_num = max_num.max(num);
        }
    }

    format!("{}{}", prefix, max_num + 1)
}

// =============================================================================
// Tests
// =============================================================================
