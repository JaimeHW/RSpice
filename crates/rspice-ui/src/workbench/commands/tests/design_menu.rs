//! The Design menu's commands and the workflow each of them owns.
//!
//! Every entry keeps the exact identity, group and registry position the
//! mockup fixes, and opens the real workflow it names rather than a surface
//! that resembles it. Inspection routes stay available on a read-only design;
//! only the mutating ones explain the refusal and decline.

use super::*;

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
