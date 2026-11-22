//! Bind editor drag intents to the app's checked transaction owner.

use super::{
    SchematicSymbolContext, SelectionWindow,
    navigation::primary_pan_gesture_active,
    parts::CanvasParts,
    requests::editor_request_source,
    sheet_visibility::{select_in_rect_on_active_sheet, with_active_wire_topology},
    viewport::Viewport,
};
use crate::state::DragRubberBand;
use crate::state::Point;
use crate::workbench::app_state::DragType;
use rspice_design::schematic::label_position::InstanceText;
use rspice_schematic_editor::view::instance_text::{self, InstanceTextHit};
use rspice_schematic_editor::view::pointer_target::PointerTarget;
use rspice_schematic_editor::view::selection_drag::{
    self, SelectionDragAction, SelectionDragRequest, SelectionDragView, select_drag_can_start,
    select_drag_is_authorized, select_drag_target, wire_segment_drag_motion,
};

fn view<'a>(state: &'a CanvasParts<'_>, ctx: &egui::Context) -> SelectionDragView<'a> {
    SelectionDragView {
        design: state.design_view(),
        editor: &state.model.schematic.session.editor,
        filter: state.design.schematic_selection_filter,
        drag: state.schematic_drag_snapshot(ctx),
    }
}

pub(super) fn handle_select_dragging(
    ui: &egui::Ui,
    response: &egui::Response,
    state: &mut CanvasParts<'_>,
    viewport: &Viewport,
    symbols: &SchematicSymbolContext,
) {
    if !select_drag_is_authorized(
        state.model.schematic.session.editor.tool,
        state.design.move_selection.armed,
    ) {
        return;
    }
    state.design.interaction.hover_wire_vertex =
        selection_drag::hover_wire_vertex(response, &view(state, ui.ctx()), viewport, symbols)
            .map(|point| (point.x, point.y));
    if response.is_pointer_button_down_on()
        && ui.input(|input| input.pointer.primary_pressed() && input.pointer.primary_down())
    {
        state.prepare_schematic_drag();
    }
    if response.drag_started_by(egui::PointerButton::Primary)
        && !select_drag_can_start(primary_pan_gesture_active(ui, response))
    {
        return;
    }
    let action = selection_drag::start(
        ui,
        response,
        &view(state, ui.ctx()),
        viewport,
        symbols,
        state.schematic_edit_read_only(),
        || state.model.sheet_text_variables(),
    );
    if let Some(action) = action {
        dispatch(state, ui.ctx(), symbols, action);
    }
    if let Some(action) = selection_drag::update(response, &view(state, ui.ctx()), viewport) {
        dispatch(state, ui.ctx(), symbols, action);
    }
    if let Some(action) = selection_drag::finish(ui, response, &view(state, ui.ctx())) {
        dispatch(state, ui.ctx(), symbols, action);
    }
}

pub fn capture(state: &CanvasParts<'_>, action: SelectionDragAction) -> SelectionDragRequest {
    SelectionDragRequest {
        source: editor_request_source(state),
        selection: state.model.schematic.session.editor.selection.clone(),
        rectangle: state.model.schematic.session.editor.selection_rect,
        action,
    }
}

pub fn dispatch(
    state: &mut CanvasParts<'_>,
    ctx: &egui::Context,
    symbols: &SchematicSymbolContext,
    action: SelectionDragAction,
) {
    let request = capture(state, action);
    apply(state, ctx, symbols, request);
}

pub fn apply(
    state: &mut CanvasParts<'_>,
    ctx: &egui::Context,
    symbols: &SchematicSymbolContext,
    request: SelectionDragRequest,
) {
    if request.source != editor_request_source(state)
        || request.selection != state.model.schematic.session.editor.selection
        || request.rectangle != state.model.schematic.session.editor.selection_rect
        || !select_drag_is_authorized(
            state.model.schematic.session.editor.tool,
            state.design.move_selection.armed,
        )
    {
        return;
    }
    match request.action {
        // A read-only view selects what the drag began on and says why it
        // does not move, as a drawing tool there does.
        SelectionDragAction::BeginSelection { target, position } => {
            select_drag_target(&mut state.model.schematic.session.editor.selection, target);
            if state.deny_read_only_edit() {
                return;
            }
            state.begin_schematic_drag((position.x, position.y), DragType::MoveSelection, ctx);
        }
        // The text's part is selected, as a click on the text selects it; the
        // drag moves the text alone.
        SelectionDragAction::BeginText { hit, position } => {
            select_drag_target(
                &mut state.model.schematic.session.editor.selection,
                PointerTarget::Component(hit.component),
            );
            if state.deny_read_only_edit() {
                return;
            }
            state.begin_schematic_drag(
                (position.x, position.y),
                DragType::MoveText(hit.component, hit.text),
                ctx,
            );
        }
        SelectionDragAction::BeginWireVertex(position) => {
            if state.deny_read_only_edit() {
                return;
            }
            state.begin_schematic_drag((position.x, position.y), DragType::WireVertex, ctx);
        }
        // The wire is taken alone, as a click on it takes it; the drawing
        // changes on release, when the segment lands.
        SelectionDragAction::BeginWireSegment {
            wire_id,
            segment_index,
            position,
        } => {
            state
                .model
                .schematic
                .session
                .editor
                .selection
                .select_only_wire(wire_id);
            if state.deny_read_only_edit() {
                return;
            }
            state.begin_schematic_drag(
                (position.x, position.y),
                DragType::WireSegment(wire_id, segment_index),
                ctx,
            );
        }
        SelectionDragAction::BeginMarquee(position) => state
            .model
            .schematic
            .session
            .editor
            .selection_rect
            .start_at(position),
        SelectionDragAction::BeginTouchPan => {
            selection_drag::set_touch_pan(ctx, true);
            // The finger crossed the drag threshold before the pan began;
            // the sheet catches up with it now, and follows it from here.
            if let Some(travel) = ctx
                .input(|input| Some(input.pointer.interact_pos()? - input.pointer.press_origin()?))
            {
                let pan = &mut state.model.schematic.session.editor.pan;
                pan.0 += f64::from(travel.x);
                pan.1 += f64::from(travel.y);
            }
        }
        SelectionDragAction::UpdateMarquee(position) => state
            .model
            .schematic
            .session
            .editor
            .selection_rect
            .update(position),
        SelectionDragAction::MoveWireVertex {
            operation_id,
            from,
            position,
        } => {
            if !owns_operation(state, ctx, operation_id, Some(DragType::WireVertex))
                || state.design.interaction.drag.last_pos != Some((from.x, from.y))
            {
                return;
            }
            if with_active_wire_topology(state.model, |schematic, scope| {
                schematic.move_all_vertices_at(from, position, scope)
            }) {
                state
                    .design
                    .interaction
                    .drag
                    .update((position.x, position.y));
            }
        }
        SelectionDragAction::MoveWireSegment {
            operation_id,
            from,
            position,
        } => {
            if !owns_operation(state, ctx, operation_id, None)
                || !matches!(
                    state.design.interaction.drag.drag_type,
                    DragType::WireSegment(..)
                )
                || state.design.interaction.drag.last_pos != Some((from.x, from.y))
            {
                return;
            }
            state
                .design
                .interaction
                .drag
                .update((position.x, position.y));
        }
        SelectionDragAction::MoveText {
            operation_id,
            hit,
            from,
            delta,
            position,
        } => {
            if !owns_operation(
                state,
                ctx,
                operation_id,
                Some(DragType::MoveText(hit.component, hit.text)),
            ) || state.design.interaction.drag.last_pos != Some((from.x, from.y))
            {
                return;
            }
            if move_instance_text(state, symbols, hit, delta) {
                state
                    .design
                    .interaction
                    .drag
                    .update((position.x, position.y));
            }
        }
        SelectionDragAction::MoveSelection {
            operation_id,
            from,
            delta,
            position,
        } => {
            if !owns_operation(state, ctx, operation_id, Some(DragType::MoveSelection))
                || state.design.interaction.drag.last_pos != Some((from.x, from.y))
            {
                return;
            }
            // The rubber band follows the routing style: square unless the
            // user draws at 45° or free angles.
            let rubber_band = if state
                .model
                .schematic
                .session
                .editor
                .wire_drawing
                .routing_mode
                .is_orthogonal()
            {
                DragRubberBand::Orthogonal
            } else {
                DragRubberBand::Direct
            };
            with_active_wire_topology(state.model, |schematic, scope| {
                schematic.move_selection_with_rubber_band_resolved(
                    delta,
                    rubber_band,
                    |component| symbols.terminal_points(component),
                    scope,
                )
            });
            state
                .design
                .interaction
                .drag
                .update((position.x, position.y));
        }
        SelectionDragAction::Finish { operation_id } => {
            if !owns_operation(state, ctx, operation_id, None) {
                return;
            }
            if let DragType::WireSegment(wire_id, segment_index) =
                state.design.interaction.drag.drag_type
            {
                land_wire_segment(state, symbols, wire_id, segment_index);
                state.design.interaction.drag.cancel();
                return;
            }
            // A moved name or value leaves every pin and wire where it was.
            if !matches!(
                state.design.interaction.drag.drag_type,
                DragType::MoveText(..)
            ) {
                let automatic_junctions = state.model.schematic.session.editor.automatic_junctions;
                let moved = state
                    .model
                    .schematic
                    .session
                    .editor
                    .selection
                    .components
                    .clone();
                with_active_wire_topology(state.model, |schematic, scope| {
                    // A pin dropped inside a wire is connected there; the vertex
                    // it gets is what lets the next move carry the wire. A pin
                    // dragged off a pin it touched keeps the connection with a
                    // wire.
                    schematic.connect_moved_pins(
                        &moved,
                        |component| symbols.terminal_points(component),
                        scope,
                    );
                    schematic.cleanup_wire_topology_with_junction_policy(automatic_junctions, scope)
                });
            }
            // One undo entry for the whole gesture (no-ops deduplicate).
            if state.model.schematic.end_operation() {
                state.sync_active_schematic_to_workspace();
            }
            state.design.interaction.drag.cancel();
        }
        SelectionDragAction::FinishMarquee { additive } => {
            let rectangle = &mut state.model.schematic.session.editor.selection_rect;
            let (start, current) = (rectangle.start, rectangle.current);
            if rectangle.finish().is_none() {
                return;
            }
            // The window the live preview drew from, so release selects what
            // the drag showed selected.
            let window = SelectionWindow::from_band(
                start,
                current,
                state.model.schematic.session.editor.selection_crossing,
            );
            select_in_rect_on_active_sheet(state, symbols, window, additive);
        }
    }
}

/// Where a segment drag's pointer went, from where it began, as the motion
/// and policy the stretch is asked for; `None` before the pointer has moved.
pub(super) fn wire_segment_drag(
    state: &CanvasParts<'_>,
    wire_id: u64,
    segment_index: usize,
) -> Option<(
    Point,
    crate::state::StretchTarget,
    crate::state::StretchOrthogonalPolicy,
)> {
    let drag = &state.design.interaction.drag;
    let (start, last) = (drag.start_pos?, drag.last_pos?);
    Some(wire_segment_drag_motion(
        state.model.schematic.document(),
        wire_id,
        segment_index,
        Point::new(start.0, start.1),
        Point::new(last.0, last.1),
        state
            .model
            .schematic
            .session
            .editor
            .wire_drawing
            .routing_mode
            .is_orthogonal(),
    ))
}

/// Release a segment drag: the segment lands where the preview showed it,
/// with the wires at its ends following, as one undo step. A drop the
/// drawing cannot take leaves every wire as drawn and says why.
fn land_wire_segment(
    state: &mut CanvasParts<'_>,
    symbols: &SchematicSymbolContext,
    wire_id: u64,
    segment_index: usize,
) {
    let Some((delta, target, policy)) = wire_segment_drag(state, wire_id, segment_index)
        .filter(|(delta, ..)| *delta != Point::origin())
    else {
        state.model.schematic.cancel_operation();
        return;
    };
    let landed = with_active_wire_topology(state.model, |schematic, scope| {
        schematic.stretch_target_resolved(
            delta,
            target,
            policy,
            |component| symbols.terminal_points(component),
            |component| symbols.component_bounds_tuple(component),
            scope,
        )
    });
    match landed {
        Ok(_) => {
            if state.model.schematic.end_operation() {
                state.sync_active_schematic_to_workspace();
            }
        }
        Err(error) => {
            state.model.schematic.cancel_operation();
            state.prompt(format!("Wire not moved: {error}"));
        }
    }
}

/// Move one part's name or value by `delta` inside the drag's operation: the
/// text starts from where the canvas lettered it, and the part stays put.
fn move_instance_text(
    state: &mut CanvasParts<'_>,
    symbols: &SchematicSymbolContext,
    hit: InstanceTextHit,
    delta: Point,
) -> bool {
    let Some(part) = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|part| part.id == hit.component)
        .cloned()
    else {
        return false;
    };
    let placed = instance_text::placement(&part, symbols, hit.text);
    let mut moved = part.clone();
    *hit.text.position_mut(&mut moved) = hit.text.position(&part).moved(placed, part.pos, delta);
    let description = match hit.text {
        InstanceText::Name => "move name",
        InstanceText::Value => "move value",
    };
    crate::state::bulk_edit::replace_components(
        &mut state.model.schematic,
        vec![(part, moved)],
        description,
    )
    .is_ok()
}

fn owns_operation(
    state: &CanvasParts<'_>,
    ctx: &egui::Context,
    operation_id: u64,
    kind: Option<DragType>,
) -> bool {
    !state.schematic_edit_read_only()
        && state.schematic_drag_snapshot(ctx).is_some_and(|drag| {
            drag.operation_id == operation_id && kind.is_none_or(|kind| drag.kind == kind)
        })
}
