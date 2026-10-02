//! Symbol pointer input and local edit drafts. The host validates requests and publishes edits.

use egui::Pos2;
use rspice_app_types::product::ProjectId;
use rspice_design::occurrence::DocumentOccurrence;
use rspice_design::symbol::{
    SYMBOL_TERMINAL_GRID, SymbolAttributeKind, SymbolDocument, SymbolEditorMetadata, SymbolShape,
    SymbolTextAlign, SymbolTextSize, pin_side_against_body,
};
use rspice_design_model::{
    Point, cell_view::CellViewRef, port::PortDirection, symbol_pin::SymbolPinSide,
};

use super::session::{SymbolEditorSession, SymbolSelection, SymbolTool};
use super::{
    SymbolViewport, hit_label, hit_origin, hit_pin, hit_shape, snap_point, snap_to_terminal_grid,
};

/// Identity of the actual symbol view and the registry revision used to resolve an event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolRequestSource {
    pub project: ProjectId,
    pub document: CellViewRef,
    pub occurrence: Option<DocumentOccurrence>,
    pub design_epoch: u64,
    pub document_epoch: u64,
    pub library_revision: u64,
}

impl SymbolRequestSource {
    fn same_view(&self, other: &Self) -> bool {
        self.project == other.project
            && self.document == other.document
            && self.occurrence == other.occurrence
            && self.design_epoch == other.design_epoch
            && self.document_epoch == other.document_epoch
    }
}

/// Only the pointer facts consumed by the symbol canvas; no global app access.
#[derive(Debug, Clone, Copy, Default)]
pub struct SymbolCanvasInput {
    pub pointer: Pos2,
    pub secondary_clicked: bool,
    pub drag_started: bool,
    pub dragged: bool,
    pub drag_stopped: bool,
    pub clicked: bool,
    pub extend_selection: bool,
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
    let input = SymbolCanvasInput {
        pointer: response.interact_pointer_pos()?,
        secondary_clicked: response.secondary_clicked(),
        drag_started: response.drag_started_by(egui::PointerButton::Primary),
        dragged: response.dragged_by(egui::PointerButton::Primary),
        drag_stopped: response.drag_stopped_by(egui::PointerButton::Primary),
        clicked: response.clicked_by(egui::PointerButton::Primary),
        extend_selection: response.ctx.input(|input| input.modifiers.shift),
    };
    if !(input.secondary_clicked
        || input.drag_started
        || input.dragged
        || input.drag_stopped
        || input.clicked)
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
        session.pending_polyline.clear();
        session.shape_start = None;
        if !same_view {
            session.clear_selection();
        }
    }
    session.canvas_source = Some(source);
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
    pub edit_denied: bool,
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

    fn record_undo(&mut self, document: &SymbolDocument) {
        if self.outcome.undo_before.is_none() {
            self.outcome.undo_before = Some(Box::new(document.clone()));
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

    if input.secondary_clicked
        && matches!(edit.session.tool, SymbolTool::Line | SymbolTool::Polygon)
        && finish_pending_polyline(edit, document)
    {
        return true;
    }

    if input.drag_started {
        // Grabbing something that is already part of a multi-object
        // selection moves the whole selection. Grabbing anything else
        // reduces the selection to it first, which is what makes a
        // mis-grab recoverable rather than a silent group move.
        if grab_belongs_to_group(edit, document, editor, viewport, pointer) {
            edit.session.dragging_group = Some(body_point);
            edit.session.drag_undo_recorded = false;
        } else if let Some(pin) = hit_pin(document, viewport, pointer) {
            edit.session.select_pin(pin.clone());
            edit.session.dragging_pin = Some(pin);
            edit.session.drag_undo_recorded = false;
        } else if let Some(kind) = hit_label(editor, viewport, pointer) {
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
        && let Some(last_point) = edit.session.dragging_group
    {
        if edit.deny_read_only_edit() {
            edit.session.clear_drag_state();
            return false;
        }
        let delta = body_point - last_point;
        if delta == Point::origin() {
            return false;
        }
        record_drag_symbol_edit(edit, document);
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
        if document.pin(&name).and_then(|pin| pin.position) != Some(terminal_point) {
            record_drag_symbol_edit(edit, document);
            let bounds = document.body_bounds();
            if let Some(pin) = document.pin_mut(&name) {
                let side = inferred_side_from_point(terminal_point, bounds);
                let offset = match side {
                    SymbolPinSide::Left | SymbolPinSide::Right => terminal_point.y,
                    SymbolPinSide::Top | SymbolPinSide::Bottom => terminal_point.x,
                };
                pin.set_side_and_offset(side, offset, bounds);
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
            record_drag_symbol_edit(edit, document);
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
            record_drag_symbol_edit(edit, document);
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
            record_drag_symbol_edit(edit, document);
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
            edit.session
                .set_selection(SymbolSelection::in_rect(document, editor, start, end));
        }
        edit.session.clear_drag_state();
    }

    if !input.clicked {
        return false;
    }

    match edit.session.tool {
        SymbolTool::Select => {
            let extend = input.extend_selection;
            if let Some(pin) = hit_pin(document, viewport, pointer) {
                toggle_or_select(
                    edit,
                    extend,
                    |selection| selection.toggle_pin(&pin),
                    || SymbolSelection::single_pin(pin.clone()),
                );
            } else if let Some(kind) = hit_label(editor, viewport, pointer) {
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
        SymbolTool::PlacePin => place_selected_pin(edit, document, terminal_point),
        SymbolTool::Line | SymbolTool::Polygon => add_polyline_point(edit, body_point),
        SymbolTool::Rectangle => add_rectangle(edit, document, body_point),
        SymbolTool::Circle => add_round_shape(edit, document, body_point, false),
        SymbolTool::Arc => add_round_shape(edit, document, body_point, true),
        SymbolTool::Text => add_text(edit, document, body_point),
    }
}

fn record_drag_symbol_edit(edit: &mut CanvasEdit<'_>, document: &SymbolDocument) {
    if edit.session.drag_undo_recorded {
        return;
    }
    edit.record_undo(document);
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

/// Whether the object under `pointer` is one of several already selected.
///
/// A single-object selection is not a group: dragging it must keep the
/// established single-object behaviour, including its side/offset snapping.
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
    if let Some(kind) = hit_label(editor, viewport, pointer) {
        return selection.attributes.contains(&kind);
    }
    hit_shape(document, viewport, pointer).is_some_and(|index| selection.shapes.contains(&index))
}

/// Move every selected object by `delta` as one edit.
///
/// Terminals travel by the same delta rounded to the terminal pitch, so a
/// group moved on a fine display grid arrives with its pins still on the
/// lattice a parent schematic wires to. Each pin keeps the side it was
/// authored on: a group move is a translation, and re-deriving the edge from
/// the new coordinates would turn a lead through ninety degrees whenever the
/// selection carried the body past it.
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
        let moved = position + pin_delta;
        let Some(pin) = document.pin_mut(name) else {
            continue;
        };
        let side = pin.side();
        pin.side = Some(side);
        pin.position = Some(moved);
        pin.offset = match side {
            SymbolPinSide::Left | SymbolPinSide::Right => moved.y,
            SymbolPinSide::Top | SymbolPinSide::Bottom => moved.x,
        };
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

fn place_selected_pin(
    edit: &mut CanvasEdit<'_>,
    document: &mut SymbolDocument,
    point: Point,
) -> bool {
    let selected = edit
        .session
        .selected_pin
        .clone()
        .or_else(|| next_unplaced_pin(document));
    let Some(name) = selected else {
        return false;
    };
    if edit.deny_read_only_edit() {
        return false;
    }
    if document.pin(&name).is_none() {
        return false;
    }
    let changed = document.pin(&name).and_then(|pin| pin.position) != Some(point);
    if changed {
        edit.record_undo(document);
    }
    let bounds = document.body_bounds();
    if let Some(pin) = document.pin_mut(&name) {
        let side = inferred_side_from_point(point, bounds);
        let offset = match side {
            SymbolPinSide::Left | SymbolPinSide::Right => point.y,
            SymbolPinSide::Top | SymbolPinSide::Bottom => point.x,
        };
        pin.set_side_and_offset(side, offset, bounds);
        edit.session.select_pin(name);
        edit.session.tool = SymbolTool::Select;
        return changed;
    }
    false
}

fn add_polyline_point(edit: &mut CanvasEdit<'_>, point: Point) -> bool {
    if edit.deny_read_only_edit() {
        return false;
    }
    edit.session.pending_polyline.push(point);
    false
}

fn finish_pending_polyline(edit: &mut CanvasEdit<'_>, document: &mut SymbolDocument) -> bool {
    if edit.session.pending_polyline.len() < 2 {
        return false;
    }
    if edit.deny_read_only_edit() {
        return false;
    }
    edit.record_undo(document);
    let points = std::mem::take(&mut edit.session.pending_polyline);
    document.body.push(SymbolShape::Polyline {
        points,
        closed: matches!(edit.session.tool, SymbolTool::Polygon),
    });
    if let Some(index) = document.body.len().checked_sub(1) {
        edit.session.select_shape(index);
    }
    edit.session.tool = SymbolTool::Select;
    true
}

/// The edge a dropped terminal belongs to.
///
/// A terminal placed clear of the body belongs to the edge it stands off
/// from — not merely the edge whose coordinate it happens to sit closest to,
/// which on a tall body reads an outer left pin as a rail.
fn inferred_side_from_point(point: Point, bounds: (Point, Point)) -> SymbolPinSide {
    pin_side_against_body(point, PortDirection::InOut, Some(bounds))
}

fn add_rectangle(edit: &mut CanvasEdit<'_>, document: &mut SymbolDocument, point: Point) -> bool {
    if edit.deny_read_only_edit() {
        return false;
    }
    let Some(start) = edit.session.shape_start.take() else {
        edit.session.shape_start = Some(point);
        return false;
    };
    if start == point {
        edit.session.shape_start = Some(start);
        return false;
    }
    edit.record_undo(document);
    document.body.push(SymbolShape::Polyline {
        points: vec![
            start,
            Point::new(point.x, start.y),
            point,
            Point::new(start.x, point.y),
        ],
        closed: true,
    });
    if let Some(index) = document.body.len().checked_sub(1) {
        edit.session.select_shape(index);
    }
    edit.session.tool = SymbolTool::Select;
    true
}

fn add_text(edit: &mut CanvasEdit<'_>, document: &mut SymbolDocument, point: Point) -> bool {
    if edit.deny_read_only_edit() {
        return false;
    }
    edit.record_undo(document);
    document.body.push(SymbolShape::Text {
        anchor: point,
        text: "Text".to_owned(),
        size: SymbolTextSize::default(),
        align: SymbolTextAlign::default(),
    });
    if let Some(index) = document.body.len().checked_sub(1) {
        edit.session.select_shape(index);
    }
    edit.session.tool = SymbolTool::Select;
    true
}

fn add_round_shape(
    edit: &mut CanvasEdit<'_>,
    document: &mut SymbolDocument,
    point: Point,
    arc: bool,
) -> bool {
    if edit.deny_read_only_edit() {
        return false;
    }
    if let Some(center) = edit.session.shape_start.take() {
        edit.record_undo(document);
        let radius = center
            .distance_squared(point)
            .isqrt()
            .max(SYMBOL_TERMINAL_GRID);
        let shape = if arc {
            SymbolShape::Arc {
                center,
                radius,
                start_degrees: 0,
                sweep_degrees: 180,
            }
        } else {
            SymbolShape::Circle { center, radius }
        };
        document.body.push(shape);
        if let Some(index) = document.body.len().checked_sub(1) {
            edit.session.select_shape(index);
        }
        edit.session.tool = SymbolTool::Select;
        true
    } else {
        edit.session.shape_start = Some(point);
        false
    }
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

fn next_unplaced_pin(document: &SymbolDocument) -> Option<String> {
    document
        .pins
        .iter()
        .find(|pin| pin.position.is_none())
        .map(|pin| pin.name.clone())
}

#[cfg(test)]
mod tests;
