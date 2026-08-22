//! Design navigator and component shelf from the workbench mockup.

use std::collections::{BTreeMap, HashSet};

use egui::{Key, Modifiers, Response, ScrollArea, Ui};

use crate::schematic::view::SchematicShelfDragPayload;
use crate::schematic::view::sheet_visibility::{self, SheetScope};
use crate::schematic::{ComponentPaletteEntry, component_palette};
use crate::simulation::netlist_gen::DesignNet;
use crate::state::model_hub::{
    ModelHubPartRow, PartPlacement, PartProvenance, PartState, plan_library_placement,
    refusal_sentence,
};
use crate::state::model_library::{ModelLibrary, ModelSourceAuthority};
use crate::state::{
    ComponentType, LibraryCellInstance, LibraryCellPlacementCandidate, PortDirection,
    SavedOutputKind, Tool, builtin_xspice_library_binding, builtin_xspice_vector_ports,
    engine_only_xspice_devices, generated_veriloga_devices, generated_veriloga_library_binding,
    library_cell_placement_candidates,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;

use super::super::super::commands::vocabulary::Command;
use super::super::super::design_system::{
    PANEL_SECTION_H, PANEL_TABS_H, WorkbenchIcon, schematic_section_header as shelf_section_header,
};
use super::super::super::state::DesignPanel;
use super::{
    empty_navigator_row, panel_search, schematic_nav_row_indented_drag_response,
    schematic_nav_row_indented_response,
};

mod hierarchy_tree;

#[cfg(test)]
mod tests;

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
/// Clearance between the location line and the sheet scope beneath it, so the
/// two read as one header block rather than two stacked rows.
const SCOPE_CONTROL_GAP: f32 = 6.0;

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
    Masters,
    Occurrences,
    Ports,
    Nets,
    Excitations,
    NamedSignals,
}

const DESIGN_NAVIGATOR_SECTION_ORDER: [DesignNavigatorSection; 6] = [
    DesignNavigatorSection::Masters,
    DesignNavigatorSection::Occurrences,
    DesignNavigatorSection::Ports,
    DesignNavigatorSection::Nets,
    DesignNavigatorSection::Excitations,
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
    navigator_search(ui, &mut app.state);
    let (occurrence, master, can_ascend) = navigator_path(&app.state.workspace);
    let t = Tokens::get(ui.ctx());
    let mut ascend = false;
    // A cell view with one sheet has nothing to scope, so the control is
    // absent there rather than offered with a single meaningful position.
    let scope = sheet_visibility::multi_sheet_catalog(&app.state)
        .map(|_| sheet_visibility::sheet_scope(ui.ctx()));
    let mut chosen_scope = None;
    let path_frame = egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 3.0;
                let occurrence_text = egui::RichText::new(&occurrence)
                    .font(theme::mono(tokens::FS_1, FontWeight::Medium))
                    .color(t.color.text);
                if can_ascend {
                    let response = ui
                        .add(egui::Button::new(occurrence_text).frame(false))
                        .on_hover_text("Ascend to the parent sheet");
                    if response.clicked() {
                        ascend = true;
                    }
                } else {
                    ui.label(occurrence_text);
                }
                ui.label(
                    egui::RichText::new("\u{00b7}")
                        .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text_faint),
                );
                ui.label(
                    egui::RichText::new(&master)
                        .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            });
            if let Some(scope) = scope {
                ui.add_space(SCOPE_CONTROL_GAP);
                chosen_scope = sheet_scope_control(ui, scope);
            }
        });
    ui.painter().hline(
        path_frame.response.rect.x_range(),
        path_frame.response.rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    if let Some(chosen) = chosen_scope {
        sheet_visibility::set_sheet_scope(ui.ctx(), chosen);
    }
    if ascend {
        Command::AscendHierarchy.execute(app);
    }

    ScrollArea::vertical()
        .id_salt("workbench.design.navigator")
        .show(ui, |ui| {
            for section in DESIGN_NAVIGATOR_SECTION_ORDER {
                match section {
                    DesignNavigatorSection::Masters => {
                        hierarchy_tree::masters_section(ui, &mut app.state)
                    }
                    DesignNavigatorSection::Occurrences => {
                        hierarchy_tree::occurrences_section(ui, app);
                    }
                    DesignNavigatorSection::Ports => port_section(ui, app),
                    DesignNavigatorSection::Nets => net_section(ui, app),
                    DesignNavigatorSection::Excitations => excitation_section(ui, &mut app.state),
                    DesignNavigatorSection::NamedSignals => named_signal_section(ui, app),
                }
            }
        });
}

/// The navigator's location line: the occurrence the session is editing, the
/// library cell bound there, and whether a parent exists to ascend to.
///
/// The design root is implicit, so the occurrence names instances only and the
/// top sheet reads `/`. The master is a separate fact from the occurrence — one
/// cell is instantiated under many instance names — so it is named beside the
/// path instead of being folded into it.
fn navigator_path(workspace: &crate::state::ProjectWorkspace) -> (String, String, bool) {
    let master = format!(
        "{}/{}",
        workspace.active_view.library, workspace.active_view.cell
    );
    (
        workspace.occurrence_path().to_string(),
        master,
        workspace.hierarchy_stack.len() > 1,
    )
}

/// The navigator's sheet scope: whether the object rails below list the whole
/// cell view or only the sheet on screen.
///
/// It sits under the location line because it narrows what that location
/// means. The hierarchy above it is never scoped — masters, occurrences and
/// sheet nodes describe the design's structure, which does not belong to one
/// sheet — so only the object rails change when this moves.
///
/// Returns the position chosen this frame, and nothing when it did not move.
fn sheet_scope_control(ui: &mut Ui, scope: SheetScope) -> Option<SheetScope> {
    let labels = SheetScope::OPTIONS.map(SheetScope::label);
    let mut index = SheetScope::OPTIONS
        .iter()
        .position(|candidate| *candidate == scope)
        .unwrap_or_default();
    let mut changed = false;
    ui.allocate_ui_with_layout(
        egui::vec2(
            ui.available_width().max(1.0),
            Tokens::get(ui.ctx()).metrics.ctl_h,
        ),
        egui::Layout::right_to_left(egui::Align::Center),
        |ui| {
            changed = crate::ui::widgets::segmented(
                ui,
                "workbench.design.navigator.sheet-scope",
                &labels,
                &mut index,
                crate::ui::widgets::SegmentedWidth::Natural,
            );
        },
    );
    changed.then(|| SheetScope::OPTIONS[index])
}

fn navigator_search(ui: &mut Ui, state: &mut crate::workbench::app_state::AppState) {
    let workspace = state.workbench.workspace;
    panel_search(
        ui,
        state.workbench.navigator_trees.filter_mut(workspace),
        "workbench.design.navigator.search",
        "Find instance, net or port…",
        &mut state.workbench.focus_navigator_search,
    );
}

/// A net is listed while any conductor or terminal it binds is in scope.
///
/// Membership is a property of an object, and a net is not one — it is what
/// the objects on a sheet add up to. A net crossing a sheet boundary therefore
/// stays listed on both sides rather than disappearing from the sheet that
/// only holds one end of it.
fn net_is_in_scope(
    state: &crate::workbench::app_state::AppState,
    scope: SheetScope,
    net: &DesignNet,
) -> bool {
    if scope == SheetScope::AllSheets {
        return true;
    }
    net.wire_ids
        .iter()
        .copied()
        .chain(net.terminals.iter().map(|terminal| terminal.component_id))
        .any(|id| sheet_visibility::object_is_in_scope(state, scope, id))
}

fn net_section(ui: &mut Ui, app: &mut RSpiceApp) {
    let scope = sheet_visibility::sheet_scope(ui.ctx());
    let query = normalized(app.state.workbench.navigator_filter());
    // The rail lists the nets the configured design has. When that design
    // does not resolve, the reason takes the list's place: a rail populated
    // from the editor buffer would offer conductors the run has no name for.
    let projection = match app.state.workspace.design_projection(
        &app.state.library_manager,
        &app.state.workspace.active_view,
        &app.state.schematic,
    ) {
        Ok(projection) => projection,
        Err(error) => {
            navigator_section_header(ui, "Nets", "\u{2014}");
            empty_navigator_row(ui, &error.to_string());
            return;
        }
    };
    let nets = crate::simulation::netlist_gen::projection_nets(
        &app.state.library_manager,
        &projection,
        &app.state.workspace.active_view.key(),
    )
    .iter()
    .filter(|net| net_is_in_scope(&app.state, scope, net))
    .filter(|net| matches_query(&query, &[net.name.as_str(), net.class.keyword(), "net"]))
    .cloned()
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
    let scope = sheet_visibility::sheet_scope(ui.ctx());
    let query = normalized(app.state.workbench.navigator_filter());
    let ports = app
        .state
        .schematic
        .components
        .iter()
        .filter(|component| sheet_visibility::object_is_in_scope(&app.state, scope, component.id))
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

/// Every excitation placed on this sheet, and what the plan reads each one as.
///
/// The row worth finding is the one with no reader: a source that is drawn,
/// will be netlisted, that no analysis in the plan names, and that none of the
/// plan's whole-design analyses reads either. Stating the reader count in the
/// row itself is what makes that visible without opening anything, so the count
/// is never elided.
///
/// It takes the session rather than the application because everything a row
/// does — select, clear the highlight, centre — is a schematic edit, and a rail
/// that cannot reach past the session cannot grow a command that does.
fn excitation_section(ui: &mut Ui, state: &mut crate::workbench::app_state::AppState) {
    let scope = sheet_visibility::sheet_scope(ui.ctx());
    let query = normalized(state.workbench.navigator_filter());
    let sources = crate::simulation::placed_sources::placed_sources(
        &state.schematic,
        state.sim_setup.analysis_plan.as_ref(),
    )
    .into_iter()
    .filter(|source| sheet_visibility::object_is_in_scope(state, scope, source.component_id))
    .filter(|source| {
        matches_query(
            &query,
            &[
                source.reference.as_str(),
                source.family,
                source.key_figure.as_str(),
                "excitation",
            ],
        )
    })
    .collect::<Vec<_>>();

    navigator_section_header(ui, "Excitations", &sources.len().to_string());
    if sources.is_empty() {
        empty_navigator_row(
            ui,
            if query.is_empty() {
                "No sources placed"
            } else {
                "No excitations match this filter"
            },
        );
        return;
    }
    for source in sources {
        let readers = match source.consumers.len() {
            0 => "no reader".to_owned(),
            1 => source.consumers[0].role.to_owned(),
            count => format!("{count} analyses"),
        };
        let meta = format!(
            "{} \u{00b7} {} \u{00b7} {readers}",
            source.quantity(),
            source.summary()
        );
        let position = state
            .schematic
            .components
            .iter()
            .find(|component| component.id == source.component_id)
            .map(|component| component.pos);
        let response = nav_row_indented_mono_response(
            ui,
            WorkbenchIcon::ArrowRight,
            &source.reference,
            state.schematic.selection.has_component(source.component_id),
            Some(&meta),
            1,
        )
        .on_hover_text(excitation_tooltip(&source));
        if response.clicked() {
            state
                .schematic
                .selection
                .select_only_component(source.component_id);
            state.schematic.net_highlight.clear();
            state.schematic.center_request = position;
        }
    }
}

/// The full reading of one excitation: its terminals, and every analysis that
/// names it with the part it plays there.
fn excitation_tooltip(source: &crate::simulation::placed_sources::PlacedSource) -> String {
    let mut lines = vec![format!(
        "{} \u{00b7} {}",
        source.reference,
        source.summary()
    )];
    if !source.nets.is_empty() {
        lines.push(source.nets.join(" \u{2192} "));
    }
    if source.consumers.is_empty() {
        lines.push(
            "No analysis in the plan names this source, and none reads every source".to_owned(),
        );
    } else {
        for consumer in &source.consumers {
            lines.push(format!("{} \u{00b7} {}", consumer.analysis, consumer.role));
        }
    }
    lines.join("\n")
}

/// The saved probes this plan reads, which is what "named signal" means once
/// the Excitations section above owns every placed source.
///
/// This section used to list the sources too, so a drawing with eight sources
/// carried each of them twice in one rail -- once under Excitations with its
/// reader count, and once here saying only "source". The second listing said
/// strictly less than the first.
fn named_signal_section(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = normalized(app.state.workbench.navigator_filter());
    // A saved output belongs to the run plan rather than to a sheet, so the
    // sheet scope has nothing to say about it and it is listed either way.
    let probes = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .and_then(|plan| app.state.workspace.plan_data(plan.id()))
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

    navigator_section_header(ui, "Named signals", &probes.len().to_string());
    if probes.is_empty() {
        empty_navigator_row(
            ui,
            if query.is_empty() {
                "No saved probes"
            } else {
                "No probe matches this filter"
            },
        );
        return;
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
            let resolved = app.state.workspace.design_projection(
                &app.state.library_manager,
                &app.state.workspace.active_view,
                &app.state.schematic,
            );
            let projection = match resolved {
                Ok(projection) => projection,
                Err(error) => {
                    app.state
                        .push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                            "The probed net cannot be revealed: {error}"
                        )));
                    open_measurements(app);
                    return;
                }
            };
            let nets = crate::simulation::netlist_gen::projection_nets(
                &app.state.library_manager,
                &projection,
                &app.state.workspace.active_view.key(),
            );
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
        // Only an instance emits a deck card, so the row that is not one
        // never offers the jump rather than offering a permanently dead one.
        if matches!(object, NavigatorObject::Component { .. }) {
            let show_in_netlist = ui.add_enabled(
                Command::ShowInNetlist.is_enabled(app),
                egui::Button::new("Show in netlist"),
            );
            if show_in_netlist.clicked() {
                Command::ShowInNetlist.execute(app);
                ui.close();
            }
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
    let library_parts = library_part_rows(app, &query);
    let visible_matches = component_shelf_match_count(app, &query) + library_parts.len();
    let mut primitive = None;
    let mut builtin = None;
    let mut generated = None;
    let mut cell = None;
    let mut requested_part = None;
    ScrollArea::vertical()
        .id_salt("workbench.design.component_shelf")
        .show(ui, |ui| {
            primitive = pinned(ui, app).or_else(|| primitive_catalog(ui, app));
            builtin = builtin_xspice_catalog(ui, app);
            generated = generated_veriloga_catalog(ui, app);
            requested_part = library_parts_section(ui, app, &library_parts);
            cell = project_library(ui, app);
            if !query.is_empty() && visible_matches == 0 {
                empty_navigator_row(ui, "No component or cell matches this filter");
            }
        });
    if let Some(kind) = primitive {
        arm_primitive(app, kind, ui.ctx());
    } else if let Some(binding) = builtin {
        arm_cell(&mut app.state, binding, ui.ctx());
    } else if let Some(binding) = generated {
        arm_cell(&mut app.state, binding, ui.ctx());
    } else if let Some(binding) = cell {
        arm_cell(&mut app.state, binding, ui.ctx());
    } else if let Some(row) = requested_part {
        match row.action {
            LibraryPartAction::Arm(placement) => {
                arm_library_part(&mut app.state, *placement, ui.ctx());
            }
            LibraryPartAction::Review {
                pack_id, version, ..
            } => request_library_part(app, row.part_id, pack_id, version),
            // A refused row renders disabled, so its click never arrives.
            LibraryPartAction::Refused(_) => {}
        }
    }
}

/// One indexed part the shelf can offer, with what a click would do.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LibraryPartRow {
    part_id: String,
    /// The meta column: the part's device class, and where it stands.
    meta: String,
    action: LibraryPartAction,
}

/// Where one library-part click goes.
///
/// The same fork the Models workspace shelf decides in its `place` module: a
/// definition the project already holds is armed directly, a release it has
/// not adopted is reviewed first, and a part that cannot be drawn is refused
/// with the sentence the disabled row carries.
#[derive(Debug, Clone, PartialEq, Eq)]
enum LibraryPartAction {
    /// Arm now: the definition is already in this project's catalog —
    /// compiled in, or retained when the part was added.
    Arm(Box<PartPlacement>),
    /// Raise the pack confirmation in the Models workspace. Only right for a
    /// release that still needs installing or retaining; the hub operation
    /// arms the cursor when the retention lands.
    Review {
        pack_id: String,
        version: String,
        pack_name: String,
    },
    /// No placement exists for this part; the sentence the disabled control
    /// carries.
    Refused(String),
}

/// Every part the unified index can offer this shelf.
///
/// A part the project retained — and a compiled-in foundation part — is
/// listed here and arms directly. The "Project library" section below reads
/// the symbol-cell library, not the model-library manager, so this section is
/// the only door such a definition has onto the canvas. Pack releases the
/// project has not adopted sit beside them and go through the pack
/// confirmation instead.
fn library_part_rows(app: &RSpiceApp, query: &str) -> Vec<LibraryPartRow> {
    let libraries = app.state.model_library_manager.libraries_sorted();
    let index = app.model_hub.part_index(&libraries);
    shelf_rows(&index, &libraries, query)
}

/// Decides each indexed row's door, dropping the rows that have none.
///
/// Separated from [`library_part_rows`] so a test can feed it an index
/// without opening a hub over a store.
fn shelf_rows(
    index: &[ModelHubPartRow],
    libraries: &[&ModelLibrary],
    query: &str,
) -> Vec<LibraryPartRow> {
    /// Rows the shelf lists before asking the user to narrow the search.
    ///
    /// A published catalog is unbounded; a navigator column is not. The cap is
    /// a rendering decision, and the footer says when it bit.
    const SHELF_ROWS: usize = 200;

    // A part the project adopted is the retained row's to place: its
    // installed-pack row is dropped rather than offering to review and add
    // something that is already added — the same rule the Models workspace
    // shelf applies to its own two halves.
    let retained = index
        .iter()
        .filter(|row| matches!(row.provenance, PartProvenance::ProjectRetained { .. }))
        .map(|row| row.part_id.as_str())
        .collect::<HashSet<_>>();
    index
        .iter()
        .filter(|row| match &row.provenance {
            PartProvenance::InstalledPack { .. } => !retained.contains(row.part_id.as_str()),
            // A section-scoped subcircuit key cannot be referenced by its
            // bare name, so it is not a part a reader picks.
            PartProvenance::Foundation | PartProvenance::ProjectRetained { .. } => {
                !row.part_id.contains('\u{1f}')
            }
            PartProvenance::RemoteRelease { .. } => true,
        })
        .filter(|row| {
            // The third search field is the row's address: the pack for a
            // pack row, the holding library for a project one — the same
            // column the Models workspace shelf searches.
            let source = match &row.provenance {
                PartProvenance::InstalledPack { pack_id, .. }
                | PartProvenance::RemoteRelease { pack_id, .. } => pack_id.as_str(),
                PartProvenance::ProjectRetained { library } => library.as_str(),
                PartProvenance::Foundation => "",
            };
            matches_query(query, &[&row.part_id, &row.device, source])
        })
        .take(SHELF_ROWS)
        .filter_map(|row| library_part_row(libraries, row))
        .collect()
}

/// One shelf row, with the route its click takes.
fn library_part_row(libraries: &[&ModelLibrary], row: &ModelHubPartRow) -> Option<LibraryPartRow> {
    let (meta, action) = match &row.provenance {
        PartProvenance::Foundation | PartProvenance::ProjectRetained { .. } => {
            let holder = holding_library(libraries, &row.provenance, &row.part_id)?;
            let action = match plan_library_placement(holder, &row.part_id) {
                Ok(placement) => LibraryPartAction::Arm(Box::new(placement)),
                Err(reason) => LibraryPartAction::Refused(refusal_sentence(reason)),
            };
            let standing = if matches!(row.provenance, PartProvenance::Foundation) {
                "built in"
            } else {
                "in project"
            };
            (format!("{} · {standing}", row.device), action)
        }
        PartProvenance::InstalledPack { pack_id, version }
        | PartProvenance::RemoteRelease { pack_id, version } => {
            let pack_name = row.pack_name.clone().unwrap_or_else(|| pack_id.clone());
            let meta = match &row.state {
                PartState::Installed => format!("{} · installed", row.device),
                PartState::Available => format!("{} · available", row.device),
                PartState::UpdateAvailable { latest, .. } => {
                    format!("{} · update {latest}", row.device)
                }
                PartState::Incompatible { missing } => {
                    format!("{} · needs {}", row.device, missing.join(", "))
                }
            };
            let action = match &row.state {
                // The row stays searchable and stays readable; only the
                // action is refused, and the refusal says why in the same
                // words the pack manifest used.
                PartState::Incompatible { missing } => LibraryPartAction::Refused(format!(
                    "This build of RSpice does not offer {}, which {} requires.",
                    missing.join(", "),
                    pack_name
                )),
                _ => LibraryPartAction::Review {
                    pack_id: pack_id.clone(),
                    version: version.clone(),
                    pack_name,
                },
            };
            (meta, action)
        }
    };
    Some(LibraryPartRow {
        part_id: row.part_id.clone(),
        meta,
        action,
    })
}

/// The loaded library whose definition a foundation or retained row names.
///
/// Both come straight out of the same library set the index was built from in
/// this frame, so a miss means the row and the set disagree — the row is
/// dropped rather than offered against bytes that are not there.
fn holding_library<'a>(
    libraries: &[&'a ModelLibrary],
    provenance: &PartProvenance,
    part: &str,
) -> Option<&'a ModelLibrary> {
    libraries.iter().copied().find(|library| match provenance {
        PartProvenance::ProjectRetained { library: name } => library.name == *name,
        PartProvenance::Foundation => {
            matches!(library.source_authority, ModelSourceAuthority::BuiltIn)
                && (library.models.contains_key(part)
                    || library.top_level_models.contains_key(part)
                    || library.subcircuits.contains_key(part))
        }
        PartProvenance::InstalledPack { .. } | PartProvenance::RemoteRelease { .. } => false,
    })
}

/// The shelf section for the parts the unified model index lists.
fn library_parts_section(
    ui: &mut Ui,
    app: &RSpiceApp,
    rows: &[LibraryPartRow],
) -> Option<LibraryPartRow> {
    if rows.is_empty() {
        return None;
    }
    let query = normalized(&app.state.workbench.placement_query);
    let visible = if query.is_empty() {
        catalog_group_row(
            ui,
            "component-shelf-library-parts",
            WorkbenchIcon::Models,
            "Library parts",
            rows.len(),
        )
    } else {
        shelf_section_header(ui, "Library parts", Some(&rows.len().to_string()));
        true
    };
    if !visible {
        return None;
    }

    let mut requested = None;
    for row in rows {
        let placeable = !matches!(row.action, LibraryPartAction::Refused(_));
        let clicked = ui
            .add_enabled_ui(placeable, |ui| {
                let response = nav_row_indented_response(
                    ui,
                    WorkbenchIcon::Models,
                    &row.part_id,
                    false,
                    Some(&row.meta),
                    if query.is_empty() { 2 } else { 0 },
                );
                match &row.action {
                    // The row stays searchable and stays readable; only the
                    // action is refused, and the refusal says why.
                    LibraryPartAction::Refused(reason) => {
                        response.on_disabled_hover_text(reason.as_str());
                        false
                    }
                    LibraryPartAction::Arm(_) => response
                        .clone()
                        .on_hover_text(format!("Click to arm {}", row.part_id))
                        .clicked(),
                    LibraryPartAction::Review {
                        pack_name, version, ..
                    } => response
                        .clone()
                        .on_hover_text(format!(
                            "Review and add {} from {} {}",
                            row.part_id, pack_name, version
                        ))
                        .clicked(),
                }
            })
            .inner;
        if clicked {
            requested = Some(row.clone());
        }
    }
    requested
}

/// Arms the cursor with a definition the project already holds.
///
/// The same completion the Models workspace shelf performs when it arms a
/// held part, minus that workspace's operation receipt: this click happened
/// on the canvas side, so the toast and the canvas focus are the whole story.
fn arm_library_part(
    state: &mut crate::workbench::app_state::AppState,
    placement: PartPlacement,
    ctx: &egui::Context,
) {
    let armed = state.schematic.arm_pack_part(placement);
    crate::schematic::view::request_schematic_canvas_focus(ctx);
    state.ui.toasts.success(
        ctx,
        "Component placement armed",
        format!("{armed} will snap to the schematic grid."),
    );
}

/// Raises the pack confirmation for one shelf part.
///
/// The decision is shown in the Models workspace rather than over the canvas:
/// it commits the project to a licence, a download, and a capability claim,
/// and those are exactly what that workspace is for. The placement is armed
/// on the cursor when the install completes, so the round trip ends where the
/// user started.
fn request_library_part(app: &mut RSpiceApp, part_id: String, pack_id: String, version: String) {
    use crate::workbench::state::{ModelsPage, ModelsWorkbenchDialog, PackReleaseConfirmation};

    let Some(release) = PackReleaseConfirmation::for_release(
        &app.model_hub,
        &pack_id,
        &version,
        Some(part_id.clone()),
    ) else {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "The model hub no longer describes {pack_id} {version}, so '{part_id}' cannot \
                 be added."
            )));
        return;
    };
    app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmPack {
        pack_id,
        attach: true,
        release: Some(Box::new(release)),
    });
    crate::workbench::commands::vocabulary::Command::ModelsPage(ModelsPage::Models).execute(app);
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
    let builtin_matches = engine_only_xspice_devices()
        .iter()
        .filter(|descriptor| {
            matches_query(
                query,
                &[
                    descriptor.display_name,
                    descriptor.model_type,
                    descriptor.stable_id,
                ],
            )
        })
        .count();
    let generated_matches = generated_veriloga_devices()
        .iter()
        .filter(|descriptor| {
            matches_query(
                query,
                &[
                    descriptor.model_name,
                    descriptor.module_name,
                    descriptor.source_digest,
                ],
            )
        })
        .count();
    primitive_matches + builtin_matches + generated_matches + library_matches
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
        crate::workbench::commands::vocabulary::Command::PlaceInstance,
        crate::workbench::app_state::runtime_command_platform(ui.ctx()),
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

fn builtin_xspice_catalog(ui: &mut Ui, app: &mut RSpiceApp) -> Option<LibraryCellInstance> {
    let query = normalized(&app.state.workbench.placement_query);
    let descriptors = engine_only_xspice_devices()
        .iter()
        .filter(|descriptor| {
            matches_query(
                &query,
                &[
                    descriptor.display_name,
                    descriptor.model_type,
                    descriptor.stable_id,
                ],
            )
        })
        .collect::<Vec<_>>();
    if descriptors.is_empty() {
        return None;
    }

    let visible = if query.is_empty() {
        catalog_group_row(
            ui,
            "component-shelf-builtin-xspice",
            WorkbenchIcon::Design,
            "Built-in XSPICE",
            descriptors.len(),
        )
    } else {
        shelf_section_header(ui, "Built-in XSPICE", Some(&descriptors.len().to_string()));
        true
    };
    if !visible {
        return None;
    }

    let mut armed = None;
    for descriptor in descriptors {
        let selected = app
            .state
            .schematic
            .pending_library_cell
            .as_ref()
            .and_then(|binding| binding.builtin_xspice.as_ref())
            .is_some_and(|binding| binding.stable_id == descriptor.stable_id)
            && app.state.schematic.tool == Tool::Place(ComponentType::CellInstance);
        let response = schematic_nav_row_indented_drag_response(
            ui,
            WorkbenchIcon::Design,
            descriptor.display_name,
            selected,
            Some(descriptor.model_type),
            if query.is_empty() { 2 } else { 0 },
            false,
            false,
            false,
        );
        match builtin_xspice_library_binding(descriptor) {
            Ok(binding) => {
                response
                    .dnd_set_drag_payload(SchematicShelfDragPayload::library_cell(binding.clone()));
                if response.clicked() {
                    match builtin_xspice_vector_ports(descriptor) {
                        Ok(vector_ports)
                            if vector_ports.iter().any(|port| {
                                port.maximum.is_none_or(|maximum| maximum != port.minimum)
                            }) =>
                        {
                            app.state.dialogs.builtin_xspice_placement.open(
                                descriptor.stable_id,
                                descriptor.display_name,
                                vector_ports,
                                app.state.design_execution_epoch,
                                app.state.active_schematic_epoch,
                                app.state.workspace.active_view.display_path(),
                            );
                        }
                        Ok(_) => armed = Some(binding),
                        Err(error) => {
                            log::error!(
                                "Cannot configure {} in the XSPICE catalog: {error}",
                                descriptor.stable_id
                            );
                        }
                    }
                }
            }
            Err(error) => {
                log::error!(
                    "Cannot expose {} in the XSPICE catalog: {error}",
                    descriptor.stable_id
                );
            }
        }
    }
    armed
}

fn generated_veriloga_catalog(ui: &mut Ui, app: &RSpiceApp) -> Option<LibraryCellInstance> {
    let query = normalized(&app.state.workbench.placement_query);
    let descriptors = generated_veriloga_devices()
        .iter()
        .filter(|descriptor| {
            matches_query(
                &query,
                &[
                    descriptor.model_name,
                    descriptor.module_name,
                    descriptor.source_digest,
                ],
            )
        })
        .collect::<Vec<_>>();
    if descriptors.is_empty() {
        return None;
    }
    let visible = if query.is_empty() {
        catalog_group_row(
            ui,
            "component-shelf-generated-veriloga",
            WorkbenchIcon::Design,
            "Generated Verilog-A",
            descriptors.len(),
        )
    } else {
        shelf_section_header(
            ui,
            "Generated Verilog-A",
            Some(&descriptors.len().to_string()),
        );
        true
    };
    if !visible {
        return None;
    }

    let mut armed = None;
    for descriptor in descriptors {
        let selected = app
            .state
            .schematic
            .pending_library_cell
            .as_ref()
            .and_then(|binding| binding.generated_veriloga.as_ref())
            .is_some_and(|binding| binding.model_name == descriptor.model_name)
            && app.state.schematic.tool == Tool::Place(ComponentType::CellInstance);
        let response = schematic_nav_row_indented_drag_response(
            ui,
            WorkbenchIcon::Design,
            descriptor.model_name,
            selected,
            Some(&format!(
                "{} pin · {}",
                descriptor.terminals.len(),
                descriptor.module_name
            )),
            if query.is_empty() { 2 } else { 0 },
            false,
            false,
            false,
        );
        match generated_veriloga_library_binding(descriptor) {
            Ok(binding) => {
                response
                    .dnd_set_drag_payload(SchematicShelfDragPayload::library_cell(binding.clone()));
                if response.clicked() {
                    armed = Some(binding);
                }
            }
            Err(error) => log::error!(
                "Cannot expose generated Verilog-A model '{}': {error}",
                descriptor.model_name
            ),
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
    key: impl std::hash::Hash + std::fmt::Debug,
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
        crate::workbench::commands::vocabulary::Command::PlacePin.execute(app);
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

/// Arms one library cell for placement.
///
/// It takes the session state rather than the application because that is all
/// arming touches: a pending binding, a tool, a toast, and the canvas focus.
fn arm_cell(
    state: &mut crate::workbench::app_state::AppState,
    binding: LibraryCellInstance,
    ctx: &egui::Context,
) {
    let label = format!("{}/{}", binding.library, binding.cell);
    state.schematic.pending_library_cell = Some(binding);
    state
        .schematic
        .arm_tool(Tool::Place(ComponentType::CellInstance));
    state.ui.toasts.success(
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
