//! Commands that arm a placement tool or open a placement transaction.
//!
//! Arming is not editing. None of these may touch the document at the moment
//! it runs: the schematic changes when the placement completes, so the cases
//! here assert the armed tool, the opened transaction, and the exact state
//! that cancelling or escaping returns to.

use super::*;

#[test]
fn safe_mode_disables_schematic_mutation_commands_but_keeps_canvas_settings_available() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    app.state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..Default::default()
        },
        String::new(),
    );

    for command in [
        Command::PlaceInstance,
        Command::PlaceWire,
        Command::PlaceBus,
        Command::PlaceBusTap,
        Command::PlaceJunction,
        Command::PlaceLabel,
        Command::PlacePin,
        Command::PlaceText,
    ] {
        assert!(!command.is_enabled(&app), "{command:?}");
    }
    assert!(Command::CycleGrid.is_enabled(&app));
    assert!(Command::GridSnapRouting.is_enabled(&app));

    Command::PlaceWire.execute(&mut app);
    assert_eq!(app.state.schematic.tool, Tool::Select);
}

#[test]
fn bus_commands_have_stable_mockup_identities() {
    assert_eq!(Command::PlaceBus.stable_id(), "place-bus");
    assert_eq!(Command::PlaceBus.spec().label, "Draw bus");
    assert_eq!(Command::PlaceBusTap.stable_id(), "place-bus-tap");
    assert_eq!(Command::PlaceBusTap.spec().label, "Place bus tap");
    assert_eq!(
        Command::from_stable_id("place-bus"),
        Some(Command::PlaceBus)
    );
    assert_eq!(
        Command::from_stable_id("place-bus-tap"),
        Some(Command::PlaceBusTap)
    );
}

#[test]
fn place_pin_command_has_the_exact_mockup_identity() {
    assert_eq!(Command::PlacePin.stable_id(), "place-pin");
    assert_eq!(Command::PlacePin.spec().label, "Place pin or port\u{2026}");
    assert_eq!(Command::PlacePin.spec().group, "Design");
    assert_eq!(
        Command::from_stable_id("place-pin"),
        Some(Command::PlacePin)
    );
    assert_ne!(Command::PlacePin, Command::SymbolPinTool);
}

#[test]
fn place_text_command_has_the_exact_mockup_identity() {
    assert_eq!(Command::PlaceText.stable_id(), "place-text");
    assert_eq!(
        Command::PlaceText.spec().label,
        "Place text or note\u{2026}"
    );
    assert_eq!(Command::PlaceText.spec().group, "Design");
    assert_eq!(
        Command::from_stable_id("place-text"),
        Some(Command::PlaceText)
    );
}

#[test]
fn place_shape_command_has_the_exact_mockup_identity_and_no_shortcut() {
    assert_eq!(Command::PlaceShape.stable_id(), "place-shape");
    assert_eq!(
        Command::PlaceShape.spec().label,
        "Draw documentation shape\u{2026}"
    );
    assert_eq!(Command::PlaceShape.spec().group, "Design");
    assert_eq!(
        Command::from_stable_id("place-shape"),
        Some(Command::PlaceShape)
    );
    assert!(Command::PlaceShape.shortcut_bindings().is_empty());
}

#[test]
fn draw_bus_arms_directly_but_bus_tap_waits_for_its_validated_dialog() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;

    Command::PlaceBus.execute(&mut app);
    assert_eq!(app.state.schematic.tool, Tool::Bus);

    app.state.schematic.tool = Tool::Select;
    Command::PlaceBusTap.execute(&mut app);
    assert!(app.state.dialogs.bus_tap.open);
    assert_eq!(app.state.schematic.tool, Tool::Select);
    assert!(app.state.schematic.pending_bus_tap.is_none());
}

#[test]
fn place_pin_opens_the_isolated_mockup_transaction_without_mutating_the_document() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let components = app.state.schematic.components.clone();
    let topology = app.state.schematic.topology_version();
    let dirty = app.state.schematic.is_dirty;
    let tool = app.state.schematic.tool;

    Command::PlacePin.execute(&mut app);

    assert!(app.state.dialogs.pin_port.open);
    assert_eq!(app.state.dialogs.pin_port.name, "BIAS_EN");
    assert_eq!(app.state.schematic.components, components);
    assert_eq!(app.state.schematic.topology_version(), topology);
    assert_eq!(app.state.schematic.is_dirty, dirty);
    assert_eq!(app.state.schematic.tool, tool);
    assert!(app.state.schematic.pending_port.is_none());
    assert!(!app.state.schematic.can_undo());
}

#[test]
fn place_text_opens_the_isolated_mockup_transaction_without_mutating_the_document() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let notes = app.state.schematic.design_notes.clone();
    let topology = app.state.schematic.topology_version();
    let dirty = app.state.schematic.is_dirty;
    let tool = app.state.schematic.tool;

    Command::PlaceText.execute(&mut app);

    assert!(app.state.dialogs.design_note.open);
    assert_eq!(app.state.dialogs.design_note.text, "Bias network");
    assert_eq!(app.state.schematic.design_notes, notes);
    assert_eq!(app.state.schematic.topology_version(), topology);
    assert_eq!(app.state.schematic.is_dirty, dirty);
    assert_eq!(app.state.schematic.tool, tool);
    assert!(app.state.schematic.pending_design_note.is_none());
    assert!(!app.state.schematic.can_undo());
}

#[test]
fn place_shape_opens_the_isolated_mockup_transaction_without_mutating_the_document() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let shapes = app.state.schematic.documentation_shapes.clone();
    let topology = app.state.schematic.topology_version();
    let dirty = app.state.schematic.is_dirty;
    let tool = app.state.schematic.tool;

    Command::PlaceShape.execute(&mut app);

    assert!(app.state.dialogs.documentation_shape.open);
    assert_eq!(
        app.state.dialogs.documentation_shape.kind,
        crate::state::DocumentationShapeKind::Rectangle
    );
    assert_eq!(app.state.schematic.documentation_shapes, shapes);
    assert_eq!(app.state.schematic.topology_version(), topology);
    assert_eq!(app.state.schematic.is_dirty, dirty);
    assert_eq!(app.state.schematic.tool, tool);
    assert!(app.state.schematic.pending_documentation_shape.is_none());
    assert!(!app.state.schematic.can_undo());
}

#[test]
fn every_raw_port_command_route_is_projected_through_the_same_dialog() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;

    Command::Place(ComponentType::Port).execute(&mut app);

    assert!(app.state.dialogs.pin_port.open);
    assert_eq!(app.state.schematic.tool, Tool::Select);
    assert!(app.state.schematic.pending_port.is_none());
    assert!(app.state.schematic.components.is_empty());
}

#[test]
fn port_undo_and_redo_resynchronize_the_generated_symbol_contract() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let reference = app.state.workspace.active_view.clone();
    let pending = crate::state::PendingPortPlacement::new(
        "BIAS_EN",
        crate::state::PortDirectionType::InputLogic,
        crate::state::PortDiscipline::Logic,
        app.state.schematic.topology_version(),
        app.state.schematic.next_interface_order(),
    );
    app.state
        .schematic
        .place_pending_port(crate::state::Point::origin(), pending)
        .expect("port places");
    app.state.sync_active_schematic_to_workspace();
    let symbol_ports = |app: &RSpiceApp| {
        app.state
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view("symbol"))
            .and_then(|view| view.metadata.get("ports"))
            .cloned()
    };
    assert_eq!(symbol_ports(&app).as_deref(), Some("BIAS_EN:in"));

    Command::Undo.execute(&mut app);
    assert!(app.state.schematic.components.is_empty());
    assert!(symbol_ports(&app).is_none());

    Command::Redo.execute(&mut app);
    assert_eq!(symbol_ports(&app).as_deref(), Some("BIAS_EN:in"));
}

#[test]
fn bus_authoring_commands_are_unavailable_on_read_only_schematics() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    assert!(Command::PlaceBus.is_enabled(&app));
    assert!(Command::PlaceBusTap.is_enabled(&app));
    assert!(Command::PlacePin.is_enabled(&app));
    assert!(Command::PlaceText.is_enabled(&app));
    assert!(Command::PlaceShape.is_enabled(&app));

    app.state.schematic.read_only = true;
    assert!(!Command::PlaceBus.is_enabled(&app));
    assert!(!Command::PlaceBusTap.is_enabled(&app));
    assert!(!Command::PlacePin.is_enabled(&app));
    assert!(!Command::PlaceText.is_enabled(&app));
    assert!(!Command::PlaceShape.is_enabled(&app));
    app.state.schematic.read_only = false;
    app.state.workbench.workspace = Workspace::Results;
    assert!(!Command::PlacePin.is_enabled(&app));
    assert!(!Command::PlaceText.is_enabled(&app));
    assert!(!Command::PlaceShape.is_enabled(&app));
}

#[test]
fn switching_conductor_tools_cancels_incompatible_routes_and_tap_state() {
    use crate::state::{BusDeclaration, BusSlice, BusTapOrientation, PendingBusTap, Point};

    let mut schematic = crate::state::SchematicState::default();
    schematic.arm_tool(Tool::Wire);
    schematic.start_wire(Point::origin());
    assert!(schematic.wire_drawing.active);

    schematic.arm_tool(Tool::Bus);
    assert!(!schematic.wire_drawing.active);
    schematic.start_bus(Point::new(2, 3), None).unwrap();
    assert!(schematic.bus_drawing.active);

    schematic.pending_bus_tap = Some(
        PendingBusTap::new(
            BusDeclaration::parse("DATA[15:0]").unwrap(),
            BusSlice::parse("DATA[7:0]").unwrap(),
            BusTapOrientation::Automatic,
        )
        .unwrap(),
    );
    schematic.arm_tool(Tool::BusTap);
    assert!(!schematic.bus_drawing.active);
    assert!(schematic.pending_bus_tap.is_some());

    schematic.arm_tool(Tool::Wire);
    assert!(schematic.pending_bus_tap.is_none());
}

#[test]
fn cancel_clears_even_hidden_conductor_routes() {
    use crate::state::Point;

    let mut schematic = crate::state::SchematicState::default();
    schematic.tool = Tool::Select;
    schematic.start_wire(Point::origin());
    schematic.start_bus(Point::new(4, 5), None).unwrap();
    assert!(schematic.wire_drawing.active);
    assert!(schematic.bus_drawing.active);

    schematic.cancel_tool();

    assert_eq!(schematic.tool, Tool::Select);
    assert!(!schematic.wire_drawing.active);
    assert!(!schematic.bus_drawing.active);
    assert!(schematic.pending_bus_tap.is_none());
}

#[test]
fn escape_walks_route_then_tool_then_selection_without_collapsing_stages() {
    use crate::state::{ComponentType, Point};

    let mut schematic = crate::state::SchematicState::default();
    let selected = schematic.add_component(ComponentType::Resistor, Point::origin());
    schematic.selection.select_only_component(selected);
    schematic.tool = Tool::Wire;
    schematic.start_wire(Point::origin());
    schematic.extend_wire(Point::new(10, 0));

    schematic.cancel_interaction_step();
    assert!(!schematic.wire_drawing.active);
    assert_eq!(schematic.tool, Tool::Wire);
    assert!(schematic.selection.has_component(selected));

    schematic.cancel_interaction_step();
    assert_eq!(schematic.tool, Tool::Select);
    assert!(schematic.selection.has_component(selected));

    schematic.cancel_interaction_step();
    assert!(schematic.selection.is_empty());
}
