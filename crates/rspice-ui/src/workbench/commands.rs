//! Typed workbench commands and their single dispatch boundary.
//!
//! The workbench never paints a visible action without routing it here.  A
//! command is omitted from a menu when its behavior is not implemented; the
//! UI does not advertise speculative or placeholder capability.

use crate::common::RSpiceApp;
use crate::common::menu_bar::{FileMenuAction, dispatch_file_menu_action};
use crate::schematic::view::SchematicSymbolContext;
use crate::state::{ComponentType, Tool};

use super::state::{
    ModelsPage, ProjectLauncherFilter, ProjectPage, VerificationPage, WorkbenchState, Workspace,
};

mod registry;

fn stop_simulation_enabled(is_running: bool) -> bool {
    is_running && crate::simulation::execution::execution_target_supports_cancellation()
}

pub use registry::{
    CommandAvailability, CommandPlatform, ShortcutBinding, ShortcutChord, ShortcutContext,
    ShortcutKind,
};

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
    ImportNetlist,
    ImportVerilogA,
    ExportSchematicSvg,
    ExportWaveformsCsv,
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
    Preferences,
    ZoomIn,
    ZoomOut,
    ZoomFit,
    ZoomOneToOne,
    CycleGrid,
    ToggleFullScreen,
    ResetActiveView,
    ToggleNavigator,
    ToggleInspector,
    ToggleConsole,
    OpenConsole,
    OpenProblems,
    ToggleConsoleMaximized,
    ClearConsole,
    ToggleFocusMode,
    ResetLayout,
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
    SymbolPinTool,
    SymbolPolylineTool,
    SymbolCircleTool,
    SymbolArcTool,
    SymbolArrowTool,
    SymbolDotTool,
    Place(ComponentType),
    RotateSelection,
    MirrorSelectionHorizontal,
    MirrorSelectionVertical,
    Cancel,
    AscendHierarchy,
    DescendHierarchy,
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
    PdkSettings,
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
            Self::ToggleFullScreen => spec("full-screen", "Enter full screen", "View"),
            Self::ResetActiveView => spec("reset-active-view", "Reset active view", "View"),
            Self::ToggleNavigator => spec("toggle-navigator", "Toggle navigator", "Window"),
            Self::ToggleInspector => spec("toggle-inspector", "Toggle inspector", "Window"),
            Self::ToggleConsole => spec("console", "Toggle console", "Window"),
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
            Self::SymbolPinTool => spec("symbol-pin-tool", "Place symbol pin", "Design"),
            Self::SymbolPolylineTool => {
                spec("symbol-polyline-tool", "Draw symbol polyline", "Design")
            }
            Self::SymbolCircleTool => spec("symbol-circle-tool", "Draw symbol circle", "Design"),
            Self::SymbolArcTool => spec("symbol-arc-tool", "Draw symbol arc", "Design"),
            Self::SymbolArrowTool => spec("symbol-arrow-tool", "Draw symbol arrow", "Design"),
            Self::SymbolDotTool => spec("symbol-dot-tool", "Place symbol dot", "Design"),
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
                "Mirror horizontally",
                "Design",
            ),
            Self::MirrorSelectionVertical => {
                spec("mirror-selection-vertical", "Mirror vertically", "Design")
            }
            Self::Cancel => spec("cancel-active-command", "Cancel active command", "Design"),
            Self::AscendHierarchy => spec("ascend-hierarchy", "Ascend hierarchy", "Design"),
            Self::DescendHierarchy => spec(
                "descend-hierarchy",
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
            Self::ResultViewer(crate::workbench::ResultViewer::Specs) => spec(
                "result-specifications",
                "Open specification results",
                "Results",
            ),
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
            Self::ModelsPage(ModelsPage::Include) => {
                spec("include-graph", "Include graph", "Models")
            }
            Self::ModelsPage(ModelsPage::Qualification) => {
                spec("model-metadata-audit", "Metadata audit", "Models")
            }
            Self::ModelBrowser => spec("model-browser", "Model browser…", "Models"),
            Self::PdkSettings => spec("pdk-settings", "PDK and model paths…", "Models"),
            Self::CompileVerilogA => spec("veriloga", "Verilog-A/AMS compiler", "Models"),
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
            Self::About => spec("about", "About RSpice", "Help"),
        }
    }

    pub fn is_enabled(self, app: &RSpiceApp) -> bool {
        let state = &app.state;
        if crate::common::project_lifecycle::operation_in_progress(state)
            && self.blocked_by_project_operation()
        {
            return false;
        }
        if self.requires_open_project() && !state.project_lifecycle.project_open {
            return false;
        }
        match self {
            Self::OpenWorkspace(workspace) => {
                workspace_available(state.project_lifecycle.project_open, workspace)
            }
            Self::Save => {
                state.project_lifecycle.project_open
                    || state.schematic.current_file.is_some()
                    || state.browser_schematic_save_name.is_some()
            }
            Self::SaveAs => state.project_lifecycle.project_open,
            Self::SaveAll => {
                state.project_lifecycle.project_open
                    && crate::common::project_lifecycle::has_unsaved_changes(state)
            }
            Self::RevertActiveDocument => {
                state.project_lifecycle.accepted().is_some()
                    && crate::common::project_lifecycle::active_document_is_dirty(state)
                    && !state.simulation.is_running
            }
            Self::CloseActiveDocument => {
                crate::common::project_lifecycle::can_close_active_document(state)
            }
            Self::CloseProject => state.project_lifecycle.project_open,
            Self::Undo => {
                if active_symbol_editor(app) {
                    state.can_undo_active_symbol_document()
                } else {
                    active_schematic_editor(app) && state.schematic.can_undo()
                }
            }
            Self::Redo => {
                if active_symbol_editor(app) {
                    state.can_redo_active_symbol_document()
                } else {
                    active_schematic_editor(app) && state.schematic.can_redo()
                }
            }
            Self::Cut | Self::Delete => {
                if active_symbol_editor(app) {
                    !state.active_view_read_only()
                        && !state.ui.symbol.effective_selection().is_empty()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic.read_only
                        && schematic_selection_has_live_object(&state.schematic)
                }
            }
            Self::Duplicate => {
                if active_symbol_editor(app) {
                    !state.active_view_read_only()
                        && !state.ui.symbol.effective_selection().is_empty()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic.read_only
                        && schematic_selection_has_duplicable_object(&state.schematic)
                }
            }
            Self::Copy => {
                if state.workbench.workspace == Workspace::Results {
                    state.ui.results.cursors.a.is_some()
                } else if active_symbol_editor(app) {
                    !state.ui.symbol.effective_selection().is_empty()
                } else {
                    active_schematic_editor(app)
                        && schematic_selection_has_live_object(&state.schematic)
                }
            }
            Self::Paste => {
                if active_symbol_editor(app) {
                    !state.active_view_read_only() && !state.ui.symbol.clipboard.is_empty()
                } else {
                    active_schematic_editor(app)
                        && !state.schematic.read_only
                        && !state.schematic.clipboard.is_empty()
                }
            }
            Self::SelectAll => active_symbol_editor(app) || active_schematic_editor(app),
            Self::RenameSelection => {
                active_schematic_editor(app)
                    && crate::common::app::rename_selection_available(state)
            }
            Self::RotateSelection
            | Self::MirrorSelectionHorizontal
            | Self::MirrorSelectionVertical => {
                active_schematic_editor(app)
                    && !state.schematic.read_only
                    && state.schematic.selection.components.iter().any(|id| {
                        state
                            .schematic
                            .components
                            .iter()
                            .any(|component| component.id == *id)
                    })
            }
            Self::MoveSelection => {
                active_schematic_editor(app)
                    && !state.schematic.read_only
                    && !state.active_view_read_only()
                    && state.schematic.has_live_movable_selection()
            }
            Self::StretchSelection => {
                active_schematic_editor(app)
                    && !state.schematic.read_only
                    && !state.active_view_read_only()
                    && state.schematic.default_stretch_target().is_some()
            }
            Self::ArraySelection => {
                active_schematic_editor(app)
                    && !state.schematic.read_only
                    && !state.active_view_read_only()
                    && state.schematic.validate_array_source_selection().is_ok()
            }
            Self::ObjectProperties => {
                if active_symbol_editor(app) {
                    let selection = state.ui.symbol.effective_selection();
                    selection.pins.len() + selection.shapes.len() == 1
                } else {
                    active_schematic_editor(app)
                        && crate::common::app::selected_object_properties_available(state)
                }
            }
            Self::ZoomFit => {
                active_symbol_editor(app)
                    || active_schematic_editor(app)
                    || state.workbench.workspace == Workspace::Results
            }
            Self::ZoomIn | Self::ZoomOut | Self::ZoomOneToOne => {
                active_symbol_editor(app) || active_schematic_editor(app)
            }
            Self::ResetActiveView => reset_active_view_available(state.workbench.workspace),
            Self::CycleGrid => active_symbol_editor(app) || active_schematic_editor(app),
            Self::SelectTool => active_symbol_editor(app) || active_schematic_editor(app),
            Self::PlaceWire
            | Self::PlaceBus
            | Self::PlaceBusTap
            | Self::PlaceJunction
            | Self::PlaceProbe
            | Self::PlacePin
            | Self::PlaceText
            | Self::PlaceShape => {
                active_schematic_editor(app)
                    && !state.schematic.read_only
                    && !state.active_view_read_only()
            }
            Self::PlaceInstance | Self::PlaceLabel | Self::Place(_) => {
                active_schematic_editor(app)
                    && !state.schematic.read_only
                    && !state.active_view_read_only()
            }
            Self::SymbolPinTool
            | Self::SymbolPolylineTool
            | Self::SymbolCircleTool
            | Self::SymbolArcTool
            | Self::SymbolArrowTool
            | Self::SymbolDotTool => active_symbol_editor(app) && !state.active_view_read_only(),
            Self::AscendHierarchy => {
                active_schematic_editor(app) && state.workspace.hierarchy_stack.len() > 1
            }
            Self::DescendHierarchy => {
                active_schematic_editor(app)
                    && state.schematic.selection.single_component().is_some()
            }
            Self::RunChecks | Self::CheckAndSave => active_schematic_editor(app),
            Self::ClearChecks => state.dialogs.drc_results.is_some(),
            Self::NextViolation | Self::PreviousViolation => state.dialogs.drc_results.is_some(),
            Self::RunSimulation => {
                if state.workbench.workspace == Workspace::Netlist {
                    app.manual_deck_run_block_reason().is_none()
                } else {
                    state.can_run_simulation()
                }
            }
            Self::FindCodeDocument => state.workbench.workspace == Workspace::Netlist,
            Self::ValidateCodeDocument => {
                state.workbench.workspace == Workspace::Netlist
                    && state.ui.netlist.active_document
                        != super::netlist_document::ActiveNetlistDocument::GeneratedDiff
                    && !state.simulation.netlist_content.trim().is_empty()
            }
            Self::CompareGeneratedRevisions => {
                state.workbench.workspace == Workspace::Netlist
                    && !state.ui.netlist.generated_history.is_empty()
                    && state.ui.netlist.generated_document.is_some()
            }
            Self::StopSimulation => stop_simulation_enabled(state.simulation.is_running),
            Self::ClearResults => {
                state.simulation.has_results()
                    && state.simulation.active_execution.is_none()
                    && !state.simulation.is_running
            }
            Self::ToggleLinkedCursors => {
                state.workbench.workspace == Workspace::Results && state.simulation.has_results()
            }
            Self::VisualizationStudio => state.project_lifecycle.project_open,
            // The source-authoring executor is intentionally dormant until
            // exact plot-artwork publication and release handoff complete the
            // mockup's full Report Authoring contract.
            Self::ReportAuthoring
            | Self::SaveReportDocument
            | Self::AddReportPage
            | Self::ReportPageProperties => false,
            Self::AddVisualizationPane
            | Self::VisualizationTraceManager
            | Self::VisualizationCursorManager
            | Self::ExportVisualizationDocument => {
                state.workbench.current_route().surface_id()
                    == super::SurfaceId::VisualizationStudio
                    && state.project_lifecycle.project_open
                    && state.simulation.has_results()
            }
            Self::VisualizationDocumentProperties => {
                state.workbench.current_route().surface_id()
                    == super::SurfaceId::VisualizationStudio
                    && state.project_lifecycle.project_open
            }
            Self::ExportWaveformsCsv => state.simulation.has_results(),
            Self::VerificationPage(page) if !page.is_operational() => false,
            Self::ClearConsole => {
                !state.log_buffer.is_empty() || !state.script_console.history.is_empty()
            }
            _ => true,
        }
    }

    pub fn execute(self, app: &mut RSpiceApp) {
        if crate::common::project_lifecycle::operation_in_progress(&app.state)
            && self.blocked_by_project_operation()
        {
            app.state
                .push_user_message(crate::common::app::ConsoleMessage::warning(
                    "Wait for the current project operation to finish before starting another.",
                ));
            return;
        }
        if self.requires_open_project() && !app.state.project_lifecycle.project_open {
            app.state
                .push_user_message(crate::common::app::ConsoleMessage::warning(
                    "Open a project before using this command.",
                ));
            return;
        }
        match self {
            Self::OpenWorkspace(workspace) => {
                if workspace_available(app.state.project_lifecycle.project_open, workspace) {
                    activate_workspace(app, workspace);
                }
            }
            Self::ProjectLauncher => app.state.workbench.open_project_launcher(),
            Self::RecentProjects => open_recent_projects(&mut app.state.workbench),
            Self::NewProject => file_action(app, FileMenuAction::NewProject),
            Self::OpenProject => file_action(app, FileMenuAction::OpenProject),
            Self::Save => {
                if app.state.workbench.workspace == Workspace::Netlist
                    && app.state.ui.netlist.active_document
                        == super::netlist_document::ActiveNetlistDocument::OwnedSource
                {
                    app.state.ui.netlist.save_dialog.open = true;
                    app.state.ui.netlist.save_dialog.error = None;
                } else {
                    file_action(app, FileMenuAction::Save);
                }
            }
            Self::SaveAs => file_action(app, FileMenuAction::SaveProjectAs),
            Self::SaveAll => file_action(app, FileMenuAction::SaveAll),
            Self::RevertActiveDocument => file_action(app, FileMenuAction::RevertActiveDocument),
            Self::CloseActiveDocument => file_action(app, FileMenuAction::CloseActiveDocument),
            Self::CloseProject => file_action(app, FileMenuAction::CloseProject),
            Self::NewCell => open_new_cell_dialog(app),
            Self::OpenDocument => file_action(app, FileMenuAction::Open),
            Self::ImportNetlist => {
                file_action(app, FileMenuAction::ImportNetlist);
                activate_workspace(app, Workspace::Netlist);
            }
            Self::ImportVerilogA => file_action(app, FileMenuAction::ImportVerilogA),
            Self::ExportSchematicSvg => file_action(app, FileMenuAction::ExportSvg),
            Self::ExportWaveformsCsv => file_action(app, FileMenuAction::ExportCsvWaveforms),
            Self::ExportNetlist(format) => {
                if app.state.workbench.workspace == Workspace::Netlist {
                    app.state.ui.netlist.export_dialog.open = true;
                    app.state.ui.netlist.export_dialog.format = format;
                    app.state.ui.netlist.export_dialog.error = None;
                } else {
                    crate::common::menu_bar::action_export_netlist_with_io(
                        &mut app.state,
                        format,
                        app.export_workflow_io.as_ref(),
                    );
                }
            }
            Self::Exit => file_action(app, FileMenuAction::Exit),
            Self::Undo => {
                if active_symbol_editor(app) {
                    if let Err(error) = app.state.undo_active_symbol_document() {
                        app.state
                            .push_user_message(crate::common::app::ConsoleMessage::warning(error));
                    }
                } else {
                    if app.state.schematic.undo() {
                        app.state.sync_active_schematic_to_workspace();
                    }
                }
            }
            Self::Redo => {
                if active_symbol_editor(app) {
                    if let Err(error) = app.state.redo_active_symbol_document() {
                        app.state
                            .push_user_message(crate::common::app::ConsoleMessage::warning(error));
                    }
                } else {
                    if app.state.schematic.redo() {
                        app.state.sync_active_schematic_to_workspace();
                    }
                }
            }
            Self::Cut => {
                if active_symbol_editor(app) {
                    app.delete_selected_symbol_item(true);
                } else {
                    app.state.schematic.copy_selection();
                    app.state.schematic.delete_selection();
                }
            }
            Self::Copy => {
                if app.state.workbench.workspace == Workspace::Results {
                    if let Some(text) = super::result_document::copy_cursor_text(&mut app.state) {
                        app.state.ui.clipboard_text_request = Some(text);
                    }
                } else if active_symbol_editor(app) {
                    app.copy_selected_symbol_shape();
                } else {
                    app.state.schematic.copy_selection();
                }
            }
            Self::Paste => {
                if active_symbol_editor(app) {
                    app.paste_symbol_shape();
                } else {
                    let anchor = app.state.schematic_paste_anchor();
                    if !app.state.schematic.paste_at(anchor) {
                        app.state
                            .push_user_message(crate::common::app::ConsoleMessage::warning(
                                "Paste could not be completed at the current canvas target",
                            ));
                    }
                }
            }
            Self::Duplicate => {
                if active_symbol_editor(app) {
                    app.copy_selected_symbol_shape();
                    app.paste_symbol_shape();
                } else {
                    app.state.schematic.copy_selection();
                    let anchor =
                        app.state.schematic_paste_anchor() + crate::state::Point::new(2, 2);
                    if !app.state.schematic.paste_at(anchor) {
                        app.state
                            .push_user_message(crate::common::app::ConsoleMessage::warning(
                                "Duplicate could not be completed at the current canvas target",
                            ));
                    }
                }
            }
            Self::Delete => {
                if active_symbol_editor(app) {
                    app.delete_selected_symbol_item(false);
                } else {
                    app.state.schematic.delete_selection();
                }
            }
            Self::SelectAll => {
                if active_symbol_editor(app) {
                    app.select_all_symbol_items();
                } else {
                    // Keep the menu/shortcut projection on the state layer's
                    // complete object taxonomy. Repeating a hand-maintained
                    // subset here previously made Ctrl+A silently omit buses,
                    // taps, and net labels even though every downstream edit
                    // operation already supported them.
                    app.state.schematic.select_all_objects();
                }
            }
            Self::RenameSelection => {
                crate::common::app::open_selected_object_rename(&mut app.state);
            }
            Self::ObjectProperties => {
                if active_symbol_editor(app) {
                    app.state.workbench.inspector_visible = true;
                    app.state.workbench.drawer = Some(super::state::Drawer::Inspector);
                } else {
                    crate::common::app::open_selected_object_properties(&mut app.state);
                }
            }
            Self::FindInDesign => {
                if app.state.workbench.workspace == Workspace::Netlist {
                    app.state.ui.netlist.find.open = true;
                } else {
                    activate_workspace(app, Workspace::Design);
                    app.state.workbench.navigator_visible = true;
                    app.state.workbench.drawer = Some(super::state::Drawer::Navigator);
                    app.state.workbench.focus_navigator_search = true;
                }
            }
            Self::Preferences => {
                let route = super::SurfaceRoute::surface(super::SurfaceId::Preferences);
                if let Err(error) = app
                    .state
                    .workbench
                    .navigate(route, super::RouteTransitionSource::User)
                {
                    app.state
                        .push_user_message(crate::common::app::ConsoleMessage::warning(
                            error.to_string(),
                        ));
                }
            }
            Self::ZoomIn => {
                if active_symbol_editor(app) {
                    app.state.ui.symbol.zoom = (app.state.ui.symbol.zoom * 1.25).min(16.0);
                } else {
                    app.state.schematic.zoom = (app.state.schematic.zoom * 1.25).min(8.0);
                }
            }
            Self::ZoomOut => {
                if active_symbol_editor(app) {
                    app.state.ui.symbol.zoom = (app.state.ui.symbol.zoom / 1.25).max(0.1);
                } else {
                    app.state.schematic.zoom = (app.state.schematic.zoom / 1.25).max(0.1);
                }
            }
            Self::ZoomFit => {
                if app.state.workbench.workspace == Workspace::Results {
                    let viewer = app.state.ui.results.viewer;
                    app.state.ui.results.reset_plot_view(viewer, 0);
                } else if active_symbol_editor(app) {
                    app.state.ui.symbol.needs_fit = true;
                } else {
                    app.state.schematic.needs_fit = true;
                }
            }
            Self::ZoomOneToOne => {
                if active_symbol_editor(app) {
                    app.state.ui.symbol.zoom = 1.0;
                } else {
                    app.state.schematic.zoom = 1.0;
                }
            }
            Self::CycleGrid => app.state.ui.grid = app.state.ui.grid.cycled(),
            Self::ToggleFullScreen => {
                let enabled = !app.state.workbench.full_screen;
                app.state.workbench.full_screen = enabled;
                app.state.ui.request_full_screen(enabled);
            }
            Self::ResetActiveView => {
                if reset_active_view_available(app.state.workbench.workspace) {
                    reset_active_view(app);
                }
            }
            Self::ToggleNavigator => {
                if app.state.workbench.navigator_visible {
                    app.state.workbench.dismiss_navigator();
                } else {
                    app.state.workbench.navigator_visible = true;
                }
            }
            Self::ToggleInspector => {
                if app.state.workbench.inspector_visible {
                    app.state.workbench.dismiss_inspector();
                } else {
                    app.state.workbench.inspector_visible = true;
                }
            }
            Self::ToggleConsole => {
                if app.state.workbench.focus_mode {
                    app.state.workbench.focus_mode = false;
                    app.state.workbench.console_visible = true;
                } else {
                    app.state.workbench.console_visible = !app.state.workbench.console_visible;
                }
                if !app.state.workbench.console_visible {
                    app.state.workbench.console_maximized = false;
                }
            }
            Self::OpenProblems => {
                app.state.workbench.focus_mode = false;
                app.state.workbench.console_visible = true;
                app.state.workbench.console_maximized = false;
                app.state.workbench.console_page = super::state::ConsolePage::Problems;
            }
            Self::OpenConsole => {
                app.state.workbench.focus_mode = false;
                app.state.workbench.console_visible = true;
                app.state.workbench.console_maximized = false;
            }
            Self::ToggleConsoleMaximized => {
                app.state.workbench.focus_mode = false;
                app.state.workbench.console_maximized = !app.state.workbench.console_maximized;
                app.state.workbench.console_visible = true;
            }
            Self::ClearConsole => {
                app.state.clear_primary_log();
                app.state.script_console.history.clear();
            }
            Self::ToggleFocusMode => {
                app.state.workbench.focus_mode = !app.state.workbench.focus_mode;
                if app.state.workbench.focus_mode {
                    app.state.workbench.close_drawer();
                }
            }
            Self::ResetLayout => app.state.workbench.reset_layout(),
            Self::PreviousWorkspace => app.state.workbench.cycle_workspace(true),
            Self::NextWorkspace => app.state.workbench.cycle_workspace(false),
            Self::SelectTool => {
                if active_symbol_editor(app) {
                    app.state.ui.symbol.tool = crate::workbench::SymbolTool::Select;
                } else {
                    set_tool(app, Tool::Select);
                }
            }
            Self::PlaceInstance => {
                activate_workspace(app, Workspace::Design);
                app.state.workbench.navigator_visible = true;
                app.state.workbench.drawer = Some(super::state::Drawer::Navigator);
                app.state.workbench.design_panel = super::state::DesignPanel::ComponentShelf;
                app.state.workbench.focus_placement_search = true;
            }
            Self::PlaceWire => set_tool(app, Tool::Wire),
            Self::PlaceBus => set_tool(app, Tool::Bus),
            Self::PlaceBusTap => {
                activate_workspace(app, Workspace::Design);
                app.state.dialogs.bus_tap.open();
            }
            Self::PlaceJunction => set_tool(app, Tool::Junction),
            Self::PlaceLabel => set_tool(app, Tool::Label),
            Self::PlaceProbe => set_tool(app, Tool::Probe),
            Self::PlacePin => {
                activate_workspace(app, Workspace::Design);
                let name = app.state.schematic.suggested_port_name("BIAS_EN");
                let design_execution_epoch = app.state.design_execution_epoch;
                let active_schematic_epoch = app.state.active_schematic_epoch;
                let topology_version = app.state.schematic.topology_version();
                let view_path = app.state.workspace.active_view.display_path();
                app.state.dialogs.pin_port.open(
                    name,
                    design_execution_epoch,
                    active_schematic_epoch,
                    topology_version,
                    view_path,
                );
            }
            Self::PlaceText => {
                activate_workspace(app, Workspace::Design);
                let design_execution_epoch = app.state.design_execution_epoch;
                let active_schematic_epoch = app.state.active_schematic_epoch;
                let topology_version = app.state.schematic.topology_version();
                let view_path = app.state.workspace.active_view.display_path();
                app.state.dialogs.design_note.open(
                    design_execution_epoch,
                    active_schematic_epoch,
                    topology_version,
                    view_path,
                );
            }
            Self::PlaceShape => {
                activate_workspace(app, Workspace::Design);
                let design_execution_epoch = app.state.design_execution_epoch;
                let active_schematic_epoch = app.state.active_schematic_epoch;
                let topology_version = app.state.schematic.topology_version();
                let view_path = app.state.workspace.active_view.display_path();
                let expected_shapes = app.state.schematic.documentation_shapes.clone();
                app.state.dialogs.documentation_shape.open(
                    design_execution_epoch,
                    active_schematic_epoch,
                    topology_version,
                    view_path,
                    expected_shapes,
                );
            }
            Self::MoveSelection => {
                crate::common::app::open_move_selection_dialog(&mut app.state);
            }
            Self::StretchSelection => {
                crate::common::app::open_stretch_selection_dialog(&mut app.state);
            }
            Self::ArraySelection => {
                crate::common::app::open_array_selection_dialog(&mut app.state);
            }
            Self::SymbolPinTool => {
                app.state.ui.symbol.tool = super::SymbolTool::PlacePin;
                let next = app
                    .state
                    .load_active_symbol_document()
                    .ok()
                    .and_then(|document| {
                        document
                            .pins
                            .iter()
                            .find(|pin| pin.position.is_none())
                            .map(|pin| pin.name.clone())
                    });
                if let Some(pin) = next {
                    app.state.ui.symbol.select_pin(pin);
                } else {
                    app.state.ui.symbol.clear_selection();
                }
            }
            Self::SymbolPolylineTool => {
                app.state.ui.symbol.tool = super::SymbolTool::Polyline;
                app.state.ui.symbol.pending_polyline.clear();
            }
            Self::SymbolCircleTool => {
                app.state.ui.symbol.tool = super::SymbolTool::Circle;
                app.state.ui.symbol.shape_start = None;
            }
            Self::SymbolArcTool => {
                app.state.ui.symbol.tool = super::SymbolTool::Arc;
                app.state.ui.symbol.shape_start = None;
            }
            Self::SymbolArrowTool => app.state.ui.symbol.tool = super::SymbolTool::Arrow,
            Self::SymbolDotTool => app.state.ui.symbol.tool = super::SymbolTool::Dot,
            Self::Place(ComponentType::Port) => Self::PlacePin.execute(app),
            Self::Place(kind) => set_tool(app, Tool::Place(kind)),
            Self::RotateSelection => {
                let symbol_context = SchematicSymbolContext::from_state(&app.state);
                app.state.schematic.rotate_selection_resolved(|component| {
                    symbol_context.terminal_points(component)
                });
            }
            Self::MirrorSelectionHorizontal => {
                let symbol_context = SchematicSymbolContext::from_state(&app.state);
                app.state
                    .schematic
                    .mirror_selection_h_resolved(|component| {
                        symbol_context.terminal_points(component)
                    });
            }
            Self::MirrorSelectionVertical => {
                let symbol_context = SchematicSymbolContext::from_state(&app.state);
                app.state
                    .schematic
                    .mirror_selection_v_resolved(|component| {
                        symbol_context.terminal_points(component)
                    });
            }
            Self::Cancel => {
                if app.state.dialogs.move_selection.armed {
                    crate::common::app::cancel_armed_move_selection(&mut app.state);
                } else if app.state.dialogs.stretch_selection.armed {
                    crate::common::app::cancel_armed_stretch_selection(&mut app.state);
                } else if app.state.dialogs.array_selection.armed {
                    crate::common::app::cancel_armed_array_selection(&mut app.state);
                } else if app.state.workbench.drawer.is_some() {
                    app.state.workbench.close_drawer();
                } else if app.state.dialogs.object_properties.open {
                    app.state.dialogs.object_properties.attempt_close();
                } else if app.state.tabbed_property_dialog.open {
                    app.state.tabbed_property_dialog.attempt_close();
                } else if app.state.workbench.workspace == Workspace::Results
                    && app.state.ui.results.cursors.any()
                {
                    app.state.ui.results.clear_cursors();
                } else {
                    cancel_schematic_tool(&mut app.state.schematic);
                    app.state.schematic.selection.clear();
                    app.state.schematic.selection_rect.cancel();
                }
            }
            Self::AscendHierarchy => {
                app.state.ascend_workspace_level();
            }
            Self::DescendHierarchy => {
                app.state.open_selected_instance_master();
            }
            Self::RunChecks => crate::common::menu_bar::run_design_rule_check(&mut app.state),
            Self::CheckAndSave => {
                crate::common::menu_bar::run_design_rule_check(&mut app.state);
                let passed = app
                    .state
                    .dialogs
                    .drc_results
                    .as_ref()
                    .is_some_and(crate::services::drc::DrcResult::passed);
                if passed {
                    file_action(app, FileMenuAction::SaveProject);
                } else {
                    activate_workspace(app, Workspace::Verify);
                    app.state.workbench.verification_page = VerificationPage::Yield;
                    app.state.workbench.console_visible = true;
                    app.state.workbench.console_page = super::state::ConsolePage::Problems;
                }
            }
            Self::ClearChecks => app.state.dialogs.drc_results = None,
            Self::NextViolation => {
                crate::schematic::view::violations::cycle_violation(&mut app.state, 1);
            }
            Self::PreviousViolation => {
                crate::schematic::view::violations::cycle_violation(&mut app.state, -1);
            }
            Self::RunSimulation => {
                if app.state.workbench.workspace == Workspace::Netlist {
                    if app.manual_deck_run_block_reason().is_none() {
                        app.state.request_netlist_manual_deck_run();
                    }
                } else if app.state.can_run_simulation() {
                    app.state.request_run_set_simulation();
                    activate_workspace(app, Workspace::Simulate);
                }
            }
            Self::StopSimulation => {
                if stop_simulation_enabled(app.state.simulation.is_running) {
                    if let Err(error) = app.state.simulation.request_abort_active_run() {
                        app.state
                            .push_sim_message(crate::common::app::ConsoleMessage::warning(error));
                    }
                } else if app.state.simulation.is_running {
                    app.state.push_sim_message(
                        crate::common::app::ConsoleMessage::warning(
                            "This execution target cannot yet guarantee cancellation; the active run was left intact",
                        ),
                    );
                }
            }
            Self::JobsManager => super::jobs_manager::open(app),
            Self::PreflightChecks => super::preflight::run(app),
            Self::SimulationOptions => {
                crate::common::menu_bar::open_simulation_options(&mut app.state)
            }
            Self::GenerateNetlist => {
                crate::common::menu_bar::action_view_netlist(&mut app.state);
                activate_workspace(app, Workspace::Netlist);
            }
            Self::FindCodeDocument => app.state.ui.netlist.find.open = true,
            Self::ValidateCodeDocument => {
                crate::common::netlist_workflow::validate_visible_netlist_source(app);
            }
            Self::CompareGeneratedRevisions => {
                app.state.ui.netlist.comparison_dialog.open = true;
                app.state
                    .ui
                    .netlist
                    .comparison_dialog
                    .selected_history_index = app
                    .state
                    .ui
                    .netlist
                    .generated_history
                    .len()
                    .saturating_sub(1);
            }
            Self::ClearResults => {
                if app.state.simulation.active_execution.is_some()
                    || app.state.simulation.is_running
                {
                    app.state
                        .push_sim_message(crate::common::app::ConsoleMessage::warning(
                        "Result history cannot be cleared while a simulation execution owns a run"
                            .to_owned(),
                    ));
                } else {
                    app.state.clear_simulation_results();
                }
            }
            Self::ToggleLinkedCursors => app.state.ui.results.toggle_linked_cursors(),
            Self::WaveformCalculator => app.state.dialogs.waveform_calculator_dialog = true,
            Self::ResultViewer(viewer) => {
                app.state.ui.results.viewer = viewer;
                activate_workspace(app, Workspace::Results);
            }
            Self::EditSpecifications => {
                app.state.ui.results.viewer = crate::workbench::ResultViewer::Specs;
                crate::workbench::result_document::open_specification_editor(&mut app.state);
                activate_workspace(app, Workspace::Results);
            }
            Self::VerificationPage(VerificationPage::Drc) => {
                app.state.push_user_message(
                    crate::common::app::ConsoleMessage::warning(
                        "Physical DRC is unavailable until a retained layout source, qualified rule deck, and immutable marker database are integrated.",
                    ),
                );
            }
            Self::VerificationPage(page) => {
                activate_workspace(app, Workspace::Verify);
                app.state.workbench.verification_page = page;
            }
            Self::ProjectPage(page) => {
                activate_workspace(app, Workspace::Project);
                app.state.workbench.project_page = page;
            }
            Self::ModelsPage(page) => {
                activate_workspace(app, Workspace::Models);
                app.state.workbench.models_page = page;
            }
            Self::ModelBrowser => app.state.model_browser_state.open = true,
            Self::PdkSettings => app
                .state
                .pdk_settings_dialog
                .open(app.state.pdk_config.clone()),
            Self::CompileVerilogA => {
                activate_workspace(app, Workspace::Netlist);
                app.state.ui.code_workspace.page =
                    super::code_workspace::CodeWorkspacePage::VerilogA;
            }
            Self::AutomationConsole => {
                activate_workspace(app, Workspace::Netlist);
                app.state.ui.code_workspace.page =
                    super::code_workspace::CodeWorkspacePage::Automation;
            }
            Self::CommandPalette => app.state.dialogs.command_palette.open(),
            Self::KeyboardShortcuts => app.state.dialogs.shortcuts_help = true,
            Self::AccountOrganization => super::account_organization::open(app),
            Self::License => app.open_license_dialog(),
            Self::SpecialistToolBrowser => super::specialist_tool_browser::open(app),
            Self::VisualizationStudio => super::visualization_studio::open(app),
            Self::ReportAuthoring => super::surfaces::report_authoring::open(app),
            Self::SaveReportDocument => super::surfaces::report_authoring::save_document(app),
            Self::AddReportPage => super::surfaces::report_authoring::open_add_page(app),
            Self::ReportPageProperties => {
                super::surfaces::report_authoring::open_page_properties(app);
            }
            Self::AddVisualizationPane => super::visualization_studio::open_add_pane(app),
            Self::VisualizationTraceManager => {
                super::visualization_studio::open_trace_manager(app);
            }
            Self::VisualizationCursorManager => {
                super::visualization_studio::open_cursor_manager(app);
            }
            Self::VisualizationDocumentProperties => {
                super::visualization_studio::open_document_properties(app);
            }
            Self::ExportVisualizationDocument => {
                super::visualization_studio::export_document(app);
            }
            Self::FeatureAvailability => {
                let route = super::SurfaceRoute::surface(super::SurfaceId::FeatureAvailability);
                if let Err(error) = app
                    .state
                    .workbench
                    .navigate(route, super::RouteTransitionSource::User)
                {
                    app.state
                        .push_user_message(crate::common::app::ConsoleMessage::warning(
                            error.to_string(),
                        ));
                }
            }
            Self::InteroperabilityMatrix => {
                let route = super::SurfaceRoute::capability_workflow(
                    super::CapabilityWorkflowId::InteroperabilityMatrix,
                );
                if let Err(error) = app
                    .state
                    .workbench
                    .navigate(route, super::RouteTransitionSource::User)
                {
                    app.state
                        .push_user_message(crate::common::app::ConsoleMessage::warning(
                            error.to_string(),
                        ));
                }
            }
            Self::About => app.state.dialogs.about = true,
        }
    }
}

const fn spec(id: &'static str, label: &'static str, group: &'static str) -> CommandSpec {
    CommandSpec { id, label, group }
}

fn activate_workspace(app: &mut RSpiceApp, workspace: Workspace) {
    app.state.workbench.activate(workspace);
}

const fn workspace_available(project_open: bool, workspace: Workspace) -> bool {
    project_open || matches!(workspace, Workspace::Project)
}

fn active_symbol_editor(app: &RSpiceApp) -> bool {
    matches!(
        app.state.workbench.workspace,
        Workspace::Design | Workspace::Models
    ) && app.state.workspace.active_view_type() == crate::state::ViewType::Symbol
}

fn active_schematic_editor(app: &RSpiceApp) -> bool {
    app.state.workbench.workspace == Workspace::Design
        && matches!(
            app.state.workspace.active_view_type(),
            crate::state::ViewType::Schematic | crate::state::ViewType::Testbench
        )
}

fn schematic_selection_has_live_object(schematic: &crate::state::SchematicState) -> bool {
    let selection = &schematic.selection;
    schematic
        .components
        .iter()
        .any(|component| selection.has_component(component.id))
        || schematic
            .wires
            .iter()
            .any(|wire| selection.has_wire(wire.id))
        || schematic
            .junctions
            .iter()
            .any(|junction| selection.has_junction(junction.pos))
        || schematic
            .net_labels
            .iter()
            .any(|label| selection.has_net_label(label.id))
        || schematic.buses.iter().any(|bus| selection.has_bus(bus.id))
        || schematic
            .bus_taps
            .iter()
            .any(|tap| selection.has_bus_tap(tap.id))
        || schematic
            .design_notes
            .iter()
            .any(|note| selection.has_design_note(note.id))
        || schematic
            .documentation_shapes
            .iter()
            .any(|shape| selection.has_documentation_shape(shape.id))
}

fn schematic_selection_has_duplicable_object(schematic: &crate::state::SchematicState) -> bool {
    let selection = &schematic.selection;
    selection.wire_segments.is_empty()
        && selection.wire_vertices.is_empty()
        && (schematic
            .components
            .iter()
            .any(|component| selection.has_component(component.id))
            || schematic
                .wires
                .iter()
                .any(|wire| selection.has_wire(wire.id))
            || schematic
                .net_labels
                .iter()
                .any(|label| selection.has_net_label(label.id))
            || schematic.buses.iter().any(|bus| selection.has_bus(bus.id))
            || schematic
                .bus_taps
                .iter()
                .any(|tap| selection.has_bus_tap(tap.id))
            || schematic
                .design_notes
                .iter()
                .any(|note| selection.has_design_note(note.id))
            || schematic
                .documentation_shapes
                .iter()
                .any(|shape| selection.has_documentation_shape(shape.id)))
}

fn set_tool(app: &mut RSpiceApp, tool: Tool) {
    activate_workspace(app, Workspace::Design);
    if app.state.dialogs.move_selection.armed && tool != Tool::MoveSelection {
        app.state.dialogs.move_selection.close();
    }
    if app.state.dialogs.stretch_selection.armed && tool != Tool::StretchSelection {
        app.state.dialogs.stretch_selection.close();
    }
    if app.state.dialogs.array_selection.armed && tool != Tool::ArraySelection {
        app.state.dialogs.array_selection.close();
    }
    arm_schematic_tool(&mut app.state.schematic, tool);
}

/// Arm one schematic tool through the single conductor-lifecycle boundary.
/// Switching tools cancels every incompatible unfinished route, and leaving
/// typed placement retires every incompatible runtime configuration.
pub(crate) fn arm_schematic_tool(schematic: &mut crate::state::SchematicState, tool: Tool) {
    if schematic.tool != tool {
        schematic.cancel_routing_gestures();
    }
    if tool != Tool::BusTap {
        schematic.pending_bus_tap = None;
    }
    if tool != Tool::Place(ComponentType::Port) {
        schematic.pending_port = None;
    }
    if tool != Tool::DesignNote {
        schematic.pending_design_note = None;
    }
    if tool != Tool::DocumentationShape {
        schematic.pending_documentation_shape = None;
        schematic.documentation_shape_drawing.clear();
    }
    schematic.tool = tool;
}

/// Escape/cancel is stronger than re-arming Select: it also clears any
/// inconsistent hidden route restored from legacy or interrupted state.
pub(crate) fn cancel_schematic_tool(schematic: &mut crate::state::SchematicState) {
    schematic.cancel_routing_gestures();
    schematic.pending_bus_tap = None;
    schematic.pending_port = None;
    schematic.pending_design_note = None;
    schematic.pending_documentation_shape = None;
    schematic.documentation_shape_drawing.clear();
    schematic.tool = Tool::Select;
}

fn open_recent_projects(workbench: &mut WorkbenchState) {
    workbench.open_project_launcher();
    workbench.project_launcher_filter = ProjectLauncherFilter::Recent;
}

const fn reset_active_view_available(workspace: Workspace) -> bool {
    !matches!(workspace, Workspace::Project)
}

fn reset_active_view(app: &mut RSpiceApp) {
    app.state.workbench.navigator_query.clear();
    match app.state.workbench.workspace {
        Workspace::Project => {}
        Workspace::Design => {
            if app.state.workspace.active_view_type() == crate::state::ViewType::Symbol {
                app.state.ui.symbol.needs_fit = true;
            } else {
                app.state.schematic.needs_fit = true;
            }
        }
        Workspace::Simulate => {
            app.state.workbench.analysis_query.clear();
        }
        Workspace::Results => {
            let viewer = app.state.ui.results.viewer;
            app.state
                .ui
                .results
                .views
                .retain(|(candidate, _), _| *candidate != viewer);
            app.state.ui.results.clear_cursors();
            app.state.ui.results.rf_pin.remove(&viewer);
            if viewer == crate::workbench::ResultViewer::Waves {
                app.state.ui.results.hidden_strips.clear();
                app.state.ui.results.maximized_strip = None;
            }
        }
        Workspace::Verify => {
            app.state.workbench.selected_spec = None;
        }
        Workspace::Models => {
            app.state.workbench.selected_model = None;
        }
        Workspace::Netlist => {
            app.state.ui.netlist.cursor_line = 0;
            app.state.ui.netlist.completion_open = false;
            app.state.ui.netlist.completion_index = 0;
        }
    }
}

fn open_new_cell_dialog(app: &mut RSpiceApp) {
    let selected = app
        .state
        .library_manager
        .selected_library
        .as_ref()
        .and_then(|name| {
            app.state
                .library_manager
                .get_library(name)
                .filter(|library| !library.read_only)
                .map(|library| library.name.clone())
        });
    let target_library = selected.or_else(|| {
        app.state
            .library_manager
            .libraries_sorted()
            .into_iter()
            .find(|library| !library.read_only)
            .map(|library| library.name.clone())
    });

    let dialogs = &mut app.state.dialogs;
    dialogs.new_cell_library = target_library.unwrap_or_default();
    dialogs.new_cell_name.clear();
    dialogs.new_cell_description.clear();
    dialogs.new_cell_create_schematic = true;
    dialogs.new_cell_create_symbol = false;
    dialogs.new_cell_create_testbench = false;
    dialogs.new_cell_error = None;
    dialogs.new_cell_dialog = true;
}

fn file_action(app: &mut RSpiceApp, action: FileMenuAction) {
    dispatch_file_menu_action(
        &mut app.state,
        action,
        app.file_workflow_io.as_ref(),
        app.export_workflow_io.as_ref(),
    );
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
    Command::ImportNetlist,
    Command::ImportVerilogA,
    Command::ExportSchematicSvg,
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
    Command::ZoomIn,
    Command::ZoomOut,
    Command::ZoomFit,
    Command::ZoomOneToOne,
    Command::CycleGrid,
    Command::ToggleFullScreen,
    Command::ResetActiveView,
    Command::ToggleNavigator,
    Command::ToggleInspector,
    Command::ToggleConsole,
    Command::OpenConsole,
    Command::OpenProblems,
    Command::ToggleConsoleMaximized,
    Command::ClearConsole,
    Command::ToggleFocusMode,
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
    Command::SymbolPinTool,
    Command::SymbolPolylineTool,
    Command::SymbolCircleTool,
    Command::SymbolArcTool,
    Command::SymbolArrowTool,
    Command::SymbolDotTool,
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
    Command::AscendHierarchy,
    Command::DescendHierarchy,
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
    Command::ModelsPage(ModelsPage::Include),
    Command::ModelsPage(ModelsPage::Qualification),
    Command::ModelBrowser,
    Command::PdkSettings,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn app_with_selected_authored_symbol() -> RSpiceApp {
        use crate::state::{
            Cell, Component, Library, LibraryCellInstance, Point, PortDirection, PortSpec,
            SymbolDocument, SymbolPin, View, ViewType, Wire,
        };

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;

        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
            ],
            ..SymbolDocument::default()
        };
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document
            .store_in_view(&mut symbol_view)
            .expect("authored symbol stores");
        let mut cell = Cell::new("amp");
        cell.add_view(symbol_view);
        let mut library = Library::new("command_test");
        library.add_cell(cell);
        app.state.library_manager.add_library(library);

        let interface = [
            PortSpec {
                name: "IN".to_owned(),
                direction: PortDirection::In,
            },
            PortSpec {
                name: "OUT".to_owned(),
                direction: PortDirection::Out,
            },
        ];
        let mut binding = LibraryCellInstance::new("command_test", "amp", "schematic");
        binding.bind_interface(&interface);
        app.state.schematic.components.push(
            Component::new(701, ComponentType::CellInstance, Point::new(100, 50))
                .with_library_cell(binding),
        );
        app.state
            .schematic
            .wires
            .push(Wire::segment(702, Point::new(60, 40), Point::new(60, 0)));
        app.state.schematic.selection.select_component(701);
        app
    }

    #[test]
    fn transform_commands_keep_wires_attached_to_authored_symbol_pins() {
        use crate::state::{Point, Rotation};

        let cases = [
            (Command::RotateSelection, Point::new(110, 10), Rotation::R90),
            (
                Command::MirrorSelectionHorizontal,
                Point::new(140, 40),
                Rotation::R0,
            ),
            (
                Command::MirrorSelectionVertical,
                Point::new(60, 60),
                Rotation::R0,
            ),
        ];

        for (command, expected_wire_endpoint, expected_rotation) in cases {
            let mut app = app_with_selected_authored_symbol();

            command.execute(&mut app);

            assert_eq!(
                app.state.schematic.wires[0].points[0],
                expected_wire_endpoint
            );
            assert_eq!(
                app.state.schematic.components[0].rotation,
                expected_rotation
            );
        }
    }

    fn app_with_every_complete_schematic_object() -> RSpiceApp {
        use crate::state::{
            Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Component, Junction,
            NetLabel, Point, Wire,
        };

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        let bus = Bus::segment(
            5,
            Point::new(0, 20),
            Point::new(20, 20),
            Some(BusDeclaration::parse("DATA[3:0]").unwrap()),
        )
        .unwrap();
        let tap = BusTap::new(
            6,
            &bus,
            Point::new(10, 20),
            Point::new(10, 30),
            BusSlice::parse("DATA[1]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        app.state.schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::origin(),
        ));
        app.state
            .schematic
            .wires
            .push(Wire::segment(2, Point::new(0, 0), Point::new(20, 0)));
        app.state
            .schematic
            .junctions
            .push(Junction::new(3, Point::new(10, 0)));
        app.state
            .schematic
            .net_labels
            .push(NetLabel::new(4, Point::new(10, 0), "sense_out"));
        app.state.schematic.buses.push(bus);
        app.state.schematic.bus_taps.push(tap);
        app
    }

    #[test]
    fn edit_command_enablement_covers_every_complete_schematic_object_class() {
        let mut app = app_with_every_complete_schematic_object();

        let selectable = [
            ("component", 1_u64),
            ("wire", 2),
            ("net label", 4),
            ("bus", 5),
            ("bus tap", 6),
        ];
        for (kind, id) in selectable {
            app.state.schematic.selection.clear();
            match kind {
                "component" => app.state.schematic.selection.select_component(id),
                "wire" => app.state.schematic.selection.select_wire(id),
                "net label" => app.state.schematic.selection.select_net_label(id),
                "bus" => app.state.schematic.selection.select_bus(id),
                "bus tap" => app.state.schematic.selection.select_bus_tap(id),
                _ => unreachable!(),
            }
            assert!(Command::Copy.is_enabled(&app), "copy disabled for {kind}");
            assert!(Command::Cut.is_enabled(&app), "cut disabled for {kind}");
            assert!(
                Command::Delete.is_enabled(&app),
                "delete disabled for {kind}"
            );
            assert!(
                Command::Duplicate.is_enabled(&app),
                "duplicate disabled for {kind}"
            );
        }

        app.state
            .schematic
            .selection
            .select_only_junction(crate::state::Point::new(10, 0));
        assert!(Command::Copy.is_enabled(&app));
        assert!(Command::Cut.is_enabled(&app));
        assert!(Command::Delete.is_enabled(&app));
        assert!(
            !Command::Duplicate.is_enabled(&app),
            "a fixed-offset duplicate cannot invent a valid junction target"
        );
    }

    #[test]
    fn select_all_command_delegates_to_the_complete_schematic_object_taxonomy() {
        let mut app = app_with_every_complete_schematic_object();

        Command::SelectAll.execute(&mut app);

        let selection = &app.state.schematic.selection;
        assert_eq!(selection.count(), 6);
        assert!(selection.has_component(1));
        assert!(selection.has_wire(2));
        assert!(selection.has_junction(crate::state::Point::new(10, 0)));
        assert!(selection.has_net_label(4));
        assert!(selection.has_bus(5));
        assert!(selection.has_bus_tap(6));
    }

    #[test]
    fn bus_commands_have_stable_mockup_identities() {
        assert_eq!(Command::PlaceBus.stable_id(), "place-bus");
        assert_eq!(Command::PlaceBus.spec().label, "Draw bus");
        assert_eq!(Command::PlaceBusTap.stable_id(), "place-bus-tap");
        assert_eq!(Command::PlaceBusTap.spec().label, "Place bus tap");
        assert_eq!(
            Command::from_stable_id("place-bus"),
            Some(Command::PlaceBus)
        );
        assert_eq!(
            Command::from_stable_id("place-bus-tap"),
            Some(Command::PlaceBusTap)
        );
    }

    #[test]
    fn place_pin_command_has_the_exact_mockup_identity() {
        assert_eq!(Command::PlacePin.stable_id(), "place-pin");
        assert_eq!(Command::PlacePin.spec().label, "Place pin or port\u{2026}");
        assert_eq!(Command::PlacePin.spec().group, "Design");
        assert_eq!(
            Command::from_stable_id("place-pin"),
            Some(Command::PlacePin)
        );
        assert_ne!(Command::PlacePin, Command::SymbolPinTool);
    }

    #[test]
    fn place_text_command_has_the_exact_mockup_identity() {
        assert_eq!(Command::PlaceText.stable_id(), "place-text");
        assert_eq!(
            Command::PlaceText.spec().label,
            "Place text or note\u{2026}"
        );
        assert_eq!(Command::PlaceText.spec().group, "Design");
        assert_eq!(
            Command::from_stable_id("place-text"),
            Some(Command::PlaceText)
        );
    }

    #[test]
    fn place_shape_command_has_the_exact_mockup_identity_and_no_shortcut() {
        assert_eq!(Command::PlaceShape.stable_id(), "place-shape");
        assert_eq!(
            Command::PlaceShape.spec().label,
            "Draw documentation shape\u{2026}"
        );
        assert_eq!(Command::PlaceShape.spec().group, "Design");
        assert_eq!(
            Command::from_stable_id("place-shape"),
            Some(Command::PlaceShape)
        );
        assert!(Command::PlaceShape.shortcut_bindings().is_empty());
    }

    #[test]
    fn move_selection_command_has_the_exact_mockup_identity() {
        assert_eq!(Command::MoveSelection.stable_id(), "move-selection");
        assert_eq!(Command::MoveSelection.spec().label, "Move selection");
        assert_eq!(Command::MoveSelection.spec().group, "Design");
        assert_eq!(
            Command::from_stable_id("move-selection"),
            Some(Command::MoveSelection)
        );

        let registry_index = COMMAND_REGISTRY
            .iter()
            .position(|command| *command == Command::MoveSelection)
            .expect("move-selection must be registered");
        assert_eq!(COMMAND_REGISTRY[registry_index - 1], Command::PlaceShape);
        assert_eq!(
            COMMAND_REGISTRY[registry_index + 1],
            Command::StretchSelection
        );
    }

    #[test]
    fn stretch_selection_command_has_the_exact_mockup_identity() {
        assert_eq!(Command::StretchSelection.stable_id(), "stretch-selection");
        assert_eq!(Command::StretchSelection.spec().label, "Stretch selection");
        assert_eq!(Command::StretchSelection.spec().group, "Design");
        assert_eq!(
            Command::from_stable_id("stretch-selection"),
            Some(Command::StretchSelection)
        );

        let registry_index = COMMAND_REGISTRY
            .iter()
            .position(|command| *command == Command::StretchSelection)
            .expect("stretch-selection must be registered");
        assert_eq!(COMMAND_REGISTRY[registry_index - 1], Command::MoveSelection);
        assert_eq!(
            COMMAND_REGISTRY[registry_index + 1],
            Command::ArraySelection
        );
    }

    #[test]
    fn array_selection_command_has_the_exact_mockup_identity_and_no_shortcut() {
        assert_eq!(Command::ArraySelection.stable_id(), "array-selection");
        assert_eq!(Command::ArraySelection.spec().label, "Create array\u{2026}");
        assert_eq!(Command::ArraySelection.spec().group, "Design");
        assert_eq!(
            Command::from_stable_id("array-selection"),
            Some(Command::ArraySelection)
        );
        assert!(Command::ArraySelection.shortcut_bindings().is_empty());

        let registry_index = COMMAND_REGISTRY
            .iter()
            .position(|command| *command == Command::ArraySelection)
            .expect("array-selection must be registered");
        assert_eq!(
            COMMAND_REGISTRY[registry_index - 1],
            Command::StretchSelection
        );
        assert_eq!(COMMAND_REGISTRY[registry_index + 1], Command::SymbolPinTool);
    }

    #[test]
    fn move_selection_requires_one_live_object_in_an_editable_active_schematic() {
        use crate::state::{Component, Point};

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        assert!(!Command::MoveSelection.is_enabled(&app));

        app.state.schematic.selection.select_component(404);
        assert!(
            !Command::MoveSelection.is_enabled(&app),
            "a stale selection identity is not a movable object"
        );

        app.state.schematic.components.push(Component::new(
            404,
            ComponentType::Resistor,
            Point::origin(),
        ));
        assert!(Command::MoveSelection.is_enabled(&app));
        assert_eq!(
            Command::MoveSelection.availability(&app),
            CommandAvailability::Available
        );

        app.state.schematic.read_only = true;
        assert!(!Command::MoveSelection.is_enabled(&app));
        assert_eq!(
            Command::MoveSelection.availability(&app),
            CommandAvailability::Disabled("select an editable object")
        );

        app.state.schematic.read_only = false;
        app.state.workbench.workspace = Workspace::Results;
        assert!(!Command::MoveSelection.is_enabled(&app));
    }

    #[test]
    fn stretch_selection_requires_one_live_eligible_geometry_target() {
        use crate::state::{Point, Wire};

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        app.state.schematic.selection.select_wire_segment(17, 0);
        assert!(
            !Command::StretchSelection.is_enabled(&app),
            "a stale segment identity cannot open the workflow"
        );

        app.state
            .schematic
            .wires
            .push(Wire::new(17, vec![Point::new(0, 0), Point::new(40, 0)]));
        assert!(Command::StretchSelection.is_enabled(&app));
        assert_eq!(
            Command::StretchSelection.availability(&app),
            CommandAvailability::Available
        );

        app.state.schematic.read_only = true;
        assert!(!Command::StretchSelection.is_enabled(&app));
        assert_eq!(
            Command::StretchSelection.availability(&app),
            CommandAvailability::Disabled("select an editable object")
        );

        app.state.schematic.read_only = false;
        app.state.workbench.workspace = Workspace::Results;
        assert!(!Command::StretchSelection.is_enabled(&app));
    }

    #[test]
    fn array_selection_requires_a_live_eligible_editable_selection() {
        use crate::state::{Component, Point};

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        app.state.schematic.selection.select_component(404);
        assert!(
            !Command::ArraySelection.is_enabled(&app),
            "a stale selection identity cannot open the workflow"
        );

        app.state.schematic.components.push(Component::new(
            404,
            ComponentType::Resistor,
            Point::origin(),
        ));
        assert!(Command::ArraySelection.is_enabled(&app));
        assert_eq!(
            Command::ArraySelection.availability(&app),
            CommandAvailability::Available
        );

        app.state.schematic.read_only = true;
        assert!(!Command::ArraySelection.is_enabled(&app));
        assert_eq!(
            Command::ArraySelection.availability(&app),
            CommandAvailability::Disabled("select an editable object")
        );

        app.state.schematic.read_only = false;
        app.state.workbench.workspace = Workspace::Results;
        assert!(!Command::ArraySelection.is_enabled(&app));
    }

    #[test]
    fn cancel_retires_an_armed_array_transaction_and_restores_select() {
        let mut app = RSpiceApp::test_instance();
        app.state.dialogs.array_selection.armed = true;
        app.state.schematic.tool = Tool::ArraySelection;

        Command::Cancel.execute(&mut app);

        assert!(!app.state.dialogs.array_selection.armed);
        assert_eq!(app.state.schematic.tool, Tool::Select);
    }

    #[test]
    fn rename_command_has_mockup_identity_and_opens_the_stable_target_dialog() {
        use crate::state::Point;

        assert_eq!(Command::RenameSelection.stable_id(), "rename-selection");
        assert_eq!(
            Command::RenameSelection.spec().label,
            "Rename selected object…"
        );
        assert_eq!(
            Command::from_stable_id("rename-selection"),
            Some(Command::RenameSelection)
        );

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        let id = app
            .state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(0, 0));
        app.state.schematic.selection.select_only_component(id);
        assert!(Command::RenameSelection.is_enabled(&app));
        Command::RenameSelection.execute(&mut app);
        assert!(app.state.dialogs.rename_selection.open);
        assert!(matches!(
            app.state.dialogs.rename_selection.target.as_ref(),
            Some(crate::common::app::RenameSelectionTarget::Component(component))
                if component.id == id
        ));
    }

    #[test]
    fn draw_bus_arms_directly_but_bus_tap_waits_for_its_validated_dialog() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;

        Command::PlaceBus.execute(&mut app);
        assert_eq!(app.state.schematic.tool, Tool::Bus);

        app.state.schematic.tool = Tool::Select;
        Command::PlaceBusTap.execute(&mut app);
        assert!(app.state.dialogs.bus_tap.open);
        assert_eq!(app.state.schematic.tool, Tool::Select);
        assert!(app.state.schematic.pending_bus_tap.is_none());
    }

    #[test]
    fn place_pin_opens_the_isolated_mockup_transaction_without_mutating_the_document() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        let components = app.state.schematic.components.clone();
        let topology = app.state.schematic.topology_version();
        let dirty = app.state.schematic.is_dirty;
        let tool = app.state.schematic.tool;

        Command::PlacePin.execute(&mut app);

        assert!(app.state.dialogs.pin_port.open);
        assert_eq!(app.state.dialogs.pin_port.name, "BIAS_EN");
        assert_eq!(app.state.schematic.components, components);
        assert_eq!(app.state.schematic.topology_version(), topology);
        assert_eq!(app.state.schematic.is_dirty, dirty);
        assert_eq!(app.state.schematic.tool, tool);
        assert!(app.state.schematic.pending_port.is_none());
        assert!(!app.state.schematic.can_undo());
    }

    #[test]
    fn place_text_opens_the_isolated_mockup_transaction_without_mutating_the_document() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        let notes = app.state.schematic.design_notes.clone();
        let topology = app.state.schematic.topology_version();
        let dirty = app.state.schematic.is_dirty;
        let tool = app.state.schematic.tool;

        Command::PlaceText.execute(&mut app);

        assert!(app.state.dialogs.design_note.open);
        assert_eq!(app.state.dialogs.design_note.text, "Bias network");
        assert_eq!(app.state.schematic.design_notes, notes);
        assert_eq!(app.state.schematic.topology_version(), topology);
        assert_eq!(app.state.schematic.is_dirty, dirty);
        assert_eq!(app.state.schematic.tool, tool);
        assert!(app.state.schematic.pending_design_note.is_none());
        assert!(!app.state.schematic.can_undo());
    }

    #[test]
    fn place_shape_opens_the_isolated_mockup_transaction_without_mutating_the_document() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        let shapes = app.state.schematic.documentation_shapes.clone();
        let topology = app.state.schematic.topology_version();
        let dirty = app.state.schematic.is_dirty;
        let tool = app.state.schematic.tool;

        Command::PlaceShape.execute(&mut app);

        assert!(app.state.dialogs.documentation_shape.open);
        assert_eq!(
            app.state.dialogs.documentation_shape.kind,
            crate::state::DocumentationShapeKind::Rectangle
        );
        assert_eq!(app.state.schematic.documentation_shapes, shapes);
        assert_eq!(app.state.schematic.topology_version(), topology);
        assert_eq!(app.state.schematic.is_dirty, dirty);
        assert_eq!(app.state.schematic.tool, tool);
        assert!(app.state.schematic.pending_documentation_shape.is_none());
        assert!(!app.state.schematic.can_undo());
    }

    #[test]
    fn every_raw_port_command_route_is_projected_through_the_same_dialog() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;

        Command::Place(ComponentType::Port).execute(&mut app);

        assert!(app.state.dialogs.pin_port.open);
        assert_eq!(app.state.schematic.tool, Tool::Select);
        assert!(app.state.schematic.pending_port.is_none());
        assert!(app.state.schematic.components.is_empty());
    }

    #[test]
    fn port_undo_and_redo_resynchronize_the_generated_symbol_contract() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        let reference = app.state.workspace.active_view.clone();
        let pending = crate::state::PendingPortPlacement::new(
            "BIAS_EN",
            crate::state::PortDirectionType::InputLogic,
            crate::state::PortDiscipline::Logic,
            app.state.schematic.topology_version(),
            app.state.schematic.next_interface_order(),
        );
        app.state
            .schematic
            .place_pending_port(crate::state::Point::origin(), pending)
            .expect("port places");
        app.state.sync_active_schematic_to_workspace();
        let symbol_ports = |app: &RSpiceApp| {
            app.state
                .library_manager
                .get_library(&reference.library)
                .and_then(|library| library.get_cell(&reference.cell))
                .and_then(|cell| cell.get_view("symbol"))
                .and_then(|view| view.metadata.get("ports"))
                .cloned()
        };
        assert_eq!(symbol_ports(&app).as_deref(), Some("BIAS_EN:in"));

        Command::Undo.execute(&mut app);
        assert!(app.state.schematic.components.is_empty());
        assert!(symbol_ports(&app).is_none());

        Command::Redo.execute(&mut app);
        assert_eq!(symbol_ports(&app).as_deref(), Some("BIAS_EN:in"));
    }

    #[test]
    fn bus_authoring_commands_are_unavailable_on_read_only_schematics() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        assert!(Command::PlaceBus.is_enabled(&app));
        assert!(Command::PlaceBusTap.is_enabled(&app));
        assert!(Command::PlacePin.is_enabled(&app));
        assert!(Command::PlaceText.is_enabled(&app));
        assert!(Command::PlaceShape.is_enabled(&app));

        app.state.schematic.read_only = true;
        assert!(!Command::PlaceBus.is_enabled(&app));
        assert!(!Command::PlaceBusTap.is_enabled(&app));
        assert!(!Command::PlacePin.is_enabled(&app));
        assert!(!Command::PlaceText.is_enabled(&app));
        assert!(!Command::PlaceShape.is_enabled(&app));
        app.state.schematic.read_only = false;
        app.state.workbench.workspace = Workspace::Results;
        assert!(!Command::PlacePin.is_enabled(&app));
        assert!(!Command::PlaceText.is_enabled(&app));
        assert!(!Command::PlaceShape.is_enabled(&app));
    }

    #[test]
    fn object_properties_dispatches_selected_buses_and_taps_and_refuses_read_only() {
        use crate::state::{Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Point};

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        let bus = Bus::segment(
            80,
            Point::new(0, 0),
            Point::new(20, 0),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        let tap = BusTap::new(
            81,
            &bus,
            Point::new(5, 0),
            Point::new(5, 5),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        app.state.schematic.buses.push(bus);
        app.state.schematic.bus_taps.push(tap);

        app.state.schematic.selection.select_only_bus(80);
        assert!(Command::ObjectProperties.is_enabled(&app));
        Command::ObjectProperties.execute(&mut app);
        assert!(matches!(
            app.state.dialogs.object_properties.draft,
            Some(crate::common::app::ObjectPropertiesDraft::Bus(_))
        ));
        app.state.dialogs.object_properties.close();

        app.state.schematic.selection.select_only_bus_tap(81);
        Command::ObjectProperties.execute(&mut app);
        assert!(matches!(
            app.state.dialogs.object_properties.draft,
            Some(crate::common::app::ObjectPropertiesDraft::BusTap(_))
        ));
        app.state.dialogs.object_properties.close();

        app.state.schematic.read_only = true;
        assert!(!Command::ObjectProperties.is_enabled(&app));
        Command::ObjectProperties.execute(&mut app);
        assert!(!app.state.dialogs.object_properties.open);
    }

    #[test]
    fn object_properties_availability_includes_one_selected_net_label() {
        use crate::state::Point;

        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Design;
        let id = app
            .state
            .schematic
            .add_net_label(Point::new(0, 0), "gain_node".to_owned());
        app.state.schematic.selection.select_only_net_label(id);

        assert!(Command::ObjectProperties.is_enabled(&app));
        Command::ObjectProperties.execute(&mut app);
        assert!(matches!(
            app.state.dialogs.object_properties.draft,
            Some(crate::common::app::ObjectPropertiesDraft::NetLabel(ref draft))
                if draft.original.id == id
        ));
        app.state.dialogs.object_properties.close();
        app.state.schematic.read_only = true;
        assert!(!Command::ObjectProperties.is_enabled(&app));
        app.state.schematic.read_only = false;
        app.state.schematic.net_labels.clear();
        assert!(app.state.schematic.selection.single_net_label().is_some());
        assert!(!Command::ObjectProperties.is_enabled(&app));
    }

    #[test]
    fn switching_conductor_tools_cancels_incompatible_routes_and_tap_state() {
        use crate::state::{BusDeclaration, BusSlice, BusTapOrientation, PendingBusTap, Point};

        let mut schematic = crate::state::SchematicState::default();
        arm_schematic_tool(&mut schematic, Tool::Wire);
        schematic.start_wire(Point::origin());
        assert!(schematic.wire_drawing.active);

        arm_schematic_tool(&mut schematic, Tool::Bus);
        assert!(!schematic.wire_drawing.active);
        schematic.start_bus(Point::new(2, 3), None).unwrap();
        assert!(schematic.bus_drawing.active);

        schematic.pending_bus_tap = Some(
            PendingBusTap::new(
                BusDeclaration::parse("DATA[15:0]").unwrap(),
                BusSlice::parse("DATA[7:0]").unwrap(),
                BusTapOrientation::Automatic,
            )
            .unwrap(),
        );
        arm_schematic_tool(&mut schematic, Tool::BusTap);
        assert!(!schematic.bus_drawing.active);
        assert!(schematic.pending_bus_tap.is_some());

        arm_schematic_tool(&mut schematic, Tool::Wire);
        assert!(schematic.pending_bus_tap.is_none());
    }

    #[test]
    fn cancel_clears_even_hidden_conductor_routes() {
        use crate::state::Point;

        let mut schematic = crate::state::SchematicState::default();
        schematic.tool = Tool::Select;
        schematic.start_wire(Point::origin());
        schematic.start_bus(Point::new(4, 5), None).unwrap();
        assert!(schematic.wire_drawing.active);
        assert!(schematic.bus_drawing.active);

        cancel_schematic_tool(&mut schematic);

        assert_eq!(schematic.tool, Tool::Select);
        assert!(!schematic.wire_drawing.active);
        assert!(!schematic.bus_drawing.active);
        assert!(schematic.pending_bus_tap.is_none());
    }

    #[test]
    fn edit_specifications_opens_the_real_results_editor() {
        let mut app = RSpiceApp::test_instance();

        Command::EditSpecifications.execute(&mut app);

        assert_eq!(app.state.workbench.workspace, Workspace::Results);
        assert_eq!(
            app.state.ui.results.viewer,
            crate::workbench::ResultViewer::Specs
        );
        assert!(app.state.ui.results.spec_drafts.is_some());
    }

    #[test]
    fn every_exposed_result_viewer_command_activates_its_real_viewer() {
        for viewer in [
            crate::workbench::ResultViewer::Waves,
            crate::workbench::ResultViewer::Bode,
            crate::workbench::ResultViewer::Fft,
            crate::workbench::ResultViewer::Eye,
            crate::workbench::ResultViewer::Hist,
            crate::workbench::ResultViewer::Op,
            crate::workbench::ResultViewer::NoiseContrib,
            crate::workbench::ResultViewer::Contribution,
            crate::workbench::ResultViewer::Specs,
            crate::workbench::ResultViewer::Nyquist,
            crate::workbench::ResultViewer::Smith,
            crate::workbench::ResultViewer::PoleZero,
        ] {
            let mut app = RSpiceApp::test_instance();

            Command::ResultViewer(viewer).execute(&mut app);

            assert_eq!(app.state.workbench.workspace, Workspace::Results);
            assert_eq!(app.state.ui.results.viewer, viewer);
        }
    }

    #[test]
    fn exposed_results_calculator_opens_the_real_editor_dialog() {
        let mut app = RSpiceApp::test_instance();
        assert!(!app.state.dialogs.waveform_calculator_dialog);

        Command::WaveformCalculator.execute(&mut app);

        assert!(app.state.dialogs.waveform_calculator_dialog);
    }

    #[test]
    fn truthful_results_menu_routes_keep_their_stable_dispatch_identities() {
        assert_eq!(
            Command::ResultViewer(crate::workbench::ResultViewer::Waves).stable_id(),
            "waveforms"
        );
        assert_eq!(Command::WaveformCalculator.stable_id(), "calculator");
        assert_eq!(Command::ExportWaveformsCsv.stable_id(), "export-waveforms");
    }

    #[test]
    fn tuning_command_opens_the_transactional_sandbox() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Project;
        app.state.workbench.verification_page = VerificationPage::Yield;
        let command = Command::VerificationPage(VerificationPage::Tuning);

        assert!(command.is_enabled(&app));
        assert_eq!(command.availability(&app), CommandAvailability::Available);
        command.execute(&mut app);

        assert_eq!(app.state.workbench.workspace, Workspace::Verify);
        assert_eq!(
            app.state.workbench.verification_page,
            VerificationPage::Tuning
        );
    }

    #[test]
    fn physical_drc_command_is_inaccessible_without_physical_evidence_pipeline() {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.workspace = Workspace::Project;
        app.state.workbench.verification_page = VerificationPage::Yield;
        let command = Command::VerificationPage(VerificationPage::Drc);

        assert!(!command.is_enabled(&app));
        assert_eq!(
            command.availability(&app),
            CommandAvailability::Disabled(
                "no retained layout, qualified rule deck, or immutable marker database is available"
            )
        );
        command.execute(&mut app);

        assert_eq!(app.state.workbench.workspace, Workspace::Project);
        assert_eq!(
            app.state.workbench.verification_page,
            VerificationPage::Yield
        );
        assert!(
            app.state
                .log_buffer
                .entries()
                .any(|message| message.message.contains("Physical DRC is unavailable"))
        );
    }

    #[test]
    fn clear_results_cannot_remove_the_executor_owned_run() {
        let mut app = RSpiceApp::test_instance();
        let run = app.state.simulation.start_run();
        run.mark_running().unwrap();
        let identity = run.execution_identity().unwrap();
        app.state.simulation.active_execution = Some(identity);
        app.state.simulation.is_running = true;

        assert!(!Command::ClearResults.is_enabled(&app));
        assert_eq!(
            Command::ClearResults.availability(&app),
            CommandAvailability::Disabled(
                "an active simulation execution still owns result history"
            )
        );

        Command::ClearResults.execute(&mut app);

        assert!(
            app.state
                .simulation
                .run_by_stable_id(identity.run_id)
                .is_some()
        );
    }

    #[test]
    fn command_catalog_has_unique_stable_ids() {
        let mut ids = std::collections::HashSet::new();
        for command in COMMAND_REGISTRY {
            let id = command.spec().id;
            assert!(ids.insert(id), "duplicate command id {}", id);
            assert!(!id.is_empty());
            assert!(
                id.bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'),
                "command id is not a hyphenated product action: {id}"
            );
            assert_eq!(Command::from_stable_id(id), Some(*command));
        }
        for command in COMMAND_REGISTRY
            .iter()
            .copied()
            .filter(|command| !command.shortcut_bindings().is_empty())
        {
            assert!(
                ids.contains(command.stable_id()),
                "bindable command is missing a unique stable ID: {command:?}"
            );
        }
    }

    #[test]
    fn all_workspace_commands_are_discoverable() {
        for workspace in Workspace::ALL {
            assert!(COMMAND_REGISTRY.contains(&Command::OpenWorkspace(workspace)));
        }
    }

    #[test]
    fn protected_commands_keep_the_exact_mockup_action_ids() {
        assert_eq!(Command::CommandPalette.spec().id, "command-palette");
        assert_eq!(Command::ToggleFocusMode.spec().id, "toggle-focus-mode");
        assert_eq!(Command::RunSimulation.spec().id, "start-run");
        assert_eq!(Command::StopSimulation.spec().id, "stop-run");
        assert_eq!(Command::OpenProject.spec().id, "open-project");
        assert_eq!(Command::NewProject.spec().id, "new-project");
        assert_eq!(Command::Save.spec().id, "save-project");
        assert_eq!(Command::CloseActiveDocument.spec().id, "close-document");
        assert_eq!(Command::ToggleFullScreen.spec().id, "full-screen");
        assert_eq!(Command::GenerateNetlist.spec().id, "generated-netlist");
        assert_eq!(Command::ToggleConsole.spec().id, "console");
        assert_eq!(Command::OpenConsole.spec().id, "open-console");
        assert_eq!(Command::OpenProblems.spec().id, "open-problems");
        assert_eq!(
            Command::ToggleConsoleMaximized.spec().id,
            "console-maximize"
        );
        assert_eq!(Command::ClearConsole.spec().id, "console-clear");
        assert_eq!(
            Command::FeatureAvailability.spec(),
            CommandSpec {
                id: "feature-availability",
                label: "Product capability and platform matrix…",
                group: "Help",
            }
        );
        assert_eq!(
            Command::InteroperabilityMatrix.spec(),
            CommandSpec {
                id: "interoperability-matrix",
                label: "Interoperability and format matrix…",
                group: "Help",
            }
        );
        assert_eq!(
            Command::OpenWorkspace(Workspace::Results).spec().id,
            "results"
        );
        assert_eq!(
            Command::OpenWorkspace(Workspace::Verify).spec().id,
            "verify"
        );
        assert_eq!(
            Command::OpenWorkspace(Workspace::Models).spec().id,
            "models"
        );
        assert_eq!(
            Command::OpenWorkspace(Workspace::Netlist).spec().label,
            "Open automation workspace"
        );
        assert_eq!(
            Command::ModelsPage(ModelsPage::Models).spec().label,
            "Model & library catalog"
        );
    }

    #[test]
    fn only_exactly_implemented_reset_actions_are_discoverable() {
        let searchable = command_catalog().collect::<Vec<_>>();
        assert!(COMMAND_REGISTRY.contains(&Command::ResetActiveView));
        assert!(searchable.contains(&Command::ResetActiveView));
        assert!(COMMAND_REGISTRY.contains(&Command::ResetLayout));
        assert!(searchable.contains(&Command::ResetLayout));
        for command in [Command::PreviousWorkspace, Command::NextWorkspace] {
            assert!(!COMMAND_REGISTRY.contains(&command));
            assert!(!searchable.contains(&command));
        }
    }

    #[test]
    fn project_operation_gate_covers_every_mutating_project_command() {
        for command in [
            Command::ProjectLauncher,
            Command::RecentProjects,
            Command::NewProject,
            Command::OpenProject,
            Command::Save,
            Command::SaveAs,
            Command::SaveAll,
            Command::RevertActiveDocument,
            Command::CloseActiveDocument,
            Command::CloseProject,
            Command::NewCell,
            Command::OpenDocument,
            Command::ImportNetlist,
            Command::ImportVerilogA,
            Command::CheckAndSave,
        ] {
            assert!(
                command.blocked_by_project_operation(),
                "ungated: {command:?}"
            );
        }
        assert!(!Command::Copy.blocked_by_project_operation());
        assert!(!Command::ExportWaveformsCsv.blocked_by_project_operation());
    }

    #[test]
    fn stop_command_follows_the_execution_target_capability() {
        assert!(!stop_simulation_enabled(false));
        assert_eq!(
            stop_simulation_enabled(true),
            crate::simulation::execution::execution_target_supports_cancellation()
        );
    }

    #[test]
    fn closed_projects_expose_only_the_project_workspace() {
        assert!(workspace_available(false, Workspace::Project));
        for workspace in Workspace::ALL {
            if workspace != Workspace::Project {
                assert!(!workspace_available(false, workspace));
            }
            assert!(workspace_available(true, workspace));
        }
    }

    #[test]
    fn project_owned_subcommands_cannot_bypass_the_closed_project_boundary() {
        // This independent expectation list prevents the predicate under test
        // from silently omitting a newly exposed submenu route.
        for command in [
            Command::NewCell,
            Command::ImportNetlist,
            Command::ImportVerilogA,
            Command::ExportSchematicSvg,
            Command::ExportWaveformsCsv,
            Command::ExportNetlist(crate::io::NetlistFormat::Spice),
            Command::FindInDesign,
            Command::ProjectPage(ProjectPage::Configuration),
            Command::PreflightChecks,
            Command::SimulationOptions,
            Command::GenerateNetlist,
            Command::WaveformCalculator,
            Command::ResultViewer(crate::workbench::ResultViewer::Waves),
            Command::EditSpecifications,
            Command::VerificationPage(VerificationPage::Yield),
            Command::ModelsPage(ModelsPage::Models),
            Command::ModelBrowser,
            Command::PdkSettings,
            Command::CompileVerilogA,
            Command::AutomationConsole,
            Command::VisualizationStudio,
            Command::ReportAuthoring,
        ] {
            assert!(
                command.requires_open_project(),
                "missing closed-project boundary: {command:?}"
            );
        }

        let commands: Vec<_> = COMMAND_REGISTRY
            .iter()
            .copied()
            .filter(|command| command.requires_open_project())
            .collect();
        assert!(!commands.is_empty());

        for command in commands {
            let mut app = RSpiceApp::test_instance();
            app.state.project_lifecycle.project_open = false;
            app.state.workbench.workspace = Workspace::Project;

            assert!(
                !command.is_enabled(&app),
                "enabled without project: {command:?}"
            );
            assert_eq!(
                command.availability(&app),
                CommandAvailability::Disabled("no project is open"),
                "wrong closed-project reason for {command:?}"
            );

            command.execute(&mut app);

            assert_eq!(
                app.state.workbench.workspace,
                Workspace::Project,
                "closed-project command changed workspace: {command:?}"
            );
            assert!(
                app.state
                    .log_buffer
                    .entries()
                    .any(|entry| entry.message == "Open a project before using this command."),
                "closed-project command did not explain its boundary: {command:?}"
            );
        }
    }

    #[test]
    fn standalone_schematic_save_remains_available_without_a_project() {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.project_open = false;
        app.state.schematic.current_file = Some("standalone.rsch".into());

        assert!(!Command::Save.requires_open_project());
        assert!(Command::Save.is_enabled(&app));
        assert_eq!(
            Command::Save.availability(&app),
            CommandAvailability::Available
        );

        app.state.schematic.current_file = None;
        app.state.browser_schematic_save_name = Some("browser-import.rsch".to_owned());
        assert!(Command::Save.is_enabled(&app));
        assert_eq!(
            Command::Save.availability(&app),
            CommandAvailability::Available
        );
    }

    #[test]
    fn recent_projects_opens_the_launcher_on_the_real_recent_filter() {
        let mut workbench = WorkbenchState::default();
        workbench.project_launcher_filter = ProjectLauncherFilter::Pinned;
        workbench.project_launcher_open = false;
        workbench.focus_project_launcher_search = false;

        open_recent_projects(&mut workbench);

        assert!(workbench.project_launcher_open);
        assert!(workbench.focus_project_launcher_search);
        assert_eq!(
            workbench.project_launcher_page,
            crate::workbench::state::ProjectLauncherPage::Projects
        );
        assert_eq!(
            workbench.project_launcher_filter,
            ProjectLauncherFilter::Recent
        );
    }

    #[test]
    fn reset_active_view_is_unavailable_only_for_the_no_op_project_workspace() {
        assert!(!reset_active_view_available(Workspace::Project));
        for workspace in Workspace::ALL {
            if workspace != Workspace::Project {
                assert!(
                    reset_active_view_available(workspace),
                    "{workspace:?} has implemented reset behavior"
                );
            }
        }
    }

    #[test]
    fn repeated_violation_navigation_keeps_advancing_after_jump_to_design() {
        use crate::services::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};

        let mut app = RSpiceApp::test_instance();
        let mut result = DrcResult::new();
        for (id, x) in [(1, 10.0), (2, 20.0)] {
            result.add_violation(DrcViolation::new(
                id,
                DrcViolationType::DanglingWire,
                format!("anchored finding {id}"),
                DrcLocation::Point { x, y: 0.0 },
            ));
        }
        app.state.dialogs.drc_checked_version = app.state.schematic.topology_version();
        app.state.dialogs.drc_results = Some(result);
        app.state.workbench.activate(Workspace::Verify);

        for expected_cycle in [0, 1] {
            Command::NextViolation.execute(&mut app);
            assert_eq!(app.state.workbench.workspace, Workspace::Design);
            assert_eq!(app.state.dialogs.drc_cycle, Some(expected_cycle));
            assert!(app.state.schematic.center_request.is_some());
        }
    }
}
