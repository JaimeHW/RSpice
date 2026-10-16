//! Bind editor drag intents to the app's checked transaction owner.

use super::{
    SchematicSymbolContext, SelectionWindow,
    navigation::primary_pan_gesture_active,
    requests::editor_request_source,
    sheet_visibility::{select_in_rect_on_active_sheet, with_active_wire_topology},
    viewport::Viewport,
};
use crate::workbench::app_state::{AppState, DragType};
use rspice_schematic_editor::view::selection_drag::{
    self, SelectionDragAction, SelectionDragRequest, SelectionDragView, select_drag_can_start,
    select_drag_is_authorized, select_drag_target,
};

fn view<'a>(state: &'a AppState, ctx: &egui::Context) -> SelectionDragView<'a> {
    SelectionDragView {
        design: super::schematic_design_view(state),
        editor: &state.schematic.session.editor,
        filter: state.ui.schematic_selection_filter,
        drag: state.schematic_drag_snapshot(ctx),
    }
}

pub(super) fn handle_select_dragging(
    ui: &egui::Ui,
    response: &egui::Response,
    state: &mut AppState,
    viewport: &Viewport,
    symbols: &SchematicSymbolContext,
) {
    if !select_drag_is_authorized(
        state.schematic.session.editor.tool,
        state.dialogs.move_selection.armed,
    ) {
        return;
    }
    state.dialogs.interaction.hover_wire_vertex =
        selection_drag::hover_wire_vertex(response, &view(state, ui.ctx()), viewport, symbols)
            .map(|point| (point.x, point.y));
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
        || state.workspace.content.active_view.display_path(),
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

fn capture(state: &AppState, action: SelectionDragAction) -> SelectionDragRequest {
    SelectionDragRequest {
        source: editor_request_source(state),
        selection: state.schematic.session.editor.selection.clone(),
        rectangle: state.schematic.session.editor.selection_rect,
        action,
    }
}

fn dispatch(
    state: &mut AppState,
    ctx: &egui::Context,
    symbols: &SchematicSymbolContext,
    action: SelectionDragAction,
) {
    let request = capture(state, action);
    apply(state, ctx, symbols, request);
}

fn apply(
    state: &mut AppState,
    ctx: &egui::Context,
    symbols: &SchematicSymbolContext,
    request: SelectionDragRequest,
) {
    if request.source != editor_request_source(state)
        || request.selection != state.schematic.session.editor.selection
        || request.rectangle != state.schematic.session.editor.selection_rect
        || !select_drag_is_authorized(
            state.schematic.session.editor.tool,
            state.dialogs.move_selection.armed,
        )
    {
        return;
    }
    match request.action {
        SelectionDragAction::BeginSelection { target, position } => {
            if state.schematic_edit_read_only() {
                return;
            }
            select_drag_target(&mut state.schematic.session.editor.selection, target);
            state.begin_schematic_drag((position.x, position.y), DragType::MoveSelection, ctx);
        }
        SelectionDragAction::BeginWireVertex(position) => {
            if state.schematic_edit_read_only() {
                return;
            }
            state.begin_schematic_drag((position.x, position.y), DragType::WireVertex, ctx);
        }
        SelectionDragAction::BeginMarquee(position) => state
            .schematic
            .session
            .editor
            .selection_rect
            .start_at(position),
        SelectionDragAction::UpdateMarquee(position) => state
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
                || state.dialogs.interaction.drag.last_pos != Some((from.x, from.y))
            {
                return;
            }
            if with_active_wire_topology(state, |schematic, scope| {
                schematic.move_all_vertices_at(from, position, scope)
            }) {
                state
                    .dialogs
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
                || state.dialogs.interaction.drag.last_pos != Some((from.x, from.y))
            {
                return;
            }
            with_active_wire_topology(state, |schematic, scope| {
                schematic.move_selection_with_rubber_band_resolved(
                    delta,
                    |component| symbols.terminal_points(component),
                    scope,
                )
            });
            state
                .dialogs
                .interaction
                .drag
                .update((position.x, position.y));
        }
        SelectionDragAction::Finish { operation_id } => {
            if !owns_operation(state, ctx, operation_id, None) {
                return;
            }
            let automatic_junctions = state
                .schematic
                .document()
                .document_policy
                .wire_junctions
                .automatic_junctions();
            let moved = state.schematic.session.editor.selection.components.clone();
            with_active_wire_topology(state, |schematic, scope| {
                // A pin dropped inside a wire is connected there; the vertex
                // it gets is what lets the next move carry the wire.
                schematic.split_wires_under_pins(
                    &moved,
                    |component| symbols.terminal_points(component),
                    scope,
                );
                schematic.cleanup_wire_topology_with_junction_policy(automatic_junctions, scope)
            });
            // One undo entry for the whole gesture (no-ops deduplicate).
            if state.schematic.end_operation() {
                state.sync_active_schematic_to_workspace();
            }
            state.dialogs.interaction.drag.cancel();
        }
        SelectionDragAction::FinishMarquee { additive } => {
            let rectangle = &mut state.schematic.session.editor.selection_rect;
            let left_to_right = rectangle.current.x >= rectangle.start.x;
            let Some((min_x, min_y, max_x, max_y)) = rectangle.finish() else {
                return;
            };
            let enclosed_only = state
                .schematic
                .document()
                .document_policy
                .selection_crossing
                .enclosed_only(left_to_right);
            select_in_rect_on_active_sheet(
                state,
                symbols,
                SelectionWindow::new(min_x, min_y, max_x, max_y, enclosed_only),
                additive,
            );
        }
    }
}

fn owns_operation(
    state: &AppState,
    ctx: &egui::Context,
    operation_id: u64,
    kind: Option<DragType>,
) -> bool {
    !state.schematic_edit_read_only()
        && state.schematic_drag_snapshot(ctx).is_some_and(|drag| {
            drag.operation_id == operation_id && kind.is_none_or(|kind| drag.kind == kind)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{CellViewRef, ComponentType, Point, ViewType};
    use rspice_schematic_editor::view::pointer_target::PointerTarget;

    /// A vertex drag follows the pointer for the whole gesture, including over
    /// a point that carries a junction and no wire vertex.
    ///
    /// The anchor only advances when the edit reports a move, so a point whose
    /// move went unreported moved once and was then dragged from an anchor it
    /// had already left: every later frame matched nothing and the object sat
    /// still under a moving pointer for as long as the button was held.
    #[test]
    fn a_vertex_drag_over_a_lone_junction_follows_the_pointer_every_frame() {
        let ctx = egui::Context::default();
        let mut state = AppState::default();
        let junction = Point::new(100, 100);
        state.schematic.add_junction(junction);
        // No component takes part, so one context serves the whole gesture.
        let symbols = super::super::schematic_symbol_context(&state);

        dispatch(
            &mut state,
            &ctx,
            &symbols,
            SelectionDragAction::BeginWireVertex(junction),
        );
        let operation_id = state
            .schematic
            .pending_operation_id()
            .expect("the gesture owns a transaction");

        let mut from = junction;
        for step in 1..=3 {
            let position = Point::new(junction.x, junction.y - step * 10);
            dispatch(
                &mut state,
                &ctx,
                &symbols,
                SelectionDragAction::MoveWireVertex {
                    operation_id,
                    from,
                    position,
                },
            );
            assert_eq!(
                state.schematic.document().junctions[0].pos,
                position,
                "step {step} left the junction behind the pointer"
            );
            assert_eq!(
                state.dialogs.interaction.drag.last_pos,
                Some((position.x, position.y)),
                "step {step} left the drag anchored where the junction no longer is"
            );
            from = position;
        }
    }

    /// Drag the selected part by `delta` as one whole gesture.
    fn drag(
        state: &mut AppState,
        ctx: &egui::Context,
        symbols: &crate::schematic::view::SchematicSymbolContext,
        target: PointerTarget,
        from: Point,
        delta: Point,
    ) {
        dispatch(
            state,
            ctx,
            symbols,
            SelectionDragAction::BeginSelection {
                target,
                position: from,
            },
        );
        let operation_id = state.schematic.pending_operation_id().expect("a gesture");
        let position = Point::new(from.x + delta.x, from.y + delta.y);
        dispatch(
            state,
            ctx,
            symbols,
            SelectionDragAction::MoveSelection {
                operation_id,
                from,
                delta,
                position,
            },
        );
        dispatch(
            state,
            ctx,
            symbols,
            SelectionDragAction::Finish { operation_id },
        );
    }

    /// A part dragged until its pin lies inside a wire is connected there,
    /// so the wire splits and the pin owns the vertex: the next drag carries
    /// the wire instead of silently leaving it behind.
    #[test]
    fn a_pin_dragged_into_a_wire_splits_it_and_the_next_drag_carries_it() {
        let ctx = egui::Context::default();
        let mut state = AppState::default();
        let id = state
            .schematic
            .add_component(ComponentType::Resistor, Point::origin());
        let pin = state.schematic.document().components[0].terminal_positions()[1].1;
        let landing = Point::new(pin.x + 20, pin.y);
        state
            .schematic
            .document_mut_for_test()
            .wires
            .push(crate::state::Wire::segment(
                9,
                Point::new(landing.x, landing.y - 100),
                Point::new(landing.x, landing.y + 100),
            ));
        state.schematic.init_undo_history();
        let symbols = super::super::schematic_symbol_context(&state);
        let target = PointerTarget::Component(id);

        drag(&mut state, &ctx, &symbols, target, Point::origin(), Point::new(20, 0));
        let at_pin = |state: &AppState, point: Point| {
            state
                .schematic
                .document()
                .wires
                .iter()
                .filter(|wire| wire.points.contains(&point))
                .count()
        };
        assert_eq!(at_pin(&state, landing), 2, "the wire was split at the pin");

        drag(
            &mut state,
            &ctx,
            &symbols,
            target,
            Point::new(20, 0),
            Point::new(0, 20),
        );
        assert_eq!(
            at_pin(&state, Point::new(landing.x, landing.y + 20)),
            2,
            "both halves followed the pin"
        );
    }

    #[test]
    fn queued_drag_requests_reject_changed_sources_and_replacement_transactions() {
        let ctx = egui::Context::default();
        for change in 0..4 {
            let mut state = AppState::default();
            let id = state
                .schematic
                .add_component(ComponentType::Resistor, Point::new(100, 100));
            let master = CellViewRef::new("user", "amp", "schematic");
            state
                .workspace
                .descend_into("X1".into(), master.clone(), ViewType::Schematic);
            let symbols = super::super::schematic_symbol_context(&state);
            let action = SelectionDragAction::BeginSelection {
                target: PointerTarget::Component(id),
                position: Point::new(100, 100),
            };
            let request = capture(&state, action);
            match change {
                0 => {
                    state.workspace.ascend_one().unwrap();
                    state
                        .workspace
                        .descend_into("X2".into(), master, ViewType::Schematic);
                }
                1 => {
                    state
                        .schematic
                        .add_component(ComponentType::Resistor, Point::origin());
                }
                2 => state
                    .schematic
                    .session
                    .editor
                    .selection_rect
                    .start_at(Point::origin()),
                _ => state.schematic.session.read_only = true,
            }
            let before = state.schematic.document().components.clone();
            apply(&mut state, &ctx, &symbols, request);
            assert!(!state.schematic.has_pending_operation(), "change {change}");
            assert_eq!(state.schematic.document().components, before);
            assert!(state.schematic.session.editor.selection.is_empty());

            state.schematic.session.read_only = false;
            dispatch(&mut state, &ctx, &symbols, action);
            let operation_id = state.schematic.pending_operation_id().expect("fresh begin");
            state.cancel_schematic_drag();
            assert!(state.begin_schematic_drag((100, 100), DragType::MoveSelection, &ctx));
            let replacement = state.schematic.pending_operation_id();
            assert_ne!(replacement, Some(operation_id));
            // Capture current source revisions to isolate the operation-token guard.
            dispatch(
                &mut state,
                &ctx,
                &symbols,
                SelectionDragAction::MoveSelection {
                    operation_id,
                    from: Point::new(100, 100),
                    delta: Point::new(10, 0),
                    position: Point::new(110, 100),
                },
            );
            dispatch(
                &mut state,
                &ctx,
                &symbols,
                SelectionDragAction::Finish { operation_id },
            );
            assert_eq!(state.schematic.document().components, before);
            assert_eq!(state.schematic.pending_operation_id(), replacement);
            state.cancel_schematic_drag();
        }
    }
}
