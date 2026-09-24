//! The sheet commands: their identities, their chords, and what makes them
//! available.
//!
//! Page Up and Page Down are the only unmodified chords this lane claims, so
//! the one-owner check is the point of the first case: the document strip's
//! browser alternates are the modified `Ctrl+Alt` pair, and nothing else may
//! bind either key on its own.

use super::*;
use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate};
use crate::workbench::commands::vocabulary::CommandPlatform;

const SHEET_COMMANDS: [Command; 4] = [
    Command::NextSheet,
    Command::PreviousSheet,
    Command::NewSheet,
    Command::DeleteSheet,
];

/// Every registered command that binds any chord `command` binds.
fn chord_owners(command: Command) -> Vec<Command> {
    let chords = command
        .shortcut_bindings()
        .iter()
        .map(|binding| binding.chord)
        .collect::<Vec<_>>();
    vocabulary::COMMAND_REGISTRY
        .iter()
        .copied()
        .filter(|candidate| {
            candidate
                .shortcut_bindings()
                .iter()
                .any(|binding| chords.contains(&binding.chord))
        })
        .collect()
}

fn app_with_sheets(count: usize) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.workbench.activate(Workspace::Design);
    let key = app.state.workspace.active_schematic_reference().key();
    let first = app
        .state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", [10])
        .expect("first sheet");
    let catalog = app
        .state
        .workspace
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
fn every_sheet_command_is_registered_with_its_stable_identity() {
    for (command, id, label) in [
        (Command::NextSheet, "next-sheet", "Next sheet"),
        (Command::PreviousSheet, "previous-sheet", "Previous sheet"),
        (Command::NewSheet, "new-sheet", "New sheet"),
        (Command::DeleteSheet, "delete-sheet", "Delete active sheet"),
    ] {
        assert_eq!(command.stable_id(), id);
        assert_eq!(command.spec().label, label);
        assert_eq!(command.spec().group, "Design");
        assert_eq!(Command::from_stable_id(id), Some(command));
        assert!(
            vocabulary::COMMAND_REGISTRY.contains(&command),
            "the palette projects the registry, so an unlisted command has no seat"
        );
        assert!(command.palette_visible(), "{command:?}");
        assert_eq!(
            command.shortcut_context(),
            ShortcutContext::EngineeringCanvas,
            "{command:?}"
        );
        assert!(command.requires_open_project(), "{command:?}");
    }
}

#[test]
fn paging_chords_have_exactly_one_owner_on_every_platform() {
    let next = Command::NextSheet.shortcut_bindings();
    let previous = Command::PreviousSheet.shortcut_bindings();
    assert_eq!(next.len(), 1);
    assert_eq!(previous.len(), 1);
    assert_eq!(next[0].chord.label, "Page Down");
    assert_eq!(previous[0].chord.label, "Page Up");

    assert_eq!(chord_owners(Command::NextSheet), vec![Command::NextSheet]);
    assert_eq!(
        chord_owners(Command::PreviousSheet),
        vec![Command::PreviousSheet]
    );
    for platform in CommandPlatform::ALL {
        assert_eq!(
            Command::NextSheet.default_shortcut_label(platform),
            "Page Down"
        );
        assert_eq!(
            Command::PreviousSheet.default_shortcut_label(platform),
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

    assert!(Command::NewSheet.shortcut_bindings().is_empty());
    assert!(Command::DeleteSheet.shortcut_bindings().is_empty());
}

#[test]
fn sheet_commands_state_why_they_are_unavailable() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.project_open = true;
    app.state.workbench.activate(Workspace::Results);
    assert_eq!(
        Command::NextSheet.availability(&app),
        CommandAvailability::Disabled("open a schematic or testbench")
    );
    assert_eq!(
        Command::NewSheet.availability(&app),
        CommandAvailability::Disabled("open an editable schematic or testbench")
    );

    app.state.workbench.activate(Workspace::Design);
    app.state.schematic.read_only = true;
    assert_eq!(
        Command::NewSheet.availability(&app),
        CommandAvailability::Disabled("the active schematic is read-only")
    );
    assert_eq!(
        Command::DeleteSheet.availability(&app),
        CommandAvailability::Disabled("the active schematic is read-only")
    );

    app.state.schematic.read_only = false;
    for command in SHEET_COMMANDS {
        assert_eq!(
            command.availability(&app),
            CommandAvailability::Disabled(
                "create the governed sheet catalog in Design management first"
            ),
            "{command:?}"
        );
    }

    let single = app_with_sheets(1);
    for command in [
        Command::NextSheet,
        Command::PreviousSheet,
        Command::DeleteSheet,
    ] {
        assert_eq!(
            command.availability(&single),
            CommandAvailability::Disabled("the active cell view has a single sheet"),
            "{command:?}"
        );
    }

    app.state.project_lifecycle.project_open = false;
    for command in SHEET_COMMANDS {
        assert_eq!(
            command.availability(&app),
            CommandAvailability::Disabled("no project is open"),
            "{command:?}"
        );
    }
}

#[test]
fn navigation_needs_a_second_sheet_and_authoring_needs_a_catalog() {
    let mut app = app_with_sheets(1);
    assert!(!Command::NextSheet.is_enabled(&app));
    assert!(!Command::PreviousSheet.is_enabled(&app));
    assert!(!Command::DeleteSheet.is_enabled(&app));
    assert!(Command::NewSheet.is_enabled(&app));

    app = app_with_sheets(2);
    for command in SHEET_COMMANDS {
        assert_eq!(
            command.availability(&app),
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

    Command::NextSheet.execute(&mut app);
    assert_eq!(sheets::active_sheet_id(&app.state), Some(order[1]));
    Command::PreviousSheet.execute(&mut app);
    Command::PreviousSheet.execute(&mut app);
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
    Command::NewSheet.execute(&mut app);
    assert_eq!(sheets::sheet_count(&app.state), 3);

    let created = sheets::active_sheet_id(&app.state).expect("the new sheet is entered");
    Command::DeleteSheet.execute(&mut app);
    assert_eq!(sheets::sheet_count(&app.state), 2);
    assert_ne!(sheets::active_sheet_id(&app.state), Some(created));
}
