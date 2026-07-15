//! Stable command metadata and the keyboard contract shared by every shell.
//!
//! A command ID is a product-facing identifier: menus, search, accessibility,
//! shortcuts, telemetry, and portable preferences all refer to the same
//! [`Command`]. Bindings are typed and platform-scoped so a browser-reserved
//! chord can never silently shadow its desktop command.

use egui::Key;

use super::Command;
use crate::common::RSpiceApp;
use crate::workbench::state::Workspace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandPlatform {
    Desktop,
    Browser,
    Tablet,
    Phone,
}

impl CommandPlatform {
    pub const ALL: [Self; 4] = [Self::Desktop, Self::Browser, Self::Tablet, Self::Phone];

    const fn has_browser_reserved_primary(self) -> bool {
        matches!(self, Self::Browser)
    }
}

pub const fn current_command_platform() -> CommandPlatform {
    #[cfg(target_arch = "wasm32")]
    {
        CommandPlatform::Browser
    }
    #[cfg(target_os = "android")]
    {
        CommandPlatform::Phone
    }
    #[cfg(target_os = "ios")]
    {
        CommandPlatform::Phone
    }
    #[cfg(not(any(target_arch = "wasm32", target_os = "android", target_os = "ios")))]
    {
        CommandPlatform::Desktop
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortcutContext {
    Global,
    ApplicationChrome,
    EditContext,
    EngineeringCanvas,
    DesignWorkspace,
    SimulationWorkspace,
    RunnableProject,
}

impl ShortcutContext {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::ApplicationChrome => "application-chrome",
            Self::EditContext => "edit-context",
            Self::EngineeringCanvas => "engineering-canvas",
            Self::DesignWorkspace => "design-workspace",
            Self::SimulationWorkspace => "simulation-workspace",
            Self::RunnableProject => "runnable-project",
        }
    }

    pub const fn suppressed_by_text_focus(self) -> bool {
        !matches!(self, Self::Global | Self::RunnableProject)
    }

    pub fn matches(self, app: &RSpiceApp) -> bool {
        match self {
            Self::Global | Self::ApplicationChrome | Self::RunnableProject => true,
            Self::EditContext | Self::EngineeringCanvas => {
                app.state.workbench.workspace == Workspace::Design
                    && matches!(
                        app.state.workspace.active_view_type(),
                        crate::state::ViewType::Schematic
                            | crate::state::ViewType::Testbench
                            | crate::state::ViewType::Symbol
                    )
            }
            Self::DesignWorkspace => app.state.workbench.workspace == Workspace::Design,
            Self::SimulationWorkspace => app.state.workbench.workspace == Workspace::Simulate,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortcutKind {
    Primary,
    Alternate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShortcutChord {
    pub key: Key,
    pub primary: bool,
    pub alt: bool,
    pub shift: bool,
    pub label: &'static str,
}

impl ShortcutChord {
    const fn new(key: Key, primary: bool, alt: bool, shift: bool, label: &'static str) -> Self {
        Self {
            key,
            primary,
            alt,
            shift,
            label,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShortcutBinding {
    pub chord: ShortcutChord,
    pub platforms: &'static [CommandPlatform],
    pub kind: ShortcutKind,
}

impl ShortcutBinding {
    pub const fn supports(self, platform: CommandPlatform) -> bool {
        let mut index = 0;
        while index < self.platforms.len() {
            if self.platforms[index] as u8 == platform as u8 {
                return true;
            }
            index += 1;
        }
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandAvailability {
    Available,
    Disabled(&'static str),
    Hidden,
}

impl CommandAvailability {
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }
}

const ALL: &[CommandPlatform] = &CommandPlatform::ALL;
const DESKTOP: &[CommandPlatform] = &[CommandPlatform::Desktop];
const HOST: &[CommandPlatform] = &[
    CommandPlatform::Browser,
    CommandPlatform::Tablet,
    CommandPlatform::Phone,
];

const fn primary(chord: ShortcutChord, platforms: &'static [CommandPlatform]) -> ShortcutBinding {
    ShortcutBinding {
        chord,
        platforms,
        kind: ShortcutKind::Primary,
    }
}

const fn alternate(chord: ShortcutChord, platforms: &'static [CommandPlatform]) -> ShortcutBinding {
    ShortcutBinding {
        chord,
        platforms,
        kind: ShortcutKind::Alternate,
    }
}

const fn chord(
    key: Key,
    primary_modifier: bool,
    alt: bool,
    shift: bool,
    label: &'static str,
) -> ShortcutChord {
    ShortcutChord::new(key, primary_modifier, alt, shift, label)
}

const NONE: &[ShortcutBinding] = &[];
const PROJECT_LAUNCHER: &[ShortcutBinding] = &[primary(
    chord(Key::O, true, false, true, "Ctrl+Shift+O"),
    ALL,
)];
const OPEN_PROJECT: &[ShortcutBinding] = &[
    primary(chord(Key::O, true, false, false, "Ctrl+O"), DESKTOP),
    alternate(chord(Key::O, true, true, false, "Ctrl+Alt+O"), HOST),
];
const NEW_PROJECT: &[ShortcutBinding] = &[
    primary(chord(Key::N, true, false, true, "Ctrl+Shift+N"), DESKTOP),
    alternate(chord(Key::N, true, true, true, "Ctrl+Alt+Shift+N"), HOST),
];
const SAVE_PROJECT: &[ShortcutBinding] = &[
    primary(chord(Key::S, true, false, false, "Ctrl+S"), DESKTOP),
    alternate(chord(Key::S, true, true, false, "Ctrl+Alt+S"), HOST),
];
const SAVE_PROJECT_AS: &[ShortcutBinding] = &[primary(
    chord(Key::S, true, true, true, "Ctrl+Shift+Alt+S"),
    ALL,
)];
const SAVE_ALL: &[ShortcutBinding] = &[primary(
    chord(Key::S, true, false, true, "Ctrl+Shift+S"),
    ALL,
)];
const CLOSE_DOCUMENT: &[ShortcutBinding] = &[
    primary(chord(Key::W, true, false, false, "Ctrl+W"), DESKTOP),
    alternate(
        chord(Key::Backspace, true, false, true, "Ctrl+Shift+Backspace"),
        HOST,
    ),
];
const CLOSE_PROJECT: &[ShortcutBinding] = &[primary(
    chord(Key::W, true, false, true, "Ctrl+Shift+W"),
    DESKTOP,
)];
const COMMAND_PALETTE: &[ShortcutBinding] =
    &[primary(chord(Key::K, true, false, false, "Ctrl+K"), ALL)];
const PREFERENCES: &[ShortcutBinding] = &[primary(
    chord(Key::Comma, true, false, false, "Ctrl+,"),
    ALL,
)];
const UNDO: &[ShortcutBinding] = &[primary(chord(Key::Z, true, false, false, "Ctrl+Z"), ALL)];
const REDO: &[ShortcutBinding] = &[primary(
    chord(Key::Z, true, false, true, "Ctrl+Shift+Z"),
    ALL,
)];
const CUT: &[ShortcutBinding] = &[primary(chord(Key::X, true, false, false, "Ctrl+X"), ALL)];
const COPY: &[ShortcutBinding] = &[primary(chord(Key::C, true, false, false, "Ctrl+C"), ALL)];
const PASTE: &[ShortcutBinding] = &[primary(chord(Key::V, true, false, false, "Ctrl+V"), ALL)];
const DUPLICATE: &[ShortcutBinding] = &[primary(chord(Key::D, true, false, false, "Ctrl+D"), ALL)];
const DELETE: &[ShortcutBinding] = &[primary(
    chord(Key::Delete, false, false, false, "Delete"),
    ALL,
)];
const SELECT_ALL: &[ShortcutBinding] = &[primary(chord(Key::A, true, false, false, "Ctrl+A"), ALL)];
const OBJECT_PROPERTIES: &[ShortcutBinding] =
    &[primary(chord(Key::Q, false, false, false, "Q"), ALL)];
const FIND_DESIGN: &[ShortcutBinding] =
    &[primary(chord(Key::F, true, false, false, "Ctrl+F"), ALL)];
const ZOOM_IN: &[ShortcutBinding] = &[
    primary(chord(Key::Plus, false, false, false, "+"), ALL),
    alternate(chord(Key::Plus, false, false, true, "+"), ALL),
];
const ZOOM_OUT: &[ShortcutBinding] = &[primary(chord(Key::Minus, false, false, false, "−"), ALL)];
const ZOOM_FIT: &[ShortcutBinding] = &[primary(chord(Key::F, false, false, false, "F"), ALL)];
const GRID: &[ShortcutBinding] = &[primary(chord(Key::G, false, false, false, "G"), ALL)];
const FULL_SCREEN: &[ShortcutBinding] = &[
    primary(chord(Key::F11, false, false, false, "F11"), DESKTOP),
    alternate(chord(Key::F, true, true, false, "Ctrl+Alt+F"), HOST),
];
const TOGGLE_NAVIGATOR: &[ShortcutBinding] =
    &[primary(chord(Key::B, true, false, false, "Ctrl+B"), ALL)];
const TOGGLE_INSPECTOR: &[ShortcutBinding] =
    &[primary(chord(Key::I, true, false, false, "Ctrl+I"), ALL)];
const TOGGLE_CONSOLE: &[ShortcutBinding] = &[
    primary(chord(Key::J, true, false, false, "Ctrl+J"), DESKTOP),
    alternate(chord(Key::J, true, true, false, "Ctrl+Alt+J"), HOST),
];
const FOCUS_WORKSPACE: &[ShortcutBinding] = &[primary(
    chord(Key::F, true, false, true, "Ctrl+Shift+F"),
    ALL,
)];
const PROJECT_WORKSPACE: &[ShortcutBinding] =
    &[primary(chord(Key::Num1, false, true, false, "Alt+1"), ALL)];
const DESIGN_WORKSPACE: &[ShortcutBinding] =
    &[primary(chord(Key::Num2, false, true, false, "Alt+2"), ALL)];
const SIMULATION_WORKSPACE: &[ShortcutBinding] =
    &[primary(chord(Key::Num3, false, true, false, "Alt+3"), ALL)];
const RESULTS_WORKSPACE: &[ShortcutBinding] =
    &[primary(chord(Key::Num4, false, true, false, "Alt+4"), ALL)];
const VERIFICATION_WORKSPACE: &[ShortcutBinding] =
    &[primary(chord(Key::Num5, false, true, false, "Alt+5"), ALL)];
const MODELS_WORKSPACE: &[ShortcutBinding] =
    &[primary(chord(Key::Num6, false, true, false, "Alt+6"), ALL)];
const AUTOMATION_WORKSPACE: &[ShortcutBinding] =
    &[primary(chord(Key::Num7, false, true, false, "Alt+7"), ALL)];
const PLACE_INSTANCE: &[ShortcutBinding] =
    &[primary(chord(Key::I, false, false, true, "Shift+I"), ALL)];
const PLACE_WIRE: &[ShortcutBinding] = &[primary(chord(Key::W, false, false, false, "W"), ALL)];
const PLACE_LABEL: &[ShortcutBinding] = &[primary(chord(Key::N, false, false, false, "N"), ALL)];
const PLACE_PROBE: &[ShortcutBinding] = &[primary(chord(Key::P, false, false, false, "P"), ALL)];
const ASCEND_HIERARCHY: &[ShortcutBinding] =
    &[primary(chord(Key::H, false, false, true, "Shift+H"), ALL)];
const DESCEND_HIERARCHY: &[ShortcutBinding] =
    &[primary(chord(Key::H, false, false, false, "H"), ALL)];
const RUN_CHECKS: &[ShortcutBinding] = &[primary(chord(Key::E, true, false, false, "Ctrl+E"), ALL)];
const CHECK_AND_SAVE: &[ShortcutBinding] = &[primary(
    chord(Key::E, true, false, true, "Ctrl+Shift+E"),
    ALL,
)];
const RUN_SIMULATION: &[ShortcutBinding] = &[
    primary(chord(Key::F5, false, false, false, "F5"), DESKTOP),
    alternate(chord(Key::Enter, true, false, false, "Ctrl+Enter"), HOST),
];
const STOP_SIMULATION: &[ShortcutBinding] = &[
    primary(chord(Key::F5, false, false, true, "Shift+F5"), DESKTOP),
    alternate(
        chord(Key::Enter, true, false, true, "Ctrl+Shift+Enter"),
        HOST,
    ),
];
const PREFLIGHT: &[ShortcutBinding] = &[primary(chord(Key::E, true, false, false, "Ctrl+E"), ALL)];
const GENERATED_NETLIST: &[ShortcutBinding] = &[
    primary(chord(Key::L, true, false, false, "Ctrl+L"), DESKTOP),
    alternate(chord(Key::L, true, true, false, "Ctrl+Alt+L"), HOST),
];
const AUTOMATION_CONSOLE: &[ShortcutBinding] = &[primary(
    chord(Key::Backtick, true, false, false, "Ctrl+`"),
    ALL,
)];
const CANCEL: &[ShortcutBinding] = &[primary(
    chord(Key::Escape, false, false, false, "Escape"),
    ALL,
)];

impl Command {
    pub(crate) const fn blocked_by_project_operation(self) -> bool {
        matches!(
            self,
            Self::ProjectLauncher
                | Self::RecentProjects
                | Self::NewProject
                | Self::OpenProject
                | Self::Save
                | Self::SaveAs
                | Self::SaveAll
                | Self::RevertActiveDocument
                | Self::CloseActiveDocument
                | Self::CloseProject
                | Self::NewCell
                | Self::OpenDocument
                | Self::ImportNetlist
                | Self::ImportVerilogA
                | Self::CheckAndSave
        )
    }

    pub const fn palette_visible(self) -> bool {
        !matches!(
            self,
            Self::CommandPalette
                | Self::Cancel
                | Self::ResetActiveView
                | Self::ResetLayout
                | Self::PreviousWorkspace
                | Self::NextWorkspace
        )
    }

    pub const fn shortcut_context(self) -> ShortcutContext {
        match self {
            Self::Undo
            | Self::Redo
            | Self::Cut
            | Self::Copy
            | Self::Paste
            | Self::Duplicate
            | Self::Delete
            | Self::SelectAll => ShortcutContext::EditContext,
            Self::ObjectProperties
            | Self::FindInDesign
            | Self::ZoomIn
            | Self::ZoomOut
            | Self::ZoomFit
            | Self::ZoomOneToOne
            | Self::CycleGrid
            | Self::SelectTool
            | Self::PlaceInstance
            | Self::PlaceWire
            | Self::PlaceLabel
            | Self::PlaceProbe
            | Self::Place(_)
            | Self::RotateSelection
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical => ShortcutContext::EngineeringCanvas,
            Self::Cancel => ShortcutContext::ApplicationChrome,
            Self::AscendHierarchy
            | Self::DescendHierarchy
            | Self::RunChecks
            | Self::CheckAndSave => ShortcutContext::DesignWorkspace,
            Self::PreflightChecks => ShortcutContext::SimulationWorkspace,
            Self::RunSimulation | Self::StopSimulation => ShortcutContext::RunnableProject,
            _ => ShortcutContext::Global,
        }
    }

    pub const fn shortcut_bindings(self) -> &'static [ShortcutBinding] {
        match self {
            Self::OpenWorkspace(Workspace::Project) => PROJECT_WORKSPACE,
            Self::OpenWorkspace(Workspace::Design) => DESIGN_WORKSPACE,
            Self::OpenWorkspace(Workspace::Simulate) => SIMULATION_WORKSPACE,
            Self::OpenWorkspace(Workspace::Results) => RESULTS_WORKSPACE,
            Self::OpenWorkspace(Workspace::Verify) => VERIFICATION_WORKSPACE,
            Self::OpenWorkspace(Workspace::Models) => MODELS_WORKSPACE,
            Self::OpenWorkspace(Workspace::Netlist) => AUTOMATION_WORKSPACE,
            Self::ProjectLauncher => PROJECT_LAUNCHER,
            Self::NewProject => NEW_PROJECT,
            Self::OpenProject => OPEN_PROJECT,
            Self::Save => SAVE_PROJECT,
            Self::SaveAs => SAVE_PROJECT_AS,
            Self::SaveAll => SAVE_ALL,
            Self::CloseActiveDocument => CLOSE_DOCUMENT,
            Self::CloseProject => CLOSE_PROJECT,
            Self::Undo => UNDO,
            Self::Redo => REDO,
            Self::Cut => CUT,
            Self::Copy => COPY,
            Self::Paste => PASTE,
            Self::Duplicate => DUPLICATE,
            Self::Delete => DELETE,
            Self::SelectAll => SELECT_ALL,
            Self::ObjectProperties => OBJECT_PROPERTIES,
            Self::FindInDesign => FIND_DESIGN,
            Self::Preferences => PREFERENCES,
            Self::ZoomIn => ZOOM_IN,
            Self::ZoomOut => ZOOM_OUT,
            Self::ZoomFit => ZOOM_FIT,
            Self::CycleGrid => GRID,
            Self::ToggleFullScreen => FULL_SCREEN,
            Self::ToggleNavigator => TOGGLE_NAVIGATOR,
            Self::ToggleInspector => TOGGLE_INSPECTOR,
            Self::ToggleConsole => TOGGLE_CONSOLE,
            Self::ToggleFocusMode => FOCUS_WORKSPACE,
            Self::PlaceInstance => PLACE_INSTANCE,
            Self::PlaceWire => PLACE_WIRE,
            Self::PlaceLabel => PLACE_LABEL,
            Self::PlaceProbe => PLACE_PROBE,
            Self::AscendHierarchy => ASCEND_HIERARCHY,
            Self::DescendHierarchy => DESCEND_HIERARCHY,
            Self::RunChecks => RUN_CHECKS,
            Self::CheckAndSave => CHECK_AND_SAVE,
            Self::RunSimulation => RUN_SIMULATION,
            Self::StopSimulation => STOP_SIMULATION,
            Self::PreflightChecks => PREFLIGHT,
            Self::GenerateNetlist => GENERATED_NETLIST,
            Self::AutomationConsole => AUTOMATION_CONSOLE,
            Self::CommandPalette => COMMAND_PALETTE,
            Self::Cancel => CANCEL,
            _ => NONE,
        }
    }

    pub fn shortcut_label(self, platform: CommandPlatform) -> &'static str {
        self.shortcut_bindings()
            .iter()
            .find(|binding| binding.supports(platform))
            .map_or("", |binding| binding.chord.label)
    }

    pub fn shortcut_context_matches(self, app: &RSpiceApp) -> bool {
        self.shortcut_context().matches(app)
    }

    pub fn availability(self, app: &RSpiceApp) -> CommandAvailability {
        if matches!(
            self,
            Self::ResetActiveView
                | Self::ResetLayout
                | Self::PreviousWorkspace
                | Self::NextWorkspace
        ) {
            return CommandAvailability::Hidden;
        }
        if crate::common::project_lifecycle::operation_in_progress(&app.state)
            && self.blocked_by_project_operation()
        {
            return CommandAvailability::Disabled("project operation is still in progress");
        }
        if self.is_enabled(app) {
            return CommandAvailability::Available;
        }
        let reason = match self {
            Self::Save | Self::SaveAs | Self::SaveAll | Self::CloseProject => "no project is open",
            Self::RevertActiveDocument => "active document has no changes to revert",
            Self::CloseActiveDocument => "no closable document is active",
            Self::Undo => "nothing to undo",
            Self::Redo => "nothing to redo",
            Self::Cut
            | Self::Copy
            | Self::Duplicate
            | Self::Delete
            | Self::RotateSelection
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical => "select an editable object",
            Self::Paste => "clipboard has no compatible content",
            Self::ObjectProperties => "select one editable object",
            Self::AscendHierarchy => "already at top hierarchy",
            Self::DescendHierarchy => "select one hierarchical instance",
            Self::RunSimulation => "active plan is not runnable",
            Self::StopSimulation
                if app.state.simulation.is_running
                    && !crate::simulation::execution::execution_target_supports_cancellation() =>
            {
                "cancellation is unavailable for the current execution target"
            }
            Self::StopSimulation => "no simulation is running",
            Self::ClearResults | Self::ExportWaveformsCsv => "no result dataset is available",
            _ => "command is unavailable in this context",
        };
        CommandAvailability::Disabled(reason)
    }

    pub fn primary_is_reserved_on(self, platform: CommandPlatform) -> bool {
        platform.has_browser_reserved_primary()
            && matches!(
                self,
                Self::RunSimulation
                    | Self::StopSimulation
                    | Self::OpenProject
                    | Self::NewProject
                    | Self::Save
                    | Self::CloseActiveDocument
                    | Self::ToggleFullScreen
                    | Self::GenerateNetlist
                    | Self::ToggleConsole
            )
    }
}
