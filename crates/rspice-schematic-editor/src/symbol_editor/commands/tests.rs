//! Symbol command geometry and clipboard behavior independent of the app.

use super::*;
use rspice_design::symbol::SymbolPin;
use rspice_design_model::port::PortDirection;

#[test]
fn symbol_clipboard_copies_selected_shapes_and_pins() {
    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-10, 0))),
            SymbolPin::new("TRIM", PortDirection::InOut, Some(Point::new(0, 10))),
        ],
        body: vec![SymbolShape::Dot {
            center: Point::origin(),
            radius: 3,
        }],
        ..SymbolDocument::default()
    };
    let mut selection = SymbolSelection::default();
    selection.pins.insert("IN".to_owned());
    selection.pins.insert("TRIM".to_owned());
    selection.shapes.insert(0);

    let clipboard = clipboard_from_selection(&document, &selection);

    assert_eq!(clipboard.shapes.len(), 1);
    assert_eq!(
        clipboard
            .pins
            .iter()
            .map(|pin| pin.name.as_str())
            .collect::<Vec<_>>(),
        vec!["IN", "TRIM"]
    );
}
