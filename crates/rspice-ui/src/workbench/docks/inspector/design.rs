//! Selection-dispatched design inspector.
//!
//! The right panel inspects exactly one object class at a time, chosen from
//! the schematic selection: an instance, a conductor, a documentation
//! object, the whole selected set, or — when nothing is selected — the sheet
//! itself. Every row is backed by live document state; no panel narrates a
//! fact the open design cannot supply.

mod component;

pub(crate) use component::apply_bound_model_choice;

use component::*;

use std::collections::{HashMap, HashSet};

use egui::{Color32, RichText, Ui};

use crate::schematic::view::{
    SchematicSymbolContext,
    drawing_sheet::{
        ActiveDrawingSheet, DrawingSheetOverflowSummary, drawing_sheet_overflow_summary,
    },
};
use crate::services::drc::{DrcLocation, DrcSeverity, DrcViolation};
use crate::simulation::netlist_gen::{DesignNet, HierarchySource, NetClass};
use crate::state::{
    AnalysisResultPayload, CellViewRef, Component, ComponentType, DisplayMode,
    DrawingSheetBorderTemplate, DrawingSheetInheritance, DrawingSheetTitleBlockAnchor,
    DrawingSheetTitleBlockTemplate, NetGraph, NetNamingPolicy, PropertyDefinition, PropertyType,
    PropertyValue, SoaRuleVerdictEvidence,
};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, TreeRow, TreeRowResult};
use crate::workbench::commands::CommandAvailability;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::{
    StatusMark, WorkbenchIcon, property_row_combo, property_row_input_action,
    property_row_input_with_hint, schematic_property_row as property_row,
    schematic_property_row_status as property_row_status,
};
use crate::workbench::state::{InlineEditField, ModelsPage, VerificationPage, Workspace};
use crate::workbench::{AppState, RSpiceApp};

use super::{
    ComponentModelEvidence, component_model_evidence, muted_inspector_copy,
    schematic_section_header as section_header, schematic_section_header_action, section_block_gap,
};

/// The mockup's `.inspector-hero` is an 82 px band: an 82 px square symbol
/// stage beside four stacked text lines.
const HERO_H: f32 = 82.0;
const HERO_PREVIEW_W: f32 = 82.0;
/// Vertical centers of the eyebrow, title, subtitle, and status lines.
const HERO_BASELINES: [f32; 4] = [12.0, 31.0, 49.0, 68.0];

// =============================================================================
// Subject resolution
// =============================================================================

/// What the design inspector is inspecting this frame.
///
/// Resolution is most-specific-first: one object of a known class beats the
/// set, and the set beats the sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DesignSubject {
    /// Exactly one component instance.
    Component(u64),
    /// Exactly one non-electrical design note.
    Note(u64),
    /// Exactly one non-electrical documentation shape.
    Shape(u64),
    /// Exactly one retained schematic probe marker.
    Probe(u64),
    /// A conductor selection that resolves to exactly one named net.
    Net(String),
    /// Several objects, or conductors spanning several nets.
    Multi,
    /// Nothing is selected: the sheet is the object.
    Sheet,
}

/// Wire IDs implied by the selection, including wires that merely own a
/// selected segment or vertex.
fn selected_wire_ids(state: &AppState) -> Vec<u64> {
    let selection = &state.schematic.selection;
    let mut ids: Vec<u64> = selection.wires.iter().copied().collect();
    ids.extend(
        selection
            .wire_segments
            .iter()
            .map(|segment| segment.wire_id),
    );
    ids.extend(selection.wire_vertices.iter().map(|vertex| vertex.wire_id));
    ids.sort_unstable();
    ids.dedup();
    ids
}

/// `true` when the selection holds conductors only — wires, their segments
/// and vertices, and net labels — and at least one of them.
fn conductors_only(state: &AppState) -> bool {
    let selection = &state.schematic.selection;
    let empty = selection.wires.is_empty()
        && selection.wire_segments.is_empty()
        && selection.wire_vertices.is_empty()
        && selection.junctions.is_empty()
        && selection.net_labels.is_empty();
    !empty
        && selection.components.is_empty()
        && selection.buses.is_empty()
        && selection.bus_taps.is_empty()
        && selection.design_notes.is_empty()
        && selection.documentation_shapes.is_empty()
        && selection.probes.is_empty()
}

pub(super) fn subject(state: &AppState, nets: &[DesignNet]) -> DesignSubject {
    let selection = &state.schematic.selection;
    if let Some(name) = navigator_selected_net_name(state, nets) {
        return DesignSubject::Net(name);
    }
    if let Some(id) = selection.single_component() {
        if let Some(port) = state
            .schematic
            .components
            .iter()
            .find(|component| component.id == id)
            .and_then(Component::port_spec)
            && nets
                .iter()
                .any(|net| net.name.eq_ignore_ascii_case(&port.name))
        {
            // Interface ports are named conductors with a direction contract,
            // not ordinary instances. Route them through the same net
            // inspector so identity, terminals, OP evidence and actions stay
            // consistent whether the user selected a wire, label, or port.
            return DesignSubject::Net(port.name);
        }
        return DesignSubject::Component(id);
    }
    if let Some(id) = selection.single_design_note() {
        return DesignSubject::Note(id);
    }
    if let Some(id) = selection.single_documentation_shape() {
        return DesignSubject::Shape(id);
    }
    if let Some(id) = selection.single_probe() {
        return DesignSubject::Probe(id);
    }
    if conductors_only(state)
        && let Some(name) = selected_net_name(state, nets)
    {
        return DesignSubject::Net(name);
    }
    if selection.is_empty() {
        return DesignSubject::Sheet;
    }
    DesignSubject::Multi
}

/// The Navigator can select a legal semantic net that has no drawn wire or
/// label. Accept that runtime authority only while both the current net
/// projection and the concrete schematic selection still match exactly.
fn navigator_selected_net_name(state: &AppState, nets: &[DesignNet]) -> Option<String> {
    let authority = state.schematic.net_highlight.selected_net_name.as_deref()?;
    let net = nets
        .iter()
        .find(|net| net.name.eq_ignore_ascii_case(authority))?;
    let mut wire_ids = net.wire_ids.clone();
    wire_ids.sort_unstable();
    wire_ids.dedup();
    let mut component_ids = net
        .terminals
        .iter()
        .map(|terminal| terminal.component_id)
        .collect::<Vec<_>>();
    component_ids.sort_unstable();
    component_ids.dedup();
    if state.schematic.net_highlight.highlighted_wires != wire_ids.iter().copied().collect() {
        return None;
    }

    let selection = &state.schematic.selection;
    let no_other_classes = selection.wire_segments.is_empty()
        && selection.wire_vertices.is_empty()
        && selection.junctions.is_empty()
        && selection.buses.is_empty()
        && selection.bus_taps.is_empty()
        && selection.net_labels.is_empty()
        && selection.design_notes.is_empty()
        && selection.documentation_shapes.is_empty()
        && selection.probes.is_empty();
    let exact_wires = selection.wires.iter().copied().collect::<HashSet<_>>()
        == wire_ids.iter().copied().collect();
    let exact_components = if wire_ids.is_empty() {
        selection.components.iter().copied().collect::<HashSet<_>>()
            == component_ids.iter().copied().collect()
    } else {
        selection.components.is_empty()
    };
    (no_other_classes && exact_wires && exact_components).then(|| net.name.clone())
}

/// The single net a conductor selection resolves to, or `None` when the
/// selection spans several nets or resolves to none.
fn selected_net_name(state: &AppState, nets: &[DesignNet]) -> Option<String> {
    let mut resolved: Option<String> = None;
    let mut accept = |candidate: &str| -> bool {
        match resolved.as_deref() {
            Some(existing) => existing.eq_ignore_ascii_case(candidate),
            None => {
                resolved = Some(candidate.to_owned());
                true
            }
        }
    };
    // A selected label names its net outright.
    for label in &state.schematic.net_labels {
        if state.schematic.selection.net_labels.contains(&label.id) && !accept(&label.name) {
            return None;
        }
    }
    for wire_id in selected_wire_ids(state) {
        let Some(net) = nets.iter().find(|net| net.wire_ids.contains(&wire_id)) else {
            continue;
        };
        if !accept(&net.name) {
            return None;
        }
    }
    if !state.schematic.selection.junctions.is_empty() {
        let graph = NetGraph::build(&state.schematic.wires, &state.schematic.junctions);
        for junction in &state.schematic.selection.junctions {
            for wire_id in graph.find_connected_wires(junction.pos) {
                let Some(net) = nets.iter().find(|net| net.wire_ids.contains(&wire_id)) else {
                    continue;
                };
                if !accept(&net.name) {
                    return None;
                }
            }
        }
    }
    resolved
}

// =============================================================================
// Entry point
// =============================================================================

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let sheet = sheet_connectivity(&app.state);
    if let Some(reason) = sheet.unresolved.as_deref() {
        unresolved_projection_note(ui, reason);
    }
    let inspected = subject(&app.state, &sheet.nets);

    // An edit session belongs to one instance. If the selection moved on
    // while a field was still open, close it now so its keystrokes land in
    // the undo history as the single entry they were.
    let owner = match inspected {
        DesignSubject::Component(id) => Some(id),
        _ => None,
    };
    if let Some(before) = app.state.workbench.inline_edit.release_unless(owner) {
        app.state
            .schematic
            .commit_undo_from(before, "edit instance");
    }

    match inspected {
        DesignSubject::Component(id) => component_panel(ui, app, id, &sheet),
        DesignSubject::Net(name) => net_panel(ui, app, &name, &sheet.nets),
        DesignSubject::Note(id) => note_panel(ui, app, id),
        DesignSubject::Shape(id) => shape_panel(ui, app, id),
        DesignSubject::Probe(id) => probe_panel(ui, app, id),
        DesignSubject::Multi => multi_panel(ui, app),
        DesignSubject::Sheet => sheet_panel(ui, app, &sheet.nets),
    }
}

/// Connectivity for the open sheet as the configured design resolves it: its
/// nets, and every instance's declared terminals paired with the net each one
/// binds.
struct SheetConnectivity {
    nets: std::sync::Arc<Vec<DesignNet>>,
    /// Instance ID → declared terminals in symbol order, each with the net
    /// it binds or `None` when the pin is open.
    terminals: HashMap<u64, Vec<(String, Option<String>)>>,
    /// Why the design projection could not be built, when it could not.
    /// Present means the two collections above are empty by refusal rather
    /// than because the sheet has nothing on it.
    unresolved: Option<String>,
}

/// Resolve the open sheet's connectivity from the design projection.
///
/// The projection is memoized on the content it derives from, so reading it
/// per frame costs a digest rather than a hierarchy walk — and reading it is
/// what keeps the inspector's net names identical to the ones netlisting
/// emits. A version-keyed cache here would be wrong anyway: net identity
/// depends on instance values and port parameters, which do not advance the
/// schematic topology version.
fn sheet_connectivity(state: &AppState) -> SheetConnectivity {
    let projection = match state.workspace.design_projection(
        &state.library_manager,
        &state.workspace.active_view,
        &state.schematic,
    ) {
        Ok(projection) => projection,
        Err(error) => {
            return SheetConnectivity {
                nets: std::sync::Arc::new(Vec::new()),
                terminals: HashMap::new(),
                unresolved: Some(error.to_string()),
            };
        }
    };
    let hierarchy = HierarchySource::from_design_projection(&state.library_manager, &projection);
    let nets = crate::simulation::netlist_gen::projection_nets(
        &state.library_manager,
        &projection,
        &state.workspace.active_view.key(),
    );

    let mut bound: HashMap<(u64, &str), &str> = HashMap::new();
    for net in nets.iter() {
        let is_isolated_terminal = net.terminals.len() == 1
            && net.wire_ids.is_empty()
            && net.port.is_none()
            && !net.authored_name;
        if is_isolated_terminal {
            continue;
        }
        for terminal in &net.terminals {
            bound.insert(
                (terminal.component_id, terminal.pin.as_str()),
                net.name.as_str(),
            );
        }
    }

    let terminals = state
        .schematic
        .components
        .iter()
        .map(|component| {
            let resolved = component
                .library_cell
                .as_ref()
                .and_then(|binding| hierarchy.resolved_symbol_for(binding));
            let pins = component
                .terminal_positions_resolved(resolved.as_ref())
                .into_iter()
                .map(|(pin, _position)| {
                    let net = bound
                        .get(&(component.id, pin.as_str()))
                        .map(|name| (*name).to_owned());
                    (pin, net)
                })
                .collect();
            (component.id, pins)
        })
        .collect();

    SheetConnectivity {
        nets,
        terminals,
        unresolved: None,
    }
}

/// State the reason the configured design did not resolve where the sheet's
/// connectivity rows would have been. An inspector that silently showed the
/// editor buffer's nets instead would name conductors the run will not have.
fn unresolved_projection_note(ui: &mut Ui, reason: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    ui.add(
        egui::Label::new(
            RichText::new(reason)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.warn),
        )
        .wrap(),
    );
    ui.add_space(6.0);
}

// =============================================================================
// Hero
// =============================================================================

enum HeroPreview {
    Symbol(ComponentType),
    Icon(WorkbenchIcon),
}

struct Hero {
    preview: HeroPreview,
    eyebrow: String,
    title: String,
    subtitle: String,
    statuses: Vec<(String, Color32)>,
    /// Component whose property dialog opens on double-click.
    open_properties: Option<u64>,
}

fn hero(ui: &mut Ui, app: &mut RSpiceApp, spec: Hero) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), HERO_H),
        if spec.open_properties.is_some() {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        },
    );
    let preview = egui::Rect::from_min_max(
        rect.min,
        egui::pos2(
            (rect.left() + HERO_PREVIEW_W).min(rect.right()),
            rect.bottom(),
        ),
    );
    ui.painter().rect_filled(preview, 0.0, t.color.canvas_bg);
    ui.painter().vline(
        preview.right(),
        preview.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );

    match spec.preview {
        HeroPreview::Symbol(kind) => crate::schematic::view::draw_symbol_preview(
            ui.painter(),
            preview.shrink(12.0),
            kind,
            t.color.symbol,
            app.symbol_library.as_ref(),
        ),
        HeroPreview::Icon(icon) => {
            icon.paint(ui.painter(), preview.shrink(28.0), t.color.text_faint);
        }
    }

    let text_left = preview.right() + 10.0;
    let text_clip = egui::Rect::from_x_y_ranges(text_left..=(rect.right() - 10.0), rect.y_range());
    let painter = ui.painter().with_clip_rect(text_clip);
    let at = |index: usize| egui::pos2(text_left, rect.top() + HERO_BASELINES[index]);
    let eyebrow_job = egui::text::LayoutJob::single_section(
        spec.eyebrow.to_uppercase(),
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_1, FontWeight::Medium),
            color: t.color.text_faint,
            extra_letter_spacing: 0.09 * tokens::FS_1,
            ..Default::default()
        },
    );
    let eyebrow = ui.fonts_mut(|fonts| fonts.layout_job(eyebrow_job));
    painter.galley(
        egui::pos2(at(0).x, at(0).y - eyebrow.size().y * 0.5),
        eyebrow,
        t.color.text_faint,
    );
    painter.text(
        at(1),
        egui::Align2::LEFT_CENTER,
        &spec.title,
        theme::sans(tokens::FS_2, FontWeight::SemiBold),
        t.color.text,
    );
    painter.text(
        at(2),
        egui::Align2::LEFT_CENTER,
        &spec.subtitle,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    let status_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let mut status_x = text_left;
    for (label, tone) in &spec.statuses {
        let galley = painter.layout_no_wrap(label.clone(), status_font.clone(), *tone);
        let width = galley.size().x;
        painter.galley(
            egui::pos2(status_x, at(3).y - galley.size().y * 0.5),
            galley,
            *tone,
        );
        status_x += width + 4.0;
    }

    if let Some(component_id) = spec.open_properties {
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                "Open selected component properties",
            )
        });
        if response.double_clicked() && !app.state.schematic_edit_read_only() {
            crate::workbench::app::open_property_editor(&mut app.state, component_id);
        }
        theme::paint_focus_ring(ui, &response, rect);
        response.on_hover_text("Double-click to edit component properties");
    }
}

// =============================================================================
// Shared section pieces
// =============================================================================

/// The section body's action row: content-sized buttons that wrap, matching
/// the mockup's `.section-body.panel-action-stack` box and the design
/// system's own rule that an action row is a group, not a run-on.
///
/// The row takes the section's own step from the block above it and leaves
/// the closing step to the section, so a body that ends in buttons is framed
/// exactly like one that ends in a property list.
const ACTION_ROW_PAD_X: f32 = 10.0;
const ACTION_ROW_GAP: f32 = 6.0;

fn action_stack(ui: &mut Ui, body: impl FnOnce(&mut Ui)) {
    let width = (ui.available_width() - 2.0 * ACTION_ROW_PAD_X).max(1.0);
    section_block_gap(ui);
    ui.horizontal(|ui| {
        ui.add_space(ACTION_ROW_PAD_X);
        ui.allocate_ui_with_layout(
            egui::vec2(width, 0.0),
            egui::Layout::left_to_right(egui::Align::Center).with_main_wrap(true),
            |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(ACTION_ROW_GAP, ACTION_ROW_GAP);
                body(ui);
            },
        );
    });
}

/// One action bound to a workbench command, disabled with the registry's
/// exact reason when the command cannot run.
fn command_action(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    command: Command,
    icon: Icon,
    label: &str,
    destructive: bool,
) {
    let availability = command.availability(app);
    let response = Button::new(label)
        .icon(icon)
        .destructive(destructive)
        .enabled(availability.is_available())
        .show(ui);
    if let CommandAvailability::Disabled(reason) = availability {
        response.on_disabled_hover_text(reason);
    } else if response.clicked() {
        command.execute(app);
    }
}

fn tone_for(ui: &Ui, ok: bool) -> Color32 {
    let t = Tokens::get(ui.ctx());
    if ok { t.color.ok } else { t.color.warn }
}

fn mark_for(ok: bool) -> StatusMark {
    if ok {
        StatusMark::Success
    } else {
        StatusMark::Warning
    }
}

/// `true` when schematic checks were run against the drawing as it stands.
fn checks_current(state: &AppState) -> bool {
    state.dialogs.drc_results.is_some()
        && state.dialogs.drc_checked_version == state.schematic.topology_version()
}

fn checks_status(state: &AppState) -> String {
    if !checks_current(state) {
        return "stale".to_owned();
    }
    let blocking = state.dialogs.drc_results.as_ref().map_or(0, |result| {
        let summary = result.summary();
        summary.critical + summary.errors
    });
    format!("{blocking} errors")
}

/// The dirty bit of the open cell view.
fn active_view_dirty(state: &AppState) -> bool {
    state
        .workspace
        .open_views
        .iter()
        .find(|open| open.reference == state.workspace.active_view)
        .is_some_and(|open| open.dirty)
}

// =============================================================================
// Inline instance editing lives in `design/inline_instance.rs`.
mod inline_instance;
use inline_instance::*;

// Sheet inspector
// =============================================================================

fn sheet_panel(ui: &mut Ui, app: &mut RSpiceApp, nets: &[DesignNet]) {
    let reference = app.state.workspace.active_view.clone();
    let read_only = app.state.schematic_edit_read_only();
    let current = checks_current(&app.state);
    let dirty = active_view_dirty(&app.state);
    let depth = app.state.workspace.hierarchy_stack.len().saturating_sub(1);
    let child_view_count = app
        .state
        .schematic
        .components
        .iter()
        .filter(|component| {
            app.state
                .hierarchy_master_for_component(component.id)
                .is_some()
        })
        .count();
    let open_pins = current_violation_count(
        &app.state,
        crate::services::drc::DrcViolationType::UnconnectedPin,
    );
    let floating_nets = current_violation_count(
        &app.state,
        crate::services::drc::DrcViolationType::FloatingNode,
    );

    hero(
        ui,
        app,
        Hero {
            preview: HeroPreview::Icon(WorkbenchIcon::Design),
            eyebrow: format!(
                "SHEET · {} / {}",
                reference.library.to_ascii_uppercase(),
                reference.cell.to_ascii_uppercase()
            ),
            title: format!("{} · {}", reference.cell, reference.view),
            subtitle: if read_only {
                "read-only document".to_owned()
            } else if depth == 0 {
                "root sheet of the open design".to_owned()
            } else {
                format!("descended sheet · depth {depth}")
            },
            statuses: vec![
                (
                    if dirty { "unsaved" } else { "saved" }.to_owned(),
                    tone_for(ui, !dirty),
                ),
                (
                    if current {
                        "checks current"
                    } else {
                        "checks stale"
                    }
                    .to_owned(),
                    tone_for(ui, current),
                ),
            ],
            open_properties: None,
        },
    );

    section_header(ui, "Sheet", None);
    property_row(
        ui,
        "Library / cell",
        &format!("{}/{}", reference.library, reference.cell),
    );
    property_row(ui, "View", &reference.view);
    property_row(
        ui,
        "Grid / snap",
        &format!(
            "{} · snap {}",
            schematic_grid_label(app.state.schematic.document_policy.grid_pitch),
            if app.state.schematic.snap_engine.enabled {
                "on"
            } else {
                "off"
            }
        ),
    );
    property_row(
        ui,
        "Hierarchy",
        &if depth == 0 {
            format!(
                "root · {child_view_count} child view{}",
                if child_view_count == 1 { "" } else { "s" }
            )
        } else {
            format!(
                "{} · depth {depth}",
                app.state.workspace.active_display_path()
            )
        },
    );
    property_row(
        ui,
        "Working revision",
        &app.state.workspace.project.revision().get().to_string(),
    );

    drawing_sheet_inspector(ui, app);

    section_header(ui, "Contents", None);
    property_row(
        ui,
        "Instances",
        &app.state.schematic.components.len().to_string(),
    );
    property_row(ui, "Nets", &nets.len().to_string());
    property_row(
        ui,
        "Interface ports",
        &match app.state.schematic.interface_ports().len() {
            0 => "— testbench root".to_owned(),
            count => count.to_string(),
        },
    );
    let probes = app.state.schematic.probes.len();
    property_row(
        ui,
        "Probes",
        &if probes == 0 {
            "none placed".to_owned()
        } else {
            probes.to_string()
        },
    );

    section_header(ui, "Checks", Some(&checks_status(&app.state)));
    let connectivity_ok = current && open_pins == 0;
    property_row_status(
        ui,
        "Connectivity",
        &if !current {
            "pending recheck".to_owned()
        } else if open_pins == 0 {
            "all pins bound".to_owned()
        } else {
            format!(
                "{open_pins} unconnected pin{}",
                if open_pins == 1 { "" } else { "s" }
            )
        },
        tone_for(ui, connectivity_ok),
        mark_for(connectivity_ok),
    );
    let floating_ok = current && floating_nets == 0;
    property_row_status(
        ui,
        "Floating nets",
        &if !current {
            "pending recheck".to_owned()
        } else if floating_nets == 0 {
            "none".to_owned()
        } else {
            floating_nets.to_string()
        },
        tone_for(ui, floating_ok),
        mark_for(floating_ok),
    );
    let topology = app.state.schematic.topology_version();
    property_row_status(
        ui,
        "Last checked",
        &if current {
            format!("topology revision {topology}")
        } else {
            format!("rerun for topology revision {topology}")
        },
        tone_for(ui, current),
        mark_for(current),
    );
    action_stack(ui, |ui| {
        command_action(
            ui,
            app,
            Command::RunChecks,
            Icon::Check,
            "Run schematic checks",
            false,
        );
    });

    section_header(ui, "Selection", Some("none"));
    muted_inspector_copy(
        ui,
        "Click an instance or conductor to inspect it. Drag for a marquee; Shift-click adds to the selection.",
    );
}

fn drawing_sheet_inspector(ui: &mut Ui, app: &mut RSpiceApp) {
    let sheet = ActiveDrawingSheet::resolve(&app.state);
    let symbol_context = SchematicSymbolContext::from_state(&app.state);
    let overflow = drawing_sheet_overflow_summary(&app.state, &symbol_context, &sheet);
    let source = drawing_sheet_source_label(sheet.format.inheritance);
    let physical = sheet.geometry.physical;
    let unit = sheet.format.display_unit;

    section_header(ui, "Drawing sheet", Some(source));
    property_row(ui, "Format", &sheet.format_label());
    property_row(
        ui,
        "Dimensions",
        &unit.format_size_um(physical.paper.width_um, physical.paper.height_um),
    );
    property_row(
        ui,
        "Printable area",
        &unit.format_size_um(physical.printable.width_um, physical.printable.height_um),
    );
    property_row(ui, "Margins", &drawing_sheet_margins_label(&sheet.format));
    property_row(ui, "Border", &drawing_sheet_border_label(&sheet));
    property_row(ui, "Title block", &drawing_sheet_title_block_label(&sheet));
    if sheet.format.title_block_substituted() {
        drawing_sheet_title_block_substitution(ui, &sheet);
    }
    property_row(
        ui,
        "Page",
        &format!("{} · {}", sheet.page_label, sheet.sheet_name),
    );

    let advisory_count = overflow.finding_count();
    property_row_status(
        ui,
        "Content",
        &if overflow.is_clear() {
            "inside the sheet".to_owned()
        } else {
            format!("{advisory_count} outside")
        },
        tone_for(ui, overflow.is_clear()),
        mark_for(overflow.is_clear()),
    );

    if !overflow.is_clear() {
        drawing_sheet_overflow_advisory(ui, app, &overflow);
    }

    if app.state.schematic_edit_read_only() {
        drawing_sheet_lock_note(
            ui,
            "The active schematic is read-only. The sheet format is shown in full and prints exactly as stated.",
        );
    } else {
        action_stack(ui, |ui| {
            let width = ui.available_width();
            ui.allocate_ui_with_layout(
                egui::vec2(width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.spacing_mut().item_spacing.y = ACTION_ROW_GAP;

                    let availability = Command::PageSetup.availability(app);
                    let response = Button::new("Page setup…")
                        .icon(Icon::File)
                        .min_width(width)
                        .enabled(availability.is_available())
                        .show(ui);
                    if let CommandAvailability::Disabled(reason) = availability {
                        response.on_disabled_hover_text(reason);
                    } else if response.clicked() {
                        Command::PageSetup.execute(app);
                    }

                    let availability = Command::SheetFormatManager.availability(app);
                    let response = Button::new("All sheet formats…")
                        .icon(Icon::Copy)
                        .min_width(width)
                        .enabled(availability.is_available())
                        .show(ui);
                    if let CommandAvailability::Disabled(reason) = availability {
                        response.on_disabled_hover_text(reason);
                    } else if response.clicked() {
                        Command::SheetFormatManager.execute(app);
                    }
                },
            );
        });
    }
}

fn drawing_sheet_title_block_substitution(ui: &mut Ui, sheet: &ActiveDrawingSheet) {
    let t = Tokens::get(ui.ctx());
    let requested = drawing_sheet_title_template_name(sheet.format.title_block.template);
    let effective =
        drawing_sheet_title_template_name(sheet.geometry.physical.effective_title_block_template);
    let shown = egui::Frame::new()
        .fill(Color32::from_rgba_unmultiplied(
            t.color.warn.r(),
            t.color.warn.g(),
            t.color.warn.b(),
            18,
        ))
        .stroke(egui::Stroke::new(
            1.0,
            Color32::from_rgba_unmultiplied(
                t.color.warn.r(),
                t.color.warn.g(),
                t.color.warn.b(),
                102,
            ),
        ))
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::symmetric(9, 8))
        .show(ui, |ui| {
            ui.label(
                RichText::new("Title block substituted")
                    .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                    .color(t.color.warn),
            );
            ui.add(
                egui::Label::new(
                    RichText::new(format!(
                        "{requested} does not fit the current drawing area, so {effective} is rendered. The requested template remains saved; enlarge the sheet or drawing area in Page Setup to restore it."
                    ))
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
    ui.painter().vline(
        shown.response.rect.left() + 1.0,
        shown.response.rect.y_range(),
        egui::Stroke::new(2.0, t.color.warn),
    );
}

const fn drawing_sheet_title_template_name(
    template: DrawingSheetTitleBlockTemplate,
) -> &'static str {
    match template {
        DrawingSheetTitleBlockTemplate::Compact => "RSpice compact",
        DrawingSheetTitleBlockTemplate::Standard => "RSpice standard",
        DrawingSheetTitleBlockTemplate::Wide => "RSpice wide",
        DrawingSheetTitleBlockTemplate::OrganizationManaged => "Organization block",
        DrawingSheetTitleBlockTemplate::None => "No title block",
    }
}

const fn drawing_sheet_source_label(inheritance: DrawingSheetInheritance) -> &'static str {
    match inheritance {
        DrawingSheetInheritance::Explicit => "sheet override",
        DrawingSheetInheritance::ProjectDefault => "inherited · project default",
        DrawingSheetInheritance::UserDefault => "inherited · personal",
    }
}

fn drawing_sheet_margins_label(format: &crate::state::SchematicSheetFormat) -> String {
    let margins = format.margins;
    let unit = format.display_unit;
    if margins.top_um == margins.right_um
        && margins.top_um == margins.bottom_um
        && margins.top_um == margins.left_um
    {
        format!(
            "{} {} all edges",
            unit.format_um(margins.top_um),
            unit.suffix()
        )
    } else {
        format!(
            "{} · {} · {} · {} {}",
            unit.format_um(margins.top_um),
            unit.format_um(margins.right_um),
            unit.format_um(margins.bottom_um),
            unit.format_um(margins.left_um),
            unit.suffix()
        )
    }
}

fn drawing_sheet_border_label(sheet: &ActiveDrawingSheet) -> String {
    let template = match sheet.format.border {
        DrawingSheetBorderTemplate::Standard => "standard border with zones",
        DrawingSheetBorderTemplate::Plain => "plain border",
        DrawingSheetBorderTemplate::None => "no border",
        DrawingSheetBorderTemplate::OrganizationManaged => "organization border",
    };
    sheet.geometry.physical.zones.map_or_else(
        || template.to_owned(),
        |zones| format!("{template} · {} × {} zones", zones.columns, zones.rows),
    )
}

fn drawing_sheet_title_block_label(sheet: &ActiveDrawingSheet) -> String {
    let template = match sheet.format.title_block.template {
        DrawingSheetTitleBlockTemplate::Compact => "RSpice compact",
        DrawingSheetTitleBlockTemplate::Standard => "RSpice standard",
        DrawingSheetTitleBlockTemplate::Wide => "RSpice wide",
        DrawingSheetTitleBlockTemplate::OrganizationManaged => "Organization block",
        DrawingSheetTitleBlockTemplate::None => {
            return "none · identity printed in the page header".to_owned();
        }
    };
    let anchor = match sheet.format.title_block.anchor {
        DrawingSheetTitleBlockAnchor::BottomRight => "bottom right",
        DrawingSheetTitleBlockAnchor::BottomLeft => "bottom left",
        DrawingSheetTitleBlockAnchor::BottomStrip => "bottom strip",
        DrawingSheetTitleBlockAnchor::TopRight => "top right",
    };
    format!("{template} · {anchor}")
}

fn drawing_sheet_overflow_advisory(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    summary: &DrawingSheetOverflowSummary,
) {
    let t = Tokens::get(ui.ctx());
    let count = summary.finding_count();
    let mut details = Vec::with_capacity(2);
    if summary.outside_border > 0 {
        let mut outside = format!("{} outside the border", summary.outside_border);
        if summary.off_paper > 0 {
            outside.push_str(&format!(" · {} beyond the paper edge", summary.off_paper));
        }
        details.push(outside);
    }
    if summary.title_block_collisions > 0 {
        details.push(format!(
            "{} overlapping the title block",
            summary.title_block_collisions
        ));
    }
    let fill =
        Color32::from_rgba_unmultiplied(t.color.warn.r(), t.color.warn.g(), t.color.warn.b(), 18);
    let border =
        Color32::from_rgba_unmultiplied(t.color.warn.r(), t.color.warn.g(), t.color.warn.b(), 102);
    ui.add_space(2.0);
    let shown = egui::Frame::new()
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, border))
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::symmetric(9, 8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width().max(1.0));
            ui.spacing_mut().item_spacing.y = 7.0;
            ui.label(
                egui::RichText::new(format!(
                    "△ {count} object{} outside the drawing area",
                    if count == 1 { "" } else { "s" }
                ))
                .font(theme::sans(tokens::FS_1, FontWeight::Medium))
                .color(t.color.warn),
            );
            ui.add(
                egui::Label::new(
                    egui::RichText::new(format!(
                        "{}. This is advisory: the sheet saves, netlists, checks and simulates unchanged. Print and export ask what to do with it.",
                        details.join(" · ")
                    ))
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                )
                .wrap(),
            );
            ui.horizontal(|ui| {
                if Button::new("Show first")
                    .icon(Icon::ZoomFit)
                    .show(ui)
                    .clicked()
                {
                    crate::schematic::view::drawing_sheet::show_first_drawing_sheet_overflow(
                        &mut app.state,
                    );
                }
                if Button::new("Review all…").show(ui).clicked() {
                    crate::workbench::app::open_drawing_sheet_overflow_review(&mut app.state);
                }
            });
        });
    ui.painter().vline(
        shown.response.rect.left() + 1.0,
        shown.response.rect.y_range(),
        egui::Stroke::new(2.0, t.color.warn),
    );
}

fn drawing_sheet_lock_note(ui: &mut Ui, message: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(6.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new(message)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_faint),
        )
        .wrap(),
    );
}

fn schematic_grid_label(pitch: crate::state::SchematicGridPitch) -> &'static str {
    match pitch {
        crate::state::SchematicGridPitch::Mil50 => "50 mil",
        crate::state::SchematicGridPitch::Mil25 => "25 mil",
        crate::state::SchematicGridPitch::Metric => "metric",
    }
}

fn current_violation_count(
    state: &AppState,
    violation_type: crate::services::drc::DrcViolationType,
) -> usize {
    if !checks_current(state) {
        return 0;
    }
    state.dialogs.drc_results.as_ref().map_or(0, |result| {
        result
            .violations()
            .iter()
            .filter(|violation| violation.violation_type == violation_type)
            .count()
    })
}

// =============================================================================
// Multi-selection inspector
// =============================================================================

fn multi_panel(ui: &mut Ui, app: &mut RSpiceApp) {
    let selected: Vec<(u64, String, String)> = app
        .state
        .schematic
        .components
        .iter()
        .filter(|component| app.state.schematic.selection.has_component(component.id))
        .map(|component| {
            (
                component.id,
                component.name.clone(),
                component.value.clone(),
            )
        })
        .collect();
    let total = app.state.schematic.selection.count();
    let others = total.saturating_sub(selected.len());

    hero(
        ui,
        app,
        Hero {
            preview: HeroPreview::Icon(WorkbenchIcon::Select),
            eyebrow: format!(
                "MULTI-SELECTION · SHEET {}",
                app.state.workspace.active_view.cell.to_ascii_uppercase()
            ),
            title: format!("{total} objects"),
            subtitle: match (selected.len(), others) {
                (0, other) => format!("{other} conductors and annotations"),
                (instances, 0) => format!("{instances} instances"),
                (instances, other) => format!("{instances} instances · {other} other objects"),
            },
            statuses: vec![(
                "group transforms".to_owned(),
                Tokens::get(ui.ctx()).color.accent,
            )],
            open_properties: None,
        },
    );

    section_header(ui, "Selected objects", Some(&selected.len().to_string()));
    if selected.is_empty() {
        muted_inspector_copy(
            ui,
            "The selection holds conductors and annotations only. Add an instance to edit properties.",
        );
    } else {
        let mut reduce: Option<u64> = None;
        for (id, name, value) in &selected {
            let row = TreeRow::new(name)
                .mono()
                .indent(1)
                .meta(value)
                .selected(true)
                .show(ui);
            if row.response.clicked() {
                reduce = Some(*id);
            }
            row.response
                .on_hover_text(format!("Reduce the selection to {name}"));
        }
        if let Some(id) = reduce {
            select_component(app, id);
        }
        muted_inspector_copy(
            ui,
            "Selecting a row reduces the selection to that object for property editing.",
        );
    }

    section_header(ui, "Selection actions", None);
    action_stack(ui, |ui| {
        command_action(
            ui,
            app,
            Command::RotateSelection,
            Icon::Rotate,
            "Rotate 90° clockwise",
            false,
        );
        command_action(
            ui,
            app,
            Command::MirrorSelectionHorizontal,
            Icon::Mirror,
            "Mirror about vertical axis",
            false,
        );
        command_action(
            ui,
            app,
            Command::Duplicate,
            Icon::Copy,
            "Duplicate and place…",
            false,
        );
        command_action(
            ui,
            app,
            Command::Delete,
            Icon::Trash,
            "Delete selection",
            true,
        );
    });
}

// =============================================================================
// Documentation objects
// =============================================================================

fn note_panel(ui: &mut Ui, app: &mut RSpiceApp, id: u64) {
    let Some(note) = app
        .state
        .schematic
        .design_notes
        .iter()
        .find(|note| note.id == id)
        .cloned()
    else {
        return;
    };
    hero(
        ui,
        app,
        Hero {
            preview: HeroPreview::Icon(WorkbenchIcon::Label),
            eyebrow: format!("NOTE-{id}"),
            title: note.kind.label().to_owned(),
            subtitle: note.text.clone(),
            statuses: vec![(
                note.layer.label().to_owned(),
                Tokens::get(ui.ctx()).color.text_dim,
            )],
            open_properties: None,
        },
    );
    section_header(ui, "Documentation object", Some("editable"));
    property_row(ui, "Stable ID", &format!("NOTE-{id}"));
    property_row(ui, "Type", note.kind.label());
    property_row(ui, "Text", &note.text);
    property_row(ui, "Layer", note.layer.label());
    property_row(ui, "Anchor", &format!("{}, {}", note.pos.x, note.pos.y));
    if let Some(review) = note.review.as_ref() {
        property_row(ui, "Review record", &review.record_id);
        property_row(ui, "Review state", review.state.keyword());
    }
}

fn shape_panel(ui: &mut Ui, app: &mut RSpiceApp, id: u64) {
    let Some(shape) = app
        .state
        .schematic
        .documentation_shapes
        .iter()
        .find(|shape| shape.id == id)
        .cloned()
    else {
        return;
    };
    let (min, max) = shape.bounds();
    hero(
        ui,
        app,
        Hero {
            preview: HeroPreview::Icon(WorkbenchIcon::Select),
            eyebrow: format!("SHAPE-{id}"),
            title: shape.kind().label().to_owned(),
            subtitle: format!("{} control points", shape.geometry.points().len()),
            statuses: vec![(
                shape.layer.label().to_owned(),
                Tokens::get(ui.ctx()).color.text_dim,
            )],
            open_properties: None,
        },
    );
    section_header(ui, "Documentation shape", Some("editable"));
    property_row(ui, "Stable ID", &format!("SHAPE-{id}"));
    property_row(ui, "Type", shape.kind().label());
    property_row(ui, "Layer", shape.layer.label());
    property_row(ui, "Electrical connectivity", "none");
    property_row(
        ui,
        "Bounds",
        &format!("{}, {} to {}, {}", min.x, min.y, max.x, max.y),
    );
    property_row(
        ui,
        "Control points",
        &shape.geometry.points().len().to_string(),
    );
}

fn probe_panel(ui: &mut Ui, app: &mut RSpiceApp, id: u64) {
    let Some(probe) = app
        .state
        .schematic
        .probes
        .iter()
        .find(|probe| probe.id == id)
        .cloned()
    else {
        return;
    };
    let bound = probe.source_expression.as_deref();
    hero(
        ui,
        app,
        Hero {
            preview: HeroPreview::Icon(WorkbenchIcon::Label),
            eyebrow: format!("PROBE-{id}"),
            title: probe.reference.clone(),
            subtitle: bound.map_or_else(
                || "unbound saved-output marker".to_owned(),
                |expression| format!("bound to {expression}"),
            ),
            statuses: vec![
                (
                    if bound.is_some() { "bound" } else { "unbound" }.to_owned(),
                    if bound.is_some() {
                        Tokens::get(ui.ctx()).color.ok
                    } else {
                        Tokens::get(ui.ctx()).color.warn
                    },
                ),
                (
                    if probe.enabled { "enabled" } else { "disabled" }.to_owned(),
                    if probe.enabled {
                        Tokens::get(ui.ctx()).color.ok
                    } else {
                        Tokens::get(ui.ctx()).color.text_dim
                    },
                ),
            ],
            open_properties: None,
        },
    );
    section_header(ui, "Probe marker", Some("authored output intent"));
    property_row(ui, "Stable ID", &format!("PROBE-{id}"));
    property_row(ui, "Reference", &probe.reference);
    property_row(ui, "Source", bound.unwrap_or("not bound"));
    property_row(
        ui,
        "Anchor",
        &format!("{}, {}", probe.position.x, probe.position.y),
    );
    property_row(ui, "Electrical connectivity", "none");

    let writable = !app.state.schematic_edit_read_only();
    let mut enabled = probe.enabled;
    let enabled_response = ui.add_enabled(
        writable,
        egui::Checkbox::new(&mut enabled, "Include this probe in future runs"),
    );
    if !writable {
        enabled_response.on_disabled_hover_text("The active schematic is read-only.");
    }
    let mut plot_on_materialization = probe.plot_on_materialization;
    let plot_response = ui.add_enabled(
        writable && enabled,
        egui::Checkbox::new(
            &mut plot_on_materialization,
            "Show automatically when the next run materializes it",
        ),
    );
    if !writable {
        plot_response.on_disabled_hover_text("The active schematic is read-only.");
    } else if !enabled {
        plot_response
            .on_disabled_hover_text("Enable the probe before changing its display intent.");
    }
    if enabled != probe.enabled || plot_on_materialization != probe.plot_on_materialization {
        let changed = app
            .state
            .schematic
            .with_undo("edit schematic probe", |schematic| {
                if let Some(live) = schematic.probes.iter_mut().find(|probe| probe.id == id) {
                    live.enabled = enabled;
                    live.plot_on_materialization = plot_on_materialization;
                    schematic.is_dirty = true;
                }
            });
        if changed {
            app.state.sync_active_schematic_to_workspace();
            app.invalidate_simulation_preflight();
        }
    }

    action_stack(ui, |ui| {
        if let Some(expression) = bound
            && Button::new("Show in results")
                .icon(Icon::Results)
                .show(ui)
                .clicked()
        {
            if writable {
                let changed = crate::schematic::view::ensure_probe_visible_with_feedback(
                    ui,
                    &mut app.state,
                    expression,
                    expression,
                );
                if changed {
                    app.invalidate_simulation_preflight();
                }
            } else {
                crate::schematic::view::ensure_retained_probe_visible_with_feedback(
                    ui,
                    &mut app.state,
                    expression,
                    expression,
                );
            }
        }
        if Button::new("Center marker").ghost().show(ui).clicked() {
            app.state.schematic.center_request = Some(probe.position);
        }
        command_action(ui, app, Command::Delete, Icon::Trash, "Delete probe", true);
    });
}

#[cfg(test)]
mod tests;
