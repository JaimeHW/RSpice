//! Cases for symbol canvas app composition.

use super::*;
use crate::state::{Point, PortDirection, SymbolPin, SymbolShape};
use crate::workbench::SymbolTool;
use crate::workbench::design::test_host::{DesignTestHost, canvas_facts};

#[test]
fn symbol_canvas_accessibility_label_reports_editing_contract() {
    let mut state = DesignTestHost::default();
    state.design.symbol.editor.tool = SymbolTool::Circle;
    state.design.symbol.editor.select_shape(0);
    let document = SymbolDocument {
        pins: vec![SymbolPin::new(
            "OUT",
            PortDirection::Out,
            Some(Point::new(20, 0)),
        )],
        body: vec![SymbolShape::Circle {
            center: Point::origin(),
            radius: 10,
        }],
        ..SymbolDocument::default()
    };

    let label = symbol_canvas_accessibility_label(
        &document,
        &canvas_facts(&state),
        false,
        crate::state::command_catalog::CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );

    assert!(label.starts_with(
        "Symbol editor canvas. 1 shape; 1 pin, 1 pin placed; 1 item selected. Active tool: Circle. Editable."
    ));
    assert!(label.contains("P: Place symbol pin"));
    assert!(label.contains("Escape: Cancel active command"));
}
