//! Symbol pointer input and local edit drafts. The host validates requests and publishes edits.

use egui::Pos2;
use rspice_design::occurrence::DocumentOccurrence;
use rspice_design::symbol::{
    SymbolAttributeKind, SymbolDocument, SymbolEditorMetadata, SymbolPin, SymbolShape,
};
use rspice_design_model::{
    Point,
    cell_view::CellViewRef,
    port::PortDirection,
    symbol_pin::{SymbolElectricalType, SymbolPinSide},
};
use rspice_vocabulary::product::ProjectId;

use super::handles::{self, SymbolHandle, SymbolHandleRef};
use super::session::{
    SymbolArcSweep, SymbolEditorSession, SymbolPinNameDraft, SymbolSelection, SymbolTextDraft,
    SymbolTool, bearing, label_name, selection_subject, shape_kind,
};
use super::{
    SymbolPointer, SymbolViewport, hit_handle, hit_label, hit_origin, hit_pin, hit_shape,
    snap_point, snap_to_terminal_grid,
};
use crate::view::mobile_controls::{TouchLatches, touch_latches};

/// Identity of the actual symbol view and the registry revision used to resolve an event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolRequestSource {
    pub project: ProjectId,
    pub document: CellViewRef,
    pub occurrence: Option<DocumentOccurrence>,
    pub design_epoch: u64,
    pub document_epoch: u64,
    pub library_revision: u64,
    /// Host window identity, including embedded windows sharing one native viewport.
    pub window: u64,
}

impl SymbolRequestSource {
    fn same_owner(&self, other: &Self) -> bool {
        self.project == other.project
            && self.document == other.document
            && self.occurrence == other.occurrence
            && self.design_epoch == other.design_epoch
            && self.window == other.window
    }

    fn same_view(&self, other: &Self) -> bool {
        self.same_owner(other) && self.document_epoch == other.document_epoch
    }
}

/// Only the pointer and key facts consumed by the symbol canvas; no global
/// app access.
#[derive(Debug, Clone, Copy, Default)]
pub struct SymbolCanvasInput {
    pub pointer: Pos2,
    pub drag_started: bool,
    pub dragged: bool,
    pub drag_stopped: bool,
    pub clicked: bool,
    pub double_clicked: bool,
    /// Shift, or on touch the strip's add latch: a click adds to the
    /// selection or takes out of it.
    pub extend_selection: bool,
    /// The touch strip's box latch: a finger's drag off the selection draws
    /// a selection box rather than panning.
    pub box_latched: bool,
    /// Ctrl, Shift or Command held as a marquee is let go: what the marquee
    /// takes joins the selection, as the schematic's marquee adds it.
    pub add_to_selection: bool,
    /// Enter, while a line or polygon is being drawn: finish it.
    pub finish: bool,
    /// Backspace, while a shape is being drawn: take back its last point.
    pub remove_last_point: bool,
    /// R, while a pin is being dragged: turn its lead a quarter clockwise
    /// about its tip, inside the drag's one edit.
    pub turn_pin: bool,
}

#[derive(Debug, Clone)]
pub struct SymbolCanvasRequest {
    pub source: SymbolRequestSource,
    pub tool: SymbolTool,
    pub selection: SymbolSelection,
    pub viewport: SymbolViewport,
    pub input: SymbolCanvasInput,
}

pub fn canvas_request(
    source: &SymbolRequestSource,
    session: &SymbolEditorSession,
    viewport: SymbolViewport,
    response: &egui::Response,
) -> Option<SymbolCanvasRequest> {
    // A finger held still opens the context menu. egui reports the press as
    // a drag started on that same frame, and the lift that follows as its
    // end; neither is the canvas's, so the rest of the press does nothing.
    if held_for_menu(response) {
        return None;
    }
    let drag_started = response.drag_started_by(egui::PointerButton::Primary);
    // A drag reports as started only once the pointer has left the press by
    // egui's threshold, and what it picks up is what was under the press.
    let pressed_at = drag_started
        .then(|| response.ctx.input(|input| input.pointer.press_origin()))
        .flatten();
    // The keys belong to the shape being drawn, and only while the canvas
    // holds the keyboard; with nothing part drawn they are left to whatever
    // else answers them.
    let drafting = response.has_focus() && session.tool.draws() && session.has_draft();
    let finish = drafting
        && matches!(session.tool, SymbolTool::Line | SymbolTool::Polygon)
        && response
            .ctx
            .input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Enter));
    let remove_last_point = drafting
        && session.text_draft.is_none()
        && response
            .ctx
            .input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Backspace));
    // On touch the strip's latches stand in for Shift and for a box drag.
    let latches = if viewport.pointer == SymbolPointer::Finger {
        touch_latches(&response.ctx)
    } else {
        TouchLatches::default()
    };
    let input = SymbolCanvasInput {
        pointer: pressed_at
            .or_else(|| response.interact_pointer_pos())
            .or_else(|| response.hover_pos())
            .unwrap_or_else(|| response.rect.center()),
        drag_started,
        dragged: response.dragged_by(egui::PointerButton::Primary),
        drag_stopped: response.drag_stopped_by(egui::PointerButton::Primary),
        clicked: response.clicked_by(egui::PointerButton::Primary),
        double_clicked: response.double_clicked_by(egui::PointerButton::Primary),
        extend_selection: response.ctx.input(|input| input.modifiers.shift) || latches.select_add,
        box_latched: latches.select_box,
        add_to_selection: latches.select_add
            || response.ctx.input(|input| {
                input.modifiers.ctrl || input.modifiers.shift || input.modifiers.command
            }),
        finish,
        remove_last_point,
        turn_pin: false,
    };
    if !(input.drag_started
        || input.dragged
        || input.drag_stopped
        || input.clicked
        || input.finish
        || input.remove_last_point)
    {
        return None;
    }
    Some(SymbolCanvasRequest {
        source: source.clone(),
        tool: session.tool,
        selection: session.effective_selection(),
        viewport,
        input,
    })
}

/// Whether a second finger holds the canvas for a pinch this frame. The
/// pinch owns the gesture from the frame the second finger lands until the
/// first lifts: whatever the first finger had begun is let go, and none of
/// the gesture reaches the tools, its lift included.
pub fn pinch_holds_canvas(ctx: &egui::Context, session: &mut SymbolEditorSession) -> bool {
    let id = egui::Id::new("rspice.symbol-canvas.pinch");
    let (pinching, pressed) = ctx.input(|input| {
        (
            input.multi_touch().is_some(),
            input.pointer.primary_down() || input.pointer.primary_released(),
        )
    });
    let held = ctx.data(|data| data.get_temp::<bool>(id)) == Some(true);
    if pinching && !held {
        release_for_pinch(session);
        ctx.data_mut(|data| data.insert_temp(id, true));
        return true;
    }
    if held && !pinching && !pressed {
        ctx.data_mut(|data| data.remove::<bool>(id));
        return false;
    }
    held || pinching
}

/// A second finger landed: what the first finger's drag had picked up is
/// put down where it is, its selection box goes, and a shape its drag began
/// is not drawn.
fn release_for_pinch(session: &mut SymbolEditorSession) {
    session.clear_drag_state();
    session.marquee_start = None;
    session.marquee_current = None;
    if session.drawing_drag {
        session.clear_draft();
    }
}

/// Whether the press on `response` is a long press, which belongs to the
/// context menu from the frame it is recognised until the finger lifts.
fn held_for_menu(response: &egui::Response) -> bool {
    let id = response.id.with("symbol-canvas.held-for-menu");
    if response.long_touched() {
        response.ctx.data_mut(|data| data.insert_temp(id, true));
        return true;
    }
    let held = response.ctx.data(|data| data.get_temp::<bool>(id)) == Some(true);
    let pressed = response
        .ctx
        .input(|input| input.pointer.primary_down() || input.pointer.primary_released());
    if held && !pressed {
        response.ctx.data_mut(|data| data.remove::<bool>(id));
        return false;
    }
    held
}

/// Discard unfinished input if its document, occurrence or externally edited revision changed.
/// Hosts advance `canvas_source` after publishing their own edit so a drag stays one transaction.
pub fn bind_canvas_source(session: &mut SymbolEditorSession, source: SymbolRequestSource) {
    if let Some(previous) = &session.canvas_source
        && previous != &source
    {
        let same_view = previous.same_view(&source);
        session.clear_drag_state();
        session.marquee_start = None;
        session.marquee_current = None;
        session.clear_draft();
        if !same_view {
            session.clear_selection();
        }
    }
    session.canvas_source = Some(source);
}

/// Resume a retained view after the host projects it back into the active slot.
/// Queued requests keep their old epoch. Real owner or revision changes still retire drafts.
pub fn resume_canvas_source(session: &mut SymbolEditorSession, source: SymbolRequestSource) {
    if let Some(previous) = &mut session.canvas_source
        && previous.same_owner(&source)
    {
        previous.document_epoch = source.document_epoch;
        session.inspector.resume_source(&source);
    }
    bind_canvas_source(session, source);
}

#[derive(Debug, Clone, Copy)]
pub struct SymbolEditCapabilities {
    pub edit: bool,
}

/// Changes affect only the caller's draft. The host owns history and canonical publication.
#[derive(Debug, Default)]
pub struct SymbolEditOutcome {
    pub changed: bool,
    pub undo_before: Option<Box<SymbolDocument>>,
    /// What the undo step is called, as Undo and Redo name it: "Move pin IN".
    pub step: String,
    pub edit_denied: bool,
    /// Why a requested edit was refused, or did nothing, for the host to
    /// tell the author.
    pub refusal: Option<String>,
    /// What a completed edit did, where the canvas does not show it at a
    /// glance: "Snapped 3 objects to the grid."
    pub report: Option<String>,
    /// A finger's drag that began off the selection: the view follows the
    /// finger until it lifts, as a schematic's does.
    pub pan: bool,
}

pub fn edit_canvas(
    session: &mut SymbolEditorSession,
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    viewport: SymbolViewport,
    input: SymbolCanvasInput,
    capabilities: SymbolEditCapabilities,
) -> SymbolEditOutcome {
    let mut edit = CanvasEdit {
        session,
        capabilities,
        outcome: SymbolEditOutcome::default(),
    };
    edit.outcome.changed =
        handle_canvas_interaction(&mut edit, document, metadata, viewport, input);
    edit.outcome
}

struct CanvasEdit<'a> {
    session: &'a mut SymbolEditorSession,
    capabilities: SymbolEditCapabilities,
    outcome: SymbolEditOutcome,
}

impl CanvasEdit<'_> {
    fn deny_read_only_edit(&mut self) -> bool {
        if self.capabilities.edit {
            return false;
        }
        self.outcome.edit_denied = true;
        true
    }

    fn record_undo(&mut self, document: &SymbolDocument, step: impl FnOnce() -> String) {
        if self.outcome.undo_before.is_none() {
            self.outcome.undo_before = Some(Box::new(document.clone()));
            self.outcome.step = step();
        }
    }
}

fn handle_canvas_interaction(
    edit: &mut CanvasEdit<'_>,
    document: &mut SymbolDocument,
    editor: &mut SymbolEditorMetadata,
    viewport: SymbolViewport,
    input: SymbolCanvasInput,
) -> bool {
    if input.turn_pin {
        return turn_dragged_pin(edit, document);
    }
    let pointer = input.pointer;
    let raw_point = viewport.screen_to_world(pointer);
    let body_point = if edit.session.snap_to_grid {
        snap_point(raw_point, edit.session.grid_spacing)
    } else {
        raw_point
    };
    // A terminal is the wiring contract, not artwork: it snaps to the
    // terminal pitch whatever the display grid shows and whether or not the
    // author has body snapping on. A pin dropped between grid points is a
    // pin no parent schematic can reach.
    let terminal_point = snap_to_terminal_grid(raw_point);

    // A handle Stretch armed follows the pointer until a click puts it down.
    if let Some(stretch) = edit.session.stretch {
        if input.clicked || input.drag_stopped {
            edit.session.stretch = None;
            return put_down_stretch(edit, document, stretch, body_point);
        }
        return false;
    }

    if edit.session.tool.draws() {
        return draw_with_tool(edit, document, input, body_point);
    }

    // A point put on an edge by grabbing its middle is an edit already,
    // whether or not the pointer has moved it yet.
    let mut grabbed = false;
    if input.drag_started {
        let selection = edit.session.effective_selection();
        let several = selection.len() > 1;
        // A handle of a selected shape is taken before the shape or
        // anything beneath it, except the point a shape merely stands at,
        // which with more selected moves them all. Grabbing something that
        // is already part of a multi-object selection moves the whole
        // selection. Grabbing anything else reduces the selection to it
        // first, which is what makes a mis-grab recoverable rather than a
        // silent group move.
        let handle = hit_handle(document, &selection, viewport, pointer)
            .filter(|handle| !(several && handle.handle == SymbolHandle::Anchor));
        if let Some(handle) = handle {
            grabbed = grab_handle(edit, document, handle);
        } else if viewport.pointer == SymbolPointer::Finger
            && !finger_takes_hold(edit, document, editor, viewport, pointer)
        {
            // A finger moves only what is selected, so a pan across a dense
            // symbol cannot pick up what lies under it; anywhere else it pans,
            // or with the box latch on draws a selection box.
            if input.box_latched && edit.session.tool == SymbolTool::Select {
                edit.session.marquee_start = Some(body_point);
                edit.session.marquee_current = Some(body_point);
            } else {
                edit.outcome.pan = true;
            }
        } else if grab_belongs_to_group(edit, document, editor, viewport, pointer) {
            edit.session.dragging_group = Some(body_point);
            edit.session.drag_undo_recorded = false;
        } else if let Some(pin) = hit_pin(document, viewport, pointer) {
            edit.session.select_pin(pin.clone());
            edit.session.dragging_pin = Some(pin);
            edit.session.drag_undo_recorded = false;
        } else if let Some(kind) = hit_label(document, editor, edit.session, viewport, pointer) {
            edit.session.select_attribute(kind);
            edit.session.dragging_label = Some(kind);
            edit.session.drag_undo_recorded = false;
        } else if hit_origin(document, viewport, pointer) {
            edit.session.clear_selection();
            edit.session.dragging_origin = true;
            edit.session.drag_undo_recorded = false;
        } else if let Some(shape_index) = hit_shape(document, viewport, pointer) {
            edit.session.select_shape(shape_index);
            edit.session.dragging_shape = Some((shape_index, body_point));
            edit.session.drag_undo_recorded = false;
        } else if matches!(edit.session.tool, SymbolTool::Select) {
            edit.session.marquee_start = Some(body_point);
            edit.session.marquee_current = Some(body_point);
        }
    }

    if input.dragged
        && let Some(drag) = edit.session.dragging_handle
    {
        if edit.deny_read_only_edit() {
            edit.session.clear_drag_state();
            return grabbed;
        }
        let Some(shape) = document.body.get(drag.shape) else {
            return grabbed;
        };
        let Some(moved) = handles::moved(shape, &drag, body_point) else {
            return grabbed;
        };
        let step = handles::step_name(shape, drag.handle);
        record_drag_symbol_edit(edit, document, || step);
        document.body[drag.shape] = moved;
        return true;
    }
    if input.dragged
        && let Some(last_point) = edit.session.dragging_group
    {
        if edit.deny_read_only_edit() {
            edit.session.clear_drag_state();
            return false;
        }
        let delta = body_point - last_point;
        if delta == Point::origin() || group_lands_on_a_pin(edit, document, delta) {
            return false;
        }
        let moved = selection_subject(document, &edit.session.effective_selection());
        record_drag_symbol_edit(edit, document, || format!("Move {moved}"));
        translate_selection(edit, document, editor, delta);
        edit.session.dragging_group = Some(body_point);
        return true;
    }
    if input.dragged
        && let Some(name) = edit.session.dragging_pin.clone()
    {
        if edit.deny_read_only_edit() {
            edit.session.clear_drag_state();
            return false;
        }
        if document.pin(&name).and_then(|pin| pin.position) != Some(terminal_point)
            && let Some(moved) = pin_placed_at(document, &name, terminal_point)
        {
            // Passing over another pin holds this one at its last free
            // point; it never lands there, even for one frame.
            if moved
                .position
                .is_some_and(|point| pin_at(document, &name, point).is_some())
            {
                return false;
            }
            if document.pin(&name) == Some(&moved) {
                return false;
            }
            record_drag_symbol_edit(edit, document, || format!("Move pin {name}"));
            if let Some(pin) = document.pin_mut(&name) {
                *pin = moved;
                return true;
            }
        }
    }
    if input.dragged
        && let Some(kind) = edit.session.dragging_label
    {
        if edit.deny_read_only_edit() {
            edit.session.clear_drag_state();
            return false;
        }
        if editor
            .attribute(kind)
            .is_some_and(|attribute| attribute.position != body_point)
        {
            record_drag_symbol_edit(edit, document, || {
                format!("Move {} label", label_name(kind).to_lowercase())
            });
            if let Some(attribute) = editor.attribute_mut(kind) {
                attribute.position = body_point;
            }
            sync_legacy_attribute_anchor(document, kind, body_point);
            edit.session.select_attribute(kind);
            return true;
        }
    }
    if input.dragged && edit.session.dragging_origin {
        if edit.deny_read_only_edit() {
            edit.session.clear_drag_state();
            return false;
        }
        if document.origin != body_point {
            record_drag_symbol_edit(edit, document, || "Move origin".to_owned());
            document.origin = body_point;
            return true;
        }
    }
    if input.dragged
        && let Some((shape_index, last_point)) = edit.session.dragging_shape
    {
        if edit.deny_read_only_edit() {
            edit.session.clear_drag_state();
            return false;
        }
        let delta = body_point - last_point;
        if delta != Point::origin() && shape_index < document.body.len() {
            let moved = shape_kind(&document.body[shape_index]).to_lowercase();
            record_drag_symbol_edit(edit, document, || format!("Move {moved}"));
            if let Some(shape) = document.body.get_mut(shape_index) {
                shape.translate(delta);
                edit.session.dragging_shape = Some((shape_index, body_point));
                return true;
            }
        }
    }
    if input.dragged && edit.session.marquee_start.is_some() {
        edit.session.marquee_current = Some(body_point);
    }

    if input.drag_stopped {
        if let Some(start) = edit.session.marquee_start.take() {
            let end = edit.session.marquee_current.take().unwrap_or(body_point);
            let mut taken = SymbolSelection::in_rect(document, editor, start, end);
            if input.add_to_selection {
                let mut selection = edit.session.effective_selection();
                selection.pins.append(&mut taken.pins);
                selection.shapes.append(&mut taken.shapes);
                selection.attributes.append(&mut taken.attributes);
                taken = selection;
            }
            edit.session.set_selection(taken);
        }
        edit.session.clear_drag_state();
    }

    if !input.clicked {
        return grabbed;
    }

    match edit.session.tool {
        SymbolTool::Select => {
            let extend = input.extend_selection;
            let handle = hit_handle(
                document,
                &edit.session.effective_selection(),
                viewport,
                pointer,
            );
            // A click on a point of the selected line picks that point,
            // for Delete; the shape stays selected.
            if let Some(SymbolHandleRef {
                shape,
                handle: SymbolHandle::Vertex(vertex),
            }) = handle.filter(|_| !extend)
            {
                edit.session.select_shape(shape);
                edit.session.selected_vertex = Some((shape, vertex));
                return false;
            }
            edit.session.selected_vertex = None;
            if handle.is_some() && !extend {
                // Any other handle belongs to a shape already selected.
                return false;
            }
            if let Some(pin) = hit_pin(document, viewport, pointer) {
                toggle_or_select(
                    edit,
                    extend,
                    |selection| selection.toggle_pin(&pin),
                    || SymbolSelection::single_pin(pin.clone()),
                );
            } else if let Some(kind) = hit_label(document, editor, edit.session, viewport, pointer)
            {
                toggle_or_select(
                    edit,
                    extend,
                    |selection| selection.toggle_attribute(kind),
                    || SymbolSelection::single_attribute(kind),
                );
            } else if let Some(shape) = hit_shape(document, viewport, pointer) {
                toggle_or_select(
                    edit,
                    extend,
                    |selection| selection.toggle_shape(shape),
                    || SymbolSelection::single_shape(shape),
                );
            } else if !extend {
                edit.session.clear_selection();
            }
            false
        }
        SymbolTool::PlacePin => place_next_pin(edit, document, terminal_point),
        // The drawing tools took the input above.
        SymbolTool::Line
        | SymbolTool::Polygon
        | SymbolTool::Rectangle
        | SymbolTool::Circle
        | SymbolTool::Arc
        | SymbolTool::Text => false,
    }
}

/// The pin being dragged, its lead turned a quarter clockwise about its tip.
/// It is part of the drag's one edit, named as the drag is.
fn turn_dragged_pin(edit: &mut CanvasEdit<'_>, document: &mut SymbolDocument) -> bool {
    let Some(name) = edit.session.dragging_pin.clone() else {
        return false;
    };
    if edit.deny_read_only_edit() {
        return false;
    }
    let Some(side) = document.pin(&name).map(|pin| pin.side) else {
        return false;
    };
    record_drag_symbol_edit(edit, document, || format!("Move pin {name}"));
    if let Some(pin) = document.pin_mut(&name) {
        pin.side = side.turned_clockwise();
    }
    true
}

/// Take hold of `handle`. Grabbing an edge's middle puts a point there at
/// once, as an edit of its own; whether it did.
fn grab_handle(
    edit: &mut CanvasEdit<'_>,
    document: &mut SymbolDocument,
    handle: SymbolHandleRef,
) -> bool {
    let Some(shape) = document.body.get(handle.shape) else {
        return false;
    };
    edit.session.drag_undo_recorded = false;
    let inserts = matches!(handle.handle, SymbolHandle::EdgeMiddle(_));
    if inserts {
        if edit.deny_read_only_edit() {
            return false;
        }
        let step = handles::step_name(shape, handle.handle);
        record_drag_symbol_edit(edit, document, || step);
    }
    let drag = handles::grab(
        &mut document.body[handle.shape],
        handle.shape,
        handle.handle,
    );
    edit.session.select_shape(handle.shape);
    edit.session.selected_vertex = match drag.handle {
        SymbolHandle::Vertex(vertex) => Some((handle.shape, vertex)),
        _ => None,
    };
    edit.session.dragging_handle = Some(drag);
    inserts
}

/// What a click puts down where Stretch armed `handle`: the shape with that
/// handle at `point`, as one edit; nothing when that changes nothing or
/// would leave the shape with nothing to draw.
fn put_down_stretch(
    edit: &mut CanvasEdit<'_>,
    document: &mut SymbolDocument,
    handle: SymbolHandleRef,
    point: Point,
) -> bool {
    if edit.deny_read_only_edit() {
        return false;
    }
    let Some((stretched, became)) = stretched_shape(document, handle, point)
        .filter(|(stretched, _)| Some(stretched) != document.body.get(handle.shape))
    else {
        return false;
    };
    let step = handles::step_name(&document.body[handle.shape], handle.handle);
    edit.record_undo(document, || step);
    document.body[handle.shape] = stretched;
    edit.session.select_shape(handle.shape);
    edit.session.selected_vertex = match became {
        SymbolHandle::Vertex(vertex) => Some((handle.shape, vertex)),
        _ => None,
    };
    true
}

/// The shape `handle` belongs to with that handle brought to `point`, and
/// the handle it became: what a Stretch armed there would put down.
pub fn stretched_shape(
    document: &SymbolDocument,
    handle: SymbolHandleRef,
    point: Point,
) -> Option<(SymbolShape, SymbolHandle)> {
    let mut shape = document.body.get(handle.shape)?.clone();
    let drag = handles::grab(&mut shape, handle.shape, handle.handle);
    let moved = handles::moved(&shape, &drag, point)?;
    Some((moved, drag.handle))
}

/// Which handle of the selected shapes is under `hover`, for the canvas to
/// mark and Stretch to take hold of. Only Select edits shapes in place.
pub fn track_handle(
    session: &mut SymbolEditorSession,
    document: &SymbolDocument,
    viewport: SymbolViewport,
    hover: Option<Pos2>,
) {
    session.hover_handle = hover
        .filter(|_| session.tool == SymbolTool::Select)
        .and_then(|pos| hit_handle(document, &session.effective_selection(), viewport, pos));
}

/// Stretch: take hold of the handle under the pointer, which then follows
/// it until a click puts it down. Whether there was one.
pub fn arm_stretch(session: &mut SymbolEditorSession) -> bool {
    let Some(handle) = session
        .hover_handle
        .filter(|_| session.tool == SymbolTool::Select)
    else {
        return false;
    };
    session.clear_drag_state();
    session.stretch = Some(handle);
    true
}

/// Where a click would land this frame, snapped as the armed tool snaps, so
/// the shape being drawn can be shown running to it. An arc being swept
/// follows the pointer round its center.
pub fn track_pointer(
    session: &mut SymbolEditorSession,
    viewport: SymbolViewport,
    hover: Option<Pos2>,
) {
    session.pointer = hover.map(|position| {
        let raw = viewport.screen_to_world(position);
        // A pin's tip lands on the terminal pitch, as the click places it.
        if session.tool == SymbolTool::PlacePin {
            snap_to_terminal_grid(raw)
        } else {
            drawing_point(session, raw)
        }
    });
    if let (Some(center), Some(point), Some(sweep)) = (
        session.shape_start,
        session.pointer,
        session.arc_sweep.as_mut(),
    ) {
        sweep.follow(center, point);
    }
}

/// `raw` on the body grid, when the author snaps to it.
fn drawing_point(session: &SymbolEditorSession, raw: Point) -> Point {
    if session.snap_to_grid {
        snap_point(raw, session.grid_spacing)
    } else {
        raw
    }
}

/// One input to an armed drawing tool. A click places the shape's next
/// point. A press-drag places one where it was pressed, when nothing is part
/// drawn, and one where it is let go, so dragging draws a line segment, a
/// rectangle or a circle by its radius. Nothing already drawn is picked up.
fn draw_with_tool(
    edit: &mut CanvasEdit<'_>,
    document: &mut SymbolDocument,
    input: SymbolCanvasInput,
    point: Point,
) -> bool {
    let tool = edit.session.tool;
    let polyline = matches!(tool, SymbolTool::Line | SymbolTool::Polygon);
    if input.remove_last_point {
        take_back_point(edit.session);
        return false;
    }
    // The first click of a double-click placed the point; the second ends
    // the line there rather than placing it again.
    if input.finish || (polyline && input.double_clicked) {
        return finish_pending_polyline(edit, document);
    }
    if input.drag_started {
        edit.session.drawing_drag = !edit.session.has_draft();
        return edit.session.drawing_drag && place_point(edit, document, point);
    }
    if input.drag_stopped {
        let began_shape = std::mem::take(&mut edit.session.drawing_drag);
        if tool == SymbolTool::Text {
            return false;
        }
        let placed = place_point(edit, document, point);
        if began_shape && tool == SymbolTool::Line {
            return finish_pending_polyline(edit, document) || placed;
        }
        return placed;
    }
    input.clicked && place_point(edit, document, point)
}

/// Place the armed tool's next point at `point`; whether that completed a
/// shape.
fn place_point(edit: &mut CanvasEdit<'_>, document: &mut SymbolDocument, point: Point) -> bool {
    match edit.session.tool {
        SymbolTool::Line | SymbolTool::Polygon => add_polyline_point(edit, document, point),
        SymbolTool::Rectangle => add_rectangle(edit, document, point),
        SymbolTool::Circle => add_circle(edit, document, point),
        SymbolTool::Arc => add_arc_point(edit, document, point),
        SymbolTool::Text => open_text_draft(edit, point),
        SymbolTool::Select | SymbolTool::PlacePin => false,
    }
}

/// Backspace: the shape being drawn loses its last point.
fn take_back_point(session: &mut SymbolEditorSession) {
    match session.tool {
        SymbolTool::Line | SymbolTool::Polygon => {
            session.pending_polyline.pop();
        }
        SymbolTool::Arc if session.arc_sweep.is_some() => session.arc_sweep = None,
        SymbolTool::Rectangle | SymbolTool::Circle | SymbolTool::Arc => session.shape_start = None,
        SymbolTool::Text | SymbolTool::Select | SymbolTool::PlacePin => {}
    }
}

fn record_drag_symbol_edit(
    edit: &mut CanvasEdit<'_>,
    document: &SymbolDocument,
    step: impl FnOnce() -> String,
) {
    if edit.session.drag_undo_recorded {
        return;
    }
    edit.record_undo(document, step);
    edit.session.drag_undo_recorded = true;
}

/// Shift-click grows the selection; a plain click replaces it.
fn toggle_or_select(
    edit: &mut CanvasEdit<'_>,
    extend: bool,
    toggle: impl FnOnce(&mut SymbolSelection),
    replace: impl FnOnce() -> SymbolSelection,
) {
    if extend {
        let mut selection = edit.session.effective_selection();
        toggle(&mut selection);
        edit.session.set_selection(selection);
        return;
    }
    edit.session.set_selection(replace());
}

/// Whether a finger's drag from `pointer` takes hold of what is under it:
/// an object already selected, or the origin, which is never selected and
/// is moved by its own mark.
fn finger_takes_hold(
    edit: &CanvasEdit<'_>,
    document: &SymbolDocument,
    editor: &SymbolEditorMetadata,
    viewport: SymbolViewport,
    pointer: Pos2,
) -> bool {
    let selection = edit.session.effective_selection();
    if let Some(pin) = hit_pin(document, viewport, pointer) {
        return selection.contains_pin(&pin);
    }
    if let Some(kind) = hit_label(document, editor, edit.session, viewport, pointer) {
        return selection.attributes.contains(&kind);
    }
    if hit_origin(document, viewport, pointer) {
        return true;
    }
    hit_shape(document, viewport, pointer).is_some_and(|index| selection.shapes.contains(&index))
}

/// Whether the object under `pointer` is one of several already selected.
///
/// A single-object selection is not a group: dragging it must keep the
/// established single-object behaviour, including its terminal-grid snapping.
fn grab_belongs_to_group(
    edit: &CanvasEdit<'_>,
    document: &SymbolDocument,
    editor: &SymbolEditorMetadata,
    viewport: SymbolViewport,
    pointer: Pos2,
) -> bool {
    let selection = edit.session.effective_selection();
    if selection.len() < 2 {
        return false;
    }
    if let Some(pin) = hit_pin(document, viewport, pointer) {
        return selection.contains_pin(&pin);
    }
    if let Some(kind) = hit_label(document, editor, edit.session, viewport, pointer) {
        return selection.attributes.contains(&kind);
    }
    hit_shape(document, viewport, pointer).is_some_and(|index| selection.shapes.contains(&index))
}

/// Whether moving the selection by `delta` would put a selected pin on a pin
/// that stays where it is.
fn group_lands_on_a_pin(edit: &CanvasEdit<'_>, document: &SymbolDocument, delta: Point) -> bool {
    let selection = edit.session.effective_selection();
    let pin_delta = snap_to_terminal_grid(delta);
    selection.pins.iter().any(|name| {
        document
            .pin(name)
            .and_then(|pin| pin.position)
            .is_some_and(|position| {
                let moved = position + pin_delta;
                document.pins.iter().any(|other| {
                    !selection.contains_pin(&other.name) && other.position == Some(moved)
                })
            })
    })
}

/// Move every selected object by `delta` as one edit.
///
/// Terminals travel by the same delta rounded to the terminal pitch, so a
/// group moved on a fine display grid arrives with its pins still on the
/// lattice a parent schematic wires to. Each pin keeps its side and lead: a
/// group move is a translation.
fn translate_selection(
    edit: &mut CanvasEdit<'_>,
    document: &mut SymbolDocument,
    editor: &mut SymbolEditorMetadata,
    delta: Point,
) {
    let selection = edit.session.effective_selection();
    let pin_delta = snap_to_terminal_grid(delta);
    for name in &selection.pins {
        let Some(position) = document.pin(name).and_then(|pin| pin.position) else {
            continue;
        };
        if let Some(pin) = document.pin_mut(name) {
            pin.position = Some(position + pin_delta);
        }
    }
    for index in &selection.shapes {
        if let Some(shape) = document.body.get_mut(*index) {
            shape.translate(delta);
        }
    }
    for kind in &selection.attributes {
        let Some(attribute) = editor.attribute_mut(*kind) else {
            continue;
        };
        attribute.position = attribute.position + delta;
        let position = attribute.position;
        sync_legacy_attribute_anchor(document, *kind, position);
    }
}

/// A click of the Pin tool: its next pin goes down with its tip at `point`,
/// on the side the tool shows there, and its name opens for typing. The
/// tool stays armed for the pin after it.
fn place_next_pin(edit: &mut CanvasEdit<'_>, document: &mut SymbolDocument, point: Point) -> bool {
    if edit.deny_read_only_edit() {
        return false;
    }
    let offer = next_pin_offer(document, edit.session);
    let name = offer.name().to_owned();
    if let Some(other) = pin_at(document, &name, point) {
        edit.outcome.refusal = Some(format!(
            "Pin {name} was not placed: pin {other} already stands there, and two pins on one point would join their nets in every schematic that places this symbol."
        ));
        return false;
    }
    let side = pin_tool_side(document, edit.session, point);
    edit.record_undo(document, || format!("Place pin {name}"));
    match offer {
        PinOffer::Unplaced(_) => {
            let Some(pin) = document.pin_mut(&name) else {
                return false;
            };
            pin.position = Some(point);
            pin.side = side;
        }
        PinOffer::New { direction, .. } => {
            let pin = SymbolPin::new(name.clone(), direction, Some(point));
            // A pin the interface does not declare is an analog terminal
            // until its author says otherwise, as Add pin makes one.
            let declared = edit
                .session
                .declared_ports
                .iter()
                .any(|port| port.name.eq_ignore_ascii_case(&name));
            document.pins.push(if declared {
                pin.with_side(side)
            } else {
                pin.with_contract(SymbolElectricalType::Analog, side)
            });
        }
    }
    edit.session.select_pin(name.clone());
    edit.session.pin_name_draft = Some(SymbolPinNameDraft {
        pin: name.clone(),
        text: name,
        caret_placed: false,
    });
    true
}

/// The corners a polygon needs before it can close.
const POLYGON_CORNERS: usize = 3;

/// The line or polygon takes `point` as its next vertex. A point on the last
/// one adds nothing, and a polygon's first corner clicked again closes it.
fn add_polyline_point(
    edit: &mut CanvasEdit<'_>,
    document: &mut SymbolDocument,
    point: Point,
) -> bool {
    if edit.deny_read_only_edit() {
        return false;
    }
    let pending = &edit.session.pending_polyline;
    if pending.last() == Some(&point) {
        return false;
    }
    if edit.session.tool == SymbolTool::Polygon
        && pending.len() >= POLYGON_CORNERS
        && pending.first() == Some(&point)
    {
        return finish_pending_polyline(edit, document);
    }
    edit.session.pending_polyline.push(point);
    false
}

/// Put the line or polygon being drawn in the document. A polygon is always
/// closed, so it waits for its third corner.
fn finish_pending_polyline(edit: &mut CanvasEdit<'_>, document: &mut SymbolDocument) -> bool {
    let closed = edit.session.tool == SymbolTool::Polygon;
    let needed = if closed { POLYGON_CORNERS } else { 2 };
    let placed = edit.session.pending_polyline.len();
    if placed < needed {
        if closed && placed > 0 {
            edit.outcome.refusal = Some("A polygon needs three corners.".to_owned());
        }
        return false;
    }
    if edit.deny_read_only_edit() {
        return false;
    }
    edit.record_undo(document, || {
        if closed { "Add polygon" } else { "Add line" }.to_owned()
    });
    let points = std::mem::take(&mut edit.session.pending_polyline);
    document.body.push(SymbolShape::Polyline {
        points,
        closed,
        style: Default::default(),
    });
    select_last_shape(edit.session, document);
    true
}

fn select_last_shape(session: &mut SymbolEditorSession, document: &SymbolDocument) {
    if let Some(index) = document.body.len().checked_sub(1) {
        session.select_shape(index);
    }
}

/// Pin `name` with its tip at `point`. A pin being placed for the first time
/// takes the side the body beside it suggests; a placed pin keeps the side,
/// lead and style it has, wherever it is moved.
fn pin_placed_at(document: &SymbolDocument, name: &str, point: Point) -> Option<SymbolPin> {
    let mut pin = document.pin(name)?.clone();
    if pin.position.is_none() {
        pin.side = document.suggested_pin_side(point);
    }
    pin.position = Some(point);
    Some(pin)
}

/// The pin other than `name` whose terminal stands at `point`.
///
/// Two terminals on one point are one node in every schematic that places
/// the symbol, so no edit may put a pin there.
fn pin_at<'a>(document: &'a SymbolDocument, name: &str, point: Point) -> Option<&'a str> {
    document
        .pins
        .iter()
        .find(|pin| !pin.name.eq_ignore_ascii_case(name) && pin.position == Some(point))
        .map(|pin| pin.name.as_str())
}

/// The first click is a corner, the second the opposite one. A second
/// corner level with the first in either direction would draw no area, so
/// the rectangle waits for one that does.
fn add_rectangle(edit: &mut CanvasEdit<'_>, document: &mut SymbolDocument, point: Point) -> bool {
    if edit.deny_read_only_edit() {
        return false;
    }
    let Some(start) = edit.session.shape_start else {
        edit.session.shape_start = Some(point);
        return false;
    };
    if start.x == point.x || start.y == point.y {
        return false;
    }
    edit.session.shape_start = None;
    edit.record_undo(document, || "Add rectangle".to_owned());
    document.body.push(rectangle_shape(start, point));
    select_last_shape(edit.session, document);
    true
}

/// The rectangle from `start` to the opposite corner `end`, whichever way
/// the pointer went.
pub fn rectangle_shape(start: Point, end: Point) -> SymbolShape {
    SymbolShape::rectangle(start, end)
}

/// The radius of a circle about `center` through `point`, in whole units.
pub fn radius_through(center: Point, point: Point) -> i32 {
    center.distance_squared(point).isqrt()
}

/// The first click is the center, the second a point on the circle.
fn add_circle(edit: &mut CanvasEdit<'_>, document: &mut SymbolDocument, point: Point) -> bool {
    if edit.deny_read_only_edit() {
        return false;
    }
    let Some(center) = edit.session.shape_start else {
        edit.session.shape_start = Some(point);
        return false;
    };
    let radius = radius_through(center, point);
    if radius < 1 {
        return false;
    }
    edit.session.shape_start = None;
    edit.record_undo(document, || "Add circle".to_owned());
    document.body.push(SymbolShape::Circle {
        center,
        radius,
        style: Default::default(),
    });
    select_last_shape(edit.session, document);
    true
}

/// The arc is the center, then where it starts, then where it ends. It runs
/// from the start the way the pointer turned to reach the end.
fn add_arc_point(edit: &mut CanvasEdit<'_>, document: &mut SymbolDocument, point: Point) -> bool {
    if edit.deny_read_only_edit() {
        return false;
    }
    let Some(center) = edit.session.shape_start else {
        edit.session.shape_start = Some(point);
        return false;
    };
    let Some(mut sweep) = edit.session.arc_sweep else {
        if point != center {
            edit.session.arc_sweep = Some(SymbolArcSweep::new(center, point));
        }
        return false;
    };
    sweep.follow(center, point);
    edit.session.arc_sweep = Some(sweep);
    let Some(shape) = arc_shape(center, &sweep) else {
        return false;
    };
    edit.session.shape_start = None;
    edit.session.arc_sweep = None;
    edit.record_undo(document, || "Add arc".to_owned());
    document.body.push(shape);
    select_last_shape(edit.session, document);
    true
}

/// The arc about `center` that `sweep` has turned out, in whole units and
/// degrees; `None` until it has turned at least a degree.
pub fn arc_shape(center: Point, sweep: &SymbolArcSweep) -> Option<SymbolShape> {
    let sweep_degrees = sweep.degrees.round() as i32;
    (sweep_degrees != 0).then(|| SymbolShape::Arc {
        center,
        radius: radius_through(center, sweep.start),
        start_degrees: bearing(center, sweep.start).round() as i32,
        sweep_degrees,
        style: Default::default(),
    })
}

/// Text starts as a draft at `point`, typed into an entry there; the host
/// places it when the entry is committed.
fn open_text_draft(edit: &mut CanvasEdit<'_>, point: Point) -> bool {
    if edit.deny_read_only_edit() {
        return false;
    }
    edit.session.text_draft = Some(SymbolTextDraft {
        anchor: point,
        text: String::new(),
        caret_placed: false,
    });
    false
}

fn sync_legacy_attribute_anchor(
    document: &mut SymbolDocument,
    kind: SymbolAttributeKind,
    position: Point,
) {
    match kind {
        SymbolAttributeKind::Reference => document.name_anchor = position,
        SymbolAttributeKind::Value => document.value_anchor = position,
        SymbolAttributeKind::Model => {}
    }
}

/// The pin the Pin tool puts down next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PinOffer {
    /// A pin the document has that is not placed yet.
    Unplaced(String),
    /// A pin the document does not have yet: a declared port's, or the next
    /// free `PIN<n>`.
    New {
        name: String,
        direction: PortDirection,
    },
}

impl PinOffer {
    pub fn name(&self) -> &str {
        match self {
            Self::Unplaced(name) | Self::New { name, .. } => name,
        }
    }
}

/// What the Pin tool offers next: the selected pin if it is unplaced, else
/// the document's first unplaced pin, else the first declared port with no
/// pin, else the first `PIN<n>` no pin is named.
pub fn next_pin_offer(document: &SymbolDocument, session: &SymbolEditorSession) -> PinOffer {
    let unplaced = |name: &str| document.pin(name).is_some_and(|pin| pin.position.is_none());
    if let Some(selected) = session
        .selected_pin
        .as_deref()
        .filter(|name| unplaced(name))
    {
        return PinOffer::Unplaced(selected.to_owned());
    }
    if let Some(pin) = document.pins.iter().find(|pin| pin.position.is_none()) {
        return PinOffer::Unplaced(pin.name.clone());
    }
    if let Some(port) = session
        .declared_ports
        .iter()
        .find(|port| document.pin(&port.name).is_none())
    {
        return PinOffer::New {
            name: port.name.clone(),
            direction: port.direction,
        };
    }
    let name = (1usize..)
        .map(|number| format!("PIN{number}"))
        .find(|name| document.pin(name).is_none())
        .unwrap_or_default();
    PinOffer::New {
        name,
        direction: PortDirection::InOut,
    }
}

/// The side the Pin tool's pin takes with its tip at `point`: the side R
/// turned it to, else the side of the artwork nearest it.
pub fn pin_tool_side(
    document: &SymbolDocument,
    session: &SymbolEditorSession,
    point: Point,
) -> SymbolPinSide {
    session
        .pin_side
        .unwrap_or_else(|| document.suggested_pin_side(point))
}

/// R while the Pin tool is armed: its pin turns a quarter clockwise from
/// the side it shows at `point`, and keeps that side for the pins after it.
pub fn turn_pin_tool_side(
    session: &mut SymbolEditorSession,
    document: &SymbolDocument,
    point: Option<Point>,
) {
    let showing = point.map_or_else(
        || session.pin_side.unwrap_or(SymbolPinSide::Left),
        |point| pin_tool_side(document, session, point),
    );
    session.pin_side = Some(showing.turned_clockwise());
}

#[cfg(test)]
mod tests;
