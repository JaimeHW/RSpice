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

use super::super::super::design_system::{PANEL_TABS_H, WorkbenchIcon, section_header};
use super::super::super::state::DesignPanel;
use super::{nav_row, nav_row_indented, nav_row_indented_mono, panel_search};

const PRIMITIVE_GROUPS: [(&str, &[&str]); 4] = [
    ("Passives", &["Passives"]),
    ("Sources", &["Sources"]),
    (
        "Analog",
        &["Hierarchy", "Semiconductors", "Controlled sources"],
    ),
    ("Mixed signal / XSPICE", &["Behavioral (XSPICE)"]),
];
const PANEL_TABS_PADDING_X: f32 = 8.0;

fn panel_tabs_content_rect(rect: egui::Rect) -> egui::Rect {
    let padding = PANEL_TABS_PADDING_X.min(rect.width() * 0.5);
    egui::Rect::from_min_max(
        egui::pos2(rect.left() + padding, rect.top()),
        egui::pos2(rect.right() - padding, rect.bottom()),
    )
}

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    tabs(ui, app);
    match app.state.workbench.design_panel {
        DesignPanel::Navigator => navigator(ui, app),
        DesignPanel::ComponentShelf => component_shelf(ui, app),
    }
}

fn tabs(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let height = PANEL_TABS_H.max(if t.metrics.ctl_h >= 44.0 { 44.0 } else { 0.0 });
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let entries = [
        (DesignPanel::Navigator, "Navigator"),
        (DesignPanel::ComponentShelf, "Component shelf"),
    ];
    let content_rect = panel_tabs_content_rect(rect);
    let tab_width = content_rect.width() / entries.len() as f32;
    for (index, (panel, label)) in entries.into_iter().enumerate() {
        let tab_rect = egui::Rect::from_min_max(
            egui::pos2(
                content_rect.left() + tab_width * index as f32,
                content_rect.top(),
            ),
            egui::pos2(
                content_rect.left() + tab_width * (index + 1) as f32,
                content_rect.bottom(),
            ),
        );
        let response = ui.interact(
            tab_rect,
            ui.id().with(("design-panel-tab", index)),
            egui::Sense::click(),
        );
        let selected = app.state.workbench.design_panel == panel;
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::SelectableLabel,
                ui.is_enabled(),
                selected,
                label,
            )
        });
        if response.hovered() {
            ui.painter().rect_filled(tab_rect, 0.0, t.color.bg_hover);
        }
        ui.painter().text(
            tab_rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            theme::sans(tokens::FS_0, FontWeight::Medium),
            if selected {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
        if selected {
            ui.painter().rect_filled(
                egui::Rect::from_min_max(
                    egui::pos2(tab_rect.left() + 6.0, tab_rect.bottom() - 2.0),
                    egui::pos2(tab_rect.right() - 6.0, tab_rect.bottom()),
                ),
                0.0,
                t.color.accent,
            );
        }
        theme::paint_focus_ring(ui, &response, tab_rect);
        if response.clicked() {
            app.state.workbench.design_panel = panel;
        }
    }
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
    let path_frame = egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            ui.add(
                egui::Label::new(
                    egui::RichText::new(format!("/ {path}"))
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
    ui.painter().hline(
        path_frame.response.rect.x_range(),
        path_frame.response.rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );

    ScrollArea::vertical()
        .id_salt("workbench.design.navigator")
        .show(ui, |ui| {
            instance_section(ui, app);
            net_section(ui, app);
            named_signal_section(ui, app);
        });
}

fn navigator_search(ui: &mut Ui, app: &mut RSpiceApp) {
    panel_search(
        ui,
        &mut app.state.workbench.navigator_query,
        "workbench.design.navigator.search",
        "Find instance, net or port…",
        &mut app.state.workbench.focus_navigator_search,
    );
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

    navigator_section_header(ui, "Instances", &components.len().to_string());
    ui.add_space(4.0);
    let root = app.state.workspace.active_view.cell.clone();
    if nav_row(ui, WorkbenchIcon::Design, &root, false, Some("schematic")) {
        app.state.schematic.selection.clear();
        app.state.schematic.net_highlight.clear();
        app.state.schematic.needs_fit = true;
    }
    for (id, name, value, _kind, position) in components {
        let label = if value.trim().is_empty() {
            name
        } else {
            format!("{name} · {value}")
        };
        let selected = app.state.schematic.selection.has_component(id);
        if nav_row_indented(ui, WorkbenchIcon::Design, &label, selected, None, 1) {
            app.state.schematic.selection.select_only_component(id);
            app.state.schematic.net_highlight.clear();
            app.state.schematic.center_request = Some(position);
        }
    }
    ui.add_space(7.0);
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
    navigator_section_header(ui, "Nets", &labels.len().to_string());
    ui.add_space(4.0);
    let graph = NetGraph::build(&app.state.schematic.wires, &app.state.schematic.junctions);
    for (name, position, ground) in labels {
        let connected = graph.find_connected_wires(position);
        let selected = !connected.is_empty()
            && connected == app.state.schematic.net_highlight.highlighted_wires;
        let count = connected.len().to_string();
        if nav_row_indented_mono(
            ui,
            if ground {
                WorkbenchIcon::Project
            } else {
                WorkbenchIcon::Design
            },
            &name,
            selected,
            Some(if ground { "gnd" } else { &count }),
            1,
        ) {
            app.state.schematic.selection.clear();
            for wire in &connected {
                app.state.schematic.selection.select_wire(*wire);
            }
            app.state.schematic.net_highlight.highlight_wires(connected);
            app.state.schematic.center_request = Some(position);
        }
    }
    ui.add_space(7.0);
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
    navigator_section_header(ui, "Named signals", &ports.len().to_string());
    ui.add_space(4.0);
    for (component_id, position, port) in ports {
        if nav_row_indented_mono(
            ui,
            WorkbenchIcon::Probe,
            &port.name,
            app.state.schematic.selection.has_component(component_id),
            Some(port.direction.keyword()),
            1,
        ) {
            app.state
                .schematic
                .selection
                .select_only_component(component_id);
            app.state.schematic.center_request = Some(position);
        }
    }
    ui.add_space(7.0);
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
    panel_search(
        ui,
        &mut app.state.workbench.placement_query,
        "workbench.design.component_shelf.search",
        "Place component or cell…",
        &mut app.state.workbench.focus_placement_search,
    );
}

fn navigator_section_header(ui: &mut Ui, title: &str, count: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 29.0), egui::Sense::hover());
    ui.painter().rect_filled(
        rect,
        0.0,
        egui::Color32::from_rgba_unmultiplied(
            t.color.bg_panel_2.r(),
            t.color.bg_panel_2.g(),
            t.color.bg_panel_2.b(),
            204,
        ),
    );
    ui.painter().text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        "⌄",
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 26.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        title.to_uppercase(),
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text_dim,
    );
    let count_galley = ui.painter().layout_no_wrap(
        count.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    let count_x = rect.right() - 10.0 - count_galley.size().x;
    ui.painter().circle_filled(
        egui::pos2(count_x - 9.0, rect.center().y),
        2.5,
        t.color.text_faint,
    );
    ui.painter().galley(
        egui::pos2(count_x, rect.center().y - count_galley.size().y * 0.5),
        count_galley,
        t.color.text_dim,
    );
}

fn pinned(ui: &mut Ui, app: &RSpiceApp) -> Option<ComponentType> {
    let query = normalized(&app.state.workbench.placement_query);
    if !query.is_empty() {
        return None;
    }
    let shortcut = app.state.ui.preferences.shortcuts().resolved_label(
        crate::workbench::commands::Command::PlaceInstance,
        crate::common::app::runtime_command_platform(ui.ctx()),
        ui.ctx().os(),
    );
    section_header(
        ui,
        "Pinned",
        (!shortcut.is_empty()).then_some(shortcut.as_str()),
    );
    let mut selected = None;
    egui::Frame::new()
        .inner_margin(egui::Margin {
            left: 8,
            right: 8,
            top: 7,
            bottom: 8,
        })
        .show(ui, |ui| {
            egui::ScrollArea::horizontal()
                .id_salt("workbench.design.pinned.scroll")
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 5.0;
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
                });
        });
    selected
}

fn place_chip(ui: &mut Ui, kind: ComponentType, glyph: &str, selected: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let label = kind.display_name();
    let label_galley = ui.painter().layout_no_wrap(
        label.to_owned(),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    let touch = t.metrics.ctl_h >= 44.0;
    let width = (14.0 + 17.0 + 5.0 + label_galley.size().x).max(if touch { 44.0 } else { 0.0 });
    let height = if touch { 44.0 } else { 23.0 };
    let (rect, response) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click());
    let fill = if selected {
        t.color.bg_active
    } else if response.hovered() {
        t.color.bg_hover
    } else {
        t.color.bg_inset
    };
    ui.painter().rect(
        rect,
        3.0,
        fill,
        egui::Stroke::new(
            1.0,
            if selected || response.hovered() {
                t.color.border_strong
            } else {
                t.color.border
            },
        ),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 7.0 + 8.5, rect.center().y),
        egui::Align2::CENTER_CENTER,
        glyph,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.symbol,
    );
    ui.painter().galley(
        egui::pos2(
            rect.left() + 7.0 + 17.0 + 5.0,
            rect.center().y - label_galley.size().y * 0.5,
        ),
        label_galley,
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    theme::paint_focus_ring_outset(ui, &response, rect);
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
            if catalog_group_row(
                ui,
                ("component-shelf", group),
                WorkbenchIcon::Design,
                group,
                entries.len(),
            ) {
                armed = primitive_rows(ui, app, &entries, 2).or(armed);
            }
        } else {
            section_header(ui, group, Some(&entries.len().to_string()));
            armed = primitive_rows(ui, app, &entries, 0).or(armed);
        }
    }
    armed
}

fn primitive_rows(
    ui: &mut Ui,
    app: &RSpiceApp,
    entries: &[ComponentPaletteEntry],
    level: usize,
) -> Option<ComponentType> {
    let mut armed = None;
    for entry in entries {
        if nav_row_indented(
            ui,
            WorkbenchIcon::Design,
            entry.label,
            app.state.schematic.tool == Tool::Place(entry.kind),
            Some(entry.kind.spice_prefix()),
            level,
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
            if catalog_group_row(
                ui,
                ("component-shelf-library", library.as_str()),
                WorkbenchIcon::Models,
                &library,
                cells.len(),
            ) {
                armed = cell_rows(ui, &cells, 2).or_else(|| armed.take());
            }
        } else {
            section_header(ui, &library, Some(&cells.len().to_string()));
            armed = cell_rows(ui, &cells, 0).or(armed);
        }
    }
    armed
}

fn cell_rows(ui: &mut Ui, cells: &[CellCandidate], level: usize) -> Option<LibraryCellInstance> {
    let mut armed = None;
    for candidate in cells {
        let meta = if candidate.ready {
            candidate.view.as_str()
        } else {
            candidate.unavailable_reason.as_str()
        };
        let clicked = ui
            .add_enabled_ui(candidate.ready, |ui| {
                nav_row_indented(
                    ui,
                    WorkbenchIcon::Models,
                    &candidate.cell,
                    false,
                    Some(meta),
                    level,
                )
            })
            .inner;
        if clicked {
            armed = Some(candidate.binding.clone());
        }
    }
    armed
}

/// Mockup-native expandable tree row used by the component shelf.
///
/// `egui::CollapsingHeader` carries stock indentation, typography, and
/// animation that do not match the workbench's 31 px tree-row contract.  The
/// shelf keeps only the persisted disclosure state and paints the same row
/// geometry as the rest of the navigator.
fn catalog_group_row(
    ui: &mut Ui,
    key: impl std::hash::Hash,
    icon: WorkbenchIcon,
    label: &str,
    count: usize,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let id = ui.make_persistent_id(key);
    let mut open = ui.data_mut(|data| data.get_persisted::<bool>(id).unwrap_or(false));
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), t.metrics.row_h),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }

    ui.painter().text(
        egui::pos2(rect.left() + 26.5, rect.center().y),
        egui::Align2::CENTER_CENTER,
        if open { "⌄" } else { "›" },
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 46.5, rect.center().y),
            egui::vec2(15.0, 15.0),
        ),
        t.color.text_faint,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 60.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        egui::pos2(rect.right() - 8.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        count.to_string(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    theme::paint_focus_ring(ui, &response, rect);

    if response.clicked() {
        open = !open;
        ui.data_mut(|data| data.insert_persisted(id, open));
    }
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_expanded(open);
    });
    open
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
    fn design_tabs_keep_the_mockup_horizontal_inset() {
        let outer = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(260.0, 33.0));
        let content = panel_tabs_content_rect(outer);

        assert_eq!(PANEL_TABS_PADDING_X, 8.0);
        assert_eq!(content.left(), 8.0);
        assert_eq!(content.right(), 252.0);
    }

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
