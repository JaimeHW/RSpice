//! Selection-dispatched design inspector.
//!
//! The right panel inspects exactly one object class at a time, chosen from
//! the schematic selection: an instance, a conductor, a documentation
//! object, the whole selected set, or — when nothing is selected — the sheet
//! itself. Every row is backed by live document state; no panel narrates a
//! fact the open design cannot supply.

use std::collections::HashMap;

use egui::{Color32, Ui};

use crate::common::{AppState, RSpiceApp};
use crate::simulation::netlist_gen::{
    DesignNet, HierarchySource, NetClass, design_nets_with_hierarchy,
};
use crate::state::{CellViewRef, Component, ComponentType};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, TreeRow};
use crate::workbench::commands::{Command, CommandAvailability};
use crate::workbench::design_system::{
    StatusMark, WorkbenchIcon, property_row, property_row_input, property_row_status,
};
use crate::workbench::state::InlineEditField;

use super::{
    ComponentModelEvidence, component_model_evidence, muted_inspector_copy, section_header,
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
    ids.extend(selection.wire_segments.iter().map(|segment| segment.wire_id));
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
    if let Some(id) = selection.single_component() {
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
    let hierarchy =
        HierarchySource::from_workspace(&state.library_manager, &state.workspace.schematic_buffers);
    let nets = design_nets_with_hierarchy(&state.schematic, &hierarchy);

    let mut bound: HashMap<(u64, &str), &str> = HashMap::new();
    for net in &nets {
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
    Glyph(&'static str),
}

struct Hero {
    preview: HeroPreview,
    eyebrow: String,
    title: String,
    subtitle: String,
    status: String,
    status_tone: Color32,
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
        HeroPreview::Glyph(glyph) => {
            ui.painter().text(
                preview.center(),
                egui::Align2::CENTER_CENTER,
                glyph,
                theme::mono(tokens::FS_4, FontWeight::Regular),
                t.color.symbol,
            );
        }
    }

    let text_left = preview.right() + 10.0;
    let text_clip = egui::Rect::from_x_y_ranges(text_left..=(rect.right() - 10.0), rect.y_range());
    let painter = ui.painter().with_clip_rect(text_clip);
    let at = |index: usize| egui::pos2(text_left, rect.top() + HERO_BASELINES[index]);
    painter.text(
        at(0),
        egui::Align2::LEFT_CENTER,
        &spec.eyebrow,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
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
    painter.text(
        at(3),
        egui::Align2::LEFT_CENTER,
        &spec.status,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        spec.status_tone,
    );

    if let Some(component_id) = spec.open_properties {
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                "Open selected component properties",
            )
        });
        if response.double_clicked() && !app.state.active_view_read_only() {
            crate::common::app::open_property_editor(&mut app.state, component_id);
        }
        theme::paint_focus_ring(ui, &response, rect);
        response.on_hover_text("Double-click to edit component properties");
    }
}

// =============================================================================
// Shared section pieces
// =============================================================================

/// A stacked, full-width action group matching the mockup's
/// `.section-body.panel-action-stack`.
fn action_stack(ui: &mut Ui, body: impl FnOnce(&mut Ui)) {
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        let width = (ui.available_width() - 10.0).max(1.0);
        ui.allocate_ui(egui::vec2(width, 0.0), |ui| {
            ui.spacing_mut().item_spacing.y = 6.0;
            body(ui);
        });
    });
    ui.add_space(2.0);
}

/// A full-width action button inside an [`action_stack`].
fn stacked_button<'a>(ui: &Ui, button: Button<'a>) -> Button<'a> {
    let width = ui.available_width().max(1.0);
    button.min_width(width).max_width(width)
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
    let response = stacked_button(
        ui,
        Button::new(label)
            .icon(icon)
            .destructive(destructive)
            .enabled(availability.is_available()),
    )
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
        InlineEditField::Parameter(key) => crate::properties::parse_params_string(&component.params)
            .get(key)
            .cloned()
            .unwrap_or_default(),
    }
}

/// Why `candidate` cannot be applied to `field`, if it cannot.
///
/// Only the reference designator has legality rules: SPICE designators must
/// parse and must be case-insensitively unique across the sheet. Values and
/// parameters accept any text, exactly as the property dialog does.
fn field_rejection(
    state: &AppState,
    component: &Component,
    field: &InlineEditField,
    candidate: &str,
) -> Option<String> {
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
        InlineEditField::Parameter(key) => format!("edit {key}"),
    }
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
) {
    let editable = !app.state.active_view_read_only() && !app.state.schematic.read_only;
    if !editable {
        property_row(ui, label, &field_value(component, &field));
        return;
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

    let response = property_row_input(ui, label, &mut buffer, rejection.is_some());
    if let Some(reason) = rejection.as_deref() {
        rejection_note(ui, reason);
    }

    if response.gained_focus() {
        begin_edit(app, component, field.clone(), buffer.clone());
    }
    if response.changed() {
        begin_edit(app, component, field.clone(), buffer.clone());
        app.state
            .workbench
            .inline_edit
            .set_buffer(buffer.clone());
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
}

/// The reason a typed value has not been applied, in the error tone.
fn rejection_note(ui: &mut Ui, reason: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_wrapped(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new(reason)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.err),
        );
    });
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

    hero(
        ui,
        app,
        Hero {
            preview: HeroPreview::Symbol(component.kind),
            eyebrow: format!(
                "{} · /{}/{}",
                component.name, app.state.workspace.active_view.cell, component.name
            ),
            title: if component.value.trim().is_empty() {
                component.kind.display_name().to_owned()
            } else {
                component.value.clone()
            },
            subtitle: component.kind.display_name().to_owned(),
            status: evidence.status.clone(),
            status_tone: evidence.tone.color(&Tokens::get(ui.ctx())),
            open_properties: Some(component.id),
        },
    );

    identity_section(ui, app, &component);
    parameters_section(ui, app, &component, &evidence);
    connectivity_section(ui, app, &component, sheet);
    operating_point(ui, app, &component);
    component_checks(ui, app);
}

fn identity_section(ui: &mut Ui, app: &mut RSpiceApp, component: &Component) {
    section_header(ui, "Identity", Some("editable"));
    edit_row(ui, app, component, InlineEditField::Instance, "Instance");
    edit_row(ui, app, component, InlineEditField::Value, "Value");
    let library_cell = component.library_cell.as_ref().map_or_else(
        || format!("primitives/{}", component.kind.display_name()),
        |binding| format!("{}/{}", binding.library, binding.cell),
    );
    property_row(ui, "Library cell", &library_cell);
    property_row(
        ui,
        "View",
        component
            .library_cell
            .as_ref()
            .map_or("symbol · spice", |binding| binding.view.as_str()),
    );

    // Identity actions exist only when the design can actually perform
    // them: the editable-properties transaction always, the symbol
    // cellview when the instance is bound to one, and the descend
    // transaction when the command registry accepts this selection.
    let symbol_view = component.library_cell.as_ref().map(|binding| {
        CellViewRef::new(
            binding.library.clone(),
            binding.cell.clone(),
            "symbol".to_owned(),
        )
    });
    let descend = Command::DescendHierarchy.availability(app);
    action_stack(ui, |ui| {
        let editable = !app.state.active_view_read_only();
        let response = stacked_button(
            ui,
            Button::new("Properties…").ghost().enabled(editable),
        )
        .show(ui);
        if response.clicked() {
            crate::common::app::open_property_editor(&mut app.state, component.id);
        }
        response.on_disabled_hover_text("the active document is read-only");

        if let Some(reference) = symbol_view {
            let exists = app
                .state
                .library_manager
                .get_library(&reference.library)
                .and_then(|library| library.get_cell(&reference.cell))
                .and_then(|cell| cell.get_view(&reference.view))
                .is_some();
            let response = stacked_button(
                ui,
                Button::new("Edit symbol…").ghost().enabled(exists),
            )
            .show(ui);
            if response.clicked() {
                app.state.open_workspace_view(reference);
            }
            response.on_disabled_hover_text("this cell has no symbol view");
        }

        if descend.is_available() {
            let label = component.library_cell.as_ref().map_or_else(
                || "Descend into instance".to_owned(),
                |binding| format!("Descend into {}", binding.cell),
            );
            if stacked_button(ui, Button::new(&label).ghost())
                .show(ui)
                .clicked()
            {
                Command::DescendHierarchy.execute(app);
            }
        }
    });
}

fn parameters_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    component: &Component,
    evidence: &ComponentModelEvidence,
) {
    section_header(ui, "Simulation parameters", Some("editable"));
    // Model, source, and section are resolved evidence, not instance
    // fields: they follow the library binding, which the Replace instance
    // and property transactions own.
    property_row(ui, "Model", &evidence.model);
    property_row(ui, "Source", &evidence.source);
    property_row(ui, "Section", &evidence.section);

    // Instance temperature is the `temp` parameter. Left empty it inherits
    // the reference PVT point, which the row reports as its placeholder.
    let declared = crate::properties::parse_params_string(&component.params);
    let inherited = format!(
        "inherit · {} °C",
        app.state.sim_setup.reference_pvt.temperature_celsius
    );
    if declared.contains_key(TEMPERATURE_PARAM) {
        edit_row(
            ui,
            app,
            component,
            InlineEditField::Parameter(TEMPERATURE_PARAM.to_owned()),
            "Temperature",
        );
    } else {
        let response = property_row(ui, "Temperature", &inherited);
        if response.clicked() {
            begin_edit(
                app,
                component,
                InlineEditField::Parameter(TEMPERATURE_PARAM.to_owned()),
                String::new(),
            );
        }
    }

    // Every declared instance parameter, one typed row each.
    let mut keys: Vec<String> = declared
        .keys()
        .filter(|key| key.as_str() != TEMPERATURE_PARAM)
        .cloned()
        .collect();
    keys.sort_unstable();
    if keys.is_empty() && declared.is_empty() {
        property_row(ui, "Parameters", "instance value");
    }
    for key in keys {
        let label = key.clone();
        edit_row(ui, app, component, InlineEditField::Parameter(key), &label);
    }
}

/// SPICE instance-temperature parameter.
const TEMPERATURE_PARAM: &str = "temp";

/// Per-pin terminal table: every declared terminal with the net it binds,
/// clickable to select that conductor. Unbound pins read `open` and are
/// not clickable, because there is no net to select.
fn connectivity_section(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    component: &Component,
    sheet: &SheetConnectivity,
) {
    let empty = Vec::new();
    let bindings = sheet.terminals.get(&component.id).unwrap_or(&empty);
    let open_pins = bindings.iter().filter(|(_, net)| net.is_none()).count();

    section_header(
        ui,
        "Connectivity",
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
        let meta = net.as_deref().unwrap_or("open");
        let row = TreeRow::new(pin).mono().indent(1).meta(meta).show(ui);
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
                    .on_hover_text("Unconnected pin · wire it or run connectivity checks");
            }
        }
    }
    if let Some(name) = select
        && let Some(net) = sheet.nets.iter().find(|net| net.name == name)
    {
        select_net(app, net);
    }
}

fn operating_point(ui: &mut Ui, app: &RSpiceApp, component: &Component) {
    let retained = app.state.simulation.runs.iter().find_map(|run| {
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
                        )
                    })
            })
        })
    });
    if let Some((run_id, analysis, region, params)) = retained {
        section_header(
            ui,
            &format!("Operating point · Run {run_id}"),
            Some("retained"),
        );
        if let Some(region) = region {
            property_row(ui, "Region", region);
        }
        for (name, value) in params.into_iter().take(4) {
            property_row(ui, name, &format!("{value:.6e}"));
        }
        property_row(ui, "Analysis", &analysis);
    } else {
        section_header(ui, "Operating point", Some("no evidence"));
        property_row(ui, "Selection", "No retained device operating point");
        property_row(ui, "Required analysis", "DC operating point");
    }
}

fn component_checks(ui: &mut Ui, app: &RSpiceApp) {
    let topology = app.state.schematic.topology_version();
    let current = checks_current(&app.state);
    let summary = current
        .then(|| {
            app.state
                .dialogs
                .drc_results
                .as_ref()
                .map(|result| result.summary())
        })
        .flatten();
    let finding_count = summary.map_or(0, |summary| summary.critical + summary.errors);
    section_header(ui, "Checks", Some(&checks_status(&app.state)));
    let t = Tokens::get(ui.ctx());
    let (connectivity, tone, mark) = if current && summary.is_some() {
        if finding_count == 0 {
            ("checked", t.color.ok, StatusMark::Success)
        } else {
            ("findings present", t.color.warn, StatusMark::Warning)
        }
    } else {
        ("pending recheck", t.color.warn, StatusMark::Warning)
    };
    property_row_status(ui, "Connectivity", connectivity, tone, mark);
    property_row(ui, "Safe operating area", "Dataset attribution unavailable");
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

// =============================================================================
// Net inspector
// =============================================================================

const fn net_glyph(class: NetClass) -> &'static str {
    match class {
        NetClass::Ground => "\u{23da}",
        NetClass::Supply => "\u{2261}",
        NetClass::Signal => "\u{2501}",
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
            preview: HeroPreview::Glyph(net_glyph(class)),
            eyebrow: format!(
                "NET · SHEET {}",
                app.state.workspace.active_view.cell.to_ascii_uppercase()
            ),
            title: net_name.clone(),
            subtitle: scope.clone(),
            status: class.keyword().to_owned(),
            status_tone: net_class_tone(ui, class),
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
                Some("retained"),
            );
            property_row(
                ui,
                "Node voltage",
                &format!(
                    "{} {}",
                    crate::properties::format_engineering_value(annotation.voltage),
                    annotation.unit
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
    section_header(ui, "Checks", Some(&checks_status(&app.state)));
    property_row_status(
        ui,
        "Connectivity",
        if !current {
            "pending recheck"
        } else if terminals.len() >= 2 {
            "conductor closed"
        } else {
            "declared"
        },
        tone_for(ui, current),
        mark_for(current),
    );
    property_row_status(
        ui,
        "Name collisions",
        if current {
            "unique on sheet"
        } else {
            "pending recheck"
        },
        tone_for(ui, current),
        mark_for(current),
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
                if stacked_button(ui, Button::new(&label).icon(Icon::Results))
                    .show(ui)
                    .clicked()
                {
                    crate::schematic::view::toggle_probe_with_feedback(
                        ui,
                        &mut app.state,
                        &net_name,
                        &display,
                    );
                }
            }
            if !connected.is_empty()
                && stacked_button(ui, Button::new("Select connected instances").ghost())
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
}

/// The retained DC node voltage for a net, if any run reported one.
fn net_operating_point(state: &AppState, net: &str) -> Option<NetAnnotation> {
    let bare = net.to_ascii_lowercase();
    let wrapped = format!("v({bare})");
    state.simulation.runs.iter().find_map(|run| {
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
                    })
            })
        })
    })
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
    app.state
        .schematic
        .net_highlight
        .highlight_wires(net.wire_ids.iter().copied().collect());
}

// =============================================================================
// Sheet inspector
// =============================================================================

fn sheet_panel(ui: &mut Ui, app: &mut RSpiceApp, nets: &[DesignNet]) {
    let reference = app.state.workspace.active_view.clone();
    let read_only = app.state.active_view_read_only();
    let current = checks_current(&app.state);
    let dirty = active_view_dirty(&app.state);
    let depth = app.state.workspace.hierarchy_stack.len().saturating_sub(1);

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
            status: if dirty { "unsaved" } else { "saved" }.to_owned(),
            status_tone: tone_for(ui, !dirty),
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
        "Access",
        if read_only { "Read only" } else { "Editable" },
    );
    property_row(
        ui,
        "Grid / snap",
        &format!(
            "{} units · snap {}",
            app.state.schematic.grid_size,
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
            "root".to_owned()
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
    property_row(
        ui,
        "Net labels",
        &app.state.schematic.net_labels.len().to_string(),
    );
    property_row(
        ui,
        "Design notes",
        &app.state.schematic.design_notes.len().to_string(),
    );
    property_row(
        ui,
        "Documentation shapes",
        &app.state.schematic.documentation_shapes.len().to_string(),
    );

    section_header(ui, "Checks", Some(&checks_status(&app.state)));
    property_row_status(
        ui,
        "Connectivity",
        if current {
            "all pins bound"
        } else {
            "pending recheck"
        },
        tone_for(ui, current),
        mark_for(current),
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
            preview: HeroPreview::Glyph("\u{25c7}"),
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
            status: "group transforms".to_owned(),
            status_tone: Tokens::get(ui.ctx()).color.accent,
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
            Command::MirrorSelectionVertical,
            Icon::Mirror,
            "Mirror about vertical axis",
            false,
        );
        command_action(ui, app, Command::Duplicate, Icon::Copy, "Duplicate…", false);
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
            status: note.layer.label().to_owned(),
            status_tone: Tokens::get(ui.ctx()).color.text_dim,
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
            status: shape.layer.label().to_owned(),
            status_tone: Tokens::get(ui.ctx()).color.text_dim,
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
    use crate::state::Point;

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
                class: NetClass::Signal,
                terminals: Vec::new(),
                port: None,
                wire_ids: vec![7, 8],
            },
            DesignNet {
                name: "vin".to_owned(),
                class: NetClass::Signal,
                terminals: Vec::new(),
                port: None,
                wire_ids: vec![9],
            },
        ];

        state.schematic.selection.select_wire(7);
        state.schematic.selection.select_wire(8);
        assert_eq!(subject(&state, &nets), DesignSubject::Net("vout".to_owned()));

        state.schematic.selection.select_wire(9);
        assert_eq!(subject(&state, &nets), DesignSubject::Multi);
    }

    #[test]
    fn a_conductor_with_no_resolved_net_never_claims_one() {
        let mut state = AppState::default();
        state.schematic.selection.select_wire(42);

        // One unresolved wire is a single selected object, not a net.
        assert_eq!(subject(&state, &[]), DesignSubject::Multi);
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
    fn writing_a_parameter_preserves_the_other_entries_and_their_order() {
        assert_eq!(write_param("w=2u l=180n", "l", "220n"), "w=2u l=220n");
        assert_eq!(write_param("w=2u l=180n", "m", "4"), "w=2u l=180n m=4");
        assert_eq!(write_param("", "temp", "85"), "temp=85");
    }

    #[test]
    fn clearing_a_parameter_removes_it_so_the_instance_inherits_again() {
        assert_eq!(write_param("w=2u temp=85 l=180n", "temp", ""), "w=2u l=180n");
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
    fn values_and_parameters_accept_any_text() {
        let state = state_with_two_components();
        let subject = state.schematic.components[0].clone();

        assert!(field_rejection(&state, &subject, &InlineEditField::Value, "10k").is_none());
        assert!(field_rejection(&state, &subject, &InlineEditField::Value, "").is_none());
        assert!(
            field_rejection(
                &state,
                &subject,
                &InlineEditField::Parameter("tc1".to_owned()),
                "25p"
            )
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
    fn an_inline_session_folds_its_keystrokes_into_one_undo_entry() {
        let mut state = state_with_two_components();
        let id = state.schematic.components[0].id;
        state.schematic.init_undo_history();
        let before = crate::state::SchematicSnapshot::capture(&state.schematic);

        for text in ["1", "1k", "1k5"] {
            apply_field(&mut state, id, &InlineEditField::Value, text);
        }
        assert!(state.schematic.commit_undo_from(before, "edit instance value"));
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
}
