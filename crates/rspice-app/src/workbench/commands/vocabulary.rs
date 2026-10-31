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

use crate::state::command_catalog::{
    CommandCatalog, CommandKey, CommandPlatform, ShortcutBinding, ShortcutContext,
};
use crate::state::{ComponentType, ModelsPage};
use crate::workbench::cloud::commands::CloudCommand;
use crate::workbench::framework::command_registry::{self, CommandFlags, Registry};
use crate::workbench::hardcopy::commands::HardcopyCommand;
use crate::workbench::models::commands::ModelsCommand;
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::results::commands::ResultsCommand;
use crate::workbench::settings::commands::SettingsCommand;
use crate::workbench::sheets::commands::SheetsCommand;
use crate::workbench::state::{SimulationPage, VerificationPage, Workspace};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// `CommandPalette` is the feature's name, the same one the shortcut
// preferences show the user. Trimming it to `Palette` to stop repeating the
// enum would rename a thing that exists.
#[allow(clippy::enum_variant_names)]
pub enum Command {
    OpenWorkspace(Workspace),
    ProjectLauncher,
    RecentProjects,
    NewProject,
    OpenProject,
    Save,
    SaveAs,
    SaveAll,
    RevertActiveDocument,
    CloseActiveDocument,
    CloseProject,
    NewCell,
    /// A cell view of another kind beside the views the selected cell has.
    NewView,
    /// A cell whose Verilog-A module the reader writes next, with the symbol
    /// its ports generate.
    NewVerilogADevice,
    OpenDocument,
    OpenNetlist,
    ImportNetlist,
    ImportVerilogA,
    ExportNetlist(crate::io::NetlistFormat),
    Exit,
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    Duplicate,
    Delete,
    SelectAll,
    RenameSelection,
    ObjectProperties,
    FindInDesign,
    FindReplace,
    /// Run the last placement or editing command again, as it last ran.
    RepeatLastCommand,
    ZoomIn,
    ZoomOut,
    ZoomFit,
    FitSchematicContent,
    ZoomOneToOne,
    ZoomToSelection,
    ZoomBox,
    CycleGrid,
    GridSnapRouting,
    VisibilityOptions,
    ToggleFullScreen,
    ResetActiveView,
    EngineeringTableView,
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
    CloseOtherDocuments,
    CloseAllDocuments,
    WindowManager,
    SelectTool,
    PlaceInstance,
    PlaceWire,
    PlaceBus,
    PlaceBusTap,
    PlaceJunction,
    PlaceLabel,
    PlaceOffSheetConnector,
    PlaceProbe,
    PlacePin,
    PlaceText,
    PlaceShape,
    MoveSelection,
    StretchSelection,
    ArraySelection,
    ReplaceInstance,
    /// Re-place a stale instance on its own master: same cell, the interface
    /// the master presents now. It is a degenerate replacement, which is why
    /// it sits beside [`Self::ReplaceInstance`] rather than in a repair
    /// vocabulary of its own.
    UpdateInstanceInterface,
    CreateHierarchy,
    ConnectivityManager,
    DesignManagement,
    SelectionBulkEdit,
    ConfigurationSets,
    /// The project's design-rule deck: the severity each rule is reported
    /// at in this project.
    RuleSetup,
    ReviewComments,
    RevisionHistory,
    DesignSpecialistWorkspaces,
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
    Place(ComponentType),
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
    /// Lay the active sheet's wires that run on one line, end to end or
    /// overlapping, as one wire each.
    JoinCollinearWires,
    Cancel,
    AscendHierarchy,
    DescendHierarchy,
    /// Direct edit-in-place hierarchy gesture used by the toolbar and
    /// Shift+E. The Design-menu command above owns the explicit transaction.
    DescendHierarchyDirect,
    /// Sole owner of "locate this instance in the generated deck". The
    /// canvas context menu, the instance inspector and the design navigator
    /// all route here rather than each moving the workspace themselves.
    ShowInNetlist,
    /// The open cellview's own findings: what an author runs while drawing.
    /// A symbol view checks its pins against the cell's interface.
    CheckCellview,
    /// The whole design, walked from the simulation root under its
    /// configuration, with every cellview of the project's own libraries the
    /// walk never reaches.
    CheckDesign,
    CheckAndSave,
    ClearChecks,
    NextViolation,
    PreviousViolation,
    RunSimulation,
    StopSimulation,
    JobsManager,
    PreflightChecks,
    /// Global route into the versioned Simulation Plan Manager. The toolbar's
    /// plan chip, the palette and the shortcut map all reach the manager
    /// through this one command rather than constructing its draft themselves.
    ManageSimulationPlans,
    /// Open the analysis catalogue over whatever workspace the reader is on.
    ///
    /// The navigator's creating action, the empty analysis rail and the
    /// Simulate menu all arm the catalogue through this one command. They used
    /// to set the four palette fields themselves, which is how the navigator
    /// came to arm a modal on eight routes that had nothing drawing it.
    AddAnalysis,
    SimulationOptions,
    GenerateNetlist,
    FindCodeDocument,
    ValidateCodeDocument,
    ManageSourceDocument,
    SourceLanguageTools,
    /// The source editor's line, navigation and folding commands. Each acts
    /// on the source editor the page on screen shows as its document, and
    /// only there.
    ToggleLineComment,
    IndentLines,
    OutdentLines,
    DuplicateLine,
    DeleteLine,
    MoveLineUp,
    MoveLineDown,
    GoToMatchingBracket,
    GoToLine,
    FormatDocument,
    FoldSelection,
    UnfoldAtCaret,
    FoldAll,
    UnfoldAll,
    /// Language navigation from the caret of the active source editor, for
    /// languages whose language service answers it.
    GoToDefinition,
    FindReferences,
    RenameSymbol,
    CompareGeneratedRevisions,
    VerificationPage(VerificationPage),
    ProjectPage(ProjectPage),
    SimulationPage(SimulationPage),
    StimulusNewDefinition,
    StimulusDuplicateDefinition,
    StimulusDeleteDefinition,
    StimulusApplyDraft,
    StimulusRevertDraft,
    StimulusValidateLibrary,
    StimulusPlaceDefinition,
    StimulusShowAdopter,
    CompileVerilogA,
    VerilogABuildSettings,
    AutomationConsole,
    CommandPalette,
    KeyboardShortcuts,
    SpecialistToolBrowser,
    HelpCenter,
    ReleaseNotes,
    MigrationGuide,
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
            Self::ProjectLauncher => spec("project-launcher", "Project launcher…", "File"),
            Self::RecentProjects => spec("recent-projects", "Recent projects…", "File"),
            Self::NewProject => spec("new-project", "New project…", "File"),
            Self::OpenProject => spec("open-project", "Open project…", "File"),
            Self::Save => spec("save-project", "Save", "File"),
            Self::SaveAs => spec("save-project-as", "Save as…", "File"),
            Self::SaveAll => spec("save-all", "Save all", "File"),
            Self::RevertActiveDocument => {
                spec("revert-document", "Revert active document…", "File")
            }
            Self::CloseActiveDocument => spec("close-document", "Close active document", "File"),
            Self::CloseProject => spec("close-project", "Close project…", "File"),
            Self::NewCell => spec("new-cell", "New cell…", "File"),
            Self::NewView => spec("new-view", "New view…", "File"),
            Self::NewVerilogADevice => spec("new-veriloga-device", "New Verilog-A device…", "File"),
            Self::OpenDocument => spec("open-schematic", "Open schematic…", "File"),
            Self::OpenNetlist => spec("open-netlist", "Open netlist…", "File"),
            Self::ImportNetlist => spec("import-netlist", "Import SPICE deck…", "File"),
            Self::ImportVerilogA => spec("import-veriloga", "Import Verilog-A…", "File"),
            Self::ExportNetlist(crate::io::NetlistFormat::Spectre) => {
                spec("export-netlist-spectre", "Export Spectre netlist…", "File")
            }
            Self::ExportNetlist(crate::io::NetlistFormat::Spice) => {
                spec("export-netlist-spice", "Export SPICE netlist…", "File")
            }
            Self::ExportNetlist(crate::io::NetlistFormat::Hspice) => {
                spec("export-netlist-hspice", "Export HSPICE netlist…", "File")
            }
            Self::ExportNetlist(crate::io::NetlistFormat::Xyce) => {
                spec("export-netlist-xyce", "Export Xyce netlist…", "File")
            }
            Self::Exit => spec("exit-rspice", "Exit RSpice…", "File"),
            Self::Undo => spec("undo", "Undo", "Edit"),
            Self::Redo => spec("redo", "Redo", "Edit"),
            Self::Cut => spec("cut-selection", "Cut selection", "Edit"),
            Self::Copy => spec("copy-selection", "Copy selection", "Edit"),
            Self::Paste => spec("paste-selection", "Paste", "Edit"),
            Self::Duplicate => spec("duplicate-selection", "Duplicate selection", "Edit"),
            Self::Delete => spec("delete-selection", "Delete selection", "Edit"),
            Self::SelectAll => spec("select-all", "Select all", "Edit"),
            Self::RenameSelection => spec("rename-selection", "Rename selected object…", "Edit"),
            Self::ObjectProperties => spec("object-properties", "Object properties…", "Edit"),
            Self::FindInDesign => spec("find-design", "Find in design…", "Edit"),
            Self::FindReplace => spec("find-replace", "Find and replace…", "Edit"),
            Self::RepeatLastCommand => spec("repeat-last-command", "Repeat last command", "Edit"),
            Self::ZoomIn => spec("zoom-in", "Zoom in", "View"),
            Self::ZoomOut => spec("zoom-out", "Zoom out", "View"),
            Self::ZoomFit => spec("canvas-fit-sheet", "Fit drawing sheet", "View"),
            Self::FitSchematicContent => {
                spec("canvas-fit-content", "Fit schematic content", "View")
            }
            Self::ZoomOneToOne => spec("zoom-one-to-one", "Zoom 100%", "View"),
            Self::ZoomToSelection => spec("zoom-to-selection", "Zoom to selection", "View"),
            Self::ZoomBox => spec("zoom-box", "Zoom box", "View"),
            Self::CycleGrid => spec("cycle-grid", "Cycle grid display", "View"),
            Self::GridSnapRouting => spec(
                "grid-snap-routing",
                "Grid, snap and wire routing\u{2026}",
                "View",
            ),
            Self::VisibilityOptions => {
                spec("visibility-options", "Schematic visibility\u{2026}", "View")
            }
            Self::ToggleFullScreen => spec("full-screen", "Enter full screen", "View"),
            Self::ResetActiveView => spec("reset-active-view", "Reset active view", "View"),
            Self::EngineeringTableView => spec(
                "engineering-table-manager",
                "Engineering table view\u{2026}",
                "View",
            ),
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
            Self::ResetLayout => spec(
                "reset-workspace-layout",
                "Reset workspace layout…",
                "Window",
            ),
            Self::NewApplicationWindow => spec(
                "new-application-window",
                "New application window\u{2026}",
                "Window",
            ),
            Self::DetachDocument => spec(
                "detach-document",
                "Detach active document\u{2026}",
                "Window",
            ),
            Self::MoveDocumentToWindow => spec(
                "move-document-window",
                "Move active document to window\u{2026}",
                "Window",
            ),
            Self::ReattachDocument => {
                spec("reattach-document", "Reattach active document", "Window")
            }
            Self::ConsolidateWindows => spec(
                "consolidate-windows",
                "Consolidate all windows\u{2026}",
                "Window",
            ),
            Self::MonitorRecovery => spec(
                "monitor-recovery",
                "Recover off-screen windows\u{2026}",
                "Window",
            ),
            Self::PreviousDocument => spec("previous-document", "Previous document", "Window"),
            Self::NextDocument => spec("next-document", "Next document", "Window"),
            Self::CloseOtherDocuments => {
                spec("close-other-documents", "Close other documents", "Window")
            }
            Self::CloseAllDocuments => spec("close-all-documents", "Close all documents", "Window"),
            Self::WindowManager => spec(
                "window-manager",
                "Windows, documents and session\u{2026}",
                "Window",
            ),
            Self::SelectTool => spec("select-tool", "Select tool", "Design"),
            Self::PlaceInstance => spec("place-instance", "Place instance…", "Design"),
            Self::PlaceWire => spec("place-wire", "Draw wire", "Design"),
            Self::PlaceBus => spec("place-bus", "Draw bus", "Design"),
            Self::PlaceBusTap => spec("place-bus-tap", "Place bus tap", "Design"),
            Self::PlaceJunction => spec("place-junction", "Place junction", "Design"),
            Self::PlaceLabel => spec("place-label", "Place net label", "Design"),
            Self::PlaceOffSheetConnector => {
                spec("place-off-sheet-connector", "Off-sheet connector", "Design")
            }
            Self::PlaceProbe => spec("place-probe", "Place probe", "Design"),
            Self::PlacePin => spec("place-pin", "Create pins\u{2026}", "Design"),
            Self::PlaceText => spec("place-text", "Place text or note\u{2026}", "Design"),
            Self::PlaceShape => spec("place-shape", "Draw documentation shape\u{2026}", "Design"),
            Self::MoveSelection => spec("move-selection", "Move selection", "Design"),
            Self::StretchSelection => spec("stretch-selection", "Stretch selection", "Design"),
            Self::ArraySelection => spec("array-selection", "Create array\u{2026}", "Design"),
            Self::ReplaceInstance => spec("replace-instance", "Replace instance\u{2026}", "Design"),
            Self::UpdateInstanceInterface => spec(
                "update-instance-interface",
                "Update instance interface",
                "Design",
            ),
            Self::CreateHierarchy => spec(
                "create-hierarchy",
                "Create hierarchy from selection\u{2026}",
                "Design",
            ),
            Self::ConnectivityManager => spec(
                "design-connectivity-tools",
                "Connectivity and bus manager\u{2026}",
                "Design",
            ),
            Self::DesignManagement => spec(
                "design-management",
                "Sheets, variants and annotation\u{2026}",
                "Design",
            ),
            Self::SelectionBulkEdit => spec(
                "design-bulk-tools",
                "Selection and bulk editing\u{2026}",
                "Design",
            ),
            Self::ConfigurationSets => spec("configuration-sets", "Configurations", "Design"),
            Self::RuleSetup => spec("design-rule-setup", "Design rule setup\u{2026}", "Design"),
            Self::ReviewComments => spec("review-comments", "Review comments\u{2026}", "Design"),
            Self::RevisionHistory => spec("revision-history", "Revision history\u{2026}", "Design"),
            Self::DesignSpecialistWorkspaces => spec(
                "specialist-tools-design",
                "Specialist workspaces\u{2026}",
                "Design",
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
            Self::Place(ComponentType::Resistor) => {
                spec("place-resistor", "Place resistor", "Design")
            }
            Self::Place(ComponentType::Capacitor) => {
                spec("place-capacitor", "Place capacitor", "Design")
            }
            Self::Place(ComponentType::Inductor) => {
                spec("place-inductor", "Place inductor", "Design")
            }
            Self::Place(ComponentType::Diode) => spec("place-diode", "Place diode", "Design"),
            Self::Place(ComponentType::Ground) => spec("place-ground", "Place ground", "Design"),
            Self::Place(ComponentType::VoltageSource) => {
                spec("place-voltage-source", "Place voltage source", "Design")
            }
            Self::Place(ComponentType::CurrentSource) => {
                spec("place-current-source", "Place current source", "Design")
            }
            Self::Place(ComponentType::Port) => Self::PlacePin.spec(),
            Self::Place(_) => spec("place-component", "Place component", "Design"),
            Self::RotateSelection => spec("rotate-selection", "Rotate clockwise", "Design"),
            Self::RotateSelectionCounterClockwise => {
                spec("rotate-selection-ccw", "Rotate counter-clockwise", "Design")
            }
            Self::MirrorSelectionHorizontal => spec(
                "mirror-selection-horizontal",
                "Mirror about vertical axis",
                "Design",
            ),
            Self::MirrorSelectionVertical => {
                spec("mirror-selection-vertical", "Mirror vertically", "Design")
            }
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
            Self::JoinCollinearWires => {
                spec("join-collinear-wires", "Join collinear wires", "Design")
            }
            Self::Cancel => spec("cancel-active-command", "Cancel active command", "Design"),
            Self::AscendHierarchy => spec("ascend-hierarchy", "Ascend hierarchy", "Design"),
            // Two gestures, two rows: this one opens the descend dialog, and
            // the direct gesture below goes straight to the master. Sharing one
            // title made them indistinguishable everywhere the palette, the
            // shortcut editor and the command reference list them side by side.
            Self::DescendHierarchy => spec(
                "descend-hierarchy",
                "Descend into selected instance…",
                "Design",
            ),
            Self::DescendHierarchyDirect => spec(
                "descend-selected-instance",
                "Open selected instance master",
                "Design",
            ),
            Self::ShowInNetlist => spec("show-in-netlist", "Show in netlist", "Design"),
            Self::CheckCellview => spec("check-cellview", "Check cellview", "Design"),
            Self::CheckDesign => spec("check-design", "Check design", "Design"),
            Self::CheckAndSave => spec("check-and-save", "Check and save", "Design"),
            Self::ClearChecks => spec("clear-checks", "Clear check results", "Design"),
            Self::NextViolation => spec("next-violation", "Next violation", "Verify"),
            Self::PreviousViolation => spec("previous-violation", "Previous violation", "Verify"),
            Self::RunSimulation => spec("start-run", "Run active plan", "Simulate"),
            Self::StopSimulation => spec("stop-run", "Stop active run", "Simulate"),
            Self::JobsManager => spec("jobs-manager", "Run history…", "Simulate"),
            Self::PreflightChecks => spec("check", "Preflight checks", "Simulate"),
            Self::ManageSimulationPlans => spec(
                "manage-simulation-plans",
                "Manage simulation plans\u{2026}",
                "Simulate",
            ),
            Self::AddAnalysis => spec(
                "add-analysis",
                "Add analysis or workflow\u{2026}",
                "Simulate",
            ),
            Self::SimulationOptions => spec("solver", "Global solver & convergence", "Simulate"),
            Self::GenerateNetlist => {
                spec("generated-netlist", "Open generated netlist", "Simulate")
            }
            Self::FindCodeDocument => spec("find-code", "Find in source…", "Code"),
            Self::ValidateCodeDocument => {
                spec("validate-code", "Validate active Code document", "Code")
            }
            Self::ManageSourceDocument => spec(
                "manage-source-document",
                "Manage source document\u{2026}",
                "Code",
            ),
            Self::SourceLanguageTools => spec(
                "source-language-tools",
                "Source navigation and language tools\u{2026}",
                "Code",
            ),
            Self::ToggleLineComment => spec("toggle-line-comment", "Toggle line comment", "Edit"),
            Self::IndentLines => spec("indent-lines", "Indent lines", "Edit"),
            Self::OutdentLines => spec("outdent-lines", "Outdent lines", "Edit"),
            Self::DuplicateLine => spec("duplicate-line", "Duplicate line", "Edit"),
            Self::DeleteLine => spec("delete-line", "Delete line", "Edit"),
            Self::MoveLineUp => spec("move-line-up", "Move line up", "Edit"),
            Self::MoveLineDown => spec("move-line-down", "Move line down", "Edit"),
            Self::GoToMatchingBracket => {
                spec("go-to-matching-bracket", "Go to matching bracket", "Edit")
            }
            Self::GoToLine => spec("go-to-line", "Go to line\u{2026}", "Edit"),
            Self::FormatDocument => spec("format-document", "Format document", "Edit"),
            Self::FoldSelection => spec("fold-selection", "Fold selection", "View"),
            Self::UnfoldAtCaret => spec("unfold-at-caret", "Unfold at caret", "View"),
            Self::FoldAll => spec("fold-all", "Fold all", "View"),
            Self::UnfoldAll => spec("unfold-all", "Unfold all", "View"),
            Self::GoToDefinition => spec("go-to-definition", "Go to definition", "Code"),
            Self::FindReferences => spec("find-references", "Find references", "Code"),
            Self::RenameSymbol => spec("rename-symbol", "Rename symbol\u{2026}", "Code"),
            Self::CompareGeneratedRevisions => spec(
                "compare-generated-revisions",
                "Compare generated revisions",
                "Code",
            ),
            Self::VerificationPage(VerificationPage::Yield) => {
                spec("yield", "Verification cockpit", "Verify")
            }
            Self::VerificationPage(VerificationPage::Corners) => {
                spec("corners", "Corner matrix", "Verify")
            }
            Self::VerificationPage(VerificationPage::Tuning) => {
                spec("tuning", "Parameter tuning sandbox", "Verify")
            }
            Self::VerificationPage(VerificationPage::Optimization) => {
                spec("optimization", "Optimization", "Verify")
            }
            Self::VerificationPage(VerificationPage::DeviceSafety) => {
                spec("device-safety", "Safe operating area", "Verify")
            }
            Self::VerificationPage(VerificationPage::Regression) => {
                spec("regression", "Regression plan", "Verify")
            }
            Self::ProjectPage(ProjectPage::Overview) => {
                spec("project-overview", "Open project overview", "Project")
            }
            Self::ProjectPage(ProjectPage::Library) => {
                spec("project-library", "Open project library", "Project")
            }
            Self::ProjectPage(ProjectPage::Configuration) => spec(
                "project-testbench-configuration",
                "Open testbench configuration",
                "Project",
            ),
            Self::ProjectPage(ProjectPage::Dependencies) => spec(
                "project-dependencies",
                "Open dependency manifest",
                "Project",
            ),
            Self::ProjectPage(ProjectPage::Recovery) => {
                spec("project-recovery", "Open recovery center", "Project")
            }
            // Every setup route is addressable, the way the Project, Verify and
            // Models pages are. The Solver route keeps its own dedicated
            // command as well, because that one is also a menu action.
            Self::SimulationPage(SimulationPage::Analyses) => {
                spec("simulation-analyses", "Analyses", "Simulate")
            }
            Self::SimulationPage(SimulationPage::Excitations) => {
                spec("simulation-excitations", "Excitations", "Simulate")
            }
            Self::SimulationPage(SimulationPage::Variables) => {
                spec("simulation-variables", "Design variables", "Simulate")
            }
            Self::SimulationPage(SimulationPage::Outputs) => {
                spec("simulation-outputs", "Outputs & expressions", "Simulate")
            }
            Self::SimulationPage(SimulationPage::Specifications) => spec(
                "simulation-specifications",
                "Requirements & specifications",
                "Simulate",
            ),
            Self::SimulationPage(SimulationPage::RunSet) => {
                spec("simulation-run-set", "PVT, sweeps & variation", "Simulate")
            }
            Self::SimulationPage(SimulationPage::Models) => {
                spec("simulation-models", "Models & sections", "Simulate")
            }
            Self::SimulationPage(SimulationPage::Solver) => {
                spec("simulation-solver", "Solver & convergence", "Simulate")
            }
            Self::SimulationPage(SimulationPage::Save) => spec(
                "simulation-save-policy",
                "Save, streaming & retention policy",
                "Simulate",
            ),
            Self::StimulusNewDefinition => {
                spec("stimulus-new", "New stimulus definition", "Stimulus")
            }
            Self::StimulusDuplicateDefinition => spec(
                "stimulus-duplicate",
                "Duplicate stimulus definition",
                "Stimulus",
            ),
            Self::StimulusDeleteDefinition => {
                spec("stimulus-delete", "Delete stimulus definition", "Stimulus")
            }
            Self::StimulusApplyDraft => spec("stimulus-apply", "Apply stimulus draft", "Stimulus"),
            Self::StimulusRevertDraft => {
                spec("stimulus-revert", "Revert stimulus draft", "Stimulus")
            }
            Self::StimulusValidateLibrary => {
                spec("stimulus-validate", "Validate stimulus library", "Stimulus")
            }
            Self::StimulusPlaceDefinition => {
                spec("stimulus-place", "Place source from definition", "Stimulus")
            }
            Self::StimulusShowAdopter => spec(
                "stimulus-show-adopter",
                "Show adopter on schematic",
                "Stimulus",
            ),
            Self::CompileVerilogA => spec("veriloga", "Compile Verilog-A", "Models"),
            Self::VerilogABuildSettings => {
                spec("veriloga-build-settings", "Build settings", "Models")
            }
            Self::AutomationConsole => spec("automation", "Automation workspace", "Automation"),
            Self::CommandPalette => spec("command-palette", "Command palette", "Navigate"),
            Self::KeyboardShortcuts => spec("command-reference", "Command reference", "Help"),
            Self::SpecialistToolBrowser => spec(
                "specialist-tool-browser",
                "Specialist tool browser…",
                "Navigate",
            ),
            Self::HelpCenter => spec("help-center", "RSpice Help", "Help"),
            Self::ReleaseNotes => spec(
                "release-notes",
                "Release notes and compatibility changes\u{2026}",
                "Help",
            ),
            Self::MigrationGuide => {
                spec("migration-guide", "Project migration guide\u{2026}", "Help")
            }
            Self::SystemDiagnostics => spec(
                "system-diagnostics",
                "System diagnostics and log locations\u{2026}",
                "Help",
            ),
            Self::SupportBundle => spec("support-bundle", "Create support bundle\u{2026}", "Help"),
            Self::LegalPrivacy => spec(
                "legal-privacy-center",
                "Legal, privacy and notices\u{2026}",
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
    Command::ProjectLauncher,
    Command::RecentProjects,
    Command::OpenProject,
    Command::NewProject,
    Command::Save,
    Command::SaveAs,
    Command::SaveAll,
    Command::RevertActiveDocument,
    Command::CloseActiveDocument,
    Command::CloseProject,
    Command::NewCell,
    Command::NewView,
    Command::NewVerilogADevice,
    Command::OpenDocument,
    Command::OpenNetlist,
    Command::ImportNetlist,
    Command::ImportVerilogA,
    Command::ExportNetlist(crate::io::NetlistFormat::Spectre),
    Command::ExportNetlist(crate::io::NetlistFormat::Spice),
    Command::ExportNetlist(crate::io::NetlistFormat::Hspice),
    Command::ExportNetlist(crate::io::NetlistFormat::Xyce),
    Command::Exit,
    Command::Undo,
    Command::Redo,
    Command::Cut,
    Command::Copy,
    Command::Paste,
    Command::Duplicate,
    Command::Delete,
    Command::SelectAll,
    Command::RenameSelection,
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
    Command::ProjectPage(ProjectPage::Overview),
    Command::ProjectPage(ProjectPage::Library),
    Command::ProjectPage(ProjectPage::Configuration),
    Command::ProjectPage(ProjectPage::Dependencies),
    Command::ProjectPage(ProjectPage::Recovery),
    Command::SimulationPage(SimulationPage::Analyses),
    Command::SimulationPage(SimulationPage::Excitations),
    Command::SimulationPage(SimulationPage::Variables),
    Command::SimulationPage(SimulationPage::Outputs),
    Command::SimulationPage(SimulationPage::Specifications),
    Command::SimulationPage(SimulationPage::RunSet),
    Command::SimulationPage(SimulationPage::Models),
    Command::SimulationPage(SimulationPage::Solver),
    Command::SimulationPage(SimulationPage::Save),
    Command::VerificationPage(VerificationPage::Yield),
    Command::ZoomIn,
    Command::ZoomOut,
    Command::ZoomFit,
    Command::FitSchematicContent,
    Command::ZoomOneToOne,
    Command::ZoomToSelection,
    Command::ZoomBox,
    Command::CycleGrid,
    Command::GridSnapRouting,
    Command::VisibilityOptions,
    Command::ToggleFullScreen,
    Command::ResetActiveView,
    Command::EngineeringTableView,
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
    Command::CloseOtherDocuments,
    Command::CloseAllDocuments,
    Command::WindowManager,
    Command::ResetLayout,
    Command::SelectTool,
    Command::PlaceInstance,
    Command::PlaceWire,
    Command::PlaceBus,
    Command::PlaceBusTap,
    Command::PlaceJunction,
    Command::PlaceLabel,
    Command::PlaceOffSheetConnector,
    Command::PlaceProbe,
    Command::PlacePin,
    Command::PlaceText,
    Command::PlaceShape,
    Command::MoveSelection,
    Command::StretchSelection,
    Command::ArraySelection,
    Command::ReplaceInstance,
    Command::UpdateInstanceInterface,
    Command::CreateHierarchy,
    Command::DesignManagement,
    Command::SelectionBulkEdit,
    Command::ConnectivityManager,
    Command::ConfigurationSets,
    Command::RuleSetup,
    Command::ReviewComments,
    Command::RevisionHistory,
    Command::DesignSpecialistWorkspaces,
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
    Command::Place(ComponentType::Resistor),
    Command::Place(ComponentType::Capacitor),
    Command::Place(ComponentType::Inductor),
    Command::Place(ComponentType::Diode),
    Command::Place(ComponentType::Ground),
    Command::Place(ComponentType::VoltageSource),
    Command::Place(ComponentType::CurrentSource),
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
    Command::JoinCollinearWires,
    Command::AscendHierarchy,
    Command::DescendHierarchy,
    Command::DescendHierarchyDirect,
    Command::ShowInNetlist,
    Command::CheckCellview,
    Command::CheckDesign,
    Command::CheckAndSave,
    Command::ClearChecks,
    Command::NextViolation,
    Command::PreviousViolation,
    Command::RunSimulation,
    Command::StopSimulation,
    Command::JobsManager,
    Command::PreflightChecks,
    Command::ManageSimulationPlans,
    Command::AddAnalysis,
    Command::SimulationOptions,
    Command::GenerateNetlist,
    Command::FindCodeDocument,
    Command::ValidateCodeDocument,
    Command::ManageSourceDocument,
    Command::SourceLanguageTools,
    Command::ToggleLineComment,
    Command::IndentLines,
    Command::OutdentLines,
    Command::DuplicateLine,
    Command::DeleteLine,
    Command::MoveLineUp,
    Command::MoveLineDown,
    Command::GoToMatchingBracket,
    Command::GoToLine,
    Command::FormatDocument,
    Command::FoldSelection,
    Command::UnfoldAtCaret,
    Command::FoldAll,
    Command::UnfoldAll,
    Command::GoToDefinition,
    Command::FindReferences,
    Command::RenameSymbol,
    Command::CompareGeneratedRevisions,
    Command::VerificationPage(VerificationPage::Corners),
    Command::VerificationPage(VerificationPage::Optimization),
    Command::VerificationPage(VerificationPage::DeviceSafety),
    Command::VerificationPage(VerificationPage::Regression),
    Command::StimulusNewDefinition,
    Command::StimulusDuplicateDefinition,
    Command::StimulusDeleteDefinition,
    Command::StimulusApplyDraft,
    Command::StimulusRevertDraft,
    Command::StimulusValidateLibrary,
    Command::StimulusPlaceDefinition,
    Command::StimulusShowAdopter,
    Command::CompileVerilogA,
    Command::VerilogABuildSettings,
    Command::AutomationConsole,
    Command::CommandPalette,
    Command::KeyboardShortcuts,
    Command::SpecialistToolBrowser,
    Command::HelpCenter,
    Command::ReleaseNotes,
    Command::MigrationGuide,
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
    Command::ProjectLauncher.key(),
    Command::RecentProjects.key(),
    Command::OpenProject.key(),
    Command::NewProject.key(),
    Command::Save.key(),
    Command::SaveAs.key(),
    Command::SaveAll.key(),
    Command::RevertActiveDocument.key(),
    Command::CloseActiveDocument.key(),
    Command::CloseProject.key(),
    Command::NewCell.key(),
    Command::NewView.key(),
    Command::NewVerilogADevice.key(),
    Command::OpenDocument.key(),
    Command::OpenNetlist.key(),
    Command::ImportNetlist.key(),
    Command::ImportVerilogA.key(),
    ResultsCommand::ImportResultDataset.key(),
    HardcopyCommand::ExportSchematicSvg.key(),
    HardcopyCommand::ExportPublicationSnapshot.key(),
    CloudCommand::LiveSession.key(),
    CloudCommand::PublishToWeb.key(),
    Command::ExportNetlist(crate::io::NetlistFormat::Spectre).key(),
    Command::ExportNetlist(crate::io::NetlistFormat::Spice).key(),
    Command::ExportNetlist(crate::io::NetlistFormat::Hspice).key(),
    Command::ExportNetlist(crate::io::NetlistFormat::Xyce).key(),
    SheetsCommand::SheetProperties.key(),
    SheetsCommand::SheetFormatManager.key(),
    SheetsCommand::CustomSheetSizes.key(),
    SheetsCommand::SheetDefaults.key(),
    HardcopyCommand::PrintHardcopy.key(),
    HardcopyCommand::ExportActiveView.key(),
    Command::Exit.key(),
    Command::Undo.key(),
    Command::Redo.key(),
    Command::Cut.key(),
    Command::Copy.key(),
    Command::Paste.key(),
    Command::Duplicate.key(),
    Command::Delete.key(),
    Command::SelectAll.key(),
    Command::RenameSelection.key(),
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
    Command::ProjectPage(ProjectPage::Overview).key(),
    Command::ProjectPage(ProjectPage::Library).key(),
    Command::ProjectPage(ProjectPage::Configuration).key(),
    Command::ProjectPage(ProjectPage::Dependencies).key(),
    Command::ProjectPage(ProjectPage::Recovery).key(),
    Command::SimulationPage(SimulationPage::Analyses).key(),
    Command::SimulationPage(SimulationPage::Excitations).key(),
    Command::SimulationPage(SimulationPage::Variables).key(),
    Command::SimulationPage(SimulationPage::Outputs).key(),
    Command::SimulationPage(SimulationPage::Specifications).key(),
    Command::SimulationPage(SimulationPage::RunSet).key(),
    Command::SimulationPage(SimulationPage::Models).key(),
    Command::SimulationPage(SimulationPage::Solver).key(),
    Command::SimulationPage(SimulationPage::Save).key(),
    ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Waves).key(),
    Command::VerificationPage(VerificationPage::Yield).key(),
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
    Command::FitSchematicContent.key(),
    Command::ZoomOneToOne.key(),
    Command::ZoomToSelection.key(),
    Command::ZoomBox.key(),
    Command::CycleGrid.key(),
    Command::GridSnapRouting.key(),
    SheetsCommand::DrawingSheetLayers.key(),
    Command::VisibilityOptions.key(),
    Command::ToggleFullScreen.key(),
    Command::ResetActiveView.key(),
    Command::EngineeringTableView.key(),
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
    Command::CloseOtherDocuments.key(),
    Command::CloseAllDocuments.key(),
    SettingsCommand::WorkspaceLayouts.key(),
    Command::WindowManager.key(),
    Command::ResetLayout.key(),
    Command::SelectTool.key(),
    Command::PlaceInstance.key(),
    Command::PlaceWire.key(),
    Command::PlaceBus.key(),
    Command::PlaceBusTap.key(),
    Command::PlaceJunction.key(),
    Command::PlaceLabel.key(),
    Command::PlaceOffSheetConnector.key(),
    Command::PlaceProbe.key(),
    Command::PlacePin.key(),
    Command::PlaceText.key(),
    Command::PlaceShape.key(),
    Command::MoveSelection.key(),
    Command::StretchSelection.key(),
    Command::ArraySelection.key(),
    Command::ReplaceInstance.key(),
    Command::UpdateInstanceInterface.key(),
    Command::CreateHierarchy.key(),
    Command::DesignManagement.key(),
    Command::SelectionBulkEdit.key(),
    Command::ConnectivityManager.key(),
    Command::ConfigurationSets.key(),
    Command::RuleSetup.key(),
    Command::ReviewComments.key(),
    Command::RevisionHistory.key(),
    Command::DesignSpecialistWorkspaces.key(),
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
    Command::Place(ComponentType::Resistor).key(),
    Command::Place(ComponentType::Capacitor).key(),
    Command::Place(ComponentType::Inductor).key(),
    Command::Place(ComponentType::Diode).key(),
    Command::Place(ComponentType::Ground).key(),
    Command::Place(ComponentType::VoltageSource).key(),
    Command::Place(ComponentType::CurrentSource).key(),
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
    Command::JoinCollinearWires.key(),
    Command::AscendHierarchy.key(),
    Command::DescendHierarchy.key(),
    Command::DescendHierarchyDirect.key(),
    Command::ShowInNetlist.key(),
    Command::CheckCellview.key(),
    Command::CheckDesign.key(),
    Command::CheckAndSave.key(),
    Command::ClearChecks.key(),
    Command::NextViolation.key(),
    Command::PreviousViolation.key(),
    Command::RunSimulation.key(),
    Command::StopSimulation.key(),
    Command::JobsManager.key(),
    Command::PreflightChecks.key(),
    Command::ManageSimulationPlans.key(),
    Command::AddAnalysis.key(),
    Command::SimulationOptions.key(),
    Command::GenerateNetlist.key(),
    Command::FindCodeDocument.key(),
    Command::ValidateCodeDocument.key(),
    Command::ManageSourceDocument.key(),
    Command::SourceLanguageTools.key(),
    Command::ToggleLineComment.key(),
    Command::IndentLines.key(),
    Command::OutdentLines.key(),
    Command::DuplicateLine.key(),
    Command::DeleteLine.key(),
    Command::MoveLineUp.key(),
    Command::MoveLineDown.key(),
    Command::GoToMatchingBracket.key(),
    Command::GoToLine.key(),
    Command::FormatDocument.key(),
    Command::FoldSelection.key(),
    Command::UnfoldAtCaret.key(),
    Command::FoldAll.key(),
    Command::UnfoldAll.key(),
    Command::GoToDefinition.key(),
    Command::FindReferences.key(),
    Command::RenameSymbol.key(),
    Command::CompareGeneratedRevisions.key(),
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
    Command::VerificationPage(VerificationPage::Corners).key(),
    Command::VerificationPage(VerificationPage::Optimization).key(),
    Command::VerificationPage(VerificationPage::DeviceSafety).key(),
    Command::VerificationPage(VerificationPage::Regression).key(),
    ModelsCommand::Page(ModelsPage::Symbols).key(),
    ModelsCommand::Page(ModelsPage::Corners).key(),
    ModelsCommand::Page(ModelsPage::Bins).key(),
    ModelsCommand::Page(ModelsPage::Include).key(),
    ModelsCommand::Page(ModelsPage::Qualification).key(),
    ModelsCommand::ModelBrowser.key(),
    ModelsCommand::PdkSettings.key(),
    ModelsCommand::RescanModelLibraries.key(),
    Command::StimulusNewDefinition.key(),
    Command::StimulusDuplicateDefinition.key(),
    Command::StimulusDeleteDefinition.key(),
    Command::StimulusApplyDraft.key(),
    Command::StimulusRevertDraft.key(),
    Command::StimulusValidateLibrary.key(),
    Command::StimulusPlaceDefinition.key(),
    Command::StimulusShowAdopter.key(),
    Command::CompileVerilogA.key(),
    Command::VerilogABuildSettings.key(),
    Command::AutomationConsole.key(),
    Command::CommandPalette.key(),
    Command::KeyboardShortcuts.key(),
    CloudCommand::AccountOrganization.key(),
    CloudCommand::License.key(),
    Command::SpecialistToolBrowser.key(),
    HardcopyCommand::ReportAuthoring.key(),
    ResultsCommand::AddVisualizationPane.key(),
    ResultsCommand::VisualizationDocumentProperties.key(),
    Command::HelpCenter.key(),
    Command::ReleaseNotes.key(),
    Command::MigrationGuide.key(),
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
                .collect(),
            LEGACY_ALIASES,
        )
        .with_unlisted(
            HardcopyCommand::REPORT_TOOLBAR
                .map(HardcopyCommand::declared_spec)
                .to_vec(),
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
