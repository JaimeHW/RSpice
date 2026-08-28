//! The design panel: the navigator rails, and the tab strip over them.
//!
//! Every rail here answers one question about the design in front of the
//! reader — its occurrences, its interface, its nets, what excites it, what it
//! saves — and each answers it about the *whole* design rather than the buffer
//! on screen, which is why the derived rails read a frozen projection and mark
//! the rows that stand somewhere else.
//!
//! The panel's other tab is [`shelf`], which answers the opposite question:
//! what could be added. The two share only how a query is folded
//! ([`normalized`]) and what it matches ([`matches_query`]).
//!
//! A section handler here takes [`AppState`] wherever it can. The ones that
//! keep the whole application are the ones that run commands — ascending the
//! hierarchy, and the object context menu, whose entries are the command
//! vocabulary's own.

use std::collections::{BTreeMap, HashSet};

use egui::{Key, Modifiers, Response, ScrollArea, Ui};

use crate::schematic::view::sheet_visibility::{self, SheetScope};
use crate::simulation::netlist_gen::DesignNet;
use crate::state::{PortDirection, SavedOutputKind};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;

use super::super::super::commands::vocabulary::Command;
use super::super::super::design_system::{PANEL_SECTION_H, PANEL_TABS_H, WorkbenchIcon};
use super::super::super::state::{DesignPanel, Workspace};
use super::rail::{self, RailDisclosure, RailFold};
use super::{
    SCHEMATIC_NAV_LABEL_SIZE, SCHEMATIC_NAV_META_SIZE, SCHEMATIC_NAV_ROW_HEIGHT,
    empty_navigator_row, panel_search, schematic_nav_row_indented_response,
};

mod hierarchy_tree;
mod shelf;

#[cfg(test)]
mod tests;

const PANEL_TABS_PADDING_X: f32 = 8.0;
/// Clearance between the location line and the sheet scope beneath it, so the
/// two read as one header block rather than two stacked rows.
const SCOPE_CONTROL_GAP: f32 = 6.0;

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

/// One object row of a rail that reads the whole design, with the occurrence
/// path in front of its reference.
///
/// The prefix is painted in the faint register the meta column uses and the
/// reference in the row's own, because they are two different claims: the
/// reference is the instance's name, and the path is where the run reaches it.
/// A row on the sheet in front of the reader carries no prefix at all — every
/// row would otherwise be `/`-prefixed to say nothing.
///
/// Its geometry is the navigator's own indented mono row, not a second
/// grammar: same height, same 33.5 px icon slot, same 47 px label column, same
/// right-aligned meta. What it adds is a second text run inside that one label
/// column, which the shared painter cannot express — it takes one string and
/// one colour.
struct OccurrenceObjectRow<'a> {
    icon: WorkbenchIcon,
    /// `/XAFE`, or `None` for a row of the occurrence being edited.
    occurrence: Option<&'a crate::state::InstancePath>,
    reference: &'a str,
    meta: &'a str,
    /// The meta states a condition rather than a count, so it is painted as
    /// one — the same flag the hierarchy tree's rows carry.
    alert: bool,
    selected: bool,
}

fn occurrence_object_row(ui: &mut Ui, row: OccurrenceObjectRow<'_>) -> Response {
    let t = Tokens::get(ui.ctx());
    // The design root already prints as `/`, so it is its own separator: a row
    // the reader reaches from inside a child master lists the root's supplies
    // as `/VDD` rather than `//VDD`.
    let prefix = row
        .occurrence
        .map(|occurrence| {
            let path = occurrence.to_string();
            if path.ends_with('/') {
                path
            } else {
                format!("{path}/")
            }
        })
        .unwrap_or_default();
    // The accessible name is the whole name: a screen reader given `V1` twice
    // is given no way to tell the two occurrences apart.
    let announced = format!("{prefix}{}", row.reference);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), SCHEMATIC_NAV_ROW_HEIGHT),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            row.selected,
            announced.clone(),
        )
    });
    if row.selected || response.hovered() {
        ui.painter().rect_filled(
            rect,
            0.0,
            if row.selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if row.selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }
    let indent = 14.0;
    row.icon.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 33.5 + indent, rect.center().y),
            egui::Vec2::splat(15.0),
        ),
        if row.selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    let meta_color = if row.alert {
        t.color.warn
    } else {
        t.color.text_faint
    };
    let font = theme::mono(SCHEMATIC_NAV_LABEL_SIZE, FontWeight::Regular);
    let meta_width = ui
        .painter()
        .layout_no_wrap(
            row.meta.to_owned(),
            theme::mono(SCHEMATIC_NAV_META_SIZE, FontWeight::Regular),
            meta_color,
        )
        .size()
        .x;
    let label_left = rect.left() + 47.0 + indent;
    let label_right = rect.right() - 14.0 - meta_width;
    let painter = ui.painter().with_clip_rect(egui::Rect::from_x_y_ranges(
        label_left..=label_right.max(label_left),
        rect.y_range(),
    ));
    let prefix_width = if prefix.is_empty() {
        0.0
    } else {
        painter
            .text(
                egui::pos2(label_left, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &prefix,
                font.clone(),
                t.color.text_faint,
            )
            .width()
    };
    painter.text(
        egui::pos2(label_left + prefix_width, rect.center().y),
        egui::Align2::LEFT_CENTER,
        row.reference,
        font,
        if row.selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    ui.painter().text(
        egui::pos2(rect.right() - 8.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        row.meta,
        theme::mono(SCHEMATIC_NAV_META_SIZE, FontWeight::Regular),
        meta_color,
    );
    theme::paint_focus_ring(ui, &response, rect);
    // The row draws itself at one level of indent, so it sits two levels under
    // the rail's band — the same place the nets and probes rails put theirs.
    rail::row(ui, &response, 2, None);
    response
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DesignNavigatorSection {
    Masters,
    Occurrences,
    Ports,
    /// Drawn only by a design that places one; see [`rf_port_section`].
    RfPorts,
    Nets,
    Excitations,
    NamedSignals,
}

impl DesignNavigatorSection {
    /// The band's title, in the one spelling the section has.
    ///
    /// [`navigator_section_header`] derives the persisted disclosure id from
    /// this string, so a respelled title is not a copy edit — it orphans the
    /// flag every reader's fold state is filed under and reopens the section
    /// on them.
    fn title(self) -> &'static str {
        match self {
            Self::Masters => "Masters",
            Self::Occurrences => "Occurrences",
            Self::Ports => "Ports",
            Self::RfPorts => "RF ports",
            Self::Nets => "Nets",
            Self::Excitations => "Excitations",
            Self::NamedSignals => "Named signals",
        }
    }
}

const DESIGN_NAVIGATOR_SECTION_ORDER: [DesignNavigatorSection; 7] = [
    DesignNavigatorSection::Masters,
    DesignNavigatorSection::Occurrences,
    DesignNavigatorSection::Ports,
    DesignNavigatorSection::RfPorts,
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
    tabs(ui, &mut app.state);
    match app.state.workbench.design_panel {
        DesignPanel::Navigator => navigator(ui, app),
        DesignPanel::ComponentShelf => shelf::component_shelf(ui, app),
    }
}

fn tabs(ui: &mut Ui, state: &mut AppState) {
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
        let selected = state.workbench.design_panel == panel;
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
            state.workbench.design_panel = panel;
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
                state.workbench.design_panel = entries[target].0;
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
    let enter_rows = navigator_search(ui, &mut app.state);
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

    rail::open(ui.ctx());
    let mut folded = None;
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
                    DesignNavigatorSection::RfPorts => rf_port_section(ui, app),
                    DesignNavigatorSection::Nets => net_section(ui, app),
                    DesignNavigatorSection::Excitations => excitation_section(ui, app),
                    DesignNavigatorSection::NamedSignals => named_signal_section(ui, app),
                }
            }
            // Inside the scroll area and below the last row: the traversal
            // needs the whole rail before it can answer, and the row it lands
            // on has to be scrolled to by the area it sits in.
            folded = rail::traverse(ui, enter_rows);
        });
    // Applied after the rail is painted, exactly as a press on the caret is:
    // every row above was laid out against the position the tree held when the
    // frame began.
    if let Some(node) = folded {
        let workspace = app.state.workbench.workspace;
        app.state
            .workbench
            .navigator_trees
            .for_workspace(workspace)
            .toggle(node);
    }
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

fn navigator_search(ui: &mut Ui, state: &mut crate::workbench::app_state::AppState) -> bool {
    let workspace = state.workbench.workspace;
    panel_search(
        ui,
        state.workbench.navigator_trees.filter_mut(workspace),
        "workbench.design.navigator.search",
        "Find instance, net or port…",
        &mut state.workbench.focus_navigator_search,
    )
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
            if navigator_section_header(ui, DesignNavigatorSection::Nets, "\u{2014}") {
                empty_navigator_row(ui, &error.to_string());
            }
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
    if !navigator_section_header(ui, DesignNavigatorSection::Nets, &nets.len().to_string()) {
        return;
    }
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
        let position = net_anchor(&app.state, &net);
        let selected = if navigator_net_selection_matches(&app.state, &net) {
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
                WorkbenchIcon::Supply
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
            select_navigator_design_net(&mut app.state, &net, position);
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

fn net_anchor(state: &AppState, net: &DesignNet) -> Option<crate::state::Point> {
    state
        .schematic
        .net_labels
        .iter()
        .find(|label| label.name.eq_ignore_ascii_case(&net.name))
        .map(|label| label.pos)
        .or_else(|| {
            state
                .schematic
                .wires
                .iter()
                .find(|wire| net.wire_ids.contains(&wire.id))
                .and_then(crate::state::Wire::start)
        })
        .or_else(|| {
            net.terminals.iter().find_map(|terminal| {
                state
                    .schematic
                    .components
                    .iter()
                    .find(|component| component.id == terminal.component_id)
                    .map(|component| component.pos)
            })
        })
}

/// One interface pin, as the ports rail lists it.
struct PortRow {
    component_id: u64,
    position: crate::state::Point,
    spec: crate::state::PortSpec,
    contract: crate::state::PortContract,
    /// Where the pin sits in the interface: the order the contract declares,
    /// or the document position that stands in for one until the typed editor
    /// rewrites it.
    order: usize,
    /// The document position, which breaks a tie between two pins declaring
    /// the same order.
    document_index: usize,
    /// Another pin of this cell carries the same name, folded for case.
    duplicated: bool,
}

impl PortRow {
    /// The meta column: the interface position, the direction declared there,
    /// and the conductors the pin carries when its name declares a vector.
    fn meta(&self) -> String {
        let mut meta = format!("#{} \u{00b7} {}", self.order, self.spec.direction.keyword());
        let width = self.spec.width();
        if width > 1 {
            meta.push_str(&format!(" \u{00b7} [{width}]"));
        }
        meta
    }

    /// The whole contract this pin declares, which the meta column has room
    /// for only the position and the direction of.
    fn tooltip(&self) -> String {
        let mut lines = vec![
            format!("{} \u{00b7} #{}", self.spec.name, self.order),
            format!(
                "{} \u{00b7} {} \u{00b7} {}",
                self.contract.direction.keyword(),
                self.contract.signal_type.keyword(),
                self.contract.discipline.keyword()
            ),
        ];
        let width = self.spec.width();
        if width > 1 {
            lines.push(format!("{width} conductors"));
        }
        let documentation = self.contract.documentation.trim();
        if !documentation.is_empty() {
            lines.push(documentation.to_owned());
        }
        if self.duplicated {
            lines.push(
                "This name is declared more than once, so the interface takes the first \
                 declaration and the rest add no pin"
                    .to_owned(),
            );
        }
        lines.join("\n")
    }
}

/// The pins the rail lists, in the order the interface declares them.
///
/// The sort key is `(netlist_order, document order)` — the key
/// [`crate::state::SchematicState::interface_ports`] sorts by — because that
/// order is the `.SUBCKT` port list and the node order of every instance of
/// this cell. A rail in document order would number the pins in an order no
/// deck has.
///
/// Duplicate names are counted over the whole cell rather than over the rows
/// that survive the scope and the filter: a repeated name is a fact about the
/// interface, and narrowing what is on screen does not un-declare it.
fn port_rows(
    state: &crate::workbench::app_state::AppState,
    scope: SheetScope,
    query: &str,
) -> Vec<PortRow> {
    let mut declared: BTreeMap<String, usize> = BTreeMap::new();
    for spec in state
        .schematic
        .components
        .iter()
        .filter_map(crate::state::Component::port_spec)
    {
        *declared.entry(spec.name.to_ascii_lowercase()).or_default() += 1;
    }
    let mut rows = state
        .schematic
        .components
        .iter()
        .enumerate()
        .filter_map(|(document_index, component)| {
            let spec = component.port_spec()?;
            let contract = component.port_contract()?;
            Some(PortRow {
                order: contract.netlist_order.unwrap_or(document_index + 1),
                duplicated: declared
                    .get(&spec.name.to_ascii_lowercase())
                    .is_some_and(|count| *count > 1),
                component_id: component.id,
                position: component.pos,
                document_index,
                spec,
                contract,
            })
        })
        .filter(|row| sheet_visibility::object_is_in_scope(state, scope, row.component_id))
        .filter(|row| {
            matches_query(
                query,
                &[
                    &row.spec.name,
                    row.spec.direction.keyword(),
                    &row.order.to_string(),
                    row.contract.discipline.keyword(),
                ],
            )
        })
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| (row.order, row.document_index));
    rows
}

fn port_section(ui: &mut Ui, app: &mut RSpiceApp) {
    let scope = sheet_visibility::sheet_scope(ui.ctx());
    let query = normalized(app.state.workbench.navigator_filter());
    let ports = port_rows(&app.state, scope, &query);
    if !navigator_section_header(ui, DesignNavigatorSection::Ports, &ports.len().to_string()) {
        return;
    }
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
    for port in ports {
        let icon = match port.spec.direction {
            PortDirection::In => WorkbenchIcon::ArrowRight,
            PortDirection::Out => WorkbenchIcon::ArrowLeft,
            PortDirection::Supply => WorkbenchIcon::Supply,
            PortDirection::InOut => WorkbenchIcon::Design,
        };
        let meta = port.meta();
        let response = hierarchy_tree::tree_row(
            ui,
            hierarchy_tree::TreeRow {
                id: ui.id().with(("navigator-port", port.component_id)),
                level: 1,
                disclosure: None,
                icon,
                label: port.spec.name.as_str(),
                mono: true,
                meta: Some(meta.as_str()),
                alert: port.duplicated,
                selected: app
                    .state
                    .schematic
                    .selection
                    .has_component(port.component_id),
            },
        )
        .row
        .on_hover_text(port.tooltip());
        if response.clicked() {
            app.state
                .schematic
                .selection
                .select_only_component(port.component_id);
            app.state.schematic.net_highlight.clear();
            app.state.schematic.center_request = Some(port.position);
        }
        navigator_object_context_menu(
            &response,
            app,
            NavigatorObject::Component {
                id: port.component_id,
                label: port.spec.name,
                position: port.position,
            },
        );
    }
}

// ------------------------------------------------- whole-design object rails

/// Every excitation the design places, joined from the two readings a rail owes
/// its reader.
///
/// - The occurrence in front of the reader is listed from the live editor
///   buffer, exactly as this rail always listed it. Uncommitted edits are in
///   it, and a cell view opened outside the configured hierarchy — a library
///   part being inspected — still has its own sources listed, which a
///   projection-only reading would take away.
/// - Every other occurrence comes from the frozen projection, tagged with the
///   path the run reaches it through. A source drawn inside a child master is
///   driven by the run, and this rail could not see it.
///
/// The active occurrence is dropped from the projection side rather than the
/// two readings deduplicated, because where two occurrences share the master
/// being edited only one of them is the sheet in front of the reader — and the
/// other is a row that has to stay.
fn whole_design_sources(app: &RSpiceApp) -> Vec<crate::simulation::placed_sources::PlacedSource> {
    let plan = app.state.sim_setup.analysis_plan.as_ref();
    let mut rows = crate::simulation::placed_sources::placed_sources(&app.state.schematic, plan);
    let Some(projection) = design_projection(app) else {
        return rows;
    };
    let active = app.state.workspace.occurrence_path();
    rows.extend(
        crate::simulation::placed_sources::design_sources(
            &app.state.library_manager,
            &projection,
            plan,
        )
        .into_iter()
        .filter(|source| source.occurrence.as_ref() != Some(&active)),
    );
    rows
}

/// The same two readings for the RF-port rail.
fn whole_design_rf_ports(app: &RSpiceApp) -> Vec<crate::simulation::placed_sources::PlacedRfPort> {
    let plan = app.state.sim_setup.analysis_plan.as_ref();
    let mut rows = crate::simulation::placed_sources::placed_rf_ports(&app.state.schematic, plan);
    let Some(projection) = design_projection(app) else {
        return rows;
    };
    let active = app.state.workspace.occurrence_path();
    rows.extend(
        crate::simulation::placed_sources::design_rf_ports(
            &app.state.library_manager,
            &projection,
            plan,
        )
        .into_iter()
        .filter(|port| port.occurrence.as_ref() != Some(&active)),
    );
    // Re-sorted across the join: an `.sp` run indexes the flattened design by
    // port number, so a port of a child master belongs beside the root's port
    // of the same number rather than after every one of them.
    rows.sort_by_key(|port| {
        (
            port.port_number,
            port.occurrence_label(),
            port.reference.to_ascii_uppercase(),
        )
    });
    rows
}

/// The frozen projection, or `None` when the configured design does not
/// resolve.
///
/// The rails that join it to the editor's buffer state no error of their own:
/// the Nets rail above already puts the reason in the reader's way, and every
/// object rail repeating it would spend the panel on one sentence.
fn design_projection(
    app: &RSpiceApp,
) -> Option<std::sync::Arc<crate::state::workspace::DesignProjection>> {
    app.state
        .workspace
        .design_projection(
            &app.state.library_manager,
            &app.state.workspace.active_view,
            &app.state.schematic,
        )
        .ok()
}

/// Whether a derived row is one the sheet scope can speak about.
///
/// Sheet scope narrows the active cell view to the sheet on screen. A row of
/// another occurrence is not on any sheet of that view, so it is outside the
/// control's authority by definition and stays listed at either position —
/// narrowing it away would make the scope control silently answer a question
/// about the hierarchy that it does not ask.
fn derived_row_is_in_scope(
    state: &crate::workbench::app_state::AppState,
    scope: SheetScope,
    occurrence: Option<&crate::state::InstancePath>,
    component_id: u64,
) -> bool {
    if occurrence.is_some_and(|occurrence| *occurrence != state.workspace.occurrence_path()) {
        return true;
    }
    sheet_visibility::object_is_in_scope(state, scope, component_id)
}

/// The occurrence a derived row is drawn in, when that is not the one on
/// screen. `None` means the row stands on the sheet in front of the reader.
fn row_is_elsewhere<'a>(
    state: &crate::workbench::app_state::AppState,
    occurrence: Option<&'a crate::state::InstancePath>,
) -> Option<&'a crate::state::InstancePath> {
    occurrence.filter(|occurrence| **occurrence != state.workspace.occurrence_path())
}

/// Land the session on the occurrence a row names, then select and centre the
/// instance it names there.
///
/// The descent is the hierarchy tree's own, so a click here and a click on the
/// occurrence row that owns it end in the same place: ascend to the shared
/// prefix, descend the rest, never part of the way. The selection is applied
/// only once the session is actually standing on that occurrence, because a
/// component id is unique inside one buffer and repeats across them — applying
/// it to a failed descent would select whatever the sheet on screen happened to
/// carry under that id.
///
/// The position is read after the descent rather than carried from the
/// projection: a materialized buffer translates its components onto per-sheet
/// offsets, so the projection's coordinates are not the ones the canvas is
/// about to be centred in.
fn descend_to_placed(
    state: &mut AppState,
    occurrence: &crate::state::InstancePath,
    component_id: u64,
) {
    hierarchy_tree::open_occurrence(state, occurrence);
    if state.workspace.occurrence_path() != *occurrence {
        return;
    }
    let Some(position) = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .map(|component| component.pos)
    else {
        return;
    };
    state
        .schematic
        .selection
        .select_only_component(component_id);
    state.schematic.net_highlight.clear();
    state.schematic.center_request = Some(position);
}

/// Every RF port the design places, in the order an S-parameter matrix indexes
/// them.
///
/// A rail of its own rather than rows among the interface pins above, because
/// the two answer different questions: an interface pin is a name the enclosing
/// deck binds by position, and an RF port is a Z0 termination an `.sp` run
/// addresses by number. One rail stating both would have to drop the number,
/// which is the only thing that tells two ports apart.
///
/// The band is absent from a design that places no port at all — not empty,
/// absent. Every other rail here answers about something every design has;
/// a permanent empty band would spend a row of a narrow rail, on every sheet
/// that is not an RF testbench, to say that a device the design never used is
/// still unused.
///
/// The list is resolved before the band is drawn, which is the one order that
/// can decide the question: whether a port is placed is a fact about the
/// design, and the scope control and the filter narrow what is *shown* rather
/// than what exists. A rail that asked the filtered list would vanish the
/// moment a reader typed a query that missed it.
fn rf_port_section(ui: &mut Ui, app: &mut RSpiceApp) {
    let ports = whole_design_rf_ports(app);
    if ports.is_empty() {
        return;
    }
    // Counted over every placed port for the same reason the ports rail counts
    // repeated pin names over the whole cell: two ports claiming one number is
    // a fact about the design, and narrowing the rail to one sheet does not
    // un-claim it.
    let collisions = crate::simulation::placed_sources::duplicate_port_numbers(&ports);
    let scope = sheet_visibility::sheet_scope(ui.ctx());
    let query = normalized(app.state.workbench.navigator_filter());
    let listed = ports
        .into_iter()
        .filter(|port| {
            derived_row_is_in_scope(
                &app.state,
                scope,
                port.occurrence.as_ref(),
                port.component_id,
            )
        })
        .filter(|port| {
            let number = port.port_number.to_string();
            let occurrence = port.occurrence_label();
            matches_query(
                &query,
                &[
                    port.reference.as_str(),
                    "rf",
                    "port",
                    number.as_str(),
                    port.z0.as_str(),
                    occurrence.as_str(),
                ],
            )
        })
        .collect::<Vec<_>>();

    if !navigator_section_header(
        ui,
        DesignNavigatorSection::RfPorts,
        &listed.len().to_string(),
    ) {
        return;
    }
    if listed.is_empty() {
        empty_navigator_row(
            ui,
            if query.is_empty() {
                "No RF ports on this sheet"
            } else {
                "No RF ports match this filter"
            },
        );
        return;
    }
    let mut descend = None;
    for port in listed {
        let collides = collisions.contains(&port.port_number);
        let meta = rf_port_meta(&port);
        let elsewhere = row_is_elsewhere(&app.state, port.occurrence.as_ref());
        let response = occurrence_object_row(
            ui,
            OccurrenceObjectRow {
                // A coaxial face rather than a signal direction: the interface
                // pins in the rail above take the arrows, and a port carries no
                // direction of its own — an `.sp` run drives and measures every
                // one of them.
                icon: WorkbenchIcon::Target,
                occurrence: elsewhere,
                reference: port.reference.as_str(),
                meta: meta.as_str(),
                alert: collides,
                selected: elsewhere.is_none()
                    && app
                        .state
                        .schematic
                        .selection
                        .has_component(port.component_id),
            },
        )
        .on_hover_text(rf_port_tooltip(&port, collides, elsewhere));
        if let Some(occurrence) = elsewhere {
            if response.clicked() {
                descend = Some((occurrence.clone(), port.component_id));
            }
            continue;
        }
        match placed_object(&app.state, port.component_id, &port.reference) {
            Some(object) => {
                if response.clicked() {
                    select_navigator_object(&mut app.state, &object);
                }
                navigator_object_context_menu(&response, app, object);
            }
            None => {
                if response.clicked() {
                    app.state
                        .schematic
                        .selection
                        .select_only_component(port.component_id);
                    app.state.schematic.net_highlight.clear();
                    app.state.schematic.center_request = None;
                }
            }
        }
    }
    // Applied after the rail is painted, because the descent replaces the
    // active document and every row above it was laid out against the one it
    // replaces.
    if let Some((occurrence, component_id)) = descend {
        descend_to_placed(&mut app.state, &occurrence, component_id);
    }
}

/// The meta column: the number an S-parameter run addresses this port by, the
/// impedance it presents, what it does behind that impedance, and who reads it.
///
/// `Z0 50` rather than `50 Ω`, and not because the unit is obvious: this column
/// is set in the bundled mono face, which carries no `Ω`, so the unit sign would
/// paint a missing-glyph box in the shipped app while every test here passed.
/// `Z0` is also the spelling [`crate::simulation::placed_sources::PlacedRfPort::summary`]
/// already carries into the studio's Excitations page, so the two surfaces name
/// the quantity the same way.
fn rf_port_meta(port: &crate::simulation::placed_sources::PlacedRfPort) -> String {
    // Run-scoped, exactly as the excitations rail counts it: a disabled
    // instance is not in the run this plan would dispatch.
    let reading: Vec<_> = port
        .consumers
        .iter()
        .filter(|consumer| consumer.reads())
        .collect();
    let readers = match reading.len() {
        // Stated rather than flagged, and in the studio page's own words. A
        // port no `.sp` run indexes is still terminating the design, which is
        // not the case the excitation rail's `no reader` was written for —
        // calling every termination in a time-domain testbench a finding is how
        // a rail stops being read.
        0 => "no S-parameter run".to_owned(),
        1 => reading[0].role.to_owned(),
        count => format!("{count} analyses"),
    };
    let mut meta = format!("#{}", port.port_number);
    if !port.z0.is_empty() {
        meta.push_str(&format!(" \u{00b7} Z0 {}", port.z0));
    }
    meta.push_str(&format!(
        " \u{00b7} {} \u{00b7} {readers}",
        port.mode.label()
    ));
    meta
}

/// The full reading of one RF port: what it presents, the terminals it sits
/// across, and every analysis that indexes it.
///
/// A collision is named last and names its own number, because the reader's
/// next action is to open the other port carrying it — and the meta column has
/// room to paint the row as a hazard but not to say what the hazard is.
fn rf_port_tooltip(
    port: &crate::simulation::placed_sources::PlacedRfPort,
    collides: bool,
    elsewhere: Option<&crate::state::InstancePath>,
) -> String {
    let mut lines = vec![format!("{} \u{00b7} {}", port.reference, port.summary())];
    if let Some(occurrence) = elsewhere {
        lines.push(format!("Drawn in {occurrence} \u{00b7} click to open it"));
    }
    if !port.nets.is_empty() {
        lines.push(port.nets.join(" \u{2192} "));
    }
    if port.consumers.is_empty() {
        lines.push("No S-parameter analysis in this plan reads this port".to_owned());
    } else {
        for consumer in &port.consumers {
            lines.push(format!(
                "{} \u{00b7} {}{}",
                consumer.analysis,
                consumer.role,
                if consumer.reads() {
                    ""
                } else {
                    " \u{00b7} disabled"
                }
            ));
        }
    }
    if collides {
        lines.push(format!(
            "Port number {} is claimed by more than one placed port, and an S-parameter run \
             addresses a port by its number",
            port.port_number
        ));
    }
    lines.join("\n")
}

/// Every excitation the design places, and what the plan reads each one as.
///
/// The row worth finding is the one with no reader: a source that is drawn,
/// will be netlisted, that no analysis in the plan names, and that none of the
/// plan's whole-design analyses reads either. Stating the reader count in the
/// row itself is what makes that visible without opening anything, so the count
/// is never elided.
///
/// The design rather than the sheet, because a run flattens the hierarchy: a
/// source drawn inside a child master drives this circuit and this rail could
/// not see it, so the root of every hierarchical design read as one that places
/// nothing. Those rows carry the occurrence path in front of the reference and
/// descend to it when clicked; see [`whole_design_sources`] for the two
/// readings they are joined from.
///
/// A source is an instance like any other row in these rails, so it carries the
/// same object menu: the rail that could select a source but not open, rename
/// or find it was the one rail whose rows answered to the pointer alone. The
/// menu is offered only on the rows of the occurrence being edited, exactly as
/// the hierarchy tree offers it only where its commands can act.
fn excitation_section(ui: &mut Ui, app: &mut RSpiceApp) {
    let scope = sheet_visibility::sheet_scope(ui.ctx());
    let query = normalized(app.state.workbench.navigator_filter());
    let sources = whole_design_sources(app)
        .into_iter()
        .filter(|source| {
            derived_row_is_in_scope(
                &app.state,
                scope,
                source.occurrence.as_ref(),
                source.component_id,
            )
        })
        .filter(|source| {
            let occurrence = source.occurrence_label();
            matches_query(
                &query,
                &[
                    source.reference.as_str(),
                    source.family,
                    source.key_figure.as_str(),
                    "excitation",
                    occurrence.as_str(),
                ],
            )
        })
        .collect::<Vec<_>>();

    if !navigator_section_header(
        ui,
        DesignNavigatorSection::Excitations,
        &sources.len().to_string(),
    ) {
        return;
    }
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
    let mut descend = None;
    for source in sources {
        // Run-scoped, exactly as the studio's Excitations page counts it: a
        // disabled instance is not in the run this plan would dispatch.
        let reading: Vec<_> = source.reading_consumers().collect();
        let readers = match reading.len() {
            0 => "no reader".to_owned(),
            1 => reading[0].role.to_owned(),
            count => format!("{count} analyses"),
        };
        let meta = format!(
            "{} \u{00b7} {} \u{00b7} {readers}",
            source.quantity(),
            source.summary()
        );
        let elsewhere = row_is_elsewhere(&app.state, source.occurrence.as_ref());
        let response = occurrence_object_row(
            ui,
            OccurrenceObjectRow {
                icon: WorkbenchIcon::ArrowRight,
                occurrence: elsewhere,
                reference: source.reference.as_str(),
                meta: meta.as_str(),
                alert: false,
                selected: elsewhere.is_none()
                    && app
                        .state
                        .schematic
                        .selection
                        .has_component(source.component_id),
            },
        )
        .on_hover_text(excitation_tooltip(&source, elsewhere));
        if let Some(occurrence) = elsewhere {
            if response.clicked() {
                descend = Some((occurrence.clone(), source.component_id));
            }
            continue;
        }
        match placed_object(&app.state, source.component_id, &source.reference) {
            Some(object) => {
                if response.clicked() {
                    select_navigator_object(&mut app.state, &object);
                }
                navigator_object_context_menu(&response, app, object);
            }
            None => {
                if response.clicked() {
                    app.state
                        .schematic
                        .selection
                        .select_only_component(source.component_id);
                    app.state.schematic.net_highlight.clear();
                    app.state.schematic.center_request = None;
                }
            }
        }
    }
    // Applied after the rail is painted; see the same line in
    // [`rf_port_section`].
    if let Some((occurrence, component_id)) = descend {
        descend_to_placed(&mut app.state, &occurrence, component_id);
    }
}

/// The design object one derived row — a source, an RF port — stands for.
///
/// Every command the shared menu carries acts on a placed object, so a row
/// whose instance the sheet no longer holds stands for nothing: it is still
/// listed, still selectable, and offered no menu rather than a menu of dead
/// entries.
///
/// Asked by component id rather than by the row that carries it, because the
/// rails that need this answer read different derivations and the question is
/// the same one every time: a row states a reference, and the menu acts on the
/// instance that reference was taken from.
fn placed_object(
    state: &crate::workbench::app_state::AppState,
    component_id: u64,
    reference: &str,
) -> Option<NavigatorObject> {
    let position = state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .map(|component| component.pos)?;
    Some(NavigatorObject::Component {
        id: component_id,
        label: reference.to_owned(),
        position,
    })
}

/// The full reading of one excitation: where it is drawn, its terminals, and
/// every analysis that names it with the part it plays there.
fn excitation_tooltip(
    source: &crate::simulation::placed_sources::PlacedSource,
    elsewhere: Option<&crate::state::InstancePath>,
) -> String {
    let mut lines = vec![format!(
        "{} \u{00b7} {}",
        source.reference,
        source.summary()
    )];
    if let Some(occurrence) = elsewhere {
        lines.push(format!("Drawn in {occurrence} \u{00b7} click to open it"));
    }
    if !source.nets.is_empty() {
        lines.push(source.nets.join(" \u{2192} "));
    }
    if source.consumers.is_empty() {
        lines.push(
            "No analysis in the plan names this source, and none reads every source".to_owned(),
        );
    } else {
        for consumer in &source.consumers {
            lines.push(format!(
                "{} \u{00b7} {}{}",
                consumer.analysis,
                consumer.role,
                if consumer.reads() {
                    ""
                } else {
                    " \u{00b7} disabled"
                }
            ));
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

    if !navigator_section_header(
        ui,
        DesignNavigatorSection::NamedSignals,
        &probes.len().to_string(),
    ) {
        return;
    }
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
            reveal_probe_expression(&mut app.state, &expression);
        }
        navigator_object_context_menu(
            &response,
            app,
            NavigatorObject::SavedOutput { name, expression },
        );
    }
}

fn reveal_probe_expression(state: &mut AppState, expression: &str) {
    let Some(target) = raw_probe_target(expression) else {
        open_measurements(state);
        return;
    };

    match target {
        RawProbeTarget::Current(component_name) => {
            let Some(component) = state
                .schematic
                .components
                .iter()
                .find(|component| component.name.eq_ignore_ascii_case(component_name))
            else {
                open_measurements(state);
                return;
            };
            let id = component.id;
            let position = component.pos;
            state.schematic.selection.select_only_component(id);
            state.schematic.net_highlight.clear();
            state.schematic.center_request = Some(position);
        }
        RawProbeTarget::Voltage { positive, negative } => {
            let resolved = state.workspace.design_projection(
                &state.library_manager,
                &state.workspace.active_view,
                &state.schematic,
            );
            let projection = match resolved {
                Ok(projection) => projection,
                Err(error) => {
                    state.push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                        "The probed net cannot be revealed: {error}"
                    )));
                    open_measurements(state);
                    return;
                }
            };
            let nets = crate::simulation::netlist_gen::projection_nets(
                &state.library_manager,
                &projection,
                &state.workspace.active_view.key(),
            );
            let requested = std::iter::once(positive)
                .chain(negative)
                .collect::<Vec<_>>();
            let Some(resolved) = requested
                .iter()
                .map(|name| nets.iter().find(|net| net.name.eq_ignore_ascii_case(name)))
                .collect::<Option<Vec<_>>>()
            else {
                open_measurements(state);
                return;
            };
            let position = resolved.iter().find_map(|net| net_anchor(state, net));
            if resolved.len() == 1 {
                select_navigator_design_net(state, resolved[0], position);
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
            select_navigator_net(state, &wire_ids, &component_ids, position);
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

    fn stable_path(&self, state: &AppState) -> String {
        let owner = state.workspace.active_display_path();
        match self {
            Self::Component { label, id, .. } => format!("{owner}/{label}#component-{id}"),
            Self::Net { name, .. } => format!("{owner}::net/{name}"),
            Self::SavedOutput { name, .. } => format!(
                "{}::saved-output/{name}",
                state
                    .sim_setup
                    .analysis_plan
                    .as_ref()
                    .map_or_else(|| "unbound-plan".to_owned(), |plan| plan.id().to_string())
            ),
        }
    }
}

fn select_navigator_object(state: &mut AppState, object: &NavigatorObject) {
    match object {
        NavigatorObject::Component { id, position, .. } => {
            state.schematic.selection.select_only_component(*id);
            state.schematic.net_highlight.clear();
            state.schematic.center_request = Some(*position);
        }
        NavigatorObject::Net {
            name,
            wire_ids,
            component_ids,
            position,
        } => {
            select_navigator_net(state, wire_ids, component_ids, *position);
            state
                .schematic
                .net_highlight
                .highlight_named_wires(name, wire_ids.iter().copied().collect());
        }
        NavigatorObject::SavedOutput { expression, .. } => {
            reveal_probe_expression(state, expression);
        }
    }
}

fn select_navigator_design_net(
    state: &mut AppState,
    net: &DesignNet,
    position: Option<crate::state::Point>,
) {
    let component_ids = net
        .terminals
        .iter()
        .map(|terminal| terminal.component_id)
        .collect::<Vec<_>>();
    select_navigator_net(state, &net.wire_ids, &component_ids, position);
    state
        .schematic
        .net_highlight
        .highlight_named_wires(&net.name, net.wire_ids.iter().copied().collect());
}

fn select_navigator_net(
    state: &mut AppState,
    wire_ids: &[u64],
    component_ids: &[u64],
    position: Option<crate::state::Point>,
) {
    state.schematic.selection.clear();
    for wire_id in wire_ids {
        state.schematic.selection.select_wire(*wire_id);
    }
    if wire_ids.is_empty() {
        for component_id in component_ids {
            state.schematic.selection.select_component(*component_id);
        }
    }
    state
        .schematic
        .net_highlight
        .highlight_wires(wire_ids.iter().copied().collect());
    state.schematic.center_request = position;
}

fn sorted_unique(values: impl IntoIterator<Item = u64>) -> Vec<u64> {
    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort_unstable();
    values.dedup();
    values
}

fn navigator_net_selection_matches(state: &AppState, net: &DesignNet) -> bool {
    if !state
        .schematic
        .net_highlight
        .selected_net_name
        .as_deref()
        .is_some_and(|name| name.eq_ignore_ascii_case(&net.name))
        || state.schematic.net_highlight.highlighted_wires != net.wire_ids.iter().copied().collect()
    {
        return false;
    }

    let wire_ids = sorted_unique(net.wire_ids.iter().copied());
    let component_ids = sorted_unique(net.terminals.iter().map(|terminal| terminal.component_id));
    let concrete = &state.schematic.selection;
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
    let (popup, keyboard_open) = super::row_context_menu(response);
    if response.secondary_clicked() || keyboard_open {
        select_navigator_object(&mut app.state, &object);
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
            select_navigator_object(&mut app.state, &object);
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
            ui.ctx().copy_text(object.stable_path(&app.state));
            ui.close();
        }
        if ui.button("Find references and consumers…").clicked() {
            find_navigator_object_references(&mut app.state, &object);
            ui.close();
        }
    });
}

/// Raise the navigator's search over the object the row names, already
/// filtering by it. The workspace is activated first because the route
/// transition restores that workspace's saved dock layout.
fn find_navigator_object_references(state: &mut AppState, object: &NavigatorObject) {
    let name = match object {
        NavigatorObject::Component { label: name, .. }
        | NavigatorObject::Net { name, .. }
        | NavigatorObject::SavedOutput { name, .. } => name.clone(),
    };
    state.workbench.activate(Workspace::Design);
    *state
        .workbench
        .navigator_trees
        .filter_mut(Workspace::Design) = name;
    state.workbench.navigator_visible = true;
    state.workbench.design_panel = DesignPanel::Navigator;
    state.workbench.focus_navigator_search = true;
}

fn open_measurements(state: &mut AppState) {
    state.workbench.console_page = super::super::super::state::ConsolePage::Measurements;
    state.workbench.console_visible = true;
    state.workbench.console_maximized = false;
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

/// One rail's band: what the section is called, how many rows it stands over,
/// and whether those rows are painted at all. Returns the disclosure position.
///
/// The caret is the control, so it states the position it is in rather than
/// pointing down over a section that cannot fold. A section is open until a
/// reader folds it, and the position is held per title so a rail folded in one
/// frame is folded in the next.
///
/// The count is the section's own and is stated whether or not the section is
/// open: a folded rail that hid how much it holds would be a worse answer than
/// the rows it replaced.
///
/// `egui::CollapsingHeader` is refused here for the same reason the shelf's
/// own group row refuses it — its stock geometry is not this band.
fn navigator_section_header(ui: &mut Ui, section: DesignNavigatorSection, count: &str) -> bool {
    let title = section.title();
    let t = Tokens::get(ui.ctx());
    let id = ui.make_persistent_id(("navigator-section", title));
    let mut open = ui.data_mut(|data| data.get_persisted::<bool>(id).unwrap_or(true));
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), PANEL_SECTION_H),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), title)
    });
    // Before the caret is drawn, so the band paints the position this frame
    // returns rather than the one the press just left.
    if response.clicked() {
        open = !open;
        ui.data_mut(|data| data.insert_persisted(id, open));
    }
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_expanded(open);
    });
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
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    let caret = if open {
        WorkbenchIcon::ChevronDown
    } else {
        WorkbenchIcon::ChevronRight
    };
    caret.paint(
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
    theme::paint_focus_ring(ui, &response, rect);
    // The band is the rail's own root row: Left climbs out to it from every
    // row below, and folds it once there.
    rail::row(
        ui,
        &response,
        0,
        Some(RailDisclosure {
            unfolded: open,
            fold: RailFold::Persisted(id),
        }),
    );
    open
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
