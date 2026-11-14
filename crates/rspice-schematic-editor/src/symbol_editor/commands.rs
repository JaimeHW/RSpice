//! Symbol clipboard and command edits over local drafts; the app owns authority and publication.

use super::interaction::{SymbolEditCapabilities, SymbolEditOutcome, SymbolRequestSource};
use super::session::{
    SymbolClipboard, SymbolEditorSession, SymbolSelection, SymbolTool, label_name,
    mirror_point_h_about, mirror_point_v_about, mirror_shape_h_about, mirror_shape_v_about,
    rotate_point_ccw_about, rotate_point_cw_about, rotate_shape_ccw_about, rotate_shape_cw_about,
    selection_subject,
};
use rspice_design::symbol::{
    SymbolAttributeKind, SymbolDocument, SymbolEditorMetadata, SymbolShape, SymbolTextAlign,
    SymbolTextSize,
};
use rspice_design_model::{Point, symbol_pin::SymbolPinSide};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolTransform {
    Rotate,
    RotateCounterClockwise,
    MirrorHorizontal,
    MirrorVertical,
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
    /// Place the text typed into the text tool's entry at `anchor`.
    PlaceText {
        anchor: Point,
        text: String,
    },
}

impl SymbolEditAction {
    pub fn document_only(&self) -> bool {
        matches!(self, Self::PlaceText { .. })
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
            delete(session, document, metadata, &selection, cut, &mut outcome)
        }
        SymbolEditAction::Transform(transform) => {
            if let Some(metadata) = metadata {
                match transform {
                    SymbolTransform::Rotate => transform_selection(
                        document,
                        metadata,
                        &selection,
                        "Rotate",
                        rotate_point_cw_about,
                        SymbolPinSide::turned_clockwise,
                        rotate_shape_cw_about,
                        &mut outcome,
                    ),
                    SymbolTransform::RotateCounterClockwise => transform_selection(
                        document,
                        metadata,
                        &selection,
                        "Rotate",
                        rotate_point_ccw_about,
                        SymbolPinSide::turned_counter_clockwise,
                        rotate_shape_ccw_about,
                        &mut outcome,
                    ),
                    SymbolTransform::MirrorHorizontal => transform_selection(
                        document,
                        metadata,
                        &selection,
                        "Mirror",
                        mirror_point_h_about,
                        SymbolPinSide::mirrored_horizontally,
                        mirror_shape_h_about,
                        &mut outcome,
                    ),
                    SymbolTransform::MirrorVertical => transform_selection(
                        document,
                        metadata,
                        &selection,
                        "Mirror",
                        mirror_point_v_about,
                        SymbolPinSide::mirrored_vertically,
                        mirror_shape_v_about,
                        &mut outcome,
                    ),
                }
            }
        }
        SymbolEditAction::PlaceText { anchor, text } => {
            session.text_draft = None;
            if !text.trim().is_empty() {
                outcome.undo_before = Some(Box::new(document.clone()));
                outcome.step = "Add text".to_owned();
                document.body.push(SymbolShape::Text {
                    anchor,
                    text,
                    size: SymbolTextSize::default(),
                    align: SymbolTextAlign::default(),
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

/// Arm `tool`. Whatever the last tool had part drawn is let go.
pub fn activate_tool(
    session: &mut SymbolEditorSession,
    tool: SymbolTool,
    document: Option<&SymbolDocument>,
) {
    session.tool = tool;
    session.clear_draft();
    if tool == SymbolTool::PlacePin {
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
}

/// Escape on the canvas: the shape being drawn is let go and its tool stays
/// armed; with nothing part drawn, the tool is put down for Select.
pub fn finish_cancel(session: &mut SymbolEditorSession) {
    session.clear_drag_state();
    session.marquee_start = None;
    session.marquee_current = None;
    if session.has_draft() {
        session.clear_draft();
    } else {
        session.tool = SymbolTool::Select;
    }
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
    outcome.step = "Paste".to_owned();
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
        selection.pins.insert(pin.name.clone());
        document.pins.push(pin);
    }
    session.set_selection(selection);
    outcome.changed = true;
}

/// Delete the selected shapes and pins. A selected Reference, Value or Model
/// label is part of every symbol, so deleting it hides it instead.
fn delete(
    session: &mut SymbolEditorSession,
    document: &mut SymbolDocument,
    metadata: Option<&mut SymbolEditorMetadata>,
    selection: &SymbolSelection,
    cut: bool,
    outcome: &mut SymbolEditOutcome,
) {
    let before = document.clone();
    let mut clipboard = SymbolClipboard::default();
    let mut changed = false;

    if let Some(metadata) = metadata {
        for kind in &selection.attributes {
            if let Some(attribute) = metadata.attribute_mut(*kind)
                && attribute.shown
            {
                attribute.shown = false;
                changed = true;
            }
        }
    }

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
        let only_labels = selection.pins.is_empty() && selection.shapes.is_empty();
        outcome.step = match (only_labels, selection.attributes.first()) {
            (true, Some(kind)) if selection.attributes.len() == 1 => {
                format!("Hide {} label", label_name(*kind).to_lowercase())
            }
            (true, _) => "Hide labels".to_owned(),
            (false, _) => {
                let verb = if cut { "Cut" } else { "Delete" };
                format!("{verb} {}", selection_subject(&before, selection))
            }
        };
        outcome.undo_before = Some(Box::new(before));
        session.clear_selection();
    }
    outcome.changed = changed;
}

fn transform_selection(
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    selection: &SymbolSelection,
    verb: &str,
    pin_transform: impl Fn(Point, Point) -> Point,
    side_transform: impl Fn(SymbolPinSide) -> SymbolPinSide,
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
            // The pin turns with the selection: its lead still runs toward
            // the body that turned with it.
            pin.position = Some(pin_transform(position, origin));
            pin.side = side_transform(pin.side);
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
        outcome.step = format!("{verb} {}", selection_subject(&before, selection));
        outcome.undo_before = Some(Box::new(before));
    }
    outcome.changed = changed;
}

#[cfg(test)]
mod tests;
