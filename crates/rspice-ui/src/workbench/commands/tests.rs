//! Tests for command dispatch and menu availability.
//!
//! The cases here mostly assert absence: a command whose behaviour is not
//! implemented must not be offered, and one whose preconditions are unmet must
//! report unavailable rather than dispatch and fail.

use super::vocabulary::{CommandSpec, command_catalog};
use super::*;
use crate::state::Wire;
use crate::workbench::state::{ModelsPage, ProjectPage, SimulationPage};

fn app_with_selected_authored_symbol() -> RSpiceApp {
    use crate::state::{
        Cell, Component, Library, LibraryCellInstance, Point, PortDirection, PortSpec,
        SymbolDocument, SymbolPin, View, ViewType, Wire,
    };

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;

    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
        ],
        ..SymbolDocument::default()
    };
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    document
        .store_in_view(&mut symbol_view)
        .expect("authored symbol stores");
    let mut cell = Cell::new("amp");
    cell.add_view(symbol_view);
    let mut library = Library::new("command_test");
    library.add_cell(cell);
    app.state.library_manager.add_library(library);

    let interface = [
        PortSpec {
            name: "IN".to_owned(),
            direction: PortDirection::In,
        },
        PortSpec {
            name: "OUT".to_owned(),
            direction: PortDirection::Out,
        },
    ];
    let mut binding = LibraryCellInstance::new("command_test", "amp", "schematic");
    binding.bind_interface(&interface);
    app.state.schematic.components.push(
        Component::new(701, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(binding),
    );
    app.state
        .schematic
        .wires
        .push(Wire::segment(702, Point::new(60, 40), Point::new(60, 0)));
    app.state.schematic.selection.select_component(701);
    app
}

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

#[test]
fn drawing_sheet_file_commands_keep_the_canonical_palette_identities() {
    let expected = [
        (Command::PageSetup, "page-setup", "Page setup…"),
        (
            Command::SheetFormatManager,
            "sheet-format-manager",
            "Sheet formats across this document…",
        ),
        (
            Command::CustomSheetSizes,
            "sheet-preset-library",
            "Custom sheet sizes…",
        ),
    ];
    let searchable = command_catalog().collect::<Vec<_>>();

    for (command, stable_id, label) in expected {
        assert_eq!(command.stable_id(), stable_id);
        assert_eq!(command.spec().label, label);
        assert_eq!(Command::from_stable_id(stable_id), Some(command));
        assert!(searchable.contains(&command), "{stable_id}");
    }
}

#[test]
fn authored_page_setup_never_falls_back_to_symbol_hardcopy_media() {
    let mut app = app_with_selected_authored_symbol();
    app.state.project_lifecycle.project_open = true;
    app.state
        .open_workspace_view(crate::state::CellViewRef::new(
            "command_test",
            "amp",
            "symbol",
        ));
    assert!(active_symbol_editor(&app));
    assert!(
        crate::workbench::hardcopy_adapters::sources::active_app_hardcopy_source_available(
            &app.state
        ),
        "the symbol is a valid print/export source"
    );

    assert!(!Command::PageSetup.is_enabled(&app));
    Command::PageSetup.execute(&mut app);

    assert!(!app.state.dialogs.drawing_sheet_setup.open);
    assert!(
        !app.state.dialogs.hardcopy.open,
        "authored Page Setup must not substitute output-media setup"
    );
}

#[test]
fn document_sheet_commands_open_their_real_surfaces_only_in_schematic_context() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.workbench.workspace = Workspace::Design;
    let key = app.state.workspace.active_key();
    app.state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Main", [])
        .unwrap();

    assert!(Command::SheetFormatManager.is_enabled(&app));
    assert!(Command::CustomSheetSizes.is_enabled(&app));

    Command::SheetFormatManager.execute(&mut app);
    assert!(app.state.dialogs.drawing_sheet_support.manager.open);

    Command::CustomSheetSizes.execute(&mut app);
    assert!(app.state.dialogs.drawing_sheet_presets.any_open());

    let mut results = RSpiceApp::test_instance();
    results.state.project_lifecycle.project_open = true;
    results.state.workbench.workspace = Workspace::Results;
    assert!(!Command::PageSetup.is_enabled(&results));
    assert!(!Command::SheetFormatManager.is_enabled(&results));
    assert!(!Command::CustomSheetSizes.is_enabled(&results));
}

#[test]
fn sheet_format_manager_requires_live_schematic_edit_authority() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.workbench.workspace = Workspace::Design;
    let key = app.state.workspace.active_key();
    app.state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Main", [])
        .unwrap();
    app.state.schematic.read_only = true;

    assert_eq!(
        Command::SheetFormatManager.availability(&app),
        CommandAvailability::Disabled("the active schematic is read-only")
    );
    Command::SheetFormatManager.execute(&mut app);
    assert!(!app.state.dialogs.drawing_sheet_support.manager.open);
}

#[test]
fn canvas_grid_command_cycles_display_without_mutating_snap_configuration() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    app.state.ui.set_grid_style(crate::state::GridStyle::Dots);
    app.state.schematic.snap_engine.enabled = true;
    app.state.schematic.snap_engine.snap_to_wire_segments = false;
    app.state.ui.schematic_snap = app.state.schematic.snap_engine.clone();

    assert_eq!(Command::CycleGrid.stable_id(), "cycle-grid");
    assert_eq!(Command::CycleGrid.spec().label, "Cycle grid display");

    Command::CycleGrid.execute(&mut app);
    assert_eq!(app.state.ui.grid, crate::state::GridStyle::Lines);
    assert!(app.state.schematic.snap_engine.enabled);
    assert!(app.state.ui.schematic_snap.enabled);
    assert!(
        !app.state.schematic.snap_engine.snap_to_wire_segments,
        "display cycling must preserve detailed snap target choices"
    );

    Command::CycleGrid.execute(&mut app);
    assert_eq!(app.state.ui.grid, crate::state::GridStyle::Off);
    assert!(app.state.schematic.snap_engine.enabled);

    Command::CycleGrid.execute(&mut app);
    assert_eq!(app.state.ui.grid, crate::state::GridStyle::Dots);
}

#[test]
fn canvas_grid_command_does_not_reinterpret_symbol_snap_policy() {
    let mut app = app_with_selected_authored_symbol();
    app.state
        .open_workspace_view(crate::state::CellViewRef::new(
            "command_test",
            "amp",
            "symbol",
        ));
    assert!(active_symbol_editor(&app));
    app.state.ui.symbol.show_grid = true;
    app.state.ui.symbol.snap_to_grid = false;

    Command::CycleGrid.execute(&mut app);
    assert!(!app.state.ui.symbol.show_grid);
    assert!(!app.state.ui.symbol.snap_to_grid);

    Command::CycleGrid.execute(&mut app);
    assert!(app.state.ui.symbol.show_grid);
    assert!(!app.state.ui.symbol.snap_to_grid);
}

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

#[test]
fn edit_specifications_opens_the_real_results_editor() {
    let mut app = RSpiceApp::test_instance();

    Command::EditSpecifications.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        app.state.ui.results.viewer,
        crate::workbench::ResultViewer::Specs
    );
    assert!(app.state.ui.results.spec_drafts.is_some());
}

#[test]
fn generic_results_command_opens_the_workspace_without_a_dataset() {
    let mut app = RSpiceApp::test_instance();
    let command = Command::ResultViewer(crate::workbench::ResultViewer::Waves);

    assert!(command.is_enabled(&app));
    assert_eq!(command.availability(&app), CommandAvailability::Available);
    command.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        app.state.ui.results.viewer,
        crate::workbench::ResultViewer::Waves
    );
}

#[test]
fn incompatible_result_viewer_command_is_disabled_and_cannot_navigate() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Project;
    let command = Command::ResultViewer(crate::workbench::ResultViewer::Bode);

    assert!(!command.is_enabled(&app));
    // Bode reads AC responses only: ordinary-noise spectra moved to the
    // Noise viewer that owns them, so the reason names AC alone.
    assert_eq!(
        command.availability(&app),
        CommandAvailability::Disabled("Requires a usable AC response in the active dataset")
    );
    command.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Project);
    assert_eq!(
        app.state.ui.results.viewer,
        crate::workbench::ResultViewer::Waves
    );
    assert!(
        app.state
            .log_buffer
            .entries()
            .any(|message| message.message.contains("cannot be opened"))
    );
}

#[test]
fn exposed_results_calculator_opens_the_real_editor_dialog() {
    let mut app = RSpiceApp::test_instance();
    assert!(!app.state.dialogs.waveform_calculator_dialog);

    Command::WaveformCalculator.execute(&mut app);

    assert!(app.state.dialogs.waveform_calculator_dialog);
}

#[test]
fn truthful_results_menu_routes_keep_their_stable_dispatch_identities() {
    for (command, stable_id) in [
        (
            Command::ResultViewer(crate::workbench::ResultViewer::Waves),
            "waveforms",
        ),
        (Command::DatasetManifestBrowser, "dataset-browser"),
        (Command::CreateResultDocument, "create-result-document"),
        (Command::CompareResultDatasets, "compare-datasets"),
        (Command::VisualizationTraceManager, "trace-manager"),
        (Command::VisualizationCursorManager, "cursor-manager"),
        (Command::ReviewNotes, "annotation-manager"),
        (Command::WaveformCalculator, "calculator"),
        (Command::MeasurementLibrary, "measurement-library"),
        (Command::FamilySlicing, "family-slicing"),
        (Command::VisualizationDocumentProperties, "plot-properties"),
        (Command::ImportResultDataset, "import-dataset"),
        (Command::ExportWaveformsCsv, "export-results"),
        (Command::ReportAuthoring, "report-page-editor"),
    ] {
        assert_eq!(command.stable_id(), stable_id);
        assert_eq!(Command::from_stable_id(stable_id), Some(command));
        assert!(vocabulary::COMMAND_REGISTRY.contains(&command));
        assert!(command_catalog().any(|candidate| candidate == command));
    }

    for unavailable in [
        Command::SaveReportDocument,
        Command::AddReportPage,
        Command::ReportPageProperties,
    ] {
        assert!(!command_catalog().any(|candidate| candidate == unavailable));
    }
}

/// A sheet reachable only by clicking its tab is reachable only when the strip
/// happens to draw that tab. The registry is what puts a command in the
/// palette, in the shortcut editor, and within reach of `from_stable_id` — so a
/// viewer missing from it cannot be bound, searched, or restored from a saved
/// profile. Driven by `ResultViewer::every()` on purpose: the hand-written list
/// this replaced went stale for five of the sheets.
#[test]
fn every_result_sheet_is_registered_for_commands_and_shortcuts() {
    use crate::workbench::ResultViewer;

    for viewer in ResultViewer::every() {
        let command = Command::ResultViewer(viewer);
        assert!(
            vocabulary::COMMAND_REGISTRY.contains(&command),
            "Result sheet is absent from the command registry: {viewer:?}"
        );
        assert_eq!(Command::from_stable_id(command.stable_id()), Some(command));
    }
}

#[test]
fn split_results_is_truthfully_gated_by_context_and_materialized_evidence() {
    let mut app = RSpiceApp::test_instance();
    let command = Command::ToggleResultsSplit;
    app.state.workbench.activate(Workspace::Design);

    assert_eq!(
        command.availability(&app),
        CommandAvailability::Disabled("no retained result dataset is available")
    );
    app.state
        .simulation
        .start_run()
        .add_analysis(crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::Transient,
            "retained TRAN",
        ));
    assert_eq!(command.availability(&app), CommandAvailability::Available);

    app.state.workbench.activate(Workspace::Results);
    assert_eq!(
        command.availability(&app),
        CommandAvailability::Disabled("open Design, Netlist, or Simulation setup")
    );
}

#[test]
fn schematic_zoom_commands_match_the_mockup_bounds_and_request_a_real_fit() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);

    app.state.schematic.zoom = 1.0;
    Command::ZoomIn.execute(&mut app);
    assert!((app.state.schematic.zoom - 1.2).abs() < f64::EPSILON);

    app.state.schematic.zoom = 1.0;
    app.execute_shortcut_command(Command::ZoomOut);
    assert!((app.state.schematic.zoom - (1.0 / 1.2)).abs() < f64::EPSILON);

    app.state.schematic.zoom = 0.251;
    Command::ZoomOut.execute(&mut app);
    assert_eq!(app.state.schematic.zoom, 0.25);

    app.state.schematic.zoom = 7.99;
    Command::ZoomIn.execute(&mut app);
    assert_eq!(app.state.schematic.zoom, 8.0);

    app.state.schematic.zoom = 3.5;
    app.state.schematic.pan = (127.0, -81.0);
    app.state.schematic.needs_fit = false;
    app.state.schematic.needs_drawing_sheet_fit = false;
    Command::ZoomFit.execute(&mut app);
    assert_eq!(app.state.schematic.zoom, 3.5);
    assert_eq!(app.state.schematic.pan, (127.0, -81.0));
    assert!(!app.state.schematic.needs_fit);
    assert!(app.state.schematic.needs_drawing_sheet_fit);

    Command::FitSchematicContent.execute(&mut app);
    assert!(app.state.schematic.needs_fit);
    assert!(!app.state.schematic.needs_drawing_sheet_fit);
}

#[test]
fn enabling_split_selects_latest_materialized_run_without_copying_results() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    let retained_dataset = {
        let run = app.state.simulation.start_run();
        run.add_analysis(crate::state::AnalysisResult::new(
            1,
            crate::state::AnalysisType::Transient,
            "retained TRAN",
        ));
        run.dataset_id
    };
    app.state.simulation.start_run();
    assert_eq!(app.state.simulation.active_run_idx, Some(0));
    let history_len = app.state.simulation.runs.len();

    Command::ToggleResultsSplit.execute(&mut app);

    assert!(app.state.workbench.split_with_results);
    assert_eq!(app.state.simulation.active_run_idx, Some(1));
    assert_eq!(
        app.state.simulation.active_run().map(|run| run.dataset_id),
        Some(retained_dataset)
    );
    assert_eq!(
        app.state.simulation.runs.len(),
        history_len,
        "the split projects the canonical dataset instead of cloning it"
    );

    Command::ToggleResultsSplit.execute(&mut app);
    assert!(!app.state.workbench.split_with_results);
}

#[test]
fn tuning_command_opens_the_transactional_sandbox() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Project;
    app.state.workbench.verification_page = VerificationPage::Yield;
    let command = Command::VerificationPage(VerificationPage::Tuning);

    assert!(command.is_enabled(&app));
    assert_eq!(command.availability(&app), CommandAvailability::Available);
    command.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Verify);
    assert_eq!(
        app.state.workbench.verification_page,
        VerificationPage::Tuning
    );
}

#[test]
fn physical_drc_command_is_inaccessible_without_physical_evidence_pipeline() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Project;
    app.state.workbench.verification_page = VerificationPage::Yield;
    let command = Command::VerificationPage(VerificationPage::Drc);

    assert!(!command.is_enabled(&app));
    assert_eq!(
        command.availability(&app),
        CommandAvailability::Disabled(
            "no retained layout, qualified rule deck, or immutable marker database is available"
        )
    );
    command.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Project);
    assert_eq!(
        app.state.workbench.verification_page,
        VerificationPage::Yield
    );
    assert!(
        app.state
            .log_buffer
            .entries()
            .any(|message| message.message.contains("Physical DRC is unavailable"))
    );
}

#[test]
fn clear_results_cannot_remove_the_executor_owned_run() {
    let mut app = RSpiceApp::test_instance();
    let run = app.state.simulation.start_run();
    run.mark_running().unwrap();
    let identity = run.execution_identity().unwrap();
    app.state.simulation.active_execution = Some(identity);
    app.state.simulation.is_running = true;

    assert!(!Command::ClearResults.is_enabled(&app));
    assert_eq!(
        Command::ClearResults.availability(&app),
        CommandAvailability::Disabled("an active simulation execution still owns result history")
    );

    Command::ClearResults.execute(&mut app);

    assert!(
        app.state
            .simulation
            .run_by_stable_id(identity.run_id)
            .is_some()
    );
}

#[test]
fn command_catalog_has_unique_stable_ids() {
    let mut ids = std::collections::HashSet::new();
    for command in vocabulary::COMMAND_REGISTRY {
        let id = command.spec().id;
        assert!(ids.insert(id), "duplicate command id {}", id);
        assert!(!id.is_empty());
        assert!(
            id.bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'),
            "command id is not a hyphenated product action: {id}"
        );
        assert_eq!(Command::from_stable_id(id), Some(*command));
    }
    for command in vocabulary::COMMAND_REGISTRY
        .iter()
        .copied()
        .filter(|command| !command.shortcut_bindings().is_empty())
    {
        assert!(
            ids.contains(command.stable_id()),
            "bindable command is missing a unique stable ID: {command:?}"
        );
    }
}

/// The palette is a title-matched list, so two commands sharing a title are two
/// rows the user cannot tell apart — and picking the wrong one goes somewhere
/// else entirely. Unique IDs do not cover this: they are never displayed.
#[test]
fn command_catalog_titles_name_exactly_one_action() {
    let mut labels: std::collections::HashMap<&str, Command> = std::collections::HashMap::new();
    for command in vocabulary::COMMAND_REGISTRY.iter().copied() {
        let label = command.spec().label;
        if let Some(previous) = labels.insert(label, command) {
            panic!("{previous:?} and {command:?} both offer the palette row {label:?}");
        }
    }
}

#[test]
fn legacy_model_metadata_audit_identity_migrates_to_qualification() {
    assert_eq!(
        Command::from_stable_id("model-metadata-audit"),
        Some(Command::ModelsPage(ModelsPage::Qualification))
    );
    assert_eq!(
        Command::ModelsPage(ModelsPage::Qualification).stable_id(),
        "model-qualification"
    );
}

#[test]
fn all_workspace_commands_are_discoverable() {
    for workspace in Workspace::ALL {
        assert!(vocabulary::COMMAND_REGISTRY.contains(&Command::OpenWorkspace(workspace)));
    }
}

#[test]
fn every_project_tab_has_one_discoverable_command_with_a_stable_identity() {
    let expected = [
        (ProjectPage::Overview, "project-overview"),
        (ProjectPage::Library, "project-library"),
        (
            ProjectPage::Configuration,
            "project-testbench-configuration",
        ),
        (ProjectPage::Dependencies, "project-dependencies"),
        (ProjectPage::Recovery, "project-recovery"),
    ];
    assert_eq!(ProjectPage::ALL.len(), expected.len());

    for (page, stable_id) in expected {
        let command = Command::ProjectPage(page);
        assert!(
            vocabulary::COMMAND_REGISTRY.contains(&command),
            "project tab is absent from the command registry: {page:?}"
        );
        assert_eq!(command.stable_id(), stable_id);
        assert_eq!(Command::from_stable_id(stable_id), Some(command));
        assert!(
            command.requires_open_project(),
            "project tab bypasses the open-project boundary: {page:?}"
        );
    }
}

#[test]
fn project_tab_commands_activate_the_project_workspace_and_exact_tab() {
    for page in ProjectPage::ALL {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.project_open = true;
        app.state.workbench.workspace = Workspace::Results;
        app.state.workbench.project_page = ProjectPage::Overview;

        Command::ProjectPage(page).execute(&mut app);

        assert_eq!(app.state.workbench.workspace, Workspace::Project);
        assert_eq!(app.state.workbench.project_page, page);
    }
}

/// Every Simulation Studio setup route is addressable, the way the Project,
/// Verify and Models pages are. A route reachable only by clicking the
/// navigator tree cannot be bound to a shortcut or driven from automation.
#[test]
fn every_simulation_setup_route_has_one_discoverable_command_with_a_stable_identity() {
    let expected = [
        (SimulationPage::Analyses, "simulation-analyses"),
        (SimulationPage::Variables, "simulation-variables"),
        (SimulationPage::Outputs, "simulation-outputs"),
        (SimulationPage::Specifications, "simulation-specifications"),
        (SimulationPage::RunSet, "simulation-run-set"),
        (SimulationPage::Models, "simulation-models"),
        (SimulationPage::Solver, "simulation-solver"),
        (SimulationPage::Save, "simulation-save-policy"),
    ];
    assert_eq!(SimulationPage::NAVIGATION.len(), expected.len());

    for (page, stable_id) in expected {
        let command = Command::SimulationPage(page);
        assert!(
            vocabulary::COMMAND_REGISTRY.contains(&command),
            "setup route is absent from the command registry: {page:?}"
        );
        assert_eq!(command.stable_id(), stable_id);
        assert_eq!(Command::from_stable_id(stable_id), Some(command));
        assert!(
            command.requires_open_project(),
            "setup route bypasses the open-project boundary: {page:?}"
        );
    }
}

#[test]
fn simulation_route_commands_activate_the_simulate_workspace_and_exact_route() {
    for page in SimulationPage::NAVIGATION {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.project_open = true;
        app.state.workbench.workspace = Workspace::Results;
        app.state.workbench.simulation_page = SimulationPage::Analyses;

        Command::SimulationPage(page).execute(&mut app);

        assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
        assert_eq!(app.state.workbench.simulation_page, page);
    }
}

#[test]
fn protected_commands_keep_the_exact_mockup_action_ids() {
    assert_eq!(Command::CommandPalette.spec().id, "command-palette");
    assert_eq!(Command::ToggleFocusMode.spec().id, "toggle-focus-mode");
    assert_eq!(Command::RunSimulation.spec().id, "start-run");
    assert_eq!(Command::StopSimulation.spec().id, "stop-run");
    assert_eq!(Command::OpenProject.spec().id, "open-project");
    assert_eq!(Command::OpenNetlist.spec().id, "open-netlist");
    assert_eq!(Command::NewProject.spec().id, "new-project");
    assert_eq!(Command::Save.spec().id, "save-project");
    assert_eq!(Command::CloseActiveDocument.spec().id, "close-document");
    assert_eq!(Command::ToggleFullScreen.spec().id, "full-screen");
    assert_eq!(Command::GenerateNetlist.spec().id, "generated-netlist");
    assert_eq!(Command::ToggleConsole.spec().id, "console");
    assert_eq!(Command::PreviousDocument.spec().id, "previous-document");
    assert_eq!(Command::NextDocument.spec().id, "next-document");
    assert_eq!(
        Command::CloseOtherDocuments.spec().id,
        "close-other-documents"
    );
    assert_eq!(Command::CloseAllDocuments.spec().id, "close-all-documents");
    assert_eq!(Command::WorkspaceLayouts.spec().id, "workspace-layouts");
    assert_eq!(Command::WindowManager.spec().id, "window-manager");
    assert_eq!(Command::HelpCenter.spec().id, "help-center");
    assert_eq!(Command::ReleaseNotes.spec().id, "release-notes");
    assert_eq!(Command::MigrationGuide.spec().id, "migration-guide");
    assert_eq!(Command::SystemDiagnostics.spec().id, "system-diagnostics");
    assert_eq!(Command::SupportBundle.spec().id, "support-bundle");
    assert_eq!(Command::LegalPrivacy.spec().id, "legal-privacy-center");
    assert_eq!(Command::OpenConsole.spec().id, "open-console");
    assert_eq!(Command::OpenProblems.spec().id, "open-problems");
    assert_eq!(
        Command::ToggleConsoleMaximized.spec().id,
        "console-maximize"
    );
    assert_eq!(Command::ClearConsole.spec().id, "console-clear");
    assert_eq!(
        Command::FeatureAvailability.spec(),
        CommandSpec {
            id: "feature-availability",
            label: "Product capability and platform matrix…",
            group: "Help",
        }
    );
    assert_eq!(
        Command::InteroperabilityMatrix.spec(),
        CommandSpec {
            id: "interoperability-matrix",
            label: "Interoperability and format matrix…",
            group: "Help",
        }
    );
    assert_eq!(
        Command::OpenWorkspace(Workspace::Results).spec().id,
        "results"
    );
    assert_eq!(
        Command::OpenWorkspace(Workspace::Verify).spec().id,
        "verify"
    );
    assert_eq!(
        Command::OpenWorkspace(Workspace::Models).spec().id,
        "models"
    );
    assert_eq!(
        Command::OpenWorkspace(Workspace::Netlist).spec().label,
        "Open automation workspace"
    );
    assert_eq!(
        Command::ModelsPage(ModelsPage::Models).spec().label,
        "Model & library catalog"
    );
}

#[test]
fn design_specialist_command_routes_to_the_existing_real_browser() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);

    let command = Command::DesignSpecialistWorkspaces;
    assert_eq!(command.stable_id(), "specialist-tools-design");
    assert_eq!(Command::from_stable_id(command.stable_id()), Some(command));
    assert_eq!(command.availability(&app), CommandAvailability::Available);

    command.execute(&mut app);

    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::SpecialistToolBrowser
    );
}

#[test]
fn design_menu_commands_explain_wrong_context_and_read_only_states() {
    let authoring_commands = [
        Command::PlaceInstance,
        Command::PlaceWire,
        Command::PlaceBus,
        Command::PlaceBusTap,
        Command::PlaceJunction,
        Command::PlaceLabel,
        Command::PlaceProbe,
        Command::PlacePin,
        Command::PlaceText,
        Command::PlaceShape,
        Command::MoveSelection,
        Command::StretchSelection,
        Command::ArraySelection,
        Command::ReplaceInstance,
        Command::CreateHierarchy,
    ];
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.workbench.activate(Workspace::Results);

    for command in authoring_commands {
        assert_eq!(
            command.availability(&app),
            CommandAvailability::Disabled("open an editable schematic or testbench"),
            "{command:?}"
        );
    }
    assert_eq!(
        Command::AscendHierarchy.availability(&app),
        CommandAvailability::Disabled("open a schematic or testbench")
    );
    assert_eq!(
        Command::DescendHierarchy.availability(&app),
        CommandAvailability::Disabled("open a schematic or testbench")
    );
    assert_eq!(
        Command::CheckAndSave.availability(&app),
        CommandAvailability::Disabled("open an editable schematic or testbench")
    );

    app.state.workbench.activate(Workspace::Design);
    app.state.schematic.read_only = true;
    for command in authoring_commands {
        assert_eq!(
            command.availability(&app),
            CommandAvailability::Disabled("the active schematic is read-only"),
            "{command:?}"
        );
    }
    assert_eq!(
        Command::CheckAndSave.availability(&app),
        CommandAvailability::Disabled("the active schematic is read-only")
    );

    app.state.project_lifecycle.project_open = false;
    assert_eq!(
        Command::OpenWorkspace(Workspace::Design).availability(&app),
        CommandAvailability::Disabled("no project is open")
    );
}

#[test]
fn only_exactly_implemented_reset_actions_are_discoverable() {
    let searchable = command_catalog().collect::<Vec<_>>();
    assert!(vocabulary::COMMAND_REGISTRY.contains(&Command::ResetActiveView));
    assert!(searchable.contains(&Command::ResetActiveView));
    assert!(vocabulary::COMMAND_REGISTRY.contains(&Command::ResetLayout));
    assert!(searchable.contains(&Command::ResetLayout));
    for command in [Command::PreviousWorkspace, Command::NextWorkspace] {
        assert!(!vocabulary::COMMAND_REGISTRY.contains(&command));
        assert!(!searchable.contains(&command));
    }
}

#[test]
fn project_operation_gate_covers_every_mutating_project_command() {
    for command in [
        Command::ProjectLauncher,
        Command::RecentProjects,
        Command::NewProject,
        Command::OpenProject,
        Command::Save,
        Command::SaveAs,
        Command::SaveAll,
        Command::RevertActiveDocument,
        Command::CloseActiveDocument,
        Command::CloseProject,
        Command::NewCell,
        Command::OpenDocument,
        Command::ImportNetlist,
        Command::ImportVerilogA,
        Command::ImportResultDataset,
        Command::CheckAndSave,
        Command::ModelEditor,
    ] {
        assert!(
            command.blocked_by_project_operation(),
            "ungated: {command:?}"
        );
    }
    assert!(!Command::Copy.blocked_by_project_operation());
    assert!(!Command::ExportWaveformsCsv.blocked_by_project_operation());
}

#[test]
fn result_dataset_import_has_mockup_authoritative_command_identity_and_gates() {
    assert_eq!(Command::ImportResultDataset.stable_id(), "import-dataset");
    assert_eq!(
        Command::ImportResultDataset.spec().label,
        "Import result dataset…"
    );
    assert_eq!(Command::ImportResultDataset.spec().group, "Results");
    assert!(vocabulary::COMMAND_REGISTRY.contains(&Command::ImportResultDataset));

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    assert!(Command::ImportResultDataset.is_enabled(&app));
    app.state.simulation.is_running = true;
    assert!(!Command::ImportResultDataset.is_enabled(&app));
}

#[test]
fn check_and_save_obeys_write_authority_and_opens_its_real_workflow() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    assert!(Command::CheckAndSave.is_enabled(&app));

    Command::CheckAndSave.execute(&mut app);
    assert!(app.state.dialogs.check_and_save.open);
    assert!(app.state.dialogs.check_and_save.report.is_some());

    app.state.dialogs.check_and_save.close();
    app.state.schematic.read_only = true;
    assert!(!Command::CheckAndSave.is_enabled(&app));

    app.state.schematic.read_only = false;
    app.state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::workbench::state::LocalSafeModeOptions::default()
        },
        "test session".to_owned(),
    );
    assert!(!Command::CheckAndSave.is_enabled(&app));
}

#[test]
fn configuration_sets_has_mockup_identity_and_opens_the_owned_workflow() {
    assert_eq!(Command::ConfigurationSets.stable_id(), "configuration-sets");
    assert_eq!(
        Command::ConfigurationSets.spec().label,
        "Configuration sets\u{2026}"
    );
    assert_eq!(Command::ConfigurationSets.spec().group, "Design");
    assert_eq!(
        Command::from_stable_id("configuration-sets"),
        Some(Command::ConfigurationSets)
    );
    assert!(Command::ConfigurationSets.shortcut_bindings().is_empty());

    let mut app = RSpiceApp::test_instance();
    assert!(Command::ConfigurationSets.is_enabled(&app));
    Command::ConfigurationSets.execute(&mut app);
    assert!(app.state.dialogs.configuration_sets.open);
    assert!(app.state.dialogs.application_modal_open());

    app.state.dialogs.configuration_sets.open = false;
    app.state.project_lifecycle.project_open = false;
    assert!(!Command::ConfigurationSets.is_enabled(&app));
}

#[test]
fn model_editor_command_has_mockup_identity_and_fail_closed_selection_authority() {
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};

    assert_eq!(Command::ModelEditor.stable_id(), "model-editor");
    assert_eq!(
        Command::ModelEditor.spec().label,
        "Device model and parameter editor\u{2026}"
    );
    assert_eq!(Command::ModelEditor.spec().group, "Models");
    assert_eq!(
        Command::from_stable_id("model-editor"),
        Some(Command::ModelEditor)
    );
    assert!(Command::ModelEditor.shortcut_bindings().is_empty());

    let registry_index = vocabulary::COMMAND_REGISTRY
        .iter()
        .position(|command| *command == Command::ModelEditor)
        .expect("model editor command must be registered");
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index - 1],
        Command::ModelCreateProjectCopy
    );

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.model_library_manager.selected_library = None;
    app.state.workbench.selected_model = None;
    assert_eq!(
        Command::ModelEditor.availability(&app),
        CommandAvailability::Disabled("select one model in Model & library catalog")
    );

    let mut built_in = ModelLibrary::new("command-editor-built-in");
    built_in.add_model(DeviceModel::new("readonly_nch", ModelType::Nmos));
    app.state.model_library_manager.add_library(built_in);
    app.state
        .model_library_manager
        .select_library("command-editor-built-in");
    app.state.workbench.selected_model = Some("readonly_nch".to_owned());
    assert_eq!(
        Command::ModelEditor.availability(&app),
        CommandAvailability::Disabled(
            "the selected model is built-in; create an editable project copy first"
        )
    );
}

#[test]
fn editable_project_copy_command_publishes_opens_and_records_undo_history() {
    use crate::state::model_library::{DeviceModel, ModelLibrary, ModelSourceAuthority, ModelType};

    assert_eq!(
        Command::ModelCreateProjectCopy.stable_id(),
        "model-create-project-copy"
    );
    assert_eq!(
        Command::ModelCreateProjectCopy.spec().label,
        "Create editable project copy"
    );
    assert_eq!(Command::ModelCreateProjectCopy.spec().group, "Models");
    assert_eq!(
        Command::from_stable_id("model-create-project-copy"),
        Some(Command::ModelCreateProjectCopy)
    );
    assert!(
        vocabulary::COMMAND_REGISTRY.contains(&Command::ModelCreateProjectCopy),
        "project-copy action must be reachable through the command registry"
    );
    assert!(Command::ModelCreateProjectCopy.blocked_by_project_operation());

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    let initial_project_revision = app.state.workspace.project.revision();
    let mut built_in = ModelLibrary::new("command copy built-in");
    built_in.pdk_name = "Example PDK".to_owned();
    built_in.technology_node = "45nm".to_owned();
    let mut model = DeviceModel::new("copy_nch", ModelType::Nmos);
    model.spice_type = Some("NMOS".to_owned());
    model.spice_level = Some(1);
    model.description = "Built-in command copy".to_owned();
    model.parameters.insert("vth0".to_owned(), 0.46);
    built_in.add_model(model);
    app.state.model_library_manager.add_library(built_in);
    app.state
        .model_library_manager
        .select_library("command copy built-in");
    app.state.workbench.selected_model = Some("copy_nch".to_owned());

    assert_eq!(
        Command::ModelCreateProjectCopy.availability(&app),
        CommandAvailability::Available
    );
    assert!(!Command::ModelEditor.is_enabled(&app));
    Command::ModelCreateProjectCopy.execute(&mut app);

    assert!(
        app.state.workspace.project.revision() > initial_project_revision,
        "copy publication advances the guarded project revision"
    );
    assert!(app.state.workspace.project_metadata_dirty);
    assert_eq!(
        app.state.model_library_manager.selected_library.as_deref(),
        Some("copy_nch project")
    );
    assert_eq!(
        app.state.workbench.selected_model.as_deref(),
        Some("copy_nch")
    );
    let project_copy = app
        .state
        .model_library_manager
        .get_library("copy_nch project")
        .expect("command publishes the copy");
    assert!(matches!(
        project_copy.source_authority,
        ModelSourceAuthority::ProjectOwned { .. }
    ));
    assert_eq!(project_copy.pdk_name, "Example PDK");
    assert_eq!(project_copy.technology_node, "45nm");
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::ModelEditor
    );
    let draft = app
        .state
        .workbench
        .model_editor
        .draft
        .as_ref()
        .expect("the exact committed copy opens in the editor");
    assert_eq!(draft.library_name, "copy_nch project");
    assert_eq!(draft.model_name, "copy_nch");
    assert_eq!(
        draft.base_project_revision,
        app.state.workspace.project.revision()
    );
    assert!(Command::ModelEditor.is_enabled(&app));

    assert!(app.state.can_undo_project_design());
    let undo_description = app
        .state
        .undo_project_design()
        .expect("copy undo succeeds")
        .expect("copy records one history item");
    assert!(undo_description.starts_with("create editable project model "));
    assert!(
        app.state
            .model_library_manager
            .get_library("copy_nch project")
            .is_none(),
        "undo removes the newly created project library"
    );
    assert!(app.state.can_redo_project_design());
    app.state
        .redo_project_design()
        .expect("copy redo succeeds")
        .expect("copy redo has one history item");
    assert!(
        app.state
            .model_library_manager
            .get_library("copy_nch project")
            .is_some(),
        "redo restores the authenticated project copy"
    );
}

#[test]
fn editable_project_copy_command_accepts_external_models_and_rejects_owned_or_read_only_state() {
    use std::collections::BTreeMap;

    use crate::state::model_library::{
        DeviceModel, ModelLibrary, ModelSourceAuthority, ModelType, ProjectModelDefinition,
    };

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    let mut external = ModelLibrary::new("external command source");
    external.source_authority = ModelSourceAuthority::External;
    external.add_model(DeviceModel::new("external_nch", ModelType::Nmos));
    app.state.model_library_manager.add_library(external);
    app.state
        .model_library_manager
        .select_library("external command source");
    app.state.workbench.selected_model = Some("external_nch".to_owned());
    assert_eq!(
        Command::ModelCreateProjectCopy.availability(&app),
        CommandAvailability::Available
    );

    app.state
        .model_library_manager
        .create_project_model(
            "already owned",
            &ProjectModelDefinition {
                name: "owned_nch".to_owned(),
                spice_type: "NMOS".to_owned(),
                description: "Already editable".to_owned(),
                numeric_parameters: BTreeMap::new(),
                string_parameters: BTreeMap::new(),
            },
        )
        .expect("owned fixture");
    app.state
        .model_library_manager
        .select_library("already owned");
    app.state.workbench.selected_model = Some("owned_nch".to_owned());
    assert_eq!(
        Command::ModelCreateProjectCopy.availability(&app),
        CommandAvailability::Disabled("the selected model is already an editable project copy")
    );

    app.state
        .model_library_manager
        .select_library("external command source");
    app.state.workbench.selected_model = Some("external_nch".to_owned());
    app.state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::workbench::state::LocalSafeModeOptions::default()
        },
        "read-only copy test".to_owned(),
    );
    assert_eq!(
        Command::ModelCreateProjectCopy.availability(&app),
        CommandAvailability::Disabled("the project is open read-only")
    );
}

#[test]
fn model_editor_command_accepts_one_coherent_project_owned_definition() {
    use std::collections::BTreeMap;

    use crate::state::model_library::ProjectModelDefinition;

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    let commit = app
        .state
        .model_library_manager
        .create_project_model(
            "command-editor-owned",
            &ProjectModelDefinition {
                name: "command_nch".to_owned(),
                spice_type: "NMOS".to_owned(),
                description: "Command dispatch fixture".to_owned(),
                numeric_parameters: BTreeMap::from([
                    ("level".to_owned(), 1.0),
                    ("vth0".to_owned(), 0.48),
                ]),
                string_parameters: BTreeMap::new(),
            },
        )
        .expect("create coherent project-owned model");
    app.state
        .model_library_manager
        .select_library(&commit.library_name);
    app.state.workbench.selected_model = Some(commit.model_name);

    assert_eq!(
        Command::ModelEditor.availability(&app),
        CommandAvailability::Available
    );
    assert!(Command::ModelEditor.is_enabled(&app));

    app.state.workbench.safe_mode.activate(
        crate::workbench::state::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::workbench::state::LocalSafeModeOptions::default()
        },
        "read-only model review".to_owned(),
    );
    assert_eq!(
        Command::ModelEditor.availability(&app),
        CommandAvailability::Available
    );
    Command::ModelEditor.execute(&mut app);
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::ModelEditor
    );
    assert!(app.state.workbench.model_editor.draft.is_some());
    assert_eq!(
        Command::ModelSaveRevision.availability(&app),
        CommandAvailability::Disabled("the project is open read-only")
    );
    assert_eq!(
        Command::ModelRunQualificationTests.availability(&app),
        CommandAvailability::Disabled("qualification cannot run while the project is read-only")
    );
    assert!(Command::ModelValidate.is_enabled(&app));
    Command::ModelValidate.execute(&mut app);
    assert_eq!(
        active_model_editor_workflow(&app).map(|request| request.workflow),
        Some(ModelEditorWorkflow::ValidateCandidate)
    );
    close_model_editor_workflow();
}

#[test]
fn model_editor_command_requires_an_open_project_even_with_a_retained_selection() {
    use std::collections::BTreeMap;

    use crate::state::model_library::ProjectModelDefinition;

    let mut app = RSpiceApp::test_instance();
    let commit = app
        .state
        .model_library_manager
        .create_project_model(
            "command-editor-closed-project",
            &ProjectModelDefinition {
                name: "retained_nch".to_owned(),
                spice_type: "NMOS".to_owned(),
                description: "Retained selection without an open project".to_owned(),
                numeric_parameters: BTreeMap::from([("level".to_owned(), 1.0)]),
                string_parameters: BTreeMap::new(),
            },
        )
        .expect("create retained project-owned model");
    app.state
        .model_library_manager
        .select_library(&commit.library_name);
    app.state.workbench.selected_model = Some(commit.model_name);
    app.state.project_lifecycle.project_open = false;

    assert_eq!(
        Command::ModelEditor.availability(&app),
        CommandAvailability::Disabled("no project is open")
    );
    assert!(!Command::ModelEditor.is_enabled(&app));
    assert_eq!(
        selected_project_model_for_editor(&app),
        Err("open a project before editing a device model")
    );
}

#[test]
fn qualification_command_requires_a_suite_for_the_exact_open_source() {
    use std::collections::BTreeMap;

    use crate::state::model_library::ProjectModelDefinition;

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state
        .model_library_manager
        .create_project_model(
            "command-qualification-owned",
            &ProjectModelDefinition {
                name: "qualification_nch".to_owned(),
                spice_type: "NMOS".to_owned(),
                description: "Qualification command fixture".to_owned(),
                numeric_parameters: BTreeMap::from([("level".to_owned(), 1.0)]),
                string_parameters: BTreeMap::new(),
            },
        )
        .expect("create project model");
    let project_revision = app.state.workspace.project.revision();
    app.state
        .workbench
        .model_editor
        .open(
            &app.state.model_library_manager,
            "command-qualification-owned",
            "qualification_nch",
            project_revision,
        )
        .expect("open editor");
    app.state.workbench.model_editor.begin_qualification_suite();
    let authoring = &mut app.state.workbench.model_editor.qualification_authoring;
    authoring.suite_id = "dc-op".to_owned();
    authoring.suite_name = "DC operating point".to_owned();
    authoring.vector_id = "nominal".to_owned();
    authoring.vector_name = "Nominal bias".to_owned();
    authoring.executable_input =
        "V1 out 0 1\nR1 out 0 1k\nMbind 0 0 0 0 qualification_nch\n.op\n.end\n".to_owned();
    authoring.quantity = "v(out)".to_owned();
    authoring.probe_target = "out".to_owned();
    authoring.expected = "1".to_owned();
    authoring.absolute_tolerance = "1e-9".to_owned();
    authoring.relative_tolerance = "1e-9".to_owned();
    assert!(
        app.state
            .workbench
            .model_editor
            .commit_qualification_suite()
    );
    assert!(
        app.state
            .workbench
            .model_editor
            .validate_candidate(&app.state.model_library_manager, project_revision)
    );
    assert!(Command::ModelRunQualificationTests.is_enabled(&app));

    app.state
        .workbench
        .model_editor
        .draft
        .as_mut()
        .expect("draft")
        .qualification
        .suites[0]
        .vectors[0]
        .source
        .source_id = Some(crate::product::ModelSourceId::new());
    assert!(
        app.state
            .workbench
            .model_editor
            .draft
            .as_ref()
            .expect("draft")
            .qualification
            .validate_for_model("qualification_nch")
            .is_ok()
    );
    assert!(!Command::ModelRunQualificationTests.is_enabled(&app));
}

#[test]
fn design_management_has_mockup_identity_authority_and_owned_workflow() {
    assert_eq!(Command::DesignManagement.stable_id(), "design-management");
    assert_eq!(
        Command::DesignManagement.spec().label,
        "Sheets, variants and annotation\u{2026}"
    );
    assert_eq!(Command::DesignManagement.spec().group, "Design");
    assert_eq!(
        Command::from_stable_id("design-management"),
        Some(Command::DesignManagement)
    );
    assert!(Command::DesignManagement.shortcut_bindings().is_empty());

    let registry_index = vocabulary::COMMAND_REGISTRY
        .iter()
        .position(|command| *command == Command::DesignManagement)
        .expect("design-management must be registered");
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index - 1],
        Command::CreateHierarchy
    );
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index + 1],
        Command::SelectionBulkEdit
    );

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    assert!(Command::DesignManagement.is_enabled(&app));
    Command::DesignManagement.execute(&mut app);
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::DesignManagement
    );
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert!(app.state.dialogs.design_management.open);
    assert!(app.state.dialogs.application_modal_open());

    app.state.dialogs.design_management.open = false;
    app.state.schematic.read_only = true;
    assert!(!Command::DesignManagement.is_enabled(&app));

    app.state.schematic.read_only = false;
    app.state.project_lifecycle.project_open = false;
    assert!(!Command::DesignManagement.is_enabled(&app));
}

#[test]
fn connectivity_manager_has_mockup_identity_and_supports_read_only_inspection() {
    assert_eq!(
        Command::ConnectivityManager.stable_id(),
        "design-connectivity-tools"
    );
    assert_eq!(
        Command::ConnectivityManager.spec().label,
        "Connectivity and bus manager\u{2026}"
    );
    assert_eq!(Command::ConnectivityManager.spec().group, "Design");
    assert_eq!(
        Command::from_stable_id("design-connectivity-tools"),
        Some(Command::ConnectivityManager)
    );
    assert!(Command::ConnectivityManager.shortcut_bindings().is_empty());

    let registry_index = vocabulary::COMMAND_REGISTRY
        .iter()
        .position(|command| *command == Command::ConnectivityManager)
        .expect("connectivity manager must be registered");
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index - 1],
        Command::SelectionBulkEdit
    );
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index + 1],
        Command::ConfigurationSets
    );

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    app.state.schematic.read_only = true;
    assert!(Command::ConnectivityManager.is_enabled(&app));
    Command::ConnectivityManager.execute(&mut app);
    assert!(app.state.dialogs.connectivity_manager.open);
    assert!(app.state.dialogs.application_modal_open());

    app.state.dialogs.connectivity_manager.open = false;
    app.state.project_lifecycle.project_open = false;
    assert!(!Command::ConnectivityManager.is_enabled(&app));
}

#[test]
fn selection_bulk_edit_has_mockup_identity_order_and_read_only_inspection() {
    assert_eq!(Command::SelectionBulkEdit.stable_id(), "design-bulk-tools");
    assert_eq!(
        Command::SelectionBulkEdit.spec().label,
        "Selection and bulk editing\u{2026}"
    );
    assert_eq!(Command::SelectionBulkEdit.spec().group, "Design");
    assert_eq!(
        Command::from_stable_id("design-bulk-tools"),
        Some(Command::SelectionBulkEdit)
    );
    assert!(Command::SelectionBulkEdit.shortcut_bindings().is_empty());

    let registry_index = vocabulary::COMMAND_REGISTRY
        .iter()
        .position(|command| *command == Command::SelectionBulkEdit)
        .expect("selection bulk edit must be registered");
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index - 1],
        Command::DesignManagement
    );
    assert_eq!(
        vocabulary::COMMAND_REGISTRY[registry_index + 1],
        Command::ConnectivityManager
    );

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    app.state.schematic.read_only = true;
    assert!(Command::SelectionBulkEdit.is_enabled(&app));
    Command::SelectionBulkEdit.execute(&mut app);
    assert!(app.state.dialogs.selection_bulk_edit.open);
    assert!(app.state.dialogs.application_modal_open());

    app.state.dialogs.selection_bulk_edit.open = false;
    app.state.project_lifecycle.project_open = false;
    assert!(!Command::SelectionBulkEdit.is_enabled(&app));
}

#[test]
fn stop_command_follows_the_execution_target_capability() {
    assert!(!stop_simulation_enabled(false));
    assert_eq!(
        stop_simulation_enabled(true),
        crate::simulation::execution::execution_target_supports_cancellation()
    );
}

#[test]
fn closed_projects_expose_only_the_project_workspace() {
    assert!(workspace_available(false, Workspace::Project));
    for workspace in Workspace::ALL {
        if workspace != Workspace::Project {
            assert!(!workspace_available(false, workspace));
        }
        assert!(workspace_available(true, workspace));
    }
}

#[test]
fn new_cell_command_captures_exact_library_catalog_revision() {
    let mut app = RSpiceApp::test_instance();
    let revision = app.state.library_manager.revision();

    Command::NewCell.execute(&mut app);

    assert!(app.state.dialogs.new_cell_dialog);
    assert_eq!(app.state.dialogs.new_cell_library_revision, revision);
}

#[test]
fn project_owned_subcommands_cannot_bypass_the_closed_project_boundary() {
    // This independent expectation list prevents the predicate under test
    // from silently omitting a newly exposed submenu route.
    for command in [
        Command::NewCell,
        Command::ImportNetlist,
        Command::ImportVerilogA,
        Command::ImportResultDataset,
        Command::ExportSchematicSvg,
        Command::ExportWaveformsCsv,
        Command::ExportNetlist(crate::io::NetlistFormat::Spice),
        Command::FindInDesign,
        Command::CheckAndSave,
        Command::SelectionBulkEdit,
        Command::ConnectivityManager,
        Command::ProjectPage(ProjectPage::Overview),
        Command::ProjectPage(ProjectPage::Library),
        Command::ProjectPage(ProjectPage::Configuration),
        Command::ProjectPage(ProjectPage::Dependencies),
        Command::ProjectPage(ProjectPage::Recovery),
        Command::SimulationPage(SimulationPage::Variables),
        Command::PreflightChecks,
        Command::SimulationOptions,
        Command::GenerateNetlist,
        Command::WaveformCalculator,
        Command::CompareResultDatasets,
        Command::ResultViewer(crate::workbench::ResultViewer::Waves),
        Command::EditSpecifications,
        Command::VerificationPage(VerificationPage::Yield),
        Command::ModelsPage(ModelsPage::Models),
        Command::ModelBrowser,
        Command::ModelEditor,
        Command::PdkSettings,
        Command::RescanModelLibraries,
        Command::CompileVerilogA,
        Command::AutomationConsole,
        Command::VisualizationStudio,
        Command::ReportAuthoring,
    ] {
        assert!(
            command.requires_open_project(),
            "missing closed-project boundary: {command:?}"
        );
    }

    let commands: Vec<_> = vocabulary::COMMAND_REGISTRY
        .iter()
        .copied()
        .filter(|command| command.requires_open_project())
        .collect();
    assert!(!commands.is_empty());

    for command in commands {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.project_open = false;
        app.state.workbench.workspace = Workspace::Project;

        assert!(
            !command.is_enabled(&app),
            "enabled without project: {command:?}"
        );
        assert_eq!(
            command.availability(&app),
            CommandAvailability::Disabled("no project is open"),
            "wrong closed-project reason for {command:?}"
        );

        command.execute(&mut app);

        assert_eq!(
            app.state.workbench.workspace,
            Workspace::Project,
            "closed-project command changed workspace: {command:?}"
        );
        assert!(
            app.state
                .log_buffer
                .entries()
                .any(|entry| entry.message == "Open a project before using this command."),
            "closed-project command did not explain its boundary: {command:?}"
        );
    }
}

#[test]
fn model_library_rescan_discovers_files_and_reports_path_errors() {
    let nonce = crate::time_compat::unix_epoch().as_nanos();
    let root = std::env::temp_dir().join(format!(
        "rspice-command-rescan-{}-{nonce}",
        std::process::id()
    ));
    std::fs::create_dir_all(&root).expect("create model-library fixture");
    std::fs::write(root.join("device.lib"), ".model dtest d\n")
        .expect("write model-library fixture");

    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.pdk_config = crate::state::pdk_config::PdkConfig::new();
    app.state
        .pdk_config
        .add_library_path(root.to_string_lossy().into_owned());
    let ctx = egui::Context::default();

    Command::RescanModelLibraries.execute_with_feedback(&mut app, &ctx);

    assert_eq!(app.state.pdk_config.discovered_files.len(), 1);
    let success = app
        .state
        .log_buffer
        .entries()
        .last()
        .expect("rescan receipt");
    assert_eq!(success.severity, crate::diagnostics::LogSeverity::Info);
    assert!(success.message.contains("found 1 configured model file(s)"));
    assert_eq!(
        app.state.ui.toasts.activity()[0].kind(),
        crate::ui::widgets::ToastKind::Success
    );
    assert!(
        app.state.ui.toasts.activity()[0]
            .message()
            .contains("found 1 configured model file(s)")
    );

    app.state
        .pdk_config
        .add_library_path(root.join("missing").to_string_lossy().into_owned());
    Command::RescanModelLibraries.execute_with_feedback(&mut app, &ctx);

    assert_eq!(app.state.pdk_config.discovered_files.len(), 1);
    let warning = app
        .state
        .log_buffer
        .entries()
        .last()
        .expect("warning receipt");
    assert_eq!(warning.severity, crate::diagnostics::LogSeverity::Warning);
    assert!(warning.message.contains("1 configured path error(s)"));
    assert!(warning.message.contains("Path does not exist"));
    assert_eq!(
        app.state.ui.toasts.activity()[0].kind(),
        crate::ui::widgets::ToastKind::Warn
    );
    assert!(
        app.state.ui.toasts.activity()[0]
            .message()
            .contains("1 configured path error(s)")
    );

    std::fs::remove_dir_all(&root).expect("remove model-library fixture");
}

#[test]
fn standalone_schematic_save_remains_available_without_a_project() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = false;
    app.state.schematic.current_file = Some("standalone.rsch".into());

    assert!(!Command::Save.requires_open_project());
    assert!(Command::Save.is_enabled(&app));
    assert_eq!(
        Command::Save.availability(&app),
        CommandAvailability::Available
    );

    app.state.schematic.current_file = None;
    app.state.browser_schematic_save_name = Some("browser-import.rsch".to_owned());
    assert!(Command::Save.is_enabled(&app));
    assert_eq!(
        Command::Save.availability(&app),
        CommandAvailability::Available
    );
}

#[test]
fn recent_projects_opens_the_launcher_on_the_real_recent_filter() {
    let mut workbench = WorkbenchState::default();
    workbench.project_launcher_filter = ProjectLauncherFilter::Pinned;
    workbench.project_launcher_open = false;
    workbench.focus_project_launcher_search = false;

    open_recent_projects(&mut workbench);

    assert!(workbench.project_launcher_open);
    assert!(workbench.focus_project_launcher_search);
    assert_eq!(
        workbench.project_launcher_page,
        crate::workbench::state::ProjectLauncherPage::Projects
    );
    assert_eq!(
        workbench.project_launcher_filter,
        ProjectLauncherFilter::Recent
    );
}

#[test]
fn every_workspace_exposes_the_mockup_reset_active_view_workflow() {
    for workspace in Workspace::ALL {
        assert!(
            reset_active_view_available(workspace),
            "{workspace:?} has implemented reset behavior"
        );
    }
}

#[test]
fn full_screen_command_opens_review_before_host_or_layout_mutation() {
    let mut app = RSpiceApp::test_instance();
    assert!(!app.state.workbench.full_screen_presentation);
    assert_eq!(app.state.ui.take_full_screen_request(), None);

    Command::ToggleFullScreen.execute(&mut app);

    assert!(app.state.dialogs.view_operation.open);
    assert_eq!(
        app.state.dialogs.view_operation.operation,
        crate::workbench::app::ViewOperation::FullScreen
    );
    assert!(app.state.dialogs.application_modal_open());
    assert!(!app.state.workbench.full_screen_presentation);
    assert_eq!(app.state.ui.take_full_screen_request(), None);
}

#[test]
fn reset_active_view_command_captures_the_exact_workspace_for_review() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Results);

    Command::ResetActiveView.execute(&mut app);

    assert!(app.state.dialogs.view_operation.open);
    assert_eq!(
        app.state.dialogs.view_operation.operation,
        crate::workbench::app::ViewOperation::ResetActiveView
    );
    assert_eq!(
        app.state.dialogs.view_operation.workspace,
        Workspace::Results
    );
    assert!(app.state.dialogs.application_modal_open());
}

#[test]
fn resetting_the_simulation_view_clears_the_narrowing_and_not_the_plan() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Simulate);
    app.state.workbench.saved_output_filter = "db20".to_owned();
    app.state.workbench.specification_filter = "bw(".to_owned();
    app.state.workbench.specification_evidence_filter =
        crate::workbench::state::SpecificationEvidenceFilter::Failing;
    app.state.workbench.selected_specification = Some("bw(vout)".to_owned());

    super::reset_active_view(&mut app);

    assert!(app.state.workbench.saved_output_filter.is_empty());
    assert!(app.state.workbench.specification_filter.is_empty());
    assert_eq!(
        app.state.workbench.specification_evidence_filter,
        crate::workbench::state::SpecificationEvidenceFilter::All
    );
    // A view reset restores what the reader can see, not what the plan says
    // or which record they were reading.
    assert_eq!(
        app.state.workbench.selected_specification.as_deref(),
        Some("bw(vout)")
    );
}

#[test]
fn repeated_violation_navigation_keeps_advancing_after_jump_to_design() {
    use crate::services::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

    let mut app = RSpiceApp::test_instance();
    let mut result = DrcResult::new();
    for (id, x) in [(1, 10.0), (2, 20.0)] {
        result.add_violation(DrcViolation::new(
            id,
            DrcViolationType::DanglingWire,
            format!("anchored finding {id}"),
            DrcLocation::Point { x, y: 0.0 },
        ));
    }
    app.state.dialogs.drc_checked_version = app.state.schematic.topology_version();
    app.state.dialogs.drc_results = Some(result);
    app.state.workbench.activate(Workspace::Verify);

    for expected_cycle in [0, 1] {
        Command::NextViolation.execute(&mut app);
        assert_eq!(app.state.workbench.workspace, Workspace::Design);
        assert_eq!(app.state.dialogs.drc_cycle, Some(expected_cycle));
        assert!(app.state.schematic.center_request.is_some());
    }
}

/// Every one of these opens a window drawn only by the netlist page. Offered
/// from a sibling page they set a dialog open with nothing on screen, and it
/// then appeared unprompted when the user navigated back.
#[test]
fn netlist_document_commands_are_not_offered_from_the_sibling_code_pages() {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Netlist;
    app.state.ui.code_workspace.page = CodeWorkspacePage::Netlist;
    app.state.simulation.netlist_content = "deck\n.end\n".to_owned();

    assert!(Command::ValidateCodeDocument.is_enabled(&app));

    for page in [CodeWorkspacePage::VerilogA, CodeWorkspacePage::Automation] {
        app.state.ui.code_workspace.page = page;
        assert!(
            !Command::ValidateCodeDocument.is_enabled(&app),
            "{page:?} does not own the netlist deck"
        );
        assert!(
            !Command::CompareGeneratedRevisions.is_enabled(&app),
            "{page:?} does not draw the revision comparison"
        );
    }
}

/// Find is the exception to the rule above: it is page-scoped rather than
/// netlist-only. Each page has a find window of its own -- the deck's on the
/// netlist page, the bundle search on the two language pages -- so the command
/// dispatches per page instead of being withheld from two of the three. It was
/// netlist-only for exactly as long as the bundle search had no surface.
#[test]
fn find_opens_the_window_the_visible_code_page_owns() {
    use crate::workbench::documents::code_workspace::CodeWorkspacePage;

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Netlist;

    app.state.ui.code_workspace.page = CodeWorkspacePage::Netlist;
    assert!(Command::FindCodeDocument.is_enabled(&app));
    Command::FindCodeDocument.execute(&mut app);
    assert!(app.state.ui.netlist.find.open, "the deck find opened");
    assert!(
        app.state.ui.code_workspace.source_search.is_none(),
        "the deck is not a project source bundle"
    );

    app.state.ui.netlist.find.open = false;
    app.state.ui.code_workspace.page = CodeWorkspacePage::Automation;
    assert!(Command::FindCodeDocument.is_enabled(&app));
    Command::FindCodeDocument.execute(&mut app);
    assert!(
        app.state.ui.code_workspace.source_search.is_some(),
        "the bundle search opened"
    );
    assert!(
        !app.state.ui.netlist.find.open,
        "and it did not also open the deck's find window"
    );
}

/// Navigating to the Verilog-A page is not compiling it. The stage's Compile
/// button dispatches this command, so an execute that only switched pages made
/// the button a no-op on the page it lives on.
#[test]
fn compiling_veriloga_requests_the_compile_and_not_only_the_page() {
    let mut app = RSpiceApp::test_instance();
    Command::CompileVerilogA.execute(&mut app);

    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
    assert_eq!(
        app.state.ui.code_workspace.page,
        crate::workbench::documents::code_workspace::CodeWorkspacePage::VerilogA
    );
    assert!(app.state.ui.code_workspace.veriloga.compile_requested);
}
