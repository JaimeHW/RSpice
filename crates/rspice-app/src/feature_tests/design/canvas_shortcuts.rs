//! The canvases' accessibility summaries name each shortcut as the
//! application binds and labels it, so they read the application's command
//! registry: the schematic canvas's and the symbol canvas's.

use crate::schematic::symbol_editor::symbol_canvas_accessibility_label;
use crate::schematic::view::{
    canvas_accessibility_view, canvas_facts, schematic_accessibility_shortcuts,
};
use crate::state::{
    Component, ComponentType, Point, PortDirection, SymbolDocument, SymbolPin, SymbolShape,
};
use crate::workbench::SymbolTool;
use crate::workbench::app_state::AppState;
use crate::workbench::commands::vocabulary::command_registry;
use rspice_schematic_editor::view::canvas;

#[test]
fn accessibility_description_summarizes_scene_and_tool_without_selection_churn() {
    use crate::state::{Junction, NetLabel, Wire};

    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::new(0, 0)));
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::new(2, vec![Point::new(0, 0), Point::new(10, 0)]));
    state
        .model
        .schematic
        .document_mut_for_test()
        .junctions
        .push(Junction::new(3, Point::new(10, 0)));
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(NetLabel::new(4, Point::new(10, 0), "OUT"));
    state
        .model
        .schematic
        .document_mut_for_test()
        .design_notes
        .push(
            crate::state::DesignNote::new(
                5,
                Point::new(20, 10),
                crate::state::DesignNoteKind::PlainText,
                "Bias network",
            )
            .unwrap(),
        );
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(1);
    state.model.schematic.session.editor.tool = crate::state::Tool::Wire;

    let description = canvas::accessibility_description(
        &canvas_accessibility_view(&canvas_facts(&state)),
        &schematic_accessibility_shortcuts(
            &state.model,
            command_registry(),
            crate::state::command_catalog::CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
        ),
    );

    assert!(description.starts_with(
        "1 component, 1 wire, 0 buses, 0 bus taps, 1 junction, 1 net label, 1 design note, 0 documentation shapes, 0 probe flags."
    ));
    assert!(description.contains("Active tool: Wire."));
    assert!(!description.contains("Selected instance"));
    assert!(!description.contains("item selected"));
    assert!(
        description
            .contains("Arrow keys select the nearest eligible schematic object in each direction.")
    );
    assert!(description.contains("Escape: Cancel active command"));
    assert!(description.contains("Shift+T: Place text or note"));
    // Named as the Edit menu names it.
    assert!(description.contains("S: Stretch"));
    assert!(!description.contains("S: Stretch selection"));
    assert_eq!(
        canvas::selection_accessibility_status(&canvas_accessibility_view(&canvas_facts(&state))),
        "Selected instance unnamed, Resistor."
    );
    state.model.schematic.session.editor.selection.clear();
    assert_eq!(
        canvas::accessibility_description(
            &canvas_accessibility_view(&canvas_facts(&state)),
            &schematic_accessibility_shortcuts(
                &state.model,
                command_registry(),
                crate::state::command_catalog::CommandPlatform::Desktop,
                egui::os::OperatingSystem::Windows,
            )
        ),
        description
    );
    assert_eq!(
        canvas::selection_accessibility_status(&canvas_accessibility_view(&canvas_facts(&state))),
        "No schematic object selected."
    );
}

#[test]
fn symbol_canvas_accessibility_label_reports_editing_contract() {
    let mut state = AppState::default();
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
