//! The command vocabulary.
//!
//! Every command the product exposes, its stable portable identity, and the
//! label and group a menu or palette shows it under. This is a value model:
//! it describes what commands exist, not what pressing one does. Dispatch —
//! `is_enabled` and `execute`, both of which need the running application —
//! lives up in [`crate::workbench::commands`].
//!
//! Keeping the two apart is what lets the artifact writers name a command
//! without depending on the dispatcher that runs it. Shortcut profiles name
//! none: they read command facts by key through [`CommandRegistry`].

use std::collections::HashMap;
use std::sync::LazyLock;

use crate::io::NetlistFormat;
use crate::schematic::view::sheet_visibility::SchematicObjectKind;
use crate::state::ModelsPage;
use crate::state::command_catalog::{
    CommandCatalog, CommandKey, CommandPlatform, ShortcutBinding, ShortcutContext,
};
use crate::state::{SimulationPage, VerificationPage};
use crate::workbench::cloud::commands::CloudCommand;
use crate::workbench::code::commands::CodeCommand;
use crate::workbench::design::commands::DesignCommand;
use crate::workbench::framework::command_registry::{self, CommandFlags, Registry};
use crate::workbench::hardcopy::commands::HardcopyCommand;
use crate::workbench::models::commands::ModelsCommand;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::results::commands::ResultsCommand;
use crate::workbench::settings::commands::SettingsCommand;
use crate::workbench::sheets::commands::SheetsCommand;
use crate::workbench::simulate::commands::SimulateCommand;
use crate::workbench::state::Workspace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// `CommandPalette` is the feature's name, the same one the shortcut
// preferences show the user. Trimming it to `Palette` to stop repeating the
// enum would rename a thing that exists.
#[allow(clippy::enum_variant_names)]
pub enum Command {
    OpenWorkspace(Workspace),
    RevertActiveDocument,
    CloseActiveDocument,
    Exit,
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    Duplicate,
    Delete,
    SelectAll,
    /// Clear the drawing's selection.
    DeselectAll,
    /// Select every object of one kind on the active sheet.
    SelectOfKind(SchematicObjectKind),
    ObjectProperties,
    FindInDesign,
    FindReplace,
    /// Run the last placement or editing command again, as it last ran.
    RepeatLastCommand,
    ZoomIn,
    ZoomOut,
    ZoomFit,
    ZoomOneToOne,
    ZoomToSelection,
    ZoomBox,
    /// Go back to the view the canvas showed before this one.
    ZoomPrevious,
    /// Show or hide the DC operating point on the schematic.
    AnnotateOperatingPoint,
    /// Show or hide the transient's voltages at results cursor A on the
    /// schematic.
    AnnotateTransientVoltages,
    ToggleFullScreen,
    ToggleNavigator,
    ToggleInspector,
    ToggleConsole,
    OpenConsole,
    OpenProblems,
    /// Open the session notification panel. The bell opens it on a shell wide
    /// enough to draw one; this is how every other shell reaches it.
    OpenNotifications,
    ToggleConsoleMaximized,
    ClearConsole,
    ToggleFocusMode,
    ResetLayout,
    NewApplicationWindow,
    DetachDocument,
    MoveDocumentToWindow,
    ReattachDocument,
    ConsolidateWindows,
    MonitorRecovery,
    PreviousDocument,
    NextDocument,
    /// Move the keyboard to the next pane of the workbench. F6.
    NextPane,
    /// Move the keyboard to the previous pane of the workbench. Shift+F6.
    PreviousPane,
    CloseOtherDocuments,
    CloseAllDocuments,
    WindowManager,
    /// The macOS Window menu's Minimize: the window goes to the Dock.
    MinimizeWindow,
    /// The macOS Window menu's Zoom: the window fills the screen, or goes
    /// back to the size it had.
    ZoomWindow,
    MoveSelection,
    StretchSelection,
    ArraySelection,
    /// Draw the selected parts, or the whole sheet, with RSpice's own symbols
    /// instead of the artwork their drawing brought.
    SwapToRSpiceSymbols,
    /// Re-place a stale instance on its own master: same cell, the interface
    /// the master presents now. It is a degenerate replacement, which is why
    /// it sits beside [`DesignCommand::ReplaceInstance`] rather than in a repair
    /// vocabulary of its own.
    UpdateInstanceInterface,
    /// Light the net the selection is on.
    HighlightNet,
    ConfigurationSets,
    SymbolPinTool,
    SymbolPolylineTool,
    SymbolRectangleTool,
    SymbolCircleTool,
    SymbolArcTool,
    SymbolPolygonTool,
    SymbolTextTool,
    SymbolRotatePin,
    SymbolMirrorPin,
    SymbolUpdatePinsFromContract,
    SymbolSave,
    /// Edit the selected instance's symbol master where the instance stands
    /// on its parent schematic.
    EditSymbolInPlace,
    /// Leave an in-place symbol edit for the parent schematic.
    ExitSymbolInPlace,
    RotateSelection,
    RotateSelectionCounterClockwise,
    MirrorSelectionHorizontal,
    MirrorSelectionVertical,
    AlignSelectionLeft,
    AlignSelectionCenter,
    AlignSelectionRight,
    AlignSelectionTop,
    AlignSelectionMiddle,
    AlignSelectionBottom,
    DistributeSelectionHorizontal,
    DistributeSelectionVertical,
    /// Move every selected object's anchor to the nearest grid point.
    SnapSelectionToGrid,
    /// Snap each selected resistor, capacitor and inductor to the nearest
    /// value of the project's preferred-number series.
    SnapToStandardValue,
    /// Choose the preferred-number series the project's values step along.
    SetStandardValueSeries(rspice_vocabulary::quantity::ESeries),
    /// Lay the active sheet's wires that run on one line, end to end or
    /// overlapping, as one wire each.
    JoinCollinearWires,
    Cancel,
    AscendHierarchy,
    DescendHierarchy,
    /// Direct edit-in-place hierarchy gesture used by the toolbar and
    /// Shift+E. The Design-menu command above owns the explicit transaction.
    DescendHierarchyDirect,
    /// The open cellview's own findings: what an author runs while drawing.
    /// A symbol view checks its pins against the cell's interface.
    CheckCellview,
    /// The whole design, walked from the simulation root under its
    /// configuration, with every cellview of the project's own libraries the
    /// walk never reaches.
    CheckDesign,
    ClearChecks,
    NextViolation,
    PreviousViolation,
    CommandPalette,
    KeyboardShortcuts,
    HelpCenter,
    Examples,
    ReleaseNotes,
    SystemDiagnostics,
    SupportBundle,
    LegalPrivacy,
    About,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandSpec {
    pub id: &'static str,
    pub label: &'static str,
    pub group: &'static str,
}

impl Command {
    /// Stable product identity used by portable command profiles.
    pub const fn stable_id(self) -> &'static str {
        self.spec().id
    }

    /// The key a shortcut profile names this command by.
    pub const fn key(self) -> CommandKey {
        CommandKey::new(self.stable_id())
    }

    /// Resolve a stable product identity through the canonical registry.
    /// Unknown IDs are intentionally retained by profile persistence but are
    /// never guessed into a command owned by this build. The product resolves
    /// a stored id to a key; a key becomes a command only at dispatch.
    #[cfg(test)]
    pub fn from_stable_id(id: &str) -> Option<Self> {
        command_registry().resolve(id).and_then(legacy_command)
    }

    pub const fn spec(self) -> CommandSpec {
        match self {
            Self::OpenWorkspace(Workspace::Project) => {
                spec("project", "Open project workspace", "Navigate")
            }
            Self::OpenWorkspace(Workspace::Design) => {
                spec("design", "Open design workspace", "Navigate")
            }
            Self::OpenWorkspace(Workspace::Simulate) => {
                spec("simulate", "Open simulation workspace", "Navigate")
            }
            Self::OpenWorkspace(Workspace::Stimulus) => {
                spec("stimulus", "Open Stimulus Library", "Navigate")
            }
            Self::OpenWorkspace(Workspace::Results) => {
                spec("results", "Open results workspace", "Navigate")
            }
            Self::OpenWorkspace(Workspace::Verify) => {
                spec("verify", "Open verification workspace", "Navigate")
            }
            Self::OpenWorkspace(Workspace::Models) => {
                spec("models", "Open models workspace", "Navigate")
            }
            Self::OpenWorkspace(Workspace::Netlist) => {
                spec("netlist", "Open Netlist & Script Editor", "Navigate")
            }
            Self::RevertActiveDocument => spec("revert-document", "Revert to saved…", "File"),
            Self::CloseActiveDocument => spec("close-document", "Close", "File"),

            Self::Exit => spec("exit-rspice", "Exit", "File"),
            Self::Undo => spec("undo", "Undo", "Edit"),
            Self::Redo => spec("redo", "Redo", "Edit"),
            Self::Cut => spec("cut-selection", "Cut", "Edit"),
            Self::Copy => spec("copy-selection", "Copy", "Edit"),
            Self::Paste => spec("paste-selection", "Paste", "Edit"),
            Self::Duplicate => spec("duplicate-selection", "Duplicate", "Edit"),
            Self::Delete => spec("delete-selection", "Delete", "Edit"),
            Self::SelectAll => spec("select-all", "Select all", "Edit"),
            Self::DeselectAll => spec("deselect-all", "Deselect all", "Edit"),
            Self::SelectOfKind(SchematicObjectKind::Instances) => {
                spec("select-instances", "Select instances", "Edit")
            }
            Self::SelectOfKind(SchematicObjectKind::Wires) => {
                spec("select-wires", "Select wires", "Edit")
            }
            Self::SelectOfKind(SchematicObjectKind::NetLabels) => {
                spec("select-net-labels", "Select net labels", "Edit")
            }
            Self::SelectOfKind(SchematicObjectKind::Pins) => {
                spec("select-pins", "Select pins", "Edit")
            }
            Self::SelectOfKind(SchematicObjectKind::Text) => {
                spec("select-text", "Select text", "Edit")
            }
            Self::SelectOfKind(SchematicObjectKind::Shapes) => {
                spec("select-shapes", "Select shapes", "Edit")
            }
            Self::ObjectProperties => spec("object-properties", "Properties…", "Edit"),
            Self::FindInDesign => spec("find-design", "Find…", "Edit"),
            Self::FindReplace => spec("find-replace", "Find and replace…", "Edit"),
            Self::RepeatLastCommand => spec("repeat-last-command", "Repeat last command", "Edit"),
            Self::ZoomIn => spec("zoom-in", "Zoom in", "View"),
            Self::ZoomOut => spec("zoom-out", "Zoom out", "View"),
            Self::ZoomFit => spec("canvas-fit-sheet", "Fit drawing sheet", "View"),
            Self::ZoomOneToOne => spec("zoom-one-to-one", "Zoom 100%", "View"),
            Self::ZoomToSelection => spec("zoom-to-selection", "Zoom to selection", "View"),
            Self::ZoomBox => spec("zoom-box", "Zoom box", "View"),
            Self::ZoomPrevious => spec("zoom-previous", "Zoom previous", "View"),
            Self::ToggleFullScreen => spec("full-screen", "Enter full screen", "View"),
            Self::ToggleNavigator => spec("toggle-navigator", "Toggle navigator", "Window"),
            Self::ToggleInspector => spec("toggle-inspector", "Toggle inspector", "Window"),
            Self::ToggleConsole => spec("console", "Toggle console", "Window"),
            Self::OpenConsole => spec("open-console", "Open console", "Window"),
            Self::OpenProblems => spec("open-problems", "Open Problems", "Window"),
            Self::OpenNotifications => spec("open-notifications", "Open Notifications", "Window"),
            Self::ToggleConsoleMaximized => {
                spec("console-maximize", "Maximize or restore console", "Window")
            }
            Self::ClearConsole => spec("console-clear", "Clear console output", "Window"),
            Self::ToggleFocusMode => spec("toggle-focus-mode", "Focus workspace", "Window"),
            Self::ResetLayout => spec("reset-workspace-layout", "Reset layout", "Window"),
            Self::NewApplicationWindow => {
                spec("new-application-window", "New window\u{2026}", "Window")
            }
            Self::DetachDocument => spec(
                "detach-document",
                "Move document to new window\u{2026}",
                "Window",
            ),
            Self::MoveDocumentToWindow => spec(
                "move-document-window",
                "Move document to another window\u{2026}",
                "Window",
            ),
            Self::ReattachDocument => spec("reattach-document", "Reattach document", "Window"),
            Self::ConsolidateWindows => {
                spec("consolidate-windows", "Merge all windows\u{2026}", "Window")
            }
            Self::MonitorRecovery => spec(
                "monitor-recovery",
                "Recover off-screen windows\u{2026}",
                "Window",
            ),
            Self::PreviousDocument => spec("previous-document", "Previous document", "Window"),
            Self::NextDocument => spec("next-document", "Next document", "Window"),
            Self::NextPane => spec("next-pane", "Next pane", "Window"),
            Self::PreviousPane => spec("previous-pane", "Previous pane", "Window"),
            Self::CloseOtherDocuments => spec("close-other-documents", "Close others", "File"),
            Self::CloseAllDocuments => spec("close-all-documents", "Close all", "File"),
            Self::WindowManager => spec(
                "window-manager",
                "Windows, documents and session\u{2026}",
                "Window",
            ),
            Self::MinimizeWindow => spec("minimize-window", "Minimize", "Window"),
            Self::ZoomWindow => spec("zoom-window", "Zoom", "Window"),
            Self::MoveSelection => spec("move-selection", "Move", "Edit"),
            Self::StretchSelection => spec("stretch-selection", "Stretch", "Edit"),
            Self::ArraySelection => spec("array-selection", "Create array\u{2026}", "Edit"),
            Self::SwapToRSpiceSymbols => {
                spec("swap-to-rspice-symbols", "Swap to RSpice symbols", "Edit")
            }
            Self::UpdateInstanceInterface => spec(
                "update-instance-interface",
                "Update instance interface",
                "Design",
            ),
            Self::HighlightNet => spec("highlight-net", "Highlight net", "Design"),
            Self::ConfigurationSets => spec(
                "configuration-sets",
                "Configuration sets\u{2026}",
                "Simulate",
            ),
            Self::SymbolPinTool => spec("symbol-pin-tool", "Place symbol pin", "Design"),
            Self::SymbolPolylineTool => spec("symbol-line-tool", "Draw symbol line", "Design"),
            Self::SymbolRectangleTool => {
                spec("symbol-rectangle-tool", "Draw symbol rectangle", "Design")
            }
            Self::SymbolCircleTool => spec("symbol-circle-tool", "Draw symbol circle", "Design"),
            Self::SymbolArcTool => spec("symbol-arc-tool", "Draw symbol arc", "Design"),
            Self::SymbolPolygonTool => spec("symbol-polygon-tool", "Draw symbol polygon", "Design"),
            Self::SymbolTextTool => spec("symbol-text-tool", "Place symbol text", "Design"),
            Self::SymbolRotatePin => spec(
                "symbol-rotate-pin",
                "Rotate selected pin to next side",
                "Design",
            ),
            Self::SymbolMirrorPin => spec(
                "symbol-mirror-pin",
                "Mirror selected pin across body",
                "Design",
            ),
            Self::SymbolUpdatePinsFromContract => spec(
                "update-pins-from-contract",
                "Update pins from contract",
                "Design",
            ),
            Self::SymbolSave => spec("save-symbol", "Validate and save symbol", "Design"),
            Self::EditSymbolInPlace => {
                spec("edit-symbol-in-place", "Edit symbol in place", "Design")
            }
            Self::ExitSymbolInPlace => spec(
                "exit-symbol-in-place",
                "Return to parent schematic",
                "Design",
            ),
            Self::RotateSelection => spec("rotate-selection", "Rotate clockwise", "Design"),
            Self::RotateSelectionCounterClockwise => {
                spec("rotate-selection-ccw", "Rotate counter-clockwise", "Design")
            }
            Self::MirrorSelectionHorizontal => spec(
                "mirror-selection-horizontal",
                "Mirror about vertical axis",
                "Design",
            ),
            Self::MirrorSelectionVertical => spec(
                "mirror-selection-vertical",
                "Mirror about horizontal axis",
                "Design",
            ),
            Self::AlignSelectionLeft => spec("align-selection-left", "Align left", "Design"),
            Self::AlignSelectionCenter => spec(
                "align-selection-horizontal-centers",
                "Align horizontal centers",
                "Design",
            ),
            Self::AlignSelectionRight => spec("align-selection-right", "Align right", "Design"),
            Self::AlignSelectionTop => spec("align-selection-top", "Align top", "Design"),
            Self::AlignSelectionMiddle => spec(
                "align-selection-vertical-centers",
                "Align vertical centers",
                "Design",
            ),
            Self::AlignSelectionBottom => spec("align-selection-bottom", "Align bottom", "Design"),
            Self::DistributeSelectionHorizontal => spec(
                "distribute-selection-horizontal",
                "Distribute horizontally",
                "Design",
            ),
            Self::DistributeSelectionVertical => spec(
                "distribute-selection-vertical",
                "Distribute vertically",
                "Design",
            ),
            Self::SnapSelectionToGrid => {
                spec("snap-selection-to-grid", "Snap selection to grid", "Design")
            }
            Self::SnapToStandardValue => {
                spec("snap-to-standard-value", "Snap to standard value", "Design")
            }
            Self::SetStandardValueSeries(series) => {
                use rspice_vocabulary::quantity::ESeries;
                match series {
                    ESeries::E6 => spec("standard-values-e6", "E6 standard values", "Design"),
                    ESeries::E12 => spec("standard-values-e12", "E12 standard values", "Design"),
                    ESeries::E24 => spec("standard-values-e24", "E24 standard values", "Design"),
                    ESeries::E48 => spec("standard-values-e48", "E48 standard values", "Design"),
                    ESeries::E96 => spec("standard-values-e96", "E96 standard values", "Design"),
                    ESeries::E192 => spec("standard-values-e192", "E192 standard values", "Design"),
                }
            }
            Self::JoinCollinearWires => {
                spec("join-collinear-wires", "Join collinear wires", "Design")
            }
            Self::Cancel => spec("cancel-active-command", "Cancel active command", "Design"),
            Self::AscendHierarchy => spec("ascend-hierarchy", "Return to parent", "Design"),
            // Two gestures, two rows: this one opens the descend dialog, and
            // the direct gesture below goes straight to the master. Sharing one
            // title made them indistinguishable everywhere the palette, the
            // shortcut editor and the command reference list them side by side.
            Self::DescendHierarchy => spec(
                "descend-hierarchy",
                "Descend with options\u{2026}",
                "Design",
            ),
            Self::DescendHierarchyDirect => spec("descend-selected-instance", "Descend", "Design"),
            Self::CheckCellview => spec("check-cellview", "Check cellview", "Design"),
            Self::CheckDesign => spec("check-design", "Check design", "Design"),
            Self::ClearChecks => spec("clear-checks", "Clear check results", "Design"),
            Self::NextViolation => spec("next-violation", "Next violation", "Design"),
            Self::PreviousViolation => spec("previous-violation", "Previous violation", "Design"),
            Self::AnnotateOperatingPoint => spec(
                "annotate-operating-point",
                "Annotate DC operating point",
                "Simulate",
            ),
            Self::AnnotateTransientVoltages => spec(
                "annotate-transient-voltages",
                "Annotate transient voltages",
                "Simulate",
            ),
            Self::CommandPalette => spec("command-palette", "Command palette", "Navigate"),
            Self::KeyboardShortcuts => spec("command-reference", "Keyboard shortcuts", "Help"),
            Self::HelpCenter => spec("help-center", "RSpice Help", "Help"),
            Self::Examples => spec("examples", "Examples\u{2026}", "Help"),
            Self::ReleaseNotes => spec("release-notes", "Release notes", "Help"),

            Self::SystemDiagnostics => {
                spec("system-diagnostics", "Report a problem\u{2026}", "Help")
            }
            Self::SupportBundle => spec("support-bundle", "Create support bundle\u{2026}", "Help"),
            Self::LegalPrivacy => spec(
                "legal-privacy-center",
                "Third-party notices and privacy",
                "Help",
            ),
            Self::About => spec("about", "About RSpice", "Help"),
        }
    }

    /// The command as the registry holds it.
    fn registry_spec(self) -> command_registry::CommandSpec {
        command_registry::CommandSpec {
            key: self.key(),
            label: self.spec().label,
            group: self.spec().group,
            flags: CommandFlags::NONE
                .with(CommandFlags::PALETTE_VISIBLE, self.palette_visible())
                .with(
                    CommandFlags::PRIMARY_RESERVED_BY_BROWSER,
                    self.primary_is_reserved_on(CommandPlatform::Browser),
                ),
            context: self.shortcut_context(),
            bindings: self.shortcut_bindings(),
            owner: self.owner(),
        }
    }
}

pub(in crate::workbench) const fn spec(
    id: &'static str,
    label: &'static str,
    group: &'static str,
) -> CommandSpec {
    CommandSpec { id, label, group }
}

/// Every command registered with application chrome. Search, menus,
/// accessibility help, and keyboard resolution all project from this list.
/// Context-only commands are retained here but can opt out of search.
pub const COMMAND_REGISTRY: &[Command] = &[
    Command::RevertActiveDocument,
    Command::CloseActiveDocument,
    Command::Exit,
    Command::Undo,
    Command::Redo,
    Command::Cut,
    Command::Copy,
    Command::Paste,
    Command::Duplicate,
    Command::Delete,
    Command::SelectAll,
    Command::DeselectAll,
    Command::SelectOfKind(SchematicObjectKind::Instances),
    Command::SelectOfKind(SchematicObjectKind::Wires),
    Command::SelectOfKind(SchematicObjectKind::NetLabels),
    Command::SelectOfKind(SchematicObjectKind::Pins),
    Command::SelectOfKind(SchematicObjectKind::Text),
    Command::SelectOfKind(SchematicObjectKind::Shapes),
    Command::ObjectProperties,
    Command::FindInDesign,
    Command::FindReplace,
    Command::RepeatLastCommand,
    Command::OpenWorkspace(Workspace::Project),
    Command::OpenWorkspace(Workspace::Design),
    Command::OpenWorkspace(Workspace::Simulate),
    Command::OpenWorkspace(Workspace::Stimulus),
    Command::OpenWorkspace(Workspace::Results),
    Command::OpenWorkspace(Workspace::Verify),
    Command::OpenWorkspace(Workspace::Models),
    Command::OpenWorkspace(Workspace::Netlist),
    Command::ZoomIn,
    Command::ZoomOut,
    Command::ZoomFit,
    Command::ZoomOneToOne,
    Command::ZoomToSelection,
    Command::ZoomBox,
    Command::ZoomPrevious,
    Command::AnnotateOperatingPoint,
    Command::AnnotateTransientVoltages,
    Command::ToggleFullScreen,
    Command::ToggleNavigator,
    Command::ToggleInspector,
    Command::ToggleConsole,
    Command::OpenConsole,
    Command::OpenProblems,
    Command::OpenNotifications,
    Command::ToggleConsoleMaximized,
    Command::ClearConsole,
    Command::ToggleFocusMode,
    Command::NewApplicationWindow,
    Command::DetachDocument,
    Command::MoveDocumentToWindow,
    Command::ReattachDocument,
    Command::ConsolidateWindows,
    Command::MonitorRecovery,
    Command::PreviousDocument,
    Command::NextDocument,
    Command::NextPane,
    Command::PreviousPane,
    Command::CloseOtherDocuments,
    Command::CloseAllDocuments,
    Command::WindowManager,
    Command::MinimizeWindow,
    Command::ZoomWindow,
    Command::ResetLayout,
    Command::MoveSelection,
    Command::StretchSelection,
    Command::ArraySelection,
    Command::SwapToRSpiceSymbols,
    Command::UpdateInstanceInterface,
    Command::HighlightNet,
    Command::ConfigurationSets,
    Command::SymbolPinTool,
    Command::SymbolPolylineTool,
    Command::SymbolRectangleTool,
    Command::SymbolCircleTool,
    Command::SymbolArcTool,
    Command::SymbolPolygonTool,
    Command::SymbolTextTool,
    Command::SymbolRotatePin,
    Command::SymbolMirrorPin,
    Command::SymbolUpdatePinsFromContract,
    Command::SymbolSave,
    Command::EditSymbolInPlace,
    Command::ExitSymbolInPlace,
    Command::RotateSelection,
    Command::RotateSelectionCounterClockwise,
    Command::MirrorSelectionHorizontal,
    Command::MirrorSelectionVertical,
    Command::AlignSelectionLeft,
    Command::AlignSelectionCenter,
    Command::AlignSelectionRight,
    Command::AlignSelectionTop,
    Command::AlignSelectionMiddle,
    Command::AlignSelectionBottom,
    Command::DistributeSelectionHorizontal,
    Command::DistributeSelectionVertical,
    Command::SnapSelectionToGrid,
    Command::SnapToStandardValue,
    Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E6),
    Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E12),
    Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E24),
    Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E48),
    Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E96),
    Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E192),
    Command::JoinCollinearWires,
    Command::AscendHierarchy,
    Command::DescendHierarchy,
    Command::DescendHierarchyDirect,
    Command::CheckCellview,
    Command::CheckDesign,
    Command::ClearChecks,
    Command::NextViolation,
    Command::PreviousViolation,
    Command::CommandPalette,
    Command::KeyboardShortcuts,
    Command::HelpCenter,
    Command::Examples,
    Command::ReleaseNotes,
    Command::SystemDiagnostics,
    Command::SupportBundle,
    Command::LegalPrivacy,
    Command::About,
    Command::Cancel,
];

/// Every command the registry lists, by key, in the order menus list them
/// and the shortcut resolver breaks ties by: the shell's own commands and the
/// ones a feature declares alike.
static COMMAND_ORDER: &[CommandKey] = &[
    ProjectCommand::ProjectLauncher.key(),
    ProjectCommand::OpenProject.key(),
    ProjectCommand::NewProject.key(),
    ProjectCommand::Save.key(),
    ProjectCommand::SaveAs.key(),
    ProjectCommand::SaveAll.key(),
    Command::RevertActiveDocument.key(),
    Command::CloseActiveDocument.key(),
    ProjectCommand::CloseProject.key(),
    ProjectCommand::NewCell.key(),
    ProjectCommand::NewView.key(),
    CodeCommand::NewVerilogADevice.key(),
    ProjectCommand::OpenSchematic.key(),
    CodeCommand::OpenNetlist.key(),
    CodeCommand::ImportNetlist.key(),
    CodeCommand::ImportVerilogA.key(),
    CodeCommand::ImportLtspiceSchematic.key(),
    CodeCommand::ImportQspiceSchematic.key(),
    ResultsCommand::ImportResultDataset.key(),
    CloudCommand::LiveSession.key(),
    CloudCommand::PublishToWeb.key(),
    CodeCommand::ExportNetlist(NetlistFormat::Spectre).key(),
    CodeCommand::ExportNetlist(NetlistFormat::Spice).key(),
    CodeCommand::ExportBillOfMaterials.key(),
    SheetsCommand::SheetProperties.key(),
    SheetsCommand::SheetFormatManager.key(),
    SheetsCommand::SheetDefaults.key(),
    HardcopyCommand::PrintHardcopy.key(),
    HardcopyCommand::ExportActiveView.key(),
    Command::Exit.key(),
    Command::Undo.key(),
    Command::Redo.key(),
    Command::Cut.key(),
    Command::Copy.key(),
    HardcopyCommand::CopyAsImage.key(),
    Command::Paste.key(),
    Command::Duplicate.key(),
    Command::Delete.key(),
    Command::SelectAll.key(),
    Command::DeselectAll.key(),
    Command::SelectOfKind(SchematicObjectKind::Instances).key(),
    Command::SelectOfKind(SchematicObjectKind::Wires).key(),
    Command::SelectOfKind(SchematicObjectKind::NetLabels).key(),
    Command::SelectOfKind(SchematicObjectKind::Pins).key(),
    Command::SelectOfKind(SchematicObjectKind::Text).key(),
    Command::SelectOfKind(SchematicObjectKind::Shapes).key(),
    DesignCommand::RenameSelection.key(),
    Command::ObjectProperties.key(),
    Command::FindInDesign.key(),
    Command::FindReplace.key(),
    Command::RepeatLastCommand.key(),
    SettingsCommand::Preferences.key(),
    Command::OpenWorkspace(Workspace::Project).key(),
    Command::OpenWorkspace(Workspace::Design).key(),
    Command::OpenWorkspace(Workspace::Simulate).key(),
    Command::OpenWorkspace(Workspace::Stimulus).key(),
    Command::OpenWorkspace(Workspace::Results).key(),
    Command::OpenWorkspace(Workspace::Verify).key(),
    Command::OpenWorkspace(Workspace::Models).key(),
    Command::OpenWorkspace(Workspace::Netlist).key(),
    ProjectCommand::Page(ProjectPage::Overview).key(),
    ProjectCommand::Page(ProjectPage::Library).key(),
    ProjectCommand::Page(ProjectPage::Configuration).key(),
    ProjectCommand::Page(ProjectPage::Dependencies).key(),
    ProjectCommand::Page(ProjectPage::Recovery).key(),
    SimulateCommand::Page(SimulationPage::Analyses).key(),
    SimulateCommand::Page(SimulationPage::Excitations).key(),
    SimulateCommand::Page(SimulationPage::Variables).key(),
    SimulateCommand::Page(SimulationPage::Outputs).key(),
    SimulateCommand::Page(SimulationPage::Specifications).key(),
    SimulateCommand::Page(SimulationPage::RunSet).key(),
    SimulateCommand::Page(SimulationPage::Models).key(),
    SimulateCommand::Page(SimulationPage::Solver).key(),
    SimulateCommand::Page(SimulationPage::Save).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Waves).key(),
    SimulateCommand::Verify(VerificationPage::Yield).key(),
    ModelsCommand::Page(ModelsPage::Models).key(),
    ModelsCommand::ModelCreateProjectCopy.key(),
    ModelsCommand::ModelEditor.key(),
    ModelsCommand::ModelCorrelation.key(),
    ModelsCommand::ModelSaveRevision.key(),
    ModelsCommand::ModelValidate.key(),
    ModelsCommand::ModelRunQualificationTests.key(),
    ModelsCommand::ModelCompareRelease.key(),
    Command::ZoomIn.key(),
    Command::ZoomOut.key(),
    Command::ZoomFit.key(),
    DesignCommand::FitSchematicContent.key(),
    Command::ZoomOneToOne.key(),
    Command::ZoomToSelection.key(),
    Command::ZoomBox.key(),
    Command::ZoomPrevious.key(),
    DesignCommand::CycleGrid.key(),
    DesignCommand::GridDots.key(),
    DesignCommand::GridLines.key(),
    DesignCommand::GridOff.key(),
    Command::AnnotateOperatingPoint.key(),
    Command::AnnotateTransientVoltages.key(),
    DesignCommand::GridSnapRouting.key(),
    SheetsCommand::DrawingSheetLayers.key(),
    DesignCommand::VisibilityOptions.key(),
    Command::ToggleFullScreen.key(),
    DesignCommand::EngineeringTableView.key(),
    Command::ToggleNavigator.key(),
    Command::ToggleInspector.key(),
    Command::ToggleConsole.key(),
    ResultsCommand::ToggleResultsSplit.key(),
    Command::OpenConsole.key(),
    Command::OpenProblems.key(),
    Command::OpenNotifications.key(),
    Command::ToggleConsoleMaximized.key(),
    Command::ClearConsole.key(),
    Command::ToggleFocusMode.key(),
    Command::NewApplicationWindow.key(),
    Command::DetachDocument.key(),
    Command::MoveDocumentToWindow.key(),
    Command::ReattachDocument.key(),
    Command::ConsolidateWindows.key(),
    Command::MonitorRecovery.key(),
    Command::PreviousDocument.key(),
    Command::NextDocument.key(),
    Command::NextPane.key(),
    Command::PreviousPane.key(),
    Command::CloseOtherDocuments.key(),
    Command::CloseAllDocuments.key(),
    SettingsCommand::WorkspaceLayouts.key(),
    Command::WindowManager.key(),
    Command::MinimizeWindow.key(),
    Command::ZoomWindow.key(),
    Command::ResetLayout.key(),
    DesignCommand::SelectTool.key(),
    DesignCommand::PlaceInstance.key(),
    DesignCommand::PlaceWire.key(),
    DesignCommand::PlaceBus.key(),
    DesignCommand::PlaceBusTap.key(),
    DesignCommand::PlaceJunction.key(),
    DesignCommand::PlaceLabel.key(),
    DesignCommand::PlaceOffSheetConnector.key(),
    DesignCommand::PlaceProbe.key(),
    DesignCommand::PlacePin.key(),
    DesignCommand::PlaceText.key(),
    DesignCommand::PlaceShape.key(),
    Command::MoveSelection.key(),
    Command::StretchSelection.key(),
    Command::ArraySelection.key(),
    DesignCommand::ReplaceInstance.key(),
    Command::SwapToRSpiceSymbols.key(),
    Command::UpdateInstanceInterface.key(),
    DesignCommand::CreateHierarchy.key(),
    ProjectCommand::CreateSymbol.key(),
    ProjectCommand::DesignManagement.key(),
    ProjectCommand::Variants.key(),
    ProjectCommand::Annotate.key(),
    DesignCommand::SelectionBulkEdit.key(),
    DesignCommand::ConnectivityManager.key(),
    Command::HighlightNet.key(),
    Command::ConfigurationSets.key(),
    DesignCommand::RuleSetup.key(),
    DesignCommand::ReviewComments.key(),
    ProjectCommand::RevisionHistory.key(),
    SheetsCommand::NextSheet.key(),
    SheetsCommand::PreviousSheet.key(),
    SheetsCommand::NewSheet.key(),
    SheetsCommand::DeleteSheet.key(),
    Command::SymbolPinTool.key(),
    Command::SymbolPolylineTool.key(),
    Command::SymbolRectangleTool.key(),
    Command::SymbolCircleTool.key(),
    Command::SymbolArcTool.key(),
    Command::SymbolPolygonTool.key(),
    Command::SymbolTextTool.key(),
    Command::SymbolRotatePin.key(),
    Command::SymbolMirrorPin.key(),
    Command::SymbolUpdatePinsFromContract.key(),
    Command::SymbolSave.key(),
    Command::EditSymbolInPlace.key(),
    Command::ExitSymbolInPlace.key(),
    DesignCommand::PlaceResistor.key(),
    DesignCommand::PlaceCapacitor.key(),
    DesignCommand::PlaceInductor.key(),
    DesignCommand::PlaceDiode.key(),
    DesignCommand::PlaceGround.key(),
    DesignCommand::PlaceVoltageSource.key(),
    DesignCommand::PlaceCurrentSource.key(),
    Command::RotateSelection.key(),
    Command::RotateSelectionCounterClockwise.key(),
    Command::MirrorSelectionHorizontal.key(),
    Command::MirrorSelectionVertical.key(),
    Command::AlignSelectionLeft.key(),
    Command::AlignSelectionCenter.key(),
    Command::AlignSelectionRight.key(),
    Command::AlignSelectionTop.key(),
    Command::AlignSelectionMiddle.key(),
    Command::AlignSelectionBottom.key(),
    Command::DistributeSelectionHorizontal.key(),
    Command::DistributeSelectionVertical.key(),
    Command::SnapSelectionToGrid.key(),
    Command::SnapToStandardValue.key(),
    Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E6).key(),
    Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E12).key(),
    Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E24).key(),
    Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E48).key(),
    Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E96).key(),
    Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E192).key(),
    Command::JoinCollinearWires.key(),
    Command::AscendHierarchy.key(),
    Command::DescendHierarchy.key(),
    Command::DescendHierarchyDirect.key(),
    CodeCommand::ShowInNetlist.key(),
    Command::CheckCellview.key(),
    Command::CheckDesign.key(),
    ProjectCommand::CheckAndSave.key(),
    Command::ClearChecks.key(),
    Command::NextViolation.key(),
    Command::PreviousViolation.key(),
    SimulateCommand::Run.key(),
    SimulateCommand::Stop.key(),
    SimulateCommand::JobsManager.key(),
    SimulateCommand::Preflight.key(),
    SimulateCommand::ManagePlans.key(),
    SimulateCommand::AddAnalysis.key(),
    SimulateCommand::Options.key(),
    CodeCommand::GenerateNetlist.key(),
    CodeCommand::FindCodeDocument.key(),
    CodeCommand::ValidateCodeDocument.key(),
    CodeCommand::ManageSourceDocument.key(),
    CodeCommand::ToggleLineComment.key(),
    CodeCommand::IndentLines.key(),
    CodeCommand::OutdentLines.key(),
    CodeCommand::DuplicateLine.key(),
    CodeCommand::DeleteLine.key(),
    CodeCommand::MoveLineUp.key(),
    CodeCommand::MoveLineDown.key(),
    CodeCommand::GoToMatchingBracket.key(),
    CodeCommand::GoToLine.key(),
    CodeCommand::NextProblem.key(),
    CodeCommand::PreviousProblem.key(),
    CodeCommand::ShowProblemAtCaret.key(),
    CodeCommand::FormatDocument.key(),
    CodeCommand::FoldSelection.key(),
    CodeCommand::UnfoldAtCaret.key(),
    CodeCommand::FoldAll.key(),
    CodeCommand::UnfoldAll.key(),
    CodeCommand::GoToDefinition.key(),
    CodeCommand::FindReferences.key(),
    CodeCommand::RenameSymbol.key(),
    CodeCommand::CompareGeneratedRevisions.key(),
    ResultsCommand::ExportWaveformsCsv.key(),
    ResultsCommand::ClearResults.key(),
    ResultsCommand::ToggleLinkedCursors.key(),
    ResultsCommand::DatasetManifestBrowser.key(),
    ResultsCommand::OpenRunInResults.key(),
    ResultsCommand::OpenProducingPlan.key(),
    ResultsCommand::OpenTaskDeck.key(),
    ResultsCommand::RevealProducerLog.key(),
    ResultsCommand::CreateResultDocument.key(),
    ResultsCommand::WaveformCalculator.key(),
    ResultsCommand::ExpressionDiagnostics.key(),
    ResultsCommand::CompareResultDatasets.key(),
    ResultsCommand::MeasurementLibrary.key(),
    ResultsCommand::FamilySlicing.key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Bode).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::DcSweep).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Fft).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::HarmonicBalance).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::PhaseNoise).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Eye).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Hist).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Op).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::NoiseContrib).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Contribution).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::TransferFunction).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Specs).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Table).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Nyquist).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Smith).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Polar).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::NetworkMatrix).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::PoleZero).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Scatter).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::BoxViolin).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Events).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Soa).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Optimization).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Manifest).key(),
    ResultsCommand::EditSpecifications.key(),
    SimulateCommand::Verify(VerificationPage::Corners).key(),
    SimulateCommand::Verify(VerificationPage::Tuning).key(),
    SimulateCommand::Verify(VerificationPage::Optimization).key(),
    SimulateCommand::Verify(VerificationPage::DeviceSafety).key(),
    SimulateCommand::Verify(VerificationPage::Regression).key(),
    ModelsCommand::Page(ModelsPage::Symbols).key(),
    ModelsCommand::Page(ModelsPage::Corners).key(),
    ModelsCommand::Page(ModelsPage::Bins).key(),
    ModelsCommand::Page(ModelsPage::Include).key(),
    ModelsCommand::Page(ModelsPage::Qualification).key(),
    ModelsCommand::ModelBrowser.key(),
    ModelsCommand::PdkSettings.key(),
    ModelsCommand::RescanModelLibraries.key(),
    DesignCommand::NewStimulusDefinition.key(),
    DesignCommand::DuplicateStimulusDefinition.key(),
    DesignCommand::DeleteStimulusDefinition.key(),
    DesignCommand::ApplyStimulusDraft.key(),
    DesignCommand::RevertStimulusDraft.key(),
    DesignCommand::ValidateStimulusLibrary.key(),
    DesignCommand::PlaceSourceFromDefinition.key(),
    DesignCommand::ShowAdopterOnSchematic.key(),
    CodeCommand::CompileVerilogA.key(),
    CodeCommand::CancelVerilogACompile.key(),
    CodeCommand::VerilogABuildSettings.key(),
    CodeCommand::VerilogAOpenSymbol.key(),
    CodeCommand::AutomationConsole.key(),
    Command::CommandPalette.key(),
    Command::KeyboardShortcuts.key(),
    CloudCommand::AccountOrganization.key(),
    CloudCommand::License.key(),
    CloudCommand::CheckForUpdates.key(),
    HardcopyCommand::ReportAuthoring.key(),
    ResultsCommand::AddVisualizationPane.key(),
    ResultsCommand::VisualizationDocumentProperties.key(),
    Command::HelpCenter.key(),
    Command::Examples.key(),
    Command::ReleaseNotes.key(),
    Command::SystemDiagnostics.key(),
    Command::SupportBundle.key(),
    Command::LegalPrivacy.key(),
    Command::About.key(),
    Command::Cancel.key(),
];

/// Ids older builds stored for commands that now answer to another, each with
/// the key of the command it names now. Shortcut profiles those builds wrote
/// keep their bindings; nothing else about resolution is fuzzy.
const LEGACY_ALIASES: &[(&str, CommandKey)] = &[
    // `model-metadata-audit` shipped as the portable identity for this tab
    // before the mockup-defined Qualification workspace replaced the
    // temporary audit.
    (
        "model-metadata-audit",
        ModelsCommand::Page(ModelsPage::Qualification).key(),
    ),
    ("fit-canvas", Command::ZoomFit.key()),
    ("export-waveforms", ResultsCommand::ExportWaveformsCsv.key()),
    // The cursor manager's slots became the linked-cursors toggle, so a
    // shortcut bound to it keeps doing what the slots now do.
    ("cursor-manager", ResultsCommand::ToggleLinkedCursors.key()),
    (
        "visualization-cursor-manager",
        ResultsCommand::ToggleLinkedCursors.key(),
    ),
    (
        "visualization-document-properties",
        ResultsCommand::VisualizationDocumentProperties.key(),
    ),
    // The one check command became two; a shortcut bound to it keeps
    // checking the cellview in front of the reader.
    ("run-checks", Command::CheckCellview.key()),
];

/// The registry of every command in [`COMMAND_ORDER`]: the shell's own in
/// [`COMMAND_REGISTRY`] and the ones the features declare. Built on first use
/// and checked as it is built.
pub(crate) fn command_registry() -> &'static Registry {
    static REGISTRY: LazyLock<Registry> = LazyLock::new(|| {
        let registry = Registry::new(
            COMMAND_ORDER,
            COMMAND_REGISTRY
                .iter()
                .map(|command| command.registry_spec())
                .chain(SheetsCommand::ALL.map(SheetsCommand::declared_spec))
                .chain(SettingsCommand::ALL.map(SettingsCommand::declared_spec))
                .chain(HardcopyCommand::LISTED.map(HardcopyCommand::declared_spec))
                .chain(ResultsCommand::ALL.map(ResultsCommand::declared_spec))
                .chain(CloudCommand::ALL.map(CloudCommand::declared_spec))
                .chain(ModelsCommand::ALL.map(ModelsCommand::declared_spec))
                .chain(SimulateCommand::ALL.map(SimulateCommand::declared_spec))
                .chain(ProjectCommand::ALL.map(ProjectCommand::declared_spec))
                .chain(CodeCommand::ALL.map(CodeCommand::declared_spec))
                .chain(DesignCommand::ALL.map(DesignCommand::declared_spec))
                .collect(),
            LEGACY_ALIASES,
        )
        .with_unlisted(
            HardcopyCommand::REPORT_TOOLBAR
                .map(HardcopyCommand::declared_spec)
                .into_iter()
                .chain(SimulateCommand::UNLISTED.map(SimulateCommand::declared_spec))
                .collect(),
        );
        debug_assert_eq!(registry.validate(command_registry::ids::named()), Ok(()));
        registry
    });
    &REGISTRY
}

/// The command Repeat last command would run, from the id the dispatcher
/// last recorded: `None` before any command has run, and for an id no
/// registered command answers to.
pub(crate) fn last_repeatable(
    recorded: Option<&str>,
) -> Option<&'static command_registry::CommandSpec> {
    let registry = command_registry();
    registry.spec(registry.resolve(recorded?)?)
}

/// The command a registered key names. Every registered command is still
/// declared by this enum, and this table is how a key reaches it.
pub(crate) fn legacy_command(key: CommandKey) -> Option<Command> {
    static BY_KEY: LazyLock<HashMap<CommandKey, Command>> = LazyLock::new(|| {
        let mut commands = HashMap::with_capacity(COMMAND_REGISTRY.len());
        for command in COMMAND_REGISTRY {
            commands.entry(command.key()).or_insert(*command);
        }
        commands
    });
    BY_KEY.get(&key).copied()
}

/// The command registry as the catalog a shortcut profile reads command facts
/// through: the application's [`command_registry`], read as the catalog it
/// is.
#[derive(Debug, Clone, Copy)]
pub struct CommandRegistry;

impl CommandCatalog for CommandRegistry {
    fn commands(&self) -> &[CommandKey] {
        command_registry().commands()
    }

    fn resolve(&self, id: &str) -> Option<CommandKey> {
        CommandCatalog::resolve(command_registry(), id)
    }

    fn context(&self, command: CommandKey) -> Option<ShortcutContext> {
        command_registry().context(command)
    }

    fn default_bindings(&self, command: CommandKey) -> &'static [ShortcutBinding] {
        command_registry().default_bindings(command)
    }

    fn primary_is_reserved_on(&self, command: CommandKey, platform: CommandPlatform) -> bool {
        command_registry().primary_is_reserved_on(command, platform)
    }
}

/// Searchable projection of the command registry, as values: the commands
/// the palette lists. The command palette itself and context-only
/// cancellation are intentionally not self-referential rows.
#[cfg(test)]
pub fn command_catalog() -> impl Iterator<Item = Command> {
    let registry = command_registry();
    registry
        .order()
        .iter()
        .filter(move |key| {
            registry
                .spec(**key)
                .is_some_and(|spec| spec.flags.contains(CommandFlags::PALETTE_VISIBLE))
        })
        .filter_map(|key| legacy_command(*key))
}
