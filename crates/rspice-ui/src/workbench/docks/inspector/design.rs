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
use crate::simulation::netlist_gen::{
    DesignNet, HierarchySource, NetClass, design_nets_with_hierarchy,
};
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

/// Live connectivity for the open sheet: its nets, and every instance's
/// declared terminals paired with the net each one binds.
struct SheetConnectivity {
    nets: Vec<DesignNet>,
    /// Instance ID → declared terminals in symbol order, each with the net
    /// it binds or `None` when the pin is open.
    terminals: HashMap<u64, Vec<(String, Option<String>)>>,
}

/// Resolve the open sheet's connectivity.
///
/// Recomputed per frame rather than memoized: net identity depends on
/// instance values and port parameters, which do not advance the schematic
/// topology version, so a version-keyed cache would hand back stale net
/// names after a rename.
fn sheet_connectivity(state: &AppState) -> SheetConnectivity {
    let hierarchy = HierarchySource::from_workspace_with_connectivity(
        &state.library_manager,
        &state.workspace.schematic_buffers,
        &state.workspace.connectivity,
    );
    let nets = design_nets_with_hierarchy(&state.schematic, &hierarchy);

    let mut bound: HashMap<(u64, &str), &str> = HashMap::new();
    for net in &nets {
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

    SheetConnectivity { nets, terminals }
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
// Inline instance editing
// =============================================================================

/// The field's value as the design currently holds it.
fn field_value(component: &Component, field: &InlineEditField) -> String {
    match field {
        InlineEditField::Instance => component.name.clone(),
        InlineEditField::Value => component.value.clone(),
        InlineEditField::Parameters => component.params.clone(),
        InlineEditField::Parameter(key) => crate::state::parse_params_string(&component.params)
            .get(key)
            .cloned()
            .unwrap_or_default(),
    }
}

/// Why `candidate` cannot be applied to `field`, if it cannot.
///
/// Reference designators obey SPICE identity rules. Declared instance
/// parameters reuse their property-sheet type, quantity, enum, and range
/// contract; unknown extension parameters remain losslessly editable.
fn field_rejection(
    state: &AppState,
    component: &Component,
    field: &InlineEditField,
    candidate: &str,
) -> Option<String> {
    if let InlineEditField::Parameter(key) = field {
        match crate::workbench::app::authoritative_component_property_sheet(state, component) {
            Ok(Some(sheet)) => {
                if let Some(definition) = sheet.get(key) {
                    return parameter_source_rejection(state, definition, candidate);
                }
            }
            Ok(None) => {}
            Err(error) => return Some(error),
        }
    }
    let InlineEditField::Instance = field else {
        return None;
    };
    let candidate = candidate.trim();
    if candidate.is_empty() {
        return Some("Enter a non-empty instance name.".to_owned());
    }
    if candidate.eq_ignore_ascii_case(&component.name) {
        return None;
    }
    if let Err(error) = component.validate_reference_designator(candidate) {
        return Some(error);
    }
    state
        .schematic
        .components
        .iter()
        .any(|other| other.id != component.id && other.name.eq_ignore_ascii_case(candidate))
        .then(|| {
            format!(
                "A component named `{candidate}` already exists; SPICE designators are case-insensitively unique."
            )
        })
}

fn parameter_source_rejection(
    state: &AppState,
    definition: &PropertyDefinition,
    candidate: &str,
) -> Option<String> {
    let candidate = candidate.trim();
    // An empty secondary parameter means "inherit the model/property-sheet
    // default"; it is not the same transaction as authoring an empty value.
    if candidate.is_empty() {
        return definition
            .required
            .then(|| format!("{} is required.", definition.display_name));
    }
    let value = match definition.prop_type {
        PropertyType::Number | PropertyType::Expression => {
            return crate::properties::tabbed_dialog::parse_expression_source(
                definition,
                candidate,
                state.ui.preferences.quantity_presentation_policy(),
                state.ui.number_locale,
            )
            .err();
        }
        PropertyType::String => PropertyValue::String(candidate.to_owned()),
        PropertyType::Boolean => match candidate.to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" | "on" => PropertyValue::Boolean(true),
            "false" | "0" | "no" | "off" => PropertyValue::Boolean(false),
            _ => {
                return Some(format!(
                    "{} must be yes/no, true/false, on/off, or 1/0",
                    definition.display_name
                ));
            }
        },
        PropertyType::Enum => {
            let PropertyValue::Enum { options, .. } = &definition.default_value else {
                return Some(format!(
                    "{} has an invalid enumerated property contract",
                    definition.display_name
                ));
            };
            let Some(selected) = options
                .iter()
                .find(|option| option.eq_ignore_ascii_case(candidate))
            else {
                return Some(format!(
                    "{} must be one of: {}",
                    definition.display_name,
                    options.join(", ")
                ));
            };
            PropertyValue::Enum {
                selected: selected.clone(),
                options: options.clone(),
            }
        }
    };
    definition.validate(&value).err()
}

/// Write `candidate` into `field` on the live design.
///
/// Returns `true` when the design actually changed, so a session that only
/// regained and lost focus never manufactures an undo entry.
fn apply_field(state: &mut AppState, id: u64, field: &InlineEditField, candidate: &str) -> bool {
    let Some(component) = state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == id)
    else {
        return false;
    };
    let changed = match field {
        InlineEditField::Instance => {
            let candidate = candidate.trim();
            if component.name == candidate {
                false
            } else {
                component.name = candidate.to_owned();
                true
            }
        }
        InlineEditField::Value => {
            if component.value == candidate {
                false
            } else {
                component.value = candidate.to_owned();
                true
            }
        }
        InlineEditField::Parameters => {
            let updated = candidate.trim().to_owned();
            if component.params == updated {
                false
            } else {
                component.params = updated;
                true
            }
        }
        InlineEditField::Parameter(key) => {
            let updated = write_param(&component.params, key, candidate);
            if component.params == updated {
                false
            } else {
                component.params = updated;
                true
            }
        }
    };
    if changed {
        state.schematic.is_dirty = true;
        state.schematic.bump_topology_version();
    }
    changed
}

/// Set `key` to `value` in a `key=value key=value` parameter string,
/// preserving the order of the other entries. An empty value removes the
/// entry, returning the instance to whatever it inherits.
fn write_param(params: &str, key: &str, value: &str) -> String {
    let value = value.trim();
    let mut parts: Vec<String> = Vec::new();
    let mut replaced = false;
    for entry in params.split_whitespace() {
        let entry_key = entry.split_once('=').map_or(entry, |(name, _)| name);
        if entry_key.eq_ignore_ascii_case(key) {
            replaced = true;
            if !value.is_empty() {
                parts.push(format!("{key}={value}"));
            }
        } else {
            parts.push(entry.to_owned());
        }
    }
    if !replaced && !value.is_empty() {
        parts.push(format!("{key}={value}"));
    }
    parts.join(" ")
}

/// Open an edit session on `field`, seeded with `current`.
fn begin_edit(app: &mut RSpiceApp, component: &Component, field: InlineEditField, current: String) {
    if app.state.schematic_edit_read_only() {
        return;
    }
    let before = crate::state::SchematicSnapshot::capture(&app.state.schematic);
    app.state
        .workbench
        .inline_edit
        .begin(component.id, field, &current, before);
}

/// End the open session, folding everything typed into it into one undo
/// entry described by `description`.
fn commit_edit(app: &mut RSpiceApp, description: &str) {
    if let Some(before) = app.state.workbench.inline_edit.end() {
        app.state.schematic.commit_undo_from(before, description);
    }
}

fn edit_description(field: &InlineEditField) -> String {
    match field {
        InlineEditField::Instance => "rename instance".to_owned(),
        InlineEditField::Value => "edit instance value".to_owned(),
        InlineEditField::Parameters => "edit instance parameters".to_owned(),
        InlineEditField::Parameter(key) => format!("edit {key}"),
    }
}

fn tunable_value_quantity(kind: ComponentType) -> Option<crate::state::DesignVariableQuantity> {
    use crate::state::DesignVariableQuantity as Quantity;

    match kind {
        ComponentType::Resistor | ComponentType::Ccvs => Some(Quantity::Resistance),
        ComponentType::Capacitor => Some(Quantity::Capacitance),
        ComponentType::VoltageSource | ComponentType::VoltageSourceAc => Some(Quantity::Voltage),
        ComponentType::CurrentSource | ComponentType::CurrentSourceAc => Some(Quantity::Current),
        ComponentType::Vcvs | ComponentType::Cccs | ComponentType::OpAmp => {
            Some(Quantity::Dimensionless)
        }
        _ => None,
    }
}

/// Component value fields carry their quantity through the owning device, while
/// design variables are deliberately self-describing. Preserve an explicit
/// unit when one is already present and otherwise add the unit implied by the
/// selected component before constructing the typed variable.
fn typed_tuning_expression(value: &str, quantity: crate::state::DesignVariableQuantity) -> String {
    use crate::state::DesignVariableQuantity as Quantity;

    let value = value.trim();
    let has_ascii_suffix = |suffix: &str| {
        value
            .get(value.len().saturating_sub(suffix.len())..)
            .is_some_and(|candidate| candidate.eq_ignore_ascii_case(suffix))
    };
    match quantity {
        Quantity::Resistance if has_ascii_suffix("ohm") || value.ends_with('Ω') => {
            value.to_owned()
        }
        Quantity::Resistance => format!("{value} ohm"),
        Quantity::Capacitance if value.ends_with('F') => value.to_owned(),
        Quantity::Capacitance => format!("{value}F"),
        Quantity::Voltage if value.ends_with('V') => value.to_owned(),
        Quantity::Voltage => format!("{value}V"),
        Quantity::Current if value.ends_with('A') => value.to_owned(),
        Quantity::Current => format!("{value}A"),
        Quantity::Temperature | Quantity::Dimensionless => value.to_owned(),
    }
}

fn is_parameter_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn simple_parameter_reference(value: &str) -> Option<&str> {
    let value = value.trim();
    let candidate = value
        .strip_prefix('{')
        .and_then(|body| body.strip_suffix('}'))
        .map(str::trim)
        .unwrap_or(value);
    is_parameter_identifier(candidate).then_some(candidate)
}

fn proposed_tuning_variable_name(
    component: &Component,
    variables: &[crate::state::DesignVariable],
) -> String {
    let mut stem = component
        .name
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '_' {
                character.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect::<String>();
    if !stem
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic() || character == '_')
    {
        stem.insert_str(0, "P_");
    }
    stem.truncate(116);
    let base = format!("{stem}_VALUE");
    if !variables
        .iter()
        .any(|variable| variable.name.eq_ignore_ascii_case(&base))
    {
        return base;
    }
    for suffix in 2_u32.. {
        let suffix = format!("_{suffix}");
        let keep = 128_usize.saturating_sub(suffix.len());
        let mut candidate = base.clone();
        candidate.truncate(keep);
        candidate.push_str(&suffix);
        if !variables
            .iter()
            .any(|variable| variable.name.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
    }
    unreachable!("the finite variable set cannot exhaust every numeric suffix")
}

enum ComponentTuningPreparation {
    ExistingBinding {
        variable_id: crate::product::DesignVariableId,
    },
    StageBinding {
        /// Boxed: a staged variable is an order of magnitude larger than the
        /// existing-binding case's identifier.
        variable: Box<crate::state::DesignVariable>,
        creates_variable: bool,
    },
}

struct PreparedComponentTuning {
    plan_id: crate::product::SimulationPlanId,
    plan_revision: crate::product::ObjectRevision,
    variables: Vec<crate::state::DesignVariable>,
    source_view: CellViewRef,
    source_topology_version: u64,
    active_plan_run: Option<crate::product::RunId>,
    preparation: ComponentTuningPreparation,
}

/// Resolve the exact Tune transaction without mutating the sandbox or either
/// authoritative document. The inspector button and the click handler both
/// use this preflight so a control cannot advertise a workflow that staging
/// will later reject.
fn prepare_component_tuning(
    app: &RSpiceApp,
    component: &Component,
) -> Result<PreparedComponentTuning, String> {
    if app.state.schematic_edit_read_only() {
        return Err("the active schematic is read-only".to_owned());
    }
    let quantity = tunable_value_quantity(component.kind).ok_or_else(|| {
        format!(
            "{} values do not have a truthful typed design-variable mapping",
            component.kind.display_name()
        )
    })?;
    let (plan_id, plan_revision) = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| (plan.id(), plan.revision()))
        .map_err(|error| format!("the active simulation plan is unavailable: {error}"))?;
    let variables = app
        .state
        .workspace
        .active_plan_data(plan_id)
        .map(|payload| payload.design_variables.clone())
        .ok_or_else(|| "the active simulation plan has no configuration payload".to_owned())?;
    let source_view = app.state.workspace.active_schematic_reference();
    let source_topology_version = app.state.schematic.topology_version();
    let active_plan_run = app
        .state
        .simulation
        .active_run()
        .filter(|run| {
            run.prepared_receipt()
                .and_then(|receipt| receipt.simulation_plan_id())
                == Some(plan_id)
        })
        .map(|run| run.run_id);

    let session = &app.state.workbench.verification;
    let current_session = session.tuning_plan_id == Some(plan_id)
        && session.tuning_plan_revision == Some(plan_revision);
    if current_session && let Some(pending) = session.tuning_instance_binding.as_ref() {
        if pending.component_id == component.id
            && pending.source_view == source_view
            && pending.source_topology_version == source_topology_version
            && pending.source_value == component.value
        {
            return Ok(PreparedComponentTuning {
                plan_id,
                plan_revision,
                variables,
                source_view,
                source_topology_version,
                active_plan_run,
                preparation: ComponentTuningPreparation::ExistingBinding {
                    variable_id: pending.variable.id,
                },
            });
        }
        if pending.creates_variable || pending.requires_schematic_edit() {
            return Err(format!(
                "{} already has an uncommitted Value binding; commit or revert it before tuning another instance",
                pending.component_name
            ));
        }
    }

    let (variable, creates_variable) = if let Some(reference) =
        simple_parameter_reference(&component.value)
    {
        let variable = variables
            .iter()
            .find(|variable| variable.name.eq_ignore_ascii_case(reference))
            .cloned()
            .ok_or_else(|| {
                format!(
                    "the Value row references parameter '{reference}', but the active plan does not define it"
                )
            })?;
        if variable.quantity != quantity {
            return Err(format!(
                "parameter '{}' is typed as {}, but {} requires {}",
                variable.name,
                variable.quantity.label(),
                component.kind.display_name(),
                quantity.label()
            ));
        }
        (variable, false)
    } else {
        let name = proposed_tuning_variable_name(component, &variables);
        let typed_expression = typed_tuning_expression(&component.value, quantity);
        let variable = crate::state::DesignVariable::new(
            &name,
            &typed_expression,
            quantity,
            // Generated run decks emit project/testbench parameters at
            // the configured root. The explicit instance binding remains
            // the sole consumer, while project scope keeps a child-cell
            // edit executable from its parent simulation root.
            crate::state::DesignVariableScope::Project,
            format!(
                "Value of {} in {}",
                component.name,
                source_view.display_path()
            ),
            None,
            crate::state::DesignVariableSweepEligibility::NestedSweepAndOptimization,
            crate::state::DesignVariableOverridePolicy::ExplicitTestLocalOverride,
        )
        .map_err(|error| {
            format!(
                "the current Value '{}' cannot become a typed {} variable: {error}",
                component.value,
                quantity.label()
            )
        })?;
        (variable, true)
    };

    Ok(PreparedComponentTuning {
        plan_id,
        plan_revision,
        variables,
        source_view,
        source_topology_version,
        active_plan_run,
        preparation: ComponentTuningPreparation::StageBinding {
            variable: Box::new(variable),
            creates_variable,
        },
    })
}

fn component_tuning_action_block_reason(app: &RSpiceApp, component: &Component) -> Option<String> {
    match Command::VerificationPage(VerificationPage::Tuning).availability(app) {
        CommandAvailability::Available => prepare_component_tuning(app, component).err(),
        CommandAvailability::Disabled(reason) => Some(reason.to_owned()),
        CommandAvailability::Hidden => {
            Some("parameter tuning is unavailable in this context".to_owned())
        }
    }
}

/// Bind the selected Value row to the non-destructive parameter sandbox.
///
/// This function writes only runtime proposal state. The schematic and active
/// plan remain byte-for-byte authoritative until the existing review dialog
/// commits the complete transaction.
fn stage_component_tuning(app: &mut RSpiceApp, component_id: u64) -> Result<(), String> {
    let component = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or_else(|| "the selected instance no longer exists".to_owned())?;
    let prepared = prepare_component_tuning(app, &component)?;

    let session = &mut app.state.workbench.verification;
    if session.tuning_plan_id != Some(prepared.plan_id)
        || session.tuning_plan_revision != Some(prepared.plan_revision)
    {
        session.tuning_plan_id = Some(prepared.plan_id);
        session.tuning_plan_revision = Some(prepared.plan_revision);
        session.tuning_variables = prepared
            .variables
            .iter()
            .map(|variable| crate::workbench::state::TuningVariableDraft {
                variable_id: variable.id,
                baseline_expression: variable.expression.clone(),
                candidate_expression: variable.expression.clone(),
                validation_error: None,
                proposed: false,
            })
            .collect();
        session.tuning_instance_binding = None;
        session.tuning_selected_variable = None;
        session.tuning_focus_variable = None;
        session.tuning_baseline_run = prepared.active_plan_run;
        session.tuning_review_open = false;
    }

    let (variable, creates_variable) = match prepared.preparation {
        ComponentTuningPreparation::ExistingBinding { variable_id } => {
            session.tuning_selected_variable = Some(variable_id);
            session.tuning_focus_variable = Some(variable_id);
            return Ok(());
        }
        ComponentTuningPreparation::StageBinding {
            variable,
            creates_variable,
        } => (*variable, creates_variable),
    };
    if creates_variable {
        session.tuning_variables.retain(|draft| !draft.proposed);
        session
            .tuning_variables
            .push(crate::workbench::state::TuningVariableDraft {
                variable_id: variable.id,
                baseline_expression: variable.expression.clone(),
                candidate_expression: variable.expression.clone(),
                validation_error: None,
                proposed: true,
            });
    }

    let binding_expression = format!("{{{}}}", variable.name);
    session.tuning_instance_binding = Some(crate::workbench::state::TuningInstanceBindingDraft {
        component_id,
        component_name: component.name.clone(),
        source_view: prepared.source_view,
        source_topology_version: prepared.source_topology_version,
        source_value: component.value.clone(),
        binding_expression,
        variable: variable.clone(),
        creates_variable,
    });
    session.tuning_selected_variable = Some(variable.id);
    session.tuning_focus_variable = Some(variable.id);
    session.action_receipt = if creates_variable {
        format!(
            "{} is staged as a new typed {} variable for {}; the schematic and plan are unchanged.",
            variable.name,
            variable.quantity.label(),
            component.name
        )
    } else {
        format!(
            "{} is selected for {}; edits remain sandboxed until explicit review and commit.",
            variable.name, component.name
        )
    };
    Ok(())
}

/// One editable instance row.
///
/// The row applies each keystroke to the design so the canvas, connectivity,
/// and netlist track the edit as it is typed, but the undo history records a
/// single entry when the field loses focus. Illegal text is held in the
/// session buffer, outlined and explained, and never written to the design.
fn edit_row(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    component: &Component,
    field: InlineEditField,
    label: &str,
) -> Option<String> {
    edit_row_with_hint(ui, app, component, field, label, "")
}

/// An editable row whose empty authoritative value presents inherited/default
/// copy. The hint is never written into the design.
fn edit_row_with_hint(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    component: &Component,
    field: InlineEditField,
    label: &str,
    hint: &str,
) -> Option<String> {
    let editable = !app.state.schematic_edit_read_only();
    if !editable {
        let value = field_value(component, &field);
        property_row(ui, label, if value.is_empty() { hint } else { &value });
        return None;
    }

    let mut buffer = app
        .state
        .workbench
        .inline_edit
        .buffer_for(component.id, &field)
        .map_or_else(|| field_value(component, &field), str::to_owned);
    let rejection = app
        .state
        .workbench
        .inline_edit
        .error_for(component.id, &field)
        .map(str::to_owned);

    let tuning = Command::VerificationPage(VerificationPage::Tuning);
    let (response, tuning_response) = if matches!(field, InlineEditField::Value) {
        let tuning_block_reason = rejection.as_ref().map_or_else(
            || component_tuning_action_block_reason(app, component),
            |reason| {
                Some(format!(
                    "resolve the Value validation error before tuning: {reason}"
                ))
            },
        );
        let (edit, action) = property_row_input_action(
            ui,
            label,
            &mut buffer,
            rejection.is_some(),
            WorkbenchIcon::Sliders,
            &format!("Scrub-tune {} in the parameter sandbox", component.name),
            tuning_block_reason.is_none(),
            tuning_block_reason.as_deref(),
        );
        (edit, Some(action))
    } else {
        (
            property_row_input_with_hint(ui, label, &mut buffer, hint, rejection.is_some()),
            None,
        )
    };
    if response.gained_focus() {
        begin_edit(app, component, field.clone(), buffer.clone());
    }
    if response.changed() {
        begin_edit(app, component, field.clone(), buffer.clone());
        app.state.workbench.inline_edit.set_buffer(buffer.clone());
        match field_rejection(&app.state, component, &field, &buffer) {
            Some(reason) => app.state.workbench.inline_edit.set_error(Some(reason)),
            None => {
                app.state.workbench.inline_edit.set_error(None);
                apply_field(&mut app.state, component.id, &field, &buffer);
            }
        }
    }
    if response.lost_focus() {
        commit_edit(app, &edit_description(&field));
    }
    if tuning_response.is_some_and(|response| response.clicked()) {
        match stage_component_tuning(app, component.id) {
            Ok(()) => tuning.execute(app),
            Err(error) => {
                app.state.workbench.verification.action_receipt = format!("Tune blocked: {error}");
                app.state
                    .push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                        "Could not tune {}: {error}",
                        component.name
                    )));
            }
        }
    }

    let rejection = app
        .state
        .workbench
        .inline_edit
        .error_for(component.id, &field)
        .map(str::to_owned);
    if let Some(reason) = rejection.as_deref() {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_description(reason);
            node.set_invalid(egui::accesskit::Invalid::True);
        });
    }
    rejection
}

const INLINE_VALIDATION_SLOT_H: f32 = 18.0;

/// Stable validation slot for an editable property group.
///
/// The slot belongs to the open edit session: while one of the group's fields
/// holds focus it owns the same height whether or not the typed text was
/// rejected, so intermediate invalid input cannot push the terminal,
/// parameter, or operating-point sections up and down as it is typed. With no
/// session open there is nothing to report, and reserving the strip would only
/// leave every editable section framed by more space below its last row than
/// above its first.
fn rejection_slot(ui: &mut Ui, editing: bool, reason: Option<&str>) -> egui::Rect {
    let t = Tokens::get(ui.ctx());
    if !editing && reason.is_none() {
        return egui::Rect::from_min_size(ui.cursor().min, egui::Vec2::ZERO);
    }
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), INLINE_VALIDATION_SLOT_H),
        egui::Sense::hover(),
    );
    if let Some(reason) = reason {
        let label_rect = rect.shrink2(egui::vec2(10.0, 0.0));
        let label = ui.put(
            label_rect,
            egui::Label::new(
                egui::RichText::new(reason)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.err),
            )
            .truncate(),
        );
        let _ = label.on_hover_text(reason);
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), reason)
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Status);
            node.set_label(reason);
        });
    }
    rect
}
fn net_class_tone(ui: &Ui, class: NetClass) -> Color32 {
    let t = Tokens::get(ui.ctx());
    match class {
        NetClass::Ground => t.color.text_dim,
        NetClass::Supply => t.color.accent,
        NetClass::Signal => t.color.ok,
    }
}

fn net_panel(ui: &mut Ui, app: &mut RSpiceApp, name: &str, nets: &[DesignNet]) {
    let Some(net) = nets.iter().find(|net| net.name.eq_ignore_ascii_case(name)) else {
        // The conductor resolved to a net connectivity no longer reports.
        // Fall back to the sheet rather than narrate a phantom object.
        sheet_panel(ui, app, nets);
        return;
    };
    let class = net.class;
    let port = net.port;
    let net_name = net.name.clone();
    let segment_count = net.wire_ids.len();
    let terminals = net.terminals.clone();
    let scope = match port {
        Some(direction) => format!("interface port · {}", direction.keyword()),
        None if class == NetClass::Ground => "global reference".to_owned(),
        None => "sheet-local".to_owned(),
    };

    hero(
        ui,
        app,
        Hero {
            preview: HeroPreview::Icon(net_icon(class)),
            eyebrow: format!(
                "NET · SHEET {}",
                app.state.workspace.active_view.cell.to_ascii_uppercase()
            ),
            title: net_name.clone(),
            subtitle: scope.clone(),
            statuses: vec![(class.keyword().to_owned(), net_class_tone(ui, class))],
            open_properties: None,
        },
    );

    section_header(ui, "Net identity", None);
    property_row(ui, "Class", class.keyword());
    property_row(ui, "Scope", &scope);
    property_row(
        ui,
        "Conductors",
        &match segment_count {
            0 => "no drawn segments".to_owned(),
            1 => "1 wire".to_owned(),
            count => format!("{count} wires"),
        },
    );
    property_row(
        ui,
        "Terminals",
        &if terminals.is_empty() {
            "unwired".to_owned()
        } else {
            terminals.len().to_string()
        },
    );

    section_header(
        ui,
        "Connected terminals",
        Some(&terminals.len().to_string()),
    );
    if terminals.is_empty() {
        muted_inspector_copy(
            ui,
            "No bound terminals. A wire binds when it ends on an instance pin.",
        );
    } else {
        let mut select: Option<u64> = None;
        for terminal in &terminals {
            let value = app
                .state
                .schematic
                .components
                .iter()
                .find(|component| component.id == terminal.component_id)
                .map_or_else(String::new, |component| component.value.clone());
            let label = format!("{}.{}", terminal.reference, terminal.pin);
            let row = TreeRow::new(&label).mono().indent(1).meta(&value).show(ui);
            if row.response.clicked() {
                select = Some(terminal.component_id);
            }
            row.response
                .on_hover_text(format!("Select {} on the sheet", terminal.reference));
        }
        if let Some(id) = select {
            select_component(app, id);
        }
    }

    match net_operating_point(&app.state, &net_name) {
        Some(annotation) => {
            section_header(
                ui,
                &format!("Operating point · Run {}", annotation.run_id),
                Some(if annotation.current {
                    "current"
                } else {
                    "stale"
                }),
            );
            if !annotation.current {
                property_row(
                    ui,
                    "Provenance",
                    "Historical evidence · rerun for current schematic",
                );
            }
            property_row(
                ui,
                "Node voltage",
                &format!(
                    "{} {}",
                    crate::quantity::format_engineering_value(annotation.voltage),
                    annotation.unit
                ),
            );
            property_row(
                ui,
                "Detail",
                &format!(
                    "V({net_name}) · {} node solution",
                    if annotation.current {
                        "current"
                    } else {
                        "historical"
                    }
                ),
            );
            property_row(ui, "Analysis", &annotation.analysis);
            property_row(
                ui,
                "Temperature",
                &format!(
                    "{:.1} °C",
                    app.state.sim_setup.reference_pvt.temperature_celsius
                ),
            );
        }
        None => {
            section_header(ui, "Operating point", Some("no evidence"));
            muted_inspector_copy(
                ui,
                "No retained operating point names this conductor. Run a DC operating point to annotate it.",
            );
        }
    }

    let current = checks_current(&app.state);
    let findings = current.then(|| {
        app.state
            .dialogs
            .drc_results
            .as_ref()
            .map(|result| {
                result
                    .violations()
                    .iter()
                    .filter(|violation| violation_targets_net(violation, net))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    });
    let collision_count = current.then(|| net_name_collision_count(&app.state, net));
    section_header(ui, "Checks", Some(&checks_status(&app.state)));
    let t = Tokens::get(ui.ctx());
    let (connectivity, connectivity_tone, connectivity_mark) = match findings.as_ref() {
        None => (
            "pending recheck".to_owned(),
            t.color.warn,
            StatusMark::Warning,
        ),
        Some(findings) if findings.is_empty() => (
            if terminals.len() >= 2 {
                "conductor closed".to_owned()
            } else {
                "declared".to_owned()
            },
            t.color.ok,
            StatusMark::Success,
        ),
        Some(findings) => {
            let severity = findings
                .iter()
                .map(|finding| finding.severity)
                .max()
                .unwrap_or(DrcSeverity::Info);
            (
                format!(
                    "{} finding{}",
                    findings.len(),
                    if findings.len() == 1 { "" } else { "s" }
                ),
                if severity >= DrcSeverity::Error {
                    t.color.err
                } else {
                    t.color.warn
                },
                if severity >= DrcSeverity::Error {
                    StatusMark::Failure
                } else {
                    StatusMark::Warning
                },
            )
        }
    };
    property_row_status(
        ui,
        "Connectivity",
        &connectivity,
        connectivity_tone,
        connectivity_mark,
    );
    let (collision_label, collision_tone, collision_mark) = match collision_count {
        None => (
            "pending recheck".to_owned(),
            t.color.warn,
            StatusMark::Warning,
        ),
        Some(0) => (
            "unique on sheet".to_owned(),
            t.color.ok,
            StatusMark::Success,
        ),
        Some(count) => (
            format!(
                "{count} conflicting name{}",
                if count == 1 { "" } else { "s" }
            ),
            t.color.err,
            StatusMark::Failure,
        ),
    };
    property_row_status(
        ui,
        "Name collisions",
        &collision_label,
        collision_tone,
        collision_mark,
    );

    let plottable = class != NetClass::Ground;
    let connected: Vec<u64> = terminals
        .iter()
        .map(|terminal| terminal.component_id)
        .collect();
    if plottable || !connected.is_empty() {
        action_stack(ui, |ui| {
            if plottable {
                let display = format!("V({net_name})");
                let label = format!("Plot {display}");
                if Button::new(&label).icon(Icon::Results).show(ui).clicked() {
                    let configuration_changed = crate::schematic::view::toggle_probe_with_feedback(
                        ui,
                        &mut app.state,
                        &net_name,
                        &display,
                    );
                    if configuration_changed {
                        app.invalidate_simulation_preflight();
                    }
                }
            }
            if !connected.is_empty()
                && Button::new("Select connected instances")
                    .ghost()
                    .show(ui)
                    .clicked()
            {
                app.state.schematic.selection.clear();
                app.state.schematic.net_highlight.clear();
                for id in &connected {
                    app.state.schematic.selection.select_component(*id);
                }
            }
        });
    }
}

struct NetAnnotation {
    run_id: u64,
    analysis: String,
    voltage: f64,
    unit: String,
    current: bool,
}

/// The selected run's DC node voltage for a net. Historical values remain
/// inspectable but carry an explicit stale provenance label.
fn net_operating_point(state: &AppState, net: &str) -> Option<NetAnnotation> {
    let bare = net.to_ascii_lowercase();
    let wrapped = format!("v({bare})");
    let current = active_run_matches_design(state);
    state.simulation.active_run().and_then(|run| {
        run.analyses.iter().find_map(|analysis| {
            analysis.dc_op.as_ref().and_then(|op| {
                op.node_voltages
                    .iter()
                    .find(|value| {
                        let name = value.name.to_ascii_lowercase();
                        name == wrapped || name == bare
                    })
                    .map(|value| NetAnnotation {
                        run_id: run.id,
                        analysis: analysis.label.clone(),
                        voltage: value.value,
                        unit: value.unit.clone(),
                        current,
                    })
            })
        })
    })
}

fn active_run_matches_design(state: &AppState) -> bool {
    let Some(run) = state.simulation.active_run() else {
        return false;
    };
    run.prepared_receipt()
        .is_some_and(|receipt| receipt.project_revision() == state.workspace.project.revision())
        && state.simulation.cross_probe.is_current_for(
            &state.workspace.active_view,
            state.schematic.topology_version(),
        )
}

fn violation_targets_net(violation: &DrcViolation, net: &DesignNet) -> bool {
    use crate::services::drc::DrcViolationType;

    if !matches!(
        violation.violation_type,
        DrcViolationType::FloatingNode
            | DrcViolationType::UnconnectedPin
            | DrcViolationType::OrphanNetLabel
            | DrcViolationType::DanglingWire
            | DrcViolationType::ShortedOutputs
            | DrcViolationType::ShortCircuit
            | DrcViolationType::SourceToSource
            | DrcViolationType::InvalidName
    ) {
        return false;
    }
    match &violation.location {
        DrcLocation::Node { net_name } | DrcLocation::NetLabel { name: net_name } => {
            net_name.eq_ignore_ascii_case(&net.name)
        }
        DrcLocation::Wire { id } => net.wire_ids.contains(id),
        DrcLocation::Component { id, .. } => {
            violation.violation_type != DrcViolationType::UnconnectedPin
                && net
                    .terminals
                    .iter()
                    .any(|terminal| terminal.component_id == *id)
                && violation.related_items.iter().any(|item| {
                    item.eq_ignore_ascii_case(&net.name)
                        || net
                            .terminals
                            .iter()
                            .any(|terminal| item.eq_ignore_ascii_case(&terminal.reference))
                })
        }
        DrcLocation::Point { .. }
        | DrcLocation::Bus { .. }
        | DrcLocation::BusTap { .. }
        | DrcLocation::Global
        | DrcLocation::SymbolPin { .. } => violation
            .related_items
            .iter()
            .any(|item| item.eq_ignore_ascii_case(&net.name)),
    }
}

fn net_name_collision_count(state: &AppState, net: &DesignNet) -> usize {
    let graph = NetGraph::build(&state.schematic.wires, &state.schematic.junctions);
    let wire_ids = net
        .wire_ids
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>();
    let mut names = state
        .schematic
        .net_labels
        .iter()
        .filter(|label| {
            let connected = graph.find_connected_wires(label.pos);
            (!wire_ids.is_empty() && connected.iter().any(|id| wire_ids.contains(id)))
                || (wire_ids.is_empty() && label.name.eq_ignore_ascii_case(&net.name))
        })
        .map(|label| normalized_net_name(&label.name, state.schematic.document_policy.net_naming))
        .collect::<std::collections::HashSet<_>>();
    for terminal in &net.terminals {
        if let Some(port) = state
            .schematic
            .components
            .iter()
            .find(|component| component.id == terminal.component_id)
            .and_then(Component::port_spec)
        {
            names.insert(normalized_net_name(
                &port.name,
                state.schematic.document_policy.net_naming,
            ));
        }
    }
    names.insert(normalized_net_name(
        &net.name,
        state.schematic.document_policy.net_naming,
    ));
    names.len().saturating_sub(1)
}

fn normalized_net_name(name: &str, policy: NetNamingPolicy) -> String {
    match policy {
        NetNamingPolicy::StrictCaseSensitive => name.to_owned(),
        NetNamingPolicy::SpiceCompatibleRelaxed => name.to_ascii_lowercase(),
    }
}

fn select_component(app: &mut RSpiceApp, id: u64) {
    let position = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .map(|component| component.pos);
    app.state.schematic.selection.select_only_component(id);
    app.state.schematic.net_highlight.clear();
    app.state.schematic.center_request = position;
}

/// Select every conductor of a net and highlight it — the same transaction
/// the navigator's net rows commit.
fn select_net(app: &mut RSpiceApp, net: &DesignNet) {
    app.state.schematic.selection.clear();
    for wire in &net.wire_ids {
        app.state.schematic.selection.select_wire(*wire);
    }
    if net.wire_ids.is_empty() {
        for component_id in net.terminals.iter().map(|terminal| terminal.component_id) {
            app.state.schematic.selection.select_component(component_id);
        }
    }
    app.state
        .schematic
        .net_highlight
        .highlight_named_wires(&net.name, net.wire_ids.iter().copied().collect());
}

// =============================================================================
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
            statuses: vec![(
                if bound.is_some() { "bound" } else { "unbound" }.to_owned(),
                if bound.is_some() {
                    Tokens::get(ui.ctx()).color.ok
                } else {
                    Tokens::get(ui.ctx()).color.warn
                },
            )],
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

    action_stack(ui, |ui| {
        if let Some(expression) = bound {
            let writable = !app.state.schematic_edit_read_only();
            let response = Button::new("Show in results")
                .icon(Icon::Results)
                .enabled(writable)
                .show(ui);
            let clicked = response.clicked();
            if !writable {
                response.on_hover_text(
                    "The active schematic is read-only; a missing saved-output contract cannot be created.",
                );
            }
            if clicked {
                let changed = crate::schematic::view::ensure_probe_visible_with_feedback(
                    ui,
                    &mut app.state,
                    expression,
                    expression,
                );
                if changed {
                    app.invalidate_simulation_preflight();
                }
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
