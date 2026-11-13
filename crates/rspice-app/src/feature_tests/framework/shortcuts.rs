//! The accessible shortcut summary reads the effective custom profile and the
//! host's platform, not the defaults.

use egui::Key;

use crate::state::command_catalog::CommandPlatform;
use crate::state::shortcuts::{
    ShortcutBindingSlot, ShortcutPreferences, ShortcutSequence, ShortcutStroke,
};
use crate::workbench::app_state::session::shortcuts::*;
use crate::workbench::commands::vocabulary::{CommandRegistry, command_registry};
use crate::workbench::design::commands::DesignCommand;

#[test]
fn accessibility_projection_uses_effective_custom_profile_and_host() {
    let mut shortcuts = ShortcutPreferences::default();
    shortcuts
        .set_binding(
            &CommandRegistry,
            DesignCommand::SymbolCircleTool.key(),
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Desktop],
            Some(ShortcutSequence::single(ShortcutStroke::new(
                Key::F9,
                false,
                true,
                false,
            ))),
        )
        .unwrap();

    let summary = accessibility_shortcut_summary(
        &shortcuts,
        &CommandRegistry,
        command_registry(),
        CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
        &[DesignCommand::SymbolCircleTool.key()],
    );

    assert_eq!(summary, " Shortcuts: Alt+F9: Draw symbol circle.");
}
