//! Commands that arm a placement tool or open a placement transaction.
//!
//! Arming is not editing. None of these may touch the document at the moment
//! it runs: the schematic changes when the placement completes, so the cases
//! here assert the armed tool, the opened transaction, and the exact state
//! that cancelling or escaping returns to.

use super::*;
use crate::workbench::design::commands::DesignCommand;

#[test]
fn safe_mode_disables_schematic_mutation_commands_but_keeps_canvas_settings_available() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    app.state.model.safe_mode.activate(
        crate::state::write_policy::LocalSafeModeOptions {
            open_project_read_only: true,
            ..Default::default()
        },
        String::new(),
    );

    for key in [
        DesignCommand::PlaceInstance.key(),
        DesignCommand::PlaceWire.key(),
        DesignCommand::PlaceBus.key(),
        DesignCommand::PlaceBusTap.key(),
        DesignCommand::PlaceJunction.key(),
        DesignCommand::PlaceLabel.key(),
        DesignCommand::PlacePin.key(),
        DesignCommand::PlaceText.key(),
    ] {
        assert!(!app.command_key_enabled(key), "{key:?}");
    }
    assert!(Command::CycleGrid.is_enabled(&app));
    assert!(app.command_key_enabled(DesignCommand::GridSnapRouting.key()));

    app.execute_command_key(DesignCommand::PlaceWire.key());
    assert_eq!(app.state.model.schematic.session.editor.tool, Tool::Select);
}

#[test]
fn draw_bus_arms_directly_but_bus_tap_waits_for_its_validated_dialog() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;

    app.execute_command_key(DesignCommand::PlaceBus.key());
    assert_eq!(app.state.model.schematic.session.editor.tool, Tool::Bus);

    app.state.model.schematic.session.editor.tool = Tool::Select;
    app.execute_command_key(DesignCommand::PlaceBusTap.key());
    assert!(app.state.design.bus_tap.open);
    assert_eq!(
        app.state.design.bus_tap.source,
        Some(crate::workbench::app::schematic_editor_request_source(
            &app.state
        ))
    );
    assert_eq!(app.state.model.schematic.session.editor.tool, Tool::Select);
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .pending_bus_tap
            .is_none()
    );
}

#[test]
fn place_pin_opens_the_isolated_mockup_transaction_without_mutating_the_document() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let components = app.state.model.schematic.document().components.clone();
    let topology = app.state.model.schematic.topology_version();
    let dirty = app.state.model.schematic.session.is_dirty;
    let tool = app.state.model.schematic.session.editor.tool;

    app.execute_command_key(DesignCommand::PlacePin.key());

    assert!(app.state.design.pin_port.open);
    // Empty on first use: the form used to open holding a sample name.
    assert_eq!(app.state.design.pin_port.fields.names, "");
    assert_eq!(
        app.state.design.pin_port.fields.direction,
        crate::state::PortDirection::In
    );
    assert_eq!(
        app.state.design.pin_port.fields.signal_type,
        crate::state::PortSignalType::Analog
    );
    assert_eq!(app.state.model.schematic.document().components, components);
    assert_eq!(app.state.model.schematic.topology_version(), topology);
    assert_eq!(app.state.model.schematic.session.is_dirty, dirty);
    assert_eq!(app.state.model.schematic.session.editor.tool, tool);
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .pending_port_sequence
            .is_none()
    );
    assert!(!app.state.model.schematic.can_undo());
}

#[test]
fn place_text_opens_the_isolated_mockup_transaction_without_mutating_the_document() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let notes = app.state.model.schematic.document().design_notes.clone();
    let topology = app.state.model.schematic.topology_version();
    let dirty = app.state.model.schematic.session.is_dirty;
    let tool = app.state.model.schematic.session.editor.tool;

    app.execute_command_key(DesignCommand::PlaceText.key());

    assert!(app.state.design.design_note.open);
    // The form opens empty. The sample it used to open holding was written
    // into the field as a value, so an untouched form placed a note saying it.
    assert_eq!(app.state.design.design_note.text, "");
    assert_eq!(app.state.model.schematic.document().design_notes, notes);
    assert_eq!(app.state.model.schematic.topology_version(), topology);
    assert_eq!(app.state.model.schematic.session.is_dirty, dirty);
    assert_eq!(app.state.model.schematic.session.editor.tool, tool);
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .pending_design_note
            .is_none()
    );
    assert!(!app.state.model.schematic.can_undo());
}

#[test]
fn place_shape_opens_the_isolated_mockup_transaction_without_mutating_the_document() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let shapes = app
        .state
        .model
        .schematic
        .document()
        .documentation_shapes
        .clone();
    let topology = app.state.model.schematic.topology_version();
    let dirty = app.state.model.schematic.session.is_dirty;
    let tool = app.state.model.schematic.session.editor.tool;

    app.execute_command_key(DesignCommand::PlaceShape.key());

    assert!(app.state.design.documentation_shape.open);
    assert_eq!(
        app.state.design.documentation_shape.kind,
        crate::state::DocumentationShapeKind::Rectangle
    );
    assert_eq!(
        app.state.model.schematic.document().documentation_shapes,
        shapes
    );
    assert_eq!(app.state.model.schematic.topology_version(), topology);
    assert_eq!(app.state.model.schematic.session.is_dirty, dirty);
    assert_eq!(app.state.model.schematic.session.editor.tool, tool);
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .pending_documentation_shape
            .is_none()
    );
    assert!(!app.state.model.schematic.can_undo());
}

#[test]
fn every_raw_port_command_route_is_projected_through_the_same_dialog() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;

    app.execute_command_key(DesignCommand::PlacePin.key());

    assert!(app.state.design.pin_port.open);
    assert_eq!(app.state.model.schematic.session.editor.tool, Tool::Select);
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .pending_port_sequence
            .is_none()
    );
    assert!(app.state.model.schematic.document().components.is_empty());
}

#[test]
fn port_undo_and_redo_resynchronize_the_generated_symbol_contract() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let reference = app.state.model.workspace.content.active_view.clone();
    let pending = crate::state::PendingPortPlacement::new(
        "BIAS_EN",
        crate::state::PortDirectionType::InputLogic,
        crate::state::PortDiscipline::Logic,
        app.state.model.schematic.topology_version(),
        app.state.model.schematic.next_interface_order(),
    );
    app.state
        .model
        .schematic
        .place_pending_port(crate::state::Point::origin(), pending)
        .expect("port places");
    app.state.sync_active_schematic_to_workspace();
    let symbol_ports = |app: &RSpiceApp| {
        app.state
            .model
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view("symbol"))
            .and_then(|view| view.metadata.get("ports"))
            .cloned()
    };
    assert_eq!(symbol_ports(&app).as_deref(), Some("BIAS_EN:in"));

    Command::Undo.execute(&mut app);
    assert!(app.state.model.schematic.document().components.is_empty());
    assert!(symbol_ports(&app).is_none());

    Command::Redo.execute(&mut app);
    assert_eq!(symbol_ports(&app).as_deref(), Some("BIAS_EN:in"));
}

#[test]
fn bus_authoring_commands_are_unavailable_on_read_only_schematics() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    assert!(app.command_key_enabled(DesignCommand::PlaceBus.key()));
    assert!(app.command_key_enabled(DesignCommand::PlaceBusTap.key()));
    assert!(app.command_key_enabled(DesignCommand::PlacePin.key()));
    assert!(app.command_key_enabled(DesignCommand::PlaceText.key()));
    assert!(app.command_key_enabled(DesignCommand::PlaceShape.key()));

    app.state.model.schematic.session.read_only = true;
    assert!(!app.command_key_enabled(DesignCommand::PlaceBus.key()));
    assert!(!app.command_key_enabled(DesignCommand::PlaceBusTap.key()));
    assert!(!app.command_key_enabled(DesignCommand::PlacePin.key()));
    assert!(!app.command_key_enabled(DesignCommand::PlaceText.key()));
    assert!(!app.command_key_enabled(DesignCommand::PlaceShape.key()));
    app.state.model.schematic.session.read_only = false;
    app.state.workbench.workspace = Workspace::Results;
    assert!(!app.command_key_enabled(DesignCommand::PlacePin.key()));
    assert!(!app.command_key_enabled(DesignCommand::PlaceText.key()));
    assert!(!app.command_key_enabled(DesignCommand::PlaceShape.key()));
}

#[test]
fn escape_walks_transaction_route_tool_and_selection_without_collapsing_stages() {
    use crate::state::{ComponentType, Point};

    let mut schematic = crate::state::SchematicState::default();
    let selected = schematic.add_component(ComponentType::Resistor, Point::origin());
    schematic
        .session
        .editor
        .selection
        .select_only_component(selected);
    schematic.session.editor.tool = Tool::Wire;
    schematic.init_undo_history();
    schematic.start_wire(Point::origin());
    schematic.extend_wire(Point::new(10, 0));

    schematic.begin_operation("move selection");
    schematic.move_selection(Point::new(20, 30));
    assert_eq!(schematic.document().components[0].pos, Point::new(20, 30));
    schematic.cancel_interaction_step();
    assert_eq!(schematic.document().components[0].pos, Point::origin());
    assert!(!schematic.has_pending_operation());
    assert!(!schematic.can_undo());
    assert!(schematic.session.editor.wire_drawing.active);
    assert_eq!(schematic.session.editor.tool, Tool::Wire);
    assert!(schematic.session.editor.selection.has_component(selected));

    schematic.cancel_interaction_step();
    assert!(!schematic.session.editor.wire_drawing.active);
    assert_eq!(schematic.session.editor.tool, Tool::Wire);
    assert!(schematic.session.editor.selection.has_component(selected));

    schematic.cancel_interaction_step();
    assert_eq!(schematic.session.editor.tool, Tool::Select);
    assert!(schematic.session.editor.selection.has_component(selected));

    schematic.cancel_interaction_step();
    assert!(schematic.session.editor.selection.is_empty());
}

/// Repeat last command runs a feature's placement again: the wire tool a key
/// armed comes back after the select tool replaced it.
#[test]
fn repeat_last_command_rearms_a_feature_placement() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    app.state.open_project_session();
    app.execute_command_key(DesignCommand::PlaceWire.key());
    assert_eq!(
        app.state.workbench.last_repeatable_command.as_deref(),
        Some(DesignCommand::PlaceWire.key().as_str())
    );
    Command::SelectTool.execute(&mut app);
    assert_eq!(
        app.state.model.schematic.session.editor.tool,
        crate::state::Tool::Select
    );

    assert!(app.command_key_enabled(Command::RepeatLastCommand.key()));
    Command::RepeatLastCommand.execute(&mut app);
    assert_eq!(
        app.state.model.schematic.session.editor.tool,
        crate::state::Tool::Wire
    );
}
