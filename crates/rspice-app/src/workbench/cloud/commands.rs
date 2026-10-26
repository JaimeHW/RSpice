//! The commands the cloud feature declares, as the registry lists them: the
//! live session and web publication, the account console and the license.
//!
//! What each command does is the feature's own; what it declares here is all
//! the shell's order, menus, palette and shortcut profile read of it.

use crate::state::command_catalog::{CommandKey, ShortcutContext};
use crate::workbench::framework::command_registry::{CommandFlags, CommandOwner, CommandSpec, ids};

/// One command of the cloud feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CloudCommand {
    LiveSession,
    PublishToWeb,
    AccountOrganization,
    License,
}

impl CloudCommand {
    /// Every command of the feature.
    pub(crate) const ALL: [Self; 4] = [
        Self::LiveSession,
        Self::PublishToWeb,
        Self::AccountOrganization,
        Self::License,
    ];

    /// The key a shortcut profile names this command by.
    pub(crate) const fn key(self) -> CommandKey {
        match self {
            Self::LiveSession => CommandKey::new("live-session"),
            Self::PublishToWeb => CommandKey::new("publish-web"),
            Self::AccountOrganization => ids::ACCOUNT_ORGANIZATION,
            Self::License => CommandKey::new("license-activation"),
        }
    }

    /// What the command declares about itself. Every one is in the palette
    /// and has no binding of its own. Publishing needs an open project.
    /// Hosting a live session does too, but the same dialog joins by code
    /// from any state, so that command never requires one.
    pub(crate) const fn declared_spec(self) -> CommandSpec {
        let (label, group, flags) = match self {
            Self::LiveSession => ("Live session…", "File", CommandFlags::PALETTE_VISIBLE),
            Self::PublishToWeb => (
                "Publish to web…",
                "File",
                CommandFlags::PALETTE_VISIBLE.with(CommandFlags::REQUIRES_OPEN_PROJECT, true),
            ),
            Self::AccountOrganization => (
                "Account and administration…",
                "Account",
                CommandFlags::PALETTE_VISIBLE,
            ),
            Self::License => (
                "License and activation…",
                "Help",
                CommandFlags::PALETTE_VISIBLE,
            ),
        };
        CommandSpec {
            key: self.key(),
            label,
            group,
            flags,
            context: ShortcutContext::Global,
            bindings: &[],
            owner: CommandOwner::Cloud,
        }
    }
}
