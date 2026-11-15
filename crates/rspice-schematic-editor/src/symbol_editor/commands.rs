//! Symbol clipboard and command edits over local drafts; the app owns authority and publication.

use super::arrange::{
    SymbolArrangeOutcome, lay_out_selection, selection_pivot, snap_selection_to_grid,
};
use super::handles;
use super::interaction::{SymbolEditCapabilities, SymbolEditOutcome, SymbolRequestSource};
use super::session::{
    SymbolClipboard, SymbolEditorSession, SymbolSelection, SymbolTool, label_name,
    mirror_point_h_about, mirror_point_v_about, mirror_shape_h_about, mirror_shape_v_about,
    rotate_point_ccw_about, rotate_point_cw_about, rotate_shape_ccw_about, rotate_shape_cw_about,
    selection_subject,
};
use crate::view::selection_layout::SelectionLayoutCommand;
use rspice_design::symbol::{
    MAX_SYMBOL_PIN_NAME_BYTES, SymbolAttributeKind, SymbolDocument, SymbolEditorMetadata,
    SymbolShape, SymbolTextAlign, SymbolTextSize,
};
use rspice_design_model::{Point, port::PortSpec, symbol_pin::SymbolPinSide};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolTransform {
    Rotate,
    RotateCounterClockwise,
    MirrorHorizontal,
    MirrorVertical,
}

/// How a quarter turn or a mirror names itself and moves a selection about a
/// pivot: its points, each pin's side, and each shape.
struct SelectionMotion {
    verb: &'static str,
    point: fn(Point, Point) -> Point,
    side: fn(SymbolPinSide) -> SymbolPinSide,
    shape: fn(&mut SymbolShape, Point),
}

impl SymbolTransform {
    fn motion(self) -> SelectionMotion {
        match self {
            Self::Rotate => SelectionMotion {
                verb: "Rotate",
                point: rotate_point_cw_about,
                side: SymbolPinSide::turned_clockwise,
                shape: rotate_shape_cw_about,
            },
            Self::RotateCounterClockwise => SelectionMotion {
                verb: "Rotate",
                point: rotate_point_ccw_about,
                side: SymbolPinSide::turned_counter_clockwise,
                shape: rotate_shape_ccw_about,
            },
            Self::MirrorHorizontal => SelectionMotion {
                verb: "Mirror",
                point: mirror_point_h_about,
                side: SymbolPinSide::mirrored_horizontally,
                shape: mirror_shape_h_about,
            },
            Self::MirrorVertical => SelectionMotion {
                verb: "Mirror",
                point: mirror_point_v_about,
                side: SymbolPinSide::mirrored_vertically,
                shape: mirror_shape_v_about,
            },
        }
    }
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
    /// Line up or space out the selected pins, shapes and labels.
    Layout(SelectionLayoutCommand),
    /// Put each selected object back on the grid.
    SnapToGrid,
    /// Place the text typed into the text tool's entry at `anchor`.
    PlaceText {
        anchor: Point,
        text: String,
    },
    /// Name `pin`, which the Pin tool has just placed, `name`: what was
    /// typed into its entry.
    RenamePlacedPin {
        pin: String,
        name: String,
    },
}

impl SymbolEditAction {
    pub fn document_only(&self) -> bool {
        matches!(self, Self::PlaceText { .. } | Self::RenamePlacedPin { .. })
    }
}

/// Why `name` cannot name `pin` in `document`, if it cannot: the rules the
/// inspector's Name row holds a pin to.
pub fn pin_name_refusal(document: &SymbolDocument, pin: &str, name: &str) -> Option<&'static str> {
    if name.is_empty() {
        Some("A pin needs a name.")
    } else if name.len() > MAX_SYMBOL_PIN_NAME_BYTES {
        Some("That name is longer than a pin name can be.")
    } else if name.chars().any(char::is_control) {
        Some("A pin name cannot hold control characters.")
    } else if document
        .pins
        .iter()
        .any(|other| !other.name.eq_ignore_ascii_case(pin) && other.name.eq_ignore_ascii_case(name))
    {
        Some("Another pin already has that name.")
    } else {
        None
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
        // Delete with a point of the selected line picked takes that point
        // out; the line stays.
        SymbolEditAction::Delete { cut: false }
            if session
                .selected_vertex
                .is_some_and(|(shape, _)| selection == SymbolSelection::single_shape(shape)) =>
        {
            delete_vertex(session, document, &mut outcome)
        }
        SymbolEditAction::Delete { cut } => {
            delete(session, document, metadata, &selection, cut, &mut outcome)
        }
        SymbolEditAction::Transform(transform) => {
            if let Some(metadata) = metadata {
                transform_selection(
                    document,
                    metadata,
                    &selection,
                    &transform.motion(),
                    &mut outcome,
                );
            }
        }
        SymbolEditAction::Layout(command) => {
            if let Some(metadata) = metadata {
                let before = document.clone();
                let arranged =
                    lay_out_selection(document, metadata, &selection, command, body_grid(session));
                let unchanged = format!("{} produced no geometry change.", command.label());
                arrange_outcome(before, arranged, command.label(), unchanged, &mut outcome);
            }
        }
        SymbolEditAction::SnapToGrid => {
            if let Some(metadata) = metadata {
                let before = document.clone();
                let arranged =
                    snap_selection_to_grid(document, metadata, &selection, body_grid(session));
                if let SymbolArrangeOutcome::Moved(moved) = arranged {
                    outcome.report = Some(format!(
                        "Snapped {} to the grid.",
                        rspice_ui_kit::accessibility::counted(moved, "object", "objects")
                    ));
                }
                let unchanged = "The selection is on the grid already.".to_owned();
                arrange_outcome(before, arranged, "Snap to grid", unchanged, &mut outcome);
            }
        }
        SymbolEditAction::RenamePlacedPin { pin, name } => {
            session.pin_name_draft = None;
            if let Some(index) = document.pins.iter().position(|placed| placed.name == pin)
                && name != pin
                && pin_name_refusal(document, &pin, &name).is_none()
            {
                outcome.undo_before = Some(Box::new(document.clone()));
                outcome.step = format!("Rename pin {pin}");
                document.pins[index].name = name.clone();
                session.select_pin(name);
                outcome.changed = true;
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
                    rotation: Default::default(),
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

/// Arm `tool`. Whatever the last tool had part drawn is let go. The Pin
/// tool keeps `ports`, the ports the interface declares, to offer the ones
/// that have no pin before any pin of its own naming.
pub fn activate_tool(
    session: &mut SymbolEditorSession,
    tool: SymbolTool,
    document: Option<&SymbolDocument>,
    ports: &[PortSpec],
) {
    session.tool = tool;
    session.clear_draft();
    session.pin_side = None;
    if tool == SymbolTool::PlacePin {
        session.declared_ports = ports.to_vec();
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

/// Escape on the canvas: a handle Stretch holds is let go where it was;
/// the shape being drawn is let go and its tool stays armed; with nothing
/// part drawn, the tool is put down for Select.
pub fn finish_cancel(session: &mut SymbolEditorSession) {
    if session.stretch.take().is_some() {
        return;
    }
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

/// The name a pasted or duplicated pin takes: the first free one counting
/// up from the number `base` ends in, ahead of any bus range, its digits as
/// wide as they were. `D3` gives `D4`, `CLK` gives `CLK1` and `A0[7:0]`
/// gives `A1[7:0]`.
fn next_free_pin_name(document: &SymbolDocument, base: &str) -> String {
    let (stem, range) = match base.find('[') {
        Some(at) if base.ends_with(']') => base.split_at(at),
        _ => (base, ""),
    };
    let prefix = stem.trim_end_matches(|character: char| character.is_ascii_digit());
    let digits = &stem[prefix.len()..];
    let width = digits.len();
    let first = digits
        .parse::<u64>()
        .map_or(1, |number| number.saturating_add(1));
    (first..)
        .map(|number| format!("{prefix}{number:0width$}{range}"))
        .find(|candidate| document.pin(candidate).is_none())
        .unwrap_or_else(|| base.to_owned())
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
        pin.name = next_free_pin_name(document, &pin.name);
        if let Some(position) = pin.position.as_mut() {
            *position = *position + delta;
        }
        selection.pins.insert(pin.name.clone());
        document.pins.push(pin);
    }
    session.set_selection(selection);
    outcome.changed = true;
}

/// Take the picked point out of its line, as one edit, or say why it stays:
/// a line keeps two points and a polygon three corners.
fn delete_vertex(
    session: &mut SymbolEditorSession,
    document: &mut SymbolDocument,
    outcome: &mut SymbolEditOutcome,
) {
    let Some((index, vertex)) = session.selected_vertex else {
        return;
    };
    let Some(shape) = document.body.get(index) else {
        return;
    };
    let mut trimmed = shape.clone();
    match handles::remove_vertex(&mut trimmed, vertex) {
        Ok(()) => {
            outcome.undo_before = Some(Box::new(document.clone()));
            outcome.step = "Delete vertex".to_owned();
            document.body[index] = trimmed;
            session.selected_vertex = None;
            outcome.changed = true;
        }
        Err(refusal) => outcome.refusal = Some(refusal.to_owned()),
    }
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

/// The step artwork and labels move by on this canvas: the grid drawn, when
/// the author snaps to it, else a single unit.
fn body_grid(session: &SymbolEditorSession) -> i32 {
    if session.snap_to_grid {
        session.grid_spacing.model_step()
    } else {
        1
    }
}

/// Carry an Arrange command's result into `outcome`: one undo step named
/// `step` when it moved something, else what the host tells the author.
fn arrange_outcome(
    before: SymbolDocument,
    arranged: SymbolArrangeOutcome,
    step: &str,
    unchanged: String,
    outcome: &mut SymbolEditOutcome,
) {
    match arranged {
        SymbolArrangeOutcome::Moved(_) => {
            outcome.step = step.to_owned();
            outcome.undo_before = Some(Box::new(before));
            outcome.changed = true;
        }
        SymbolArrangeOutcome::Unchanged => outcome.refusal = Some(unchanged),
        SymbolArrangeOutcome::Refused(reason) => outcome.refusal = Some(reason),
    }
}

/// Turn or mirror the selection about its pivot (see [`selection_pivot`]).
/// Pins turn their sides with it, so every lead still runs from its tip into
/// the body that turned with it. A motion that changes nothing, such as a
/// lone side pin mirrored across its own lead, is no edit.
fn transform_selection(
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    selection: &SymbolSelection,
    motion: &SelectionMotion,
    outcome: &mut SymbolEditOutcome,
) {
    let Some(pivot) = selection_pivot(document, metadata, selection) else {
        return;
    };
    let before = document.clone();
    let labels_before = metadata.attributes.clone();
    for index in selection.shapes.iter().copied() {
        if let Some(shape) = document.body.get_mut(index) {
            (motion.shape)(shape, pivot);
        }
    }
    for name in &selection.pins {
        if let Some(pin) = document.pin_mut(name)
            && let Some(position) = pin.position
        {
            pin.position = Some((motion.point)(position, pivot));
            pin.side = (motion.side)(pin.side);
        }
    }
    for kind in &selection.attributes {
        if let Some(attribute) = metadata.attribute_mut(*kind) {
            attribute.position = (motion.point)(attribute.position, pivot);
            match kind {
                SymbolAttributeKind::Reference => {
                    document.name_anchor = attribute.position;
                }
                SymbolAttributeKind::Value => {
                    document.value_anchor = attribute.position;
                }
                SymbolAttributeKind::Model => {}
            }
        }
    }
    let changed = *document != before || metadata.attributes != labels_before;
    if changed {
        outcome.step = format!("{} {}", motion.verb, selection_subject(&before, selection));
        outcome.undo_before = Some(Box::new(before));
    }
    outcome.changed = changed;
}

#[cfg(test)]
mod tests;
