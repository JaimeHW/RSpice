//! Where a symbol selection turns about, and how it lines up, spaces out and
//! goes back on the grid: the Arrange commands on a symbol canvas.

use super::session::{SymbolSelection, symbol_shape_bounds};
use super::{snap_point, snap_to_terminal_grid};
use crate::view::selection_layout::{LayoutTarget, SelectionLayoutCommand, layout_deltas};
use rspice_design::symbol::{
    SYMBOL_TERMINAL_GRID, SymbolAttributeKind, SymbolDocument, SymbolEditorMetadata, SymbolShape,
};
use rspice_design_model::Point;

/// One selected object an Arrange command moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymbolArrangeObject {
    /// A pin, by its place in the document's pin list.
    Pin(usize),
    /// A body shape, by its index.
    Shape(usize),
    Label(SymbolAttributeKind),
}

/// Each selected object with its bounds: a placed pin from its tip to where
/// its lead meets the body, a shape by its drawing, a label by its anchor.
/// An unplaced pin and a hidden label are not on the canvas and take no
/// part.
pub fn selection_bounds(
    document: &SymbolDocument,
    metadata: &SymbolEditorMetadata,
    selection: &SymbolSelection,
) -> Vec<(SymbolArrangeObject, Point, Point)> {
    let mut bounds = Vec::with_capacity(selection.len());
    for (index, pin) in document.pins.iter().enumerate() {
        if !selection.pins.contains(&pin.name) {
            continue;
        }
        let (Some(tip), Some(inner)) = (pin.position, pin.lead_inner()) else {
            continue;
        };
        bounds.push((
            SymbolArrangeObject::Pin(index),
            Point::new(tip.x.min(inner.x), tip.y.min(inner.y)),
            Point::new(tip.x.max(inner.x), tip.y.max(inner.y)),
        ));
    }
    for index in &selection.shapes {
        if let Some(shape) = document.body.get(*index) {
            let (min, max) = symbol_shape_bounds(shape);
            bounds.push((SymbolArrangeObject::Shape(*index), min, max));
        }
    }
    for kind in &selection.attributes {
        if let Some(attribute) = metadata.attribute(*kind).filter(|label| label.shown) {
            bounds.push((
                SymbolArrangeObject::Label(*kind),
                attribute.position,
                attribute.position,
            ));
        }
    }
    bounds
}

/// The point a quarter turn or a mirror of `selection` carries everything
/// about. A lone pin turns about its own tip, so it stays where it is wired
/// and only its side changes. Anything else turns about the middle of what
/// is selected, put on the terminal grid so every pin in it lands back on
/// the grid a parent schematic wires to.
pub fn selection_pivot(
    document: &SymbolDocument,
    metadata: &SymbolEditorMetadata,
    selection: &SymbolSelection,
) -> Option<Point> {
    if selection.len() == 1
        && let Some(name) = selection.pins.first()
    {
        return document.pin(name)?.position;
    }
    let bounds = selection_bounds(document, metadata, selection);
    let min_x = bounds.iter().map(|(_, min, _)| min.x).min()?;
    let min_y = bounds.iter().map(|(_, min, _)| min.y).min()?;
    let max_x = bounds.iter().map(|(_, _, max)| max.x).max()?;
    let max_y = bounds.iter().map(|(_, _, max)| max.y).max()?;
    Some(snap_to_terminal_grid(Point::new(
        (min_x + max_x).div_euclid(2),
        (min_y + max_y).div_euclid(2),
    )))
}

/// What an Arrange command did to a symbol, or why it did nothing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolArrangeOutcome {
    /// The objects moved; this many of them.
    Moved(usize),
    /// Nothing needed to move.
    Unchanged,
    /// The command was refused, for the reason given.
    Refused(String),
}

/// Line up or space out the selected pins, shapes and labels by `command`.
/// Pins move by whole terminal steps, artwork and labels by whole steps of
/// `body_grid`, so nothing leaves the grid it was on.
pub fn lay_out_selection(
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    selection: &SymbolSelection,
    command: SelectionLayoutCommand,
    body_grid: i32,
) -> SymbolArrangeOutcome {
    let targets: Vec<LayoutTarget<SymbolArrangeObject>> =
        selection_bounds(document, metadata, selection)
            .into_iter()
            .map(|(object, min, max)| LayoutTarget {
                object,
                min,
                max,
                grid: i64::from(match object {
                    SymbolArrangeObject::Pin(_) => SYMBOL_TERMINAL_GRID,
                    SymbolArrangeObject::Shape(_) | SymbolArrangeObject::Label(_) => body_grid,
                }),
            })
            .collect();
    match layout_deltas(&targets, command) {
        Ok(moves) => move_objects(document, metadata, &moves, command.label()),
        Err(error) => SymbolArrangeOutcome::Refused(format!(
            "{} was not applied: {}.",
            command.label(),
            error.message()
        )),
    }
}

/// Put each selected object's anchor back on the grid: a pin's tip on the
/// terminal grid, a shape's first point, corner or centre and a label's
/// anchor on `body_grid`. Each moves by its own small step.
pub fn snap_selection_to_grid(
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    selection: &SymbolSelection,
    body_grid: i32,
) -> SymbolArrangeOutcome {
    let grid = super::session::SymbolGridSpacing::ALL
        .into_iter()
        .find(|spacing| spacing.model_step() == body_grid)
        .unwrap_or_default();
    let step = |anchor: Point, snapped: Point| snapped - anchor;
    let moves: Vec<(SymbolArrangeObject, Point)> = selection_bounds(document, metadata, selection)
        .into_iter()
        .filter_map(|(object, _, _)| {
            let delta = match object {
                SymbolArrangeObject::Pin(index) => {
                    let tip = document.pins[index].position?;
                    step(tip, snap_to_terminal_grid(tip))
                }
                SymbolArrangeObject::Shape(index) => {
                    let anchor = shape_anchor(&document.body[index]);
                    step(anchor, snap_point(anchor, grid))
                }
                SymbolArrangeObject::Label(kind) => {
                    let anchor = metadata.attribute(kind)?.position;
                    step(anchor, snap_point(anchor, grid))
                }
            };
            (delta != Point::origin()).then_some((object, delta))
        })
        .collect();
    move_objects(document, metadata, &moves, "Snap to grid")
}

/// The point a shape is put on the grid by: a line's first point, a
/// rectangle's top-left corner, a circle's or an arc's centre, an arrow's
/// tip, a text's anchor.
pub fn shape_anchor(shape: &SymbolShape) -> Point {
    match shape {
        SymbolShape::Polyline { points, .. } => points.first().copied().unwrap_or_default(),
        SymbolShape::Rectangle { min, .. } => *min,
        SymbolShape::Circle { center, .. }
        | SymbolShape::Arc { center, .. }
        | SymbolShape::Dot { center, .. } => *center,
        SymbolShape::Curve { curve, .. } => Point::new(
            curve.center[0].round() as i32,
            curve.center[1].round() as i32,
        ),
        SymbolShape::Arrow { tip, .. } => *tip,
        SymbolShape::Text { anchor, .. } | SymbolShape::Label { anchor, .. } => *anchor,
    }
}

/// Move each object by its step as one edit. A move that would put two pins
/// on one point is refused whole: two terminals on a point are one node in
/// every schematic that places the symbol.
fn move_objects(
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    moves: &[(SymbolArrangeObject, Point)],
    label: &str,
) -> SymbolArrangeOutcome {
    let moved: Vec<&(SymbolArrangeObject, Point)> = moves
        .iter()
        .filter(|(_, delta)| *delta != Point::origin())
        .collect();
    if moved.is_empty() {
        return SymbolArrangeOutcome::Unchanged;
    }
    let mut tips: Vec<Option<Point>> = document.pins.iter().map(|pin| pin.position).collect();
    for (object, delta) in &moved {
        if let SymbolArrangeObject::Pin(index) = object
            && let Some(tip) = tips[*index].as_mut()
        {
            *tip = *tip + *delta;
        }
    }
    let placed: Vec<Point> = tips.iter().flatten().copied().collect();
    if placed
        .iter()
        .enumerate()
        .any(|(index, tip)| placed[..index].contains(tip))
    {
        return SymbolArrangeOutcome::Refused(format!(
            "{label} was not applied: it would put two pins on one point."
        ));
    }
    for (object, delta) in &moved {
        match *object {
            SymbolArrangeObject::Pin(index) => document.pins[index].position = tips[index],
            SymbolArrangeObject::Shape(index) => document.body[index].translate(*delta),
            SymbolArrangeObject::Label(kind) => {
                if let Some(attribute) = metadata.attribute_mut(kind) {
                    attribute.position = attribute.position + *delta;
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
        }
    }
    SymbolArrangeOutcome::Moved(moved.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::symbol::SymbolPin;
    use rspice_design_model::port::PortDirection;
    use rspice_design_model::symbol_pin::SymbolPinSide;

    fn document() -> SymbolDocument {
        SymbolDocument {
            pins: vec![
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, 10)))
                    .with_side(SymbolPinSide::Left),
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, -10)))
                    .with_side(SymbolPinSide::Right),
            ],
            body: vec![
                SymbolShape::rectangle(Point::new(-20, -20), Point::new(20, 20)),
                SymbolShape::Circle {
                    center: Point::new(3, 7),
                    radius: 5,
                    style: Default::default(),
                },
            ],
            ..SymbolDocument::default()
        }
    }

    /// A lone pin turns about its tip; a group turns about the middle of
    /// its bounds, on the terminal grid.
    #[test]
    fn a_lone_pin_pivots_on_its_tip_and_a_group_on_its_middle() {
        let document = document();
        let metadata = SymbolEditorMetadata::for_document(&document);
        assert_eq!(
            selection_pivot(&document, &metadata, &SymbolSelection::single_pin("IN")),
            Some(Point::new(-40, 10))
        );
        // The circle alone spans (-2, 2) to (8, 12): its middle (3, 7) goes
        // to the terminal grid point nearest it.
        assert_eq!(
            selection_pivot(&document, &metadata, &SymbolSelection::single_shape(1)),
            Some(Point::new(0, 10))
        );
        let mut both = SymbolSelection::single_pin("IN");
        both.shapes.insert(0);
        // From the IN tip at x -40 to the rectangle's right edge at 20, and
        // from -20 to 20: the middle is (-10, 0).
        assert_eq!(
            selection_pivot(&document, &metadata, &both),
            Some(Point::new(-10, 0))
        );
    }

    /// Align left lines the selection up on its leftmost edge; a pin moves
    /// by whole terminal steps and the outermost object stays put.
    #[test]
    fn align_left_lines_pins_and_shapes_up_on_the_leftmost_edge() {
        let mut document = document();
        let mut metadata = SymbolEditorMetadata::for_document(&document);
        let mut selection = SymbolSelection::single_pin("IN");
        selection.pins.insert("OUT".to_owned());
        selection.shapes.insert(0);
        let outcome = lay_out_selection(
            &mut document,
            &mut metadata,
            &selection,
            SelectionLayoutCommand::AlignLeft,
            10,
        );
        assert_eq!(outcome, SymbolArrangeOutcome::Moved(2));
        assert_eq!(document.pins[0].position, Some(Point::new(-40, 10)));
        // OUT's bounds start where its lead meets the body, at x 20.
        assert_eq!(document.pins[1].position, Some(Point::new(-20, -10)));
        assert_eq!(
            document.body[0],
            SymbolShape::rectangle(Point::new(-40, -20), Point::new(0, 20))
        );
    }

    /// Lining up two pins on one point is refused whole, and too few
    /// objects say how many are needed.
    #[test]
    fn a_layout_that_would_stack_two_pins_is_refused() {
        let mut document = document();
        document.pins[1].position = Some(Point::new(-40, -10));
        document.pins[1].side = SymbolPinSide::Left;
        let mut metadata = SymbolEditorMetadata::for_document(&document);
        let before = document.clone();
        let mut pins = SymbolSelection::single_pin("IN");
        pins.pins.insert("OUT".to_owned());
        let outcome = lay_out_selection(
            &mut document,
            &mut metadata,
            &pins,
            SelectionLayoutCommand::AlignTop,
            10,
        );
        assert_eq!(
            outcome,
            SymbolArrangeOutcome::Refused(
                "Align top was not applied: it would put two pins on one point.".to_owned()
            )
        );
        assert_eq!(document, before);
        let outcome = lay_out_selection(
            &mut document,
            &mut metadata,
            &pins,
            SelectionLayoutCommand::DistributeVertical,
            10,
        );
        assert!(
            matches!(outcome, SymbolArrangeOutcome::Refused(ref reason) if reason.contains("at least three")),
            "{outcome:?}"
        );
    }

    /// Snap to grid moves each object by its own anchor: the circle's
    /// centre onto the body grid, an off-grid pin tip onto the terminal grid.
    #[test]
    fn snap_to_grid_puts_each_anchor_back_on_its_grid() {
        let mut document = document();
        document.pins[0].position = Some(Point::new(-37, 12));
        let mut metadata = SymbolEditorMetadata::for_document(&document);
        let mut selection = SymbolSelection::single_pin("IN");
        selection.shapes.extend([0, 1]);
        let outcome = snap_selection_to_grid(&mut document, &mut metadata, &selection, 5);
        assert_eq!(outcome, SymbolArrangeOutcome::Moved(2));
        assert_eq!(document.pins[0].position, Some(Point::new(-40, 10)));
        assert_eq!(
            document.body[1],
            SymbolShape::Circle {
                center: Point::new(5, 5),
                radius: 5,
                style: Default::default(),
            }
        );
        assert_eq!(
            snap_selection_to_grid(&mut document, &mut metadata, &selection, 5),
            SymbolArrangeOutcome::Unchanged
        );
    }
}
