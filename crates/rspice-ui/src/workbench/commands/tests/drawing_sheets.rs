//! Commands owning the drawing sheet: page setup, formats, and preset sizes.
//!
//! Sheet setup belongs to a schematic document; output media belongs to
//! print and export. Keeping the two apart is the invariant here — an
//! authored symbol is a valid hardcopy source, so Page Setup must report
//! unavailable there rather than substitute the output-media dialog that
//! happens to be reachable.

use super::*;

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
