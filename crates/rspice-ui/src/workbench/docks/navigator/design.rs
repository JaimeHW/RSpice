//! Design navigator and component shelf from the workbench mockup.

use std::collections::{BTreeMap, HashSet};

use egui::{Key, Modifiers, Response, ScrollArea, Ui};

use crate::schematic::view::SchematicShelfDragPayload;
use crate::schematic::{ComponentPaletteEntry, component_palette};
use crate::simulation::netlist_gen::{DesignNet, HierarchySource, design_nets_with_hierarchy};
use crate::state::{
    ComponentType, LibraryCellInstance, LibraryCellPlacementCandidate, PortDirection,
    SavedOutputKind, Tool, library_cell_placement_candidates,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;

use super::super::super::commands::Command;
use super::super::super::design_system::{
    PANEL_SECTION_H, PANEL_TABS_H, WorkbenchIcon, schematic_section_header as shelf_section_header,
};
use super::super::super::state::DesignPanel;
use super::{
    empty_navigator_row, panel_search, schematic_nav_row_indented_drag_response,
    schematic_nav_row_indented_response,
};

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

fn nav_row_indented(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> bool {
    nav_row_indented_response(ui, icon, label, selected, meta, level).clicked()
}

fn nav_row_indented_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> Response {
    schematic_nav_row_indented_response(ui, icon, label, selected, meta, level, false, false, false)
}

fn nav_row_indented_mono_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> Response {
    schematic_nav_row_indented_response(ui, icon, label, selected, meta, level, true, false, false)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DesignNavigatorSection {
    Instances,
    Ports,
    Nets,
    NamedSignals,
}

const DESIGN_NAVIGATOR_SECTION_ORDER: [DesignNavigatorSection; 4] = [
    DesignNavigatorSection::Instances,
    DesignNavigatorSection::Ports,
    DesignNavigatorSection::Nets,
    DesignNavigatorSection::NamedSignals,
];

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
    let (rect, tablist_response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    ui.ctx()
        .accesskit_node_builder(tablist_response.id, |node| {
            node.set_role(egui::accesskit::Role::TabList);
            node.set_label("Design side panel");
        });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let entries = [
        (DesignPanel::Navigator, "Navigator"),
        (DesignPanel::ComponentShelf, "Component shelf"),
    ];
    let tab_ids = [
        ui.id().with(("design-panel-tab", 0)),
        ui.id().with(("design-panel-tab", 1)),
    ];
    let content_rect = panel_tabs_content_rect(rect);
    let font = theme::sans(tokens::FS_1, FontWeight::Medium);
    let desired_widths = entries.map(|(_, label)| {
        ui.painter()
            .layout_no_wrap(label.to_owned(), font.clone(), t.color.text)
            .size()
            .x
            + 10.0
    });
    let tab_widths = flexible_tab_widths(content_rect.width(), desired_widths);
    let mut tab_left = content_rect.left();
    for (index, (panel, label)) in entries.iter().copied().enumerate() {
        let tab_width = tab_widths[index];
        let tab_rect = egui::Rect::from_min_max(
            egui::pos2(tab_left, content_rect.top()),
            egui::pos2(tab_left + tab_width, content_rect.bottom()),
        );
        tab_left += tab_width;
        let response = ui.interact(tab_rect, tab_ids[index], egui::Sense::click());
        let selected = app.state.workbench.design_panel == panel;
        response.widget_info(|| {
            egui::WidgetInfo::selected(
                egui::WidgetType::SelectableLabel,
                ui.is_enabled(),
                selected,
                label,
            )
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Tab);
            node.set_label(label);
            node.set_selected(selected);
        });
        if response.hovered() {
            ui.painter().rect_filled(tab_rect, 0.0, t.color.bg_hover);
        }
        ui.painter().text(
            tab_rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            theme::sans(tokens::FS_1, FontWeight::Medium),
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
        if response.has_focus() {
            let target = ui.input_mut(|input| {
                if input.consume_key(Modifiers::NONE, Key::ArrowLeft)
                    || input.consume_key(Modifiers::NONE, Key::ArrowRight)
                {
                    Some(1 - index)
                } else if input.consume_key(Modifiers::NONE, Key::Home) {
                    Some(0)
                } else if input.consume_key(Modifiers::NONE, Key::End) {
                    Some(entries.len() - 1)
                } else {
                    None
                }
            });
            if let Some(target) = target {
                app.state.workbench.design_panel = entries[target].0;
                ui.memory_mut(|memory| memory.request_focus(tab_ids[target]));
            }
        }
    }
}

fn flexible_tab_widths<const N: usize>(available: f32, desired: [f32; N]) -> [f32; N] {
    if N == 0 {
        return desired;
    }
    let desired_total = desired.iter().sum::<f32>();
    if desired_total <= available {
        let extra = (available - desired_total) / N as f32;
        return desired.map(|width| width + extra);
    }
    let scale = if desired_total > 0.0 {
        available.max(0.0) / desired_total
    } else {
        0.0
    };
    desired.map(|width| width * scale)
}

fn navigator(ui: &mut Ui, app: &mut RSpiceApp) {
    navigator_search(ui, app);
    let (ancestors, current, can_ascend) = navigator_path(&app.state.workspace);
    let t = Tokens::get(ui.ctx());
    let mut ascend = false;
    let path_frame = egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 3.0;
                let ancestor_text = egui::RichText::new(&ancestors)
                    .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                    .color(t.color.text_dim);
                if can_ascend {
                    let response = ui
                        .add(egui::Button::new(ancestor_text).frame(false))
                        .on_hover_text("Ascend to the parent sheet");
                    if response.clicked() {
                        ascend = true;
                    }
                } else {
                    ui.label(ancestor_text);
                }
                ui.label(
                    egui::RichText::new("/")
                        .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
                ui.label(
                    egui::RichText::new(&current)
                        .font(theme::mono(tokens::FS_1, FontWeight::Medium))
                        .color(t.color.text),
                );
            });
        });
    ui.painter().hline(
        path_frame.response.rect.x_range(),
        path_frame.response.rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    if ascend {
        Command::AscendHierarchy.execute(app);
    }

    ScrollArea::vertical()
        .id_salt("workbench.design.navigator")
        .show(ui, |ui| {
            for section in DESIGN_NAVIGATOR_SECTION_ORDER {
                match section {
                    DesignNavigatorSection::Instances => instance_section(ui, app),
                    DesignNavigatorSection::Ports => port_section(ui, app),
                    DesignNavigatorSection::Nets => net_section(ui, app),
                    DesignNavigatorSection::NamedSignals => named_signal_section(ui, app),
                }
            }
        });
}

fn navigator_path(workspace: &crate::state::ProjectWorkspace) -> (String, String, bool) {
    let labels = workspace.occurrence_labels();
    let root_library = workspace
        .hierarchy_stack
        .first()
        .map_or(workspace.active_view.library.as_str(), |reference| {
            reference.library.as_str()
        });
    let current_occurrence = labels
        .last()
        .cloned()
        .unwrap_or_else(|| workspace.active_view.cell.clone());
    let mut ancestor_segments = Vec::with_capacity(labels.len());
    ancestor_segments.push(root_library.to_owned());
    ancestor_segments.extend(labels.iter().take(labels.len().saturating_sub(1)).cloned());
    let ancestors = format!("/ {}", ancestor_segments.join(" / "));
    let current = if labels.len() > 1 && current_occurrence != workspace.active_view.cell {
        format!("{current_occurrence} · {}", workspace.active_view.cell)
    } else {
        current_occurrence
    };
    (ancestors, current, workspace.hierarchy_stack.len() > 1)
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
    let hierarchy_labels = app.state.workspace.occurrence_labels();
    let hierarchy_references = if app.state.workspace.hierarchy_stack.is_empty() {
        vec![app.state.workspace.active_view.clone()]
    } else {
        app.state.workspace.hierarchy_stack.clone()
    };
    let hierarchy = hierarchy_references
        .into_iter()
        .enumerate()
        .map(|(depth, reference)| {
            let occurrence = hierarchy_labels
                .get(depth)
                .cloned()
                .unwrap_or_else(|| reference.cell.clone());
            (depth, (reference, occurrence))
        })
        .collect::<Vec<_>>();
    let hierarchy_depth = hierarchy.len().saturating_sub(1);
    let components = app
        .state
        .schematic
        .components
        .iter()
        // Interface ports own their dedicated mockup section below. Keeping
        // them out of Instances avoids presenting the same stable object
        // twice with two different navigation semantics.
        .filter(|component| is_instance_navigator_component(component.kind))
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
            let hierarchy_master = app
                .state
                .hierarchy_master_for_component(component.id)
                .map(|(_, reference)| reference);
            let nested_components = hierarchy_master
                .as_ref()
                .and_then(|reference| app.state.workspace.schematic_buffers.get(&reference.key()))
                .map(|schematic| {
                    schematic
                        .components
                        .iter()
                        .filter(|child| is_instance_navigator_component(child.kind))
                        .map(|child| {
                            (
                                child.id,
                                child.name.clone(),
                                child.value.clone(),
                                child.kind,
                                child.pos,
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            (
                component.id,
                component.name.clone(),
                component.value.clone(),
                component.kind,
                component.pos,
                hierarchy_master,
                nested_components,
            )
        })
        .collect::<Vec<_>>();

    navigator_section_header(ui, "Instances", &components.len().to_string());
    for (depth, (reference, occurrence)) in hierarchy {
        let active = depth == hierarchy_depth;
        let meta = if depth == 0 {
            reference.view.clone()
        } else {
            format!("{} · {}", reference.cell, reference.view)
        };
        let clicked = schematic_nav_row_indented_response(
            ui,
            WorkbenchIcon::Design,
            &occurrence,
            active,
            Some(&meta),
            depth,
            false,
            true,
            false,
        )
        .clicked();
        if clicked {
            if active {
                app.state.schematic.selection.clear();
                app.state.schematic.net_highlight.clear();
                app.state.schematic.needs_fit = true;
            } else {
                app.state.focus_workspace_breadcrumb(depth);
            }
        }
    }
    for (id, name, value, kind, position, hierarchy_master, nested_components) in components {
        let label = navigator_component_label(&name, &value, kind);
        let selected = app.state.schematic.selection.has_component(id);
        let has_hierarchy_master = hierarchy_master.is_some();
        let response = schematic_nav_row_indented_response(
            ui,
            WorkbenchIcon::Design,
            &label,
            selected,
            None,
            hierarchy_depth + 1,
            false,
            has_hierarchy_master,
            false,
        );
        if response.clicked() {
            app.state.schematic.selection.select_only_component(id);
            app.state.schematic.net_highlight.clear();
            app.state.schematic.center_request = Some(position);
        }
        if response.double_clicked() {
            app.state.schematic.selection.select_only_component(id);
            if has_hierarchy_master {
                app.state.open_selected_instance_master();
            } else {
                Command::ObjectProperties.execute(app);
            }
        }
        navigator_object_context_menu(
            &response,
            app,
            NavigatorObject::Component {
                id,
                label: name,
                position,
            },
        );

        if let Some(master) = hierarchy_master {
            for (child_id, child_name, child_value, child_kind, child_position) in nested_components
            {
                let child_label = navigator_component_label(&child_name, &child_value, child_kind);
                let child_response = schematic_nav_row_indented_response(
                    ui,
                    WorkbenchIcon::Design,
                    &child_label,
                    false,
                    None,
                    0,
                    false,
                    false,
                    true,
                )
                .on_hover_text(format!(
                    "Open {} and select {}",
                    master.display_path(),
                    child_name
                ));
                if child_response.clicked() {
                    app.state.schematic.selection.select_only_component(id);
                    app.state.open_selected_instance_master();
                    app.state
                        .schematic
                        .selection
                        .select_only_component(child_id);
                    app.state.schematic.net_highlight.clear();
                    app.state.schematic.center_request = Some(child_position);
                }
            }
        }
    }
}

fn net_section(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = normalized(&app.state.workbench.navigator_query);
    let hierarchy = HierarchySource::from_workspace_with_connectivity(
        &app.state.library_manager,
        &app.state.workspace.schematic_buffers,
        &app.state.workspace.connectivity,
    );
    let nets = design_nets_with_hierarchy(&app.state.schematic, &hierarchy)
        .into_iter()
        .filter(|net| matches_query(&query, &[net.name.as_str(), net.class.keyword(), "net"]))
        .collect::<Vec<_>>();
    navigator_section_header(ui, "Nets", &nets.len().to_string());
    if nets.is_empty() {
        empty_navigator_row(
            ui,
            if query.is_empty() {
                "No nets in this sheet"
            } else {
                "No nets match this filter"
            },
        );
        return;
    }
    for net in nets {
        let position = net_anchor(app, &net);
        let selected = if navigator_net_selection_matches(app, &net) {
            true
        } else if net.wire_ids.is_empty() {
            net.terminals.iter().any(|terminal| {
                app.state
                    .schematic
                    .selection
                    .has_component(terminal.component_id)
            })
        } else {
            net.wire_ids.iter().copied().collect::<HashSet<_>>()
                == app.state.schematic.net_highlight.highlighted_wires
        };
        let connection_count = net.pin_count().to_string();
        let response = nav_row_indented_mono_response(
            ui,
            if net.class == crate::simulation::netlist_gen::NetClass::Ground {
                WorkbenchIcon::Project
            } else {
                WorkbenchIcon::Design
            },
            &net.name,
            selected,
            Some(
                if net.class == crate::simulation::netlist_gen::NetClass::Ground {
                    "gnd"
                } else {
                    &connection_count
                },
            ),
            1,
        );
        if response.clicked() {
            select_navigator_design_net(app, &net, position);
        }
        navigator_object_context_menu(
            &response,
            app,
            NavigatorObject::Net {
                name: net.name,
                wire_ids: net.wire_ids,
                component_ids: net
                    .terminals
                    .iter()
                    .map(|terminal| terminal.component_id)
                    .collect(),
                position,
            },
        );
    }
}

fn net_anchor(app: &RSpiceApp, net: &DesignNet) -> Option<crate::state::Point> {
    app.state
        .schematic
        .net_labels
        .iter()
        .find(|label| label.name.eq_ignore_ascii_case(&net.name))
        .map(|label| label.pos)
        .or_else(|| {
            app.state
                .schematic
                .wires
                .iter()
                .find(|wire| net.wire_ids.contains(&wire.id))
                .and_then(crate::state::Wire::start)
        })
        .or_else(|| {
            net.terminals.iter().find_map(|terminal| {
                app.state
                    .schematic
                    .components
                    .iter()
                    .find(|component| component.id == terminal.component_id)
                    .map(|component| component.pos)
            })
        })
}

fn port_section(ui: &mut Ui, app: &mut RSpiceApp) {
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
    navigator_section_header(ui, "Ports", &ports.len().to_string());
    if ports.is_empty() {
        empty_navigator_row(
            ui,
            if query.is_empty() {
                "No ports declared in this sheet"
            } else {
                "No ports match this filter"
            },
        );
        return;
    }
    for (component_id, position, port) in ports {
        let icon = match port.direction {
            PortDirection::In => WorkbenchIcon::ArrowRight,
            PortDirection::Out => WorkbenchIcon::ArrowLeft,
            PortDirection::Supply => WorkbenchIcon::Supply,
            PortDirection::InOut => WorkbenchIcon::Design,
        };
        let response = nav_row_indented_mono_response(
            ui,
            icon,
            &port.name,
            app.state.schematic.selection.has_component(component_id),
            Some(port.direction.keyword()),
            1,
        );
        if response.clicked() {
            app.state
                .schematic
                .selection
                .select_only_component(component_id);
            app.state.schematic.net_highlight.clear();
            app.state.schematic.center_request = Some(position);
        }
        navigator_object_context_menu(
            &response,
            app,
            NavigatorObject::Component {
                id: component_id,
                label: port.name,
                position,
            },
        );
    }
}

fn named_signal_section(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = normalized(&app.state.workbench.navigator_query);
    let sources = app
        .state
        .schematic
        .components
        .iter()
        .filter(|component| is_named_source(component.kind))
        .filter(|component| {
            matches_query(
                &query,
                &[
                    component.name.as_str(),
                    component.value.as_str(),
                    component.kind.display_name(),
                    "source",
                ],
            )
        })
        .map(|component| {
            (
                component.id,
                component.pos,
                component.name.clone(),
                component.value.clone(),
            )
        })
        .collect::<Vec<_>>();
    let probes = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .and_then(|plan| app.state.workspace.active_plan_data(plan.id()))
        .map(|payload| {
            payload
                .saved_outputs
                .iter()
                .filter(|output| output.kind == SavedOutputKind::RawVoltageOrCurrent)
                .filter(|output| {
                    matches_query(
                        &query,
                        &[
                            output.name.as_str(),
                            output.source_expression.as_str(),
                            "probe",
                        ],
                    )
                })
                .map(|output| (output.name.clone(), output.source_expression.clone()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    navigator_section_header(
        ui,
        "Named signals",
        &(sources.len() + probes.len()).to_string(),
    );
    if sources.is_empty() && probes.is_empty() {
        empty_navigator_row(
            ui,
            if query.is_empty() {
                "No named sources or saved probes"
            } else {
                "No named sources or probes match this filter"
            },
        );
        return;
    }
    for (component_id, position, name, value) in sources {
        let meta = if value.trim().is_empty() {
            "source".to_owned()
        } else {
            format!("source \u{00b7} {value}")
        };
        let response = nav_row_indented_mono_response(
            ui,
            WorkbenchIcon::ArrowRight,
            &name,
            app.state.schematic.selection.has_component(component_id),
            Some(&meta),
            1,
        );
        if response.clicked() {
            app.state
                .schematic
                .selection
                .select_only_component(component_id);
            app.state.schematic.net_highlight.clear();
            app.state.schematic.center_request = Some(position);
        }
        navigator_object_context_menu(
            &response,
            app,
            NavigatorObject::Component {
                id: component_id,
                label: name,
                position,
            },
        );
    }
    for (name, expression) in probes {
        let response = nav_row_indented_mono_response(
            ui,
            WorkbenchIcon::ArrowLeft,
            &name,
            false,
            Some("probe"),
            1,
        );
        if response.clicked() {
            reveal_probe_expression(app, &expression);
        }
        navigator_object_context_menu(
            &response,
            app,
            NavigatorObject::SavedOutput { name, expression },
        );
    }
}

const fn is_named_source(kind: ComponentType) -> bool {
    matches!(
        kind,
        ComponentType::VoltageSource
            | ComponentType::CurrentSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm
            | ComponentType::CurrentSourceAc
            | ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourceExp
            | ComponentType::CurrentSourceNoise
            | ComponentType::BehavioralSource
            | ComponentType::RfPort
    )
}

fn is_instance_navigator_component(kind: ComponentType) -> bool {
    kind != ComponentType::Port
}

fn reveal_probe_expression(app: &mut RSpiceApp, expression: &str) {
    let Some(target) = raw_probe_target(expression) else {
        open_measurements(app);
        return;
    };

    match target {
        RawProbeTarget::Current(component_name) => {
            let Some(component) = app
                .state
                .schematic
                .components
                .iter()
                .find(|component| component.name.eq_ignore_ascii_case(component_name))
            else {
                open_measurements(app);
                return;
            };
            let id = component.id;
            let position = component.pos;
            app.state.schematic.selection.select_only_component(id);
            app.state.schematic.net_highlight.clear();
            app.state.schematic.center_request = Some(position);
        }
        RawProbeTarget::Voltage { positive, negative } => {
            let hierarchy = HierarchySource::from_workspace_with_connectivity(
                &app.state.library_manager,
                &app.state.workspace.schematic_buffers,
                &app.state.workspace.connectivity,
            );
            let nets = design_nets_with_hierarchy(&app.state.schematic, &hierarchy);
            let requested = std::iter::once(positive)
                .chain(negative)
                .collect::<Vec<_>>();
            let Some(resolved) = requested
                .iter()
                .map(|name| nets.iter().find(|net| net.name.eq_ignore_ascii_case(name)))
                .collect::<Option<Vec<_>>>()
            else {
                open_measurements(app);
                return;
            };
            let position = resolved.iter().find_map(|net| net_anchor(app, net));
            if resolved.len() == 1 {
                select_navigator_design_net(app, resolved[0], position);
                return;
            }

            let mut wire_ids = resolved
                .iter()
                .flat_map(|net| net.wire_ids.iter().copied())
                .collect::<Vec<_>>();
            wire_ids.sort_unstable();
            wire_ids.dedup();
            let mut component_ids = resolved
                .iter()
                .flat_map(|net| net.terminals.iter().map(|terminal| terminal.component_id))
                .collect::<Vec<_>>();
            component_ids.sort_unstable();
            component_ids.dedup();
            select_navigator_net(app, &wire_ids, &component_ids, position);
        }
    }
}

#[derive(Clone)]
enum NavigatorObject {
    Component {
        id: u64,
        label: String,
        position: crate::state::Point,
    },
    Net {
        name: String,
        wire_ids: Vec<u64>,
        component_ids: Vec<u64>,
        position: Option<crate::state::Point>,
    },
    SavedOutput {
        name: String,
        expression: String,
    },
}

impl NavigatorObject {
    fn summary(&self) -> String {
        match self {
            Self::Component { label, .. } | Self::Net { name: label, .. } => label.clone(),
            Self::SavedOutput {
                name, expression, ..
            } => format!("{name} · {expression}"),
        }
    }

    fn stable_path(&self, app: &RSpiceApp) -> String {
        let owner = app.state.workspace.active_display_path();
        match self {
            Self::Component { label, id, .. } => format!("{owner}/{label}#component-{id}"),
            Self::Net { name, .. } => format!("{owner}::net/{name}"),
            Self::SavedOutput { name, .. } => format!(
                "{}::saved-output/{name}",
                app.state
                    .sim_setup
                    .analysis_plan
                    .as_ref()
                    .map_or_else(|| "unbound-plan".to_owned(), |plan| plan.id().to_string())
            ),
        }
    }
}

fn select_navigator_object(app: &mut RSpiceApp, object: &NavigatorObject) {
    match object {
        NavigatorObject::Component { id, position, .. } => {
            app.state.schematic.selection.select_only_component(*id);
            app.state.schematic.net_highlight.clear();
            app.state.schematic.center_request = Some(*position);
        }
        NavigatorObject::Net {
            name,
            wire_ids,
            component_ids,
            position,
        } => {
            select_navigator_net(app, wire_ids, component_ids, *position);
            app.state
                .schematic
                .net_highlight
                .highlight_named_wires(name, wire_ids.iter().copied().collect());
        }
        NavigatorObject::SavedOutput { expression, .. } => {
            reveal_probe_expression(app, expression);
        }
    }
}

fn select_navigator_design_net(
    app: &mut RSpiceApp,
    net: &DesignNet,
    position: Option<crate::state::Point>,
) {
    let component_ids = net
        .terminals
        .iter()
        .map(|terminal| terminal.component_id)
        .collect::<Vec<_>>();
    select_navigator_net(app, &net.wire_ids, &component_ids, position);
    app.state
        .schematic
        .net_highlight
        .highlight_named_wires(&net.name, net.wire_ids.iter().copied().collect());
}

fn select_navigator_net(
    app: &mut RSpiceApp,
    wire_ids: &[u64],
    component_ids: &[u64],
    position: Option<crate::state::Point>,
) {
    app.state.schematic.selection.clear();
    for wire_id in wire_ids {
        app.state.schematic.selection.select_wire(*wire_id);
    }
    if wire_ids.is_empty() {
        for component_id in component_ids {
            app.state
                .schematic
                .selection
                .select_component(*component_id);
        }
    }
    app.state
        .schematic
        .net_highlight
        .highlight_wires(wire_ids.iter().copied().collect());
    app.state.schematic.center_request = position;
}

fn sorted_unique(values: impl IntoIterator<Item = u64>) -> Vec<u64> {
    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort_unstable();
    values.dedup();
    values
}

fn navigator_net_selection_matches(app: &RSpiceApp, net: &DesignNet) -> bool {
    if !app
        .state
        .schematic
        .net_highlight
        .selected_net_name
        .as_deref()
        .is_some_and(|name| name.eq_ignore_ascii_case(&net.name))
        || app.state.schematic.net_highlight.highlighted_wires
            != net.wire_ids.iter().copied().collect()
    {
        return false;
    }

    let wire_ids = sorted_unique(net.wire_ids.iter().copied());
    let component_ids = sorted_unique(net.terminals.iter().map(|terminal| terminal.component_id));
    let concrete = &app.state.schematic.selection;
    let no_other_classes = concrete.wire_segments.is_empty()
        && concrete.wire_vertices.is_empty()
        && concrete.junctions.is_empty()
        && concrete.buses.is_empty()
        && concrete.bus_taps.is_empty()
        && concrete.net_labels.is_empty()
        && concrete.design_notes.is_empty()
        && concrete.documentation_shapes.is_empty();
    no_other_classes
        && concrete.wires.iter().copied().collect::<HashSet<_>>()
            == wire_ids.iter().copied().collect()
        && if wire_ids.is_empty() {
            concrete.components.iter().copied().collect::<HashSet<_>>()
                == component_ids.iter().copied().collect()
        } else {
            concrete.components.is_empty()
        }
}

fn navigator_object_context_menu(
    response: &Response,
    app: &mut RSpiceApp,
    object: NavigatorObject,
) {
    let keyboard_open = response.has_focus()
        && response
            .ctx
            .input_mut(|input| input.consume_key(Modifiers::SHIFT, Key::F10));
    if response.secondary_clicked() || keyboard_open {
        select_navigator_object(app, &object);
    }

    let popup_id = egui::Popup::default_response_id(response);
    let mut popup = egui::Popup::context_menu(response).id(popup_id);
    if keyboard_open {
        popup = popup.open_memory(Some(egui::SetOpenCommand::Bool(true)));
    }
    popup.show(|ui| {
        ui.label(
            egui::RichText::new("NAVIGATOR OBJECT")
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(Tokens::get(ui.ctx()).color.text_faint),
        );
        ui.label(object.summary());
        ui.separator();

        if ui.button("Open selected object").clicked() {
            select_navigator_object(app, &object);
            match &object {
                NavigatorObject::Component { id, .. }
                    if app.state.hierarchy_master_for_component(*id).is_some() =>
                {
                    app.state.open_selected_instance_master();
                }
                NavigatorObject::Component { .. } if Command::ObjectProperties.is_enabled(app) => {
                    Command::ObjectProperties.execute(app);
                }
                NavigatorObject::Net { .. }
                | NavigatorObject::SavedOutput { .. }
                | NavigatorObject::Component { .. } => {}
            }
            ui.close();
        }
        let properties = ui.add_enabled(
            Command::ObjectProperties.is_enabled(app),
            egui::Button::new("Properties…"),
        );
        if properties.clicked() {
            Command::ObjectProperties.execute(app);
            ui.close();
        }
        let rename = ui.add_enabled(
            Command::RenameSelection.is_enabled(app),
            egui::Button::new("Rename…"),
        );
        if rename.clicked() {
            Command::RenameSelection.execute(app);
            ui.close();
        }
        ui.separator();
        if ui.button("Copy stable path").clicked() {
            ui.ctx().copy_text(object.stable_path(app));
            ui.close();
        }
        if ui.button("Find references and consumers…").clicked() {
            Command::FindInDesign.execute(app);
            ui.close();
        }
        if ui.button("Show dependency impact…").clicked() {
            Command::RevisionHistory.execute(app);
            ui.close();
        }
    });
}

fn open_measurements(app: &mut RSpiceApp) {
    app.state.workbench.console_page = super::super::super::state::ConsolePage::Measurements;
    app.state.workbench.console_visible = true;
    app.state.workbench.console_maximized = false;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RawProbeTarget<'a> {
    Voltage {
        positive: &'a str,
        negative: Option<&'a str>,
    },
    Current(&'a str),
}

fn raw_probe_target(expression: &str) -> Option<RawProbeTarget<'_>> {
    let expression = expression.trim();
    let open = expression.find('(')?;
    let function = expression[..open].trim();
    let body = expression.get(open + 1..)?.strip_suffix(')')?.trim();
    let arguments = body.split(',').map(str::trim).collect::<Vec<_>>();
    if arguments.iter().any(|argument| argument.is_empty()) {
        return None;
    }
    if function.eq_ignore_ascii_case("V") && matches!(arguments.len(), 1 | 2) {
        return Some(RawProbeTarget::Voltage {
            positive: arguments[0],
            negative: arguments.get(1).copied(),
        });
    }
    if function.eq_ignore_ascii_case("I") && arguments.len() == 1 {
        return Some(RawProbeTarget::Current(arguments[0]));
    }
    None
}

fn component_shelf(ui: &mut Ui, app: &mut RSpiceApp) {
    shelf_search(ui, app);
    let query = normalized(&app.state.workbench.placement_query);
    let visible_matches = component_shelf_match_count(app, &query);
    let mut primitive = None;
    let mut cell = None;
    ScrollArea::vertical()
        .id_salt("workbench.design.component_shelf")
        .show(ui, |ui| {
            primitive = pinned(ui, app).or_else(|| primitive_catalog(ui, app));
            cell = project_library(ui, app);
            if !query.is_empty() && visible_matches == 0 {
                empty_navigator_row(ui, "No component or cell matches this filter");
            }
        });
    if let Some(kind) = primitive {
        arm_primitive(app, kind, ui.ctx());
    } else if let Some(binding) = cell {
        arm_cell(app, binding, ui.ctx());
    }
}

fn component_shelf_match_count(app: &RSpiceApp, query: &str) -> usize {
    let primitive_matches = PRIMITIVE_GROUPS
        .iter()
        .map(|(_, section_names)| {
            primitive_entries(section_names)
                .into_iter()
                .filter(|entry| matches_query(query, &[entry.label, entry.kind.display_name()]))
                .count()
        })
        .sum::<usize>();
    let library_matches = cell_candidates(app)
        .into_iter()
        .filter(|candidate| {
            matches_query(
                query,
                &[&candidate.library, &candidate.cell, &candidate.view],
            )
        })
        .count();
    primitive_matches + library_matches
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
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), PANEL_SECTION_H),
        egui::Sense::hover(),
    );
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
    WorkbenchIcon::ChevronDown.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 15.0, rect.center().y),
            egui::vec2(12.0, 12.0),
        ),
        t.color.text_dim,
    );
    let title_job = egui::text::LayoutJob::single_section(
        title.to_uppercase(),
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_2, FontWeight::SemiBold),
            color: t.color.text_dim,
            extra_letter_spacing: 0.055 * tokens::FS_2,
            ..Default::default()
        },
    );
    let title_galley = ui.fonts_mut(|fonts| fonts.layout_job(title_job));
    ui.painter().galley(
        egui::pos2(
            rect.left() + 26.0,
            rect.center().y - title_galley.size().y * 0.5,
        ),
        title_galley,
        t.color.text_dim,
    );
    let count_galley = ui.painter().layout_no_wrap(
        count.to_owned(),
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_dim,
    );
    let count_x = rect.right() - 10.0 - count_galley.size().x;
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
        crate::workbench::app::runtime_command_platform(ui.ctx()),
        ui.ctx().os(),
    );
    shelf_section_header(
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
                            let response = place_chip(
                                ui,
                                kind,
                                glyph,
                                app.state.schematic.tool == Tool::Place(kind),
                            );
                            if let Some(payload) = SchematicShelfDragPayload::primitive(kind) {
                                response.dnd_set_drag_payload(payload);
                            }
                            if response.clicked() {
                                selected = Some(kind);
                            }
                        }
                    });
                });
        });
    selected
}

fn place_chip(ui: &mut Ui, kind: ComponentType, glyph: &str, selected: bool) -> Response {
    let t = Tokens::get(ui.ctx());
    let label = kind.display_name();
    let label_galley = ui.painter().layout_no_wrap(
        label.to_owned(),
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.text_dim,
    );
    let touch = t.metrics.ctl_h >= 44.0;
    let width = (14.0 + 17.0 + 5.0 + label_galley.size().x).max(if touch { 44.0 } else { 0.0 });
    let height = if touch { 44.0 } else { 23.0 };
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::click_and_drag());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            ui.is_enabled(),
            selected,
            format!("Arm {label} placement"),
        )
    });
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
    let glyph_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 7.0 + 8.5, rect.center().y),
        egui::vec2(15.0, 15.0),
    );
    if kind == ComponentType::Ground {
        // The bundled engineering faces are not required to carry the
        // Unicode earth-ground glyph. Paint the same three-bar mark as vector
        // geometry so the pinned shelf never degrades to a tofu box.
        WorkbenchIcon::Supply.paint(ui.painter(), glyph_rect, t.color.symbol);
    } else {
        ui.painter().text(
            glyph_rect.center(),
            egui::Align2::CENTER_CENTER,
            glyph,
            theme::mono(tokens::FS_0, FontWeight::Medium),
            t.color.symbol,
        );
    }
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
    response.on_hover_text(format!(
        "Click to arm {} placement or drag it onto the sheet",
        kind.display_name()
    ))
}

fn primitive_catalog(ui: &mut Ui, app: &RSpiceApp) -> Option<ComponentType> {
    let query = normalized(&app.state.workbench.placement_query);
    let mut armed = None;
    let visible_count = PRIMITIVE_GROUPS
        .iter()
        .map(|(_, section_names)| {
            primitive_entries(section_names)
                .into_iter()
                .filter(|entry| matches_query(&query, &[entry.label, entry.kind.display_name()]))
                .count()
        })
        .sum::<usize>();
    if visible_count == 0 && !query.is_empty() {
        return None;
    }
    shelf_section_header(ui, "Primitives", Some(&visible_count.to_string()));
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
            shelf_section_header(ui, group, Some(&entries.len().to_string()));
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
        let response = schematic_nav_row_indented_drag_response(
            ui,
            WorkbenchIcon::Design,
            entry.label,
            app.state.schematic.tool == Tool::Place(entry.kind),
            Some(entry.kind.spice_prefix()),
            level,
            false,
            false,
            false,
        );
        if let Some(payload) = SchematicShelfDragPayload::primitive(entry.kind) {
            response.dnd_set_drag_payload(payload);
        }
        if response.clicked() {
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
    shelf_section_header(ui, "Project library", None);
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
            shelf_section_header(ui, &library, Some(&cells.len().to_string()));
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
                if candidate.ready {
                    let payload =
                        SchematicShelfDragPayload::library_cell(candidate.binding.clone());
                    let response = schematic_nav_row_indented_drag_response(
                        ui,
                        WorkbenchIcon::Models,
                        &candidate.cell,
                        false,
                        Some(meta),
                        level,
                        false,
                        false,
                        false,
                    );
                    response.dnd_set_drag_payload(payload);
                    response.clone().on_hover_text(format!(
                        "Click to arm {}/{} or drag it onto the sheet",
                        candidate.library, candidate.cell
                    ));
                    response.clicked()
                } else {
                    nav_row_indented(
                        ui,
                        WorkbenchIcon::Models,
                        &candidate.cell,
                        false,
                        Some(meta),
                        level,
                    )
                }
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
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 24.0), egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }

    let caret_center = egui::pos2(rect.left() + 26.5, rect.center().y);
    let caret_stroke = egui::Stroke::new(1.25, t.color.text_faint);
    let caret_points = if open {
        [
            egui::pos2(caret_center.x - 3.0, caret_center.y - 1.5),
            egui::pos2(caret_center.x, caret_center.y + 1.5),
            egui::pos2(caret_center.x + 3.0, caret_center.y - 1.5),
        ]
    } else {
        [
            egui::pos2(caret_center.x - 1.5, caret_center.y - 3.0),
            egui::pos2(caret_center.x + 1.5, caret_center.y),
            egui::pos2(caret_center.x - 1.5, caret_center.y + 3.0),
        ]
    };
    ui.painter()
        .add(egui::Shape::line(caret_points.to_vec(), caret_stroke));
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
        theme::sans(tokens::FS_1, FontWeight::Regular),
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

type CellCandidate = LibraryCellPlacementCandidate;

fn cell_candidates(app: &RSpiceApp) -> Vec<CellCandidate> {
    library_cell_placement_candidates(&app.state.library_manager, &app.state.workspace)
}

fn arm_primitive(app: &mut RSpiceApp, kind: ComponentType, ctx: &egui::Context) {
    if kind == ComponentType::Port {
        crate::workbench::commands::Command::PlacePin.execute(app);
        return;
    }
    app.state.schematic.pending_library_cell = None;
    app.state.schematic.arm_tool(Tool::Place(kind));
    app.state.ui.toasts.success(
        ctx,
        "Component placement armed",
        format!("{} will snap to the schematic grid.", kind.display_name()),
    );
    crate::schematic::view::request_schematic_canvas_focus(ctx);
}

fn arm_cell(app: &mut RSpiceApp, binding: LibraryCellInstance, ctx: &egui::Context) {
    let label = format!("{}/{}", binding.library, binding.cell);
    app.state.schematic.pending_library_cell = Some(binding);
    app.state
        .schematic
        .arm_tool(Tool::Place(ComponentType::CellInstance));
    app.state.ui.toasts.success(
        ctx,
        "Component placement armed",
        format!("{label} will snap to the schematic grid."),
    );
    crate::schematic::view::request_schematic_canvas_focus(ctx);
}

fn primitive_entries(section_names: &[&str]) -> Vec<ComponentPaletteEntry> {
    component_palette()
        .iter()
        .filter(|section| section_names.contains(&section.title))
        .flat_map(|section| section.entries.iter().copied())
        .collect()
}

#[cfg(test)]
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

fn navigator_component_label(name: &str, value: &str, kind: ComponentType) -> String {
    match (name.trim(), value.trim()) {
        ("", "") => kind.display_name().to_owned(),
        ("", value) => format!("{} · {value}", kind.display_name()),
        (name, "") => name.to_owned(),
        (name, value) => format!("{name} · {value}"),
    }
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
    fn design_tabs_flex_from_their_label_widths_like_the_mockup() {
        let widths = flexible_tab_widths(239.0, [59.0, 95.0]);
        assert!((widths[0] - 101.5).abs() <= 0.001);
        assert!((widths[1] - 137.5).abs() <= 0.001);
        assert!((widths.iter().sum::<f32>() - 239.0).abs() <= 0.001);
    }

    #[test]
    fn design_navigator_sections_follow_the_upgraded_mockup_order() {
        assert_eq!(
            DESIGN_NAVIGATOR_SECTION_ORDER,
            [
                DesignNavigatorSection::Instances,
                DesignNavigatorSection::Ports,
                DesignNavigatorSection::Nets,
                DesignNavigatorSection::NamedSignals,
            ]
        );
    }

    #[test]
    fn navigator_path_includes_library_and_current_occurrence() {
        let mut workspace = crate::state::ProjectWorkspace::default();
        let (ancestors, current, can_ascend) = navigator_path(&workspace);
        assert_eq!(ancestors, "/ user");
        assert_eq!(current, "top");
        assert!(!can_ascend);

        workspace.descend_into(
            "XAFE".to_owned(),
            crate::state::CellViewRef::new("user", "afe_core", "schematic"),
            crate::state::ViewType::Schematic,
        );
        let (ancestors, current, can_ascend) = navigator_path(&workspace);
        assert_eq!(ancestors, "/ user / top");
        assert_eq!(current, "XAFE · afe_core");
        assert!(can_ascend);
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
    fn named_signal_sources_exclude_passive_and_interface_objects() {
        assert!(is_named_source(ComponentType::VoltageSourcePulse));
        assert!(is_named_source(ComponentType::CurrentSourceNoise));
        assert!(is_named_source(ComponentType::BehavioralSource));
        assert!(!is_named_source(ComponentType::Resistor));
        assert!(!is_named_source(ComponentType::Port));
    }

    #[test]
    fn interface_ports_have_one_navigator_owner() {
        assert!(!is_instance_navigator_component(ComponentType::Port));
        assert!(is_instance_navigator_component(ComponentType::Resistor));
        assert!(is_instance_navigator_component(ComponentType::CellInstance));
    }

    #[test]
    fn unnamed_structural_components_keep_a_visible_navigator_label() {
        assert_eq!(
            navigator_component_label("", "", ComponentType::Ground),
            "Ground"
        );
        assert_eq!(
            navigator_component_label("", "0", ComponentType::Ground),
            "Ground · 0"
        );
        assert_eq!(
            navigator_component_label("R1", "1k", ComponentType::Resistor),
            "R1 · 1k"
        );
    }

    #[test]
    fn raw_probe_targets_cover_scalar_differential_and_current_navigation() {
        assert_eq!(
            raw_probe_target("V(afe_out)"),
            Some(RawProbeTarget::Voltage {
                positive: "afe_out",
                negative: None,
            })
        );
        assert_eq!(
            raw_probe_target(" v(VREF) "),
            Some(RawProbeTarget::Voltage {
                positive: "VREF",
                negative: None,
            })
        );
        assert_eq!(
            raw_probe_target("V(out, in)"),
            Some(RawProbeTarget::Voltage {
                positive: "out",
                negative: Some("in"),
            })
        );
        assert_eq!(
            raw_probe_target("I(VDD)"),
            Some(RawProbeTarget::Current("VDD"))
        );
        assert_eq!(raw_probe_target("gain"), None);
        assert_eq!(raw_probe_target("V(out,)"), None);
    }

    #[test]
    fn wireless_navigator_net_selection_is_exact_and_self_invalidating() {
        let mut app = RSpiceApp::test_instance();
        let net = DesignNet {
            name: "PORT_OUT".to_owned(),
            authored_name: true,
            class: crate::simulation::netlist_gen::NetClass::Signal,
            terminals: vec![crate::simulation::netlist_gen::NetTerminal {
                component_id: 9,
                reference: "X1".to_owned(),
                pin: "OUT".to_owned(),
            }],
            port: Some(crate::state::PortDirection::Out),
            wire_ids: Vec::new(),
        };
        app.state.schematic.selection.select_only_component(9);
        app.state
            .schematic
            .net_highlight
            .highlight_named_wires(&net.name, HashSet::new());
        assert!(navigator_net_selection_matches(&app, &net));

        app.state.schematic.selection.select_only_component(10);
        app.state.schematic.net_highlight.clear();
        assert!(!navigator_net_selection_matches(&app, &net));
    }

    #[test]
    fn shelf_match_count_drives_a_truthful_filtered_empty_state() {
        let app = RSpiceApp::test_instance();
        assert!(component_shelf_match_count(&app, "resistor") > 0);
        assert_eq!(
            component_shelf_match_count(&app, "no-such-component-or-cell"),
            0
        );
    }

    #[test]
    fn palette_placement_cancels_every_unfinished_conductor_route() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .schematic
            .start_wire(crate::state::Point::origin());
        app.state
            .schematic
            .start_bus(crate::state::Point::new(2, 3), None)
            .unwrap();

        arm_primitive(&mut app, ComponentType::Resistor, &egui::Context::default());

        assert_eq!(
            app.state.schematic.tool,
            Tool::Place(ComponentType::Resistor)
        );
        assert!(!app.state.schematic.wire_drawing.active);
        assert!(!app.state.schematic.bus_drawing.active);
    }

    #[test]
    fn port_shelf_entry_uses_the_typed_place_pin_transaction() {
        let mut app = RSpiceApp::test_instance();

        arm_primitive(&mut app, ComponentType::Port, &egui::Context::default());

        assert!(app.state.dialogs.pin_port.open);
        assert_eq!(app.state.schematic.tool, Tool::Select);
        assert!(app.state.schematic.pending_port.is_none());
        assert!(app.state.schematic.components.is_empty());
    }
}
