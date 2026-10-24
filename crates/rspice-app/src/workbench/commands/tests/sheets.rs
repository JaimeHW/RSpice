//! The sheet commands: their chords and what makes them available.
//!
//! Page Up and Page Down are the only unmodified chords this lane claims, so
//! the one-owner check is the point of the first case: the document strip's
//! browser alternates are the modified `Ctrl+Alt` pair, and nothing else may
//! bind either key on its own.

use super::*;
use crate::state::command_catalog::{
    CommandCatalog, CommandKey, CommandPlatform, ShortcutBinding, ShortcutContext,
};
use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate};
use crate::workbench::commands::vocabulary::command_registry;
use crate::workbench::framework::command_registry::CommandFlags;

const SHEET_COMMANDS: [SheetsCommand; 4] = [
    SheetsCommand::NextSheet,
    SheetsCommand::PreviousSheet,
    SheetsCommand::NewSheet,
    SheetsCommand::DeleteSheet,
];

/// The bindings `command` ships with, as the registry declares them.
fn shortcut_bindings(command: SheetsCommand) -> &'static [ShortcutBinding] {
    vocabulary::CommandRegistry.default_bindings(command.key())
}

/// The chord the registry shows for `command` on `platform`.
fn default_shortcut_label(command: SheetsCommand, platform: CommandPlatform) -> &'static str {
    shortcut_bindings(command)
        .iter()
        .find(|binding| binding.supports(platform))
        .map_or("", |binding| binding.chord.label)
}

/// Every registered command that binds any chord `command` binds.
fn chord_owners(command: SheetsCommand) -> Vec<CommandKey> {
    let chords = shortcut_bindings(command)
        .iter()
        .map(|binding| binding.chord)
        .collect::<Vec<_>>();
    command_registry()
        .ordered()
        .filter(|candidate| {
            candidate
                .bindings
                .iter()
                .any(|binding| chords.contains(&binding.chord))
        })
        .map(|candidate| candidate.key)
        .collect()
}

fn app_with_sheets(count: usize) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    let key = app
        .state
        .model
        .workspace
        .content
        .active_schematic_reference()
        .key();
    let first = app
        .state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", [10])
        .expect("first sheet");
    let catalog = app
        .state
        .model
        .workspace
        .content
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog");
    for ordinal in 2..=count {
        catalog
            .create_sheet(
                SheetDefinition {
                    name: format!("Sheet {ordinal}"),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(ordinal as u32),
                },
                None,
            )
            .expect("further sheet");
    }
    catalog.set_active(first).expect("active sheet");
    app
}

#[test]
fn every_sheet_command_is_a_registered_canvas_command() {
    for command in SHEET_COMMANDS {
        let spec = command_registry().spec(command.key());
        assert!(
            command_registry().order().contains(&command.key()),
            "the palette projects the registry, so an unlisted command has no seat"
        );
        let spec = spec.expect("an ordered command is declared");
        assert!(
            spec.flags.contains(CommandFlags::PALETTE_VISIBLE),
            "{command:?}"
        );
        assert_eq!(
            spec.context,
            ShortcutContext::EngineeringCanvas,
            "{command:?}"
        );
        assert!(
            spec.flags.contains(CommandFlags::REQUIRES_OPEN_PROJECT),
            "{command:?}"
        );
    }
}

#[test]
fn paging_chords_have_exactly_one_owner_on_every_platform() {
    let next = shortcut_bindings(SheetsCommand::NextSheet);
    let previous = shortcut_bindings(SheetsCommand::PreviousSheet);
    assert_eq!(next.len(), 1);
    assert_eq!(previous.len(), 1);
    assert_eq!(next[0].chord.label, "Page Down");
    assert_eq!(previous[0].chord.label, "Page Up");

    assert_eq!(
        chord_owners(SheetsCommand::NextSheet),
        vec![SheetsCommand::NextSheet.key()]
    );
    assert_eq!(
        chord_owners(SheetsCommand::PreviousSheet),
        vec![SheetsCommand::PreviousSheet.key()]
    );
    for platform in CommandPlatform::ALL {
        assert_eq!(
            default_shortcut_label(SheetsCommand::NextSheet, platform),
            "Page Down"
        );
        assert_eq!(
            default_shortcut_label(SheetsCommand::PreviousSheet, platform),
            "Page Up"
        );
        // The document strip reaches the same keys only with Ctrl+Alt, so a
        // bare Page Up/Down can never mean two things at once.
        assert_eq!(
            Command::NextDocument.default_shortcut_label(platform),
            if platform == CommandPlatform::Desktop {
                "Ctrl+Tab"
            } else {
                "Ctrl+Alt+Page Down"
            }
        );
    }

    assert!(shortcut_bindings(SheetsCommand::NewSheet).is_empty());
    assert!(shortcut_bindings(SheetsCommand::DeleteSheet).is_empty());
}

#[test]
fn sheet_commands_state_why_they_are_unavailable() {
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Results);
    assert_eq!(
        app.command_availability(SheetsCommand::NextSheet.key()),
        CommandAvailability::Disabled("open a schematic or testbench")
    );
    assert_eq!(
        app.command_availability(SheetsCommand::NewSheet.key()),
        CommandAvailability::Disabled("open an editable schematic or testbench")
    );

    app.state.workbench.activate(Workspace::Design);
    app.state.model.schematic.session.read_only = true;
    assert_eq!(
        app.command_availability(SheetsCommand::NewSheet.key()),
        CommandAvailability::Disabled("the active schematic is read-only")
    );
    assert_eq!(
        app.command_availability(SheetsCommand::DeleteSheet.key()),
        CommandAvailability::Disabled("the active schematic is read-only")
    );

    app.state.model.schematic.session.read_only = false;
    for command in [
        SheetsCommand::NextSheet,
        SheetsCommand::PreviousSheet,
        SheetsCommand::DeleteSheet,
    ] {
        assert_eq!(
            app.command_availability(command.key()),
            CommandAvailability::Disabled("the active cell view has a single sheet"),
            "a drawing with no sheet catalog is one sheet: {command:?}"
        );
    }
    assert_eq!(
        app.command_availability(SheetsCommand::NewSheet.key()),
        CommandAvailability::Available,
        "a new sheet on a single drawing creates the catalog"
    );

    let single = app_with_sheets(1);
    for command in [
        SheetsCommand::NextSheet,
        SheetsCommand::PreviousSheet,
        SheetsCommand::DeleteSheet,
    ] {
        assert_eq!(
            single.command_availability(command.key()),
            CommandAvailability::Disabled("the active cell view has a single sheet"),
            "{command:?}"
        );
    }

    app.state.close_project_session();
    for command in SHEET_COMMANDS {
        assert_eq!(
            app.command_availability(command.key()),
            CommandAvailability::Disabled("no project is open"),
            "{command:?}"
        );
    }
}

#[test]
fn navigation_needs_a_second_sheet_and_authoring_needs_a_catalog() {
    let mut app = app_with_sheets(1);
    assert!(!app.command_key_enabled(SheetsCommand::NextSheet.key()));
    assert!(!app.command_key_enabled(SheetsCommand::PreviousSheet.key()));
    assert!(!app.command_key_enabled(SheetsCommand::DeleteSheet.key()));
    assert!(app.command_key_enabled(SheetsCommand::NewSheet.key()));

    app = app_with_sheets(2);
    for command in SHEET_COMMANDS {
        assert_eq!(
            app.command_availability(command.key()),
            CommandAvailability::Available,
            "{command:?}"
        );
    }
}

#[test]
fn paging_commands_move_the_active_sheet_and_wrap() {
    use crate::workbench::app::sheets;

    let mut app = app_with_sheets(3);
    let order = sheets::sheet_entries(&app.state)
        .into_iter()
        .map(|entry| entry.id)
        .collect::<Vec<_>>();
    assert_eq!(sheets::active_sheet_id(&app.state), Some(order[0]));

    app.execute_command_key(SheetsCommand::NextSheet.key());
    assert_eq!(sheets::active_sheet_id(&app.state), Some(order[1]));
    app.execute_command_key(SheetsCommand::PreviousSheet.key());
    app.execute_command_key(SheetsCommand::PreviousSheet.key());
    assert_eq!(
        sheets::active_sheet_id(&app.state),
        Some(order[2]),
        "paging wraps at the ends"
    );
}

#[test]
fn creating_and_deleting_sheets_run_their_real_transactions() {
    use crate::workbench::app::sheets;

    let mut app = app_with_sheets(2);
    app.execute_command_key(SheetsCommand::NewSheet.key());
    assert_eq!(sheets::sheet_count(&app.state), 3);

    let created = sheets::active_sheet_id(&app.state).expect("the new sheet is entered");
    app.execute_command_key(SheetsCommand::DeleteSheet.key());
    assert_eq!(sheets::sheet_count(&app.state), 2);
    assert_ne!(sheets::active_sheet_id(&app.state), Some(created));
}
