//! Commands that act on what is selected in the active schematic.
//!
//! Availability is the assertion. Each command is offered only when the
//! selection it needs is live, editable, and of a class it can actually
//! operate on; where it is not, the command reports unavailable instead of
//! dispatching into a selection it cannot serve. A command enabled over an
//! empty or read-only selection has already lied to the user.

use super::*;

#[test]
fn descend_requires_a_resolved_schematic_master() {
    use crate::state::{Cell, Component, Library, LibraryCellInstance, Point, View, ViewType};

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    app.state.schematic.components.push(Component::new(
        1,
        ComponentType::Resistor,
        Point::origin(),
    ));
    app.state.schematic.selection.select_only_component(1);
    assert!(
        !Command::DescendHierarchy.availability(&app).is_available(),
        "a primitive is not hierarchy"
    );

    let mut cell = Cell::new("child");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    cell.add_view(View::new("symbol", ViewType::Symbol));
    let mut library = Library::new("work");
    library.add_cell(cell);
    app.state.library_manager.add_library(library);

    app.state.schematic.components.clear();
    app.state.schematic.components.push(
        Component::new(2, ComponentType::CellInstance, Point::origin())
            .with_library_cell(LibraryCellInstance::new("work", "child", "schematic")),
    );
    app.state.schematic.selection.select_only_component(2);
    assert!(
        Command::DescendHierarchy.availability(&app).is_available(),
        "a resolved schematic master is descendable"
    );
    let parent = app.state.workspace.active_view.clone();
    Command::DescendHierarchy.execute(&mut app);
    assert!(
        app.state.dialogs.descend_hierarchy.open,
        "the menu/palette command owns the explicit edit-context transaction"
    );
    assert_eq!(
        app.state.workspace.active_view, parent,
        "opening the transaction must not navigate before commit"
    );
    app.state.dialogs.descend_hierarchy.close();

    app.state.schematic.components[0]
        .library_cell
        .as_mut()
        .expect("binding")
        .view = "symbol".to_owned();
    assert!(
        !Command::DescendHierarchy.availability(&app).is_available(),
        "a symbol binding is not a descendable schematic master"
    );
}

#[test]
fn transform_commands_keep_wires_attached_to_authored_symbol_pins() {
    use crate::state::{Point, Rotation};

    let cases = [
        (Command::RotateSelection, Point::new(110, 10), Rotation::R90),
        (
            Command::MirrorSelectionHorizontal,
            Point::new(140, 40),
            Rotation::R0,
        ),
        (
            Command::MirrorSelectionVertical,
            Point::new(60, 60),
            Rotation::R0,
        ),
    ];

    for (command, expected_wire_endpoint, expected_rotation) in cases {
        let mut app = app_with_selected_authored_symbol();

        command.execute(&mut app);

        assert_eq!(
            app.state.schematic.wires[0].points[0],
            expected_wire_endpoint
        );
        assert_eq!(
            app.state.schematic.components[0].rotation,
            expected_rotation
        );
    }
}

#[test]
fn horizontal_coordinate_reflection_is_labeled_by_its_vertical_mirror_axis() {
    assert_eq!(
        Command::MirrorSelectionHorizontal.spec().label,
        "Mirror about vertical axis"
    );
}

fn app_with_every_complete_schematic_object() -> RSpiceApp {
    use crate::state::{
        Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Component, Junction, NetLabel,
        Point, Wire,
    };

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let bus = Bus::segment(
        5,
        Point::new(0, 20),
        Point::new(20, 20),
        Some(BusDeclaration::parse("DATA[3:0]").unwrap()),
    )
    .unwrap();
    let tap = BusTap::new(
        6,
        &bus,
        Point::new(10, 20),
        Point::new(10, 30),
        BusSlice::parse("DATA[1]").unwrap(),
        BusTapOrientation::Down,
    )
    .unwrap();
    app.state.schematic.components.push(Component::new(
        1,
        ComponentType::Resistor,
        Point::origin(),
    ));
    app.state
        .schematic
        .wires
        .push(Wire::segment(2, Point::new(0, 0), Point::new(20, 0)));
    app.state
        .schematic
        .junctions
        .push(Junction::new(3, Point::new(10, 0)));
    app.state
        .schematic
        .net_labels
        .push(NetLabel::new(4, Point::new(10, 0), "sense_out"));
    app.state.schematic.buses.push(bus);
    app.state.schematic.bus_taps.push(tap);
    app
}

#[test]
fn edit_command_enablement_covers_every_complete_schematic_object_class() {
    let mut app = app_with_every_complete_schematic_object();

    let selectable = [
        ("component", 1_u64),
        ("wire", 2),
        ("net label", 4),
        ("bus", 5),
        ("bus tap", 6),
    ];
    for (kind, id) in selectable {
        app.state.schematic.selection.clear();
        match kind {
            "component" => app.state.schematic.selection.select_component(id),
            "wire" => app.state.schematic.selection.select_wire(id),
            "net label" => app.state.schematic.selection.select_net_label(id),
            "bus" => app.state.schematic.selection.select_bus(id),
            "bus tap" => app.state.schematic.selection.select_bus_tap(id),
            _ => unreachable!(),
        }
        assert!(Command::Copy.is_enabled(&app), "copy disabled for {kind}");
        assert!(Command::Cut.is_enabled(&app), "cut disabled for {kind}");
        assert!(
            Command::Delete.is_enabled(&app),
            "delete disabled for {kind}"
        );
        assert!(
            Command::Duplicate.is_enabled(&app),
            "duplicate disabled for {kind}"
        );
    }

    app.state
        .schematic
        .selection
        .select_only_junction(crate::state::Point::new(10, 0));
    assert!(Command::Copy.is_enabled(&app));
    assert!(Command::Cut.is_enabled(&app));
    assert!(Command::Delete.is_enabled(&app));
    assert!(
        !Command::Duplicate.is_enabled(&app),
        "a fixed-offset duplicate cannot invent a valid junction target"
    );
}

#[test]
fn delete_promotes_live_wire_handles_without_enabling_partial_copy_or_cut() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    app.state.schematic.wires.push(Wire::new(
        17,
        vec![
            crate::state::Point::new(0, 0),
            crate::state::Point::new(20, 0),
            crate::state::Point::new(20, 20),
        ],
    ));

    app.state
        .schematic
        .selection
        .select_only_wire_segment(17, 1);
    assert!(Command::Delete.is_enabled(&app));
    assert!(!Command::Copy.is_enabled(&app));
    assert!(!Command::Cut.is_enabled(&app));
    assert!(!Command::Duplicate.is_enabled(&app));

    app.state.schematic.selection.select_only_wire_vertex(17, 1);
    assert!(Command::Delete.is_enabled(&app));
    assert!(!Command::Copy.is_enabled(&app));
    assert!(!Command::Cut.is_enabled(&app));
    assert!(!Command::Duplicate.is_enabled(&app));

    app.state
        .schematic
        .selection
        .select_only_wire_segment(17, 2);
    assert!(
        !Command::Delete.is_enabled(&app),
        "an out-of-range wire handle is stale, not a deletable object"
    );
}

#[test]
fn select_all_command_opens_the_governed_schematic_scope_workflow() {
    let mut app = app_with_every_complete_schematic_object();

    Command::SelectAll.execute(&mut app);

    assert!(app.state.dialogs.selection_workflow.open);
    assert!(app.state.dialogs.application_modal_open());
    assert!(app.state.schematic.selection.is_empty());
}

#[test]
fn move_selection_command_has_the_exact_mockup_identity() {
    assert_eq!(Command::MoveSelection.stable_id(), "move-selection");
    assert_eq!(Command::MoveSelection.spec().label, "Move selection");
    assert_eq!(Command::MoveSelection.spec().group, "Design");
    assert_eq!(
        Command::from_stable_id("move-selection"),
        Some(Command::MoveSelection)
    );

    let registry_index = vocabulary::COMMAND_REGISTRY
        .iter()
        .position(|command| *command == Command::MoveSelection)
        .expect("move-selection must be registered");
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index - 1],
        Command::PlaceShape
    );
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index + 1],
        Command::StretchSelection
    );
}

#[test]
fn stretch_selection_command_has_the_exact_mockup_identity() {
    assert_eq!(Command::StretchSelection.stable_id(), "stretch-selection");
    assert_eq!(Command::StretchSelection.spec().label, "Stretch selection");
    assert_eq!(Command::StretchSelection.spec().group, "Design");
    assert_eq!(
        Command::from_stable_id("stretch-selection"),
        Some(Command::StretchSelection)
    );

    let registry_index = vocabulary::COMMAND_REGISTRY
        .iter()
        .position(|command| *command == Command::StretchSelection)
        .expect("stretch-selection must be registered");
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index - 1],
        Command::MoveSelection
    );
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index + 1],
        Command::ArraySelection
    );
}

#[test]
fn array_selection_command_has_the_exact_mockup_identity_and_no_shortcut() {
    assert_eq!(Command::ArraySelection.stable_id(), "array-selection");
    assert_eq!(Command::ArraySelection.spec().label, "Create array\u{2026}");
    assert_eq!(Command::ArraySelection.spec().group, "Design");
    assert_eq!(
        Command::from_stable_id("array-selection"),
        Some(Command::ArraySelection)
    );
    assert!(Command::ArraySelection.shortcut_bindings().is_empty());

    let registry_index = vocabulary::COMMAND_REGISTRY
        .iter()
        .position(|command| *command == Command::ArraySelection)
        .expect("array-selection must be registered");
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index - 1],
        Command::StretchSelection
    );
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index + 1],
        Command::ReplaceInstance
    );
}

#[test]
fn replace_instance_command_has_the_exact_mockup_identity_and_no_shortcut() {
    assert_eq!(Command::ReplaceInstance.stable_id(), "replace-instance");
    assert_eq!(
        Command::ReplaceInstance.spec().label,
        "Replace instance\u{2026}"
    );
    assert_eq!(Command::ReplaceInstance.spec().group, "Design");
    assert_eq!(
        Command::from_stable_id("replace-instance"),
        Some(Command::ReplaceInstance)
    );
    assert!(Command::ReplaceInstance.shortcut_bindings().is_empty());

    let registry_index = vocabulary::COMMAND_REGISTRY
        .iter()
        .position(|command| *command == Command::ReplaceInstance)
        .expect("replace-instance must be registered");
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index - 1],
        Command::ArraySelection
    );
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index + 1],
        Command::CreateHierarchy
    );
}

#[test]
fn move_selection_requires_one_live_object_in_an_editable_active_schematic() {
    use crate::state::{Component, Point};

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    assert!(!Command::MoveSelection.is_enabled(&app));

    app.state.schematic.selection.select_component(404);
    assert!(
        !Command::MoveSelection.is_enabled(&app),
        "a stale selection identity is not a movable object"
    );

    app.state.schematic.components.push(Component::new(
        404,
        ComponentType::Resistor,
        Point::origin(),
    ));
    assert!(Command::MoveSelection.is_enabled(&app));
    assert_eq!(
        Command::MoveSelection.availability(&app),
        CommandAvailability::Available
    );

    app.state.schematic.read_only = true;
    assert!(!Command::MoveSelection.is_enabled(&app));
    assert_eq!(
        Command::MoveSelection.availability(&app),
        CommandAvailability::Disabled("the active schematic is read-only")
    );

    app.state.schematic.read_only = false;
    app.state.workbench.workspace = Workspace::Results;
    assert!(!Command::MoveSelection.is_enabled(&app));
}

#[test]
fn stretch_selection_requires_one_live_eligible_geometry_target() {
    use crate::state::{Point, Wire};

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    app.state.schematic.selection.select_wire_segment(17, 0);
    assert!(
        !Command::StretchSelection.is_enabled(&app),
        "a stale segment identity cannot open the workflow"
    );

    app.state
        .schematic
        .wires
        .push(Wire::new(17, vec![Point::new(0, 0), Point::new(40, 0)]));
    assert!(Command::StretchSelection.is_enabled(&app));
    assert_eq!(
        Command::StretchSelection.availability(&app),
        CommandAvailability::Available
    );

    app.state.schematic.read_only = true;
    assert!(!Command::StretchSelection.is_enabled(&app));
    assert_eq!(
        Command::StretchSelection.availability(&app),
        CommandAvailability::Disabled("the active schematic is read-only")
    );

    app.state.schematic.read_only = false;
    app.state.workbench.workspace = Workspace::Results;
    assert!(!Command::StretchSelection.is_enabled(&app));
}

#[test]
fn array_selection_requires_a_live_eligible_editable_selection() {
    use crate::state::{Component, Point};

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    app.state.schematic.selection.select_component(404);
    assert!(
        !Command::ArraySelection.is_enabled(&app),
        "a stale selection identity cannot open the workflow"
    );

    app.state.schematic.components.push(Component::new(
        404,
        ComponentType::Resistor,
        Point::origin(),
    ));
    assert!(Command::ArraySelection.is_enabled(&app));
    assert_eq!(
        Command::ArraySelection.availability(&app),
        CommandAvailability::Available
    );

    app.state.schematic.read_only = true;
    assert!(!Command::ArraySelection.is_enabled(&app));
    assert_eq!(
        Command::ArraySelection.availability(&app),
        CommandAvailability::Disabled("the active schematic is read-only")
    );

    app.state.schematic.read_only = false;
    app.state.workbench.workspace = Workspace::Results;
    assert!(!Command::ArraySelection.is_enabled(&app));
}

#[test]
fn cancel_retires_an_armed_array_transaction_and_restores_select() {
    let mut app = RSpiceApp::test_instance();
    app.state.dialogs.array_selection.armed = true;
    app.state.schematic.tool = Tool::ArraySelection;

    Command::Cancel.execute(&mut app);

    assert!(!app.state.dialogs.array_selection.armed);
    assert_eq!(app.state.schematic.tool, Tool::Select);
}

#[test]
fn rename_command_has_mockup_identity_and_opens_the_stable_target_dialog() {
    use crate::state::Point;

    assert_eq!(Command::RenameSelection.stable_id(), "rename-selection");
    assert_eq!(
        Command::RenameSelection.spec().label,
        "Rename selected object…"
    );
    assert_eq!(
        Command::from_stable_id("rename-selection"),
        Some(Command::RenameSelection)
    );

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let id = app
        .state
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));
    app.state.schematic.selection.select_only_component(id);
    assert!(Command::RenameSelection.is_enabled(&app));
    Command::RenameSelection.execute(&mut app);
    assert!(app.state.dialogs.rename_selection.open);
    assert!(matches!(
        app.state.dialogs.rename_selection.target.as_ref(),
        Some(crate::workbench::app::RenameSelectionTarget::Component(component))
            if component.id == id
    ));
}

#[test]
fn object_properties_dispatches_selected_buses_and_taps_and_refuses_read_only() {
    use crate::state::{Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Point};

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let bus = Bus::segment(
        80,
        Point::new(0, 0),
        Point::new(20, 0),
        Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
    )
    .unwrap();
    let tap = BusTap::new(
        81,
        &bus,
        Point::new(5, 0),
        Point::new(5, 5),
        BusSlice::parse("DATA[3]").unwrap(),
        BusTapOrientation::Down,
    )
    .unwrap();
    app.state.schematic.buses.push(bus);
    app.state.schematic.bus_taps.push(tap);

    app.state.schematic.selection.select_only_bus(80);
    assert!(Command::ObjectProperties.is_enabled(&app));
    Command::ObjectProperties.execute(&mut app);
    assert!(matches!(
        app.state.dialogs.object_properties.draft,
        Some(crate::workbench::app::ObjectPropertiesDraft::Bus(_))
    ));
    app.state.dialogs.object_properties.close();

    app.state.schematic.selection.select_only_bus_tap(81);
    Command::ObjectProperties.execute(&mut app);
    assert!(matches!(
        app.state.dialogs.object_properties.draft,
        Some(crate::workbench::app::ObjectPropertiesDraft::BusTap(_))
    ));
    app.state.dialogs.object_properties.close();

    app.state.schematic.read_only = true;
    assert!(!Command::ObjectProperties.is_enabled(&app));
    Command::ObjectProperties.execute(&mut app);
    assert!(!app.state.dialogs.object_properties.open);
}

#[test]
fn object_properties_availability_includes_one_selected_net_label() {
    use crate::state::Point;

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    let id = app
        .state
        .schematic
        .add_net_label(Point::new(0, 0), "gain_node".to_owned());
    app.state.schematic.selection.select_only_net_label(id);

    assert!(Command::ObjectProperties.is_enabled(&app));
    Command::ObjectProperties.execute(&mut app);
    assert!(matches!(
        app.state.dialogs.object_properties.draft,
        Some(crate::workbench::app::ObjectPropertiesDraft::NetLabel(ref draft))
            if draft.original.id == id
    ));
    app.state.dialogs.object_properties.close();
    app.state.schematic.read_only = true;
    assert!(!Command::ObjectProperties.is_enabled(&app));
    app.state.schematic.read_only = false;
    app.state.schematic.net_labels.clear();
    assert!(app.state.schematic.selection.single_net_label().is_some());
    assert!(!Command::ObjectProperties.is_enabled(&app));
}
