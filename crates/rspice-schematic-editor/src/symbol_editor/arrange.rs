//! Where a symbol selection turns about.

use super::session::{SymbolSelection, symbol_shape_bounds};
use super::snap_to_terminal_grid;
use rspice_design::symbol::{SymbolAttributeKind, SymbolDocument, SymbolEditorMetadata};
use rspice_design_model::Point;

/// One selected object, by what it is.
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

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::symbol::{SymbolPin, SymbolShape};
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
}
