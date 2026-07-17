//! Stable command metadata and the keyboard contract shared by every shell.
//!
//! A command ID is a product-facing identifier: menus, search, accessibility,
//! shortcuts, telemetry, and portable preferences all refer to the same
//! [`Command`]. Bindings are typed and platform-scoped so a browser-reserved
//! chord can never silently shadow its desktop command.

use egui::Key;
use serde::{Deserialize, Serialize};

use super::Command;
use crate::common::RSpiceApp;
use crate::workbench::state::Workspace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CommandPlatform {
    Desktop,
    Browser,
    Tablet,
    Phone,
}

impl CommandPlatform {
    pub const ALL: [Self; 4] = [Self::Desktop, Self::Browser, Self::Tablet, Self::Phone];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Desktop => "Desktop",
            Self::Browser => "Browser",
            Self::Tablet => "Tablet",
            Self::Phone => "Phone",
        }
    }

    const fn has_browser_reserved_primary(self) -> bool {
        matches!(self, Self::Browser)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortcutContext {
    Global,
    ApplicationChrome,
    EditContext,
    EngineeringCanvas,
    SymbolCanvas,
    DesignWorkspace,
    SimulationWorkspace,
    ResultsWorkspace,
    VerificationWorkspace,
    ViolationNavigation,
    RunnableProject,
}

impl ShortcutContext {
    pub const ALL: [Self; 11] = [
        Self::Global,
        Self::ApplicationChrome,
        Self::EditContext,
        Self::EngineeringCanvas,
        Self::SymbolCanvas,
        Self::DesignWorkspace,
        Self::SimulationWorkspace,
        Self::ResultsWorkspace,
        Self::VerificationWorkspace,
        Self::ViolationNavigation,
        Self::RunnableProject,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::ApplicationChrome => "application-chrome",
            Self::EditContext => "edit-context",
            Self::EngineeringCanvas => "engineering-canvas",
            Self::SymbolCanvas => "symbol-canvas",
            Self::DesignWorkspace => "design-workspace",
            Self::SimulationWorkspace => "simulation-workspace",
            Self::ResultsWorkspace => "results-workspace",
            Self::VerificationWorkspace => "verification-workspace",
            Self::ViolationNavigation => "violation-navigation",
            Self::RunnableProject => "runnable-project",
        }
    }

    pub const fn suppressed_by_text_focus(self) -> bool {
        !matches!(self, Self::Global | Self::RunnableProject)
    }

    pub fn matches(self, app: &RSpiceApp) -> bool {
        match self {
            Self::Global | Self::ApplicationChrome | Self::RunnableProject => true,
            Self::EditContext => match app.state.workspace.active_view_type() {
                crate::state::ViewType::Schematic | crate::state::ViewType::Testbench => {
                    app.state.workbench.workspace == Workspace::Design
                }
                crate::state::ViewType::Symbol => matches!(
                    app.state.workbench.workspace,
                    Workspace::Design | Workspace::Models
                ),
                _ => false,
            },
            Self::EngineeringCanvas => {
                app.state.workbench.workspace == Workspace::Design
                    && matches!(
                        app.state.workspace.active_view_type(),
                        crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
                    )
            }
            Self::SymbolCanvas => {
                matches!(
                    app.state.workbench.workspace,
                    Workspace::Design | Workspace::Models
                ) && app.state.workspace.active_view_type() == crate::state::ViewType::Symbol
            }
            Self::DesignWorkspace => app.state.workbench.workspace == Workspace::Design,
            Self::SimulationWorkspace => app.state.workbench.workspace == Workspace::Simulate,
            Self::ResultsWorkspace => app.state.workbench.workspace == Workspace::Results,
            Self::VerificationWorkspace => app.state.workbench.workspace == Workspace::Verify,
            Self::ViolationNavigation => matches!(
                app.state.workbench.workspace,
                Workspace::Design | Workspace::Verify
            ),
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
const EXIT: &[ShortcutBinding] = &[primary(
    chord(Key::F4, false, true, false, "Alt+F4"),
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
const SELECT_TOOL: &[ShortcutBinding] = &[primary(chord(Key::S, false, false, false, "S"), ALL)];
const PLACE_WIRE: &[ShortcutBinding] = &[primary(chord(Key::W, false, false, false, "W"), ALL)];
const PLACE_LABEL: &[ShortcutBinding] = &[primary(chord(Key::N, false, false, false, "N"), ALL)];
const PLACE_PROBE: &[ShortcutBinding] = &[primary(chord(Key::P, false, false, false, "P"), ALL)];
const SYMBOL_PIN: &[ShortcutBinding] = &[primary(chord(Key::P, false, false, false, "P"), ALL)];
const SYMBOL_POLYLINE: &[ShortcutBinding] =
    &[primary(chord(Key::W, false, false, false, "W"), ALL)];
const SYMBOL_CIRCLE: &[ShortcutBinding] = &[primary(chord(Key::C, false, false, false, "C"), ALL)];
const SYMBOL_ARC: &[ShortcutBinding] = &[primary(chord(Key::A, false, false, false, "A"), ALL)];
const SYMBOL_ARROW: &[ShortcutBinding] = &[primary(chord(Key::D, false, false, false, "D"), ALL)];
const SYMBOL_DOT: &[ShortcutBinding] = &[primary(chord(Key::O, false, false, false, "O"), ALL)];
const PLACE_RESISTOR: &[ShortcutBinding] = &[primary(chord(Key::R, false, false, false, "R"), ALL)];
const PLACE_CAPACITOR: &[ShortcutBinding] =
    &[primary(chord(Key::C, false, false, false, "C"), ALL)];
const PLACE_INDUCTOR: &[ShortcutBinding] = &[primary(chord(Key::L, false, false, false, "L"), ALL)];
const PLACE_DIODE: &[ShortcutBinding] = &[primary(chord(Key::D, false, false, false, "D"), ALL)];
const PLACE_GROUND: &[ShortcutBinding] =
    &[primary(chord(Key::G, false, false, true, "Shift+G"), ALL)];
const PLACE_VOLTAGE_SOURCE: &[ShortcutBinding] =
    &[primary(chord(Key::V, false, false, false, "V"), ALL)];
const PLACE_CURRENT_SOURCE: &[ShortcutBinding] =
    &[primary(chord(Key::I, false, false, false, "I"), ALL)];
const ASCEND_HIERARCHY: &[ShortcutBinding] =
    &[primary(chord(Key::H, false, false, true, "Shift+H"), ALL)];
const DESCEND_HIERARCHY: &[ShortcutBinding] =
    &[primary(chord(Key::H, false, false, false, "H"), ALL)];
const RUN_CHECKS: &[ShortcutBinding] = &[primary(chord(Key::E, true, false, false, "Ctrl+E"), ALL)];
const CHECK_AND_SAVE: &[ShortcutBinding] = &[primary(
    chord(Key::E, true, false, true, "Ctrl+Shift+E"),
    ALL,
)];
const NEXT_VIOLATION: &[ShortcutBinding] = &[
    primary(chord(Key::CloseBracket, false, false, false, "]"), ALL),
    alternate(chord(Key::F8, false, false, false, "F8"), ALL),
];
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
const LINKED_CURSORS: &[ShortcutBinding] =
    &[primary(chord(Key::C, false, false, true, "Shift+C"), ALL)];
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
                | Self::PreviousWorkspace
                | Self::NextWorkspace
                | Self::ToggleConsoleMaximized
                | Self::ClearConsole
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
            | Self::SelectAll
            | Self::ObjectProperties
            | Self::ZoomIn
            | Self::ZoomOut
            | Self::ZoomFit
            | Self::ZoomOneToOne
            | Self::CycleGrid
            | Self::SelectTool => ShortcutContext::EditContext,
            Self::FindInDesign
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
            Self::NextViolation | Self::PreviousViolation => ShortcutContext::ViolationNavigation,
            Self::ClearResults | Self::WaveformCalculator | Self::ResultViewer(_) => {
                ShortcutContext::ResultsWorkspace
            }
            Self::SymbolPinTool
            | Self::SymbolPolylineTool
            | Self::SymbolCircleTool
            | Self::SymbolArcTool
            | Self::SymbolArrowTool
            | Self::SymbolDotTool => ShortcutContext::SymbolCanvas,
            Self::ToggleLinkedCursors => ShortcutContext::ResultsWorkspace,
            Self::EditSpecifications | Self::VerificationPage(_) => {
                ShortcutContext::VerificationWorkspace
            }
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
            Self::Exit => EXIT,
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
            Self::SelectTool => SELECT_TOOL,
            Self::PlaceInstance => PLACE_INSTANCE,
            Self::PlaceWire => PLACE_WIRE,
            Self::PlaceLabel => PLACE_LABEL,
            Self::PlaceProbe => PLACE_PROBE,
            Self::SymbolPinTool => SYMBOL_PIN,
            Self::SymbolPolylineTool => SYMBOL_POLYLINE,
            Self::SymbolCircleTool => SYMBOL_CIRCLE,
            Self::SymbolArcTool => SYMBOL_ARC,
            Self::SymbolArrowTool => SYMBOL_ARROW,
            Self::SymbolDotTool => SYMBOL_DOT,
            Self::Place(crate::state::ComponentType::Resistor) => PLACE_RESISTOR,
            Self::Place(crate::state::ComponentType::Capacitor) => PLACE_CAPACITOR,
            Self::Place(crate::state::ComponentType::Inductor) => PLACE_INDUCTOR,
            Self::Place(crate::state::ComponentType::Diode) => PLACE_DIODE,
            Self::Place(crate::state::ComponentType::Ground) => PLACE_GROUND,
            Self::Place(crate::state::ComponentType::VoltageSource) => PLACE_VOLTAGE_SOURCE,
            Self::Place(crate::state::ComponentType::CurrentSource) => PLACE_CURRENT_SOURCE,
            Self::AscendHierarchy => ASCEND_HIERARCHY,
            Self::DescendHierarchy => DESCEND_HIERARCHY,
            Self::RunChecks => RUN_CHECKS,
            Self::CheckAndSave => CHECK_AND_SAVE,
            Self::NextViolation => NEXT_VIOLATION,
            Self::RunSimulation => RUN_SIMULATION,
            Self::StopSimulation => STOP_SIMULATION,
            Self::PreflightChecks => PREFLIGHT,
            Self::ToggleLinkedCursors => LINKED_CURSORS,
            Self::GenerateNetlist => GENERATED_NETLIST,
            Self::AutomationConsole => AUTOMATION_CONSOLE,
            Self::CommandPalette => COMMAND_PALETTE,
            Self::Cancel => CANCEL,
            _ => NONE,
        }
    }

    /// Immutable factory binding label for registry contract tests and
    /// recovery diagnostics. Product UI must use `ShortcutPreferences`.
    pub fn default_shortcut_label(self, platform: CommandPlatform) -> &'static str {
        self.shortcut_bindings()
            .iter()
            .find(|binding| binding.supports(platform))
            .map_or("", |binding| binding.chord.label)
    }

    pub fn shortcut_context_matches(self, app: &RSpiceApp) -> bool {
        self.shortcut_context().matches(app)
    }

    pub fn availability(self, app: &RSpiceApp) -> CommandAvailability {
        if matches!(self, Self::PreviousWorkspace | Self::NextWorkspace) {
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
            Self::ClearResults
                if app.state.simulation.active_execution.is_some()
                    || app.state.simulation.is_running =>
            {
                "an active simulation execution still owns result history"
            }
            Self::ClearResults | Self::ExportWaveformsCsv => "no result dataset is available",
            Self::VerificationPage(crate::workbench::state::VerificationPage::Tuning) => {
                "design-parameter discovery and transactional tuning are not implemented"
            }
            Self::VerificationPage(crate::workbench::state::VerificationPage::Drc) => {
                "no retained layout, qualified rule deck, or immutable marker database is available"
            }
            Self::ResetActiveView => "active workspace has no resettable view state",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::ResultViewer;
    use crate::workbench::state::VerificationPage;

    fn binding_owners(chord: ShortcutChord, platform: CommandPlatform) -> Vec<Command> {
        super::super::COMMAND_REGISTRY
            .iter()
            .copied()
            .filter(|command| {
                command
                    .shortcut_bindings()
                    .iter()
                    .any(|binding| binding.chord == chord && binding.supports(platform))
            })
            .collect()
    }

    #[test]
    fn workspace_surface_shortcuts_have_one_canonical_owner() {
        for platform in CommandPlatform::ALL {
            assert_eq!(
                binding_owners(RESULTS_WORKSPACE[0].chord, platform),
                vec![Command::OpenWorkspace(Workspace::Results)]
            );
            assert_eq!(
                Command::OpenWorkspace(Workspace::Results).default_shortcut_label(platform),
                "Alt+4"
            );
            assert_eq!(
                binding_owners(VERIFICATION_WORKSPACE[0].chord, platform),
                vec![Command::OpenWorkspace(Workspace::Verify)]
            );
            assert_eq!(
                Command::OpenWorkspace(Workspace::Verify).default_shortcut_label(platform),
                "Alt+5"
            );
        }

        assert!(
            Command::ResultViewer(ResultViewer::Waves)
                .shortcut_bindings()
                .is_empty()
        );
        assert!(
            Command::VerificationPage(VerificationPage::Yield)
                .shortcut_bindings()
                .is_empty()
        );
    }

    #[test]
    fn exit_shortcut_is_unambiguous_and_desktop_only() {
        assert_eq!(
            binding_owners(EXIT[0].chord, CommandPlatform::Desktop),
            vec![Command::Exit]
        );
        assert_eq!(
            Command::Exit.default_shortcut_label(CommandPlatform::Desktop),
            "Alt+F4"
        );

        for platform in [
            CommandPlatform::Browser,
            CommandPlatform::Tablet,
            CommandPlatform::Phone,
        ] {
            assert!(binding_owners(EXIT[0].chord, platform).is_empty());
            assert_eq!(Command::Exit.default_shortcut_label(platform), "");
        }
    }

    #[test]
    fn symbol_tool_defaults_are_complete_portable_and_view_scoped() {
        for (command, label) in [
            (Command::SymbolPinTool, "P"),
            (Command::SymbolPolylineTool, "W"),
            (Command::SymbolCircleTool, "C"),
            (Command::SymbolArcTool, "A"),
            (Command::SymbolArrowTool, "D"),
            (Command::SymbolDotTool, "O"),
        ] {
            assert_eq!(command.shortcut_context(), ShortcutContext::SymbolCanvas);
            for platform in CommandPlatform::ALL {
                assert_eq!(command.default_shortcut_label(platform), label);
            }
        }

        assert_eq!(
            Command::PlaceProbe.shortcut_context(),
            ShortcutContext::EngineeringCanvas
        );
        assert_eq!(
            Command::PlaceWire.shortcut_context(),
            ShortcutContext::EngineeringCanvas
        );
    }
}
