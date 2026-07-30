//! The command vocabulary.
//!
//! Every command the product exposes, its stable portable identity, and the
//! label and group a menu or palette shows it under. This is a value model:
//! it describes what commands exist, not what pressing one does. Dispatch —
//! `is_enabled` and `execute`, both of which need the running application —
//! lives up in [`crate::workbench::commands`].
//!
//! Keeping the two apart is what lets shortcut profiles, preferences, and
//! the artifact writers name a command without depending on the dispatcher
//! that runs it.

use crate::state::ComponentType;
use crate::workbench::state::{ModelsPage, ProjectPage, VerificationPage, Workspace};
use serde::{Deserialize, Serialize};

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

    pub(super) const fn has_browser_reserved_primary(self) -> bool {
        matches!(self, Self::Browser)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    OpenDocument,
    OpenNetlist,
    ImportNetlist,
    ImportVerilogA,
    ExportSchematicSvg,
    ExportWaveformsCsv,
    ExportNetlist(crate::io::NetlistFormat),
    PageSetup,
    PrintHardcopy,
    ExportActiveView,
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
    Preferences,
    ZoomIn,
    ZoomOut,
    ZoomFit,
    ZoomOneToOne,
    CycleGrid,
    VisibilityOptions,
    ToggleFullScreen,
    ResetActiveView,
    EngineeringTableView,
    ToggleNavigator,
    ToggleInspector,
    ToggleConsole,
    ToggleResultsSplit,
    OpenConsole,
    OpenProblems,
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
    WorkspaceLayouts,
    WindowManager,
    PreviousWorkspace,
    NextWorkspace,
    SelectTool,
    PlaceInstance,
    PlaceWire,
    PlaceBus,
    PlaceBusTap,
    PlaceJunction,
    PlaceLabel,
    PlaceProbe,
    PlacePin,
    PlaceText,
    PlaceShape,
    MoveSelection,
    StretchSelection,
    ArraySelection,
    ReplaceInstance,
    CreateHierarchy,
    ConnectivityManager,
    DesignManagement,
    SelectionBulkEdit,
    ConfigurationSets,
    ReviewComments,
    RevisionHistory,
    SymbolPinTool,
    SymbolPolylineTool,
    SymbolRectangleTool,
    SymbolCircleTool,
    SymbolArcTool,
    SymbolPolygonTool,
    SymbolTextTool,
    SymbolRotatePin,
    SymbolMirrorPin,
    SymbolSave,
    Place(ComponentType),
    RotateSelection,
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
    Cancel,
    AscendHierarchy,
    DescendHierarchy,
    /// Direct edit-in-place hierarchy gesture used by the toolbar and
    /// Shift+E. The Design-menu command above owns the explicit transaction.
    DescendHierarchyDirect,
    RunChecks,
    CheckAndSave,
    ClearChecks,
    NextViolation,
    PreviousViolation,
    RunSimulation,
    StopSimulation,
    JobsManager,
    PreflightChecks,
    SimulationOptions,
    GenerateNetlist,
    FindCodeDocument,
    ValidateCodeDocument,
    CompareGeneratedRevisions,
    ClearResults,
    ToggleLinkedCursors,
    WaveformCalculator,
    ResultViewer(crate::workbench::ResultViewer),
    EditSpecifications,
    VerificationPage(VerificationPage),
    ProjectPage(ProjectPage),
    ModelsPage(ModelsPage),
    ModelBrowser,
    ModelCreateProjectCopy,
    ModelEditor,
    ModelCorrelation,
    ModelSaveRevision,
    ModelValidate,
    ModelRunQualificationTests,
    ModelCompareRelease,
    PdkSettings,
    RescanModelLibraries,
    CompileVerilogA,
    AutomationConsole,
    CommandPalette,
    KeyboardShortcuts,
    AccountOrganization,
    License,
    SpecialistToolBrowser,
    VisualizationStudio,
    ReportAuthoring,
    SaveReportDocument,
    AddReportPage,
    ReportPageProperties,
    AddVisualizationPane,
    VisualizationTraceManager,
    VisualizationCursorManager,
    VisualizationDocumentProperties,
    ExportVisualizationDocument,
    FeatureAvailability,
    InteroperabilityMatrix,
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

    /// Resolve a stable product identity through the canonical registry.
    /// Unknown IDs are intentionally retained by profile persistence but are
    /// never guessed into a command owned by this build.
    pub fn from_stable_id(id: &str) -> Option<Self> {
        // `model-metadata-audit` shipped as the portable identity for this
        // tab before the mockup-defined Qualification workspace replaced the
        // temporary audit. Preserve imported shortcut profiles explicitly;
        // do not make general command resolution fuzzy.
        if id == "model-metadata-audit" {
            return Some(Self::ModelsPage(ModelsPage::Qualification));
        }
        COMMAND_REGISTRY
            .iter()
            .copied()
            .find(|command| command.stable_id() == id)
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
                spec("netlist", "Open automation workspace", "Navigate")
            }
            Self::ProjectLauncher => spec("project-launcher", "Project launcher…", "File"),
            Self::RecentProjects => spec("recent-projects", "Recent projects…", "File"),
            Self::NewProject => spec("new-project", "New project…", "File"),
            Self::OpenProject => spec("open-project", "Open project…", "File"),
            Self::Save => spec("save-project", "Save", "File"),
            Self::SaveAs => spec("save-project-as", "Save as project copy…", "File"),
            Self::SaveAll => spec("save-all", "Save all", "File"),
            Self::RevertActiveDocument => {
                spec("revert-document", "Revert active document…", "File")
            }
            Self::CloseActiveDocument => spec("close-document", "Close active document", "File"),
            Self::CloseProject => spec("close-project", "Close project…", "File"),
            Self::NewCell => spec("new-cell", "New cell…", "File"),
            Self::OpenDocument => spec("open-schematic", "Open schematic…", "File"),
            Self::OpenNetlist => spec("open-netlist", "Open netlist…", "File"),
            Self::ImportNetlist => spec("import-netlist", "Import SPICE deck…", "File"),
            Self::ImportVerilogA => spec("import-veriloga", "Import Verilog-A…", "File"),
            Self::ExportSchematicSvg => {
                spec("export-schematic-svg", "Export schematic SVG…", "File")
            }
            Self::ExportWaveformsCsv => spec("export-waveforms", "Export result data…", "File"),
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
            Self::PageSetup => spec("page-setup", "Page setup…", "File"),
            Self::PrintHardcopy => spec("print-hardcopy", "Print / hardcopy…", "File"),
            Self::ExportActiveView => spec("export-active-view", "Export active view…", "File"),
            Self::Exit => spec("exit-rspice", "Exit RSpice…", "File"),
            Self::Undo => spec("undo", "Undo", "Edit"),
            Self::Redo => spec("redo", "Redo", "Edit"),
            Self::Cut => spec("cut-selection", "Cut selection", "Edit"),
            Self::Copy => spec("copy-selection", "Copy selection", "Edit"),
            Self::Paste => spec("paste-selection", "Paste", "Edit"),
            Self::Duplicate => spec("duplicate-selection", "Duplicate selection", "Edit"),
            Self::Delete => spec("delete-selection", "Delete selection", "Edit"),
            Self::SelectAll => spec("select-all", "Select all in edit context", "Edit"),
            Self::RenameSelection => spec("rename-selection", "Rename selected object…", "Edit"),
            Self::ObjectProperties => spec("object-properties", "Object properties…", "Edit"),
            Self::FindInDesign => spec("find-design", "Find in design…", "Edit"),
            Self::Preferences => spec("preferences", "Preferences…", "Edit"),
            Self::ZoomIn => spec("zoom-in", "Zoom in", "View"),
            Self::ZoomOut => spec("zoom-out", "Zoom out", "View"),
            Self::ZoomFit => spec("fit-canvas", "Zoom active canvas to fit", "View"),
            Self::ZoomOneToOne => spec("zoom-one-to-one", "Zoom 100%", "View"),
            Self::CycleGrid => spec("toggle-grid", "Canvas grid and snap", "View"),
            Self::VisibilityOptions => spec(
                "visibility-options",
                "Hierarchy and annotation visibility\u{2026}",
                "View",
            ),
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
            Self::ToggleResultsSplit => spec("toggle-split-view", "Split with results", "Window"),
            Self::OpenConsole => spec("open-console", "Open console", "Window"),
            Self::OpenProblems => spec("open-problems", "Open Problems", "Window"),
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
            Self::WorkspaceLayouts => {
                spec("workspace-layouts", "Workspace layouts\u{2026}", "Window")
            }
            Self::WindowManager => spec(
                "window-manager",
                "Windows, documents and session\u{2026}",
                "Window",
            ),
            Self::PreviousWorkspace => spec("previous-workspace", "Previous workspace", "Window"),
            Self::NextWorkspace => spec("next-workspace", "Next workspace", "Window"),
            Self::SelectTool => spec("select-tool", "Select tool", "Design"),
            Self::PlaceInstance => spec("place-instance", "Place instance…", "Design"),
            Self::PlaceWire => spec("place-wire", "Draw wire", "Design"),
            Self::PlaceBus => spec("place-bus", "Draw bus", "Design"),
            Self::PlaceBusTap => spec("place-bus-tap", "Place bus tap", "Design"),
            Self::PlaceJunction => spec("place-junction", "Place junction", "Design"),
            Self::PlaceLabel => spec("place-label", "Place net label", "Design"),
            Self::PlaceProbe => spec("place-probe", "Place probe", "Design"),
            Self::PlacePin => spec("place-pin", "Place pin or port\u{2026}", "Design"),
            Self::PlaceText => spec("place-text", "Place text or note\u{2026}", "Design"),
            Self::PlaceShape => spec("place-shape", "Draw documentation shape\u{2026}", "Design"),
            Self::MoveSelection => spec("move-selection", "Move selection", "Design"),
            Self::StretchSelection => spec("stretch-selection", "Stretch selection", "Design"),
            Self::ArraySelection => spec("array-selection", "Create array\u{2026}", "Design"),
            Self::ReplaceInstance => spec("replace-instance", "Replace instance\u{2026}", "Design"),
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
            Self::ConfigurationSets => {
                spec("configuration-sets", "Configuration sets\u{2026}", "Design")
            }
            Self::ReviewComments => spec("review-comments", "Review comments\u{2026}", "Design"),
            Self::RevisionHistory => spec("revision-history", "Revision history\u{2026}", "Design"),
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
            Self::SymbolSave => spec("save-symbol", "Validate and save symbol", "Design"),
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
            Self::Cancel => spec("cancel-active-command", "Cancel active command", "Design"),
            Self::AscendHierarchy => spec("ascend-hierarchy", "Ascend hierarchy", "Design"),
            Self::DescendHierarchy => spec(
                "descend-hierarchy",
                "Descend into selected instance",
                "Design",
            ),
            Self::DescendHierarchyDirect => spec(
                "descend-selected-instance",
                "Descend into selected instance",
                "Design",
            ),
            Self::RunChecks => spec("run-checks", "Run schematic checks", "Design"),
            Self::CheckAndSave => spec("check-and-save", "Check and save", "Design"),
            Self::ClearChecks => spec("clear-checks", "Clear check results", "Design"),
            Self::NextViolation => spec("next-violation", "Next violation", "Verify"),
            Self::PreviousViolation => spec("previous-violation", "Previous violation", "Verify"),
            Self::RunSimulation => spec("start-run", "Run active plan", "Simulate"),
            Self::StopSimulation => spec("stop-run", "Stop active run", "Simulate"),
            Self::JobsManager => spec("jobs-manager", "Jobs, targets and run history…", "Simulate"),
            Self::PreflightChecks => spec("check", "Preflight checks", "Simulate"),
            Self::SimulationOptions => spec("solver", "Global solver & convergence", "Simulate"),
            Self::GenerateNetlist => {
                spec("generated-netlist", "Open generated netlist", "Simulate")
            }
            Self::FindCodeDocument => spec("find-code", "Find in Code document…", "Code"),
            Self::ValidateCodeDocument => {
                spec("validate-code", "Validate active Code document", "Code")
            }
            Self::CompareGeneratedRevisions => spec(
                "compare-generated-revisions",
                "Compare generated revisions",
                "Code",
            ),
            Self::ClearResults => spec("clear-results", "Clear result history", "Results"),
            Self::ToggleLinkedCursors => {
                spec("toggle-linked-cursors", "Linked A/B cursors", "Results")
            }
            Self::WaveformCalculator => spec("calculator", "Calculator…", "Results"),
            Self::ResultViewer(crate::workbench::ResultViewer::Waves) => {
                spec("waveforms", "Open results workspace", "Results")
            }
            Self::ResultViewer(crate::workbench::ResultViewer::Bode) => {
                spec("result-bode", "Open Bode viewer", "Results")
            }
            Self::ResultViewer(crate::workbench::ResultViewer::Fft) => {
                spec("result-fft", "Open FFT viewer", "Results")
            }
            Self::ResultViewer(crate::workbench::ResultViewer::Eye) => {
                spec("result-eye", "Open eye-diagram viewer", "Results")
            }
            Self::ResultViewer(crate::workbench::ResultViewer::Hist) => {
                spec("result-histogram", "Open histogram viewer", "Results")
            }
            Self::ResultViewer(crate::workbench::ResultViewer::Op) => spec(
                "result-operating-point",
                "Open operating-point viewer",
                "Results",
            ),
            Self::ResultViewer(crate::workbench::ResultViewer::NoiseContrib) => {
                spec("result-noise", "Open noise-contribution viewer", "Results")
            }
            Self::ResultViewer(crate::workbench::ResultViewer::Contribution) => spec(
                "result-sensitivity-contribution",
                "Open sensitivity-contribution viewer",
                "Results",
            ),
            Self::ResultViewer(crate::workbench::ResultViewer::TransferFunction) => spec(
                "result-transfer-function",
                "Open transfer-function viewer",
                "Results",
            ),
            Self::ResultViewer(crate::workbench::ResultViewer::Specs) => spec(
                "result-specifications",
                "Open specification results",
                "Results",
            ),
            Self::ResultViewer(crate::workbench::ResultViewer::Table) => {
                spec("result-table", "Open sample table", "Results")
            }
            Self::ResultViewer(crate::workbench::ResultViewer::Nyquist) => {
                spec("result-nyquist", "Open Nyquist viewer", "Results")
            }
            Self::ResultViewer(crate::workbench::ResultViewer::Smith) => {
                spec("result-smith", "Open Smith-chart viewer", "Results")
            }
            Self::ResultViewer(crate::workbench::ResultViewer::PoleZero) => {
                spec("result-pole-zero", "Open pole-zero viewer", "Results")
            }
            Self::EditSpecifications => {
                spec("specifications", "Edit specification matrix", "Results")
            }
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
            Self::VerificationPage(VerificationPage::Reliability) => {
                spec("reliability", "Reliability and SOA", "Verify")
            }
            Self::VerificationPage(VerificationPage::Regression) => {
                spec("regression", "Regression plan", "Verify")
            }
            Self::VerificationPage(VerificationPage::Drc) => {
                spec("open-drc", "Physical DRC unavailable", "Verify")
            }
            Self::ProjectPage(ProjectPage::Dashboard) => {
                spec("project-overview", "Open project overview", "Project")
            }
            Self::ProjectPage(ProjectPage::Configuration) => spec(
                "project-testbench-configuration",
                "Open testbench configuration",
                "Project",
            ),
            Self::ProjectPage(ProjectPage::Technology) => {
                spec("project-technology", "Open technology and PDK", "Project")
            }
            Self::ProjectPage(ProjectPage::Dependencies) => spec(
                "project-dependencies",
                "Open dependency manifest",
                "Project",
            ),
            Self::ProjectPage(ProjectPage::Recovery) => {
                spec("project-recovery", "Open recovery center", "Project")
            }
            Self::ModelsPage(ModelsPage::Models) => {
                spec("models-catalog", "Model & library catalog", "Models")
            }
            Self::ModelsPage(ModelsPage::Symbols) => spec("symbols-cdf", "Symbols & CDF", "Models"),
            Self::ModelsPage(ModelsPage::Corners) => {
                spec("corner-sections", "Corners & sections", "Models")
            }
            Self::ModelsPage(ModelsPage::Bins) => spec("model-bins", "Bins & geometry", "Models"),
            Self::ModelsPage(ModelsPage::Include) => {
                spec("include-graph", "Include graph", "Models")
            }
            Self::ModelsPage(ModelsPage::Qualification) => {
                spec("model-qualification", "Model qualification", "Models")
            }
            Self::ModelBrowser => spec("model-browser", "Model browser…", "Models"),
            Self::ModelCreateProjectCopy => spec(
                "model-create-project-copy",
                "Create editable project copy",
                "Models",
            ),
            Self::ModelEditor => spec(
                "model-editor",
                "Device model and parameter editor…",
                "Models",
            ),
            Self::ModelCorrelation => {
                spec("model-correlation", "Measurement correlation…", "Models")
            }
            Self::ModelSaveRevision => spec("model-save-revision", "Save model revision", "Models"),
            Self::ModelValidate => spec("model-validate", "Validate model", "Models"),
            Self::ModelRunQualificationTests => {
                spec("model-run-tests", "Run qualification tests", "Models")
            }
            Self::ModelCompareRelease => spec("model-compare", "Compare with release", "Models"),
            Self::PdkSettings => spec("pdk-settings", "PDK and model paths…", "Models"),
            Self::RescanModelLibraries => {
                spec("rescan-model-libraries", "Rescan model libraries", "Models")
            }
            Self::CompileVerilogA => spec("veriloga", "Compile Verilog-A", "Models"),
            Self::AutomationConsole => spec("automation", "Automation workspace", "Automation"),
            Self::CommandPalette => spec("command-palette", "Command palette", "Navigate"),
            Self::KeyboardShortcuts => spec("command-reference", "Command reference", "Help"),
            Self::AccountOrganization => spec(
                "account-organization",
                "Account and administration…",
                "Account",
            ),
            Self::License => spec("license-activation", "License and activation…", "Help"),
            Self::SpecialistToolBrowser => spec(
                "specialist-tool-browser",
                "Specialist tool browser…",
                "Navigate",
            ),
            Self::VisualizationStudio => spec(
                "visualization-studio",
                "Open Visualization Studio",
                "Navigate",
            ),
            Self::ReportAuthoring => spec("report-authoring", "Open report authoring", "Navigate"),
            Self::SaveReportDocument => {
                spec("report-save-document", "Save report document", "Results")
            }
            Self::AddReportPage => spec("report-add-page", "Add report page", "Results"),
            Self::ReportPageProperties => {
                spec("report-page-properties", "Page properties", "Results")
            }
            Self::AddVisualizationPane => spec(
                "visualization-add-pane",
                "Add visualization pane",
                "Results",
            ),
            Self::VisualizationTraceManager => {
                spec("visualization-trace-manager", "Trace manager", "Results")
            }
            Self::VisualizationCursorManager => {
                spec("visualization-cursor-manager", "Cursor manager", "Results")
            }
            Self::VisualizationDocumentProperties => spec(
                "visualization-document-properties",
                "Document properties",
                "Results",
            ),
            Self::ExportVisualizationDocument => spec(
                "visualization-export-document",
                "Export document",
                "Results",
            ),
            Self::FeatureAvailability => spec(
                "feature-availability",
                "Product capability and platform matrix…",
                "Help",
            ),
            Self::InteroperabilityMatrix => spec(
                "interoperability-matrix",
                "Interoperability and format matrix…",
                "Help",
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
    Command::OpenDocument,
    Command::OpenNetlist,
    Command::ImportNetlist,
    Command::ImportVerilogA,
    Command::ExportSchematicSvg,
    Command::ExportNetlist(crate::io::NetlistFormat::Spectre),
    Command::ExportNetlist(crate::io::NetlistFormat::Spice),
    Command::ExportNetlist(crate::io::NetlistFormat::Hspice),
    Command::ExportNetlist(crate::io::NetlistFormat::Xyce),
    Command::PageSetup,
    Command::PrintHardcopy,
    Command::ExportActiveView,
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
    Command::Preferences,
    Command::OpenWorkspace(Workspace::Project),
    Command::OpenWorkspace(Workspace::Design),
    Command::OpenWorkspace(Workspace::Simulate),
    Command::OpenWorkspace(Workspace::Results),
    Command::OpenWorkspace(Workspace::Verify),
    Command::OpenWorkspace(Workspace::Models),
    Command::OpenWorkspace(Workspace::Netlist),
    Command::ProjectPage(ProjectPage::Dashboard),
    Command::ProjectPage(ProjectPage::Configuration),
    Command::ProjectPage(ProjectPage::Technology),
    Command::ProjectPage(ProjectPage::Dependencies),
    Command::ProjectPage(ProjectPage::Recovery),
    Command::ResultViewer(crate::workbench::ResultViewer::Waves),
    Command::VerificationPage(VerificationPage::Yield),
    Command::ModelsPage(ModelsPage::Models),
    Command::ModelCreateProjectCopy,
    Command::ModelEditor,
    Command::ModelCorrelation,
    Command::ModelSaveRevision,
    Command::ModelValidate,
    Command::ModelRunQualificationTests,
    Command::ModelCompareRelease,
    Command::ZoomIn,
    Command::ZoomOut,
    Command::ZoomFit,
    Command::ZoomOneToOne,
    Command::CycleGrid,
    Command::VisibilityOptions,
    Command::ToggleFullScreen,
    Command::ResetActiveView,
    Command::EngineeringTableView,
    Command::ToggleNavigator,
    Command::ToggleInspector,
    Command::ToggleConsole,
    Command::ToggleResultsSplit,
    Command::OpenConsole,
    Command::OpenProblems,
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
    Command::WorkspaceLayouts,
    Command::WindowManager,
    Command::ResetLayout,
    Command::SelectTool,
    Command::PlaceInstance,
    Command::PlaceWire,
    Command::PlaceBus,
    Command::PlaceBusTap,
    Command::PlaceJunction,
    Command::PlaceLabel,
    Command::PlaceProbe,
    Command::PlacePin,
    Command::PlaceText,
    Command::PlaceShape,
    Command::MoveSelection,
    Command::StretchSelection,
    Command::ArraySelection,
    Command::ReplaceInstance,
    Command::CreateHierarchy,
    Command::DesignManagement,
    Command::SelectionBulkEdit,
    Command::ConnectivityManager,
    Command::ConfigurationSets,
    Command::ReviewComments,
    Command::RevisionHistory,
    Command::SymbolPinTool,
    Command::SymbolPolylineTool,
    Command::SymbolRectangleTool,
    Command::SymbolCircleTool,
    Command::SymbolArcTool,
    Command::SymbolPolygonTool,
    Command::SymbolTextTool,
    Command::SymbolRotatePin,
    Command::SymbolMirrorPin,
    Command::SymbolSave,
    Command::Place(ComponentType::Resistor),
    Command::Place(ComponentType::Capacitor),
    Command::Place(ComponentType::Inductor),
    Command::Place(ComponentType::Diode),
    Command::Place(ComponentType::Ground),
    Command::Place(ComponentType::VoltageSource),
    Command::Place(ComponentType::CurrentSource),
    Command::RotateSelection,
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
    Command::AscendHierarchy,
    Command::DescendHierarchy,
    Command::DescendHierarchyDirect,
    Command::RunChecks,
    Command::CheckAndSave,
    Command::ClearChecks,
    Command::NextViolation,
    Command::PreviousViolation,
    Command::RunSimulation,
    Command::StopSimulation,
    Command::JobsManager,
    Command::PreflightChecks,
    Command::SimulationOptions,
    Command::GenerateNetlist,
    Command::FindCodeDocument,
    Command::ValidateCodeDocument,
    Command::CompareGeneratedRevisions,
    Command::ExportWaveformsCsv,
    Command::ClearResults,
    Command::ToggleLinkedCursors,
    Command::WaveformCalculator,
    Command::ResultViewer(crate::workbench::ResultViewer::Bode),
    Command::ResultViewer(crate::workbench::ResultViewer::Fft),
    Command::ResultViewer(crate::workbench::ResultViewer::Eye),
    Command::ResultViewer(crate::workbench::ResultViewer::Hist),
    Command::ResultViewer(crate::workbench::ResultViewer::Op),
    Command::ResultViewer(crate::workbench::ResultViewer::NoiseContrib),
    Command::ResultViewer(crate::workbench::ResultViewer::Contribution),
    Command::ResultViewer(crate::workbench::ResultViewer::TransferFunction),
    Command::ResultViewer(crate::workbench::ResultViewer::Specs),
    Command::ResultViewer(crate::workbench::ResultViewer::Nyquist),
    Command::ResultViewer(crate::workbench::ResultViewer::Smith),
    Command::ResultViewer(crate::workbench::ResultViewer::PoleZero),
    Command::EditSpecifications,
    Command::VerificationPage(VerificationPage::Corners),
    Command::VerificationPage(VerificationPage::Optimization),
    Command::VerificationPage(VerificationPage::Reliability),
    Command::VerificationPage(VerificationPage::Regression),
    Command::ModelsPage(ModelsPage::Symbols),
    Command::ModelsPage(ModelsPage::Corners),
    Command::ModelsPage(ModelsPage::Bins),
    Command::ModelsPage(ModelsPage::Include),
    Command::ModelsPage(ModelsPage::Qualification),
    Command::ModelBrowser,
    Command::PdkSettings,
    Command::RescanModelLibraries,
    Command::CompileVerilogA,
    Command::AutomationConsole,
    Command::CommandPalette,
    Command::KeyboardShortcuts,
    Command::AccountOrganization,
    Command::License,
    Command::SpecialistToolBrowser,
    Command::VisualizationStudio,
    Command::AddVisualizationPane,
    Command::VisualizationTraceManager,
    Command::VisualizationCursorManager,
    Command::VisualizationDocumentProperties,
    Command::ExportVisualizationDocument,
    Command::FeatureAvailability,
    Command::InteroperabilityMatrix,
    Command::HelpCenter,
    Command::ReleaseNotes,
    Command::MigrationGuide,
    Command::SystemDiagnostics,
    Command::SupportBundle,
    Command::LegalPrivacy,
    Command::About,
    Command::Cancel,
];

/// Searchable projection of [`COMMAND_REGISTRY`]. The command palette itself
/// and context-only cancellation are intentionally not self-referential rows.
pub fn command_catalog() -> impl Iterator<Item = Command> {
    COMMAND_REGISTRY
        .iter()
        .copied()
        .filter(|command| command.palette_visible())
}
