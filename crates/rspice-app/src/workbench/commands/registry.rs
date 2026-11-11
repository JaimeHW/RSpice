//! Stable command metadata and the keyboard contract shared by every shell.
//!
//! A command ID is a product-facing identifier: menus, search, accessibility,
//! shortcuts, telemetry, and portable preferences all refer to the same
//! [`Command`]. Bindings are typed and platform-scoped so a browser-reserved
//! chord can never silently shadow its desktop command.

use egui::Key;

use super::vocabulary::Command;
use crate::state::command_catalog::{
    CommandPlatform, ShortcutBinding, ShortcutChord, ShortcutContext, ShortcutKind,
};
use crate::workbench::RSpiceApp;
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::state::Workspace;

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
const CLOSE_DOCUMENT: &[ShortcutBinding] = &[
    primary(chord(Key::W, true, false, false, "Ctrl+W"), DESKTOP),
    alternate(
        chord(Key::Backspace, true, false, true, "Ctrl+Shift+Backspace"),
        HOST,
    ),
];
const EXIT: &[ShortcutBinding] = &[primary(
    chord(Key::F4, false, true, false, "Alt+F4"),
    DESKTOP,
)];
const COMMAND_PALETTE: &[ShortcutBinding] =
    &[primary(chord(Key::K, true, false, false, "Ctrl+K"), ALL)];
const UNDO: &[ShortcutBinding] = &[primary(chord(Key::Z, true, false, false, "Ctrl+Z"), ALL)];
const REDO: &[ShortcutBinding] = &[
    primary(chord(Key::Z, true, false, true, "Ctrl+Shift+Z"), ALL),
    alternate(chord(Key::Y, true, false, false, "Ctrl+Y"), ALL),
];
const CUT: &[ShortcutBinding] = &[primary(chord(Key::X, true, false, false, "Ctrl+X"), ALL)];
const COPY: &[ShortcutBinding] = &[primary(chord(Key::C, true, false, false, "Ctrl+C"), ALL)];
const PASTE: &[ShortcutBinding] = &[primary(chord(Key::V, true, false, false, "Ctrl+V"), ALL)];
const DUPLICATE: &[ShortcutBinding] = &[primary(chord(Key::D, true, false, false, "Ctrl+D"), ALL)];
const DELETE: &[ShortcutBinding] = &[primary(
    chord(Key::Delete, false, false, false, "Delete"),
    ALL,
)];
const SELECT_ALL: &[ShortcutBinding] = &[primary(chord(Key::A, true, false, false, "Ctrl+A"), ALL)];
const DESKTOP_BROWSER: &[CommandPlatform] = &[CommandPlatform::Desktop, CommandPlatform::Browser];
const RENAME_SELECTION: &[ShortcutBinding] = &[primary(
    chord(Key::F2, false, false, false, "F2"),
    DESKTOP_BROWSER,
)];
const OBJECT_PROPERTIES: &[ShortcutBinding] =
    &[primary(chord(Key::Q, false, false, false, "Q"), ALL)];
const FIND_DESIGN: &[ShortcutBinding] =
    &[primary(chord(Key::F, true, false, false, "Ctrl+F"), ALL)];
// A browser keeps Ctrl+H for its history, so there the menu reaches it.
const FIND_REPLACE: &[ShortcutBinding] = &[primary(
    chord(Key::H, true, false, false, "Ctrl+H"),
    DESKTOP,
)];
// Virtuoso and ADS both repeat the last command on a bare key; `.` was
// bound nowhere.
const REPEAT_LAST_COMMAND: &[ShortcutBinding] =
    &[primary(chord(Key::Period, false, false, false, "."), ALL)];
const ZOOM_IN: &[ShortcutBinding] = &[
    primary(chord(Key::Plus, false, false, false, "+"), ALL),
    alternate(chord(Key::Plus, false, false, true, "+"), ALL),
];
const ZOOM_OUT: &[ShortcutBinding] = &[primary(chord(Key::Minus, false, false, false, "−"), ALL)];
const ZOOM_FIT: &[ShortcutBinding] = &[primary(chord(Key::F, false, false, false, "F"), ALL)];
const FIT_SCHEMATIC_CONTENT: &[ShortcutBinding] =
    &[primary(chord(Key::F, false, false, true, "Shift+F"), ALL)];
const GRID: &[ShortcutBinding] = &[primary(chord(Key::G, false, false, false, "G"), ALL)];
// The mockup gives the results managers M, A and Q. Every one of those bare
// letters already has a single canonical owner in this product — move,
// symbol arc, object properties — and that one-owner rule is pinned by test
// so a reader never has to know which context they are in to predict a key.
// These commands therefore ship with no default chord; the shortcut editor
// can assign one, and the Results menu is the discoverable way in.
const FULL_SCREEN: &[ShortcutBinding] = &[
    primary(chord(Key::F11, false, false, false, "F11"), DESKTOP),
    alternate(chord(Key::F, true, true, false, "Ctrl+Alt+F"), HOST),
];
// The browser zooms its page on Ctrl+0, so the key is the desktop's only.
const ZOOM_ONE_TO_ONE: &[ShortcutBinding] = &[primary(
    chord(Key::Num0, true, false, false, "Ctrl+0"),
    DESKTOP,
)];
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
const PREVIOUS_DOCUMENT: &[ShortcutBinding] = &[
    primary(
        chord(Key::Tab, true, false, true, "Ctrl+Shift+Tab"),
        DESKTOP,
    ),
    alternate(
        chord(Key::PageUp, true, true, false, "Ctrl+Alt+Page Up"),
        HOST,
    ),
];
const NEXT_DOCUMENT: &[ShortcutBinding] = &[
    primary(chord(Key::Tab, true, false, false, "Ctrl+Tab"), DESKTOP),
    alternate(
        chord(Key::PageDown, true, true, false, "Ctrl+Alt+Page Down"),
        HOST,
    ),
];
const NEXT_PANE: &[ShortcutBinding] = &[primary(chord(Key::F6, false, false, false, "F6"), ALL)];
const PREVIOUS_PANE: &[ShortcutBinding] =
    &[primary(chord(Key::F6, false, false, true, "Shift+F6"), ALL)];
const NEW_APPLICATION_WINDOW: &[ShortcutBinding] = &[
    primary(chord(Key::N, true, true, true, "Ctrl+Shift+Alt+N"), DESKTOP),
    alternate(chord(Key::W, true, true, true, "Ctrl+Shift+Alt+W"), HOST),
];
const HELP_CENTER: &[ShortcutBinding] = &[primary(chord(Key::F1, false, false, false, "F1"), ALL)];
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
/// The library reads fourth on the rail and eighth on the keyboard. Alt+8 is
/// the mockup's chord for it, and taking Alt+4 for rail order would have moved
/// the four chords under it — a shortcut every reader has already learned.
const STIMULUS_WORKSPACE: &[ShortcutBinding] =
    &[primary(chord(Key::Num8, false, true, false, "Alt+8"), ALL)];
const PLACE_INSTANCE: &[ShortcutBinding] =
    &[primary(chord(Key::I, false, false, true, "Shift+I"), ALL)];
const PLACE_WIRE: &[ShortcutBinding] = &[primary(chord(Key::W, false, false, false, "W"), ALL)];
const PLACE_BUS: &[ShortcutBinding] = &[primary(chord(Key::B, false, false, false, "B"), ALL)];
const PLACE_BUS_TAP: &[ShortcutBinding] = &[primary(chord(Key::T, false, false, false, "T"), ALL)];
const PLACE_JUNCTION: &[ShortcutBinding] = &[primary(chord(Key::J, false, false, false, "J"), ALL)];
const PLACE_LABEL: &[ShortcutBinding] = &[primary(chord(Key::N, false, false, false, "N"), ALL)];
const PLACE_PROBE: &[ShortcutBinding] = &[primary(chord(Key::P, false, false, false, "P"), ALL)];
const PLACE_PIN: &[ShortcutBinding] = &[primary(chord(Key::P, false, false, true, "Shift+P"), ALL)];
const PLACE_TEXT: &[ShortcutBinding] =
    &[primary(chord(Key::T, false, false, true, "Shift+T"), ALL)];
const MOVE_SELECTION: &[ShortcutBinding] = &[primary(chord(Key::M, false, false, false, "M"), ALL)];
// The symbol canvas gives Ctrl+R and Ctrl+M the same meaning for its pins;
// the two contexts never match the same view.
const ROTATE_SELECTION: &[ShortcutBinding] =
    &[primary(chord(Key::R, true, false, false, "Ctrl+R"), ALL)];
const ROTATE_SELECTION_CCW: &[ShortcutBinding] = &[primary(
    chord(Key::R, true, false, true, "Ctrl+Shift+R"),
    ALL,
)];
const MIRROR_SELECTION_HORIZONTAL: &[ShortcutBinding] =
    &[primary(chord(Key::M, true, false, false, "Ctrl+M"), ALL)];
const MIRROR_SELECTION_VERTICAL: &[ShortcutBinding] = &[primary(
    chord(Key::M, true, false, true, "Ctrl+Shift+M"),
    ALL,
)];
/// Virtuoso's `z`.
const ZOOM_BOX: &[ShortcutBinding] = &[primary(chord(Key::Z, false, false, false, "Z"), ALL)];
/// W alone draws a wire; Shift+W is taken by no other command and by no
/// browser.
const ZOOM_PREVIOUS: &[ShortcutBinding] =
    &[primary(chord(Key::W, false, false, true, "Shift+W"), ALL)];
const STRETCH_SELECTION: &[ShortcutBinding] =
    &[primary(chord(Key::S, false, false, false, "S"), ALL)];
const SYMBOL_PIN: &[ShortcutBinding] = &[primary(chord(Key::P, false, false, false, "P"), ALL)];
const SYMBOL_POLYLINE: &[ShortcutBinding] =
    &[primary(chord(Key::L, false, false, false, "L"), ALL)];
const SYMBOL_RECTANGLE: &[ShortcutBinding] =
    &[primary(chord(Key::R, false, false, false, "R"), ALL)];
const SYMBOL_CIRCLE: &[ShortcutBinding] = &[primary(chord(Key::C, false, false, false, "C"), ALL)];
const SYMBOL_ARC: &[ShortcutBinding] = &[primary(chord(Key::A, false, false, false, "A"), ALL)];
const SYMBOL_POLYGON: &[ShortcutBinding] = &[primary(chord(Key::G, false, false, false, "G"), ALL)];
const SYMBOL_TEXT: &[ShortcutBinding] = &[primary(chord(Key::T, false, false, false, "T"), ALL)];
const SYMBOL_ROTATE_PIN: &[ShortcutBinding] =
    &[primary(chord(Key::R, true, false, false, "Ctrl+R"), ALL)];
const SYMBOL_MIRROR_PIN: &[ShortcutBinding] =
    &[primary(chord(Key::M, true, false, false, "Ctrl+M"), ALL)];
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
const DESCEND_HIERARCHY_DIRECT: &[ShortcutBinding] =
    &[primary(chord(Key::E, false, false, true, "Shift+E"), ALL)];
const CHECK_CELLVIEW: &[ShortcutBinding] =
    &[primary(chord(Key::E, true, false, false, "Ctrl+E"), ALL)];
const NEXT_VIOLATION: &[ShortcutBinding] = &[
    primary(chord(Key::CloseBracket, false, false, false, "]"), ALL),
    alternate(chord(Key::F8, false, false, false, "F8"), ALL),
];
const PREVIOUS_VIOLATION: &[ShortcutBinding] = &[
    primary(chord(Key::OpenBracket, false, false, false, "["), ALL),
    alternate(chord(Key::F8, false, false, true, "Shift+F8"), ALL),
];
const CANCEL: &[ShortcutBinding] = &[primary(
    chord(Key::Escape, false, false, false, "Escape"),
    ALL,
)];
// Return leaves an in-place symbol edit, as Esc does once nothing is in flight.
const EXIT_SYMBOL_IN_PLACE: &[ShortcutBinding] = &[primary(
    chord(Key::Enter, false, false, false, "Enter"),
    ALL,
)];

impl Command {
    /// Commands whose owning surface or dialog has no valid lifecycle without
    /// an open project. Keeping this boundary on the command itself prevents
    /// submenu and command-palette routes from bypassing the workspace gate.
    pub(crate) const fn requires_open_project(self) -> bool {
        matches!(
            self,
            Self::RevertActiveDocument
                | Self::CloseActiveDocument
                | Self::CloseOtherDocuments
                | Self::CloseAllDocuments
                | Self::SetStandardValueSeries(_)
                | Self::FindInDesign
                | Self::CheckDesign
                | Self::FindReplace
                | Self::ConfigurationSets
                | Self::CreateHierarchy
                | Self::DescendHierarchyDirect
        )
    }

    pub(crate) const fn blocked_by_project_operation(self) -> bool {
        matches!(
            self,
            Self::RevertActiveDocument
                | Self::CloseActiveDocument
                | Self::CloseOtherDocuments
                | Self::CloseAllDocuments
                | Self::FindReplace
                | Self::ConfigurationSets
                | Self::CreateHierarchy
        )
    }

    pub(crate) const fn mutates_window_session(self) -> bool {
        matches!(
            self,
            Self::NewApplicationWindow
                | Self::DetachDocument
                | Self::MoveDocumentToWindow
                | Self::ReattachDocument
                | Self::ConsolidateWindows
                | Self::MonitorRecovery
        )
    }

    pub const fn palette_visible(self) -> bool {
        !matches!(
            self,
            Self::CommandPalette
                | Self::Cancel
                | Self::ToggleConsoleMaximized
                | Self::ClearConsole
                | Self::DescendHierarchyDirect
                // The macOS Window menu's: a browser page has no window to
                // minimize or zoom, and Windows and Linux do it from the
                // title bar.
                | Self::MinimizeWindow
                | Self::ZoomWindow
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
            | Self::DeselectAll
            | Self::SelectOfKind(_)
            | Self::RenameSelection
            | Self::ObjectProperties
            | Self::ZoomIn
            | Self::ZoomOut
            | Self::ZoomFit
            | Self::FitSchematicContent
            | Self::ZoomOneToOne
            | Self::ZoomPrevious
            | Self::SelectTool => ShortcutContext::EditContext,
            Self::FindInDesign
            | Self::FindReplace
            | Self::CycleGrid
            | Self::SetGridStyle(_)
            | Self::PlaceInstance
            | Self::PlaceWire
            | Self::PlaceBus
            | Self::PlaceBusTap
            | Self::PlaceJunction
            | Self::PlaceLabel
            | Self::PlaceOffSheetConnector
            | Self::PlaceProbe
            | Self::PlacePin
            | Self::PlaceText
            | Self::PlaceShape
            | Self::MoveSelection
            | Self::StretchSelection
            | Self::ArraySelection
            | Self::ReplaceInstance
            | Self::SwapToRSpiceSymbols
            | Self::UpdateInstanceInterface
            | Self::CreateHierarchy
            | Self::Place(_)
            | Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical
            | Self::SnapSelectionToGrid
            | Self::SnapToStandardValue
            | Self::RepeatLastCommand
            | Self::ZoomToSelection
            | Self::ZoomBox => ShortcutContext::EngineeringCanvas,
            Self::Cancel => ShortcutContext::ApplicationChrome,
            Self::AscendHierarchy
            | Self::DescendHierarchy
            | Self::DescendHierarchyDirect
            | Self::EditSymbolInPlace
            | Self::SetStandardValueSeries(_)
            | Self::CheckCellview
            | Self::CheckDesign
            | Self::HighlightNet
            | Self::ConfigurationSets => ShortcutContext::DesignWorkspace,
            Self::NextViolation | Self::PreviousViolation => ShortcutContext::ViolationNavigation,
            Self::SymbolPinTool
            | Self::SymbolPolylineTool
            | Self::SymbolRectangleTool
            | Self::SymbolCircleTool
            | Self::SymbolArcTool
            | Self::SymbolPolygonTool
            | Self::SymbolTextTool
            | Self::SymbolRotatePin
            | Self::SymbolMirrorPin
            | Self::SymbolUpdatePinsFromContract
            | Self::SymbolSave
            | Self::ExitSymbolInPlace => ShortcutContext::SymbolCanvas,
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
            Self::OpenWorkspace(Workspace::Stimulus) => STIMULUS_WORKSPACE,
            Self::OpenWorkspace(Workspace::Netlist) => AUTOMATION_WORKSPACE,
            Self::CloseActiveDocument => CLOSE_DOCUMENT,
            Self::Exit => EXIT,
            Self::Undo => UNDO,
            Self::Redo => REDO,
            Self::Cut => CUT,
            Self::Copy => COPY,
            Self::Paste => PASTE,
            Self::Duplicate => DUPLICATE,
            Self::Delete => DELETE,
            Self::SelectAll => SELECT_ALL,
            Self::RenameSelection => RENAME_SELECTION,
            Self::ObjectProperties => OBJECT_PROPERTIES,
            Self::FindInDesign => FIND_DESIGN,
            Self::FindReplace => FIND_REPLACE,
            Self::RepeatLastCommand => REPEAT_LAST_COMMAND,
            Self::ZoomIn => ZOOM_IN,
            Self::ZoomOut => ZOOM_OUT,
            Self::ZoomFit => ZOOM_FIT,
            Self::FitSchematicContent => FIT_SCHEMATIC_CONTENT,
            Self::ZoomBox => ZOOM_BOX,
            Self::ZoomPrevious => ZOOM_PREVIOUS,
            Self::RotateSelection => ROTATE_SELECTION,
            Self::RotateSelectionCounterClockwise => ROTATE_SELECTION_CCW,
            Self::MirrorSelectionHorizontal => MIRROR_SELECTION_HORIZONTAL,
            Self::MirrorSelectionVertical => MIRROR_SELECTION_VERTICAL,
            Self::CycleGrid => GRID,
            Self::ZoomOneToOne => ZOOM_ONE_TO_ONE,
            Self::ToggleFullScreen => FULL_SCREEN,
            Self::ToggleNavigator => TOGGLE_NAVIGATOR,
            Self::ToggleInspector => TOGGLE_INSPECTOR,
            Self::ToggleConsole => TOGGLE_CONSOLE,
            Self::ToggleFocusMode => FOCUS_WORKSPACE,
            Self::NewApplicationWindow => NEW_APPLICATION_WINDOW,
            Self::PreviousDocument => PREVIOUS_DOCUMENT,
            Self::NextDocument => NEXT_DOCUMENT,
            Self::NextPane => NEXT_PANE,
            Self::PreviousPane => PREVIOUS_PANE,
            Self::HelpCenter => HELP_CENTER,
            Self::SelectTool => NONE,
            Self::PlaceInstance => PLACE_INSTANCE,
            Self::PlaceWire => PLACE_WIRE,
            Self::PlaceBus => PLACE_BUS,
            Self::PlaceBusTap => PLACE_BUS_TAP,
            Self::PlaceJunction => PLACE_JUNCTION,
            Self::PlaceLabel => PLACE_LABEL,
            // Every plausible mnemonic in the placement family is taken; the
            // toolbar and the palette are its routes.
            Self::PlaceOffSheetConnector => NONE,
            Self::PlaceProbe => PLACE_PROBE,
            Self::PlacePin => PLACE_PIN,
            Self::PlaceText => PLACE_TEXT,
            Self::PlaceShape => NONE,
            Self::MoveSelection => MOVE_SELECTION,
            Self::StretchSelection => STRETCH_SELECTION,
            Self::ArraySelection => NONE,
            Self::ReplaceInstance => NONE,
            Self::SwapToRSpiceSymbols => NONE,
            Self::UpdateInstanceInterface => NONE,
            Self::CreateHierarchy => NONE,
            Self::ConfigurationSets => NONE,
            Self::SymbolPinTool => SYMBOL_PIN,
            Self::SymbolPolylineTool => SYMBOL_POLYLINE,
            Self::SymbolRectangleTool => SYMBOL_RECTANGLE,
            Self::SymbolCircleTool => SYMBOL_CIRCLE,
            Self::SymbolArcTool => SYMBOL_ARC,
            Self::SymbolPolygonTool => SYMBOL_POLYGON,
            Self::SymbolTextTool => SYMBOL_TEXT,
            Self::SymbolRotatePin => SYMBOL_ROTATE_PIN,
            Self::SymbolMirrorPin => SYMBOL_MIRROR_PIN,
            Self::SymbolUpdatePinsFromContract => NONE,
            Self::SymbolSave => NONE,
            Self::EditSymbolInPlace => NONE,
            Self::ExitSymbolInPlace => EXIT_SYMBOL_IN_PLACE,
            Self::Place(crate::state::ComponentType::Resistor) => PLACE_RESISTOR,
            Self::Place(crate::state::ComponentType::Capacitor) => PLACE_CAPACITOR,
            Self::Place(crate::state::ComponentType::Inductor) => PLACE_INDUCTOR,
            Self::Place(crate::state::ComponentType::Diode) => PLACE_DIODE,
            Self::Place(crate::state::ComponentType::Ground) => PLACE_GROUND,
            Self::Place(crate::state::ComponentType::VoltageSource) => PLACE_VOLTAGE_SOURCE,
            Self::Place(crate::state::ComponentType::CurrentSource) => PLACE_CURRENT_SOURCE,
            Self::AscendHierarchy => ASCEND_HIERARCHY,
            Self::DescendHierarchy => DESCEND_HIERARCHY,
            Self::DescendHierarchyDirect => DESCEND_HIERARCHY_DIRECT,
            Self::CheckCellview => CHECK_CELLVIEW,
            // Ctrl+Alt+E is AltGr+E, the euro sign, on most European
            // layouts, and every other chord of the check family is taken;
            // the Design menu, the toolbar and the palette are its routes.
            Self::CheckDesign => NONE,
            Self::NextViolation => NEXT_VIOLATION,
            Self::PreviousViolation => PREVIOUS_VIOLATION,
            Self::CommandPalette => COMMAND_PALETTE,
            Self::Cancel => CANCEL,
            _ => NONE,
        }
    }

    /// Immutable factory binding label for registry contract tests and
    /// recovery diagnostics. Product UI must use `ShortcutPreferences`.
    #[cfg(test)]
    pub fn default_shortcut_label(self, platform: CommandPlatform) -> &'static str {
        self.shortcut_bindings()
            .iter()
            .find(|binding| binding.supports(platform))
            .map_or("", |binding| binding.chord.label)
    }

    pub fn availability(self, app: &RSpiceApp) -> CommandAvailability {
        if self.held_by_project_operation(&app.state.platform) {
            return CommandAvailability::Disabled("project operation is still in progress");
        }
        if self.lacks_open_project(&app.state.platform) {
            return CommandAvailability::Disabled("no project is open");
        }
        if self.mutates_window_session() && app.state.application_modal_open() {
            return CommandAvailability::Disabled(
                "finish or cancel the active application dialog before changing window ownership",
            );
        }
        if self.is_enabled(app) {
            return CommandAvailability::Available;
        }
        let reason = match self {
            Self::OpenWorkspace(workspace) if workspace != Workspace::Project => {
                "no project is open"
            }
            Self::RevertActiveDocument => "active document has no changes to revert",
            Self::CloseActiveDocument => "no closable document is active",
            Self::DetachDocument => "no active document is available to detach",
            Self::MoveDocumentToWindow => {
                "create another application window before moving this document"
            }
            Self::ReattachDocument => "the active document is already in the main window",
            Self::ConsolidateWindows => "there are no detached application windows",
            Self::MonitorRecovery => "there are no secondary application windows to recover",
            // A step taken mid-drag would be folded into the drag when it
            // lands, so the history waits for the edit to finish or cancel.
            Self::Undo | Self::Redo if app.state.schematic_edit_in_progress() => {
                "finish or cancel the edit in progress first"
            }
            Self::Undo => "nothing to undo",
            Self::Redo => "nothing to redo",
            // The repair exists only for an instance the deck already refuses
            // to emit, so its refusal names that condition rather than the
            // generic "select an editable object" the editing family reports.
            Self::UpdateInstanceInterface if !super::active_schematic_editor(app) => {
                "open an editable schematic or testbench"
            }
            Self::UpdateInstanceInterface if app.state.schematic_edit_read_only() => {
                "the active schematic is read-only"
            }
            Self::UpdateInstanceInterface => {
                "select one instance whose master interface changed after it was placed"
            }
            Self::PlaceInstance
            | Self::PlaceWire
            | Self::PlaceBus
            | Self::PlaceBusTap
            | Self::PlaceJunction
            | Self::PlaceLabel
            | Self::PlaceOffSheetConnector
            | Self::PlaceProbe
            | Self::PlacePin
            | Self::PlaceText
            | Self::PlaceShape
            | Self::Place(_)
                if !super::active_schematic_editor(app) =>
            {
                "open an editable schematic or testbench"
            }
            Self::PlaceInstance
            | Self::PlaceWire
            | Self::PlaceBus
            | Self::PlaceBusTap
            | Self::PlaceJunction
            | Self::PlaceLabel
            | Self::PlaceOffSheetConnector
            | Self::PlaceProbe
            | Self::PlacePin
            | Self::PlaceText
            | Self::PlaceShape
            | Self::Place(_)
                if app.state.schematic_edit_read_only() =>
            {
                "the active schematic is read-only"
            }
            Self::MoveSelection
            | Self::StretchSelection
            | Self::ArraySelection
            | Self::ReplaceInstance
            | Self::SwapToRSpiceSymbols
            | Self::CreateHierarchy
            | Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical
            | Self::SnapSelectionToGrid
            | Self::SnapToStandardValue
            | Self::JoinCollinearWires
                if !super::active_schematic_editor(app) =>
            {
                "open an editable schematic or testbench"
            }
            Self::MoveSelection
            | Self::StretchSelection
            | Self::ArraySelection
            | Self::ReplaceInstance
            | Self::SwapToRSpiceSymbols
            | Self::CreateHierarchy
            | Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical
            | Self::SnapSelectionToGrid
            | Self::JoinCollinearWires
                if app.state.schematic_edit_read_only() =>
            {
                "the active schematic is read-only"
            }
            Self::Cut
            | Self::Copy
            | Self::Duplicate
            | Self::Delete
            | Self::MoveSelection
            | Self::StretchSelection
            | Self::ArraySelection
            | Self::ReplaceInstance
            | Self::CreateHierarchy
            | Self::RotateSelection
            | Self::RotateSelectionCounterClockwise
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical
            | Self::SnapSelectionToGrid => "select an editable object",
            Self::SwapToRSpiceSymbols => "no part here draws an imported symbol",
            Self::SnapToStandardValue => {
                crate::schematic::view::standard_value_snap::snap_block_reason(&app.state)
                    .unwrap_or("select a resistor, capacitor or inductor")
            }
            Self::HighlightNet if !super::active_schematic_editor(app) => {
                "open a schematic or testbench"
            }
            // What the command would have refused with is the reason.
            Self::HighlightNet => crate::schematic::view::selected_net(&app.state)
                .err()
                .unwrap_or("command is unavailable in this context"),
            Self::ConfigurationSets => "open a project",
            Self::Paste => "clipboard has no compatible content",
            Self::RenameSelection => "select one editable component, net label, or declared bus",
            Self::ObjectProperties => "select one inspectable object",
            Self::AscendHierarchy if !super::active_schematic_editor(app) => {
                "open a schematic or testbench"
            }
            Self::AscendHierarchy => "already at top hierarchy",
            Self::DescendHierarchy | Self::DescendHierarchyDirect
                if !super::active_schematic_editor(app) =>
            {
                "open a schematic or testbench"
            }
            Self::DescendHierarchy => "select one hierarchical instance",
            Self::DescendHierarchyDirect => "select one hierarchical or Verilog-A instance",
            Self::EditSymbolInPlace => app
                .state
                .selected_symbol_in_place_target()
                .err()
                .map_or("select one instance", |refusal| refusal.reason()),
            Self::ExitSymbolInPlace => "no symbol is being edited in place",
            Self::RepeatLastCommand if app.state.workbench.last_repeatable_command.is_none() => {
                "no placement or edit has run yet"
            }
            Self::DeselectAll | Self::ZoomToSelection => "nothing is selected",
            Self::ZoomPrevious
                if !super::active_schematic_editor(app)
                    && !super::active_symbol_editor(app)
                    && app.routed_command_key_enabled(self.key()).is_none() =>
            {
                "open a schematic or symbol"
            }
            Self::ZoomPrevious => "the view has not changed yet",
            Self::SelectOfKind(_) if !super::active_schematic_editor(app) => {
                "open a schematic or testbench"
            }
            Self::SelectOfKind(kind) => kind.absent(),
            Self::NextViolation | Self::PreviousViolation => "the last check found no violations",
            Self::ClearChecks => "no check has run",
            Self::SetGridStyle(_)
            | Self::AnnotateOperatingPoint
            | Self::AnnotateTransientVoltages => "open a schematic or testbench",
            Self::CloseOtherDocuments => "no other closable document is open",
            Self::CloseAllDocuments => "no closable document is open",
            Self::PreviousDocument | Self::NextDocument => "only one document is open",
            _ => "command is unavailable in this context",
        };
        CommandAvailability::Disabled(reason)
    }

    pub fn primary_is_reserved_on(self, platform: CommandPlatform) -> bool {
        platform.has_browser_reserved_primary()
            && matches!(
                self,
                Self::CloseActiveDocument | Self::ToggleFullScreen | Self::ToggleConsole
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::VerificationPage;
    use crate::state::command_catalog::{CommandCatalog, CommandKey};
    use crate::workbench::ResultViewer;
    use crate::workbench::code::commands::CodeCommand;
    use crate::workbench::framework::command_registry::CommandFlags;
    use crate::workbench::results::commands::ResultsCommand;
    use crate::workbench::simulate::commands::SimulateCommand;

    fn binding_owners(chord: ShortcutChord, platform: CommandPlatform) -> Vec<Command> {
        crate::workbench::commands::vocabulary::COMMAND_REGISTRY
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

    /// Every command the registry orders, the shell's and the features',
    /// that binds `chord` on `platform`, in the registry's order.
    fn ordered_binding_owners(chord: ShortcutChord, platform: CommandPlatform) -> Vec<CommandKey> {
        crate::workbench::commands::vocabulary::command_registry()
            .ordered()
            .filter(|spec| {
                spec.bindings
                    .iter()
                    .any(|binding| binding.chord == chord && binding.supports(platform))
            })
            .map(|spec| spec.key)
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
            // The eighth workspace reads fourth on the rail, so the chord it
            // would have taken there belongs to Results. Alt+8 has to be
            // unclaimed for that to hold.
            assert_eq!(
                binding_owners(STIMULUS_WORKSPACE[0].chord, platform),
                vec![Command::OpenWorkspace(Workspace::Stimulus)]
            );
            assert_eq!(
                Command::OpenWorkspace(Workspace::Stimulus).default_shortcut_label(platform),
                "Alt+8"
            );
        }

        assert!(
            crate::workbench::commands::vocabulary::CommandRegistry
                .default_bindings(ResultsCommand::ResultViewer(ResultViewer::Waves).key())
                .is_empty()
        );
        assert!(
            crate::workbench::commands::vocabulary::CommandRegistry
                .default_bindings(SimulateCommand::Verify(VerificationPage::Yield).key())
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

    /// F2 renames whatever the focused surface holds: the schematic selection
    /// on a canvas, the symbol under the caret in a source editor. The two
    /// owners live in contexts that can never be active together.
    #[test]
    fn rename_shortcut_is_unambiguous_and_desktop_browser_only() {
        let rename_symbol = CodeCommand::RenameSymbol.declared_spec();
        for platform in [CommandPlatform::Desktop, CommandPlatform::Browser] {
            assert_eq!(
                ordered_binding_owners(RENAME_SELECTION[0].chord, platform),
                vec![Command::RenameSelection.key(), rename_symbol.key]
            );
            assert!(!crate::state::shortcuts::shortcut_contexts_overlap(
                Command::RenameSelection.shortcut_context(),
                rename_symbol.context,
            ));
            assert_eq!(
                rename_symbol
                    .bindings
                    .iter()
                    .find(|binding| binding.supports(platform))
                    .map_or("", |binding| binding.chord.label),
                "F2"
            );
            assert_eq!(
                Command::RenameSelection.default_shortcut_label(platform),
                "F2"
            );
        }
        for platform in [CommandPlatform::Tablet, CommandPlatform::Phone] {
            assert!(ordered_binding_owners(RENAME_SELECTION[0].chord, platform).is_empty());
            assert_eq!(
                Command::RenameSelection.default_shortcut_label(platform),
                ""
            );
        }
        assert_eq!(
            Command::RenameSelection.shortcut_context(),
            ShortcutContext::EditContext
        );
    }

    #[test]
    fn symbol_tool_defaults_are_complete_portable_and_view_scoped() {
        for (command, label) in [
            (Command::SymbolPinTool, "P"),
            (Command::SymbolPolylineTool, "L"),
            (Command::SymbolRectangleTool, "R"),
            (Command::SymbolCircleTool, "C"),
            (Command::SymbolArcTool, "A"),
            (Command::SymbolPolygonTool, "G"),
            (Command::SymbolTextTool, "T"),
            (Command::SymbolRotatePin, "Ctrl+R"),
            (Command::SymbolMirrorPin, "Ctrl+M"),
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
        assert_eq!(
            Command::PlaceBus.shortcut_context(),
            ShortcutContext::EngineeringCanvas
        );
        assert_eq!(
            Command::PlaceBusTap.shortcut_context(),
            ShortcutContext::EngineeringCanvas
        );
        assert_eq!(
            Command::PlaceJunction.shortcut_context(),
            ShortcutContext::EngineeringCanvas
        );
        for platform in CommandPlatform::ALL {
            assert_eq!(Command::PlaceBus.default_shortcut_label(platform), "B");
            assert_eq!(Command::PlaceBusTap.default_shortcut_label(platform), "T");
            assert_eq!(Command::PlaceJunction.default_shortcut_label(platform), "J");
        }
    }

    #[test]
    fn schematic_place_pin_is_shift_p_without_displacing_probe_or_symbol_pin() {
        assert_eq!(
            Command::PlacePin.shortcut_context(),
            ShortcutContext::EngineeringCanvas
        );
        assert_eq!(
            Command::PlaceProbe.shortcut_context(),
            ShortcutContext::EngineeringCanvas
        );
        assert_eq!(
            Command::SymbolPinTool.shortcut_context(),
            ShortcutContext::SymbolCanvas
        );

        for platform in CommandPlatform::ALL {
            assert_eq!(
                Command::PlacePin.default_shortcut_label(platform),
                "Shift+P"
            );
            assert_eq!(Command::PlaceProbe.default_shortcut_label(platform), "P");
            assert_eq!(Command::SymbolPinTool.default_shortcut_label(platform), "P");
            assert_eq!(
                binding_owners(PLACE_PIN[0].chord, platform),
                vec![Command::PlacePin]
            );
            assert_eq!(
                binding_owners(PLACE_PROBE[0].chord, platform),
                vec![Command::PlaceProbe, Command::SymbolPinTool]
            );
        }
    }

    #[test]
    fn schematic_place_text_is_shift_t_without_displacing_bus_tap() {
        assert_eq!(
            Command::PlaceText.shortcut_context(),
            ShortcutContext::EngineeringCanvas
        );
        assert_eq!(
            Command::PlaceBusTap.shortcut_context(),
            ShortcutContext::EngineeringCanvas
        );
        for platform in CommandPlatform::ALL {
            assert_eq!(
                Command::PlaceText.default_shortcut_label(platform),
                "Shift+T"
            );
            assert_eq!(Command::PlaceBusTap.default_shortcut_label(platform), "T");
            assert_eq!(
                binding_owners(PLACE_TEXT[0].chord, platform),
                vec![Command::PlaceText]
            );
            assert_eq!(
                binding_owners(PLACE_BUS_TAP[0].chord, platform),
                vec![Command::PlaceBusTap, Command::SymbolTextTool]
            );
            assert_eq!(
                Command::SymbolTextTool.shortcut_context(),
                ShortcutContext::SymbolCanvas
            );
        }
    }

    #[test]
    fn move_selection_is_unmodified_m_in_the_engineering_canvas() {
        assert_eq!(
            Command::MoveSelection.shortcut_context(),
            ShortcutContext::EngineeringCanvas
        );
        for platform in CommandPlatform::ALL {
            assert_eq!(Command::MoveSelection.default_shortcut_label(platform), "M");
            assert_eq!(
                binding_owners(MOVE_SELECTION[0].chord, platform),
                vec![Command::MoveSelection]
            );
        }
    }

    #[test]
    fn stretch_selection_solely_owns_unmodified_s_in_the_engineering_canvas() {
        assert_eq!(
            Command::StretchSelection.shortcut_context(),
            ShortcutContext::EngineeringCanvas
        );
        assert!(Command::SelectTool.shortcut_bindings().is_empty());
        for platform in CommandPlatform::ALL {
            assert_eq!(
                Command::StretchSelection.default_shortcut_label(platform),
                "S"
            );
            assert_eq!(
                binding_owners(STRETCH_SELECTION[0].chord, platform),
                vec![Command::StretchSelection]
            );
        }
    }

    #[test]
    fn split_results_owns_the_mockup_shortcut_on_every_platform() {
        let catalog = crate::workbench::commands::vocabulary::CommandRegistry;
        let split = ResultsCommand::ToggleResultsSplit.key();
        assert_eq!(catalog.context(split), Some(ShortcutContext::Global));
        for platform in CommandPlatform::ALL {
            let binding = catalog
                .default_bindings(split)
                .iter()
                .find(|binding| binding.supports(platform))
                .expect("the split is bound on every platform");
            assert_eq!(binding.chord.label, "Ctrl+\\");
            let owners = catalog
                .commands()
                .iter()
                .copied()
                .filter(|key| {
                    catalog.default_bindings(*key).iter().any(|candidate| {
                        candidate.chord == binding.chord && candidate.supports(platform)
                    })
                })
                .collect::<Vec<_>>();
            assert_eq!(owners, vec![split]);
        }
    }

    #[test]
    fn create_array_is_canvas_scoped_and_has_no_factory_shortcut() {
        assert_eq!(
            Command::ArraySelection.shortcut_context(),
            ShortcutContext::EngineeringCanvas
        );
        assert!(Command::ArraySelection.shortcut_bindings().is_empty());
        for platform in CommandPlatform::ALL {
            assert_eq!(Command::ArraySelection.default_shortcut_label(platform), "");
        }
    }

    /// The connector declares no chord, so the palette seat and the toolbar
    /// are its whole reachability. A registry row that lost either would ship
    /// a command nobody can invoke.
    #[test]
    fn the_off_sheet_connector_is_reachable_without_a_chord_of_its_own() {
        let spec = Command::PlaceOffSheetConnector.spec();
        assert_eq!(spec.id, "place-off-sheet-connector");
        assert_eq!(spec.label, "Off-sheet connector");
        assert_eq!(spec.group, "Design");
        assert!(
            crate::workbench::commands::vocabulary::COMMAND_REGISTRY
                .contains(&Command::PlaceOffSheetConnector),
            "the palette projects the registry, so an unlisted command has no seat"
        );
        assert!(Command::PlaceOffSheetConnector.palette_visible());
        assert!(
            Command::PlaceOffSheetConnector
                .shortcut_bindings()
                .is_empty()
        );
        assert_eq!(
            Command::PlaceOffSheetConnector.shortcut_context(),
            ShortcutContext::EngineeringCanvas
        );
        assert!(!Command::PlaceOffSheetConnector.requires_open_project());
        assert!(super::super::command_edits_schematic(
            Command::PlaceOffSheetConnector
        ));
    }

    /// Arming is what separates the two naming tools: they share one placement
    /// transaction, and only the armed tool tells it which label to publish.
    #[test]
    fn the_off_sheet_connector_arms_a_tool_the_label_command_does_not() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.activate(Workspace::Design);
        app.state.open_project_session();
        assert!(Command::PlaceOffSheetConnector.is_enabled(&app));

        Command::PlaceOffSheetConnector.execute(&mut app);
        assert_eq!(
            app.state.model.schematic.session.editor.tool,
            crate::state::Tool::OffSheetConnector
        );

        Command::PlaceLabel.execute(&mut app);
        assert_eq!(
            app.state.model.schematic.session.editor.tool,
            crate::state::Tool::Label
        );
    }

    const SHOW_IN_NETLIST_DECK: &str =
        "instance locator fixture\nV1 in 0 1\nR1 in out 1k\n.op\n.end\n";

    /// An open design whose retained generated deck states `R1` and not `C9`.
    fn app_with_generated_deck() -> (RSpiceApp, u64, u64) {
        use crate::state::{
            ComponentType, GeneratedArtifact, GeneratedProvenance, GeneratedSourceMapEntry,
            GenerationInput, NetlistDocument, NetlistDocumentId, Point,
        };

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        app.state.open_project_session();
        app.state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .clear();

        let origin = Point::new(20, 20);
        let load_id = app
            .state
            .model
            .schematic
            .add_component(ComponentType::Resistor, origin);
        let unmapped_id = app
            .state
            .model
            .schematic
            .add_component(ComponentType::Capacitor, origin + Point::new(20, 0));
        for (id, name) in [(load_id, "R1"), (unmapped_id, "C9")] {
            if let Some(component) = app
                .state
                .model
                .schematic
                .document_mut_for_test()
                .components
                .iter_mut()
                .find(|component| component.id == id)
            {
                component.name = name.to_owned();
            }
        }

        let view = app.state.model.workspace.content.active_view.key();
        let cell = format!(
            "{}/{}",
            app.state.model.workspace.content.active_view.library,
            app.state.model.workspace.content.active_view.cell
        );
        let digest = crate::state::content_digest("instance-locator-input");
        let artifact = GeneratedArtifact::try_from_utf8(
            GeneratedProvenance::try_new(
                "rspice-instance-locator-test",
                GenerationInput::new(crate::product::ObjectRevision::INITIAL, digest),
            )
            .expect("provenance"),
            SHOW_IN_NETLIST_DECK.as_bytes().to_vec(),
            Vec::new(),
            vec![
                GeneratedSourceMapEntry::try_new(
                    3,
                    cell,
                    view.clone(),
                    Some("R1".to_owned()),
                    Some(GeneratedSourceMapEntry::component_identity_for(
                        &view, load_id,
                    )),
                )
                .expect("mapping"),
            ],
        )
        .expect("artifact");

        app.state.model.netlist_artifacts.generated_source = SHOW_IN_NETLIST_DECK.to_owned();
        app.state.model.netlist_artifacts.generated_document = Some(
            NetlistDocument::from_generated(NetlistDocumentId::new(), artifact).expect("document"),
        );
        app.state.model.netlist_artifacts.generated_input_digest = Some(digest);
        app.state
            .model
            .netlist_artifacts
            .current_generation_input_digest = Some(digest);
        app.state.code.netlist.active_document_initialized = true;
        (app, load_id, unmapped_id)
    }

    /// The jump is reachable from the palette and carries no shortcut it would
    /// have to take from another command. A route offered only by a
    /// context-menu row cannot be bound or automated.
    #[test]
    fn show_in_netlist_is_one_discoverable_command() {
        let spec = CodeCommand::ShowInNetlist.declared_spec();
        assert!(
            crate::workbench::commands::vocabulary::command_registry()
                .order()
                .contains(&spec.key)
        );
        assert_eq!(spec.context, ShortcutContext::DesignWorkspace);
        assert!(spec.flags.contains(CommandFlags::PALETTE_VISIBLE));
        assert!(spec.bindings.is_empty());
    }

    /// Every disabled state names the thing that is missing, in the locator's
    /// own wording, so the context-menu row, the inspector and the palette
    /// cannot explain the same block three different ways.
    #[test]
    fn show_in_netlist_reports_the_missing_precondition_rather_than_a_generic_block() {
        let (mut app, load_id, unmapped_id) = app_with_generated_deck();

        assert_eq!(
            app.command_availability(CodeCommand::ShowInNetlist.key()),
            CommandAvailability::Disabled("select one instance")
        );

        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(load_id);
        assert_eq!(
            app.command_availability(CodeCommand::ShowInNetlist.key()),
            CommandAvailability::Available
        );

        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(unmapped_id);
        assert_eq!(
            app.command_availability(CodeCommand::ShowInNetlist.key()),
            CommandAvailability::Disabled("no netlist line for this instance")
        );

        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(load_id);
        app.state.model.netlist_artifacts.generated_document = None;
        app.state.model.netlist_artifacts.generated_source.clear();
        assert_eq!(
            app.command_availability(CodeCommand::ShowInNetlist.key()),
            CommandAvailability::Disabled(
                "no generated netlist yet — open the Netlist workspace to generate one"
            )
        );

        // Outside a schematic there is no instance to locate at all, and
        // "select one instance" would send the engineer to the wrong place.
        let (mut app, load_id, _) = app_with_generated_deck();
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(load_id);
        app.state.workbench.workspace = Workspace::Results;
        assert_eq!(
            app.command_availability(CodeCommand::ShowInNetlist.key()),
            CommandAvailability::Disabled("open a schematic or testbench")
        );
    }

    /// Dispatch moves the workspace, the document and the caret, and leaves
    /// the schematic selection where it was — the netlist navigator's reveal
    /// is what brings it back, and it reads that same selection.
    #[test]
    fn show_in_netlist_opens_the_generated_deck_without_moving_the_selection() {
        let (mut app, load_id, _) = app_with_generated_deck();
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_only_component(load_id);

        app.execute_command_key(CodeCommand::ShowInNetlist.key());

        assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
        assert_eq!(
            app.state.code.netlist.active_document,
            crate::workbench::documents::netlist_document::ActiveNetlistDocument::Generated
        );
        assert_eq!(app.state.code.netlist.requested_line, Some(3));
        assert_eq!(app.state.code.netlist.cursor_line, 2);
        assert_eq!(
            app.state
                .model
                .schematic
                .session
                .editor
                .selection
                .single_component(),
            Some(load_id)
        );
    }
}
