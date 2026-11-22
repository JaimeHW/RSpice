//! Selection gesture interpretation over borrowed design and session inputs.

use super::{
    coordinates::screen_to_schematic,
    design_view::DesignView,
    instance_text::{InstanceTextHit, instance_texts_at},
    mobile_controls::touch_latches,
    pointer_target::{PointerHit, PointerQuery, PointerTarget, canvas_touch_mode, pointer_target},
    snap_resolution::{resolve_grid_pointer, resolve_target_pointer, target_acquisition_radius},
    stretch_interaction::stretch_delta_for_policy,
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
use rspice_design::schematic::{
    document::SchematicDocument,
    selection::Selection,
    stretch::{StretchOrthogonalPolicy, StretchTarget},
    wire::WireSegment,
};
use rspice_design_model::Point;
use rspice_design_model::design_management::SheetTextVariables;

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
    /// A drag that began on a part's name or value: it moves that text and
    /// leaves the part, and anything else selected, where they are.
    BeginText {
        hit: InstanceTextHit,
        position: Point,
    },
    BeginWireVertex(Point),
    /// A drag that began on a wire's body, away from its corners: it takes
    /// the wire alone and slides that segment, and the wires meeting its
    /// ends follow (see [`wire_segment_drag_motion`]).
    BeginWireSegment {
        wire_id: u64,
        segment_index: usize,
        position: Point,
    },
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
    MoveText {
        operation_id: u64,
        hit: InstanceTextHit,
        from: Point,
        delta: Point,
        position: Point,
    },
    /// The pointer moved during a segment drag. The drawing is untouched
    /// until release; the canvas previews where the segment would land.
    MoveWireSegment {
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
    sheet_variables: impl FnOnce() -> SheetTextVariables,
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
        sheet_variables,
    );
    // The rubber band follows the pointer, not the grid.
    let band = screen_to_schematic(viewport, position);
    // A press on a part's name or value takes the part, as a click does,
    // and the drag carries the text alone.
    let text = match target {
        Some(PointerTarget::Component(id)) => {
            let components = view
                .design
                .visible_objects(&view.design.document.components, |item| item.id);
            instance_texts_at(
                &view.design,
                &components,
                symbols,
                viewport,
                ui.ctx().pixels_per_point(),
                position,
                false,
            )
            .into_iter()
            .find(|hit| hit.component == id)
        }
        _ => None,
    };
    if canvas_touch_mode(ui.ctx()) {
        // A finger moves only what is selected; elsewhere it pans.
        if let Some(hit) = text
            && !read_only
            && view.editor.selection.has_component(hit.component)
        {
            return Some(SelectionDragAction::BeginText {
                hit,
                position: grid,
            });
        }
        let start = TouchDragStart {
            grid,
            wire: wire(),
            band,
            read_only,
            box_latched: touch_latches(ui.ctx()).select_box,
        };
        return Some(start.action(view, target));
    }
    // On a read-only view a drag that starts on an object still asks to
    // move it, so the owner can select it and say why it stays put; a drag
    // from empty canvas draws the selection box as anywhere else.
    if let Some(hit) = text {
        return Some(SelectionDragAction::BeginText {
            hit,
            position: grid,
        });
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
            } else if let Some(PointerTarget::Wire(id)) = target {
                wire_body_drag(view, id, band, grid)
                    .unwrap_or(SelectionDragAction::BeginMarquee(band))
            } else {
                SelectionDragAction::BeginMarquee(band)
            }
        }
        _ => SelectionDragAction::BeginMarquee(band),
    })
}

/// What a drag that began on wire `id`'s body, away from its corners,
/// starts. Grabbed as one member of a larger selection, the wire carries the
/// whole selection, as a part does; otherwise the wire is taken alone and
/// the segment under `band` slides, keeping the wires at its ends attached.
fn wire_body_drag(
    view: &SelectionDragView<'_>,
    id: u64,
    band: Point,
    grid: Point,
) -> Option<SelectionDragAction> {
    let selection = &view.editor.selection;
    if selection.has_wire(id) && selection.count() > 1 {
        return Some(SelectionDragAction::BeginSelection {
            target: PointerTarget::Wire(id),
            position: grid,
        });
    }
    Some(SelectionDragAction::BeginWireSegment {
        wire_id: id,
        segment_index: wire_segment_at(view.design.document, id, band)?,
        position: grid,
    })
}

/// The segment of wire `id` nearest `point`.
pub fn wire_segment_at(document: &SchematicDocument, id: u64, point: Point) -> Option<usize> {
    let wire = document.wires.iter().find(|wire| wire.id == id)?;
    wire.points
        .windows(2)
        .enumerate()
        .map(|(index, ends)| {
            (
                index,
                WireSegment::new(ends[0], ends[1]).distance_to_point(point),
            )
        })
        .min_by(|left, right| left.1.total_cmp(&right.1))
        .map(|(index, _)| index)
}

/// What a segment drag from `from` to `to` asks of the stretch that carries
/// it out. A square segment under square routing slides sideways only, so it
/// and the wires at its ends stay square, as a dragged segment does in KiCad
/// and Altium; an angled segment, or any segment under 45-degree or free
/// routing, follows the pointer and its neighbours may turn diagonal.
pub fn wire_segment_drag_motion(
    document: &SchematicDocument,
    wire_id: u64,
    segment_index: usize,
    from: Point,
    to: Point,
    square_routing: bool,
) -> (Point, StretchTarget, StretchOrthogonalPolicy) {
    let target = StretchTarget::WireSegment {
        wire_id,
        segment_index,
    };
    let delta = Point::new(to.x.saturating_sub(from.x), to.y.saturating_sub(from.y));
    let square = document
        .wires
        .iter()
        .find(|wire| wire.id == wire_id)
        .and_then(|wire| {
            wire.points
                .get(segment_index..segment_index.checked_add(2)?)
        })
        .is_some_and(|ends| WireSegment::new(ends[0], ends[1]).is_orthogonal());
    if square_routing && square {
        let policy = StretchOrthogonalPolicy::PreserveOrthogonal;
        (
            stretch_delta_for_policy(delta, target, policy, document),
            target,
            policy,
        )
    } else {
        (delta, target, StretchOrthogonalPolicy::AllowDiagonal)
    }
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
                        if !view.filter.wires {
                            SelectionDragAction::BeginTouchPan
                        } else if view.design.active_wire_point_is_draggable(self.wire) {
                            SelectionDragAction::BeginWireVertex(self.wire)
                        } else if let PointerTarget::Wire(id) = target {
                            wire_body_drag(view, id, self.band, self.grid)
                                .unwrap_or(SelectionDragAction::BeginTouchPan)
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
            kind: DragType::WireSegment(..),
            last_position: Some(from),
            operation_id,
        }) => (position != from).then_some(SelectionDragAction::MoveWireSegment {
            operation_id,
            from,
            position,
        }),
        Some(ActiveSelectionDrag {
            kind: DragType::MoveText(component, text),
            last_position: Some(last),
            operation_id,
        }) => {
            let delta = Point::new(
                position.x.saturating_sub(last.x),
                position.y.saturating_sub(last.y),
            );
            (delta.x != 0 || delta.y != 0).then_some(SelectionDragAction::MoveText {
                operation_id,
                hit: InstanceTextHit { component, text },
                from: last,
                delta,
                position,
            })
        }
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
