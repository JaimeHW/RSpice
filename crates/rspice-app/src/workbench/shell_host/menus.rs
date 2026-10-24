//! What each application menu holds, written down as entries the menu
//! renderer draws and a test can read. The source editor's right-click menu
//! is written the same way.

use crate::state::ModelsPage;
use crate::workbench::chrome::title_bar::application_menus::ApplicationMenu;
use crate::workbench::chrome::title_bar::menu_entries::MenuEntry::Separator;
use crate::workbench::chrome::title_bar::menu_entries::{MenuContext, MenuEntry, palette_scope};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::state::{VerificationPage, Workspace};

/// The command's row under the vocabulary's label.
const fn command(command: Command) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// The command's row under a label of the menu's own.
const fn command_as(command: Command, label: &'static str) -> MenuEntry {
    MenuEntry::command_as(command.key(), label)
}

impl ApplicationMenu {
    pub(in crate::workbench) const fn entries(self) -> &'static [MenuEntry] {
        match self {
            Self::File => FILE_MENU,
            Self::Edit => EDIT_MENU,
            Self::View => VIEW_MENU,
            Self::Design => DESIGN_MENU,
            Self::Simulate => SIMULATE_MENU,
            Self::Results => RESULTS_MENU,
            Self::Verify => VERIFY_MENU,
            Self::Models => MODELS_MENU,
            Self::Automation => AUTOMATION_MENU,
            Self::Window => WINDOW_MENU,
            Self::Help => HELP_MENU,
        }
    }
}

const FILE_MENU: &[MenuEntry] = &[
    command(Command::ProjectLauncher),
    Separator,
    command(Command::OpenProject),
    command(Command::OpenNetlist),
    command(Command::NewProject),
    command(Command::RecentProjects),
    Separator,
    command(Command::NewCell),
    command(Command::OpenDocument),
    Separator,
    command(Command::Save),
    command(Command::SaveAs),
    command(Command::SaveAll),
    command(Command::RevertActiveDocument),
    Separator,
    command(Command::ImportNetlist),
    command(Command::ImportVerilogA),
    Separator,
    command(Command::ExportSchematicSvg),
    command(Command::ExportWaveformsCsv),
    command(Command::ExportPublicationSnapshot),
    command(Command::ExportNetlist(crate::io::NetlistFormat::Spice)),
    Separator,
    command(Command::LiveSession),
    command(Command::PublishToWeb),
    Separator,
    command(Command::CloseActiveDocument),
    command(Command::CloseProject),
    Separator,
    // The drawing sheet's workflows, in the order the sheet is set up.
    command(Command::SheetProperties),
    command(Command::SheetFormatManager),
    command(Command::CustomSheetSizes),
    command(Command::SheetDefaults),
    command(Command::PrintHardcopy),
    command(Command::ExportActiveView),
    MenuEntry::Only(
        MenuContext::DesktopApplication,
        &[Separator, command(Command::Exit)],
    ),
];

/// Edit acts on what the page shows: with a source editor as the document,
/// on its text; otherwise on the drawing's selection.
const EDIT_MENU: &[MenuEntry] = &[
    command(Command::Undo),
    command(Command::Redo),
    Separator,
    command(Command::Cut),
    command(Command::Copy),
    command(Command::Paste),
    MenuEntry::Only(
        MenuContext::SourceEditor,
        &[command(Command::DuplicateLine)],
    ),
    MenuEntry::Only(MenuContext::Drawing, &[command(Command::Duplicate)]),
    command(Command::Delete),
    command(Command::SelectAll),
    Separator,
    MenuEntry::Only(
        MenuContext::SourceEditor,
        &[
            command(Command::ToggleLineComment),
            command(Command::FormatDocument),
            command(Command::GoToLine),
            command(Command::FindCodeDocument),
        ],
    ),
    MenuEntry::Only(
        MenuContext::Drawing,
        &[
            command(Command::RenameSelection),
            command(Command::ObjectProperties),
            command(Command::FindInDesign),
            command(Command::FindReplace),
        ],
    ),
    Separator,
    command(Command::Preferences),
];

const VIEW_MENU: &[MenuEntry] = &[
    command(Command::ZoomIn),
    command(Command::ZoomOut),
    command(Command::ZoomFit),
    command(Command::FitSchematicContent),
    command(Command::ZoomOneToOne),
    command(Command::ZoomToSelection),
    command(Command::ZoomBox),
    command(Command::CycleGrid),
    command(Command::GridSnapRouting),
    command(Command::DrawingSheetLayers),
    command(Command::VisibilityOptions),
    // Folding, offered while a source editor is the document on screen.
    MenuEntry::Only(
        MenuContext::SourceEditor,
        &[
            Separator,
            command(Command::FoldSelection),
            command(Command::UnfoldAtCaret),
            command(Command::FoldAll),
            command(Command::UnfoldAll),
        ],
    ),
    Separator,
    command_as(Command::ToggleFullScreen, "Full screen"),
    command(Command::ResetActiveView),
    Separator,
    command(Command::EngineeringTableView),
];

const DESIGN_MENU: &[MenuEntry] = &[
    command_as(
        Command::OpenWorkspace(Workspace::Design),
        "Open active schematic",
    ),
    command(Command::AscendHierarchy),
    command(Command::DescendHierarchy),
    Separator,
    command(Command::PlaceInstance),
    command(Command::PlaceWire),
    command(Command::PlaceBus),
    command(Command::PlaceBusTap),
    command(Command::PlaceJunction),
    command(Command::PlaceLabel),
    command(Command::PlaceProbe),
    command(Command::PlacePin),
    command(Command::PlaceText),
    command(Command::PlaceShape),
    Separator,
    command(Command::MoveSelection),
    command(Command::StretchSelection),
    command(Command::ArraySelection),
    command(Command::ReplaceInstance),
    MenuEntry::Submenu {
        label: "Arrange",
        entries: ARRANGE_MENU,
    },
    command(Command::JoinCollinearWires),
    Separator,
    // The sheets of the active drawing, in the order the strip shows them:
    // navigate first, then author.
    command(Command::PreviousSheet),
    command(Command::NextSheet),
    command(Command::NewSheet),
    command(Command::DeleteSheet),
    Separator,
    command(Command::CreateHierarchy),
    command(Command::DesignManagement),
    command(Command::SelectionBulkEdit),
    command(Command::ConnectivityManager),
    command(Command::ConfigurationSets),
    command(Command::RuleSetup),
    command(Command::ReviewComments),
    command(Command::RevisionHistory),
    Separator,
    command_as(
        Command::DesignSpecialistWorkspaces,
        "Specialist workspaces\u{2026}",
    ),
    Separator,
    // The design checks, narrowest first: the cellview in front of the
    // reader, the whole design, and the check that gates a save.
    command(Command::CheckCellview),
    command(Command::CheckDesign),
    command(Command::CheckAndSave),
    Separator,
    palette_scope("Browse all design commands\u{2026}", "Design"),
];

/// The Design menu's Arrange submenu, group by group: turn and flip, line
/// up, space out, put back on the grid.
const ARRANGE_MENU: &[MenuEntry] = &[
    command(Command::RotateSelection),
    command(Command::RotateSelectionCounterClockwise),
    command(Command::MirrorSelectionHorizontal),
    command(Command::MirrorSelectionVertical),
    Separator,
    command(Command::AlignSelectionLeft),
    command(Command::AlignSelectionCenter),
    command(Command::AlignSelectionRight),
    command(Command::AlignSelectionTop),
    command(Command::AlignSelectionMiddle),
    command(Command::AlignSelectionBottom),
    Separator,
    command(Command::DistributeSelectionHorizontal),
    command(Command::DistributeSelectionVertical),
    Separator,
    command(Command::SnapSelectionToGrid),
];

const SIMULATE_MENU: &[MenuEntry] = &[
    // The manager owns which plan is active, and every row below it acts on
    // that plan - "Run active plan", "Stop active run", its job history, its
    // preflight checks. Choosing the plan therefore reads above the actions
    // taken on it, in the same group they belong to.
    command(Command::ManageSimulationPlans),
    // Filling the chosen plan reads directly under choosing it, and above the
    // actions that dispatch what it holds. It is here because the navigator is
    // the only other route to the catalogue, and the navigator is a panel a
    // reader can collapse.
    command(Command::AddAnalysis),
    command(Command::RunSimulation),
    command(Command::StopSimulation),
    command(Command::JobsManager),
    command(Command::PreflightChecks),
    Separator,
    command_as(
        Command::OpenWorkspace(Workspace::Simulate),
        "Simulation Studio",
    ),
    // Beside the studio, because a definition is what the studio's
    // excitations read: the mockup's Simulate menu lists the two together.
    command_as(
        Command::OpenWorkspace(Workspace::Stimulus),
        "Stimulus Library",
    ),
    command(Command::SimulationOptions),
    command(Command::GenerateNetlist),
    command(Command::EditSpecifications),
];

const RESULTS_MENU: &[MenuEntry] = &[
    command(Command::ResultViewer(crate::workbench::ResultViewer::Waves)),
    command(Command::DatasetManifestBrowser),
    command(Command::CreateResultDocument),
    command(Command::CompareResultDatasets),
    command(Command::ReportAuthoring),
    Separator,
    command_as(Command::ToggleLinkedCursors, "Link A/B cursors"),
    command(Command::WaveformCalculator),
    command(Command::ExpressionDiagnostics),
    command(Command::MeasurementLibrary),
    command(Command::FamilySlicing),
    command(Command::VisualizationDocumentProperties),
    Separator,
    command(Command::ImportResultDataset),
    command(Command::ExportWaveformsCsv),
];

const VERIFY_MENU: &[MenuEntry] = &[
    command_as(
        Command::VerificationPage(VerificationPage::Yield),
        "Verification cockpit",
    ),
    command_as(
        Command::VerificationPage(VerificationPage::Corners),
        "Corner matrix",
    ),
    command_as(
        Command::VerificationPage(VerificationPage::Optimization),
        "Optimization",
    ),
    command_as(
        Command::VerificationPage(VerificationPage::DeviceSafety),
        "Safe operating area",
    ),
    command_as(
        Command::VerificationPage(VerificationPage::Regression),
        "Regression plan",
    ),
];

const MODELS_MENU: &[MenuEntry] = &[
    command_as(
        Command::ModelsPage(ModelsPage::Models),
        "Model & library catalog",
    ),
    command(Command::ModelEditor),
    command(Command::ModelCorrelation),
    Separator,
    command_as(Command::CompileVerilogA, "Verilog-A/AMS compiler"),
];

const AUTOMATION_MENU: &[MenuEntry] = &[command_as(
    Command::AutomationConsole,
    "Automation workspace",
)];

const WINDOW_MENU: &[MenuEntry] = &[
    command_as(Command::ToggleNavigator, "Navigator"),
    command_as(Command::ToggleInspector, "Inspector"),
    command_as(Command::ToggleConsole, "Console"),
    command(Command::ToggleResultsSplit),
    command_as(Command::ToggleFocusMode, "Focus mode"),
    Separator,
    command(Command::NewApplicationWindow),
    command(Command::DetachDocument),
    command(Command::MoveDocumentToWindow),
    command(Command::ReattachDocument),
    command(Command::ConsolidateWindows),
    command(Command::MonitorRecovery),
    Separator,
    command(Command::PreviousDocument),
    command(Command::NextDocument),
    command(Command::CloseOtherDocuments),
    command(Command::CloseAllDocuments),
    Separator,
    command(Command::WorkspaceLayouts),
    command(Command::WindowManager),
    command(Command::ResetLayout),
];

const HELP_MENU: &[MenuEntry] = &[
    command(Command::HelpCenter),
    command(Command::KeyboardShortcuts),
    command(Command::ReleaseNotes),
    command(Command::MigrationGuide),
    command(Command::SystemDiagnostics),
    command(Command::SupportBundle),
    command(Command::LegalPrivacy),
    Separator,
    command(Command::About),
];

/// What the compact layout's Menu button holds: every application menu,
/// in the bar's order, as a submenu.
pub(in crate::workbench) const COMPACT_MENU: &[MenuEntry] = &[
    compact_submenu(ApplicationMenu::File),
    compact_submenu(ApplicationMenu::Edit),
    compact_submenu(ApplicationMenu::View),
    compact_submenu(ApplicationMenu::Design),
    compact_submenu(ApplicationMenu::Simulate),
    compact_submenu(ApplicationMenu::Results),
    compact_submenu(ApplicationMenu::Verify),
    compact_submenu(ApplicationMenu::Models),
    compact_submenu(ApplicationMenu::Automation),
    compact_submenu(ApplicationMenu::Window),
    compact_submenu(ApplicationMenu::Help),
];

const fn compact_submenu(menu: ApplicationMenu) -> MenuEntry {
    MenuEntry::Submenu {
        label: menu.label(),
        entries: menu.entries(),
    }
}

/// The source editor's right-click menu: the language group, shown only
/// where the document's language service can answer it, then the
/// clipboard, then the line edits. Mutating rows are greyed on a read-only
/// source.
pub(in crate::workbench) const SOURCE_EDITOR_MENU: &[MenuEntry] = &[
    MenuEntry::Only(
        MenuContext::NavigableSourceEditor,
        &[
            command(Command::GoToDefinition),
            command(Command::FindReferences),
            command(Command::RenameSymbol),
            Separator,
        ],
    ),
    command(Command::Cut),
    command(Command::Copy),
    command(Command::Paste),
    Separator,
    command(Command::ToggleLineComment),
    command(Command::FormatDocument),
];

/// The icon a menu row shows: the one the command's toolbar button wears
/// (`chrome/toolbar.rs`), for a command that has a button. A command with
/// no button shows none, so an icon in a menu always marks the toolbar's
/// twin. The column stays for the rows that carry a check mark.
pub(in crate::workbench) fn command_icon(command: Command) -> Option<WorkbenchIcon> {
    let icon = match command {
        Command::SelectTool => WorkbenchIcon::Select,
        Command::PlaceInstance => WorkbenchIcon::Instance,
        Command::PlaceWire => WorkbenchIcon::Wire,
        Command::PlaceBus => WorkbenchIcon::Bus,
        Command::PlaceBusTap => WorkbenchIcon::BusTap,
        Command::PlaceJunction => WorkbenchIcon::Junction,
        Command::PlaceLabel => WorkbenchIcon::Label,
        Command::PlaceOffSheetConnector => WorkbenchIcon::ArrowRight,
        Command::PlacePin => WorkbenchIcon::Pin,
        Command::PlaceProbe => WorkbenchIcon::Probe,
        Command::PlaceText => WorkbenchIcon::Text,
        Command::ProjectLauncher | Command::ImportResultDataset | Command::ManageSourceDocument => {
            WorkbenchIcon::Folder
        }
        Command::NewProject | Command::NewCell | Command::AddReportPage => WorkbenchIcon::Add,
        Command::Save
        | Command::SymbolSave
        | Command::ModelSaveRevision
        | Command::SaveReportDocument => WorkbenchIcon::Save,
        Command::RescanModelLibraries => WorkbenchIcon::Refresh,
        // The Models toolbar's button; the Verilog-A view's wears Run.
        Command::CompileVerilogA => WorkbenchIcon::Netlist,
        Command::RotateSelection => WorkbenchIcon::Rotate,
        Command::MirrorSelectionHorizontal => WorkbenchIcon::Mirror,
        Command::MirrorSelectionVertical => WorkbenchIcon::MirrorVertical,
        Command::Duplicate => WorkbenchIcon::Copy,
        Command::Delete => WorkbenchIcon::Trash,
        Command::Undo => WorkbenchIcon::Undo,
        Command::Redo => WorkbenchIcon::Redo,
        Command::DescendHierarchyDirect => WorkbenchIcon::Layers,
        Command::AscendHierarchy | Command::ExitSymbolInPlace => WorkbenchIcon::ArrowUp,
        Command::ZoomIn => WorkbenchIcon::ZoomIn,
        Command::ZoomOut => WorkbenchIcon::ZoomOut,
        Command::ZoomFit => WorkbenchIcon::ZoomFit,
        Command::VisibilityOptions => WorkbenchIcon::Visibility,
        Command::CycleGrid => WorkbenchIcon::Grid,
        Command::CheckCellview
        | Command::CheckDesign
        | Command::PreflightChecks
        | Command::ModelValidate
        | Command::ValidateCodeDocument => WorkbenchIcon::Check,
        Command::VerilogABuildSettings
        | Command::SimulationOptions
        | Command::WaveformCalculator
        | Command::ReportPageProperties => WorkbenchIcon::Sliders,
        Command::FindCodeDocument => WorkbenchIcon::Search,
        Command::SourceLanguageTools | Command::EditSpecifications => WorkbenchIcon::Target,
        Command::CompareResultDatasets
        | Command::ModelCompareRelease
        | Command::CompareGeneratedRevisions => WorkbenchIcon::Compare,
        Command::ExportWaveformsCsv | Command::ExportNetlist(crate::io::NetlistFormat::Spice) => {
            WorkbenchIcon::Export
        }
        Command::PrintHardcopy => WorkbenchIcon::File,
        Command::RunSimulation | Command::ModelRunQualificationTests => WorkbenchIcon::Run,
        Command::StopSimulation => WorkbenchIcon::Stop,
        Command::AutomationConsole => WorkbenchIcon::Terminal,
        _ => return None,
    };
    Some(icon)
}
