//! The Design menu's commands and the workflow each of them owns.
//!
//! Every entry opens the real workflow it names rather than a surface that
//! resembles it. Inspection routes stay available on a read-only design;
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
    app.state.open_project_session();
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
    app.state.model.schematic.session.read_only = true;
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

    app.state.close_project_session();
    assert_eq!(
        Command::OpenWorkspace(Workspace::Design).availability(&app),
        CommandAvailability::Disabled("no project is open")
    );
}

/// The one check command became two, and a shortcut profile that bound the
/// old one keeps checking the cellview in front of the reader.
#[test]
fn the_check_commands_name_their_reach_and_keep_the_check_family_keys() {
    assert_eq!(Command::CheckCellview.stable_id(), "check-cellview");
    assert_eq!(Command::CheckCellview.spec().label, "Check cellview");
    assert_eq!(Command::CheckDesign.stable_id(), "check-design");
    assert_eq!(Command::CheckDesign.spec().label, "Check design");
    assert_eq!(
        Command::from_stable_id("run-checks"),
        Some(Command::CheckCellview)
    );
    for command in [Command::CheckCellview, Command::CheckDesign] {
        assert_eq!(command.spec().group, "Design");
        assert!(command.palette_visible(), "{command:?}");
        assert!(
            vocabulary::COMMAND_REGISTRY.contains(&command),
            "{command:?}"
        );
    }
    // The cellview check keeps the key the one check had, beside Check and
    // Save's; the design check takes no chord the family has left, because
    // Ctrl+Alt+E is AltGr+E on most European layouts.
    assert_eq!(
        Command::CheckCellview.shortcut_bindings()[0].chord.label,
        "Ctrl+E"
    );
    assert_eq!(
        Command::CheckAndSave.shortcut_bindings()[0].chord.label,
        "Ctrl+Shift+E"
    );
    assert!(Command::CheckDesign.shortcut_bindings().is_empty());
}

/// Check Design checks the configured design whichever view is open; Check
/// Cellview needs a drawing in front of the reader.
#[test]
fn check_design_is_offered_wherever_a_project_is_open() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Results);
    assert!(Command::CheckDesign.is_enabled(&app));
    assert!(!Command::CheckCellview.is_enabled(&app));

    app.state.workbench.activate(Workspace::Design);
    assert!(Command::CheckCellview.is_enabled(&app));

    app.state.close_project_session();
    assert!(!Command::CheckDesign.is_enabled(&app));
}

#[test]
fn check_and_save_obeys_write_authority_and_opens_its_real_workflow() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    assert!(Command::CheckAndSave.is_enabled(&app));

    Command::CheckAndSave.execute(&mut app);
    assert!(app.state.project.check_and_save.open);
    assert!(app.state.project.check_and_save.report.is_some());

    app.state.project.check_and_save.close();
    app.state.model.schematic.session.read_only = true;
    assert!(!Command::CheckAndSave.is_enabled(&app));

    app.state.model.schematic.session.read_only = false;
    app.state.model.safe_mode.activate(
        crate::state::write_policy::LocalSafeModeOptions {
            open_project_read_only: true,
            ..crate::state::write_policy::LocalSafeModeOptions::default()
        },
        "test session".to_owned(),
    );
    assert!(!Command::CheckAndSave.is_enabled(&app));
}

#[test]
fn configuration_sets_opens_the_owned_workflow() {
    let mut app = RSpiceApp::test_instance();
    assert!(Command::ConfigurationSets.is_enabled(&app));
    Command::ConfigurationSets.execute(&mut app);
    assert!(app.state.simulate.configuration_sets.open);
    assert!(app.state.dialog_modal_open());

    app.state.simulate.configuration_sets.open = false;
    app.state.close_project_session();
    assert!(!Command::ConfigurationSets.is_enabled(&app));
}

#[test]
fn design_management_has_authority_and_an_owned_workflow() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    assert!(Command::DesignManagement.is_enabled(&app));
    Command::DesignManagement.execute(&mut app);
    assert_eq!(
        app.state.workbench.current_route().surface_id(),
        crate::workbench::SurfaceId::DesignManagement
    );
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert!(app.state.project.design_management.open);
    assert!(app.state.dialog_modal_open());

    app.state.project.design_management.open = false;
    app.state.model.schematic.session.read_only = true;
    assert!(!Command::DesignManagement.is_enabled(&app));

    app.state.model.schematic.session.read_only = false;
    app.state.close_project_session();
    assert!(!Command::DesignManagement.is_enabled(&app));
}

#[test]
fn connectivity_manager_supports_read_only_inspection() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    app.state.model.schematic.session.read_only = true;
    assert!(Command::ConnectivityManager.is_enabled(&app));
    Command::ConnectivityManager.execute(&mut app);
    assert!(app.state.design.connectivity_manager.open);
    assert!(app.state.dialog_modal_open());

    app.state.design.connectivity_manager.open = false;
    app.state.close_project_session();
    assert!(!Command::ConnectivityManager.is_enabled(&app));
}

#[test]
fn selection_bulk_edit_supports_read_only_inspection() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    app.state.model.schematic.session.read_only = true;
    assert!(Command::SelectionBulkEdit.is_enabled(&app));
    Command::SelectionBulkEdit.execute(&mut app);
    assert!(app.state.design.selection_bulk_edit.open);
    assert!(app.state.dialog_modal_open());

    app.state.design.selection_bulk_edit.open = false;
    app.state.close_project_session();
    assert!(!Command::SelectionBulkEdit.is_enabled(&app));
}
