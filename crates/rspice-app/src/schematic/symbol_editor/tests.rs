//! Cases for symbol canvas app composition.

use super::*;
use crate::state::{Point, PortDirection, SymbolPin, SymbolShape};
use crate::workbench::SymbolTool;

#[test]
fn symbol_editor_has_no_direct_product_key_bypass() {
    for source in [
        include_str!("../symbol_editor.rs"),
        include_str!("../../../../rspice-schematic-editor/src/symbol_editor/interaction.rs"),
    ] {
        assert!(!source.contains(concat!("input.", "key_pressed(")));
        assert!(!source.contains(concat!("ctx.input_mut(|input| input.", "consume_key")));
        assert!(!source.contains(concat!("Use S", " for select")));
        assert!(!source.contains(concat!("Escape", " to cancel")));
    }
}

#[test]
fn symbol_canvas_accessibility_label_reports_editing_contract() {
    let mut state = AppState::default();
    state.ui.symbol.editor.tool = SymbolTool::Circle;
    state.ui.symbol.editor.select_shape(0);
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
        &state,
        false,
        crate::workbench::commands::vocabulary::CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );

    assert!(label.starts_with(
        "Symbol editor canvas. 1 shape; 1 pin, 1 pin placed; 1 item selected. Active tool: Circle. Editable."
    ));
    assert!(label.contains("P: Place symbol pin"));
    assert!(label.contains("Escape: Cancel active command"));
}
