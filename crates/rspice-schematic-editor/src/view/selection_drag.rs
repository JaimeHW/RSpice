//! Selection gesture interpretation over borrowed design and session inputs.

use super::{
    coordinates::screen_to_schematic,
    design_view::DesignView,
    mobile_controls::touch_latches,
    pointer_target::{PointerHit, PointerQuery, PointerTarget, canvas_touch_mode, pointer_target},
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
    /// A finger's drag that pans the sheet: it began on empty canvas or on
    /// an object outside the selection (R-3). The view follows the finger
    /// until it lifts; see [`touch_pan_active`].
    BeginTouchPan,
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
    // Where a wire vertex is picked up. Only a press on a wire or a junction
    // reads it, and resolving it scans every pin on the sheet for the snap,
    // so a part's drag does not pay for it.
    let wire = || {
        resolve_target_pointer(
            &view.design,
            &view.editor.snap_engine,
            symbols,
            viewport,
            position,
        )
        .snapped_position
    };
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
    // The rubber band follows the pointer, not the grid.
    let band = screen_to_schematic(viewport, position);
    if canvas_touch_mode(ui.ctx()) {
        let start = TouchDragStart {
            grid,
            wire: wire(),
            band,
            read_only,
            box_latched: touch_latches(ui.ctx()).select_box,
        };
        return Some(start.action(view, target));
    }
    if read_only {
        return Some(SelectionDragAction::BeginMarquee(band));
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
        Some(PointerTarget::Junction(_) | PointerTarget::Wire(_)) if view.filter.wires => {
            let wire = wire();
            if view.design.active_wire_point_is_draggable(wire) {
                SelectionDragAction::BeginWireVertex(wire)
            } else {
                SelectionDragAction::BeginMarquee(band)
            }
        }
        _ => SelectionDragAction::BeginMarquee(band),
    })
}

/// Where a one-finger drag began, for [`TouchDragStart::action`].
struct TouchDragStart {
    grid: Point,
    wire: Point,
    band: Point,
    read_only: bool,
    box_latched: bool,
}

impl TouchDragStart {
    /// What a one-finger drag starts (D1 section 10, R-3). Only a drag that
    /// begins on the selection moves it; anywhere else the finger pans, so
    /// a pan on a dense sheet cannot pick up the part under it (E-11). With
    /// the box latch on, a drag off the selection draws a selection box.
    fn action(
        &self,
        view: &SelectionDragView<'_>,
        target: Option<PointerTarget>,
    ) -> SelectionDragAction {
        match target {
            Some(target)
                if !self.read_only && target_is_selected(&view.editor.selection, target) =>
            {
                match target {
                    PointerTarget::Junction(_) | PointerTarget::Wire(_) => {
                        if view.filter.wires
                            && view.design.active_wire_point_is_draggable(self.wire)
                        {
                            SelectionDragAction::BeginWireVertex(self.wire)
                        } else {
                            SelectionDragAction::BeginTouchPan
                        }
                    }
                    target => SelectionDragAction::BeginSelection {
                        target,
                        position: self.grid,
                    },
                }
            }
            _ if self.box_latched => SelectionDragAction::BeginMarquee(self.band),
            _ => SelectionDragAction::BeginTouchPan,
        }
    }
}

/// Whether `target` is in `selection`.
pub fn target_is_selected(selection: &Selection, target: PointerTarget) -> bool {
    match target {
        PointerTarget::Component(id) => selection.has_component(id),
        PointerTarget::DesignNote(id) => selection.has_design_note(id),
        PointerTarget::DocumentationShape(id) => selection.has_documentation_shape(id),
        PointerTarget::Probe(id) => selection.has_probe(id),
        PointerTarget::NetLabel(id) => selection.has_net_label(id),
        PointerTarget::BusTap(id) => selection.has_bus_tap(id),
        PointerTarget::Bus(id) => selection.has_bus(id),
        PointerTarget::Wire(id) => selection.has_wire(id),
        PointerTarget::Junction(position) => selection.has_junction(position),
    }
}

fn touch_pan_id() -> egui::Id {
    egui::Id::new("rspice.schematic.touch_pan")
}

/// Whether a one-finger pan is under way. The navigation pass moves the
/// view with the finger while it is, and ends it when the finger lifts.
pub fn touch_pan_active(ctx: &egui::Context) -> bool {
    ctx.data(|data| data.get_temp(touch_pan_id()).unwrap_or(false))
}

pub fn set_touch_pan(ctx: &egui::Context, active: bool) {
    ctx.data_mut(|data| {
        if active {
            data.insert_temp(touch_pan_id(), true);
        } else {
            data.remove::<bool>(touch_pan_id());
        }
    });
}

pub fn update(
    response: &Response,
    view: &SelectionDragView<'_>,
    viewport: &Viewport,
) -> Option<SelectionDragAction> {
    if !response.dragged_by(PointerButton::Primary) {
        return None;
    }
    let pointer = response.hover_pos()?;
    let position = resolve_grid_pointer(
        &view.editor.snap_engine,
        view.design.document.grid_size,
        viewport,
        pointer,
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
        _ if view.editor.selection_rect.is_active() => Some(SelectionDragAction::UpdateMarquee(
            screen_to_schematic(viewport, pointer),
        )),
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
            // The add latch stands in for the modifier on touch.
            additive: ui.input(|i| i.modifiers.ctrl || i.modifiers.shift || i.modifiers.command)
                || (canvas_touch_mode(ui.ctx()) && touch_latches(ui.ctx()).select_add),
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
