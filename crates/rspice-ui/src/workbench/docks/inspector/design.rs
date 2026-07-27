//! Selection-dispatched design inspector.
//!
//! The right panel inspects exactly one object class at a time, chosen from
//! the schematic selection: an instance, a conductor, a documentation
//! object, the whole selected set, or — when nothing is selected — the sheet
//! itself. Every row is backed by live document state; no panel narrates a
//! fact the open design cannot supply.

use std::collections::{HashMap, HashSet};

use egui::{Color32, Ui};

use crate::workbench::{AppState, RSpiceApp};
use crate::services::drc::{DrcLocation, DrcSeverity, DrcViolation};
use crate::simulation::netlist_gen::{
    DesignNet, HierarchySource, NetClass, design_nets_with_hierarchy,
};
use crate::state::{
    AnalysisResultPayload, CellViewRef, Component, ComponentType, DisplayMode, NetGraph,
    NetNamingPolicy, PropertyDefinition, PropertyType, PropertyValue, SoaRuleVerdictEvidence,
};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, TreeRow, TreeRowResult};
use crate::workbench::commands::{Command, CommandAvailability};
use crate::workbench::design_system::{
    StatusMark, WorkbenchIcon, property_row_combo, property_row_input_action,
    property_row_input_with_hint, schematic_property_row as property_row,
    schematic_property_row_status as property_row_status,
};
use crate::workbench::state::{InlineEditField, ModelsPage, VerificationPage, Workspace};

use super::{
    ComponentModelEvidence, component_model_evidence, muted_inspector_copy,
    schematic_annotation_section_header, schematic_section_header as section_header,
    schematic_section_header_action, schematic_tree_section_header,
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
        && selection.documentation_shapes.is_empty();
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
        if response.double_clicked() && !app.state.active_view_read_only() {
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
const ACTION_ROW_PAD_X: f32 = 10.0;
const ACTION_ROW_PAD_Y: f32 = 8.0;
const ACTION_ROW_GAP: f32 = 6.0;

fn action_stack(ui: &mut Ui, body: impl FnOnce(&mut Ui)) {
    let width = (ui.available_width() - 2.0 * ACTION_ROW_PAD_X).max(1.0);
    ui.add_space(ACTION_ROW_PAD_Y);
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
    ui.add_space(ACTION_ROW_PAD_Y);
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
        InlineEditField::Parameter(key) => {
            crate::state::parse_params_string(&component.params)
                .get(key)
                .cloned()
                .unwrap_or_default()
        }
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
    if let InlineEditField::Parameter(key) = field
        && let Some(definition) = state
            .property_registry
            .get(component.kind)
            .and_then(|sheet| sheet.get(key))
    {
        return parameter_source_rejection(state, definition, candidate);
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
        return None;
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
    if app.state.active_view_read_only() || app.state.schematic.read_only {
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

/// Bind the selected Value row to the non-destructive parameter sandbox.
///
/// This function writes only runtime proposal state. The schematic and active
/// plan remain byte-for-byte authoritative until the existing review dialog
/// commits the complete transaction.
fn stage_component_tuning(app: &mut RSpiceApp, component_id: u64) -> Result<(), String> {
    if app.state.active_view_read_only() || app.state.schematic.read_only {
        return Err("the active schematic is read-only".to_owned());
    }
    let component = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
        .ok_or_else(|| "the selected instance no longer exists".to_owned())?;
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

    let session = &mut app.state.workbench.verification;
    if session.tuning_plan_id != Some(plan_id)
        || session.tuning_plan_revision != Some(plan_revision)
    {
        session.tuning_plan_id = Some(plan_id);
        session.tuning_plan_revision = Some(plan_revision);
        session.tuning_variables = variables
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
        session.tuning_baseline_run = active_plan_run;
        session.tuning_review_open = false;
    }

    if let Some(pending) = session.tuning_instance_binding.as_ref() {
        if pending.component_id == component_id
            && pending.source_view == source_view
            && pending.source_topology_version == source_topology_version
            && pending.source_value == component.value
        {
            session.tuning_selected_variable = Some(pending.variable.id);
            session.tuning_focus_variable = Some(pending.variable.id);
            return Ok(());
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
        let name = proposed_tuning_variable_name(&component, &variables);
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
        session.tuning_variables.retain(|draft| !draft.proposed);
        session
            .tuning_variables
            .push(crate::workbench::state::TuningVariableDraft {
                variable_id: variable.id,
                baseline_expression: typed_expression.clone(),
                candidate_expression: typed_expression,
                validation_error: None,
                proposed: true,
            });
        (variable, true)
    };

    let binding_expression = format!("{{{}}}", variable.name);
    session.tuning_instance_binding = Some(crate::workbench::state::TuningInstanceBindingDraft {
        component_id,
        component_name: component.name.clone(),
        source_view,
        source_topology_version,
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
    let editable = !app.state.active_view_read_only() && !app.state.schematic.read_only;
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
        let available = tuning.availability(app).is_available();
        let (edit, action) = property_row_input_action(
            ui,
            label,
            &mut buffer,
            rejection.is_some(),
            WorkbenchIcon::Sliders,
            &format!("Scrub-tune {} in the parameter sandbox", component.name),
            available,
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
                    .push_user_message(crate::workbench::ConsoleMessage::warning(format!(
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
/// The slot always owns the same height, so intermediate invalid input cannot
/// push the terminal, parameter, or operating-point sections up and down.
fn rejection_slot(ui: &mut Ui, reason: Option<&str>) -> egui::Rect {
    let t = Tokens::get(ui.ctx());
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

// =============================================================================
// Component inspector
// =============================================================================

fn component_panel(ui: &mut Ui, app: &mut RSpiceApp, id: u64, sheet: &SheetConnectivity) {
    let Some(component) = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == id)
        .cloned()
    else {
        sheet_panel(ui, app, &sheet.nets);
        return;
    };
    let evidence = component_model_evidence(&app.state, &component);
    let editable = !app.state.active_view_read_only() && !app.state.schematic.read_only;

    hero(
        ui,
        app,
        Hero {
            preview: HeroPreview::Symbol(component.kind),
            eyebrow: format!(
                "{} · {}",
                component.name,
                component_occurrence_path(&app.state, &component)
            ),
            title: if component.value.trim().is_empty() {
                component.kind.display_name().to_owned()
            } else {
                component.value.clone()
            },
            subtitle: component.kind.display_name().to_owned(),
            statuses: vec![
                (
                    evidence.status.clone(),
                    evidence.tone.color(&Tokens::get(ui.ctx())),
                ),
                (
                    app.state
                        .sim_setup
                        .reference_pvt
                        .process
                        .short_name()
                        .to_owned(),
                    Tokens::get(ui.ctx()).color.text_dim,
                ),
            ],
            open_properties: editable.then_some(component.id),
        },
    );

    identity_section(ui, app, &component);
    terminals_section(ui, app, &component, sheet);
    parameters_section(ui, app, &component, &evidence);
    operating_point(ui, app, &component);
    component_checks(ui, app, &component);
}

fn component_occurrence_path(state: &AppState, component: &Component) -> String {
    let mut labels = state.workspace.occurrence_labels();
    if labels.is_empty() {
        labels.push(state.workspace.active_view.cell.clone());
    }
    labels.push(component.name.clone());
    format!("/{}", labels.join("/"))
}

fn component_view_contract(component: &Component) -> String {
    component.library_cell.as_ref().map_or_else(
        || "symbol · spice".to_owned(),
        |binding| {
            let view = binding.view.trim();
            if view.is_empty() {
                "symbol · spice".to_owned()
            } else if view.eq_ignore_ascii_case("spice") {
                "spice".to_owned()
            } else {
                format!("{view} · spice")
            }
        },
    )
}

fn identity_section(ui: &mut Ui, app: &mut RSpiceApp, component: &Component) {
    let editable = !app.state.active_view_read_only() && !app.state.schematic.read_only;
    let properties = schematic_section_header_action(ui, "Identity", "Properties…", editable);
    if properties.clicked() {
        crate::workbench::app::open_property_editor(&mut app.state, component.id);
    }
    properties.on_disabled_hover_text("the active document is read-only");
    let instance_rejection = edit_row(ui, app, component, InlineEditField::Instance, "Instance");
    let value_rejection = edit_row(ui, app, component, InlineEditField::Value, "Value");
    let library_cell = component.library_cell.as_ref().map_or_else(
        || format!("primitives/{}", component.kind.display_name()),
        |binding| format!("{}/{}", binding.library, binding.cell),
    );
    property_row(ui, "Library cell", &library_cell);
    property_row(ui, "View", &component_view_contract(component));
    if editable {
        rejection_slot(
            ui,
            instance_rejection.as_deref().or(value_rejection.as_deref()),
        );
    }

    // Identity actions exist only when the design can actually perform
    // them: the editable-properties transaction always, the symbol
    // cellview when the instance is bound to one, and the exact catalog
    // model when its source identity resolves.
    let symbol_view = component
        .library_cell
        .as_ref()
        .map(|binding| {
            CellViewRef::new(
                binding.library.clone(),
                binding.cell.clone(),
                "symbol".to_owned(),
            )
        })
        .filter(|reference| {
            app.state
                .library_manager
                .get_library(&reference.library)
                .and_then(|library| library.get_cell(&reference.cell))
                .and_then(|cell| cell.get_view(&reference.view))
                .is_some()
        });
    let symbol_writable = symbol_view.as_ref().is_some_and(|reference| {
        app.state
            .library_manager
            .get_library(&reference.library)
            .is_some_and(|library| !library.read_only)
    });
    let model_source = component_model_source_target(&app.state, component);
    let hierarchy_master = app
        .state
        .hierarchy_master_for_component(component.id)
        .map(|(_, reference)| reference);
    if symbol_view.is_some() || hierarchy_master.is_some() || model_source.is_some() {
        action_stack(ui, |ui| {
            if let Some(reference) = symbol_view {
                let response = Button::new(if symbol_writable {
                    "Edit symbol…"
                } else {
                    "Open symbol…"
                })
                .ghost()
                .show(ui);
                if response.clicked() {
                    app.state.open_workspace_view(reference);
                }
            }

            if let Some(reference) = hierarchy_master.as_ref() {
                let label = format!("Descend into {}", reference.cell);
                if Button::new(&label).ghost().show(ui).clicked() {
                    app.state.open_selected_instance_master();
                }
            }

            if let Some(target) = model_source.as_ref() {
                let response = Button::new("Open model source…").ghost().show(ui);
                if response.clicked() {
                    match target {
                        ComponentModelSourceTarget::Catalog { library, model } => {
                            app.state
                                .model_library_manager
                                .select_library(library.as_str());
                            app.state.workbench.selected_model = Some(model.clone());
                            app.state.workbench.models_page = ModelsPage::Models;
                            app.state.workbench.activate(Workspace::Models);
                        }
                        ComponentModelSourceTarget::VerilogA(reference) => {
                            app.state.open_workspace_view(reference.clone());
                            app.state.ui.code_workspace.page =
                                crate::workbench::code_workspace::CodeWorkspacePage::VerilogA;
                            app.state.workbench.activate(Workspace::Netlist);
                        }
                    }
                }
            }
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ComponentModelSourceTarget {
    Catalog { library: String, model: String },
    VerilogA(CellViewRef),
}

fn component_model_source_target(
    state: &AppState,
    component: &Component,
) -> Option<ComponentModelSourceTarget> {
    if let Some(binding) = component.library_cell.as_ref() {
        let reference = CellViewRef::new(
            binding.library.clone(),
            binding.cell.clone(),
            binding.view.clone(),
        );
        let is_veriloga = state
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .is_some_and(|view| view.view_type == crate::state::ViewType::VerilogA);
        if is_veriloga {
            return Some(ComponentModelSourceTarget::VerilogA(reference));
        }
    }
    catalog_model_location(state, component)
        .map(|(library, model)| ComponentModelSourceTarget::Catalog { library, model })
}

fn catalog_model_location(state: &AppState, component: &Component) -> Option<(String, String)> {
    if let Some(binding) = component.library_cell.as_ref()
        && let Some(library) = state.model_library_manager.get_library(&binding.library)
    {
        let candidates = [
            binding.module_name.as_deref(),
            Some(binding.cell.as_str()),
            (!component.value.trim().is_empty()).then_some(component.value.trim()),
        ];
        if let Some(model) = candidates.into_iter().flatten().find_map(|candidate| {
            library
                .models
                .values()
                .find(|model| model.name.eq_ignore_ascii_case(candidate))
        }) {
            return Some((library.name.clone(), model.name.clone()));
        }
    }

    let model_name = super::explicit_component_model(component)?;
    let matches = state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .filter_map(|library| {
            library
                .models
                .values()
                .find(|model| model.name.eq_ignore_ascii_case(&model_name))
                .map(|model| (library.name.clone(), model.name.clone()))
        })
        .collect::<Vec<_>>();
    (matches.len() == 1).then(|| matches[0].clone())
}

fn bound_model_choices(
    state: &AppState,
    component: &Component,
    current: &str,
) -> Vec<(String, String)> {
    let Some(binding) = component
        .library_cell
        .as_ref()
        .filter(|binding| binding.netlist_template.is_some())
    else {
        return Vec::new();
    };
    let Some(library) = state.model_library_manager.get_library(&binding.library) else {
        return Vec::new();
    };
    let Some(current_model) = library
        .models
        .values()
        .find(|model| model.name.eq_ignore_ascii_case(current))
    else {
        return Vec::new();
    };
    let mut models = library
        .models
        .values()
        .filter(|model| model.model_type == current_model.model_type)
        .map(|model| model.name.clone())
        .collect::<Vec<_>>();
    models.sort_by_key(|name| name.to_ascii_lowercase());
    models.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    models
        .into_iter()
        .map(|model| (model.clone(), model))
        .collect()
}

fn bound_model_section_choices(
    state: &AppState,
    component: &Component,
    selected_model: &str,
) -> Vec<(String, String)> {
    let Some(binding) = component.library_cell.as_ref() else {
        return Vec::new();
    };
    let Some(library) = state.model_library_manager.get_library(&binding.library) else {
        return Vec::new();
    };
    let mut sections = library
        .model_definition_metadata
        .iter()
        .find(|(model, _)| model.eq_ignore_ascii_case(selected_model))
        .map(|(_, metadata)| {
            metadata
                .sections
                .iter()
                .map(|section| section.name.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if let Some(section) = binding
        .model_section
        .as_deref()
        .filter(|section| !section.trim().is_empty())
        && !sections
            .iter()
            .any(|candidate| candidate.eq_ignore_ascii_case(section))
    {
        sections.push(section.to_owned());
    }
    sections.sort_by_key(|section| section.to_ascii_lowercase());
    sections.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    let mut choices = vec![(String::new(), "default".to_owned())];
    choices.extend(sections.into_iter().map(|section| {
        let display = section.clone();
        (section, display)
    }));
    choices
}

fn apply_bound_model_choice(app: &mut RSpiceApp, component_id: u64, selected_model: &str) {
    let Some(component) = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
    else {
        return;
    };
    let Some(binding) = component.library_cell.as_ref() else {
        return;
    };
    let Some(library) = app
        .state
        .model_library_manager
        .get_library(&binding.library)
    else {
        return;
    };
    let current_name = binding
        .module_name
        .as_deref()
        .unwrap_or(binding.cell.as_str());
    let Some(current_model) = library
        .models
        .values()
        .find(|model| model.name.eq_ignore_ascii_case(current_name))
    else {
        return;
    };
    let Some(candidate) = library
        .models
        .values()
        .find(|model| model.name.eq_ignore_ascii_case(selected_model))
        .filter(|model| model.model_type == current_model.model_type)
    else {
        return;
    };
    let candidate_name = candidate.name.clone();
    let candidate_source = candidate
        .file_path
        .clone()
        .or_else(|| library.root_path.clone());
    let before = crate::state::SchematicSnapshot::capture(&app.state.schematic);
    let Some(component) = app
        .state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == component_id)
    else {
        return;
    };
    let Some(binding) = component.library_cell.as_mut() else {
        return;
    };
    let mut changed = binding.module_name.as_deref() != Some(candidate_name.as_str());
    binding.module_name = Some(candidate_name);
    if candidate_source.is_some() && binding.source_path != candidate_source {
        binding.source_path = candidate_source;
        binding.model_section = None;
        changed = true;
    }
    if changed {
        app.state.schematic.is_dirty = true;
        app.state.schematic.bump_topology_version();
        app.state
            .schematic
            .commit_undo_from(before, "select instance model");
        app.invalidate_simulation_preflight();
    }
}

fn apply_bound_model_section(app: &mut RSpiceApp, component_id: u64, selected_section: &str) {
    let Some(component) = app
        .state
        .schematic
        .components
        .iter()
        .find(|component| component.id == component_id)
        .cloned()
    else {
        return;
    };
    let selected_model = component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.module_name.as_deref())
        .unwrap_or(component.value.as_str());
    let allowed = bound_model_section_choices(&app.state, &component, selected_model);
    if !allowed
        .iter()
        .any(|(value, _)| value.eq_ignore_ascii_case(selected_section))
    {
        return;
    }
    let selected = (!selected_section.trim().is_empty()).then(|| selected_section.to_owned());
    let before = crate::state::SchematicSnapshot::capture(&app.state.schematic);
    let Some(binding) = app
        .state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == component_id)
        .and_then(|component| component.library_cell.as_mut())
    else {
        return;
    };
    if binding.model_section == selected {
        return;
    }
    binding.model_section = selected;
    app.state.schematic.is_dirty = true;
    app.state.schematic.bump_topology_version();
    app.state
        .schematic
        .commit_undo_from(before, "select instance model section");
    app.invalidate_simulation_preflight();
}

fn parameters_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    component: &Component,
    evidence: &ComponentModelEvidence,
) {
    let editable = !app.state.active_view_read_only() && !app.state.schematic.read_only;
    section_header(
        ui,
        "Simulation parameters",
        Some(if editable { "editable" } else { "read-only" }),
    );
    let model_choices = bound_model_choices(&app.state, component, &evidence.model);
    let mut selected_model = component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.module_name.clone())
        .unwrap_or_else(|| evidence.model.clone());
    if model_choices.len() > 1 {
        if property_row_combo(
            ui,
            "Model",
            ("design-inspector-model", component.id),
            &mut selected_model,
            &model_choices,
            editable,
        ) {
            apply_bound_model_choice(app, component.id, &selected_model);
        }
    } else {
        property_row(ui, "Model", &evidence.model);
    }

    let section_choices = bound_model_section_choices(&app.state, component, &selected_model);
    let mut selected_section = component
        .library_cell
        .as_ref()
        .and_then(|binding| binding.model_section.clone())
        .unwrap_or_default();
    if section_choices.len() > 1 {
        if property_row_combo(
            ui,
            "Section",
            ("design-inspector-model-section", component.id),
            &mut selected_section,
            &section_choices,
            editable,
        ) {
            apply_bound_model_section(app, component.id, &selected_section);
        }
    } else {
        property_row(ui, "Section", &evidence.section);
    }

    // Instance temperature is the `temp` parameter. Left empty it inherits
    // the reference PVT point, which the row reports as its placeholder.
    let declared = crate::state::parse_params_string(&component.params);
    let inherited = format!(
        "inherit · {} °C",
        app.state.sim_setup.reference_pvt.temperature_celsius
    );
    let mut rejection = edit_row_with_hint(
        ui,
        app,
        component,
        InlineEditField::Parameter(TEMPERATURE_PARAM.to_owned()),
        "Temperature",
        &inherited,
    );

    // Every parameter declared by the authoritative family sheet is present
    // even on a freshly placed instance whose durable parameter string is
    // still empty. Authored extension fields are appended and never hidden.
    let parameter_contract = inline_parameter_contract(&app.state, component, &declared);
    let raw_field = InlineEditField::Parameters;
    let raw_editor_active = app
        .state
        .workbench
        .inline_edit
        .buffer_for(component.id, &raw_field)
        .is_some();
    if parameter_contract.is_empty() || raw_editor_active {
        let raw_rejection = edit_row_with_hint(
            ui,
            app,
            component,
            raw_field,
            "Parameters",
            "instance value",
        );
        rejection = rejection.or(raw_rejection);
        if editable {
            rejection_slot(ui, rejection.as_deref());
        }
        return;
    }
    for parameter in parameter_contract {
        if parameter.editable {
            let row_rejection = edit_row_with_hint(
                ui,
                app,
                component,
                InlineEditField::Parameter(parameter.key),
                &parameter.label,
                &parameter.hint,
            );
            rejection = rejection.or(row_rejection);
        } else {
            let value = declared
                .get(&parameter.key)
                .map_or(parameter.hint.as_str(), String::as_str);
            property_row(ui, &parameter.label, value);
        }
    }
    if editable {
        rejection_slot(ui, rejection.as_deref());
    }
}

/// SPICE instance-temperature parameter.
const TEMPERATURE_PARAM: &str = "temp";

#[derive(Debug, Clone, PartialEq, Eq)]
struct InlineParameterContract {
    key: String,
    label: String,
    hint: String,
    editable: bool,
}

fn inline_parameter_contract(
    state: &AppState,
    component: &Component,
    declared: &HashMap<String, String>,
) -> Vec<InlineParameterContract> {
    let primary = crate::properties::property_bridge::get_primary_property_name(component.kind);
    let mut rows = Vec::new();
    let mut seen = HashSet::new();
    if let Some(sheet) = state.property_registry.get(component.kind) {
        for definition in sheet.iter().filter(|definition| {
            definition.display_mode != DisplayMode::Hidden
                && definition.name != "name"
                && definition.name != "symbol"
                && !definition.name.eq_ignore_ascii_case(primary)
                && definition.name != TEMPERATURE_PARAM
        }) {
            let hint = match &definition.default_value {
                PropertyValue::Expression(source) => source.clone(),
                value => value.display_string(),
            };
            seen.insert(definition.name.to_ascii_lowercase());
            rows.push(InlineParameterContract {
                key: definition.name.clone(),
                label: definition.display_name.clone(),
                hint,
                editable: !definition.read_only && definition.display_mode != DisplayMode::Readonly,
            });
        }
    }

    if let Some(binding) = component.library_cell.as_ref() {
        for key in &binding.parameter_order {
            let normalized = key.to_ascii_lowercase();
            if normalized == primary || normalized == TEMPERATURE_PARAM || !seen.insert(normalized)
            {
                continue;
            }
            rows.push(InlineParameterContract {
                key: key.clone(),
                label: key.clone(),
                hint: "inherit".to_owned(),
                editable: true,
            });
        }
    }

    let mut extensions = declared
        .keys()
        .filter(|key| {
            key.as_str() != TEMPERATURE_PARAM
                && !key.eq_ignore_ascii_case(primary)
                && !seen.contains(&key.to_ascii_lowercase())
        })
        .cloned()
        .collect::<Vec<_>>();
    extensions.sort_unstable_by_key(|key| key.to_ascii_lowercase());
    rows.extend(extensions.into_iter().map(|key| InlineParameterContract {
        label: key.clone(),
        key,
        hint: "instance value".to_owned(),
        editable: true,
    }));
    rows
}

/// Per-pin terminal table: every declared terminal with the net it binds,
/// clickable to select that conductor. Unbound pins read `open` and are
/// not clickable, because there is no net to select.
fn terminal_row(ui: &mut Ui, pin: &str, net: Option<&str>) -> TreeRowResult {
    ui.add_enabled_ui(net.is_some(), |ui| {
        let t = Tokens::get(ui.ctx());
        let meta = net.unwrap_or("open");
        let accessible_label = format!("{pin}, {meta}");
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 24.0), egui::Sense::click());
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::SelectableLabel,
                ui.is_enabled(),
                &accessible_label,
            )
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::TreeItem);
            node.set_level(2);
        });

        if ui.is_rect_visible(rect) {
            if response.hovered() && ui.is_enabled() {
                ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
            }

            // Mockup `.tree-row.level-1`: 22 px leading inset, a blank
            // 9 px caret column, 6 px gap, then a 15 px terminal-status icon.
            let icon_center = egui::pos2(rect.left() + 44.5, rect.center().y);
            if net.is_some() {
                ui.painter().hline(
                    egui::Rangef::new(icon_center.x - 5.5, icon_center.x + 5.5),
                    icon_center.y,
                    egui::Stroke::new(1.25, t.color.wire),
                );
            } else {
                ui.painter()
                    .circle_stroke(icon_center, 4.0, egui::Stroke::new(1.0, t.color.err));
            }

            let meta_galley = ui.painter().layout_no_wrap(
                meta.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            );
            let meta_left = rect.right() - 8.0 - meta_galley.size().x;
            ui.painter()
                .with_clip_rect(egui::Rect::from_x_y_ranges(
                    (rect.left() + 58.0)..=(meta_left - 8.0).max(rect.left() + 58.0),
                    rect.y_range(),
                ))
                .text(
                    egui::pos2(rect.left() + 58.0, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    pin,
                    theme::mono(tokens::FS_1, FontWeight::Regular),
                    if net.is_some() {
                        t.color.text_dim
                    } else {
                        t.color.text_faint
                    },
                );
            ui.painter().galley(
                egui::pos2(meta_left, rect.center().y - meta_galley.size().y * 0.5),
                meta_galley,
                t.color.text_faint,
            );
            theme::paint_focus_ring(ui, &response, rect);
        }

        TreeRowResult {
            response,
            checkbox_changed: false,
        }
    })
    .inner
}

fn terminals_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    component: &Component,
    sheet: &SheetConnectivity,
) {
    let empty = Vec::new();
    let bindings = sheet.terminals.get(&component.id).unwrap_or(&empty);
    let open_pins = bindings.iter().filter(|(_, net)| net.is_none()).count();

    schematic_tree_section_header(
        ui,
        "Terminals",
        Some(&if open_pins == 0 {
            format!("{} bound", bindings.len())
        } else {
            format!("{open_pins} open")
        }),
    );
    if bindings.is_empty() {
        muted_inspector_copy(ui, "This instance declares no terminals.");
        return;
    }
    let mut select: Option<&str> = None;
    for (pin, net) in bindings {
        let row = terminal_row(ui, pin, net.as_deref());
        match net {
            Some(name) => {
                if row.response.clicked() {
                    select = Some(name.as_str());
                }
                row.response
                    .on_hover_text(format!("Select net {name} on the sheet"));
            }
            None => {
                row.response
                    .on_disabled_hover_text("Unconnected pin · wire it or run connectivity checks");
            }
        }
    }
    if let Some(name) = select
        && let Some(net) = sheet.nets.iter().find(|net| net.name == name)
    {
        select_net(app, net);
    }
}

const OP_SUMMARY_MARGIN_X: f32 = 8.0;
const OP_SUMMARY_PADDING: f32 = 9.0;
const OP_SUMMARY_ROW_H: f32 = 22.0;

fn operating_point_summary(ui: &mut Ui, rows: &[(String, String)]) {
    let t = Tokens::get(ui.ctx());
    let height = OP_SUMMARY_PADDING * 2.0 + OP_SUMMARY_ROW_H * rows.len() as f32;
    let (outer, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), height),
        egui::Sense::hover(),
    );
    let card = egui::Rect::from_min_max(
        egui::pos2(outer.left() + OP_SUMMARY_MARGIN_X, outer.top()),
        egui::pos2(outer.right() - OP_SUMMARY_MARGIN_X, outer.bottom()),
    );
    ui.painter().rect(
        card,
        t.radius,
        t.color.bg_inset,
        egui::Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let painter = ui.painter().with_clip_rect(card);
    for (index, (label, value)) in rows.iter().enumerate() {
        let y = card.top() + OP_SUMMARY_PADDING + OP_SUMMARY_ROW_H * (index as f32 + 0.5);
        painter.text(
            egui::pos2(card.left() + OP_SUMMARY_PADDING, y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::sans(tokens::FS_2, FontWeight::Regular),
            t.color.text_dim,
        );
        painter.text(
            egui::pos2(card.right() - OP_SUMMARY_PADDING, y),
            egui::Align2::RIGHT_CENTER,
            value,
            theme::mono(tokens::FS_2, FontWeight::Medium),
            t.color.text,
        );
    }
}

fn operating_point(ui: &mut Ui, app: &RSpiceApp, component: &Component) {
    let retained = app.state.simulation.active_run().and_then(|run| {
        run.analyses.iter().find_map(|analysis| {
            analysis.device_op.as_ref().and_then(|report| {
                report
                    .entries
                    .iter()
                    .find(|entry| entry.name.eq_ignore_ascii_case(&component.name))
                    .map(|entry| {
                        (
                            run.id,
                            analysis.label.clone(),
                            entry.region,
                            entry.params.clone(),
                            active_run_matches_design(&app.state),
                        )
                    })
            })
        })
    });
    if let Some((run_id, analysis, region, params, current)) = retained {
        schematic_annotation_section_header(
            ui,
            &format!("Operating point · Run {run_id}"),
            Some(if current { "current" } else { "stale" }),
        );
        let mut rows = Vec::new();
        if !current {
            rows.push((
                "Provenance".to_owned(),
                "Historical evidence · rerun for current schematic".to_owned(),
            ));
        }
        if let Some(region) = region {
            rows.push(("Region".to_owned(), region.to_owned()));
        }
        for (name, value) in params.into_iter().take(2_usize.saturating_sub(rows.len())) {
            rows.push((name.to_owned(), format!("{value:.6e}")));
        }
        rows.push((
            "Temperature".to_owned(),
            format!(
                "{:.1} °C",
                app.state.sim_setup.reference_pvt.temperature_celsius
            ),
        ));
        rows.push(("Analysis".to_owned(), analysis));
        operating_point_summary(ui, &rows);
    } else {
        schematic_annotation_section_header(ui, "Operating point", Some("no evidence"));
        operating_point_summary(
            ui,
            &[
                (
                    "Selection".to_owned(),
                    "No retained device operating point".to_owned(),
                ),
                (
                    "Required analysis".to_owned(),
                    "DC operating point".to_owned(),
                ),
            ],
        );
    }
}

fn component_checks(ui: &mut Ui, app: &RSpiceApp, component: &Component) {
    let topology = app.state.schematic.topology_version();
    let current = checks_current(&app.state);
    let findings = current
        .then(|| {
            app.state.dialogs.drc_results.as_ref().map(|result| {
                result
                    .violations()
                    .iter()
                    .filter(|violation| violation_targets_component(violation, component))
                    .collect::<Vec<_>>()
            })
        })
        .flatten();
    section_header(ui, "Checks", Some(&checks_status(&app.state)));
    let t = Tokens::get(ui.ctx());
    let (connectivity, tone, mark) = if let Some(findings) = findings {
        if findings.is_empty() {
            ("checked".to_owned(), t.color.ok, StatusMark::Success)
        } else {
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
    } else {
        (
            "pending recheck".to_owned(),
            t.color.warn,
            StatusMark::Warning,
        )
    };
    property_row_status(ui, "Connectivity", &connectivity, tone, mark);
    let soa = retained_component_soa(&app.state, &component.name);
    property_row_status(
        ui,
        "Safe operating area",
        &soa.label,
        match soa.tone {
            ComponentSoaTone::Pass => t.color.ok,
            ComponentSoaTone::Warning => t.color.warn,
            ComponentSoaTone::Failure => t.color.err,
            ComponentSoaTone::NoEvidence => t.color.text_dim,
        },
        match soa.tone {
            ComponentSoaTone::Pass => StatusMark::Success,
            ComponentSoaTone::Warning => StatusMark::Warning,
            ComponentSoaTone::Failure => StatusMark::Failure,
            ComponentSoaTone::NoEvidence => StatusMark::Neutral,
        },
    );
    property_row(
        ui,
        "Last checked",
        &if current {
            format!("topology revision {topology}")
        } else {
            format!("rerun for topology revision {topology}")
        },
    );
}

fn violation_targets_component(violation: &DrcViolation, component: &Component) -> bool {
    matches!(
        &violation.location,
        DrcLocation::Component { id, .. } if *id == component.id
    ) || violation
        .related_items
        .iter()
        .any(|item| item.eq_ignore_ascii_case(&component.name))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComponentSoaTone {
    Pass,
    Warning,
    Failure,
    NoEvidence,
}

#[derive(Debug, Clone, PartialEq)]
struct ComponentSoaDisplay {
    label: String,
    tone: ComponentSoaTone,
}

/// Resolve only current, source-attributed SOA evidence for the selected
/// instance. A retained run from another project revision is deliberately
/// ignored; showing its margin beside a changed schematic would be unsafe.
fn retained_component_soa(state: &AppState, component_name: &str) -> ComponentSoaDisplay {
    let Some(run) = state.simulation.active_run() else {
        return no_component_soa();
    };
    let Some(receipt) = run.prepared_receipt() else {
        return no_component_soa();
    };
    if receipt.project_revision() != state.workspace.project.revision() {
        return no_component_soa();
    }

    let mut evaluations = Vec::new();
    let mut violations = 0usize;
    for analysis in &run.analyses {
        if !analysis.success || analysis.provenance.is_none() {
            continue;
        }
        let Some(AnalysisResultPayload::Soa {
            evaluations: retained,
            violations: retained_violations,
        }) = analysis.result_payload.as_ref()
        else {
            continue;
        };
        evaluations.extend(
            retained
                .iter()
                .filter(|evaluation| evaluation.device_id.eq_ignore_ascii_case(component_name)),
        );
        violations += retained_violations
            .iter()
            .filter(|violation| violation.device_id.eq_ignore_ascii_case(component_name))
            .count();
    }

    component_soa_display(&evaluations, violations)
}

fn no_component_soa() -> ComponentSoaDisplay {
    ComponentSoaDisplay {
        label: "No retained device evidence".to_owned(),
        tone: ComponentSoaTone::NoEvidence,
    }
}

fn component_soa_display(
    evaluations: &[&crate::state::SoaEvaluationEvidence],
    violations: usize,
) -> ComponentSoaDisplay {
    if violations > 0 {
        return ComponentSoaDisplay {
            label: format!(
                "{violations} retained violation{}",
                if violations == 1 { "" } else { "s" }
            ),
            tone: ComponentSoaTone::Failure,
        };
    }
    if evaluations.is_empty() {
        return no_component_soa();
    }

    let verdict = evaluations
        .iter()
        .map(|evaluation| evaluation.verdict)
        .max()
        .unwrap_or(SoaRuleVerdictEvidence::Pass);
    let margin_percent = evaluations
        .iter()
        .filter_map(|evaluation| {
            let limit = evaluation.limit_value.abs();
            (limit > f64::EPSILON)
                .then_some((limit - evaluation.worst_actual_value.abs()) / limit * 100.0)
        })
        .min_by(f64::total_cmp);
    let margin = margin_percent
        .map(|value| format!(" · {value:.1}% margin"))
        .unwrap_or_default();
    let (label, tone) = match verdict {
        SoaRuleVerdictEvidence::Pass => (format!("pass{margin}"), ComponentSoaTone::Pass),
        SoaRuleVerdictEvidence::Warning => (format!("warning{margin}"), ComponentSoaTone::Warning),
        SoaRuleVerdictEvidence::Violation | SoaRuleVerdictEvidence::Critical => {
            (format!("failed{margin}"), ComponentSoaTone::Failure)
        }
    };
    ComponentSoaDisplay { label, tone }
}

// =============================================================================
// Net inspector
// =============================================================================

const fn net_icon(class: NetClass) -> WorkbenchIcon {
    match class {
        NetClass::Ground | NetClass::Supply => WorkbenchIcon::Supply,
        NetClass::Signal => WorkbenchIcon::Wire,
    }
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

    schematic_tree_section_header(
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
    let read_only = app.state.active_view_read_only() || app.state.schematic.read_only;
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
        "Sheet size",
        &app.state.schematic.document_policy.page_size_display(),
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};
    use crate::state::{Cell, Library, LibraryCellInstance, Point, PortDirection, View, ViewType};

    fn state_with_two_components() -> AppState {
        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(0, 0));
        state
            .schematic
            .add_component(ComponentType::Capacitor, Point::new(40, 0));
        state
    }

    fn app_with_model_bound_instance() -> (RSpiceApp, u64) {
        let mut app = RSpiceApp::test_instance();
        let mut library = ModelLibrary::new("vendor_analog");
        library.add_model(DeviceModel::new("OPA189_A", ModelType::Other));
        library.add_model(DeviceModel::new("OPA189_B", ModelType::Other));
        library.add_model(DeviceModel::new("unrelated_nmos", ModelType::Nmos));
        app.state.model_library_manager.add_library(library);

        let mut binding = LibraryCellInstance::new("vendor_analog", "OPA189", "spice");
        binding.module_name = Some("OPA189_A".to_owned());
        binding.netlist_template = Some("X{name} {nodes} {model} {params}".to_owned());
        binding.model_section = Some("tt".to_owned());
        let component = Component::new(41, ComponentType::CellInstance, Point::origin())
            .with_library_cell(binding)
            .with_name_value("XU1", "OPA189");
        app.state.schematic.components.push(component);
        app.state.schematic.init_undo_history();
        (app, 41)
    }

    fn validation_slot_height(reason: Option<&str>) -> f32 {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut height = 0.0;
        let _ = ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(312.0);
                height = rejection_slot(ui, reason).height();
            });
        });
        height
    }

    #[test]
    fn inspector_validation_slot_never_reflows_following_sections() {
        assert_eq!(validation_slot_height(None), INLINE_VALIDATION_SLOT_H);
        assert_eq!(
            validation_slot_height(Some(
                "This deliberately long validation message must truncate without growing the slot"
            )),
            INLINE_VALIDATION_SLOT_H
        );
    }

    fn accesskit_nodes(
        mut add_contents: impl FnMut(&mut egui::Ui),
    ) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.run(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| add_contents(ui));
        })
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes
    }

    #[test]
    fn an_empty_selection_inspects_the_sheet() {
        let state = AppState::default();
        assert_eq!(subject(&state, &[]), DesignSubject::Sheet);
    }

    #[test]
    fn one_instance_beats_the_set_and_several_fall_through_to_multi() {
        let mut state = state_with_two_components();
        let ids: Vec<u64> = state
            .schematic
            .components
            .iter()
            .map(|component| component.id)
            .collect();

        state.schematic.selection.select_only_component(ids[0]);
        assert_eq!(subject(&state, &[]), DesignSubject::Component(ids[0]));

        state.schematic.selection.select_component(ids[1]);
        assert_eq!(subject(&state, &[]), DesignSubject::Multi);
    }

    #[test]
    fn conductors_on_one_net_inspect_that_net_and_a_split_falls_back_to_multi() {
        let mut state = AppState::default();
        let nets = vec![
            DesignNet {
                name: "vout".to_owned(),
                authored_name: true,
                class: NetClass::Signal,
                terminals: Vec::new(),
                port: None,
                wire_ids: vec![7, 8],
            },
            DesignNet {
                name: "vin".to_owned(),
                authored_name: true,
                class: NetClass::Signal,
                terminals: Vec::new(),
                port: None,
                wire_ids: vec![9],
            },
        ];

        state.schematic.selection.select_wire(7);
        state.schematic.selection.select_wire(8);
        assert_eq!(
            subject(&state, &nets),
            DesignSubject::Net("vout".to_owned())
        );

        state.schematic.selection.select_wire(9);
        assert_eq!(subject(&state, &nets), DesignSubject::Multi);
    }

    #[test]
    fn wireless_semantic_net_selection_routes_to_the_exact_net_inspector() {
        let mut state = AppState::default();
        state.schematic.components.push(Component::new(
            71,
            ComponentType::CellInstance,
            Point::origin(),
        ));
        state.schematic.selection.select_only_component(71);
        state
            .schematic
            .net_highlight
            .highlight_named_wires("PORT_OUT", HashSet::new());
        let nets = vec![DesignNet {
            name: "PORT_OUT".to_owned(),
            authored_name: true,
            class: NetClass::Signal,
            terminals: vec![crate::simulation::netlist_gen::NetTerminal {
                component_id: 71,
                reference: "X1".to_owned(),
                pin: "OUT".to_owned(),
            }],
            port: Some(PortDirection::Out),
            wire_ids: Vec::new(),
        }];

        assert_eq!(
            subject(&state, &nets),
            DesignSubject::Net("PORT_OUT".to_owned())
        );
        state.schematic.net_highlight.clear();
        assert_eq!(subject(&state, &nets), DesignSubject::Component(71));
    }

    #[test]
    fn explicit_junction_selection_resolves_its_live_net() {
        let mut state = AppState::default();
        state.schematic.wires.push(crate::state::Wire::segment(
            7,
            Point::new(-20, 0),
            Point::new(20, 0),
        ));
        state
            .schematic
            .selection
            .select_only_junction(Point::origin());
        let nets = vec![DesignNet {
            name: "BIAS".to_owned(),
            authored_name: true,
            class: NetClass::Signal,
            terminals: Vec::new(),
            port: None,
            wire_ids: vec![7],
        }];

        assert_eq!(
            subject(&state, &nets),
            DesignSubject::Net("BIAS".to_owned())
        );
    }

    #[test]
    fn a_selected_interface_port_routes_to_the_shared_net_inspector() {
        let mut state = AppState::default();
        let port =
            Component::new(77, ComponentType::Port, Point::origin()).with_name_value("P1", "VIN");
        state.schematic.components.push(port);
        state.schematic.selection.select_only_component(77);
        let nets = vec![DesignNet {
            name: "VIN".to_owned(),
            authored_name: true,
            class: NetClass::Signal,
            terminals: Vec::new(),
            port: Some(PortDirection::In),
            wire_ids: Vec::new(),
        }];

        assert_eq!(subject(&state, &nets), DesignSubject::Net("VIN".to_owned()));
    }

    #[test]
    fn a_conductor_with_no_resolved_net_never_claims_one() {
        let mut state = AppState::default();
        state.schematic.selection.select_wire(42);

        // One unresolved wire is a single selected object, not a net.
        assert_eq!(subject(&state, &[]), DesignSubject::Multi);
    }

    #[test]
    fn isolated_instance_terminals_are_open_not_bound_to_synthetic_nodes() {
        let mut state = AppState::default();
        state.schematic.components.push(Component::new(
            42,
            ComponentType::Resistor,
            Point::origin(),
        ));

        let sheet = sheet_connectivity(&state);
        let terminals = sheet.terminals.get(&42).expect("resistor terminals");

        assert_eq!(terminals.len(), 2);
        assert!(
            terminals.iter().all(|(_, net)| net.is_none()),
            "isolated terminals must not claim the netlister's synthetic node names"
        );
    }

    #[test]
    fn open_terminal_rows_are_disabled_while_bound_rows_remain_actionable() {
        let nodes = accesskit_nodes(|ui| {
            terminal_row(ui, "1", None);
            terminal_row(ui, "2", Some("VOUT"));
        });

        let open = nodes
            .iter()
            .find(|(_, node)| node.label() == Some("1, open"))
            .map(|(_, node)| node)
            .expect("open terminal accessibility node");
        let bound = nodes
            .iter()
            .find(|(_, node)| node.label() == Some("2, VOUT"))
            .map(|(_, node)| node)
            .expect("bound terminal accessibility node");

        assert!(open.is_disabled());
        assert!(!bound.is_disabled());
    }

    #[test]
    fn checks_are_stale_until_they_run_against_the_current_topology() {
        let mut state = AppState::default();
        assert!(!checks_current(&state));
        assert_eq!(checks_status(&state), "stale");

        state.dialogs.drc_results = Some(crate::services::drc::DrcResult::new());
        state.dialogs.drc_checked_version = state.schematic.topology_version();
        assert!(checks_current(&state));
        assert_eq!(checks_status(&state), "0 errors");
    }

    #[test]
    fn sheet_check_rows_report_real_unconnected_and_floating_counts() {
        let mut state = AppState::default();
        let mut result = crate::services::drc::DrcResult::new();
        result.add_violation(crate::services::drc::DrcViolation::new(
            1,
            crate::services::drc::DrcViolationType::UnconnectedPin,
            "R1.+ is open",
            crate::services::drc::DrcLocation::Component {
                id: 1,
                name: "R1".to_owned(),
            },
        ));
        result.add_violation(crate::services::drc::DrcViolation::new(
            2,
            crate::services::drc::DrcViolationType::FloatingNode,
            "net OUT is floating",
            crate::services::drc::DrcLocation::Point { x: 0.0, y: 0.0 },
        ));
        state.dialogs.drc_results = Some(result);
        state.dialogs.drc_checked_version = state.schematic.topology_version();

        assert_eq!(
            current_violation_count(
                &state,
                crate::services::drc::DrcViolationType::UnconnectedPin
            ),
            1
        );
        assert_eq!(
            current_violation_count(&state, crate::services::drc::DrcViolationType::FloatingNode),
            1
        );
    }

    #[test]
    fn soa_display_reports_real_worst_margin_and_failures() {
        let pass = crate::state::SoaEvaluationEvidence {
            device_id: "M1".to_owned(),
            parameter: crate::state::SoaParameterEvidence::DrainSourceVoltage,
            limit_value: 5.0,
            worst_actual_value: 3.0,
            worst_time_s: 2.0e-6,
            sample_count: 101,
            unit: "V".to_owned(),
            description: "Drain-source voltage".to_owned(),
            verdict: SoaRuleVerdictEvidence::Pass,
        };
        let warning = crate::state::SoaEvaluationEvidence {
            device_id: "M1".to_owned(),
            parameter: crate::state::SoaParameterEvidence::PowerDissipation,
            limit_value: 1.0,
            worst_actual_value: 0.75,
            worst_time_s: 3.0e-6,
            sample_count: 101,
            unit: "W".to_owned(),
            description: "Power dissipation".to_owned(),
            verdict: SoaRuleVerdictEvidence::Warning,
        };

        let display = component_soa_display(&[&pass, &warning], 0);
        assert_eq!(display.label, "warning · 25.0% margin");
        assert_eq!(display.tone, ComponentSoaTone::Warning);

        let failed = component_soa_display(&[&pass], 2);
        assert_eq!(failed.label, "2 retained violations");
        assert_eq!(failed.tone, ComponentSoaTone::Failure);
    }

    #[test]
    fn soa_display_does_not_invent_evidence() {
        let display = component_soa_display(&[], 0);
        assert_eq!(display.label, "No retained device evidence");
        assert_eq!(display.tone, ComponentSoaTone::NoEvidence);
    }

    #[test]
    fn writing_a_parameter_preserves_the_other_entries_and_their_order() {
        assert_eq!(write_param("w=2u l=180n", "l", "220n"), "w=2u l=220n");
        assert_eq!(write_param("w=2u l=180n", "m", "4"), "w=2u l=180n m=4");
        assert_eq!(write_param("", "temp", "85"), "temp=85");
    }

    #[test]
    fn clearing_a_parameter_removes_it_so_the_instance_inherits_again() {
        assert_eq!(
            write_param("w=2u temp=85 l=180n", "temp", ""),
            "w=2u l=180n"
        );
        assert_eq!(write_param("temp=85", "temp", "   "), "");
        // A bare flag with no value is left untouched by an unrelated write.
        assert_eq!(write_param("off w=2u", "w", "3u"), "off w=3u");
    }

    #[test]
    fn a_parameter_key_matches_case_insensitively_and_is_written_back_once() {
        assert_eq!(write_param("TEMP=85", "temp", "27"), "temp=27");
        assert_eq!(write_param("W=2u w=3u", "w", "4u"), "w=4u w=4u");
    }

    #[test]
    fn inherited_temperature_materializes_one_undoable_instance_override() {
        let mut app = RSpiceApp::test_instance();
        let id = app
            .state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        app.state.schematic.init_undo_history();
        let component = app.state.schematic.components[0].clone();
        let field = InlineEditField::Parameter(TEMPERATURE_PARAM.to_owned());

        assert_eq!(field_value(&component, &field), "");
        begin_edit(&mut app, &component, field.clone(), String::new());
        assert!(apply_field(&mut app.state, id, &field, "85"));
        commit_edit(&mut app, &edit_description(&field));

        assert_eq!(
            crate::state::parse_params_string(&app.state.schematic.components[0].params)
                .get(TEMPERATURE_PARAM)
                .map(String::as_str),
            Some("85")
        );
        assert!(app.state.schematic.undo());
        assert!(
            !crate::state::parse_params_string(&app.state.schematic.components[0].params)
                .contains_key(TEMPERATURE_PARAM)
        );
        assert!(!app.state.schematic.can_undo());
    }

    #[test]
    fn free_form_parameters_edit_is_atomic_and_undoable() {
        let mut app = RSpiceApp::test_instance();
        let id = app
            .state
            .schematic
            .add_component(ComponentType::CellInstance, Point::origin());
        app.state.schematic.init_undo_history();
        let component = app.state.schematic.components[0].clone();
        let field = InlineEditField::Parameters;

        assert_eq!(field_value(&component, &field), "");
        begin_edit(&mut app, &component, field.clone(), String::new());
        for candidate in ["m=2", "m=2 tc1=0.01"] {
            assert!(apply_field(&mut app.state, id, &field, candidate));
        }
        commit_edit(&mut app, &edit_description(&field));

        assert_eq!(app.state.schematic.components[0].params, "m=2 tc1=0.01");
        assert!(app.state.schematic.undo());
        assert!(app.state.schematic.components[0].params.is_empty());
        assert!(
            !app.state.schematic.can_undo(),
            "one inline session must create exactly one undo entry"
        );
    }

    #[test]
    fn an_instance_rename_is_rejected_when_it_collides_or_is_empty() {
        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(0, 0));
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(40, 0));
        state.schematic.components[0].name = "R1".to_owned();
        state.schematic.components[1].name = "R2".to_owned();
        let subject = state.schematic.components[0].clone();

        assert!(field_rejection(&state, &subject, &InlineEditField::Instance, "R7").is_none());
        assert!(
            field_rejection(&state, &subject, &InlineEditField::Instance, "R1").is_none(),
            "its own name is not a collision"
        );
        let collision = field_rejection(&state, &subject, &InlineEditField::Instance, "r2")
            .expect("case-insensitive collision is rejected");
        assert!(collision.contains("already exists"), "was {collision}");
        assert!(field_rejection(&state, &subject, &InlineEditField::Instance, "  ").is_some());
    }

    #[test]
    fn an_instance_rename_still_obeys_the_family_designator_rule() {
        let mut state = state_with_two_components();
        state.schematic.components[0].name = "R1".to_owned();
        let resistor = state.schematic.components[0].clone();

        let rejected = field_rejection(&state, &resistor, &InlineEditField::Instance, "C1")
            .expect("a resistor cannot take a capacitor designator");
        assert!(rejected.contains('R'), "was {rejected}");
    }

    #[test]
    fn declared_parameters_are_typed_while_unknown_extensions_remain_lossless() {
        let state = state_with_two_components();
        let subject = state.schematic.components[0].clone();

        assert!(field_rejection(&state, &subject, &InlineEditField::Value, "10k").is_none());
        assert!(field_rejection(&state, &subject, &InlineEditField::Value, "").is_none());
        assert!(
            field_rejection(
                &state,
                &subject,
                &InlineEditField::Parameter("noisy".to_owned()),
                "maybe"
            )
            .is_some()
        );
        assert!(
            field_rejection(
                &state,
                &subject,
                &InlineEditField::Parameter("noisy".to_owned()),
                "yes"
            )
            .is_none()
        );
        assert!(
            field_rejection(
                &state,
                &subject,
                &InlineEditField::Parameter("vendor_extension".to_owned()),
                "arbitrary source text"
            )
            .is_none()
        );
    }

    #[test]
    fn fresh_standard_instances_project_their_complete_typed_parameter_contract() {
        let state = AppState::default();
        let component = Component::new(7, ComponentType::Resistor, Point::origin());
        let contract = inline_parameter_contract(&state, &component, &HashMap::new());
        let keys = contract
            .iter()
            .map(|parameter| parameter.key.as_str())
            .collect::<Vec<_>>();

        assert!(keys.contains(&"m"));
        assert!(keys.contains(&"tc1"));
        assert!(keys.contains(&"noisy"));
        assert!(!keys.contains(&"r"), "the primary value is owned by Value");
        assert!(
            contract
                .iter()
                .find(|parameter| parameter.key == "m")
                .is_some_and(|parameter| parameter.label == "Multiplier" && parameter.hint == "1")
        );
    }

    #[test]
    fn component_identity_reports_occurrence_and_graphical_execution_views() {
        let mut state = AppState::default();
        state.workspace.descend_into(
            "XAFE".to_owned(),
            CellViewRef::new("user", "afe_core", "schematic"),
            ViewType::Schematic,
        );
        let mut binding = LibraryCellInstance::new("user", "precision_r", "symbol");
        binding.netlist_template = Some("R{name} {nodes} {model} {params}".to_owned());
        let component = Component::new(9, ComponentType::CellInstance, Point::origin())
            .with_library_cell(binding)
            .with_name_value("RGAIN", "499");

        assert_eq!(
            component_occurrence_path(&state, &component),
            "/top/XAFE/RGAIN"
        );
        assert_eq!(component_view_contract(&component), "symbol · spice");
    }

    #[test]
    fn literal_value_tuning_stages_a_typed_variable_without_mutating_authority() {
        let mut app = RSpiceApp::test_instance();
        let component_id = app
            .state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        app.state.schematic.components[0].name = "RLOAD".to_owned();
        app.state.schematic.components[0].value = "10k".to_owned();
        let plan_id = app.state.sim_setup.stable_analysis_plan().unwrap().id();
        let variables_before = app
            .state
            .workspace
            .active_plan_data(plan_id)
            .unwrap()
            .design_variables
            .len();
        let topology_before = app.state.schematic.topology_version();

        stage_component_tuning(&mut app, component_id).expect("resistor literal is tunable");

        assert_eq!(app.state.schematic.components[0].value, "10k");
        assert_eq!(app.state.schematic.topology_version(), topology_before);
        assert_eq!(
            app.state
                .workspace
                .active_plan_data(plan_id)
                .unwrap()
                .design_variables
                .len(),
            variables_before
        );
        let binding = app
            .state
            .workbench
            .verification
            .tuning_instance_binding
            .as_ref()
            .expect("transient instance binding");
        assert!(binding.creates_variable);
        assert_eq!(binding.variable.name, "RLOAD_VALUE");
        assert_eq!(
            binding.variable.quantity,
            crate::state::DesignVariableQuantity::Resistance
        );
        assert!(binding.variable.allowed_range.is_none());
        assert_eq!(binding.binding_expression, "{RLOAD_VALUE}");
        assert!(
            app.state
                .workbench
                .verification
                .tuning_variables
                .iter()
                .any(|draft| draft.variable_id == binding.variable.id && draft.proposed)
        );
    }

    #[test]
    fn parameter_bound_value_tuning_selects_the_existing_typed_variable() {
        let mut app = RSpiceApp::test_instance();
        let component_id = app
            .state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        app.state.schematic.components[0].value = "{RGAIN}".to_owned();
        let plan_id = app.state.sim_setup.stable_analysis_plan().unwrap().id();
        let variable = crate::state::DesignVariable::new(
            "RGAIN",
            "499 ohm",
            crate::state::DesignVariableQuantity::Resistance,
            crate::state::DesignVariableScope::Project,
            "Gain resistor",
            None,
            crate::state::DesignVariableSweepEligibility::NestedSweepAndOptimization,
            crate::state::DesignVariableOverridePolicy::ExplicitTestLocalOverride,
        )
        .unwrap();
        let variable_id = variable.id;
        app.state
            .workspace
            .add_design_variable(plan_id, variable)
            .unwrap();

        stage_component_tuning(&mut app, component_id)
            .expect("existing typed parameter reference is tunable");

        let session = &app.state.workbench.verification;
        let binding = session
            .tuning_instance_binding
            .as_ref()
            .expect("instance context is retained");
        assert!(!binding.creates_variable);
        assert_eq!(binding.variable.id, variable_id);
        assert_eq!(session.tuning_selected_variable, Some(variable_id));
        assert_eq!(session.tuning_focus_variable, Some(variable_id));
        assert!(session.tuning_variables.iter().all(|draft| !draft.proposed));
        assert_eq!(
            app.state
                .workspace
                .active_plan_data(plan_id)
                .unwrap()
                .design_variables
                .len(),
            1
        );
    }

    #[test]
    fn value_tuning_fails_closed_when_no_truthful_quantity_exists() {
        let mut app = RSpiceApp::test_instance();
        let component_id = app
            .state
            .schematic
            .add_component(ComponentType::Inductor, Point::origin());
        app.state.schematic.components[0].value = "10u".to_owned();

        let error = stage_component_tuning(&mut app, component_id)
            .expect_err("inductance is not representable by the current typed variable schema");

        assert!(error.contains("truthful typed design-variable mapping"));
        assert!(
            app.state
                .workbench
                .verification
                .tuning_instance_binding
                .is_none()
        );
    }

    #[test]
    fn applying_a_field_reports_whether_the_design_actually_changed() {
        let mut state = state_with_two_components();
        let id = state.schematic.components[0].id;
        let before = state.schematic.topology_version();

        assert!(apply_field(&mut state, id, &InlineEditField::Value, "10k"));
        assert_eq!(state.schematic.components[0].value, "10k");
        assert!(state.schematic.topology_version() > before);

        let settled = state.schematic.topology_version();
        assert!(
            !apply_field(&mut state, id, &InlineEditField::Value, "10k"),
            "rewriting the same text is not a change"
        );
        assert_eq!(
            state.schematic.topology_version(),
            settled,
            "an unchanged write must not advance topology"
        );
    }

    #[test]
    fn model_choices_stay_inside_the_bound_library_and_device_family() {
        let (app, id) = app_with_model_bound_instance();
        let component = app
            .state
            .schematic
            .components
            .iter()
            .find(|component| component.id == id)
            .expect("bound component");
        let choices = bound_model_choices(&app.state, component, "OPA189_A");
        let values = choices
            .iter()
            .map(|(value, _)| value.as_str())
            .collect::<Vec<_>>();
        assert_eq!(values, ["OPA189_A", "OPA189_B"]);
        assert_eq!(
            catalog_model_location(&app.state, component),
            Some(("vendor_analog".to_owned(), "OPA189_A".to_owned()))
        );
    }

    #[test]
    fn a_veriloga_cell_opens_its_exact_code_source_target() {
        let mut state = AppState::default();
        let mut cell = Cell::new("sensor_bridge");
        cell.add_view(View::new("veriloga", ViewType::VerilogA));
        let mut library = Library::new("behavioral");
        library.add_cell(cell);
        state.library_manager.add_library(library);
        let binding = LibraryCellInstance::new("behavioral", "sensor_bridge", "veriloga");
        let component = Component::new(7, ComponentType::CellInstance, Point::origin())
            .with_library_cell(binding)
            .with_name_value("XBRIDGE", "sensor_bridge");

        assert_eq!(
            component_model_source_target(&state, &component),
            Some(ComponentModelSourceTarget::VerilogA(CellViewRef::new(
                "behavioral",
                "sensor_bridge",
                "veriloga"
            )))
        );
    }

    #[test]
    fn selecting_a_bound_model_is_atomic_undoable_and_netlist_authoritative() {
        let (mut app, id) = app_with_model_bound_instance();
        let before = app.state.schematic.topology_version();
        apply_bound_model_choice(&mut app, id, "OPA189_B");
        let binding = app.state.schematic.components[0]
            .library_cell
            .as_ref()
            .expect("binding");
        assert_eq!(binding.module_name.as_deref(), Some("OPA189_B"));
        assert!(app.state.schematic.topology_version() > before);
        assert!(app.state.schematic.can_undo());

        assert!(app.state.schematic.undo());
        assert_eq!(
            app.state.schematic.components[0]
                .library_cell
                .as_ref()
                .and_then(|binding| binding.module_name.as_deref()),
            Some("OPA189_A")
        );
    }

    #[test]
    fn the_default_section_choice_removes_the_instance_override() {
        let (mut app, id) = app_with_model_bound_instance();
        apply_bound_model_section(&mut app, id, "");
        assert_eq!(
            app.state.schematic.components[0]
                .library_cell
                .as_ref()
                .and_then(|binding| binding.model_section.as_deref()),
            None
        );
        assert!(app.state.schematic.can_undo());
    }

    #[test]
    fn an_inline_session_folds_its_keystrokes_into_one_undo_entry() {
        let mut state = state_with_two_components();
        let id = state.schematic.components[0].id;
        state.schematic.init_undo_history();
        let before = crate::state::SchematicSnapshot::capture(&state.schematic);

        for text in ["1", "1k", "1k5"] {
            apply_field(&mut state, id, &InlineEditField::Value, text);
        }
        assert!(
            state
                .schematic
                .commit_undo_from(before, "edit instance value")
        );
        assert_eq!(state.schematic.components[0].value, "1k5");

        assert!(state.schematic.undo());
        assert_ne!(state.schematic.components[0].value, "1k5");
        assert!(
            !state.schematic.can_undo(),
            "three keystrokes produced more than one undo step"
        );
    }

    #[test]
    fn the_hero_band_matches_the_mockup_geometry() {
        assert_eq!(HERO_H, 82.0);
        assert_eq!(HERO_PREVIEW_W, 82.0);
        assert_eq!(HERO_BASELINES, [12.0, 31.0, 49.0, 68.0]);
        assert!(HERO_BASELINES.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(HERO_BASELINES[3] < HERO_H);
    }

    #[test]
    fn operating_point_summary_matches_the_upgraded_inset_contract() {
        assert_eq!(OP_SUMMARY_MARGIN_X, 8.0);
        assert_eq!(OP_SUMMARY_PADDING, 9.0);
        assert_eq!(OP_SUMMARY_ROW_H, 22.0);
    }
}
