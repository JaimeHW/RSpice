//! What each application menu holds, written down as entries the menu
//! renderer draws and a test can read. The source editor's right-click menu
//! is written the same way.

use crate::state::ModelsPage;
use crate::state::VerificationPage;
use crate::state::command_catalog::CommandKey;
use crate::workbench::chrome::title_bar::application_menus::ApplicationMenu;
use crate::workbench::chrome::title_bar::menu_entries::MenuEntry::Separator;
use crate::workbench::chrome::title_bar::menu_entries::{MenuContext, MenuEntry, palette_scope};
use crate::workbench::cloud::commands::CloudCommand;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::hardcopy::commands::HardcopyCommand;
use crate::workbench::models::commands::ModelsCommand;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::results::commands::ResultsCommand;
use crate::workbench::settings::commands::SettingsCommand;
use crate::workbench::sheets::commands::SheetsCommand;
use crate::workbench::simulate::commands::SimulateCommand;
use crate::workbench::state::Workspace;

/// The command's row under the vocabulary's label.
const fn command(command: Command) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// The command's row under a label of the menu's own.
const fn command_as(command: Command, label: &'static str) -> MenuEntry {
    MenuEntry::command_as(command.key(), label)
}

/// The sheets feature's command's row under the label it declares.
const fn sheets(command: SheetsCommand) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// The results feature's command's row under the label it declares.
const fn results(command: ResultsCommand) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// The settings feature's command's row under the label it declares.
const fn settings(command: SettingsCommand) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// The hardcopy feature's command's row under the label it declares.
const fn hardcopy(command: HardcopyCommand) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// The cloud feature's command's row under the label it declares.
const fn cloud(command: CloudCommand) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// The models feature's command's row under the label it declares.
const fn models(command: ModelsCommand) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// The simulate feature's command's row under the label it declares.
const fn simulate(command: SimulateCommand) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// The project feature's command's row under the label it declares.
const fn project(command: ProjectCommand) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// What the application menu `menu` holds.
pub(in crate::workbench) const fn application_menu_entries(
    menu: ApplicationMenu,
) -> &'static [MenuEntry] {
    match menu {
        ApplicationMenu::File => FILE_MENU,
        ApplicationMenu::Edit => EDIT_MENU,
        ApplicationMenu::View => VIEW_MENU,
        ApplicationMenu::Design => DESIGN_MENU,
        ApplicationMenu::Simulate => SIMULATE_MENU,
        ApplicationMenu::Results => RESULTS_MENU,
        ApplicationMenu::Verify => VERIFY_MENU,
        ApplicationMenu::Models => MODELS_MENU,
        ApplicationMenu::Automation => AUTOMATION_MENU,
        ApplicationMenu::Window => WINDOW_MENU,
        ApplicationMenu::Help => HELP_MENU,
    }
}

/// File makes, opens, closes, keeps, brings in and sends out: the project,
/// its cells and the documents open on them.
const FILE_MENU: &[MenuEntry] = &[
    project(ProjectCommand::NewProject),
    project(ProjectCommand::NewCell),
    project(ProjectCommand::NewView),
    command(Command::NewVerilogADevice),
    Separator,
    // A schematic file is opened as a document of its own only on the start
    // screen; inside a project it is imported as a new cell view.
    MenuEntry::Only(
        MenuContext::NoProject,
        &[project(ProjectCommand::OpenSchematic)],
    ),
    project(ProjectCommand::OpenProject),
    command(Command::OpenNetlist),
    MenuEntry::Submenu {
        label: "Open recent",
        entries: OPEN_RECENT_MENU,
    },
    Separator,
    command(Command::CloseActiveDocument),
    command(Command::CloseOtherDocuments),
    command(Command::CloseAllDocuments),
    project(ProjectCommand::CloseProject),
    Separator,
    project(ProjectCommand::Save),
    project(ProjectCommand::SaveAs),
    project(ProjectCommand::SaveAll),
    command(Command::RevertActiveDocument),
    project(ProjectCommand::RevisionHistory),
    Separator,
    MenuEntry::Submenu {
        label: "Import",
        entries: IMPORT_MENU,
    },
    MenuEntry::Submenu {
        label: "Export",
        entries: EXPORT_MENU,
    },
    Separator,
    cloud(CloudCommand::PublishToWeb),
    cloud(CloudCommand::LiveSession),
    Separator,
    hardcopy(HardcopyCommand::PrintHardcopy),
    MenuEntry::Only(
        MenuContext::ExitInFileMenu,
        &[Separator, command(Command::Exit)],
    ),
];

/// The files opened lately, newest first, then the launcher holding every
/// project.
const OPEN_RECENT_MENU: &[MenuEntry] = &[
    MenuEntry::RecentFiles(ProjectCommand::OpenProject.key()),
    Separator,
    MenuEntry::command_as(
        ProjectCommand::ProjectLauncher.key(),
        "All projects\u{2026}",
    ),
];

const IMPORT_MENU: &[MenuEntry] = &[
    command_as(Command::ImportNetlist, "SPICE deck\u{2026}"),
    command_as(Command::ImportVerilogA, "Verilog-A\u{2026}"),
    MenuEntry::command_as(
        ResultsCommand::ImportResultDataset.key(),
        "Result dataset\u{2026}",
    ),
    MenuEntry::Only(
        MenuContext::ProjectOpen,
        &[MenuEntry::command_as(
            ProjectCommand::OpenSchematic.key(),
            "Schematic into new cell view\u{2026}",
        )],
    ),
];

const EXPORT_MENU: &[MenuEntry] = &[
    MenuEntry::command_as(
        HardcopyCommand::ExportActiveView.key(),
        "Image or PDF\u{2026}",
    ),
    command_as(
        Command::ExportNetlist(crate::io::NetlistFormat::Spice),
        "SPICE netlist\u{2026}",
    ),
    command_as(
        Command::ExportNetlist(crate::io::NetlistFormat::Spectre),
        "Spectre netlist\u{2026}",
    ),
    MenuEntry::command_as(ResultsCommand::ExportWaveformsCsv.key(), "Dataset\u{2026}"),
];

/// Edit acts on what the page shows: with a source editor as the document,
/// on its text; otherwise on the drawing's selection.
const EDIT_MENU: &[MenuEntry] = &[
    command(Command::Undo),
    command(Command::Redo),
    MenuEntry::Only(MenuContext::Drawing, &[command(Command::RepeatLastCommand)]),
    Separator,
    command(Command::Cut),
    command(Command::Copy),
    command(Command::Paste),
    MenuEntry::Only(
        MenuContext::SourceEditor,
        &[
            command(Command::DuplicateLine),
            command(Command::DeleteLine),
        ],
    ),
    MenuEntry::Only(
        MenuContext::Drawing,
        &[command(Command::Duplicate), command(Command::Delete)],
    ),
    Separator,
    command(Command::SelectAll),
    MenuEntry::Only(MenuContext::Drawing, &[command(Command::DeselectAll)]),
    Separator,
    MenuEntry::Only(
        MenuContext::SourceEditor,
        &[
            command(Command::ToggleLineComment),
            command(Command::FormatDocument),
            command(Command::GoToLine),
            Separator,
            command(Command::FindCodeDocument),
        ],
    ),
    // What is done to the selection, then to one object, then finding one.
    MenuEntry::Only(
        MenuContext::Drawing,
        &[
            command(Command::MoveSelection),
            command(Command::StretchSelection),
            command(Command::ArraySelection),
            command(Command::ReplaceInstance),
            command(Command::SelectionBulkEdit),
            Separator,
            command(Command::RenameSelection),
            command(Command::ObjectProperties),
            Separator,
            command(Command::FindInDesign),
            command(Command::FindReplace),
        ],
    ),
    Separator,
    settings(SettingsCommand::Preferences),
];

/// View: how near, what is drawn over the drawing, which panels stand
/// around it, and how much of the screen it takes.
const VIEW_MENU: &[MenuEntry] = &[
    command(Command::ZoomIn),
    command(Command::ZoomOut),
    command(Command::ZoomOneToOne),
    command(Command::ZoomFit),
    command(Command::FitSchematicContent),
    command(Command::ZoomToSelection),
    command(Command::ZoomBox),
    Separator,
    MenuEntry::Submenu {
        label: "Grid",
        entries: GRID_MENU,
    },
    command(Command::VisibilityOptions),
    Separator,
    command_as(Command::ToggleNavigator, "Navigator"),
    command_as(Command::ToggleInspector, "Inspector"),
    command_as(Command::ToggleConsole, "Console"),
    results(ResultsCommand::ToggleResultsSplit),
    command_as(Command::OpenProblems, "Problems"),
    Separator,
    // Folding, offered while a source editor is the document on screen.
    MenuEntry::Only(
        MenuContext::SourceEditor,
        &[MenuEntry::Submenu {
            label: "Folding",
            entries: FOLDING_MENU,
        }],
    ),
    command_as(Command::ToggleFocusMode, "Focus mode"),
    command_as(Command::ToggleFullScreen, "Full screen"),
];

/// The grid's styles, one of which is drawn; G steps through them.
const GRID_MENU: &[MenuEntry] = &[
    MenuEntry::radio(
        Command::SetGridStyle(crate::state::GridStyle::Dots).key(),
        "Dots",
    ),
    MenuEntry::radio(
        Command::SetGridStyle(crate::state::GridStyle::Lines).key(),
        "Lines",
    ),
    MenuEntry::radio(
        Command::SetGridStyle(crate::state::GridStyle::Off).key(),
        "Off",
    ),
    Separator,
    command_as(Command::GridSnapRouting, "Grid, snap and routing\u{2026}"),
];

const FOLDING_MENU: &[MenuEntry] = &[
    command(Command::FoldSelection),
    command(Command::UnfoldAtCaret),
    command(Command::FoldAll),
    command(Command::UnfoldAll),
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
    MenuEntry::Submenu {
        label: "Arrange",
        entries: ARRANGE_MENU,
    },
    command(Command::JoinCollinearWires),
    Separator,
    // The sheets of the active drawing, in the order the strip shows them:
    // navigate first, then author.
    sheets(SheetsCommand::PreviousSheet),
    sheets(SheetsCommand::NextSheet),
    sheets(SheetsCommand::NewSheet),
    sheets(SheetsCommand::DeleteSheet),
    sheets(SheetsCommand::SheetProperties),
    sheets(SheetsCommand::SheetFormatManager),
    sheets(SheetsCommand::SheetDefaults),
    Separator,
    command(Command::CreateHierarchy),
    project(ProjectCommand::DesignManagement),
    command(Command::ConnectivityManager),
    command(Command::ConfigurationSets),
    command(Command::RuleSetup),
    command(Command::ReviewComments),
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
    project(ProjectCommand::CheckAndSave),
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
    simulate(SimulateCommand::ManagePlans),
    // Filling the chosen plan reads directly under choosing it, and above the
    // actions that dispatch what it holds. It is here because the navigator is
    // the only other route to the catalogue, and the navigator is a panel a
    // reader can collapse.
    simulate(SimulateCommand::AddAnalysis),
    simulate(SimulateCommand::Run),
    simulate(SimulateCommand::Stop),
    simulate(SimulateCommand::JobsManager),
    simulate(SimulateCommand::Preflight),
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
    simulate(SimulateCommand::Options),
    command(Command::GenerateNetlist),
    results(ResultsCommand::EditSpecifications),
];

const RESULTS_MENU: &[MenuEntry] = &[
    results(ResultsCommand::ResultViewer(
        crate::workbench::ResultViewer::Waves,
    )),
    results(ResultsCommand::DatasetManifestBrowser),
    results(ResultsCommand::CreateResultDocument),
    results(ResultsCommand::CompareResultDatasets),
    hardcopy(HardcopyCommand::ReportAuthoring),
    Separator,
    MenuEntry::command_as(
        ResultsCommand::ToggleLinkedCursors.key(),
        "Link A/B cursors",
    ),
    results(ResultsCommand::WaveformCalculator),
    results(ResultsCommand::ExpressionDiagnostics),
    results(ResultsCommand::MeasurementLibrary),
    results(ResultsCommand::FamilySlicing),
    results(ResultsCommand::VisualizationDocumentProperties),
];

const VERIFY_MENU: &[MenuEntry] = &[
    MenuEntry::command_as(
        SimulateCommand::Verify(VerificationPage::Yield).key(),
        "Verification cockpit",
    ),
    MenuEntry::command_as(
        SimulateCommand::Verify(VerificationPage::Corners).key(),
        "Corner matrix",
    ),
    MenuEntry::command_as(
        SimulateCommand::Verify(VerificationPage::Optimization).key(),
        "Optimization",
    ),
    MenuEntry::command_as(
        SimulateCommand::Verify(VerificationPage::DeviceSafety).key(),
        "Safe operating area",
    ),
    MenuEntry::command_as(
        SimulateCommand::Verify(VerificationPage::Regression).key(),
        "Regression plan",
    ),
];

const MODELS_MENU: &[MenuEntry] = &[
    MenuEntry::command_as(
        ModelsCommand::Page(ModelsPage::Models).key(),
        "Model & library catalog",
    ),
    models(ModelsCommand::ModelEditor),
    models(ModelsCommand::ModelCorrelation),
    Separator,
    command_as(Command::CompileVerilogA, "Verilog-A/AMS compiler"),
];

const AUTOMATION_MENU: &[MenuEntry] = &[command_as(
    Command::AutomationConsole,
    "Automation workspace",
)];

const WINDOW_MENU: &[MenuEntry] = &[
    command(Command::NewApplicationWindow),
    command(Command::DetachDocument),
    command(Command::MoveDocumentToWindow),
    command(Command::ReattachDocument),
    command(Command::ConsolidateWindows),
    command(Command::MonitorRecovery),
    Separator,
    command(Command::PreviousDocument),
    command(Command::NextDocument),
    Separator,
    settings(SettingsCommand::WorkspaceLayouts),
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
        entries: application_menu_entries(menu),
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
        Command::ManageSourceDocument => WorkbenchIcon::Folder,
        Command::SymbolSave => WorkbenchIcon::Save,
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
        Command::CheckCellview | Command::CheckDesign | Command::ValidateCodeDocument => {
            WorkbenchIcon::Check
        }
        Command::VerilogABuildSettings => WorkbenchIcon::Sliders,
        Command::FindCodeDocument => WorkbenchIcon::Search,
        Command::SourceLanguageTools => WorkbenchIcon::Target,
        Command::CompareGeneratedRevisions => WorkbenchIcon::Compare,
        Command::ExportNetlist(crate::io::NetlistFormat::Spice) => WorkbenchIcon::Export,
        Command::AutomationConsole => WorkbenchIcon::Terminal,
        _ => return None,
    };
    Some(icon)
}

/// The icon a feature command's menu row shows, by the rule
/// [`command_icon`] follows.
pub(in crate::workbench) fn feature_command_icon(key: CommandKey) -> Option<WorkbenchIcon> {
    if let Some(command) = HardcopyCommand::from_key(key) {
        return match command {
            HardcopyCommand::AddReportPage => Some(WorkbenchIcon::Add),
            HardcopyCommand::SaveReportDocument => Some(WorkbenchIcon::Save),
            HardcopyCommand::ReportPageProperties => Some(WorkbenchIcon::Sliders),
            HardcopyCommand::PrintHardcopy => Some(WorkbenchIcon::File),
            _ => None,
        };
    }
    if let Some(command) = ProjectCommand::from_key(key) {
        return match command {
            ProjectCommand::ProjectLauncher => Some(WorkbenchIcon::Folder),
            ProjectCommand::NewProject | ProjectCommand::NewCell => Some(WorkbenchIcon::Add),
            ProjectCommand::Save => Some(WorkbenchIcon::Save),
            _ => None,
        };
    }
    if let Some(command) = SimulateCommand::from_key(key) {
        return match command {
            SimulateCommand::Run => Some(WorkbenchIcon::Run),
            SimulateCommand::Stop => Some(WorkbenchIcon::Stop),
            SimulateCommand::Preflight => Some(WorkbenchIcon::Check),
            SimulateCommand::Options => Some(WorkbenchIcon::Sliders),
            _ => None,
        };
    }
    if let Some(command) = ModelsCommand::from_key(key) {
        return match command {
            ModelsCommand::ModelSaveRevision => Some(WorkbenchIcon::Save),
            ModelsCommand::RescanModelLibraries => Some(WorkbenchIcon::Refresh),
            ModelsCommand::ModelValidate => Some(WorkbenchIcon::Check),
            ModelsCommand::ModelCompareRelease => Some(WorkbenchIcon::Compare),
            ModelsCommand::ModelRunQualificationTests => Some(WorkbenchIcon::Run),
            _ => None,
        };
    }
    match ResultsCommand::from_key(key)? {
        ResultsCommand::ImportResultDataset => Some(WorkbenchIcon::Folder),
        ResultsCommand::WaveformCalculator => Some(WorkbenchIcon::Sliders),
        ResultsCommand::EditSpecifications => Some(WorkbenchIcon::Target),
        ResultsCommand::CompareResultDatasets => Some(WorkbenchIcon::Compare),
        ResultsCommand::ExportWaveformsCsv => Some(WorkbenchIcon::Export),
        _ => None,
    }
}
