//! The shortcut reference lists every discoverable command exactly once.

use crate::state::command_catalog::CommandPlatform;
use crate::workbench::app_state::session::shortcuts::category::*;
use crate::workbench::commands::vocabulary::{Command, command_registry};
use crate::workbench::framework::command_registry::CommandFlags;

#[test]
fn command_reference_is_a_lossless_projection_of_discoverable_commands() {
    let mut seen = Vec::new();
    for category in ShortcutCategory::ALL {
        for command in category.commands(command_registry()) {
            assert!(command_registry().order().contains(&command));
            assert!(!seen.contains(&command), "duplicate help row: {command:?}");
            seen.push(command);
        }
    }

    for spec in command_registry().ordered() {
        if spec.flags.contains(CommandFlags::PALETTE_VISIBLE) {
            assert!(
                seen.contains(&spec.key),
                "discoverable command missing from reference: {:?}",
                spec.key
            );
        }
    }

    assert_eq!(
        Command::OpenProject.default_shortcut_label(CommandPlatform::Browser),
        "Ctrl+Alt+O"
    );
}
