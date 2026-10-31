//! Selection-dispatched design inspector.
//!
//! The right panel inspects exactly one object class at a time, chosen from
//! the schematic selection: an instance, a conductor, a documentation
//! object, the whole selected set, or — when nothing is selected — the sheet
//! itself. Every row is backed by live document state; no panel narrates a
//! fact the open design cannot supply.

mod component;
mod model_evidence;

pub(crate) use component::{analysis_soa_display, bind_component_model_from_catalog};

use component::*;
use model_evidence::{ComponentModelEvidence, component_model_evidence};

use std::collections::{HashMap, HashSet};

use egui::Ui;

use crate::schematic::view::sheet_visibility::{self, SheetScope};
use crate::state::{
    AnalysisResultPayload, CellViewRef, Component, ComponentType, DisplayMode, PropertyDefinition,
    PropertyType, PropertyValue, SoaRuleVerdictEvidence,
};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, TreeRow, TreeRowResult};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::{
    Problem, PropertyRowCaption, WorkbenchIcon, problem_line, property_row_captioned,
    property_row_combo, property_row_flagged, property_row_input_action,
    property_row_input_captioned, property_row_input_with_hint,
    schematic_property_row as property_row, schematic_property_row_problem as property_row_problem,
};
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::host::{Host, InspectorScope, Point};
use crate::workbench::framework::requests::RevealModel;
use crate::workbench::state::{InlineEditField, VerificationPage, Workspace};
use crate::workbench::{AppState, RSpiceApp};
use rspice_design::connectivity::extract_with_hierarchy as extract;
use rspice_design::connectivity::summary::{DesignNet, NetClass};
use rspice_design::drc::{DrcLocation, DrcViolation};
use rspice_design::hierarchy::HierarchySource;

use super::shared::{
    action_stack, checks_current, count_label, current_findings, findings_problem,
};
use rspice_ui_kit::panels::inspector::muted_inspector_copy;
use rspice_ui_kit::panels::{
    panel_bleed, panel_inset, section_block_gap, section_header, section_header_action,
    section_header_explained,
};

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
    let selection = &state.model.schematic.session.editor.selection;
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
    let selection = &state.model.schematic.session.editor.selection;
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
    let selection = &state.model.schematic.session.editor.selection;
    if let Some(name) = navigator_selected_net_name(state, nets) {
        return DesignSubject::Net(name);
    }
    if let Some(id) = selection.single_component() {
        if let Some(port) = state
            .model
            .schematic
            .document()
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
    let authority = state
        .model
        .schematic
        .session
        .editor
        .net_highlight
        .selected_net_name
        .as_deref()?;
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
    if state
        .model
        .schematic
        .session
        .editor
        .net_highlight
        .highlighted_wires
        != wire_ids.iter().copied().collect()
    {
        return None;
    }

    let selection = &state.model.schematic.session.editor.selection;
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
    for label in &state.model.schematic.document().net_labels {
        if state
            .model
            .schematic
            .session
            .editor
            .selection
            .net_labels
            .contains(&label.id)
            && !accept(&label.name)
        {
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
    if !state
        .model
        .schematic
        .session
        .editor
        .selection
        .junctions
        .is_empty()
    {
        // A junction marker names the conductors the one extraction puts on
        // its node, so the inspector resolves the same net the deck emits. A
        // marker standing mid-conductor is not itself a node of the traced
        // graph: geometry says which conductor lies under it and the extraction
        // still says which net that conductor is on, so membership is never
        // re-derived here. An ambiguous crossing takes one stable seed rather
        // than joining two electrical nets.
        let connectivity = extract(&state.model.schematic, None);
        for junction in &state.model.schematic.session.editor.selection.junctions {
            let Some(extracted) = connectivity.net_at(junction.pos).or_else(|| {
                state
                    .model
                    .schematic
                    .document()
                    .wires
                    .iter()
                    .filter(|wire| wire.contains_point(junction.pos))
                    .min_by_key(|wire| wire.id)
                    .and_then(|wire| connectivity.net_of_wire(wire.id))
            }) else {
                continue;
            };
            for wire_id in &extracted.wires {
                let Some(net) = nets.iter().find(|net| net.wire_ids.contains(wire_id)) else {
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

pub(in crate::workbench) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    if let Some(session) = app.state.workbench.inline_edit.session()
        && (session.authority != app.state.inline_edit_authority()
            || app.state.schematic_edit_read_only()
            || !app
                .state
                .model
                .schematic
                .document()
                .components
                .contains(&session.expected))
    {
        if session.candidate.as_ref() == Some(&session.expected) {
            app.state.workbench.inline_edit.end();
        } else {
            muted_inspector_copy(
                ui,
                "The design changed under your edit. Your text was not applied.",
            );
            // Wrapped rather than elided: the draft is the reader's own unsaved
            // text, and all of it has to stay readable until it is discarded.
            crate::workbench::design_system::property_row_wrapped(
                ui,
                &session.expected.name,
                &session.buffer,
            );
            if ui.button("Discard inspector draft").clicked() {
                app.state.workbench.inline_edit.end();
            }
            return;
        }
    }
    let mut sheet = sheet_connectivity(&app.state);
    if let Some(reason) = sheet.unresolved.as_deref() {
        unresolved_projection_note(ui, reason);
    }
    let mut inspected = subject(&app.state, &sheet.nets);

    // Resolve a pending field before the selection can adopt a different owner.
    let owner = match inspected {
        DesignSubject::Component(id) => Some(id),
        _ => None,
    };
    if let Some(editing) = app
        .state
        .workbench
        .inline_edit
        .session()
        .map(|session| session.expected.id)
        && owner != Some(editing)
    {
        match app.state.commit_inline_component_edit() {
            Ok(true) => sheet = sheet_connectivity(&app.state),
            Ok(false) => {}
            Err(_) => {
                app.state
                    .model
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_component(editing);
                inspected = DesignSubject::Component(editing);
            }
        }
    }

    if inspected != DesignSubject::Sheet {
        app.commit_inspector_section(InspectorScope::Document(Workspace::Design), ui.ctx());
    }

    match inspected {
        DesignSubject::Component(id) => component_panel(ui, app, id, &sheet),
        DesignSubject::Net(name) => net_panel(ui, app, &name, &sheet.nets),
        DesignSubject::Note(id) => note_panel(ui, app, id),
        DesignSubject::Shape(id) => shape_panel(ui, app, id),
        DesignSubject::Probe(id) => probe_panel(ui, app, id),
        DesignSubject::Multi => multi_panel(ui, app),
        DesignSubject::Sheet => app.contribute(
            Point::Inspector(InspectorScope::Document(Workspace::Design)),
            ui,
        ),
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
    let projection = match state.model.workspace.design_projection(
        &state.model.library_manager,
        &state.model.workspace.content.active_view,
        &state.model.schematic,
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
    let hierarchy =
        HierarchySource::from_design_projection(state.model.library_manager.catalog(), &projection);
    let nets = rspice_design::connectivity::summary::projection_nets(
        state.model.library_manager.catalog(),
        &projection,
        &state.model.workspace.content.active_view.key(),
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
        .model
        .schematic
        .document()
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
    problem_line(ui, Problem::Warning, reason);
}

// =============================================================================
// Title
// =============================================================================

/// Space between the title's name, its kind and its occurrence.
const TITLE_GAP: f32 = 8.0;

/// The inspected object, named on one row: its name, then what it is, then —
/// below the design root — the occurrence it sits in. A line too long for the
/// panel is cut at its end, so the name always stays readable.
fn object_title(ui: &mut Ui, name: &str, kind: Option<&str>, occurrence: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    let inset = panel_inset(ui);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            ui.available_width(),
            crate::workbench::design_system::property_row_height(ui),
        ),
        egui::Sense::hover(),
    );
    let spoken = [Some(name), kind, occurrence]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ");
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &spoken)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Heading);
        node.set_label(spoken.as_str());
        node.set_level(3);
    });
    if !ui.is_rect_visible(rect) {
        return;
    }
    let mut job = egui::text::LayoutJob::default();
    job.append(
        name,
        0.0,
        egui::TextFormat::simple(
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            t.color.text,
        ),
    );
    if let Some(kind) = kind {
        job.append(
            kind,
            TITLE_GAP,
            egui::TextFormat::simple(
                theme::sans(tokens::FS_1, FontWeight::Regular),
                t.color.text_dim,
            ),
        );
    }
    if let Some(occurrence) = occurrence {
        job.append(
            occurrence,
            TITLE_GAP,
            egui::TextFormat::simple(
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            ),
        );
    }
    job.wrap = egui::text::TextWrapping::truncate_at_width((rect.width() - 2.0 * inset).max(1.0));
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    ui.painter().galley(
        egui::pos2(rect.left() + inset, rect.center().y - galley.size().y * 0.5),
        galley,
        t.color.text,
    );
}

// =============================================================================
// Shared section pieces
// =============================================================================

/// The sheet one inspected object lives on, stated where the object's own
/// identity is.
///
/// It appears only above one sheet: in a single-sheet cell view the sheet and
/// the drawing are the same object, so a row repeating the document's name
/// says nothing. The catalog answers for both the page number and the name,
/// which is what keeps this row, the strip's chip and the status bar's page
/// segment from disagreeing.
fn sheet_row(ui: &mut Ui, state: &AppState, object_id: u64) {
    let Some(catalog) = sheet_visibility::multi_sheet_catalog(state) else {
        return;
    };
    let key = state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let Some(id) = state
        .model
        .workspace
        .content
        .design_management
        .sheet_for_object_or_active(&key, object_id)
    else {
        return;
    };
    let Some(((page, _), name)) = catalog
        .page_number_and_count(id)
        .zip(catalog.find(id).map(|sheet| sheet.name()))
    else {
        return;
    };
    property_row(ui, "Sheet", &format!("Sheet {page} \u{00b7} {name}"));
}

// =============================================================================
// Inline instance editing lives in `design/inline_instance.rs`.
mod inline_instance;
use inline_instance::*;

// The sheet inspector, shown with nothing selected, is the sheets feature's;
// the shell's feature tests reach it here.
#[cfg(test)]
pub(crate) use rspice_app_sheets::workbench::docks::inspector::design::sheet;

// =============================================================================
// Multi-selection inspector
// =============================================================================

fn multi_panel(ui: &mut Ui, app: &mut RSpiceApp) {
    // The docks share one reading position, so a selection reaching across a
    // sheet boundary — which a navigator net row can produce — is reported
    // here exactly as the navigator lists it.
    let scope = sheet_visibility::sheet_scope(ui.ctx());
    let selected: Vec<(u64, String, String)> = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| {
            app.state
                .model
                .schematic
                .session
                .editor
                .selection
                .has_component(component.id)
        })
        .filter(|component| sheet_visibility::object_is_in_scope(&app.state, scope, component.id))
        .map(|component| {
            (
                component.id,
                component.name.clone(),
                component.value.clone(),
            )
        })
        .collect();
    let total = match scope {
        SheetScope::AllSheets => app.state.model.schematic.session.editor.selection.count(),
        SheetScope::ActiveSheet => sheet_visibility::selection_filtered_to_active_sheet(
            &app.state,
            &app.state.model.schematic.session.editor.selection,
        )
        .count(),
    };
    // The instances are named only when the set holds something else too: a
    // set of instances alone is counted once, by the title itself.
    let instances = (!selected.is_empty() && selected.len() < total).then(|| {
        format!(
            "{} instance{}",
            selected.len(),
            if selected.len() == 1 { "" } else { "s" }
        )
    });
    object_title(ui, &format!("{total} selected"), instances.as_deref(), None);

    // The set's instances, one row each: a click picks that one out of the
    // set. The toolbar, the context menu and the keys act on the set itself.
    section_block_gap(ui);
    if selected.is_empty() {
        muted_inspector_copy(ui, "No instances selected.");
        return;
    }
    let mut reduce: Option<u64> = None;
    for (id, name, value) in &selected {
        let row = TreeRow::new(name).mono().indent(1).meta(value).show(ui);
        if row.response.clicked() {
            reduce = Some(*id);
        }
        row.response
            .on_hover_text(format!("Reduce the selection to {name}"));
    }
    if let Some(id) = reduce {
        select_component(app, id);
    }
}

// =============================================================================
// Documentation objects
// =============================================================================

fn note_panel(ui: &mut Ui, app: &mut RSpiceApp, id: u64) {
    let Some(note) = app
        .state
        .model
        .schematic
        .document()
        .design_notes
        .iter()
        .find(|note| note.id == id)
        .cloned()
    else {
        return;
    };
    object_title(ui, "Note", Some(note.kind.label()), None);
    section_block_gap(ui);
    property_row(ui, "Text", &note.text);
    property_row(ui, "Layer", note.layer.label());
    sheet_row(ui, &app.state, id);
    if let Some(review) = note.review.as_ref() {
        property_row(ui, "Review state", review.state.keyword());
    }
}

fn shape_panel(ui: &mut Ui, app: &mut RSpiceApp, id: u64) {
    let Some(shape) = app
        .state
        .model
        .schematic
        .document()
        .documentation_shapes
        .iter()
        .find(|shape| shape.id == id)
        .cloned()
    else {
        return;
    };
    object_title(ui, "Shape", Some(shape.kind().label()), None);
    section_block_gap(ui);
    property_row(ui, "Layer", shape.layer.label());
    sheet_row(ui, &app.state, id);
}

fn probe_panel(ui: &mut Ui, app: &mut RSpiceApp, id: u64) {
    let Some(probe) = app
        .state
        .model
        .schematic
        .document()
        .probes
        .iter()
        .find(|probe| probe.id == id)
        .cloned()
    else {
        return;
    };
    let bound = probe.source_expression.as_deref();
    object_title(ui, &probe.reference, Some("Probe"), None);
    section_block_gap(ui);
    // A probe that names no signal saves nothing; that is the one thing about
    // it to act on.
    property_row_problem(
        ui,
        "Signal",
        bound.unwrap_or("not bound"),
        bound.is_none().then_some(Problem::Warning),
    );
    sheet_row(ui, &app.state, id);

    let writable = !app.state.schematic_edit_read_only();
    let mut enabled = probe.enabled;
    section_block_gap(ui);
    let enabled_response = ui.add_enabled(
        writable,
        egui::Checkbox::new(&mut enabled, "Save this signal"),
    );
    if !writable {
        enabled_response.on_disabled_hover_text("The active schematic is read-only.");
    }
    let mut plot_on_materialization = probe.plot_on_materialization;
    let plot_response = ui.add_enabled(
        writable && enabled,
        egui::Checkbox::new(&mut plot_on_materialization, "Plot after the next run"),
    );
    if !writable {
        plot_response.on_disabled_hover_text("The active schematic is read-only.");
    } else if !enabled {
        plot_response.on_disabled_hover_text("Save the signal before choosing to plot it.");
    }
    if enabled != probe.enabled || plot_on_materialization != probe.plot_on_materialization {
        let changed =
            app.state
                .model
                .schematic
                .edit_probe_intent(id, enabled, plot_on_materialization);
        if changed {
            app.state.sync_active_schematic_to_workspace();
            app.invalidate_simulation_preflight();
        }
    }

    let Some(expression) = bound else {
        return;
    };
    action_stack(ui, |ui| {
        if Button::new("Show in results")
            .icon(Icon::Results)
            .show(ui)
            .clicked()
        {
            if writable {
                let changed = crate::schematic::view::ensure_probe_visible_with_feedback(
                    &mut app.state,
                    expression,
                    expression,
                );
                if changed {
                    app.invalidate_simulation_preflight();
                }
            } else {
                crate::schematic::view::ensure_retained_probe_visible_with_feedback(
                    &mut app.state,
                    expression,
                    expression,
                );
            }
        }
    });
}

impl AppState {
    /// Every occurrence the open document's master is instantiated at, in path
    /// order.
    ///
    /// This is the one derivation of how far an edit here reaches. The canvas
    /// strip in `workbench::surfaces::design` states it, and no surface counts
    /// instances for itself.
    ///
    /// The design projection is the authority. It is the same value a run is
    /// prepared from, memoized on the content it derives from, so the number a
    /// surface states is the number the deck emits rather than the result of a
    /// second hierarchy walk. A projection that does not resolve yields nothing
    /// rather than a guess: an edit whose scope cannot be established is not
    /// announced as landing anywhere in particular.
    ///
    /// The document's occurrence ends at its own reference by construction, so
    /// the master asked about is `active_view`.
    pub(crate) fn master_occurrence_paths(&self) -> Vec<String> {
        let Ok(projection) = self.model.workspace.design_projection(
            &self.model.library_manager,
            &self.model.workspace.content.active_view,
            &self.model.schematic,
        ) else {
            return Vec::new();
        };
        let master = self.model.workspace.content.active_view.key();
        projection
            .plan()
            .bindings()
            .filter(|binding| {
                binding
                    .resolved_reference()
                    .key()
                    .eq_ignore_ascii_case(&master)
            })
            .map(|binding| binding.instance_path().to_string())
            .collect()
    }
}

#[cfg(test)]
pub(crate) mod tests;
