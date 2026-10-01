//! Selection gesture interpretation over borrowed design and session inputs.

use super::{
    coordinates::screen_to_schematic,
    design_view::DesignView,
    pointer_target::{PointerHit, PointerQuery, PointerTarget, pointer_target},
    snap_resolution::{resolve_grid_pointer, resolve_target_pointer, target_acquisition_radius},
    symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};
use crate::{
    requests::EditorRequestSource,
    session::{
        EditorSession,
        drag::DragType,
        selection::{SchematicSelectionFilter, SelectionRect},
        tool::Tool,
    },
};
use egui::{PointerButton, Response, Ui};
use rspice_design::schematic::selection::Selection;
use rspice_design_model::Point;

/// A host-validated transaction owned by this canvas context.
#[derive(Debug, Clone, Copy)]
pub struct ActiveSelectionDrag {
    pub kind: DragType,
    pub last_position: Option<Point>,
    pub operation_id: u64,
}

pub struct SelectionDragView<'a> {
    pub design: DesignView<'a>,
    pub editor: &'a EditorSession,
    pub filter: SchematicSelectionFilter,
    pub drag: Option<ActiveSelectionDrag>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionDragAction {
    BeginSelection {
        target: PointerTarget,
        position: Point,
    },
    BeginWireVertex(Point),
    BeginMarquee(Point),
    MoveSelection {
        operation_id: u64,
        from: Point,
        delta: Point,
        position: Point,
    },
    MoveWireVertex {
        operation_id: u64,
        from: Point,
        position: Point,
    },
    UpdateMarquee(Point),
    Finish {
        operation_id: u64,
    },
    FinishMarquee {
        additive: bool,
    },
}

#[derive(Debug, Clone)]
pub struct SelectionDragRequest {
    pub source: EditorRequestSource,
    pub selection: Selection,
    pub rectangle: SelectionRect,
    pub action: SelectionDragAction,
}

pub fn select_drag_can_start(primary_pan_requested: bool) -> bool {
    !primary_pan_requested
}

pub fn select_drag_is_authorized(tool: Tool, move_selection_armed: bool) -> bool {
    tool == Tool::Select && !move_selection_armed
}

pub fn hover_wire_vertex(
    response: &Response,
    view: &SelectionDragView<'_>,
    viewport: &Viewport,
    symbols: &SchematicSymbolContext,
) -> Option<Point> {
    if !view.filter.wires {
        return None;
    }
    let position = resolve_target_pointer(
        &view.design,
        &view.editor.snap_engine,
        symbols,
        viewport,
        response.hover_pos()?,
    )
    .snapped_position;
    view.design
        .active_wire_point_is_draggable(position)
        .then_some(position)
}

pub fn start(
    ui: &Ui,
    response: &Response,
    view: &SelectionDragView<'_>,
    viewport: &Viewport,
    symbols: &SchematicSymbolContext,
    read_only: bool,
    view_path: impl FnOnce() -> String,
) -> Option<SelectionDragAction> {
    if !response.drag_started_by(PointerButton::Primary) {
        return None;
    }
    let position = ui.input(|input| input.pointer.press_origin())?;
    let grid = resolve_grid_pointer(
        &view.editor.snap_engine,
        view.design.document.grid_size,
        viewport,
        position,
    )
    .snapped_position;
    let wire = resolve_target_pointer(
        &view.design,
        &view.editor.snap_engine,
        symbols,
        viewport,
        position,
    )
    .snapped_position;
    let target = pointer_target(
        &view.design,
        PointerQuery {
            hit: PointerHit::new(grid, screen_to_schematic(viewport, position)),
            radius: target_acquisition_radius(viewport),
            position,
        },
        view.filter,
        symbols,
        ui.ctx(),
        viewport,
        view_path,
    );
    if read_only {
        return Some(SelectionDragAction::BeginMarquee(grid));
    }
    Some(match target {
        Some(
            target @ (PointerTarget::Component(_)
            | PointerTarget::DesignNote(_)
            | PointerTarget::DocumentationShape(_)
            | PointerTarget::Probe(_)
            | PointerTarget::NetLabel(_)
            | PointerTarget::BusTap(_)
            | PointerTarget::Bus(_)),
        ) => SelectionDragAction::BeginSelection {
            target,
            position: grid,
        },
        Some(PointerTarget::Junction(_) | PointerTarget::Wire(_))
            if view.filter.wires && view.design.active_wire_point_is_draggable(wire) =>
        {
            SelectionDragAction::BeginWireVertex(wire)
        }
        _ => SelectionDragAction::BeginMarquee(grid),
    })
}

pub fn update(
    response: &Response,
    view: &SelectionDragView<'_>,
    viewport: &Viewport,
) -> Option<SelectionDragAction> {
    if !response.dragged_by(PointerButton::Primary) {
        return None;
    }
    let position = resolve_grid_pointer(
        &view.editor.snap_engine,
        view.design.document.grid_size,
        viewport,
        response.hover_pos()?,
    )
    .snapped_position;
    match view.drag {
        Some(ActiveSelectionDrag {
            kind: DragType::WireVertex,
            last_position: Some(from),
            operation_id,
        }) => Some(SelectionDragAction::MoveWireVertex {
            operation_id,
            from,
            position,
        }),
        Some(ActiveSelectionDrag {
            kind: DragType::MoveSelection,
            last_position: Some(last),
            operation_id,
        }) => {
            let delta = Point::new(
                position.x.saturating_sub(last.x),
                position.y.saturating_sub(last.y),
            );
            (delta.x != 0 || delta.y != 0).then_some(SelectionDragAction::MoveSelection {
                operation_id,
                from: last,
                delta,
                position,
            })
        }
        _ if view.editor.selection_rect.is_active() => {
            Some(SelectionDragAction::UpdateMarquee(position))
        }
        _ => None,
    }
}

pub fn finish(
    ui: &Ui,
    response: &Response,
    view: &SelectionDragView<'_>,
) -> Option<SelectionDragAction> {
    if !response.drag_stopped_by(PointerButton::Primary) {
        return None;
    }
    Some(match view.drag {
        Some(drag) => SelectionDragAction::Finish {
            operation_id: drag.operation_id,
        },
        None => SelectionDragAction::FinishMarquee {
            additive: ui.input(|i| i.modifiers.ctrl || i.modifiers.shift || i.modifiers.command),
        },
    })
}

/// Preserve an existing group selection when its member starts a drag.
pub fn select_drag_target(selection: &mut Selection, target: PointerTarget) {
    match target {
        PointerTarget::Component(id) if !selection.has_component(id) => {
            selection.clear();
            selection.select_component(id);
        }
        PointerTarget::DesignNote(id) if !selection.has_design_note(id) => {
            selection.select_only_design_note(id)
        }
        PointerTarget::DocumentationShape(id) if !selection.has_documentation_shape(id) => {
            selection.select_only_documentation_shape(id)
        }
        PointerTarget::Probe(id) if !selection.has_probe(id) => selection.select_only_probe(id),
        PointerTarget::NetLabel(id) if !selection.has_net_label(id) => {
            selection.select_only_net_label(id)
        }
        PointerTarget::BusTap(id) if !selection.has_bus_tap(id) => {
            selection.select_only_bus_tap(id)
        }
        PointerTarget::Bus(id) if !selection.has_bus(id) => selection.select_only_bus(id),
        _ => {}
    }
}
