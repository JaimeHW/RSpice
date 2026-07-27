use crate::workbench::commands::{COMMAND_REGISTRY, Command};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShortcutCategory {
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
    pub(crate) const ALL: [Self; 9] = [
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

    pub(crate) const fn display_name(self) -> &'static str {
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
    pub(crate) fn commands(self) -> impl Iterator<Item = Command> {
        COMMAND_REGISTRY.iter().copied().filter(move |command| {
            command.palette_visible() && self.matches_group(command.spec().group)
        })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::commands::CommandPlatform;

    #[test]
    fn command_reference_is_a_lossless_projection_of_discoverable_commands() {
        let mut seen = Vec::new();
        for category in ShortcutCategory::ALL {
            for command in category.commands() {
                assert!(COMMAND_REGISTRY.contains(&command));
                assert!(!seen.contains(&command), "duplicate help row: {command:?}");
                seen.push(command);
            }
        }

        for command in COMMAND_REGISTRY {
            if command.palette_visible() {
                assert!(
                    seen.contains(command),
                    "discoverable command missing from reference: {command:?}"
                );
            }
        }

        assert_eq!(
            Command::OpenProject.default_shortcut_label(CommandPlatform::Browser),
            "Ctrl+Alt+O"
        );
    }
}
