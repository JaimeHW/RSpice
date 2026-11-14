//! Selection-dispatched design inspector.
//!
//! The right panel inspects exactly one object class at a time, chosen from
//! the schematic selection: an instance, a conductor, a documentation
//! object, the whole selected set, or — when nothing is selected — the sheet
//! itself. Every row is backed by live document state; no panel narrates a
//! fact the open design cannot supply.

mod component;
mod model_evidence;
/// Visible to the workbench so the library browser's own gate can render this
/// pane beside it: the two panels state the same counts about the same design,
/// and a review that only ever opened one of them would not see them disagree.
pub(crate) mod stimulus;
pub(in crate::workbench) mod symbol;

pub(crate) use component::bind_component_model_from_catalog;

use component::*;
use model_evidence::{ComponentModelEvidence, component_model_evidence};

use std::collections::{HashMap, HashSet};

use egui::Ui;

use crate::schematic::view::sheet_visibility::{self, SheetScope};
use crate::state::{
    AnalysisResultPayload, CellViewRef, Component, ComponentType, DisplayMode, PropertyDefinition,
    PropertyType, PropertyValue,
};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, TreeRow, TreeRowResult};
use crate::workbench::app::actions::workspace::{
    commit_inline_component_edit, commit_pending_inspector_edit, inline_edit_authority,
    sync_active_schematic,
};
use crate::workbench::app_state::Model;
use crate::workbench::design::DesignFeature;
use crate::workbench::design_system::{
    Problem, PropertyRowCaption, WorkbenchIcon, problem_line, property_row, property_row_captioned,
    property_row_combo, property_row_flagged, property_row_input_action,
    property_row_input_captioned, property_row_input_with_hint, property_row_problem,
};
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::feature::Cx;
use crate::workbench::framework::requests::{Drain, Request, RevealModel};
use crate::workbench::slots::DesignState;
use crate::workbench::state::{InlineEditField, InlineEditSession, WorkbenchSession, Workspace};
use rspice_design::connectivity::extract_with_hierarchy as extract;
use rspice_design::connectivity::summary::{DesignNet, NetClass};
use rspice_design::drc::{DrcLocation, DrcViolation};
use rspice_design::hierarchy::HierarchySource;

use crate::workbench::docks::inspector::shared::{
    action_stack, checks_current, count_label, current_findings, findings_problem,
};
use crate::workbench::docks::inspector::soa::{ComponentSoaDisplay, component_soa_display};
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
fn selected_wire_ids(model: &Model) -> Vec<u64> {
    let selection = &model.schematic.session.editor.selection;
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
fn conductors_only(model: &Model) -> bool {
    let selection = &model.schematic.session.editor.selection;
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

pub(super) fn subject(model: &Model, nets: &[DesignNet]) -> DesignSubject {
    let selection = &model.schematic.session.editor.selection;
    if let Some(name) = navigator_selected_net_name(model, nets) {
        return DesignSubject::Net(name);
    }
    if let Some(id) = selection.single_component() {
        if let Some(port) = model
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
    if conductors_only(model)
        && let Some(name) = selected_net_name(model, nets)
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
fn navigator_selected_net_name(model: &Model, nets: &[DesignNet]) -> Option<String> {
    let authority = model
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
    if model
        .schematic
        .session
        .editor
        .net_highlight
        .highlighted_wires
        != wire_ids.iter().copied().collect()
    {
        return None;
    }

    let selection = &model.schematic.session.editor.selection;
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
fn selected_net_name(model: &Model, nets: &[DesignNet]) -> Option<String> {
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
    for label in &model.schematic.document().net_labels {
        if model
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
    for wire_id in selected_wire_ids(model) {
        let Some(net) = nets.iter().find(|net| net.wire_ids.contains(&wire_id)) else {
            continue;
        };
        if !accept(&net.name) {
            return None;
        }
    }
    if !model
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
        let connectivity = extract(&model.schematic, None);
        for junction in &model.schematic.session.editor.selection.junctions {
            let Some(extracted) = connectivity.net_at(junction.pos).or_else(|| {
                model
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

/// Settle the inline edit before the shell lends the inspector its context,
/// so the dispatcher's answers it reads are taken for the selection it then
/// shows. An edit whose instance changed under it ends silently when nothing
/// typed would be lost; one the selection has moved off is committed, and
/// when that fails its instance is selected again and held in the inspector
/// so the edit can be corrected. A symbol's inspector holds no inline edit.
pub(in crate::workbench) fn prepare(cx: &mut Cx<'_, DesignFeature>) {
    cx.state.inspector_holds_failed_edit = None;
    if cx.model.workspace.content.active_view_type() == crate::state::ViewType::Symbol {
        return;
    }
    let Some(session) = cx.wb.inline_edit.session() else {
        return;
    };
    if inline_edit_is_stale(cx.model, session) {
        if session.candidate.as_ref() == Some(&session.expected) {
            cx.wb.inline_edit.end();
        }
        return;
    }
    let editing = session.expected.id;
    let sheet = sheet_connectivity(cx.model);
    if subject(cx.model, &sheet.nets) == DesignSubject::Component(editing) {
        return;
    }
    if commit_inline_component_edit(cx.model, cx.wb).is_err() {
        cx.model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(editing);
        cx.state.inspector_holds_failed_edit = Some(editing);
    }
}

/// Whether the design changed under the inline edit `session`: the edit
/// authority moved, the schematic became read-only, or the instance it
/// edits is no longer the one the document holds.
fn inline_edit_is_stale(model: &Model, session: &InlineEditSession) -> bool {
    session.authority != inline_edit_authority(model)
        || model.schematic_edit_read_only()
        || !model
            .schematic
            .document()
            .components
            .contains(&session.expected)
}

/// The request that commits what the sheet's section holds typed, raised
/// by a pass of the design inspector that shows something other than the
/// sheet. The shell answers it once the inspector has drawn.
#[derive(Clone, Debug)]
pub(crate) struct CommitSheetSection(pub(crate) egui::Context);

impl Request for CommitSheetSection {
    const DRAIN: Drain = Drain::AfterContribution;
}

pub(in crate::workbench) fn show(ui: &mut Ui, cx: &mut Cx<'_, DesignFeature>) {
    cx.state.inspector_yields_to_sheet = false;
    // prepare() ended a stale edit with nothing typed; one with a draft
    // says so and offers to discard it.
    if let Some(session) = cx.wb.inline_edit.session()
        && inline_edit_is_stale(cx.model, session)
        && session.candidate.as_ref() != Some(&session.expected)
    {
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
            cx.wb.inline_edit.end();
        }
        return;
    }
    let sheet = sheet_connectivity(cx.model);
    if let Some(reason) = sheet.unresolved.as_deref() {
        unresolved_projection_note(ui, reason);
    }
    let inspected = match cx.state.inspector_holds_failed_edit {
        Some(editing) => DesignSubject::Component(editing),
        None => subject(cx.model, &sheet.nets),
    };

    if inspected != DesignSubject::Sheet {
        cx.wb.outbox.push(CommitSheetSection(ui.ctx().clone()));
    }

    match inspected {
        DesignSubject::Component(id) => component_panel(ui, cx, id, &sheet),
        DesignSubject::Net(name) => net_panel(ui, cx, &name, &sheet.nets),
        DesignSubject::Note(id) => note_panel(ui, cx, id),
        DesignSubject::Shape(id) => shape_panel(ui, cx, id),
        DesignSubject::Probe(id) => probe_panel(ui, cx, id),
        DesignSubject::Multi => multi_panel(ui, cx),
        DesignSubject::Sheet => cx.state.inspector_yields_to_sheet = true,
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
fn sheet_connectivity(model: &Model) -> SheetConnectivity {
    let projection = match model.workspace.design_projection(
        &model.library_manager,
        &model.workspace.content.active_view,
        &model.schematic,
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
        HierarchySource::from_design_projection(model.library_manager.catalog(), &projection);
    let nets = rspice_design::connectivity::summary::projection_nets(
        model.library_manager.catalog(),
        &projection,
        &model.workspace.content.active_view.key(),
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

    let terminals = model
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
fn sheet_row(ui: &mut Ui, model: &Model, object_id: u64) {
    let Some(catalog) = sheet_visibility::multi_sheet_catalog(model) else {
        return;
    };
    let key = model.workspace.content.active_schematic_reference().key();
    let Some(id) = model
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

fn multi_panel(ui: &mut Ui, cx: &mut Cx<'_, DesignFeature>) {
    // The docks share one reading position, so a selection reaching across a
    // sheet boundary — which a navigator net row can produce — is reported
    // here exactly as the navigator lists it.
    let scope = sheet_visibility::sheet_scope(ui.ctx());
    let selected: Vec<(u64, String, String)> = cx
        .model
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| {
            cx.model
                .schematic
                .session
                .editor
                .selection
                .has_component(component.id)
        })
        .filter(|component| sheet_visibility::object_is_in_scope(cx.model, scope, component.id))
        .map(|component| {
            (
                component.id,
                component.name.clone(),
                component.value.clone(),
            )
        })
        .collect();
    let total = match scope {
        SheetScope::AllSheets => cx.model.schematic.session.editor.selection.count(),
        SheetScope::ActiveSheet => cx
            .model
            .selection_on_active_sheet(&cx.model.schematic.session.editor.selection)
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
        select_component(cx, id);
    }
}

// =============================================================================
// Documentation objects
// =============================================================================

fn note_panel(ui: &mut Ui, cx: &mut Cx<'_, DesignFeature>, id: u64) {
    let Some(note) = cx
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
    sheet_row(ui, cx.model, id);
    if let Some(review) = note.review.as_ref() {
        property_row(ui, "Review state", review.state.keyword());
    }
}

fn shape_panel(ui: &mut Ui, cx: &mut Cx<'_, DesignFeature>, id: u64) {
    let Some(shape) = cx
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
    sheet_row(ui, cx.model, id);
}

fn probe_panel(ui: &mut Ui, cx: &mut Cx<'_, DesignFeature>, id: u64) {
    let Some(probe) = cx
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
    sheet_row(ui, cx.model, id);

    let writable = !cx.model.schematic_edit_read_only();
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
        let changed = cx
            .model
            .schematic
            .edit_probe_intent(id, enabled, plot_on_materialization);
        if changed {
            sync_active_schematic(cx.model, cx.wb);
            rspice_app_runs::workbench::preflight::invalidate(
                &mut cx.runs.preflight,
                &mut *cx.handles.run_controller,
            );
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
                    &mut crate::schematic::view::canvas_parts_in(cx),
                    expression,
                    expression,
                );
                if changed {
                    rspice_app_runs::workbench::preflight::invalidate(
                        &mut cx.runs.preflight,
                        &mut *cx.handles.run_controller,
                    );
                }
            } else {
                crate::schematic::view::ensure_retained_probe_visible_with_feedback(
                    &mut crate::schematic::view::canvas_parts_in(cx),
                    expression,
                    expression,
                );
            }
        }
    });
}

#[cfg(test)]
pub(crate) mod tests;
