//! The command vocabulary.
//!
//! Every command the product exposes, its stable portable identity, and the
//! label and group a menu or palette shows it under. This is a value model:
//! it describes what commands exist, not what pressing one does. What a
//! shell command does and whether it can, both of which need the running
//! application, live up in [`crate::workbench::commands`].
//!
//! Keeping the two apart is what lets the artifact writers name a command
//! without depending on the dispatcher that runs it. Shortcut profiles name
//! none: they read command facts by key through [`CommandRegistry`].

use std::collections::HashMap;
use std::sync::LazyLock;

use rspice_app_cloud::workbench::cloud::commands::CloudCommand;
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_design::schematic::view::sheet_visibility::SchematicObjectKind;
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_hardcopy::workbench::hardcopy::commands::HardcopyCommand;
use rspice_app_models::workbench::models::commands::ModelsCommand;
use rspice_app_project::workbench::project::commands::ProjectCommand;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_results::workbench::results::commands::ResultsCommand;
use rspice_app_services::io::NetlistFormat;
use rspice_app_settings::workbench::settings::commands::SettingsCommand;
use rspice_app_sheets::workbench::sheets::commands::SheetsCommand;
use rspice_app_simulate::workbench::simulate::commands::SimulateCommand;
use rspice_app_state::state::ModelsPage;
use rspice_app_state::state::command_catalog::{
    CommandCatalog, CommandKey, CommandPlatform, ShortcutBinding, ShortcutContext,
};
use rspice_app_state::state::{SimulationPage, VerificationPage};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::WorkbenchCommand;
use rspice_app_workbench::workbench::framework::command_registry::{self, CommandFlags, Registry};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// `CommandPalette` is the feature's name, the same one the shortcut
// preferences show the user. Trimming it to `Palette` to stop repeating the
// enum would rename a thing that exists.
#[allow(clippy::enum_variant_names)]
pub enum ShellCommand {
    OpenWorkspace(Workspace),
    RevertActiveDocument,
    CloseActiveDocument,
    Exit,
    /// Run the last placement or editing command again, as it last ran.
    RepeatLastCommand,
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
    ConfigurationSets,
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

impl ShellCommand {
    /// Stable product identity used by portable command profiles.
    pub const fn stable_id(self) -> &'static str {
        self.spec().id
    }

    /// The key a shortcut profile names this command by. A command a
    /// feature names by the framework's key returns that key.
    pub const fn key(self) -> CommandKey {
        match self {
            Self::OpenWorkspace(workspace) => command_registry::ids::open_workspace(workspace),
            Self::ConfigurationSets => command_registry::ids::CONFIGURATION_SETS,
            _ => CommandKey::new(self.stable_id()),
        }
    }

    /// Resolve a stable product identity through the canonical registry.
    /// Unknown IDs are intentionally retained by profile persistence but are
    /// never guessed into a command owned by this build. The product resolves
    /// a stored id to a key; a key becomes a command only at dispatch.
    #[cfg(test)]
    pub fn from_stable_id(id: &str) -> Option<Self> {
        command_registry().resolve(id).and_then(Self::from_key)
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
            Self::RepeatLastCommand => spec("repeat-last-command", "Repeat last command", "Edit"),
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
            Self::ConfigurationSets => spec(
                "configuration-sets",
                "Configuration sets\u{2026}",
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

    /// The command as the registry holds it, with the flags the
    /// dispatcher's prologue reads.
    pub(crate) fn declared_spec(self) -> command_registry::CommandSpec {
        command_registry::CommandSpec {
            key: self.key(),
            label: self.spec().label,
            group: self.spec().group,
            flags: CommandFlags::NONE
                .with(CommandFlags::PALETTE_VISIBLE, self.palette_visible())
                .with(
                    CommandFlags::PRIMARY_RESERVED_BY_BROWSER,
                    self.primary_is_reserved_on(CommandPlatform::Browser),
                )
                .with(
                    CommandFlags::REQUIRES_OPEN_PROJECT,
                    self.requires_open_project(),
                )
                .with(
                    CommandFlags::BLOCKED_BY_PROJECT_OPERATION,
                    self.blocked_by_project_operation(),
                )
                .with(
                    CommandFlags::COMMITS_INSPECTOR_EDIT,
                    self.commits_inspector_edit(),
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

impl ShellCommand {
    /// Every command the shell carries out itself, in the order the
    /// registry lists them among the features' commands.
    pub const ALL: &'static [Self] = &[
        Self::RevertActiveDocument,
        Self::CloseActiveDocument,
        Self::Exit,
        Self::RepeatLastCommand,
        Self::OpenWorkspace(Workspace::Project),
        Self::OpenWorkspace(Workspace::Design),
        Self::OpenWorkspace(Workspace::Simulate),
        Self::OpenWorkspace(Workspace::Stimulus),
        Self::OpenWorkspace(Workspace::Results),
        Self::OpenWorkspace(Workspace::Verify),
        Self::OpenWorkspace(Workspace::Models),
        Self::OpenWorkspace(Workspace::Netlist),
        Self::ToggleFullScreen,
        Self::ToggleNavigator,
        Self::ToggleInspector,
        Self::ToggleConsole,
        Self::OpenConsole,
        Self::OpenProblems,
        Self::OpenNotifications,
        Self::ToggleConsoleMaximized,
        Self::ClearConsole,
        Self::ToggleFocusMode,
        Self::NewApplicationWindow,
        Self::DetachDocument,
        Self::MoveDocumentToWindow,
        Self::ReattachDocument,
        Self::ConsolidateWindows,
        Self::MonitorRecovery,
        Self::PreviousDocument,
        Self::NextDocument,
        Self::NextPane,
        Self::PreviousPane,
        Self::CloseOtherDocuments,
        Self::CloseAllDocuments,
        Self::WindowManager,
        Self::MinimizeWindow,
        Self::ZoomWindow,
        Self::ResetLayout,
        Self::ConfigurationSets,
        Self::CommandPalette,
        Self::KeyboardShortcuts,
        Self::HelpCenter,
        Self::Examples,
        Self::ReleaseNotes,
        Self::SystemDiagnostics,
        Self::SupportBundle,
        Self::LegalPrivacy,
        Self::About,
    ];
}

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
    ShellCommand::RevertActiveDocument.key(),
    ShellCommand::CloseActiveDocument.key(),
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
    ShellCommand::Exit.key(),
    WorkbenchCommand::Undo.key(),
    WorkbenchCommand::Redo.key(),
    WorkbenchCommand::Cut.key(),
    WorkbenchCommand::Copy.key(),
    HardcopyCommand::CopyAsImage.key(),
    WorkbenchCommand::Paste.key(),
    WorkbenchCommand::Duplicate.key(),
    WorkbenchCommand::Delete.key(),
    WorkbenchCommand::SelectAll.key(),
    WorkbenchCommand::DeselectAll.key(),
    DesignCommand::SelectOfKind(SchematicObjectKind::Instances).key(),
    DesignCommand::SelectOfKind(SchematicObjectKind::Wires).key(),
    DesignCommand::SelectOfKind(SchematicObjectKind::NetLabels).key(),
    DesignCommand::SelectOfKind(SchematicObjectKind::Pins).key(),
    DesignCommand::SelectOfKind(SchematicObjectKind::Text).key(),
    DesignCommand::SelectOfKind(SchematicObjectKind::Shapes).key(),
    DesignCommand::RenameSelection.key(),
    DesignCommand::ObjectProperties.key(),
    WorkbenchCommand::FindInDesign.key(),
    DesignCommand::FindReplace.key(),
    ShellCommand::RepeatLastCommand.key(),
    SettingsCommand::Preferences.key(),
    ShellCommand::OpenWorkspace(Workspace::Project).key(),
    ShellCommand::OpenWorkspace(Workspace::Design).key(),
    ShellCommand::OpenWorkspace(Workspace::Simulate).key(),
    ShellCommand::OpenWorkspace(Workspace::Stimulus).key(),
    ShellCommand::OpenWorkspace(Workspace::Results).key(),
    ShellCommand::OpenWorkspace(Workspace::Verify).key(),
    ShellCommand::OpenWorkspace(Workspace::Models).key(),
    ShellCommand::OpenWorkspace(Workspace::Netlist).key(),
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
    SimulateCommand::Page(SimulationPage::ConnectRules).key(),
    SimulateCommand::Page(SimulationPage::Solver).key(),
    SimulateCommand::Page(SimulationPage::Save).key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Waves,
    )
    .key(),
    SimulateCommand::Verify(VerificationPage::Yield).key(),
    ModelsCommand::Page(ModelsPage::Models).key(),
    ModelsCommand::ModelCreateProjectCopy.key(),
    ModelsCommand::ModelEditor.key(),
    ModelsCommand::ModelCorrelation.key(),
    ModelsCommand::ModelSaveRevision.key(),
    ModelsCommand::ModelValidate.key(),
    ModelsCommand::ModelRunQualificationTests.key(),
    ModelsCommand::ModelCompareRelease.key(),
    WorkbenchCommand::ZoomIn.key(),
    WorkbenchCommand::ZoomOut.key(),
    WorkbenchCommand::ZoomFit.key(),
    DesignCommand::FitSchematicContent.key(),
    WorkbenchCommand::ZoomOneToOne.key(),
    WorkbenchCommand::ZoomToSelection.key(),
    WorkbenchCommand::ZoomBox.key(),
    WorkbenchCommand::ZoomPrevious.key(),
    DesignCommand::CycleGrid.key(),
    DesignCommand::GridDots.key(),
    DesignCommand::GridLines.key(),
    DesignCommand::GridOff.key(),
    DesignCommand::AnnotateOperatingPoint.key(),
    DesignCommand::AnnotateTransientVoltages.key(),
    DesignCommand::GridSnapRouting.key(),
    SheetsCommand::DrawingSheetLayers.key(),
    DesignCommand::VisibilityOptions.key(),
    DesignCommand::SymbolPlacedPreview.key(),
    DesignCommand::SymbolSnapToGrid.key(),
    DesignCommand::SymbolCycleGridSpacing.key(),
    ShellCommand::ToggleFullScreen.key(),
    DesignCommand::EngineeringTableView.key(),
    ShellCommand::ToggleNavigator.key(),
    ShellCommand::ToggleInspector.key(),
    ShellCommand::ToggleConsole.key(),
    ResultsCommand::ToggleResultsSplit.key(),
    ShellCommand::OpenConsole.key(),
    ShellCommand::OpenProblems.key(),
    ShellCommand::OpenNotifications.key(),
    ShellCommand::ToggleConsoleMaximized.key(),
    ShellCommand::ClearConsole.key(),
    ShellCommand::ToggleFocusMode.key(),
    ShellCommand::NewApplicationWindow.key(),
    ShellCommand::DetachDocument.key(),
    ShellCommand::MoveDocumentToWindow.key(),
    ShellCommand::ReattachDocument.key(),
    ShellCommand::ConsolidateWindows.key(),
    ShellCommand::MonitorRecovery.key(),
    ShellCommand::PreviousDocument.key(),
    ShellCommand::NextDocument.key(),
    ShellCommand::NextPane.key(),
    ShellCommand::PreviousPane.key(),
    ShellCommand::CloseOtherDocuments.key(),
    ShellCommand::CloseAllDocuments.key(),
    SettingsCommand::WorkspaceLayouts.key(),
    ShellCommand::WindowManager.key(),
    ShellCommand::MinimizeWindow.key(),
    ShellCommand::ZoomWindow.key(),
    ShellCommand::ResetLayout.key(),
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
    DesignCommand::MoveSelection.key(),
    DesignCommand::StretchSelection.key(),
    DesignCommand::ArraySelection.key(),
    DesignCommand::ReplaceInstance.key(),
    DesignCommand::SwapToRSpiceSymbols.key(),
    DesignCommand::UpdateInstanceInterface.key(),
    DesignCommand::CreateHierarchy.key(),
    ProjectCommand::CreateSymbol.key(),
    ProjectCommand::DesignManagement.key(),
    ProjectCommand::Variants.key(),
    ProjectCommand::Annotate.key(),
    DesignCommand::SelectionBulkEdit.key(),
    DesignCommand::ConnectivityManager.key(),
    DesignCommand::HighlightNet.key(),
    ShellCommand::ConfigurationSets.key(),
    DesignCommand::RuleSetup.key(),
    DesignCommand::ReviewComments.key(),
    ProjectCommand::RevisionHistory.key(),
    SheetsCommand::NextSheet.key(),
    SheetsCommand::PreviousSheet.key(),
    SheetsCommand::NewSheet.key(),
    SheetsCommand::DeleteSheet.key(),
    DesignCommand::SymbolPinTool.key(),
    DesignCommand::SymbolPolylineTool.key(),
    DesignCommand::SymbolRectangleTool.key(),
    DesignCommand::SymbolCircleTool.key(),
    DesignCommand::SymbolArcTool.key(),
    DesignCommand::SymbolPolygonTool.key(),
    DesignCommand::SymbolTextTool.key(),
    DesignCommand::SymbolUpdatePinsFromContract.key(),
    DesignCommand::SymbolCreateSchematicPins.key(),
    DesignCommand::EditSymbolInPlace.key(),
    DesignCommand::ExitSymbolInPlace.key(),
    DesignCommand::PlaceResistor.key(),
    DesignCommand::PlaceCapacitor.key(),
    DesignCommand::PlaceInductor.key(),
    DesignCommand::PlaceDiode.key(),
    DesignCommand::PlaceGround.key(),
    DesignCommand::PlaceVoltageSource.key(),
    DesignCommand::PlaceCurrentSource.key(),
    DesignCommand::RotateSelection.key(),
    DesignCommand::RotateSelectionCounterClockwise.key(),
    DesignCommand::MirrorSelectionHorizontal.key(),
    DesignCommand::MirrorSelectionVertical.key(),
    DesignCommand::AlignSelectionLeft.key(),
    DesignCommand::AlignSelectionCenter.key(),
    DesignCommand::AlignSelectionRight.key(),
    DesignCommand::AlignSelectionTop.key(),
    DesignCommand::AlignSelectionMiddle.key(),
    DesignCommand::AlignSelectionBottom.key(),
    DesignCommand::DistributeSelectionHorizontal.key(),
    DesignCommand::DistributeSelectionVertical.key(),
    DesignCommand::SnapSelectionToGrid.key(),
    DesignCommand::SnapToStandardValue.key(),
    DesignCommand::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E6).key(),
    DesignCommand::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E12).key(),
    DesignCommand::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E24).key(),
    DesignCommand::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E48).key(),
    DesignCommand::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E96).key(),
    DesignCommand::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E192).key(),
    DesignCommand::JoinCollinearWires.key(),
    DesignCommand::AscendHierarchy.key(),
    DesignCommand::DescendHierarchy.key(),
    DesignCommand::DescendHierarchyDirect.key(),
    CodeCommand::ShowInNetlist.key(),
    DesignCommand::CheckCellview.key(),
    DesignCommand::CheckDesign.key(),
    ProjectCommand::CheckAndSave.key(),
    DesignCommand::ClearChecks.key(),
    DesignCommand::NextViolation.key(),
    DesignCommand::PreviousViolation.key(),
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
    ResultsCommand::CursorToMaximum.key(),
    ResultsCommand::CursorToMinimum.key(),
    ResultsCommand::CursorToNextPeak.key(),
    ResultsCommand::CursorToPreviousPeak.key(),
    ResultsCommand::CursorToNextCrossing.key(),
    ResultsCommand::CursorToPreviousCrossing.key(),
    ResultsCommand::SwitchSteppingCursor.key(),
    ResultsCommand::MarkerAtCursor.key(),
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
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Bode,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::DcSweep,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Fft,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::HarmonicBalance,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::PhaseNoise,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Eye,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Hist,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Op,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::NoiseContrib,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Contribution,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::TransferFunction,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Specs,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Table,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Nyquist,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Smith,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Polar,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::NetworkMatrix,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::PoleZero,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Scatter,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::BoxViolin,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Events,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Soa,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Optimization,
    )
    .key(),
    ResultsCommand::ResultViewer(
        rspice_app_results::workbench::documents::result_document::ResultViewer::Manifest,
    )
    .key(),
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
    ShellCommand::CommandPalette.key(),
    ShellCommand::KeyboardShortcuts.key(),
    CloudCommand::AccountOrganization.key(),
    CloudCommand::License.key(),
    CloudCommand::CheckForUpdates.key(),
    HardcopyCommand::ReportAuthoring.key(),
    ResultsCommand::AddVisualizationPane.key(),
    ResultsCommand::VisualizationDocumentProperties.key(),
    ShellCommand::HelpCenter.key(),
    ShellCommand::Examples.key(),
    ShellCommand::ReleaseNotes.key(),
    ShellCommand::SystemDiagnostics.key(),
    ShellCommand::SupportBundle.key(),
    ShellCommand::LegalPrivacy.key(),
    ShellCommand::About.key(),
    WorkbenchCommand::Cancel.key(),
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
    ("fit-canvas", WorkbenchCommand::ZoomFit.key()),
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
    ("run-checks", DesignCommand::CheckCellview.key()),
];

/// The registry of every command in [`COMMAND_ORDER`]: the shell's own in
/// [`ShellCommand::ALL`] and the ones the features declare. Built on first use
/// and checked as it is built.
pub(crate) fn command_registry() -> &'static Registry {
    static REGISTRY: LazyLock<Registry> = LazyLock::new(|| {
        let registry = Registry::new(
            COMMAND_ORDER,
            ShellCommand::ALL
                .iter()
                .map(|command| command.declared_spec())
                .chain(WorkbenchCommand::ALL.map(WorkbenchCommand::declared_spec))
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

impl ShellCommand {
    /// The shell command `key` names; `None` for a feature's key, a routed
    /// command's, or nobody's.
    pub(crate) fn from_key(key: CommandKey) -> Option<Self> {
        static BY_KEY: LazyLock<HashMap<CommandKey, ShellCommand>> = LazyLock::new(|| {
            let mut commands = HashMap::with_capacity(ShellCommand::ALL.len());
            for command in ShellCommand::ALL {
                commands.entry(command.key()).or_insert(*command);
            }
            commands
        });
        BY_KEY.get(&key).copied()
    }
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
pub fn command_catalog() -> impl Iterator<Item = ShellCommand> {
    let registry = command_registry();
    registry
        .order()
        .iter()
        .filter(move |key| {
            registry
                .spec(**key)
                .is_some_and(|spec| spec.flags.contains(CommandFlags::PALETTE_VISIBLE))
        })
        .filter_map(|key| ShellCommand::from_key(*key))
}
