use crate::workbench::commands::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShortcutCategory {
    File,
    Edit,
    View,
    Design,
    Simulation,
    Navigation,
    General,
}

impl ShortcutCategory {
    pub(crate) const ALL: [Self; 7] = [
        Self::File,
        Self::Edit,
        Self::View,
        Self::Design,
        Self::Simulation,
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
            Self::Navigation => "Workspaces",
            Self::General => "General",
        }
    }

    pub(crate) const fn commands(self) -> &'static [Command] {
        match self {
            Self::File => &[
                Command::ProjectLauncher,
                Command::NewProject,
                Command::OpenProject,
                Command::Save,
                Command::SaveAs,
                Command::SaveAll,
                Command::CloseActiveDocument,
                Command::CloseProject,
                Command::Exit,
            ],
            Self::Edit => &[
                Command::Undo,
                Command::Redo,
                Command::Cut,
                Command::Copy,
                Command::Paste,
                Command::Duplicate,
                Command::Delete,
                Command::SelectAll,
                Command::ObjectProperties,
                Command::FindInDesign,
            ],
            Self::View => &[
                Command::ZoomIn,
                Command::ZoomOut,
                Command::ZoomFit,
                Command::CycleGrid,
                Command::ToggleFullScreen,
                Command::ToggleNavigator,
                Command::ToggleInspector,
                Command::ToggleConsole,
                Command::ToggleFocusMode,
            ],
            Self::Design => &[
                Command::PlaceInstance,
                Command::PlaceWire,
                Command::PlaceLabel,
                Command::PlaceProbe,
                Command::AscendHierarchy,
                Command::DescendHierarchy,
                Command::RunChecks,
                Command::CheckAndSave,
            ],
            Self::Simulation => &[
                Command::RunSimulation,
                Command::StopSimulation,
                Command::PreflightChecks,
                Command::GenerateNetlist,
            ],
            Self::Navigation => &[
                Command::OpenWorkspace(crate::workbench::state::Workspace::Project),
                Command::OpenWorkspace(crate::workbench::state::Workspace::Design),
                Command::OpenWorkspace(crate::workbench::state::Workspace::Simulate),
                Command::ResultViewer(crate::workbench::ResultViewer::Waves),
                Command::VerificationPage(crate::workbench::state::VerificationPage::Yield),
                Command::OpenWorkspace(crate::workbench::state::Workspace::Models),
                Command::OpenWorkspace(crate::workbench::state::Workspace::Netlist),
            ],
            Self::General => &[
                Command::CommandPalette,
                Command::Preferences,
                Command::AutomationConsole,
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::commands::{COMMAND_REGISTRY, CommandPlatform};

    #[test]
    fn help_is_a_lossless_projection_of_registered_shortcuts() {
        let mut seen = Vec::new();
        for category in ShortcutCategory::ALL {
            for command in category.commands() {
                assert!(COMMAND_REGISTRY.contains(command));
                assert!(
                    !command.shortcut_bindings().is_empty(),
                    "help row has no typed binding: {command:?}"
                );
                assert!(!seen.contains(command), "duplicate help row: {command:?}");
                seen.push(*command);
            }
        }

        for command in COMMAND_REGISTRY {
            if !command.shortcut_bindings().is_empty() && *command != Command::Cancel {
                assert!(
                    seen.contains(command),
                    "registered shortcut missing from help: {command:?}"
                );
            }
        }

        assert_eq!(
            Command::OpenProject.shortcut_label(CommandPlatform::Browser),
            "Ctrl+Alt+O"
        );
    }
}
