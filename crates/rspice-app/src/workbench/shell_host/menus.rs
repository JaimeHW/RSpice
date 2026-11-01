//! What each application menu holds, written down as entries the menu
//! renderer draws and a test can read. The source editor's right-click menu
//! is written the same way.

use crate::state::ComponentType;
use crate::state::ModelsPage;
use crate::state::SimulationPage;
use crate::state::VerificationPage;
use crate::state::command_catalog::CommandKey;
use crate::workbench::ResultViewer;
use crate::workbench::chrome::title_bar::application_menus::ApplicationMenu;
use crate::workbench::chrome::title_bar::menu_entries::MenuEntry::Separator;
use crate::workbench::chrome::title_bar::menu_entries::{MenuContext, MenuEntry};
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
        ApplicationMenu::Tools => TOOLS_MENU,
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

/// Design: move through the hierarchy, place, arrange and lay out sheets,
/// manage the design, then check it. A symbol view places its own shapes,
/// and a Verilog-A view compiles.
const DESIGN_MENU: &[MenuEntry] = &[
    command(Command::DescendHierarchyDirect),
    command(Command::DescendHierarchy),
    command(Command::AscendHierarchy),
    // Inside a symbol edited in place, the way back out stands where the
    // way in did.
    MenuEntry::Only(
        MenuContext::NotSymbolView,
        &[command(Command::EditSymbolInPlace)],
    ),
    MenuEntry::Only(
        MenuContext::SymbolView,
        &[command(Command::ExitSymbolInPlace)],
    ),
    command(Command::CreateHierarchy),
    project(ProjectCommand::CreateSymbol),
    Separator,
    MenuEntry::Only(
        MenuContext::NotSymbolView,
        &[MenuEntry::Submenu {
            label: "Place",
            entries: PLACE_MENU,
        }],
    ),
    MenuEntry::Only(
        MenuContext::SymbolView,
        &[MenuEntry::Submenu {
            label: "Place",
            entries: SYMBOL_PLACE_MENU,
        }],
    ),
    MenuEntry::Submenu {
        label: "Arrange",
        entries: ARRANGE_MENU,
    },
    MenuEntry::Submenu {
        label: "Sheet",
        entries: SHEET_MENU,
    },
    Separator,
    project(ProjectCommand::Variants),
    project(ProjectCommand::Annotate),
    command(Command::ConnectivityManager),
    command(Command::ReviewComments),
    command(Command::EngineeringTableView),
    Separator,
    // The design checks, narrowest first: the cellview in front of the
    // reader, the whole design, and the check that gates a save.
    command(Command::CheckCellview),
    command(Command::CheckDesign),
    MenuEntry::Only(
        MenuContext::NotSymbolView,
        &[project(ProjectCommand::CheckAndSave)],
    ),
    MenuEntry::Only(MenuContext::SymbolView, &[command(Command::SymbolSave)]),
    MenuEntry::Submenu {
        label: "Violations",
        entries: VIOLATIONS_MENU,
    },
    command(Command::RuleSetup),
    MenuEntry::Only(
        MenuContext::VerilogAView,
        &[
            Separator,
            command(Command::CompileVerilogA),
            command(Command::VerilogABuildSettings),
        ],
    ),
];

/// What a schematic places: the tools, then a part of each kind the
/// one-key placements put down.
const PLACE_MENU: &[MenuEntry] = &[
    command_as(Command::PlaceInstance, "Instance\u{2026}"),
    command_as(Command::PlaceWire, "Wire"),
    command_as(Command::PlaceBus, "Bus"),
    command_as(Command::PlaceBusTap, "Bus tap"),
    command_as(Command::PlaceLabel, "Net label"),
    command_as(Command::PlaceOffSheetConnector, "Off-sheet connector"),
    command_as(Command::PlacePin, "Pins\u{2026}"),
    command_as(Command::PlaceJunction, "Junction"),
    command_as(Command::PlaceText, "Text or note\u{2026}"),
    command_as(Command::PlaceShape, "Documentation shape\u{2026}"),
    Separator,
    command_as(Command::Place(ComponentType::Resistor), "Resistor"),
    command_as(Command::Place(ComponentType::Capacitor), "Capacitor"),
    command_as(Command::Place(ComponentType::Inductor), "Inductor"),
    command_as(Command::Place(ComponentType::Diode), "Diode"),
    command_as(
        Command::Place(ComponentType::VoltageSource),
        "Voltage source",
    ),
    command_as(
        Command::Place(ComponentType::CurrentSource),
        "Current source",
    ),
    command_as(Command::Place(ComponentType::Ground), "Ground"),
];

/// What a symbol view places: its drawing tools, then the edits to one pin.
const SYMBOL_PLACE_MENU: &[MenuEntry] = &[
    command_as(Command::SymbolPinTool, "Pin"),
    command_as(Command::SymbolPolylineTool, "Line"),
    command_as(Command::SymbolRectangleTool, "Rectangle"),
    command_as(Command::SymbolCircleTool, "Circle"),
    command_as(Command::SymbolArcTool, "Arc"),
    command_as(Command::SymbolPolygonTool, "Polygon"),
    command_as(Command::SymbolTextTool, "Text"),
    Separator,
    command(Command::SymbolRotatePin),
    command(Command::SymbolMirrorPin),
    command(Command::SymbolUpdatePinsFromContract),
];

/// The sheets of the active drawing, in the order the strip shows them:
/// navigate, then add and remove, then the sheet's own setup in the order
/// it is done.
const SHEET_MENU: &[MenuEntry] = &[
    sheets(SheetsCommand::PreviousSheet),
    sheets(SheetsCommand::NextSheet),
    sheets(SheetsCommand::NewSheet),
    MenuEntry::command_as(SheetsCommand::DeleteSheet.key(), "Delete\u{2026}"),
    Separator,
    sheets(SheetsCommand::SheetProperties),
    sheets(SheetsCommand::SheetFormatManager),
    sheets(SheetsCommand::SheetDefaults),
];

/// The findings the checks left: step through them, or clear them.
const VIOLATIONS_MENU: &[MenuEntry] = &[
    command(Command::NextViolation),
    command(Command::PreviousViolation),
    Separator,
    command(Command::ClearChecks),
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
    command(Command::JoinCollinearWires),
];

/// Simulate: choose and fill the active plan, set it up, check and run it,
/// then read the operating point back onto the schematic.
const SIMULATE_MENU: &[MenuEntry] = &[
    // The manager owns which plan is active, and every row below it acts on
    // that plan. Filling the chosen plan reads directly under choosing it;
    // the navigator, the catalogue's other route, is a panel a reader can
    // collapse.
    simulate(SimulateCommand::ManagePlans),
    simulate(SimulateCommand::AddAnalysis),
    Separator,
    MenuEntry::Submenu {
        label: "Setup",
        entries: SETUP_MENU,
    },
    Separator,
    simulate(SimulateCommand::Preflight),
    MenuEntry::command_as(SimulateCommand::Run.key(), "Run"),
    MenuEntry::command_as(SimulateCommand::Stop.key(), "Stop"),
    command(Command::GenerateNetlist),
    simulate(SimulateCommand::JobsManager),
    Separator,
    command_as(Command::PlaceProbe, "Probe"),
    command(Command::AnnotateOperatingPoint),
    MenuEntry::command_as(
        ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Op).key(),
        "Operating-point table",
    ),
];

/// The Simulation Studio's pages in the order a plan is authored, then what
/// the plan draws on from outside it.
const SETUP_MENU: &[MenuEntry] = &[
    simulate(SimulateCommand::Page(SimulationPage::Analyses)),
    simulate(SimulateCommand::Page(SimulationPage::Excitations)),
    simulate(SimulateCommand::Page(SimulationPage::Variables)),
    simulate(SimulateCommand::Page(SimulationPage::Outputs)),
    simulate(SimulateCommand::Page(SimulationPage::Specifications)),
    simulate(SimulateCommand::Page(SimulationPage::RunSet)),
    simulate(SimulateCommand::Page(SimulationPage::Models)),
    simulate(SimulateCommand::Page(SimulationPage::Solver)),
    simulate(SimulateCommand::Page(SimulationPage::Save)),
    Separator,
    command(Command::ConfigurationSets),
    command_as(
        Command::OpenWorkspace(Workspace::Stimulus),
        "Stimulus library",
    ),
];

/// Results: where results are read, what is built from them, then what is
/// done on the plot in front.
const RESULTS_MENU: &[MenuEntry] = &[
    results(ResultsCommand::DatasetManifestBrowser),
    MenuEntry::command_as(viewer(ResultViewer::Waves), "Waveforms"),
    MenuEntry::Submenu {
        label: "View as",
        entries: VIEW_AS_MENU,
    },
    Separator,
    results(ResultsCommand::CreateResultDocument),
    results(ResultsCommand::AddVisualizationPane),
    results(ResultsCommand::CompareResultDatasets),
    results(ResultsCommand::MeasurementLibrary),
    results(ResultsCommand::FamilySlicing),
    Separator,
    MenuEntry::command_as(
        ResultsCommand::ToggleLinkedCursors.key(),
        "Link A/B cursors",
    ),
    results(ResultsCommand::VisualizationDocumentProperties),
    results(ResultsCommand::WaveformCalculator),
    results(ResultsCommand::ExpressionDiagnostics),
    hardcopy(HardcopyCommand::ReportAuthoring),
    results(ResultsCommand::ClearResults),
];

/// The key of the command that opens `viewer`.
const fn viewer(viewer: ResultViewer) -> CommandKey {
    ResultsCommand::ResultViewer(viewer).key()
}

/// The other viewers the selected result can be read in, each offered only
/// while it fits that result. The operating-point table is Simulate's.
const VIEW_AS_MENU: &[MenuEntry] = &[
    MenuEntry::offered(viewer(ResultViewer::DcSweep), "DC sweep"),
    MenuEntry::offered(viewer(ResultViewer::Bode), "Bode"),
    MenuEntry::offered(viewer(ResultViewer::Nyquist), "Nyquist"),
    MenuEntry::offered(viewer(ResultViewer::Smith), "Smith chart"),
    MenuEntry::offered(viewer(ResultViewer::Polar), "Polar response"),
    MenuEntry::offered(viewer(ResultViewer::PoleZero), "Pole-zero"),
    MenuEntry::offered(viewer(ResultViewer::TransferFunction), "Transfer function"),
    MenuEntry::offered(viewer(ResultViewer::NetworkMatrix), "Network matrix"),
    MenuEntry::offered(viewer(ResultViewer::Fft), "FFT"),
    MenuEntry::offered(
        viewer(ResultViewer::HarmonicBalance),
        "Harmonic balance tones",
    ),
    MenuEntry::offered(viewer(ResultViewer::PhaseNoise), "Phase noise"),
    MenuEntry::offered(viewer(ResultViewer::NoiseContrib), "Noise contribution"),
    MenuEntry::offered(
        viewer(ResultViewer::Contribution),
        "Sensitivity contribution",
    ),
    MenuEntry::offered(viewer(ResultViewer::Eye), "Eye diagram"),
    MenuEntry::offered(viewer(ResultViewer::Hist), "Histogram"),
    MenuEntry::offered(viewer(ResultViewer::Scatter), "Correlation scatter"),
    MenuEntry::offered(viewer(ResultViewer::BoxViolin), "Box and violin"),
    MenuEntry::offered(viewer(ResultViewer::Events), "Event history"),
    MenuEntry::offered(viewer(ResultViewer::Soa), "Safe operating area"),
    MenuEntry::offered(viewer(ResultViewer::Optimization), "Optimization"),
    MenuEntry::offered(viewer(ResultViewer::Specs), "Specification results"),
    MenuEntry::offered(viewer(ResultViewer::Table), "Sample table"),
    MenuEntry::offered(viewer(ResultViewer::Manifest), "Dataset manifest"),
];

/// Verify: the analyses run over a design to sign it off, each opening its
/// Verify page, with the specification matrix they check against.
const VERIFY_MENU: &[MenuEntry] = &[
    simulate(SimulateCommand::Verify(VerificationPage::Yield)),
    simulate(SimulateCommand::Verify(VerificationPage::Corners)),
    simulate(SimulateCommand::Verify(VerificationPage::Tuning)),
    simulate(SimulateCommand::Verify(VerificationPage::Optimization)),
    simulate(SimulateCommand::Verify(VerificationPage::DeviceSafety)),
    results(ResultsCommand::EditSpecifications),
    simulate(SimulateCommand::Verify(VerificationPage::Regression)),
];

/// Tools: the libraries and models a design draws on, then the editors for
/// netlists and scripts.
const TOOLS_MENU: &[MenuEntry] = &[
    models(ModelsCommand::Page(ModelsPage::Models)),
    models(ModelsCommand::ModelBrowser),
    models(ModelsCommand::ModelEditor),
    models(ModelsCommand::ModelCreateProjectCopy),
    models(ModelsCommand::ModelCorrelation),
    models(ModelsCommand::PdkSettings),
    models(ModelsCommand::RescanModelLibraries),
    Separator,
    command_as(
        Command::OpenWorkspace(Workspace::Netlist),
        "Netlist and script editor",
    ),
    command(Command::AutomationConsole),
];

/// Window: the application's windows and the document in front, the
/// workspaces and the documents in turn, then how the panels are laid out.
const WINDOW_MENU: &[MenuEntry] = &[
    command(Command::NewApplicationWindow),
    MenuEntry::Submenu {
        label: "Move document to",
        entries: MOVE_DOCUMENT_MENU,
    },
    command(Command::ReattachDocument),
    // Gathering windows back and finding ones left off screen are a desktop
    // application's: the browser and a tablet show one window.
    MenuEntry::Only(
        MenuContext::DesktopApplication,
        &[
            command(Command::ConsolidateWindows),
            command(Command::MonitorRecovery),
        ],
    ),
    Separator,
    MenuEntry::Submenu {
        label: "Workspace",
        entries: WORKSPACE_MENU,
    },
    command(Command::PreviousDocument),
    command(Command::NextDocument),
    Separator,
    MenuEntry::Submenu {
        label: "Layout",
        entries: LAYOUT_MENU,
    },
    command_as(Command::WindowManager, "Windows\u{2026}"),
];

/// Where the document in front goes: a window of its own, or one of the
/// windows already open, chosen in the dialog.
const MOVE_DOCUMENT_MENU: &[MenuEntry] = &[
    command_as(Command::DetachDocument, "New window\u{2026}"),
    command_as(Command::MoveDocumentToWindow, "Another window\u{2026}"),
];

/// The eight workspaces in the order of their keys, Alt+1 to Alt+8: the
/// Stimulus library, fourth on the rail, kept the key it was given last.
const WORKSPACE_MENU: &[MenuEntry] = &[
    command_as(Command::OpenWorkspace(Workspace::Project), "Project"),
    command_as(Command::OpenWorkspace(Workspace::Design), "Design"),
    command_as(Command::OpenWorkspace(Workspace::Simulate), "Simulate"),
    command_as(Command::OpenWorkspace(Workspace::Results), "Results"),
    command_as(Command::OpenWorkspace(Workspace::Verify), "Verify"),
    command_as(Command::OpenWorkspace(Workspace::Models), "Models"),
    command_as(
        Command::OpenWorkspace(Workspace::Netlist),
        "Netlist and script editor",
    ),
    command_as(
        Command::OpenWorkspace(Workspace::Stimulus),
        "Stimulus library",
    ),
];

/// The saved panel layouts, chosen in their dialog, and the way back to
/// the workspace's own.
const LAYOUT_MENU: &[MenuEntry] = &[
    MenuEntry::command_as(
        SettingsCommand::WorkspaceLayouts.key(),
        "Saved layouts\u{2026}",
    ),
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
    compact_submenu(ApplicationMenu::Tools),
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
