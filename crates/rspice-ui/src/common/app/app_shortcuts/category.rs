use super::ShortcutCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ShortcutCategory {
    File,
    Edit,
    View,
    Tools,
    ComponentPlacement,
    Transform,
    Simulation,
    Navigation,
    General,
}

impl ShortcutCategory {
    pub(crate) const ALL: [ShortcutCategory; 9] = [
        ShortcutCategory::File,
        ShortcutCategory::Edit,
        ShortcutCategory::View,
        ShortcutCategory::Tools,
        ShortcutCategory::ComponentPlacement,
        ShortcutCategory::Transform,
        ShortcutCategory::Simulation,
        ShortcutCategory::Navigation,
        ShortcutCategory::General,
    ];

    pub(crate) fn display_name(self) -> &'static str {
        match self {
            ShortcutCategory::File => "File",
            ShortcutCategory::Edit => "Edit",
            ShortcutCategory::View => "View",
            ShortcutCategory::Tools => "Tools",
            ShortcutCategory::ComponentPlacement => "Component Placement",
            ShortcutCategory::Transform => "Transform",
            ShortcutCategory::Simulation => "Simulation",
            ShortcutCategory::Navigation => "Navigation",
            ShortcutCategory::General => "General",
        }
    }

    pub(crate) fn commands(self) -> &'static [ShortcutCommand] {
        match self {
            ShortcutCategory::File => &[
                ShortcutCommand::FileNew,
                ShortcutCommand::FileOpen,
                ShortcutCommand::FileSave,
            ],
            ShortcutCategory::Edit => &[
                ShortcutCommand::EditUndo,
                ShortcutCommand::EditRedo,
                ShortcutCommand::EditCopy,
                ShortcutCommand::EditPaste,
                ShortcutCommand::EditCut,
                ShortcutCommand::EditDelete,
                ShortcutCommand::EditSelectAll,
            ],
            ShortcutCategory::View => &[
                ShortcutCommand::ToggleBrowserPanel,
                ShortcutCommand::ToggleLogPanel,
            ],
            ShortcutCategory::Tools => &[
                ShortcutCommand::ToolSelect,
                ShortcutCommand::ToolWire,
                ShortcutCommand::ToolLabel,
                ShortcutCommand::ToolProbe,
            ],
            ShortcutCategory::ComponentPlacement => &[
                ShortcutCommand::PlaceResistor,
                ShortcutCommand::PlaceGround,
                ShortcutCommand::PlaceVoltageSource,
                ShortcutCommand::PlaceCurrentSource,
                ShortcutCommand::PlaceCapacitor,
                ShortcutCommand::PlaceInductor,
                ShortcutCommand::PlaceDiode,
                ShortcutCommand::PlaceNmos,
                ShortcutCommand::PlaceNpnBjt,
            ],
            ShortcutCategory::Transform => &[
                ShortcutCommand::RotateSelectionOrPreview,
                ShortcutCommand::MirrorSelectionHorizontal,
                ShortcutCommand::MirrorSelectionVertical,
            ],
            ShortcutCategory::Simulation => &[
                ShortcutCommand::RunSimulation,
                ShortcutCommand::StopSimulation,
                ShortcutCommand::RunChecks,
            ],
            ShortcutCategory::Navigation => &[
                ShortcutCommand::ZoomIn,
                ShortcutCommand::ZoomOut,
                ShortcutCommand::ZoomFit,
                ShortcutCommand::Zoom100,
                ShortcutCommand::NextWorkspaceTab,
                ShortcutCommand::DescendIntoSelected,
                ShortcutCommand::AscendHierarchy,
                ShortcutCommand::FocusDesignSearch,
                ShortcutCommand::FocusCellSearch,
            ],
            ShortcutCategory::General => &[
                ShortcutCommand::OpenPropertiesEditor,
                ShortcutCommand::EscapeCancel,
                ShortcutCommand::ShowShortcutsHelp,
                ShortcutCommand::OpenPreferences,
                ShortcutCommand::OpenCommandPalette,
            ],
        }
    }
}
