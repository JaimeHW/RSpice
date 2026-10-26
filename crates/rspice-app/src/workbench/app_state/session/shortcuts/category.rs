//! Shortcut categories, for grouping in the preferences editor and the
//! keyboard reference.

use crate::state::command_catalog::CommandKey;
use crate::workbench::framework::command_registry::{CommandFlags, Registry};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutCategory {
    File,
    Edit,
    View,
    Design,
    Simulation,
    Results,
    Verification,
    Navigation,
    General,
}

impl ShortcutCategory {
    pub const ALL: [Self; 9] = [
        Self::File,
        Self::Edit,
        Self::View,
        Self::Design,
        Self::Simulation,
        Self::Results,
        Self::Verification,
        Self::Navigation,
        Self::General,
    ];

    pub const fn display_name(self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::View => "View and window",
            Self::Design => "Design",
            Self::Simulation => "Simulation",
            Self::Results => "Results",
            Self::Verification => "Verification",
            Self::Navigation => "Workspaces and navigation",
            Self::General => "General",
        }
    }

    /// Lossless help projection from the canonical command metadata. Commands
    /// are categorized from `CommandSpec::group`; this deliberately avoids a
    /// second hand-maintained command inventory drifting from the registry.
    pub fn commands(self, registry: &Registry) -> impl Iterator<Item = CommandKey> {
        registry
            .ordered()
            .filter(move |spec| {
                spec.flags.contains(CommandFlags::PALETTE_VISIBLE) && self.matches_group(spec.group)
            })
            .map(|spec| spec.key)
    }

    const fn matches_group(self, group: &str) -> bool {
        match self {
            Self::File => matches!(group.as_bytes(), b"File"),
            Self::Edit => matches!(group.as_bytes(), b"Edit"),
            Self::View => matches!(group.as_bytes(), b"View" | b"Window"),
            Self::Design => matches!(group.as_bytes(), b"Design"),
            Self::Simulation => matches!(group.as_bytes(), b"Simulate"),
            Self::Results => matches!(group.as_bytes(), b"Results"),
            Self::Verification => matches!(group.as_bytes(), b"Verify"),
            Self::Navigation => matches!(group.as_bytes(), b"Navigate"),
            Self::General => !matches!(
                group.as_bytes(),
                b"File"
                    | b"Edit"
                    | b"View"
                    | b"Window"
                    | b"Design"
                    | b"Simulate"
                    | b"Results"
                    | b"Verify"
                    | b"Navigate"
            ),
        }
    }
}
