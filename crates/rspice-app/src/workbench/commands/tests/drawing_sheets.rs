//! Commands owning the drawing sheet: its properties, formats, and preset
//! sizes.
//!
//! The sheet belongs to a schematic document; output media belongs to print
//! and export. Keeping the two apart is the invariant here — an authored
//! symbol is a valid hardcopy source, so Sheet properties must report
//! unavailable there rather than substitute the output-media dialog that
//! happens to be reachable.

use super::*;
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::framework::command_registry::CommandFlags;

#[test]
fn drawing_sheet_file_commands_keep_the_canonical_palette_identities() {
    let expected = [
        // The stable ID shortcut profiles saved it under as Page setup.
        (
            SheetsCommand::SheetProperties,
            "page-setup",
            "Sheet properties",
        ),
        (
            SheetsCommand::SheetFormatManager,
            "sheet-format-manager",
            "Sheet formats across this document…",
        ),
        (
            SheetsCommand::CustomSheetSizes,
            "sheet-preset-library",
            "Custom sheet sizes…",
        ),
        (
            SheetsCommand::SheetDefaults,
            "sheet-defaults",
            "Sheet defaults…",
        ),
    ];
    let registry = command_registry();
    let searchable = registry
        .ordered()
        .filter(|spec| spec.flags.contains(CommandFlags::PALETTE_VISIBLE))
        .map(|spec| spec.key)
        .collect::<Vec<_>>();

    for (command, stable_id, label) in expected {
        assert_eq!(command.key().as_str(), stable_id);
        assert_eq!(registry.label(command.key()), label);
        assert_eq!(registry.resolve(stable_id), Some(command.key()));
        assert!(searchable.contains(&command.key()), "{stable_id}");
    }
}

#[test]
fn sheet_properties_never_fall_back_to_symbol_hardcopy_media() {
    let mut app = app_with_selected_authored_symbol();
    app.state.open_project_session();
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

    assert!(!app.command_key_enabled(SheetsCommand::SheetProperties.key()));
    app.execute_command_key(SheetsCommand::SheetProperties.key());

    assert!(!app.state.sheets.reveal_sheet_section);
    assert!(
        !app.state.hardcopy.hardcopy.open,
        "Sheet properties must not substitute output-media setup"
    );
}

/// Sheet properties show the sheet where it is edited: the selection is
/// cleared, so the inspector is the sheet's; the inspector opens where it was
/// closed; and the sheet panel is asked to bring its Sheet section into view.
#[test]
fn sheet_properties_open_the_inspector_on_the_sheet() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    crate::workbench::examples::load_example("RC Lowpass Filter", &mut app.state.model.schematic);
    app.state.select_all_schematic_objects();
    app.state.workbench.inspector_visible = false;
    assert!(app.command_key_enabled(SheetsCommand::SheetProperties.key()));

    app.execute_command_key(SheetsCommand::SheetProperties.key());

    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .is_empty()
    );
    assert!(app.state.workbench.inspector_visible);
    assert!(app.state.sheets.reveal_sheet_section);
}

#[test]
fn document_sheet_commands_open_their_real_surfaces_only_in_schematic_context() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.workspace = Workspace::Design;
    let key = app.state.model.workspace.content.active_key();
    app.state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Main", [])
        .unwrap();

    assert!(app.command_key_enabled(SheetsCommand::SheetFormatManager.key()));
    assert!(app.command_key_enabled(SheetsCommand::CustomSheetSizes.key()));

    app.execute_command_key(SheetsCommand::SheetFormatManager.key());
    assert!(app.state.sheets.drawing_sheet_support.manager.open);

    app.execute_command_key(SheetsCommand::CustomSheetSizes.key());
    assert!(app.state.sheets.drawing_sheet_presets.any_open());

    // The project default is where a format reaches several sheets, so its
    // dialog is a command of its own.
    assert!(app.command_key_enabled(SheetsCommand::SheetDefaults.key()));
    app.execute_command_key(SheetsCommand::SheetDefaults.key());
    assert!(app.state.sheets.drawing_sheet_defaults.open);

    let mut results = RSpiceApp::test_instance();
    results.state.open_project_session();
    results.state.workbench.workspace = Workspace::Results;
    assert!(!results.command_key_enabled(SheetsCommand::SheetProperties.key()));
    assert!(!results.command_key_enabled(SheetsCommand::SheetFormatManager.key()));
    assert!(!results.command_key_enabled(SheetsCommand::CustomSheetSizes.key()));
    assert!(!results.command_key_enabled(SheetsCommand::SheetDefaults.key()));
}

#[test]
fn sheet_format_manager_requires_live_schematic_edit_authority() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.workspace = Workspace::Design;
    let key = app.state.model.workspace.content.active_key();
    app.state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Main", [])
        .unwrap();
    app.state.model.schematic.session.read_only = true;

    assert_eq!(
        app.command_availability(SheetsCommand::SheetFormatManager.key()),
        CommandAvailability::Disabled("the active schematic is read-only")
    );
    app.execute_command_key(SheetsCommand::SheetFormatManager.key());
    assert!(!app.state.sheets.drawing_sheet_support.manager.open);
}
