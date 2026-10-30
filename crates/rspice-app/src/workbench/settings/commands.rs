//! The commands the settings feature declares, as the registry lists them:
//! Preferences and the workspace layout manager.
//!
//! What each command does is the feature's own; what it declares here is all
//! the shell's order, menus, palette and shortcut profile read of it.

use egui::Key;

use crate::state::command_catalog::{
    CommandKey, CommandPlatform, ShortcutBinding, ShortcutChord, ShortcutContext, ShortcutKind,
};
use crate::workbench::framework::command_registry::{CommandFlags, CommandOwner, CommandSpec, ids};

/// One command of the settings feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsCommand {
    Preferences,
    WorkspaceLayouts,
}

const NONE: &[ShortcutBinding] = &[];

const PREFERENCES: &[ShortcutBinding] = &[ShortcutBinding {
    chord: ShortcutChord::new(Key::Comma, true, false, false, "Ctrl+,"),
    platforms: &CommandPlatform::ALL,
    kind: ShortcutKind::Primary,
}];

impl SettingsCommand {
    /// Every command of the feature.
    pub const ALL: [Self; 2] = [Self::Preferences, Self::WorkspaceLayouts];

    /// The key a shortcut profile names this command by.
    pub const fn key(self) -> CommandKey {
        match self {
            Self::Preferences => ids::PREFERENCES,
            Self::WorkspaceLayouts => ids::WORKSPACE_LAYOUTS,
        }
    }

    /// What the command declares about itself. Both are listed in the
    /// palette and run anywhere, with or without a project.
    pub const fn declared_spec(self) -> CommandSpec {
        let (label, group, bindings) = match self {
            Self::Preferences => ("Preferences…", "Edit", PREFERENCES),
            Self::WorkspaceLayouts => ("Workspace layouts\u{2026}", "Window", NONE),
        };
        CommandSpec {
            key: self.key(),
            label,
            group,
            flags: CommandFlags::PALETTE_VISIBLE,
            context: ShortcutContext::Global,
            bindings,
            owner: CommandOwner::Settings,
        }
    }
}
