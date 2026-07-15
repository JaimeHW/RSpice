//! Design navigator and component shelf from the workbench mockup.

use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

use egui::{ScrollArea, Ui};

use crate::common::RSpiceApp;
use crate::schematic::{ComponentPaletteEntry, component_palette};
use crate::state::{
    CellViewRef, ComponentType, LibraryCellInstance, NetGraph, PortDirection, PortSpec, Tool,
    ViewType,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::super::design_system::{WorkbenchIcon, section_header};
use super::super::super::state::DesignPanel;
use super::nav_row;

const PRIMITIVE_GROUPS: [(&str, &[&str]); 4] = [
    ("Passives", &["Passives"]),
    ("Sources", &["Sources"]),
    (
        "Analog",
        &["Hierarchy", "Semiconductors", "Controlled sources"],
    ),
    ("Mixed signal / XSPICE", &["Behavioral (XSPICE)"]),
];

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    tabs(ui, app);
    match app.state.workbench.design_panel {
        DesignPanel::Navigator => navigator(ui, app),
        DesignPanel::ComponentShelf => component_shelf(ui, app),
    }
}

fn tabs(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for (panel, label) in [
            (DesignPanel::Navigator, "Navigator"),
            (DesignPanel::ComponentShelf, "Component shelf"),
        ] {
            let selected = app.state.workbench.design_panel == panel;
            let response = ui.selectable_label(
                selected,
                egui::RichText::new(label)
                    .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                    .color(if selected {
                        t.color.text
                    } else {
                        t.color.text_dim
                    }),
            );
            if response.clicked() {
                app.state.workbench.design_panel = panel;
            }
        }
    });
    ui.add_space(4.0);
}

fn navigator(ui: &mut Ui, app: &mut RSpiceApp) {
    navigator_search(ui, app);
    let path = app
        .state
        .workspace
        .hierarchy_stack
        .iter()
        .map(|reference| reference.cell.as_str())
        .collect::<Vec<_>>()
        .join(" / ");
    let t = Tokens::get(ui.ctx());
    ui.add_space(2.0);
    ui.label(
        egui::RichText::new(format!("/ {path}"))
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_faint),
    );
    ui.add_space(4.0);

    ScrollArea::vertical()
        .id_salt("workbench.design.navigator")
        .show(ui, |ui| {
            instance_section(ui, app);
            net_section(ui, app);
            named_signal_section(ui, app);
        });
}

fn navigator_search(ui: &mut Ui, app: &mut RSpiceApp) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let response = ui.add_sized(
            [ui.available_width() - 16.0, 28.0],
            egui::TextEdit::singleline(&mut app.state.workbench.navigator_query)
                .id_salt("workbench.design.navigator.search")
                .hint_text("Find instance, net or port…")
                .margin(egui::Margin::symmetric(8, 5)),
        );
        if std::mem::take(&mut app.state.workbench.focus_navigator_search) {
            response.request_focus();
        }
    });
}

fn instance_section(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = normalized(&app.state.workbench.navigator_query);
    let components = app
        .state
        .schematic
        .components
        .iter()
        .filter(|component| {
            matches_query(
                &query,
                &[
                    &component.name,
                    &component.value,
                    component.kind.display_name(),
                ],
            )
        })
        .map(|component| {
            (
                component.id,
                component.name.clone(),
                component.value.clone(),
                component.kind,
                component.pos,
            )
        })
        .collect::<Vec<_>>();

    section_header(ui, "Instances", Some(&components.len().to_string()));
    let root = app.state.workspace.active_view.cell.clone();
    if nav_row(ui, WorkbenchIcon::Design, &root, false, Some("schematic")) {
        app.state.schematic.selection.clear();
        app.state.schematic.net_highlight.clear();
        app.state.schematic.needs_fit = true;
    }
    for (id, name, value, kind, position) in components {
        let label = if value.trim().is_empty() {
            name
        } else {
            format!("{name} · {value}")
        };
        let selected = app.state.schematic.selection.has_component(id);
        if nav_row(
            ui,
            WorkbenchIcon::Design,
            &label,
            selected,
            Some(kind.display_name()),
        ) {
            app.state.schematic.selection.select_only_component(id);
            app.state.schematic.net_highlight.clear();
            app.state.schematic.center_request = Some(position);
        }
    }
}

fn net_section(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = normalized(&app.state.workbench.navigator_query);
    let mut seen = HashSet::new();
    let labels = app
        .state
        .schematic
        .net_labels
        .iter()
        .filter(|label| seen.insert(label.name.to_ascii_lowercase()))
        .filter(|label| matches_query(&query, &[&label.name]))
        .map(|label| (label.name.clone(), label.pos, label.is_ground()))
        .collect::<Vec<_>>();
    section_header(ui, "Nets", Some(&labels.len().to_string()));
    let graph = NetGraph::build(&app.state.schematic.wires, &app.state.schematic.junctions);
    for (name, position, ground) in labels {
        let connected = graph.find_connected_wires(position);
        let selected = !connected.is_empty()
            && connected == app.state.schematic.net_highlight.highlighted_wires;
        let count = connected.len().to_string();
        if nav_row(
            ui,
            if ground {
                WorkbenchIcon::Project
            } else {
                WorkbenchIcon::Design
            },
            &name,
            selected,
            Some(if ground { "gnd" } else { &count }),
        ) {
            app.state.schematic.selection.clear();
            for wire in &connected {
                app.state.schematic.selection.select_wire(*wire);
            }
            app.state.schematic.net_highlight.highlight_wires(connected);
            app.state.schematic.center_request = Some(position);
        }
    }
}

fn named_signal_section(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = normalized(&app.state.workbench.navigator_query);
    let ports = app
        .state
        .schematic
        .components
        .iter()
        .filter_map(|component| {
            component
                .port_spec()
                .map(|port| (component.id, component.pos, port))
        })
        .filter(|(_, _, port)| matches_query(&query, &[&port.name, port.direction.keyword()]))
        .collect::<Vec<_>>();
    section_header(ui, "Named signals", Some(&ports.len().to_string()));
    for (component_id, position, port) in ports {
        if nav_row(
            ui,
            WorkbenchIcon::Probe,
            &port.name,
            app.state.schematic.selection.has_component(component_id),
            Some(port.direction.keyword()),
        ) {
            app.state
                .schematic
                .selection
                .select_only_component(component_id);
            app.state.schematic.center_request = Some(position);
        }
    }
}

fn component_shelf(ui: &mut Ui, app: &mut RSpiceApp) {
    shelf_search(ui, app);
    let mut primitive = None;
    let mut cell = None;
    ScrollArea::vertical()
        .id_salt("workbench.design.component_shelf")
        .show(ui, |ui| {
            primitive = pinned(ui, app).or_else(|| primitive_catalog(ui, app));
            cell = project_library(ui, app);
        });
    if let Some(kind) = primitive {
        arm_primitive(app, kind, ui.ctx());
    } else if let Some(binding) = cell {
        arm_cell(app, binding, ui.ctx());
    }
}

fn shelf_search(ui: &mut Ui, app: &mut RSpiceApp) {
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let response = ui.add_sized(
            [ui.available_width() - 16.0, 28.0],
            egui::TextEdit::singleline(&mut app.state.workbench.placement_query)
                .id_salt("workbench.design.component_shelf.search")
                .hint_text("Place component or cell…")
                .margin(egui::Margin::symmetric(8, 5)),
        );
        if std::mem::take(&mut app.state.workbench.focus_placement_search) {
            response.request_focus();
        }
    });
    ui.add_space(4.0);
}

fn pinned(ui: &mut Ui, app: &RSpiceApp) -> Option<ComponentType> {
    let query = normalized(&app.state.workbench.placement_query);
    if !query.is_empty() {
        return None;
    }
    section_header(ui, "Pinned", Some("Shift+I"));
    let mut selected = None;
    ui.horizontal_wrapped(|ui| {
        for (kind, glyph) in [
            (ComponentType::Resistor, "R"),
            (ComponentType::Capacitor, "C"),
            (ComponentType::Ground, "⏚"),
        ] {
            if place_chip(
                ui,
                kind,
                glyph,
                app.state.schematic.tool == Tool::Place(kind),
            ) {
                selected = Some(kind);
            }
        }
    });
    selected
}

fn place_chip(ui: &mut Ui, kind: ComponentType, glyph: &str, selected: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let label = format!("{glyph}  {}", kind.display_name());
    let response = ui.selectable_label(
        selected,
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_1, FontWeight::Medium))
            .color(if selected {
                t.color.text
            } else {
                t.color.text_dim
            }),
    );
    response
        .on_hover_text(format!("Arm {} placement", kind.display_name()))
        .clicked()
}

fn primitive_catalog(ui: &mut Ui, app: &RSpiceApp) -> Option<ComponentType> {
    let query = normalized(&app.state.workbench.placement_query);
    let mut armed = None;
    section_header(ui, "Primitives", Some(&primitive_entry_count().to_string()));
    for (group, section_names) in PRIMITIVE_GROUPS {
        let entries = primitive_entries(section_names)
            .into_iter()
            .filter(|entry| matches_query(&query, &[entry.label, entry.kind.display_name()]))
            .collect::<Vec<_>>();
        if entries.is_empty() {
            continue;
        }
        if query.is_empty() {
            egui::CollapsingHeader::new(format!("{group}   {}", entries.len()))
                .id_salt(("component-shelf", group))
                .default_open(false)
                .show(ui, |ui| {
                    armed = primitive_rows(ui, app, &entries).or(armed);
                });
        } else {
            section_header(ui, group, Some(&entries.len().to_string()));
            armed = primitive_rows(ui, app, &entries).or(armed);
        }
    }
    armed
}

fn primitive_rows(
    ui: &mut Ui,
    app: &RSpiceApp,
    entries: &[ComponentPaletteEntry],
) -> Option<ComponentType> {
    let mut armed = None;
    for entry in entries {
        if nav_row(
            ui,
            WorkbenchIcon::Design,
            entry.label,
            app.state.schematic.tool == Tool::Place(entry.kind),
            Some(entry.kind.spice_prefix()),
        ) {
            armed = Some(entry.kind);
        }
    }
    armed
}

fn project_library(ui: &mut Ui, app: &RSpiceApp) -> Option<LibraryCellInstance> {
    let query = normalized(&app.state.workbench.placement_query);
    let mut grouped = BTreeMap::<String, Vec<CellCandidate>>::new();
    for candidate in cell_candidates(app) {
        if matches_query(
            &query,
            &[&candidate.library, &candidate.cell, &candidate.view],
        ) {
            grouped
                .entry(candidate.library.clone())
                .or_default()
                .push(candidate);
        }
    }
    if grouped.is_empty() && !query.is_empty() {
        return None;
    }
    section_header(ui, "Project library", None);
    let mut armed = None;
    for (library, cells) in grouped {
        if query.is_empty() {
            egui::CollapsingHeader::new(format!("{library}   {}", cells.len()))
                .id_salt(("component-shelf-library", &library))
                .default_open(false)
                .show(ui, |ui| {
                    armed = cell_rows(ui, &cells).or_else(|| armed.take());
                });
        } else {
            section_header(ui, &library, Some(&cells.len().to_string()));
            armed = cell_rows(ui, &cells).or(armed);
        }
    }
    armed
}

fn cell_rows(ui: &mut Ui, cells: &[CellCandidate]) -> Option<LibraryCellInstance> {
    let mut armed = None;
    for candidate in cells {
        let meta = if candidate.ready {
            candidate.view.as_str()
        } else {
            candidate.unavailable_reason.as_str()
        };
        let clicked = ui
            .add_enabled_ui(candidate.ready, |ui| {
                nav_row(
                    ui,
                    WorkbenchIcon::Models,
                    &candidate.cell,
                    false,
                    Some(meta),
                )
            })
            .inner;
        if clicked {
            armed = Some(candidate.binding.clone());
        }
    }
    armed
}

#[derive(Clone)]
struct CellCandidate {
    library: String,
    cell: String,
    view: String,
    binding: LibraryCellInstance,
    ready: bool,
    unavailable_reason: String,
}

fn cell_candidates(app: &RSpiceApp) -> Vec<CellCandidate> {
    let active = &app.state.workspace.active_view;
    let mut candidates = Vec::new();
    for library in app.state.library_manager.libraries_sorted() {
        for cell in library.cells_sorted() {
            let preferred = cell
                .views_sorted()
                .into_iter()
                .find(|view| view.view_type == ViewType::Schematic)
                .or_else(|| {
                    cell.views_sorted()
                        .into_iter()
                        .find(|view| view.view_type == ViewType::VerilogA)
                })
                .or_else(|| {
                    cell.views_sorted()
                        .into_iter()
                        .find(|view| view.view_type == ViewType::Spice)
                });
            let Some(view) = preferred else {
                continue;
            };
            let mut binding = LibraryCellInstance::new(&library.name, &cell.name, &view.name);
            let is_current = library.name == active.library && cell.name == active.cell;
            let (ready, unavailable_reason) = match view.view_type {
                ViewType::Schematic => {
                    let reference = CellViewRef::new(&library.name, &cell.name, &view.name);
                    if let Some(master) =
                        app.state.workspace.schematic_buffers.get(&reference.key())
                    {
                        binding.bind_interface(&master.interface_ports());
                        (
                            !is_current,
                            if is_current {
                                "current cell".to_owned()
                            } else {
                                String::new()
                            },
                        )
                    } else {
                        (false, "open master first".to_owned())
                    }
                }
                ViewType::VerilogA | ViewType::Spice => {
                    let source = view
                        .file_path
                        .clone()
                        .or_else(|| metadata_path(&view.metadata))
                        .or_else(|| metadata_path(&cell.metadata));
                    let ports = metadata_ports(&view.metadata)
                        .or_else(|| metadata_ports(&cell.metadata))
                        .unwrap_or_default();
                    if !ports.is_empty() {
                        binding.bind_interface(&ports);
                    }
                    binding.source_path = source;
                    binding.module_name = view
                        .metadata
                        .get("veriloga.module")
                        .or_else(|| cell.metadata.get("veriloga.module"))
                        .cloned();
                    let ready = binding.source_path.is_some() && !binding.terminal_order.is_empty();
                    (
                        ready,
                        if ready {
                            String::new()
                        } else {
                            "missing source or ports".to_owned()
                        },
                    )
                }
                _ => unreachable!("candidate view was filtered"),
            };
            candidates.push(CellCandidate {
                library: library.name.clone(),
                cell: cell.name.clone(),
                view: view.name.clone(),
                binding,
                ready,
                unavailable_reason,
            });
        }
    }
    candidates
}

fn metadata_path(metadata: &std::collections::HashMap<String, String>) -> Option<PathBuf> {
    metadata
        .get("netlist.source_path")
        .or_else(|| metadata.get("veriloga.source_path"))
        .filter(|path| !path.trim().is_empty())
        .map(PathBuf::from)
}

fn metadata_ports(metadata: &std::collections::HashMap<String, String>) -> Option<Vec<PortSpec>> {
    let encoded = metadata
        .get("netlist.ports")
        .or_else(|| metadata.get("netlist.terminals"))
        .or_else(|| metadata.get("veriloga.ports"))?;
    let names = serde_json::from_str::<Vec<String>>(encoded).unwrap_or_else(|_| {
        encoded
            .split([',', ' ', '\t', '\n'])
            .filter(|name| !name.trim().is_empty())
            .map(|name| name.trim().to_owned())
            .collect()
    });
    Some(
        names
            .into_iter()
            .map(|name| PortSpec {
                name,
                direction: PortDirection::InOut,
            })
            .collect(),
    )
}

fn arm_primitive(app: &mut RSpiceApp, kind: ComponentType, ctx: &egui::Context) {
    app.state.schematic.pending_library_cell = None;
    app.state.schematic.tool = Tool::Place(kind);
    app.state.ui.toasts.success(
        ctx,
        "Component placement armed",
        format!("{} will snap to the schematic grid.", kind.display_name()),
    );
}

fn arm_cell(app: &mut RSpiceApp, binding: LibraryCellInstance, ctx: &egui::Context) {
    let label = format!("{}/{}", binding.library, binding.cell);
    app.state.schematic.pending_library_cell = Some(binding);
    app.state.schematic.tool = Tool::Place(ComponentType::CellInstance);
    app.state.ui.toasts.success(
        ctx,
        "Component placement armed",
        format!("{label} will snap to the schematic grid."),
    );
}

fn primitive_entries(section_names: &[&str]) -> Vec<ComponentPaletteEntry> {
    component_palette()
        .iter()
        .filter(|section| section_names.contains(&section.title))
        .flat_map(|section| section.entries.iter().copied())
        .collect()
}

fn primitive_entry_count() -> usize {
    component_palette()
        .iter()
        .map(|section| section.entries.len())
        .sum()
}

fn normalized(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

fn matches_query(query: &str, values: &[&str]) -> bool {
    query.is_empty()
        || values
            .iter()
            .any(|value| value.to_ascii_lowercase().contains(query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mockup_primitive_groups_cover_every_placeable_palette_entry_once() {
        let entries = PRIMITIVE_GROUPS
            .iter()
            .flat_map(|(_, sections)| primitive_entries(sections))
            .collect::<Vec<_>>();
        let unique = entries
            .iter()
            .map(|entry| entry.kind)
            .collect::<HashSet<_>>();

        assert_eq!(entries.len(), primitive_entry_count());
        assert_eq!(unique.len(), entries.len());
    }

    #[test]
    fn shelf_search_matches_labels_case_insensitively() {
        assert!(matches_query("nmos", &["NMOS", "Semiconductors"]));
        assert!(!matches_query("nmos", &["Resistor", "Passives"]));
    }

    #[test]
    fn veriloga_port_metadata_accepts_json_and_legacy_lists() {
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("veriloga.ports".to_owned(), r#"["in","out"]"#.to_owned());
        assert_eq!(metadata_ports(&metadata).unwrap().len(), 2);
        metadata.insert("veriloga.ports".to_owned(), "in, out vss".to_owned());
        assert_eq!(metadata_ports(&metadata).unwrap().len(), 3);
    }
}
