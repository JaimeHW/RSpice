//! Commands owning the drawing sheet: its properties, formats, and preset
//! sizes.
//!
//! The sheet belongs to a schematic document; output media belongs to print
//! and export. Keeping the two apart is the invariant here — an authored
//! symbol is a valid hardcopy source, so Sheet properties must report
//! unavailable there rather than substitute the output-media dialog that
//! happens to be reachable.

use super::*;

#[test]
fn drawing_sheet_file_commands_keep_the_canonical_palette_identities() {
    let expected = [
        // The stable ID shortcut profiles saved it under as Page setup.
        (Command::SheetProperties, "page-setup", "Sheet properties"),
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
        (Command::SheetDefaults, "sheet-defaults", "Sheet defaults…"),
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
fn sheet_properties_never_fall_back_to_symbol_hardcopy_media() {
    let mut app = app_with_selected_authored_symbol();
    app.state.project_lifecycle.authority.open_session();
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

    assert!(!Command::SheetProperties.is_enabled(&app));
    Command::SheetProperties.execute(&mut app);

    assert!(!app.state.workbench.reveal_sheet_section);
    assert!(
        !app.state.dialogs.hardcopy.open,
        "Sheet properties must not substitute output-media setup"
    );
}

/// Sheet properties show the sheet where it is edited: the selection is
/// cleared, so the inspector is the sheet's; the inspector opens where it was
/// closed; and the sheet panel is asked to bring its Sheet section into view.
#[test]
fn sheet_properties_open_the_inspector_on_the_sheet() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    crate::workbench::examples::load_example("RC Lowpass Filter", &mut app.state.schematic);
    app.state.select_all_schematic_objects();
    app.state.workbench.inspector_visible = false;
    assert!(Command::SheetProperties.is_enabled(&app));

    Command::SheetProperties.execute(&mut app);

    assert!(app.state.schematic.session.editor.selection.is_empty());
    assert!(app.state.workbench.inspector_visible);
    assert!(app.state.workbench.reveal_sheet_section);
}

#[test]
fn document_sheet_commands_open_their_real_surfaces_only_in_schematic_context() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app.state.workbench.workspace = Workspace::Design;
    let key = app.state.workspace.content.active_key();
    app.state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Main", [])
        .unwrap();

    assert!(Command::SheetFormatManager.is_enabled(&app));
    assert!(Command::CustomSheetSizes.is_enabled(&app));

    Command::SheetFormatManager.execute(&mut app);
    assert!(app.state.dialogs.drawing_sheet_support.manager.open);

    Command::CustomSheetSizes.execute(&mut app);
    assert!(app.state.dialogs.drawing_sheet_presets.any_open());

    // The project default is where a format reaches several sheets, so its
    // dialog is a command of its own.
    assert!(Command::SheetDefaults.is_enabled(&app));
    Command::SheetDefaults.execute(&mut app);
    assert!(app.state.dialogs.drawing_sheet_defaults.open);

    let mut results = RSpiceApp::test_instance();
    results.state.project_lifecycle.authority.open_session();
    results.state.workbench.workspace = Workspace::Results;
    assert!(!Command::SheetProperties.is_enabled(&results));
    assert!(!Command::SheetFormatManager.is_enabled(&results));
    assert!(!Command::CustomSheetSizes.is_enabled(&results));
    assert!(!Command::SheetDefaults.is_enabled(&results));
}

#[test]
fn sheet_format_manager_requires_live_schematic_edit_authority() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app.state.workbench.workspace = Workspace::Design;
    let key = app.state.workspace.content.active_key();
    app.state
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Main", [])
        .unwrap();
    app.state.schematic.session.read_only = true;

    assert_eq!(
        Command::SheetFormatManager.availability(&app),
        CommandAvailability::Disabled("the active schematic is read-only")
    );
    Command::SheetFormatManager.execute(&mut app);
    assert!(!app.state.dialogs.drawing_sheet_support.manager.open);
}
