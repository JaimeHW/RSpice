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
            &app.state.hardcopy.report_authoring,
            &app.state.model,
            &app.state.workbench,
            &*app.state.results,
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
