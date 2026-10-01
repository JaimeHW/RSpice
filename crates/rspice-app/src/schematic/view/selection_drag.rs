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
            if with_active_wire_topology(state, |schematic| {
                schematic.move_all_vertices_at(from, position)
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
            with_active_wire_topology(state, |schematic| {
                schematic.move_selection_with_rubber_band_resolved(delta, |component| {
                    symbols.terminal_points(component)
                })
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
            with_active_wire_topology(state, |schematic| {
                schematic.cleanup_wire_topology_with_junction_policy(automatic_junctions)
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
