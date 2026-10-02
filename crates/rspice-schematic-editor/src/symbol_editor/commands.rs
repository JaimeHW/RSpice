//! Symbol clipboard and command edits over local drafts; the app owns authority and publication.

use super::interaction::{SymbolEditCapabilities, SymbolEditOutcome, SymbolRequestSource};
use super::session::{
    SymbolClipboard, SymbolEditorSession, SymbolSelection, SymbolTool, mirror_point_h_about,
    mirror_point_v_about, mirror_shape_h_about, mirror_shape_v_about, rotate_point_cw_about,
    rotate_shape_cw_about,
};
use rspice_design::symbol::{
    SymbolAttributeKind, SymbolDocument, SymbolEditorMetadata, SymbolShape,
};
use rspice_design_model::{Point, symbol_pin::SymbolPinSide};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolTransform {
    Rotate,
    MirrorHorizontal,
    MirrorVertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolPinTransform {
    Rotate,
    Mirror,
}

#[derive(Debug, Clone)]
pub enum SymbolEditAction {
    Paste {
        clipboard: SymbolClipboard,
        target: Point,
    },
    Delete {
        cut: bool,
    },
    Transform(SymbolTransform),
    TransformPin(SymbolPinTransform),
    FinishPolyline {
        points: Vec<Point>,
    },
}

impl SymbolEditAction {
    pub fn document_only(&self) -> bool {
        matches!(self, Self::FinishPolyline { .. })
    }
}

#[derive(Debug, Clone)]
pub struct SymbolEditRequest {
    pub source: SymbolRequestSource,
    pub selection: SymbolSelection,
    pub tool: SymbolTool,
    pub action: SymbolEditAction,
}

/// Called only on the host's local draft after it validates the request source and authority.
pub fn apply_edit(
    session: &mut SymbolEditorSession,
    document: &mut SymbolDocument,
    metadata: Option<&mut SymbolEditorMetadata>,
    action: SymbolEditAction,
    capabilities: SymbolEditCapabilities,
) -> SymbolEditOutcome {
    let mut outcome = SymbolEditOutcome::default();
    if !capabilities.edit {
        outcome.edit_denied = true;
        return outcome;
    }
    let selection = session.effective_selection();
    match action {
        SymbolEditAction::Paste { clipboard, target } => {
            paste(session, document, clipboard, target, &mut outcome)
        }
        SymbolEditAction::Delete { cut } => {
            delete(session, document, &selection, cut, &mut outcome)
        }
        SymbolEditAction::Transform(transform) => {
            if let Some(metadata) = metadata {
                match transform {
                    SymbolTransform::Rotate => transform_selection(
                        document,
                        metadata,
                        &selection,
                        rotate_point_cw_about,
                        rotate_shape_cw_about,
                        &mut outcome,
                    ),
                    SymbolTransform::MirrorHorizontal => transform_selection(
                        document,
                        metadata,
                        &selection,
                        mirror_point_h_about,
                        mirror_shape_h_about,
                        &mut outcome,
                    ),
                    SymbolTransform::MirrorVertical => transform_selection(
                        document,
                        metadata,
                        &selection,
                        mirror_point_v_about,
                        mirror_shape_v_about,
                        &mut outcome,
                    ),
                }
            }
        }
        SymbolEditAction::TransformPin(transform) => {
            transform_pin(document, &selection, transform, &mut outcome)
        }
        SymbolEditAction::FinishPolyline { points } => {
            if points.len() >= 2 {
                outcome.undo_before = Some(Box::new(document.clone()));
                session.pending_polyline.clear();
                document.body.push(SymbolShape::Polyline {
                    points,
                    closed: false,
                });
                if let Some(index) = document.body.len().checked_sub(1) {
                    session.select_shape(index);
                }
                outcome.changed = true;
            }
        }
    }
    outcome
}

pub fn activate_tool(
    session: &mut SymbolEditorSession,
    tool: SymbolTool,
    document: Option<&SymbolDocument>,
) {
    session.tool = tool;
    match tool {
        SymbolTool::PlacePin => {
            let next = document.and_then(|document| {
                document
                    .pins
                    .iter()
                    .find(|pin| pin.position.is_none())
                    .map(|pin| pin.name.clone())
            });
            if let Some(pin) = next {
                session.select_pin(pin);
            } else {
                session.clear_selection();
            }
        }
        SymbolTool::Line | SymbolTool::Polygon => session.pending_polyline.clear(),
        SymbolTool::Rectangle | SymbolTool::Circle | SymbolTool::Arc => session.shape_start = None,
        SymbolTool::Select | SymbolTool::Text => {}
    }
}

pub fn finish_cancel(session: &mut SymbolEditorSession) {
    session.tool = SymbolTool::Select;
    session.clear_drag_state();
    session.shape_start = None;
    session.marquee_start = None;
    session.marquee_current = None;
}

pub fn clipboard_from_selection(
    document: &SymbolDocument,
    selection: &SymbolSelection,
) -> SymbolClipboard {
    let shapes = selection
        .shapes
        .iter()
        .filter_map(|index| document.body.get(*index).cloned())
        .collect();
    let pins = selection
        .pins
        .iter()
        .filter_map(|name| document.pin(name).cloned())
        .collect();
    SymbolClipboard { pins, shapes }
}

fn unique_symbol_pin_name(document: &SymbolDocument, base: &str) -> String {
    let mut candidate = format!("{base}_copy");
    let mut suffix = 2usize;
    while document
        .pins
        .iter()
        .any(|pin| pin.name.eq_ignore_ascii_case(&candidate))
    {
        candidate = format!("{base}_copy{suffix}");
        suffix += 1;
    }
    candidate
}

fn paste(
    session: &mut SymbolEditorSession,
    document: &mut SymbolDocument,
    clipboard: SymbolClipboard,
    target: Point,
    outcome: &mut SymbolEditOutcome,
) {
    let Some((min, max)) = clipboard.bounds() else {
        return;
    };
    let center = Point::new((min.x + max.x) / 2, (min.y + max.y) / 2);
    let delta = target - center;
    outcome.undo_before = Some(Box::new(document.clone()));
    let mut selection = SymbolSelection::default();
    for mut shape in clipboard.shapes {
        shape.translate(delta);
        document.body.push(shape);
        if let Some(index) = document.body.len().checked_sub(1) {
            selection.shapes.insert(index);
        }
    }
    for mut pin in clipboard.pins {
        pin.name = unique_symbol_pin_name(document, &pin.name);
        if let Some(position) = pin.position.as_mut() {
            *position = *position + delta;
        }
        pin.offset = pin.position.map_or(0, |position| match pin.side() {
            SymbolPinSide::Left | SymbolPinSide::Right => position.y,
            SymbolPinSide::Top | SymbolPinSide::Bottom => position.x,
        });
        selection.pins.insert(pin.name.clone());
        document.pins.push(pin);
    }
    session.set_selection(selection);
    outcome.changed = true;
}

fn delete(
    session: &mut SymbolEditorSession,
    document: &mut SymbolDocument,
    selection: &SymbolSelection,
    cut: bool,
    outcome: &mut SymbolEditOutcome,
) {
    let before = document.clone();
    let mut clipboard = SymbolClipboard::default();
    let mut changed = false;

    for index in selection.shapes.iter().rev().copied() {
        if index < document.body.len() {
            let removed = document.body.remove(index);
            if cut {
                clipboard.shapes.push(removed);
            }
            changed = true;
        }
    }

    let mut retained = Vec::with_capacity(document.pins.len());
    for pin in std::mem::take(&mut document.pins) {
        if selection.pins.contains(&pin.name) {
            if cut {
                clipboard.pins.push(pin);
            }
            changed = true;
        } else {
            retained.push(pin);
        }
    }
    document.pins = retained;

    if cut {
        clipboard.shapes.reverse();
        session.clipboard = clipboard;
    }
    if changed {
        outcome.undo_before = Some(Box::new(before));
        session.clear_selection();
    }
    outcome.changed = changed;
}

fn transform_selection(
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    selection: &SymbolSelection,
    pin_transform: impl Fn(Point, Point) -> Point,
    shape_transform: impl Fn(&mut SymbolShape, Point),
    outcome: &mut SymbolEditOutcome,
) {
    let before = document.clone();
    let origin = document.origin;
    let mut changed = false;
    for index in selection.shapes.iter().copied() {
        if let Some(shape) = document.body.get_mut(index) {
            shape_transform(shape, origin);
            changed = true;
        }
    }
    for name in &selection.pins {
        if let Some(pin) = document.pin_mut(name)
            && let Some(position) = pin.position
        {
            let transformed = pin_transform(position, origin);
            pin.position = Some(transformed);
            pin.offset = match pin.side() {
                SymbolPinSide::Left | SymbolPinSide::Right => transformed.y,
                SymbolPinSide::Top | SymbolPinSide::Bottom => transformed.x,
            };
            changed = true;
        }
    }
    for kind in &selection.attributes {
        if let Some(attribute) = metadata.attribute_mut(*kind) {
            attribute.position = pin_transform(attribute.position, origin);
            match kind {
                SymbolAttributeKind::Reference => {
                    document.name_anchor = attribute.position;
                }
                SymbolAttributeKind::Value => {
                    document.value_anchor = attribute.position;
                }
                SymbolAttributeKind::Model => {}
            }
            changed = true;
        }
    }
    if changed {
        outcome.undo_before = Some(Box::new(before));
    }
    outcome.changed = changed;
}

fn transform_pin(
    document: &mut SymbolDocument,
    selection: &SymbolSelection,
    transform: SymbolPinTransform,
    outcome: &mut SymbolEditOutcome,
) {
    let Some(name) = (selection.pins.len() == 1)
        .then(|| selection.pins.iter().next())
        .flatten()
    else {
        return;
    };
    let bounds = document.body_bounds();
    let before = document.clone();
    let Some(pin) = document.pin_mut(name) else {
        return;
    };
    let side = pin.side();
    let offset = pin.offset();
    let (side, offset) = match transform {
        SymbolPinTransform::Rotate => (
            match side {
                SymbolPinSide::Left => SymbolPinSide::Top,
                SymbolPinSide::Top => SymbolPinSide::Right,
                SymbolPinSide::Right => SymbolPinSide::Bottom,
                SymbolPinSide::Bottom => SymbolPinSide::Left,
            },
            offset,
        ),
        SymbolPinTransform::Mirror => match side {
            SymbolPinSide::Left => (SymbolPinSide::Right, offset),
            SymbolPinSide::Right => (SymbolPinSide::Left, offset),
            SymbolPinSide::Top | SymbolPinSide::Bottom => (side, -offset),
        },
    };
    pin.set_side_and_offset(side, offset, bounds);
    outcome.undo_before = Some(Box::new(before));
    outcome.changed = true;
}

#[cfg(test)]
mod tests;
