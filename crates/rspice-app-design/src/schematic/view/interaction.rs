//! Canvas pointer and keyboard interaction.
//!
//! Resolves what is under the pointer, then dispatches to the operation the
//! armed tool implies. Hit resolution is ordered — a terminal beats a wire
//! vertex, which beats the wire body — so a click near a junction does what
//! the designer meant.

use egui::{Response, Ui};

use crate::state::{
    ComponentType, OccurrenceProbeSpelling, Point, SavedOutput, SavedOutputCompatibility,
    SavedOutputKind, SavedOutputOrigin, SavedOutputPolicy, SavedOutputPrecision,
    SavedOutputStreaming, SchematicNetHighlighting, SchematicProbe, Tool, ViewType,
};
use crate::workbench::app_state::Model;
use rspice_design::connectivity::summary::{DesignNet, projection_nets};
use rspice_design::schematic::label_position::InstanceText;
use rspice_schematic_editor::view::documentation_shape_input::{
    self, ShapeInputAction, ShapeInputRequest, ShapeInputTransition, ShapeInputView,
};

use super::SchematicSymbolContext;
use super::array_interaction::handle_armed_array_selection;
use super::bus_interaction::{BusTapCandidateError, resolve_bus_tap_candidate_on_active_sheet};
use super::move_interaction::handle_armed_move_selection;
use super::navigation::primary_pan_gesture_active;
use super::parts::CanvasParts;
use super::requests::{DesignSurfaceRequest, PlotProbedTrace, ProbePlot};
use super::selection_drag::handle_select_dragging;
use super::sheet_visibility::{finish_wire_on_active_sheet, with_active_wire_topology_in};
use super::snap_resolution::resolve_grid_pointer;
use super::stretch_interaction::handle_armed_stretch_selection;
use super::viewport::Viewport;
use rspice_schematic_editor::view::scene::normalized_probe_expression;
use rspice_schematic_editor::view::tool_input::{ToolInput, ToolPointerAction, ToolPointerView};

mod pin_placement;
mod pointer_target;
use pin_placement::place_pending_port;
use pointer_target::pointer_target_with_filter;
pub(super) use pointer_target::{PointerHit, PointerTarget, pointer_target};

pub fn handle_tool_interactions(
    ui: &Ui,
    response: &Response,
    state: &mut CanvasParts<'_>,
    viewport: &Viewport,
    symbol_context: &SchematicSymbolContext,
) {
    if state.reconcile_schematic_drag(ui.ctx(), false) {
        return;
    }
    if state.reconcile_conductor_route(ui.ctx()) {
        return;
    }
    state.model.retain_selection_on_active_sheet();
    if primary_pan_gesture_active(ui, response) {
        return;
    }
    // The armed zoom box owns the next primary drag, whatever the tool.
    if state.model.schematic.session.editor.zoom_box.armed {
        super::zoom_box::handle_zoom_box(ui, response, state, viewport);
        return;
    }
    let grid_size = state.model.schematic.document().grid_size;
    let current_tool = state.model.schematic.session.editor.tool;
    // A floating copy owns the canvas until it lands; a tool chosen since
    // it began is the next command, and the copy is let go.
    if state.design.floating_paste.is_some() && current_tool != Tool::Select {
        crate::workbench::app::cancel_floating_paste_in(
            &mut state.design.floating_paste,
            state.workbench,
        );
    }
    if super::floating_paste::handle_floating_paste(ui, response, state, viewport) {
        return;
    }
    if state.design.move_selection.armed && current_tool != Tool::MoveSelection {
        state.design.move_selection.close();
        // The pointer event that exposed an inconsistent tool/draft pair belongs
        // to neither workflow. Do not hand it to Select after cancelling the
        // transactional move or the same gesture could mutate geometry through
        // the legacy direct-drag path.
        return;
    } else if current_tool == Tool::MoveSelection && !state.design.move_selection.armed {
        state.model.schematic.cancel_tool();
        return;
    } else if state.design.stretch_selection.armed && current_tool != Tool::StretchSelection {
        state.design.stretch_selection.close();
        return;
    } else if current_tool == Tool::StretchSelection && !state.design.stretch_selection.armed {
        state.model.schematic.cancel_tool();
        return;
    } else if state.design.array_selection.armed && current_tool != Tool::ArraySelection {
        state.design.array_selection.close();
        return;
    } else if current_tool == Tool::ArraySelection && !state.design.array_selection.armed {
        state.model.schematic.cancel_tool();
        return;
    }
    let input = ToolInput::read(
        ui,
        response,
        current_tool,
        state.model.schematic.session.editor.wire_drawing.active
            || state.model.schematic.session.editor.bus_drawing.active,
    );
    if input.finish_route {
        finish_active_route(ui, state);
        return;
    }
    if input.flip_route_corner {
        flip_route_corner(state);
        return;
    }
    if input.remove_route_point {
        if current_tool == Tool::Bus {
            state
                .model
                .schematic
                .session
                .editor
                .bus_drawing
                .remove_last_point();
        } else {
            state.model.schematic.retract_wire();
        }
        return;
    }

    if current_tool == Tool::DocumentationShape
        && let Some(transition) = documentation_shape_input::pointer_motion(
            ui,
            response,
            viewport,
            shape_input_view(state),
        )
    {
        let request = capture_shape_input(state, transition);
        apply_shape_input(state, request);
    }
    if current_tool == Tool::DocumentationShape
        && let Some(transition) =
            documentation_shape_input::pointer_drag(ui, response, viewport, shape_input_view(state))
    {
        let request = capture_shape_input(state, transition);
        apply_shape_input(state, request);
    }

    if matches!(current_tool, Tool::Select) {
        handle_select_dragging(ui, response, state, viewport, symbol_context);
    } else if current_tool == Tool::MoveSelection {
        handle_armed_move_selection(ui, response, state, viewport, symbol_context);
    } else if current_tool == Tool::StretchSelection {
        handle_armed_stretch_selection(ui, response, state, viewport, symbol_context);
    } else if current_tool == Tool::ArraySelection {
        handle_armed_array_selection(ui, response, state, viewport, grid_size, symbol_context);
    }

    if let Some(pos) = input.shape_double_click {
        let position = resolve_grid_pointer(state.model, viewport, pos).snapped_position;
        handle_documentation_shape_click(state, position, true);
    }

    if let Some(pos) = input.primary_click
        && let Some(action) =
            tool_pointer_view(state).action(current_tool, viewport, symbol_context, pos)
    {
        match action {
            ToolPointerAction::ReadOnly => {
                state.deny_read_only_edit();
            }
            ToolPointerAction::Place(component_type, position) => {
                place_component(state, component_type, position);
            }
            ToolPointerAction::Wire {
                position,
                ends_route,
            } => match position {
                Some(wire_pos) if state.model.schematic.session.editor.wire_drawing.active => {
                    state.model.schematic.extend_wire(wire_pos);
                    if ends_route {
                        let _ = finish_wire_on_active_sheet(state.model);
                    }
                }
                Some(wire_pos) => state.start_canvas_wire(ui.ctx(), wire_pos),
                None => report_unrepresentable_conductor_attachment(state),
            },
            ToolPointerAction::Bus {
                position: bus_pos,
                joins,
            } => {
                if state.model.schematic.session.editor.bus_drawing.active {
                    state.model.schematic.extend_bus(bus_pos);
                    if let Some(bus_id) = joins {
                        finish_bus_on(state, bus_id);
                    }
                } else {
                    let declaration = state
                        .model
                        .schematic
                        .session
                        .editor
                        .bus_drawing
                        .declaration
                        .clone();
                    if let Err(error) = state.start_canvas_bus(ui.ctx(), bus_pos, declaration) {
                        state.prompt(format!("Bus not started: {error}"));
                    }
                }
            }
            ToolPointerAction::BusTap { position, radius } => {
                handle_bus_tap_click(state, position, radius);
            }
            ToolPointerAction::Junction(position) => handle_junction_click(state, position),
            ToolPointerAction::DesignNote(position) => place_pending_design_note(state, position),
            ToolPointerAction::DocumentationShape(position) => {
                handle_documentation_shape_click(state, position, false);
            }
            ToolPointerAction::Select(query) => {
                handle_select_click(
                    ui,
                    state,
                    query.hit,
                    query.radius,
                    symbol_context,
                    viewport,
                    query.position,
                );
            }
            ToolPointerAction::Probe(position) => {
                handle_probe_click(state, position, symbol_context);
            }
            // The armed tool determines whether the naming transaction makes
            // a local label or an off-sheet connector at this shared anchor.
            ToolPointerAction::NetLabel(anchor) => {
                let dialog_modal_open = state.dialog_modal_open();
                if label_anchor_on_unmarked_crossing(state, anchor) {
                    refuse_label_on_unmarked_crossing(ui, state, anchor);
                } else if !crate::workbench::app::place_sequenced_net_label_in(
                    state.model,
                    state.workbench,
                    dialog_modal_open,
                    anchor,
                ) {
                    crate::workbench::app::open_net_label_placement_in(
                        &mut state.design.net_label_placement,
                        state.model,
                        state.workbench,
                        dialog_modal_open,
                        anchor,
                    );
                }
            }
        }
    }

    if let Some(pos) = input.select_double_click {
        let query = tool_pointer_view(state).selection_query(viewport, pos);
        let hit = query.hit;
        let hit_radius = query.radius;
        let target = pointer_target(
            state,
            hit,
            hit_radius,
            symbol_context,
            ui.ctx(),
            viewport,
            pos,
        );
        let canvas_is_empty_at_pointer = target.is_none()
            && pointer_target_with_filter(
                state,
                hit,
                hit_radius,
                symbol_context,
                ui.ctx(),
                viewport,
                pos,
                crate::state::SchematicSelectionFilter::default(),
            )
            .is_none();
        // The title block is the sheet's form: a double-click on it puts the
        // cursor in the sheet's Title, as KiCad's opens its page settings.
        let on_title_block = canvas_is_empty_at_pointer
            && super::drawing_sheet::editable_title_block_under(
                &state.facts(),
                &state.model.active_drawing_sheet(),
                viewport,
                pos,
            )
            .is_some();
        // A double-click on a part's name or value edits that text where it
        // stands.
        let text = match target {
            Some(PointerTarget::Component(id)) => super::text_entry::editable_text_at(
                state,
                symbol_context,
                viewport,
                ui.ctx(),
                pos,
                id,
            )
            .map(|text| (id, text)),
            _ => None,
        };
        let action = if on_title_block {
            SelectDoubleClickAction::EditSheetTitle
        } else if let Some((id, text)) = text {
            SelectDoubleClickAction::EditText(id, text)
        } else {
            select_double_click_action(state, target, canvas_is_empty_at_pointer)
        };
        match action {
            SelectDoubleClickAction::Descend(id) => {
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_component(id);
                state.open_selected_instance_master();
            }
            SelectDoubleClickAction::OpenVerilogA(id) => {
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .selection
                    .select_only_component(id);
                let _ = state.open_veriloga_source_for_component(id);
            }
            SelectDoubleClickAction::ActivateRequirement(id) => {
                if activate_requirement_link(state, id, ui.ctx()) {
                    state
                        .model
                        .schematic
                        .session
                        .editor
                        .selection
                        .select_only_design_note(id);
                } else {
                    open_object_properties(
                        state,
                        hit,
                        hit_radius,
                        symbol_context,
                        ui.ctx(),
                        viewport,
                        pos,
                    );
                }
            }
            SelectDoubleClickAction::OpenProperties => {
                open_object_properties(
                    state,
                    hit,
                    hit_radius,
                    symbol_context,
                    ui.ctx(),
                    viewport,
                    pos,
                );
            }
            SelectDoubleClickAction::ShowNet(target) => show_conductor_net(state, target),
            SelectDoubleClickAction::EditText(id, text) => {
                let selection = &mut state.model.schematic.session.editor.selection;
                selection.select_only_component(id);
                if let Some(part) = state
                    .model
                    .schematic
                    .document()
                    .components
                    .iter()
                    .find(|part| part.id == id)
                    .cloned()
                {
                    super::text_entry::open(state, &part, text, pos);
                }
            }
            SelectDoubleClickAction::EditSheetTitle => super::context_menu::show_sheet_properties(
                state,
                Some(crate::state::DrawingSheetTitleFieldId::SheetTitle),
            ),
            SelectDoubleClickAction::Ascend => state.ascend_workspace_level(),
            SelectDoubleClickAction::None => {}
        }
    }

    if state.model.schematic.session.editor.tool == Tool::DocumentationShape && response.has_focus()
    {
        handle_documentation_shape_keyboard(ui, response, state, viewport, grid_size);
    }
}

fn tool_pointer_view<'a>(state: &'a CanvasParts<'_>) -> ToolPointerView<'a> {
    ToolPointerView {
        design: state.design_view(),
        snap_engine: &state.model.schematic.session.editor.snap_engine,
        can_edit: !state.schematic_edit_read_only(),
    }
}

fn finish_active_route(ui: &Ui, state: &mut CanvasParts<'_>) -> bool {
    if state.reconcile_conductor_route(ui.ctx()) {
        return false;
    }
    if state.model.schematic.session.editor.wire_drawing.active {
        return finish_wire_on_active_sheet(state.model).is_some();
    }
    if state.model.schematic.session.editor.bus_drawing.active {
        return match state.model.schematic.finish_bus() {
            Ok(bus) => bus.is_some(),
            Err(error) => {
                state.prompt(format!("Bus not committed: {error}"));
                false
            }
        };
    }
    false
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SelectDoubleClickAction {
    Descend(u64),
    OpenVerilogA(u64),
    ActivateRequirement(u64),
    OpenProperties,
    /// Show the net of a wire or junction in the inspector.
    ShowNet(PointerTarget),
    /// Edit a part's name or value where it stands.
    EditText(u64, InstanceText),
    EditSheetTitle,
    Ascend,
    None,
}

fn select_double_click_action(
    state: &CanvasParts<'_>,
    target: Option<PointerTarget>,
    canvas_is_empty_at_pointer: bool,
) -> SelectDoubleClickAction {
    match target {
        Some(PointerTarget::Component(id))
            if state.hierarchy_master_for_component(id).is_some() =>
        {
            SelectDoubleClickAction::Descend(id)
        }
        Some(PointerTarget::Component(id)) if state.veriloga_source_for_component(id).is_some() => {
            SelectDoubleClickAction::OpenVerilogA(id)
        }
        Some(PointerTarget::DesignNote(id)) => SelectDoubleClickAction::ActivateRequirement(id),
        Some(target @ (PointerTarget::Wire(_) | PointerTarget::Junction(_))) => {
            SelectDoubleClickAction::ShowNet(target)
        }
        Some(_) => SelectDoubleClickAction::OpenProperties,
        // Ascend belongs to the document under the pointer, so the depth read
        // is the open document's own occurrence rather than the session-global
        // breadcrumb, which describes whichever document was activated last.
        None if canvas_is_empty_at_pointer
            && state.model.workspace.content.occurrence_depth() > 1 =>
        {
            SelectDoubleClickAction::Ascend
        }
        None => SelectDoubleClickAction::None,
    }
}

pub fn activate_requirement_link(
    state: &mut CanvasParts<'_>,
    note_id: u64,
    ctx: &egui::Context,
) -> bool {
    let target = state
        .model
        .schematic
        .document()
        .design_notes
        .iter()
        .find(|note| note.id == note_id)
        .and_then(|note| note.requirement_target());
    match target {
        Some(crate::state::RequirementTarget::ExternalUri(uri)) => {
            let uri = uri.to_owned();
            ctx.open_url(egui::OpenUrl::new_tab(&uri));
            state.prompt(format!("Opened requirement link {uri}."));
            true
        }
        Some(crate::state::RequirementTarget::ProjectSpecification(reference)) => {
            let reference = reference.to_owned();
            // Verify turns to its Specifications view once the surface has
            // drawn. The workspace itself opens now, so the rest of the
            // frame reads the route it will show.
            state
                .workbench
                .outbox
                .push(DesignSurfaceRequest::OpenSpecifications);
            state
                .workbench
                .activate(crate::workbench::state::Workspace::Verify);
            state.prompt(format!(
                "Opened project specifications for requirement {reference}."
            ));
            true
        }
        None => false,
    }
}

fn handle_bus_tap_click(state: &mut CanvasParts<'_>, requested: Point, hit_radius: i32) {
    if state.schematic_edit_read_only() {
        state.deny_read_only_edit();
        state.model.schematic.cancel_tool();
        return;
    }
    let Some(placement) = state
        .model
        .schematic
        .session
        .editor
        .pending_bus_tap
        .as_ref()
    else {
        report_bus_candidate_error(state, BusTapCandidateError::MissingConfiguration);
        return;
    };
    if state.model.schematic.session.editor.tool != Tool::BusTap
        || !placement
            .authority
            .matches(&super::requests::editor_request_source(state))
    {
        state.prompt("The active schematic changed. Reopen Place bus tap.");
        state.model.schematic.cancel_tool();
        return;
    }
    let candidate = match resolve_bus_tap_candidate_on_active_sheet(state, requested, hit_radius) {
        Ok(candidate) => candidate,
        Err(error) => {
            report_bus_candidate_error(state, error);
            return;
        }
    };
    let pending = placement.configuration.clone();
    let configured = crate::state::PendingBusTap {
        orientation: candidate.orientation,
        ..pending.clone()
    };
    match state.model.schematic.place_configured_bus_tap(
        candidate.bus_id,
        candidate.bus_point,
        candidate.connection_point,
        &configured,
    ) {
        Ok(_) => {
            // One bit taps the next on the following click, so a bus is
            // fanned out one click per bit; a range stays armed as it is.
            let following = pending.following();
            let message = match &following {
                Some(next) => format!(
                    "Placed tap {}. The next click taps {}.",
                    pending.slice, next.slice
                ),
                None if pending.slice.is_scalar() => format!(
                    "Placed tap {}, the last bit of {}.",
                    pending.slice, pending.bus_declaration
                ),
                None => format!("Placed tap {}.", pending.slice),
            };
            match following {
                Some(next) => {
                    if let Some(placement) = state
                        .model
                        .schematic
                        .session
                        .editor
                        .pending_bus_tap
                        .as_mut()
                    {
                        placement.configuration = next;
                    }
                }
                None if pending.slice.is_scalar() => state.model.schematic.cancel_tool(),
                None => {}
            }
            state.prompt(message);
        }
        Err(error) => state.prompt(format!("Bus tap not placed: {error}")),
    }
}

/// End a live bus route on the bus `bus_id` it reached, as a wire route ends
/// on the conductor it reaches. A route drawn without a name takes the name
/// of the bus it joins, since buses drawn touching are one connection.
fn finish_bus_on(state: &mut CanvasParts<'_>, bus_id: u64) {
    if state
        .model
        .schematic
        .session
        .editor
        .bus_drawing
        .declaration
        .is_none()
    {
        let joined = state
            .model
            .schematic
            .document()
            .buses
            .iter()
            .find(|bus| bus.id == bus_id)
            .and_then(|bus| bus.declaration.clone());
        state.model.schematic.session.editor.bus_drawing.declaration = joined;
    }
    if let Err(error) = state.model.schematic.finish_bus() {
        state.prompt(format!("Bus not committed: {error}"));
    }
}

fn report_bus_candidate_error(state: &mut CanvasParts<'_>, error: BusTapCandidateError) {
    state.prompt(error.message());
}

fn report_unrepresentable_conductor_attachment(state: &mut CanvasParts<'_>) {
    state.prompt("No schematic point can be attached to that conductor; the wire was not started.");
}

/// Whether a label dropped at `anchor` would sit exactly where two of this
/// sheet's wires cross with no junction, naming neither of them.
fn label_anchor_on_unmarked_crossing(state: &CanvasParts<'_>, anchor: Point) -> bool {
    let document = state.model.schematic.document();
    let wires = state
        .design_view()
        .objects_on_active_sheet(&document.wires, |wire| wire.id);
    let junctions = state
        .design_view()
        .objects_on_active_sheet(&document.junctions, |junction| junction.id);
    rspice_design::schematic::junction_edit::CrossingIndex::new(&wires, &junctions)
        .is_unmarked_crossing(anchor)
}

fn refuse_label_on_unmarked_crossing(ui: &Ui, state: &mut CanvasParts<'_>, anchor: Point) {
    let message = format!(
        "A label at ({}, {}) would sit where two wires cross with no junction, and would join \
         neither. Place a junction there to join the wires, or drop the label onto one wire.",
        anchor.x, anchor.y
    );
    // The toast is the answer: the label the user dropped is not there, and
    // the reason is longer than the status bar holds.
    state
        .workbench
        .toasts
        .warn_with_title(ui.ctx(), "Label not placed", message);
}

/// "/" while routing: the bend of the route flips between horizontal first
/// and vertical first, for wires and buses alike, live on the route being
/// drawn. The choice is the one Grid, snap and routing sets, so it lasts.
pub(super) fn flip_route_corner(state: &mut CanvasParts<'_>) {
    use crate::state::WireRoutingMode;

    let flipped = match state
        .model
        .schematic
        .session
        .editor
        .wire_drawing
        .routing_mode
    {
        WireRoutingMode::HorizontalFirst => WireRoutingMode::VerticalFirst,
        WireRoutingMode::VerticalFirst => WireRoutingMode::HorizontalFirst,
        WireRoutingMode::FortyFiveDegree | WireRoutingMode::Diagonal => {
            state.prompt(
                "Only a square bend flips. Choose Horizontal first or Vertical first in Grid, snap and routing.",
            );
            return;
        }
    };
    state
        .design
        .set_routing_mode(&mut state.model.schematic, flipped);
    state.prompt(if flipped == WireRoutingMode::VerticalFirst {
        "Routing: vertical first."
    } else {
        "Routing: horizontal first."
    });
}

/// A placed part whose pin lands inside one of this sheet's wires is
/// connected there: the wire splits so the pin owns a vertex and the wire
/// follows the part. The dots any join earns, inside a wire, at a wire's
/// end or on another pin, are placed under the document's junction policy.
pub(super) fn connect_pins_inside_wires(
    workspace: &crate::state::ProjectWorkspace,
    schematic: &mut crate::state::SchematicState,
    component: u64,
    symbols: &SchematicSymbolContext,
) {
    with_active_wire_topology_in(workspace, schematic, |schematic, scope| {
        let placed = std::collections::HashSet::from([component]);
        schematic.split_wires_under_pins(
            &placed,
            |component| symbols.terminal_points(component),
            scope,
        );
        // A pin dropped onto a wire's end or onto another pin joins them
        // with no wire to split, and can still make a point where three
        // conductor ends meet: the dots follow the drawing either way.
        if schematic.session.editor.automatic_junctions {
            schematic.update_wire_junctions(scope);
        }
    });
}

pub(super) fn place_component(
    state: &mut CanvasParts<'_>,
    component_type: ComponentType,
    grid_pos: Point,
) {
    match component_type {
        ComponentType::Port => place_pending_port(state, grid_pos),
        ComponentType::CellInstance => {
            let Some(pending) = state
                .model
                .schematic
                .session
                .editor
                .pending_library_cell
                .as_ref()
            else {
                state.prompt("Choose a library cell to place.");
                state.model.schematic.cancel_tool();
                return;
            };
            if state.schematic_edit_read_only()
                || state.model.schematic.session.editor.tool
                    != Tool::Place(ComponentType::CellInstance)
                || !pending
                    .authority
                    .matches(&super::requests::editor_request_source(state))
            {
                state.prompt(
                    "Library cell not placed: the active schematic changed. Choose the cell again.",
                );
                state.model.schematic.cancel_tool();
                return;
            }
            let library_cell = pending.binding.clone();
            let symbols = state.model.schematic_symbol_context();
            let workspace = &state.model.workspace;
            let changed = state
                .model
                .schematic
                .with_undo("place library cell", |schematic| {
                    let id = schematic.add_library_cell_component(grid_pos, library_cell);
                    connect_pins_inside_wires(workspace, schematic, id, &symbols);
                });
            if changed {
                log::info!("Placed library cell instance at {:?}", grid_pos);
            }
        }
        // An armed stimulus definition is placed as an adopter in this undo group.
        _ => {
            let symbols = state.model.schematic_symbol_context();
            let workspace = &state.model.workspace;
            let changed = state.model.schematic.with_undo(
                format!("place {}", component_type.display_name()),
                |schematic| {
                    let id = schematic.add_armed_component(component_type, grid_pos);
                    connect_pins_inside_wires(workspace, schematic, id, &symbols);
                },
            );
            if changed {
                log::info!("Placed {:?} at {:?}", component_type, grid_pos);
            }
        }
    }
}

fn place_pending_design_note(state: &mut CanvasParts<'_>, grid_pos: Point) {
    let Some(pending) = state
        .model
        .schematic
        .session
        .editor
        .pending_design_note
        .clone()
    else {
        state.prompt("No note is armed. Reopen Place text or note.");
        state.model.schematic.cancel_tool();
        return;
    };
    let authority_matches = pending
        .source
        .as_ref()
        .is_some_and(|source| *source == super::requests::editor_request_source(state));
    if state.schematic_edit_read_only() || !authority_matches {
        state.prompt("Note not placed: the active schematic changed. Reopen Place text or note.");
        state.model.schematic.cancel_tool();
        return;
    }
    let kind = pending.kind.label();
    match state
        .model
        .schematic
        .place_pending_design_note(grid_pos, pending)
    {
        Ok(_) => {
            state.model.schematic.cancel_tool();
            state.sync_active_schematic_to_workspace();
            state.prompt(format!("Placed {kind}."));
        }
        Err(error) => {
            state.prompt(format!("Note not placed: {error}"));
            state.model.schematic.cancel_tool();
        }
    }
}

fn handle_documentation_shape_click(
    state: &mut CanvasParts<'_>,
    grid_pos: Point,
    finish_polygon: bool,
) {
    state
        .model
        .schematic
        .session
        .editor
        .documentation_shape_drawing
        .keyboard_cursor = Some(grid_pos);
    state
        .model
        .schematic
        .session
        .editor
        .documentation_shape_drawing
        .keyboard_active = false;
    let Some(pending) = state
        .model
        .schematic
        .session
        .editor
        .pending_documentation_shape
        .as_ref()
    else {
        state.prompt("No shape is armed. Reopen Draw documentation shape.");
        state.model.schematic.cancel_tool();
        return;
    };
    let authority_matches = pending
        .source
        .as_ref()
        .is_some_and(|source| *source == super::requests::editor_request_source(state));
    if state.schematic_edit_read_only() || !authority_matches {
        state.prompt(
            "Shape not placed: the active schematic changed. Reopen Draw documentation shape.",
        );
        state.model.schematic.cancel_tool();
        return;
    }
    let kind = pending.kind;
    match state
        .model
        .schematic
        .session
        .editor
        .documentation_shape_drawing
        .add_point(kind, grid_pos)
    {
        Ok(auto_commit) if auto_commit || finish_polygon => {
            finish_documentation_shape(state, kind);
        }
        Ok(_) => {}
        Err(error) => report_documentation_shape_error(state, error),
    }
}

fn shape_input_view<'a>(state: &'a CanvasParts<'_>) -> ShapeInputView<'a> {
    ShapeInputView {
        drawing: &state
            .model
            .schematic
            .session
            .editor
            .documentation_shape_drawing,
        kind: state
            .model
            .schematic
            .session
            .editor
            .pending_documentation_shape
            .as_ref()
            .map(|pending| pending.kind),
        snap_engine: &state.model.schematic.session.editor.snap_engine,
        grid_size: state.model.schematic.document().grid_size,
    }
}

fn handle_documentation_shape_keyboard(
    ui: &Ui,
    response: &Response,
    state: &mut CanvasParts<'_>,
    viewport: &Viewport,
    grid_size: i32,
) {
    if let Some(transition) = documentation_shape_input::keyboard(
        ui,
        response,
        viewport,
        shape_input_view(state),
        grid_size,
    ) {
        let request = capture_shape_input(state, transition);
        apply_shape_input(state, request);
    }
}

pub fn capture_shape_input(
    state: &CanvasParts<'_>,
    transition: ShapeInputTransition,
) -> ShapeInputRequest {
    ShapeInputRequest {
        source: super::requests::editor_request_source(state),
        kind: state
            .model
            .schematic
            .session
            .editor
            .pending_documentation_shape
            .as_ref()
            .map(|pending| pending.kind),
        transition,
    }
}

pub fn apply_shape_input(state: &mut CanvasParts<'_>, request: ShapeInputRequest) {
    if request.source != super::requests::editor_request_source(state)
        || request.kind
            != state
                .model
                .schematic
                .session
                .editor
                .pending_documentation_shape
                .as_ref()
                .map(|pending| pending.kind)
        || request.transition.expected
            != state
                .model
                .schematic
                .session
                .editor
                .documentation_shape_drawing
        || state.model.schematic.session.editor.tool != Tool::DocumentationShape
        || state.application_modal_open()
    {
        return;
    }
    state
        .model
        .schematic
        .session
        .editor
        .documentation_shape_drawing = request.transition.next;
    match request.transition.action {
        Some(ShapeInputAction::PlacePoint(cursor)) => {
            handle_documentation_shape_click(state, cursor, false);
            if state.model.schematic.session.editor.tool == Tool::DocumentationShape {
                state
                    .model
                    .schematic
                    .session
                    .editor
                    .documentation_shape_drawing
                    .keyboard_active = true;
            }
        }
        Some(ShapeInputAction::PointerPoint(point)) => {
            handle_documentation_shape_click(state, point, false);
        }
        Some(ShapeInputAction::FinishPolygon) => finish_documentation_polygon(state),
        None => {}
    }
}

fn finish_documentation_polygon(state: &mut CanvasParts<'_>) {
    let Some(kind) = state
        .model
        .schematic
        .session
        .editor
        .pending_documentation_shape
        .as_ref()
        .map(|pending| pending.kind)
    else {
        return;
    };
    if kind == crate::state::DocumentationShapeKind::Polygon {
        finish_documentation_shape(state, kind);
    }
}

fn finish_documentation_shape(
    state: &mut CanvasParts<'_>,
    kind: crate::state::DocumentationShapeKind,
) {
    let geometry = match state
        .model
        .schematic
        .session
        .editor
        .documentation_shape_drawing
        .geometry(kind)
    {
        Ok(geometry) => geometry,
        Err(error) => {
            report_documentation_shape_error(state, error);
            return;
        }
    };
    let Some(pending) = state
        .model
        .schematic
        .session
        .editor
        .pending_documentation_shape
        .clone()
    else {
        return;
    };
    let authority_error = if state.schematic_edit_read_only() {
        Some(crate::state::DocumentationShapeError::ReadOnly)
    } else if pending.source.as_ref() != Some(&super::requests::editor_request_source(state)) {
        Some(crate::state::DocumentationShapeError::StaleDocument)
    } else {
        None
    };
    if let Some(error) = authority_error {
        report_documentation_shape_error(state, error);
        state.model.schematic.cancel_tool();
        return;
    }
    match state
        .model
        .schematic
        .commit_documentation_shape(pending, geometry)
    {
        Ok(_) => {
            state.model.schematic.cancel_tool();
            state.sync_active_schematic_to_workspace();
            state.prompt(format!("Placed {}.", kind.label()));
        }
        Err(error) => {
            report_documentation_shape_error(state, error);
            if matches!(
                error,
                crate::state::DocumentationShapeError::ReadOnly
                    | crate::state::DocumentationShapeError::StaleDocument
            ) {
                state.model.schematic.cancel_tool();
            }
        }
    }
}

fn report_documentation_shape_error(
    state: &mut CanvasParts<'_>,
    error: crate::state::DocumentationShapeError,
) {
    state.prompt(format!("Shape not placed: {error}"));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum JunctionPlacementOutcome {
    Placed(Point),
    Removed(Point),
    /// The dot marks a point where three or more conductor ends meet, and
    /// automatic junctions keep it while they do.
    Required(Point),
    NoIntersection,
    MixedBus,
}

/// Whether the active sheet's wires and pins make a dot due at `point`
/// whether or not one is placed there: a T, three wire ends or wires on a
/// pin, as opposed to a crossing that only a placed dot joins.
fn dot_required_on_active_sheet(state: &CanvasParts<'_>, point: Point) -> bool {
    let document = state.model.schematic.document();
    let sheet = rspice_design::schematic::document::SchematicDocument {
        wires: state
            .design_view()
            .objects_on_active_sheet(&document.wires, |wire| wire.id)
            .into_owned(),
        components: state
            .design_view()
            .objects_on_active_sheet(&document.components, |part| part.id)
            .into_owned(),
        ..Default::default()
    };
    rspice_design::schematic::junction_edit::JunctionRule::for_document(&sheet)
        .dot_belongs(point, false)
}

fn commit_explicit_junction(
    state: &mut CanvasParts<'_>,
    requested: Point,
) -> JunctionPlacementOutcome {
    let grid_size = state.model.schematic.document().grid_size;
    let Some(target) = state
        .design_view()
        .nearest_junction_candidate(requested, grid_size)
    else {
        return JunctionPlacementOutcome::NoIntersection;
    };

    let buses = state
        .design_view()
        .objects_on_active_sheet(&state.model.schematic.document().buses, |item| item.id);
    if buses.iter().any(|bus| bus.contains_point(target)) {
        return JunctionPlacementOutcome::MixedBus;
    }

    if let Some(junction_id) = state.design_view().active_junction_at(target) {
        let automatic = state.model.schematic.session.editor.automatic_junctions;
        if automatic && dot_required_on_active_sheet(state, target) {
            return JunctionPlacementOutcome::Required(target);
        }
        state
            .model
            .schematic
            .with_undo("remove junction", |schematic| {
                schematic.remove_junction(junction_id);
            });
        return JunctionPlacementOutcome::Removed(target);
    }

    state
        .model
        .schematic
        .with_undo("place junction", |schematic| {
            schematic.add_junction(target);
        });
    JunctionPlacementOutcome::Placed(target)
}

pub(super) fn handle_junction_click(state: &mut CanvasParts<'_>, requested: Point) {
    let message = match commit_explicit_junction(state, requested) {
        JunctionPlacementOutcome::Placed(point) => {
            format!("Added a junction at ({}, {}).", point.x, point.y)
        }
        JunctionPlacementOutcome::Removed(point) => {
            format!("Removed the junction at ({}, {}).", point.x, point.y)
        }
        JunctionPlacementOutcome::Required(point) => format!(
            "The junction at ({}, {}) stays: three or more conductors meet there.",
            point.x, point.y
        ),
        JunctionPlacementOutcome::NoIntersection => {
            "No junction: two conductors do not meet there.".to_owned()
        }
        JunctionPlacementOutcome::MixedBus => {
            "No junction: a wire joins a bus through a bus tap.".to_owned()
        }
    };
    state.prompt(message);
}

fn handle_select_click(
    ui: &Ui,
    state: &mut CanvasParts<'_>,
    hit: PointerHit,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
    viewport: &Viewport,
    pointer_pos: egui::Pos2,
) {
    // Ctrl, Shift, and the platform command modifier extend the selection;
    // a plain click replaces it. On touch the strip's add latch stands in
    // for the modifier.
    let additive = ui.input(|i| i.modifiers.ctrl || i.modifiers.shift || i.modifiers.command)
        || (rspice_schematic_editor::view::pointer_target::canvas_touch_mode(ui.ctx())
            && rspice_schematic_editor::view::mobile_controls::touch_latches(ui.ctx()).select_add);
    let alt_held = ui.input(|i| i.modifiers.alt);

    // The click that opens or closes the overlap chooser selects nothing; a
    // plain click again on the same spot takes the next object under it.
    if super::pick_chooser::chooser_takes_click(ui.ctx()) {
        return;
    }
    let target = super::pick_chooser::click_target(
        ui.ctx(),
        state.model,
        state.design,
        rspice_schematic_editor::view::pointer_target::PointerQuery {
            hit,
            radius: hit_radius,
            position: pointer_pos,
        },
        symbol_context,
        viewport,
        additive,
    );
    let request = rspice_schematic_editor::requests::EditorRequest {
        source: super::requests::editor_request_source(state),
        selection: state.model.schematic.session.editor.selection.clone(),
        action: rspice_schematic_editor::requests::EditorAction::SelectPointer {
            target,
            additive,
            alt_held,
        },
    };
    super::requests::apply_editor_request(state, request);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProbeSignalOutcome {
    /// The last run holds the signal, and the results workspace was asked to
    /// plot it; it says what it did.
    PlotRequested,
    GroundReference,
    SavedOutputCreated {
        plan_name: String,
    },
    SavedOutputAlreadyPresent {
        plan_name: String,
    },
    NotInLastRun,
    Rejected {
        reason: String,
    },
}

fn is_ground_voltage_expression(expression: &str) -> bool {
    normalized_probe_expression(expression) == "v(0)"
}

fn unique_probe_output_name(outputs: &[SavedOutput], preferred: &str) -> String {
    if preferred.len() <= 256
        && !outputs
            .iter()
            .any(|output| output.name.eq_ignore_ascii_case(preferred))
    {
        return preferred.to_owned();
    }

    for ordinal in 1..=outputs.len().saturating_add(1) {
        let candidate = format!("Schematic probe {ordinal}");
        if !outputs
            .iter()
            .any(|output| output.name.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
    }
    unreachable!("one more generated name than existing outputs must be available")
}

/// The active analysis's trace of `probe_name`, when the last run holds it.
fn materialized_probe_trace(
    model: &Model,
    probe_name: &str,
) -> Option<rspice_results_ui::selection::SelectedResultTrace> {
    let run = model.simulation.active_run()?;
    let analysis = model.simulation.active_analysis()?;
    let analysis_index = run
        .analyses
        .iter()
        .position(|candidate| std::ptr::eq(candidate, analysis))?;
    let waveform_index = analysis.waveforms.iter().position(|waveform| {
        normalized_probe_expression(&waveform.name) == normalized_probe_expression(probe_name)
            || waveform.name.eq_ignore_ascii_case(probe_name)
    })?;
    rspice_results_ui::selection::SelectedResultTrace::from_run_indices(
        run,
        analysis_index,
        waveform_index,
    )
}

/// Ask the results workspace to plot the last run's trace of `spelling`, as
/// a tick in its Data Browser does, and to select it. `None` when the last
/// run does not hold the signal.
fn plot_materialized_trace(
    state: &mut CanvasParts<'_>,
    spelling: &OccurrenceProbeSpelling,
    plot: ProbePlot,
) -> Option<ProbeSignalOutcome> {
    let trace = materialized_probe_trace(state.model, spelling.engine())?;
    state.workbench.outbox.push(PlotProbedTrace {
        trace,
        plot,
        display: spelling.display().to_owned(),
    });
    Some(ProbeSignalOutcome::PlotRequested)
}

#[derive(Debug, Clone)]
struct ProbeOutputBinding {
    plan_name: String,
    created: bool,
}

/// Make sure the active plan saves `spelling`'s signal for the next run.
///
/// `origin` says who owns a new output. A probe marker owns its own, which
/// lives as long as the marker does. An action that keeps no marker, the net
/// inspector's Plot, gives it to the plan, since a probe-owned output with
/// no marker behind it is dropped when the run is prepared; for the same
/// reason it takes over a probe-owned output of the signal it finds.
fn ensure_plan_probe_output(
    state: &mut CanvasParts<'_>,
    spelling: &OccurrenceProbeSpelling,
    origin: SavedOutputOrigin,
) -> Result<ProbeOutputBinding, String> {
    let setup = state.model.sim_setup.clone();
    let plan_id = setup.stable_analysis_plan()?.id();
    let plan_name = setup.active_plan_name().to_string();
    let expression = spelling.engine().trim();
    let expression_key = normalized_probe_expression(expression);
    let existing = state
        .model
        .workspace
        .content
        .plan_data(plan_id)
        .and_then(|payload| {
            payload
                .saved_outputs
                .iter()
                .find(|output| {
                    output.kind == SavedOutputKind::RawVoltageOrCurrent
                        && normalized_probe_expression(&output.source_expression) == expression_key
                })
                .cloned()
        });
    if let Some(existing) = existing {
        if origin != SavedOutputOrigin::Plan || existing.origin != SavedOutputOrigin::SchematicProbe
        {
            return Ok(ProbeOutputBinding {
                plan_name,
                created: false,
            });
        }
        let mut workspace = state.model.workspace.clone();
        let id = existing.id;
        workspace
            .content
            .replace_saved_output(plan_id, id, existing.with_origin(SavedOutputOrigin::Plan))
            .map_err(|error| error.to_string())?;
        publish_plan_outputs(
            state,
            setup,
            workspace,
            format!("Kept {expression} as a plan output."),
        )?;
        return Ok(ProbeOutputBinding {
            plan_name,
            created: true,
        });
    }

    let output_name = unique_probe_output_name(
        state
            .model
            .workspace
            .content
            .plan_data(plan_id)
            .map_or(&[], |payload| payload.saved_outputs.as_slice()),
        spelling.display().trim(),
    );
    let output = SavedOutput::new(
        SavedOutputKind::RawVoltageOrCurrent,
        output_name,
        expression,
        SavedOutputCompatibility::AllCompatibleAnalyses,
        // A schematic probe is the ordinary design-to-results path. It must
        // compile into a storage-bounded request for every default analysis.
        // Transient preparation maps this policy onto the configured output
        // grid and retains the exact final point.
        SavedOutputPolicy::SelectedAndFinalPoints,
        SavedOutputPrecision::DisplayCacheWithFullSourcePrecision,
        SavedOutputStreaming::LivePlotAdaptiveDisplayDecimation,
    )?
    .with_origin(origin);
    let preflight =
        crate::simulation::SimulationController::new().saved_output_preflight(state.model, &output);
    if let rspice_simulation::output_contract::SavedOutputSemanticStatus::Invalid { reason } =
        preflight.semantic_status()
    {
        return Err(format!(
            "the active plan cannot materialize this probe: {reason}"
        ));
    }

    let mut workspace = state.model.workspace.clone();
    workspace
        .content
        .add_saved_output(plan_id, output)
        .map_err(|error| error.to_string())?;
    publish_plan_outputs(
        state,
        setup,
        workspace,
        format!("Added schematic probe output {expression}."),
    )?;
    Ok(ProbeOutputBinding {
        plan_name,
        created: true,
    })
}

/// Validate a changed output list and commit it with the plan setup in one
/// step, so a failure leaves the live configuration as it was.
fn publish_plan_outputs(
    state: &mut CanvasParts<'_>,
    mut setup: rspice_app_state::workbench::app_state::sim_setup::SimSetupState,
    workspace: rspice_app_state::state::ProjectWorkspace,
    description: String,
) -> Result<(), String> {
    workspace
        .content
        .validate_simulation_configuration()
        .map_err(|error| error.to_string())?;
    let receipt = setup
        .commit_active_plan_configuration_change(description)
        .map_err(|error| error.to_string())?;
    state.model.sim_setup = setup;
    state.model.workspace = workspace;
    state.runs.preflight.invalidate();
    state.model.record_plan_receipt(receipt.status_line());
    Ok(())
}

/// Resolve a probe into either an immediate plot transaction or a durable
/// plan-owned output request.
///
/// The saved-output fallback publishes a cloned, fully validated workspace
/// and plan setup together. Failures therefore leave the live configuration
/// unchanged. Existing raw outputs are identified by their source expression,
/// making repeated probes idempotent even if an output was named elsewhere.
/// That expression is the engine's, never the reader's: two instances of one
/// master show the same `V(n1)` and request two different nodes.
///
/// A probe puts its signal on the plot and never takes one off, as
/// Virtuoso's direct plot and KiCad's simulator probe do: a net the run
/// already plots stays plotted.
fn request_probe_signal(
    state: &mut CanvasParts<'_>,
    spelling: &OccurrenceProbeSpelling,
) -> ProbeSignalOutcome {
    request_probe_plot(
        state,
        spelling,
        ProbePlot::Show,
        SavedOutputOrigin::SchematicProbe,
    )
}

/// The net inspector's Plot button: on the plot, or off it when it is
/// there. It keeps no marker, so what it saves for the next run is the
/// plan's.
fn request_net_plot(
    state: &mut CanvasParts<'_>,
    spelling: &OccurrenceProbeSpelling,
) -> ProbeSignalOutcome {
    request_probe_plot(state, spelling, ProbePlot::Toggle, SavedOutputOrigin::Plan)
}

fn request_probe_plot(
    state: &mut CanvasParts<'_>,
    spelling: &OccurrenceProbeSpelling,
    plot: ProbePlot,
    origin: SavedOutputOrigin,
) -> ProbeSignalOutcome {
    if is_ground_voltage_expression(spelling.engine()) {
        return ProbeSignalOutcome::GroundReference;
    }
    let binding = match ensure_plan_probe_output(state, spelling, origin) {
        Ok(binding) => binding,
        Err(reason) => return ProbeSignalOutcome::Rejected { reason },
    };
    if let Some(outcome) = plot_materialized_trace(state, spelling, plot) {
        return outcome;
    }
    if binding.created {
        ProbeSignalOutcome::SavedOutputCreated {
            plan_name: binding.plan_name,
        }
    } else {
        ProbeSignalOutcome::SavedOutputAlreadyPresent {
            plan_name: binding.plan_name,
        }
    }
}

/// A read-only view writes neither its drawing nor the plan, so a probe there
/// plots what the last run holds and asks the next run for nothing.
fn request_retained_probe_signal(
    state: &mut CanvasParts<'_>,
    spelling: &OccurrenceProbeSpelling,
) -> ProbeSignalOutcome {
    if is_ground_voltage_expression(spelling.engine()) {
        return ProbeSignalOutcome::GroundReference;
    }
    plot_materialized_trace(state, spelling, ProbePlot::Show)
        .unwrap_or(ProbeSignalOutcome::NotInLastRun)
}

/// Whether the last run's trace of `display`, the voltage or current of the
/// local `name`, is on the plot now, as the results workspace shows it.
pub(crate) fn probe_trace_plotted(
    model: &Model,
    results: &dyn crate::workbench::framework::ports::ResultsPort,
    name: &str,
    display: &str,
) -> bool {
    let Ok(spelling) = probe_spelling_for(model, name, display) else {
        return false;
    };
    let Some((trace, run)) =
        materialized_probe_trace(model, spelling.engine()).zip(model.simulation.active_run())
    else {
        return false;
    };
    let Some((.., waveform)) = trace.resolve(run) else {
        return false;
    };
    let key = rspice_results::result_presentation::SourceWaveformPresentationKey::new(
        trace.analysis_key(),
        trace.source_name(),
    );
    results.waveform_visibility(&key, waveform.visible)
}

/// Re-spell a probe request at the occurrence the active tab is editing.
///
/// `display` is the expression the caller means and `name` is the local leaf
/// inside it. The two only compose when `display` is exactly `V(name)` or
/// `I(name)`; anything else — a marker's stored expression, an authored
/// quantity — already names an exact node and is used as written.
fn probe_spelling_for(
    model: &Model,
    name: &str,
    display: &str,
) -> Result<OccurrenceProbeSpelling, String> {
    let Some(quantity) = ['V', 'I'].into_iter().find(|quantity| {
        rspice_schematic_editor::view::cross_probe::wrapped_signal_name(display, *quantity)
            .is_some_and(|leaf| leaf == name)
    }) else {
        return Ok(OccurrenceProbeSpelling::verbatim(display));
    };
    let occurrence = model.workspace.content.occurrence_path();
    OccurrenceProbeSpelling::for_leaf(&occurrence, quantity, name)
        .ok_or_else(|| unspellable_occurrence(&occurrence))
}

fn unspellable_occurrence(occurrence: &crate::state::InstancePath) -> String {
    format!("instance path {occurrence} has no name the engine can be asked for")
}

/// Commit a current probe on one element at the active occurrence. The reader
/// is shown the element's drawn name; the engine is asked for the element by
/// its own name, which a prefix, a behavioural lowering or a keyword makes
/// differ from the drawn one.
fn commit_element_current(
    state: &mut CanvasParts<'_>,
    drawn: &str,
    engine: &str,
    request: ProbeRequest,
) -> (OccurrenceProbeSpelling, ProbeSignalOutcome) {
    let occurrence = state.model.workspace.content.occurrence_path();
    match OccurrenceProbeSpelling::for_element(&occurrence, 'I', drawn, engine) {
        Some(spelling) => {
            let outcome = request(state, &spelling);
            (spelling, outcome)
        }
        None => (
            OccurrenceProbeSpelling::verbatim(format!("I({drawn})")),
            ProbeSignalOutcome::Rejected {
                reason: unspellable_occurrence(&occurrence),
            },
        ),
    }
}

/// Resolve a probe at the active occurrence and commit it through `request`.
///
/// The spelling comes back with the outcome because every caller reports the
/// result to the reader, and the reader is owed the design's address rather
/// than the flattened node the engine answered on.
fn commit_probe_request(
    state: &mut CanvasParts<'_>,
    name: &str,
    display: &str,
    request: ProbeRequest,
) -> (OccurrenceProbeSpelling, ProbeSignalOutcome) {
    match probe_spelling_for(state.model, name, display) {
        Ok(spelling) => {
            let outcome = request(state, &spelling);
            (spelling, outcome)
        }
        Err(reason) => (
            OccurrenceProbeSpelling::verbatim(display),
            ProbeSignalOutcome::Rejected { reason },
        ),
    }
}

/// Resolve a probed signal and report the exact committed outcome.
///
/// The canvas probe tool and the inspector's plot action commit the same
/// transaction, so a net can never be "plotted" by one surface and absent
/// from the other. Materialized data plots immediately; otherwise the active
/// simulation plan receives an idempotent saved-output contract.
pub(crate) fn toggle_probe_with_feedback(
    state: &mut CanvasParts<'_>,
    name: &str,
    display: &str,
) -> bool {
    let (spelling, outcome) = commit_probe_request(state, name, display, request_net_plot);
    let configuration_changed = matches!(&outcome, ProbeSignalOutcome::SavedOutputCreated { .. });
    report_probe_outcome(state, spelling.display(), outcome);
    configuration_changed
}

/// Inspector/navigation action: reveal a probe without ever toggling a
/// currently visible trace off.
pub(crate) fn ensure_probe_visible_with_feedback(
    state: &mut CanvasParts<'_>,
    name: &str,
    display: &str,
) -> bool {
    let (spelling, outcome) = commit_probe_request(state, name, display, request_probe_signal);
    let configuration_changed = matches!(&outcome, ProbeSignalOutcome::SavedOutputCreated { .. });
    report_probe_outcome(state, spelling.display(), outcome);
    configuration_changed
}

/// Reveal already-retained evidence without authoring a future output. This
/// path keeps cross-probing useful for read-only library/testbench views while
/// preserving their write boundary.
pub(crate) fn ensure_retained_probe_visible_with_feedback(
    state: &mut CanvasParts<'_>,
    name: &str,
    display: &str,
) -> bool {
    let spelling = match probe_spelling_for(state.model, name, display) {
        Ok(spelling) => spelling,
        Err(reason) => {
            report_probe_outcome(state, display, ProbeSignalOutcome::Rejected { reason });
            return false;
        }
    };
    let outcome = if is_ground_voltage_expression(spelling.engine()) {
        ProbeSignalOutcome::GroundReference
    } else {
        plot_materialized_trace(state, &spelling, ProbePlot::Show).unwrap_or_else(|| {
            ProbeSignalOutcome::Rejected {
                reason: "no retained compatible waveform is available in this read-only view"
                    .to_owned(),
            }
        })
    };
    let shown = matches!(&outcome, ProbeSignalOutcome::PlotRequested);
    report_probe_outcome(state, spelling.display(), outcome);
    shown
}

fn report_probe_outcome(state: &mut CanvasParts<'_>, display: &str, outcome: ProbeSignalOutcome) {
    state.prompt(match outcome {
        // The results workspace says what it did when it plots.
        ProbeSignalOutcome::PlotRequested => return,
        ProbeSignalOutcome::GroundReference => "Node 0 is the 0 V reference.".to_owned(),
        ProbeSignalOutcome::SavedOutputCreated { plan_name } => format!(
            "{display} added to the saved outputs of {plan_name}; it is plotted after the next run."
        ),
        ProbeSignalOutcome::SavedOutputAlreadyPresent { plan_name } => format!(
            "{display} is already a saved output of {plan_name}; it is plotted after the next run."
        ),
        ProbeSignalOutcome::NotInLastRun => format!(
            "{display} is not in the last run, and a read-only view cannot add it to the plan."
        ),
        ProbeSignalOutcome::Rejected { reason } => format!("{display} not saved: {reason}."),
    });
}

fn probe_edit_identity_is_current(state: &CanvasParts<'_>) -> Result<(), String> {
    if !matches!(
        state.model.workspace.content.active_view_type(),
        Some(ViewType::Schematic | ViewType::Testbench)
    ) {
        return Err("the active cell/view is not a schematic document".to_owned());
    }
    if state.model.workspace.content.active_schematic_reference()
        != state.model.workspace.content.active_view
    {
        return Err("the active schematic identity changed before the probe was placed".to_owned());
    }
    if state.model.workspace.active_schematic().is_none() {
        return Err("the active schematic has no project-owned document buffer".to_owned());
    }
    if state.schematic_edit_read_only() {
        return Err("the active schematic is read-only".to_owned());
    }
    Ok(())
}

fn current_probe_output_binding(
    state: &CanvasParts<'_>,
    expression: &str,
) -> Option<(
    crate::product::SimulationPlanId,
    crate::product::SavedOutputId,
)> {
    let plan_id = state.model.sim_setup.stable_analysis_plan().ok()?.id();
    let expression_key = normalized_probe_expression(expression);
    let output_id = state
        .model
        .workspace
        .content
        .plan_data(plan_id)?
        .saved_outputs
        .iter()
        .find_map(|output| {
            (output.kind == SavedOutputKind::RawVoltageOrCurrent
                && normalized_probe_expression(&output.source_expression) == expression_key)
                .then_some(output.id)
        })?;
    Some((plan_id, output_id))
}

/// Retain the marker for a probe.
///
/// The marker's reference is the reader's spelling and its source expression is
/// the engine's, because the label is read on the drawing while the expression
/// is matched against a plan's saved outputs.
fn retain_probe_flag(
    state: &mut CanvasParts<'_>,
    position: Point,
    spelling: Option<&OccurrenceProbeSpelling>,
    binding: Option<(
        crate::product::SimulationPlanId,
        crate::product::SavedOutputId,
    )>,
) -> Result<u64, String> {
    probe_edit_identity_is_current(state)?;
    let source_expression = spelling.map(|spelling| spelling.engine().trim().to_owned());
    let label = spelling.map(|spelling| spelling.display().trim().to_owned());
    let validation_reference = label.as_deref().unwrap_or("P1");
    SchematicProbe::new(1, position, validation_reference, source_expression.clone())?;
    let source_key = source_expression
        .as_deref()
        .map(normalized_probe_expression);
    if let Some(existing_id) = state
        .model
        .schematic
        .document()
        .probes
        .iter()
        .find_map(|probe| {
            (probe.position == position
                && probe
                    .source_expression
                    .as_deref()
                    .map(normalized_probe_expression)
                    == source_key)
                .then_some(probe.id)
        })
    {
        let needs_binding_refresh = state
            .model
            .schematic
            .document()
            .probes
            .iter()
            .find(|probe| probe.id == existing_id)
            .is_some_and(|probe| {
                binding.is_some_and(|(plan_id, output_id)| {
                    probe.plan_id != Some(plan_id) || probe.saved_output_id != Some(output_id)
                })
            });
        if needs_binding_refresh {
            state
                .model
                .schematic
                .bind_probe_saved_output(existing_id, binding);
            state.sync_active_schematic_to_workspace();
        }
        let id = existing_id;
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_probe(id);
        state.design.interaction.schematic_keyboard_focus = Some(
            crate::workbench::app_state::SchematicKeyboardFocus::Probe(id),
        );
        return Ok(id);
    }
    let probe_id =
        state
            .model
            .schematic
            .place_schematic_probe(position, label, source_expression, binding)?;
    state.design.interaction.schematic_keyboard_focus = Some(
        crate::workbench::app_state::SchematicKeyboardFocus::Probe(probe_id),
    );
    state.sync_active_schematic_to_workspace();
    Ok(probe_id)
}

/// Nets of the open view as the configured design resolves it.
///
/// A configuration that does not resolve yields no name at all rather than
/// the editor buffer's answer: a probe carries its net name into a run
/// receipt, and a name taken from a hierarchy the design does not have is
/// wrong rather than approximate. The reason is logged so the gesture that
/// found no name is explicable.
fn live_design_nets(model: &Model) -> std::sync::Arc<Vec<DesignNet>> {
    match model.workspace.design_projection(
        &model.library_manager,
        &model.workspace.content.active_view,
        &model.schematic,
    ) {
        Ok(projection) => projection_nets(
            model.library_manager.catalog(),
            &projection,
            &model.workspace.content.active_view.key(),
        ),
        Err(error) => {
            log::warn!("Probe naming has no design projection: {error}");
            std::sync::Arc::new(Vec::new())
        }
    }
}

/// Light the net a canvas gesture selects, under the name the deck gives it.
///
/// The canvas traces nothing of its own: two separated wire groups one label
/// or one interface port joins are one net to the netlister, so they light
/// together and carry the node name a probe would emit. A gesture the design
/// resolves no net for lights nothing rather than falling back to whichever
/// group the pointer happened to land on.
pub(super) fn highlight_canvas_net(model: &mut Model, select: impl Fn(&DesignNet) -> bool) {
    let lit = live_design_nets(model)
        .iter()
        .find(|net| select(net))
        .map(|net| {
            let wire_ids: std::collections::HashSet<u64> = net.wire_ids.iter().copied().collect();
            (net.name.clone(), wire_ids)
        });
    match lit {
        Some((name, wire_ids)) => {
            log::info!("Highlighted net '{name}' with {} wires", wire_ids.len());
            model
                .schematic
                .session
                .editor
                .net_highlight
                .highlight_named_wires(name, wire_ids);
        }
        None => model.schematic.session.editor.net_highlight.clear(),
    }
}

/// The one net the selection is on, as Design ▸ Highlight net lights it: a
/// selected wire's (whole, a segment or a corner), the net a selected label
/// names, or the net a selected part's terminals are on. `Err` says why
/// there is none to light.
pub fn selected_net(
    model: &Model,
    net_highlighting: SchematicNetHighlighting,
) -> Result<String, &'static str> {
    if !net_highlighting.lights_selected_net() {
        return Err("Display options light no selected net");
    }
    let selection = &model.schematic.session.editor.selection;
    let wires = selection
        .wires
        .iter()
        .copied()
        .chain(
            selection
                .wire_segments
                .iter()
                .map(|segment| segment.wire_id),
        )
        .chain(selection.wire_vertices.iter().map(|vertex| vertex.wire_id))
        .collect::<std::collections::HashSet<_>>();
    let labels = model
        .schematic
        .document()
        .net_labels
        .iter()
        .filter(|label| selection.net_labels.contains(&label.id))
        .map(|label| label.name.as_str())
        .collect::<Vec<_>>();
    if wires.is_empty() && labels.is_empty() && selection.components.is_empty() {
        return Err("select a wire, a net label or a pin");
    }
    let nets = live_design_nets(model);
    let mut on = nets.iter().filter(|net| {
        net.wire_ids.iter().any(|id| wires.contains(id))
            || labels
                .iter()
                .any(|label| label.eq_ignore_ascii_case(&net.name))
            || net
                .terminals
                .iter()
                .any(|terminal| selection.components.contains(&terminal.component_id))
    });
    let net = on.next().ok_or("the selection is on no net")?;
    if on.next().is_some() {
        return Err("the selection is on more than one net");
    }
    Ok(net.name.clone())
}

/// Light the net the selection is on, and say which, or why none was lit.
pub fn highlight_selected_net(
    model: &mut Model,
    net_highlighting: SchematicNetHighlighting,
) -> String {
    match selected_net(model, net_highlighting) {
        Ok(name) => {
            highlight_canvas_net(model, |net| net.name == name);
            format!("Highlighted net {name}.")
        }
        Err(reason) => format!("No net highlighted: {reason}."),
    }
}

fn exactly_one_net_name<'a>(mut matches: impl Iterator<Item = &'a DesignNet>) -> Option<String> {
    let name = matches.next()?.name.clone();
    matches.next().is_none().then_some(name)
}

/// Resolve the electrical name from the current schematic rather than
/// requiring a retained simulation run. Retained cross-probe data is only a
/// compatibility fallback when the live netlist projection cannot resolve a
/// source identity.
fn live_wire_probe_net_name(state: &CanvasParts<'_>, wire_id: u64) -> Option<String> {
    let nets = live_design_nets(state.model);
    exactly_one_net_name(nets.iter().filter(|net| net.wire_ids.contains(&wire_id)))
}

fn live_terminal_probe_net_name(
    state: &CanvasParts<'_>,
    component_id: u64,
    pin: &str,
    position: Point,
) -> Option<String> {
    let nets = live_design_nets(state.model);
    exactly_one_net_name(nets.iter().filter(|net| {
        net.terminals
            .iter()
            .any(|terminal| terminal.component_id == component_id && terminal.pin == pin)
    }))
    .or_else(|| {
        let wire_id = state.design_view().active_wire_at(position)?;
        exactly_one_net_name(nets.iter().filter(|net| net.wire_ids.contains(&wire_id)))
    })
}

fn retained_probe_net_name(state: &CanvasParts<'_>, position: Point) -> Option<String> {
    state
        .model
        .simulation
        .source
        .cross_probe
        .net_at_in(
            &state.model.workspace.content.active_view,
            state.model.schematic.topology_version(),
            super::sheet_visibility::active_sheet_id(state.model),
            position,
        )
        .cloned()
}

/// What a probe measures, unscoped: a click on a wire or a pin asks for a
/// net's voltage, a click on a part's body for its current.
#[derive(Debug, Clone, PartialEq, Eq)]
enum ComponentProbe {
    /// The voltage of the net under an exact pin.
    Voltage(String),
    /// The current of the element itself, shown by the name the drawing
    /// gives it and asked for by the name the engine knows it by.
    Current { drawn: String, engine: String },
}

/// The quantity and the local leaf a click on a component probes, unscoped.
///
/// The occurrence is applied later, at the request boundary: what the drawing
/// knows is `n1` and `R2`, and what makes those addresses is the tab.
fn component_probe_expression(
    state: &CanvasParts<'_>,
    component_id: u64,
    grid_pos: Point,
    symbol_context: &SchematicSymbolContext,
) -> Option<ComponentProbe> {
    let component = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == component_id)?;
    let resolved_symbol = symbol_context.resolved_symbol(component);
    // A cell instance without an authored/resolved symbol has no authoritative
    // pin identity. Its generic two-pin placeholder geometry must never be
    // treated as an electrical source contract for a retained probe.
    if component.kind == ComponentType::CellInstance && resolved_symbol.is_none() {
        return None;
    }
    let terminals = component.terminal_positions_resolved(resolved_symbol);

    // A snapped click on an exact pin is a node-voltage gesture. This matters
    // for unwired/dangling pins too: connectivity can still give that node an
    // authoritative generated name. A body click must never silently turn
    // into the voltage at whichever terminal happened to be nearest.
    if let Some((pin, terminal_position)) = terminals
        .iter()
        .find(|(_, terminal_position)| *terminal_position == grid_pos)
        .map(|(pin, terminal_position)| (pin.as_str(), *terminal_position))
    {
        let net_name = live_terminal_probe_net_name(state, component.id, pin, terminal_position)
            .or_else(|| retained_probe_net_name(state, terminal_position))?;
        return Some(ComponentProbe::Voltage(net_name));
    }

    // Structural objects and synthesized/multi-port blocks do not own one
    // unambiguous device-current observable. Their users must choose a
    // conductor or author an exact winding/lead expression. Ordinary emitted
    // SPICE devices use the conventional positive-reference I(instance)
    // quantity, including multi-terminal devices whose dialect defines that
    // accessor (for example the drain/reference lead of a MOS device). The
    // engine knows the device by the element the deck writes: a resistor
    // drawn `C4` is `RC4`, a limited VCVS drawn `E1` is the `B1` card, and a
    // memristor `MR1` is `YMEMRISTOR!MR1`.
    if matches!(
        component.kind,
        ComponentType::Ground
            | ComponentType::Port
            | ComponentType::Transformer
            | ComponentType::CoupledInductor
            | ComponentType::CellInstance
    ) || component.kind.is_xspice()
    {
        return None;
    }

    Some(ComponentProbe::Current {
        drawn: component.spice_instance_name(),
        engine: component.engine_element_name(),
    })
}

/// What a probe measures: the wire or the part a click or a menu named.
#[derive(Debug, Clone, Copy)]
enum ProbeSubject {
    Wire(u64),
    Part(u64),
}

fn handle_probe_click(
    state: &mut CanvasParts<'_>,
    grid_pos: Point,
    symbol_context: &SchematicSymbolContext,
) {
    let subject = probe_subject_at(state, grid_pos, symbol_context);
    probe_subject(state, grid_pos, subject, symbol_context);
}

/// The context menu's Probe row. On a wire or a part it probes that object
/// where the menu was opened, as a Probe-tool click there does, so the trace
/// comes with no second click; on the bare canvas it arms the Probe tool.
pub(super) fn probe_context_target(
    state: &mut CanvasParts<'_>,
    target: crate::workbench::app_state::ContextTarget,
    click_pos: Point,
    symbol_context: &SchematicSymbolContext,
) {
    use crate::workbench::app_state::ContextTarget;
    let (position, subject) = match target {
        ContextTarget::Canvas => {
            state.model.schematic.arm_tool(Tool::Probe);
            return;
        }
        // The menu opens within a few pixels of the wire; the marker goes on
        // the wire itself so it binds to what was probed.
        ContextTarget::Wire(wire_id) => {
            let on_wire = state
                .model
                .schematic
                .document()
                .wires
                .iter()
                .find(|wire| wire.id == wire_id)
                .and_then(|wire| wire.closest_point(click_pos))
                .map_or(click_pos, |(point, _)| point);
            (on_wire, ProbeSubject::Wire(wire_id))
        }
        ContextTarget::Component(part) => (click_pos, ProbeSubject::Part(part)),
    };
    probe_subject(state, position, Some(subject), symbol_context);
}

fn probe_subject_at(
    state: &CanvasParts<'_>,
    grid_pos: Point,
    symbol_context: &SchematicSymbolContext,
) -> Option<ProbeSubject> {
    if let Some(wire_id) = state.design_view().active_wire_at(grid_pos) {
        return Some(ProbeSubject::Wire(wire_id));
    }
    let components = state
        .design_view()
        .objects_on_active_sheet(&state.model.schematic.document().components, |item| item.id);
    symbol_context
        .component_at_resolved_symbol(components.as_ref(), grid_pos)
        .map(ProbeSubject::Part)
}

fn probe_subject(
    state: &mut CanvasParts<'_>,
    grid_pos: Point,
    subject: Option<ProbeSubject>,
    symbol_context: &SchematicSymbolContext,
) {
    // A read-only view writes neither its drawing nor the plan: a probe there
    // plots what the last run holds and keeps no marker.
    let read_only = state.schematic_edit_read_only();
    if !read_only && let Err(reason) = probe_edit_identity_is_current(state) {
        state.prompt(format!("Probe not placed: {reason}"));
        return;
    }
    let request = probe_request(read_only);

    if let Some(ProbeSubject::Wire(wire_id)) = subject {
        if let Some(net_name) = live_wire_probe_net_name(state, wire_id)
            .or_else(|| retained_probe_net_name(state, grid_pos))
        {
            log::info!("Probe: clicked net '{}' at {:?}", net_name, grid_pos);

            let display = format!("V({net_name})");
            let (spelling, outcome) = commit_probe_request(state, &net_name, &display, request);
            finish_probe(state, grid_pos, &spelling, outcome, read_only);
            if net_name != "0"
                && state
                    .model
                    .preferences
                    .toggle(crate::workbench::TogglePreference::CrossProbeBehavior)
            {
                highlight_canvas_net(state.model, |net| net.name.eq_ignore_ascii_case(&net_name));
            }
        } else {
            log::info!(
                "Probe: wire at {:?} has no unambiguous live or retained net identity",
                grid_pos
            );
            state.prompt("Nothing probed: this wire does not resolve to one net.");
        }
    } else {
        // A probe is always on a signal: a click on the bare sheet places
        // nothing, and the status line says what to click.
        let Some(ProbeSubject::Part(comp_id)) = subject else {
            state.prompt("Nothing probed: click a wire, a pin or a part to probe it.");
            return;
        };
        handle_component_probe(state, comp_id, grid_pos, symbol_context, read_only);
    }
}

type ProbeRequest = fn(&mut CanvasParts<'_>, &OccurrenceProbeSpelling) -> ProbeSignalOutcome;

/// What a probe at `place` measures, as a Probe-tool click there resolves
/// it: the net of the wire under it, the voltage at an exact pin, or the
/// current of the part under it. `None` when no signal is there.
fn probe_signal_at(
    state: &CanvasParts<'_>,
    place: Point,
    symbol_context: &SchematicSymbolContext,
) -> Option<ComponentProbe> {
    match probe_subject_at(state, place, symbol_context)? {
        ProbeSubject::Wire(wire_id) => live_wire_probe_net_name(state, wire_id)
            .or_else(|| retained_probe_net_name(state, place))
            .map(ComponentProbe::Voltage),
        ProbeSubject::Part(part) => component_probe_expression(state, part, place, symbol_context),
    }
}

/// Ask for what `probe` measures through `request`.
fn commit_signal(
    state: &mut CanvasParts<'_>,
    probe: &ComponentProbe,
    request: ProbeRequest,
) -> (OccurrenceProbeSpelling, ProbeSignalOutcome) {
    match probe {
        ComponentProbe::Voltage(net) => {
            commit_probe_request(state, net, &format!("V({net})"), request)
        }
        ComponentProbe::Current { drawn, engine } => {
            commit_element_current(state, drawn, engine, request)
        }
    }
}

/// The probe markers a floating Paste or Duplicate carries, and where each
/// lands if the copy is placed now.
///
/// The drawing pastes no probe: a copy keeping the original's expression
/// would measure the original's signal. Each marker is probed again where it
/// lands instead, as a Probe-tool click there would, and keeps no marker
/// where no signal is under it.
pub(super) struct PastedProbes {
    places: Vec<Point>,
    /// The copy holds nothing but these markers.
    alone: bool,
    /// "Pasted" or "Duplicated", for the line the placement answers with.
    verb: &'static str,
}

impl PastedProbes {
    /// The markers of the copy floating in `floating`, or `None` when it
    /// carries none or would be cancelled rather than placed.
    pub(super) fn of(
        floating: &Option<
            crate::workbench::app::dialogs::selection::commands::floating::FloatingPaste,
        >,
        model: &Model,
    ) -> Option<Self> {
        use crate::workbench::app::dialogs::selection::commands::floating::FloatingPasteOrigin;
        let copy = floating.as_ref()?;
        if copy.objects.probes.is_empty()
            || crate::workbench::app::floating_paste_scope(floating, model).is_err()
        {
            return None;
        }
        // The paste moves every object by its anchor less the copy's origin.
        let (at, origin) = (copy.at(), copy.objects.origin);
        let offset = Point::new(at.x.saturating_sub(origin.x), at.y.saturating_sub(origin.y));
        Some(Self {
            places: copy
                .objects
                .probes
                .iter()
                .map(|probe| {
                    Point::new(
                        probe.position.x.saturating_add(offset.x),
                        probe.position.y.saturating_add(offset.y),
                    )
                })
                .collect(),
            alone: copy.objects.count() == copy.objects.probes.len(),
            verb: match copy.origin {
                FloatingPasteOrigin::Paste => "Pasted",
                FloatingPasteOrigin::Duplicate { .. } => "Duplicated",
            },
        })
    }

    /// Probe each place once the rest of the copy has landed (`placed`),
    /// within the paste's own undo step. The markers kept join the paste's
    /// selection, and the status line adds how many found no signal.
    /// Returns whether any marker was kept.
    pub(super) fn land(self, state: &mut CanvasParts<'_>, placed: bool) -> bool {
        // A paste that failed lands none of its probes either.
        if !placed && !self.alone {
            return false;
        }
        let paste_line = state
            .workbench
            .prompt
            .as_ref()
            .map(|line| line.text.clone());
        let mut selection = state.model.schematic.session.editor.selection.clone();
        let focus = state.design.interaction.schematic_keyboard_focus;
        let symbol_context = state.model.schematic_symbol_context();
        let kept: Vec<u64> = self
            .places
            .iter()
            .filter_map(|place| probe_pasted_place(state, *place, &symbol_context))
            .collect();
        let missed = self.places.len() - kept.len();

        if self.alone {
            selection.clear();
        }
        for id in &kept {
            selection.select_probe(*id);
        }
        state.model.schematic.session.editor.selection = selection;
        state.design.interaction.schematic_keyboard_focus = focus;

        let left_out = |count: usize| match count {
            1 => "1 probe landed on no signal and was left out.".to_owned(),
            count => format!("{count} probes landed on no signal and were left out."),
        };
        let line = match (self.alone, kept.len(), missed) {
            (true, 0, _) => format!(
                "Nothing was {}: a probe goes on a wire, a pin or a part.",
                self.verb.to_lowercase()
            ),
            (true, 1, 0) => format!("{} 1 probe.", self.verb),
            (true, kept, 0) => format!("{} {kept} probes.", self.verb),
            (true, 1, missed) => format!("{} 1 probe. {}", self.verb, left_out(missed)),
            (true, kept, missed) => format!("{} {kept} probes. {}", self.verb, left_out(missed)),
            (false, _, 0) => paste_line.unwrap_or_default(),
            (false, _, missed) => match paste_line {
                Some(line) => format!("{line} {}", left_out(missed)),
                None => left_out(missed),
            },
        };
        if !line.is_empty() {
            state.prompt(line);
        }
        !kept.is_empty()
    }
}

/// Probe `place` for a pasted marker, as a Probe-tool click there does, and
/// keep the marker on the signal under it. `None` when no signal is there.
fn probe_pasted_place(
    state: &mut CanvasParts<'_>,
    place: Point,
    symbol_context: &SchematicSymbolContext,
) -> Option<u64> {
    let signal = probe_signal_at(state, place, symbol_context)?;
    let (spelling, outcome) = commit_signal(state, &signal, request_probe_signal);
    if matches!(
        outcome,
        ProbeSignalOutcome::Rejected { .. } | ProbeSignalOutcome::GroundReference
    ) {
        return None;
    }
    let binding = current_probe_output_binding(state, spelling.engine());
    retain_probe_flag(state, place, Some(&spelling), binding).ok()
}

/// How a probe asks for its signal: an editable view saves it for the next
/// run as well; a read-only view only plots what the last run holds.
fn probe_request(read_only: bool) -> ProbeRequest {
    if read_only {
        request_retained_probe_signal
    } else {
        request_probe_signal
    }
}

/// Report what a probe got and, on an editable view, keep its marker.
fn finish_probe(
    state: &mut CanvasParts<'_>,
    grid_pos: Point,
    spelling: &OccurrenceProbeSpelling,
    outcome: ProbeSignalOutcome,
    read_only: bool,
) {
    let retain_marker = !read_only
        && !matches!(
            &outcome,
            ProbeSignalOutcome::Rejected { .. } | ProbeSignalOutcome::GroundReference
        );
    report_probe_outcome(state, spelling.display(), outcome);
    if retain_marker {
        let binding = current_probe_output_binding(state, spelling.engine());
        if let Err(reason) = retain_probe_flag(state, grid_pos, Some(spelling), binding) {
            state.prompt(format!("Probe marker not kept: {reason}"));
        }
    }
}

fn handle_component_probe(
    state: &mut CanvasParts<'_>,
    comp_id: u64,
    grid_pos: Point,
    symbol_context: &SchematicSymbolContext,
    read_only: bool,
) {
    if let Some(component) = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|c| c.id == comp_id)
    {
        let comp_name = component.name.clone();
        log::info!(
            "Probe: clicked component '{}' ({})",
            comp_name,
            component.kind.display_name()
        );

        let Some(probe) = component_probe_expression(state, comp_id, grid_pos, symbol_context)
        else {
            state.prompt(format!(
                "{comp_name} has no single current there. Probe a terminal or a wire, or add the lead current in Saved outputs."
            ));
            return;
        };

        let (spelling, outcome) = commit_signal(state, &probe, probe_request(read_only));
        finish_probe(state, grid_pos, &spelling, outcome, read_only);
    }
}

/// A double-click on a wire or a junction shows its net: every conductor of
/// the net is selected and lit, as a Navigator net row selects it, and the
/// inspector opens on the net where it was closed. The net is the one the
/// deck gives the conductor, so two wire groups a label joins show as one.
fn show_conductor_net(state: &mut CanvasParts<'_>, target: PointerTarget) {
    let document = state.model.schematic.document();
    let wire_ids: Vec<u64> = match target {
        PointerTarget::Wire(id) => vec![id],
        PointerTarget::Junction(at) => document
            .wires
            .iter()
            .filter(|wire| wire.contains_point(at))
            .map(|wire| wire.id)
            .collect(),
        _ => return,
    };
    let nets = live_design_nets(state.model);
    let Some(net) = nets
        .iter()
        .find(|net| net.wire_ids.iter().any(|id| wire_ids.contains(id)))
    else {
        state.prompt("The design resolves no net for that wire.");
        return;
    };
    let editor = &mut state.model.schematic.session.editor;
    editor.selection.clear();
    for wire in &net.wire_ids {
        editor.selection.select_wire(*wire);
    }
    editor
        .net_highlight
        .highlight_named_wires(&net.name, net.wire_ids.iter().copied().collect());
    state.workbench.inspector_visible = true;
    state.workbench.drawer = Some(crate::workbench::state::Drawer::Inspector);
    state.prompt(format!("Net {} is in the inspector.", net.name));
}

fn open_object_properties(
    state: &mut CanvasParts<'_>,
    hit: PointerHit,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
    ctx: &egui::Context,
    viewport: &Viewport,
    pointer_pos: egui::Pos2,
) {
    let target = pointer_target(
        state,
        hit,
        hit_radius,
        symbol_context,
        ctx,
        viewport,
        pointer_pos,
    );
    let selection = &mut state.model.schematic.session.editor.selection;
    match target {
        Some(PointerTarget::Component(id)) => selection.select_only_component(id),
        Some(PointerTarget::DesignNote(id)) => selection.select_only_design_note(id),
        Some(PointerTarget::DocumentationShape(id)) => {
            selection.select_only_documentation_shape(id)
        }
        Some(PointerTarget::Probe(id)) => selection.select_only_probe(id),
        Some(PointerTarget::NetLabel(id)) => selection.select_only_net_label(id),
        Some(PointerTarget::BusTap(id)) => selection.select_only_bus_tap(id),
        Some(PointerTarget::Bus(id)) => selection.select_only_bus(id),
        Some(PointerTarget::Junction(_)) | Some(PointerTarget::Wire(_)) | None => return,
    }
    crate::workbench::app::open_selected_object_properties_in(
        &mut state.design.property_form_parts(),
        state.model,
        state.workbench,
    );
}

#[cfg(test)]
mod tests;
