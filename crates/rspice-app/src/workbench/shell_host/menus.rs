//! What each application menu holds, written down as entries the menu
//! renderer draws and a test can read. The source editor's right-click menu
//! is written the same way.

use crate::schematic::view::sheet_visibility::SchematicObjectKind;
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
use crate::workbench::code::commands::CodeCommand;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::design::commands::DesignCommand;
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

/// The code feature's command's row under the label it declares.
const fn code(command: CodeCommand) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// The design feature's command's row under the label it declares.
const fn design(command: DesignCommand) -> MenuEntry {
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
///
/// Its 23 rows have 21 letters among their labels to give as access keys:
/// Save all (which has its key) and Live session (shown only to a signed-in
/// account) go without.
const FILE_MENU: &[MenuEntry] = &[
    project(ProjectCommand::NewProject).access('N'),
    project(ProjectCommand::NewCell).access('W'),
    project(ProjectCommand::NewView).access('V'),
    code(CodeCommand::NewVerilogADevice).access('G'),
    Separator,
    // A schematic file is opened as a document of its own only on the start
    // screen; inside a project it is imported as a new cell view.
    MenuEntry::Only(
        MenuContext::NoProject,
        &[project(ProjectCommand::OpenSchematic).access('M')],
    ),
    project(ProjectCommand::OpenProject).access('O'),
    code(CodeCommand::OpenNetlist).access('T'),
    MenuEntry::submenu("Open recent", OPEN_RECENT_MENU).access('R'),
    Separator,
    command(Command::CloseActiveDocument).access('C'),
    command(Command::CloseOtherDocuments).access('H'),
    command(Command::CloseAllDocuments).access('L'),
    project(ProjectCommand::CloseProject).access('J'),
    Separator,
    project(ProjectCommand::Save).access('S'),
    project(ProjectCommand::SaveAs).access('A'),
    project(ProjectCommand::SaveAll),
    command(Command::RevertActiveDocument).access('D'),
    project(ProjectCommand::RevisionHistory).access('Y'),
    Separator,
    MenuEntry::submenu("Import", IMPORT_MENU).access('I'),
    MenuEntry::submenu("Export", EXPORT_MENU).access('E'),
    Separator,
    cloud(CloudCommand::PublishToWeb).access('B'),
    cloud(CloudCommand::LiveSession),
    Separator,
    hardcopy(HardcopyCommand::PrintHardcopy).access('P'),
    MenuEntry::Only(
        MenuContext::ExitInFileMenu,
        &[Separator, command(Command::Exit).access('X')],
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
    )
    .access('A'),
];

const IMPORT_MENU: &[MenuEntry] = &[
    MenuEntry::command_as(CodeCommand::ImportNetlist.key(), "SPICE deck\u{2026}").access('S'),
    MenuEntry::command_as(CodeCommand::ImportVerilogA.key(), "Verilog-A\u{2026}").access('V'),
    MenuEntry::command_as(
        CodeCommand::ImportLtspiceSchematic.key(),
        "LTspice schematic (.asc)\u{2026}",
    )
    .access('L'),
    MenuEntry::command_as(
        CodeCommand::ImportQspiceSchematic.key(),
        "QSPICE schematic (.qsch)\u{2026}",
    )
    .access('Q'),
    MenuEntry::command_as(
        ResultsCommand::ImportResultDataset.key(),
        "Result dataset\u{2026}",
    )
    .access('R'),
    MenuEntry::Only(
        MenuContext::ProjectOpen,
        &[MenuEntry::command_as(
            ProjectCommand::OpenSchematic.key(),
            "Schematic into new cell view\u{2026}",
        )
        .access('C')],
    ),
];

const EXPORT_MENU: &[MenuEntry] = &[
    MenuEntry::command_as(
        HardcopyCommand::ExportActiveView.key(),
        "Image or PDF\u{2026}",
    )
    .access('I'),
    MenuEntry::command_as(
        CodeCommand::ExportNetlist(crate::io::NetlistFormat::Spice).key(),
        "SPICE netlist\u{2026}",
    )
    .access('S'),
    MenuEntry::command_as(
        CodeCommand::ExportNetlist(crate::io::NetlistFormat::Spectre).key(),
        "Spectre netlist\u{2026}",
    )
    .access('N'),
    MenuEntry::command_as(ResultsCommand::ExportWaveformsCsv.key(), "Dataset\u{2026}").access('D'),
    MenuEntry::command_as(
        CodeCommand::ExportBillOfMaterials.key(),
        "Bill of materials (CSV)\u{2026}",
    )
    .access('B'),
];

/// Edit acts on what the page shows: with a source editor as the document,
/// on its text; otherwise on the drawing's selection. With the drawing's,
/// 22 rows have 20 letters to give as access keys: Deselect all and the
/// Select submenu go without.
const EDIT_MENU: &[MenuEntry] = &[
    command(Command::Undo).access('U'),
    command(Command::Redo).access('R'),
    MenuEntry::Only(
        MenuContext::Drawing,
        &[command(Command::RepeatLastCommand).access('L')],
    ),
    Separator,
    command(Command::Cut).access('T'),
    command(Command::Copy).access('C'),
    // A picture of the selection or the view, for a document or a message.
    MenuEntry::Only(
        MenuContext::Drawing,
        &[hardcopy(HardcopyCommand::CopyAsImage).access('G')],
    ),
    command(Command::Paste).access('P'),
    MenuEntry::Only(
        MenuContext::SourceEditor,
        &[
            code(CodeCommand::DuplicateLine).access('L'),
            code(CodeCommand::DeleteLine).access('D'),
        ],
    ),
    MenuEntry::Only(
        MenuContext::Drawing,
        &[
            command(Command::Duplicate).access('E'),
            command(Command::Delete).access('D'),
        ],
    ),
    Separator,
    command(Command::SelectAll).access('A'),
    MenuEntry::Only(
        MenuContext::Drawing,
        &[
            command(Command::DeselectAll),
            // A symbol view's objects are its own; these kinds are a
            // schematic's.
            MenuEntry::Only(
                MenuContext::NotSymbolView,
                &[MenuEntry::submenu("Select", SELECT_MENU)],
            ),
        ],
    ),
    Separator,
    MenuEntry::Only(
        MenuContext::SourceEditor,
        &[
            code(CodeCommand::ToggleLineComment).access('M'),
            code(CodeCommand::FormatDocument).access('O'),
            code(CodeCommand::GoToLine).access('G'),
            Separator,
            code(CodeCommand::FindCodeDocument).access('F'),
        ],
    ),
    // What is done to the selection, then to one object, then finding one.
    MenuEntry::Only(
        MenuContext::Drawing,
        &[
            command(Command::MoveSelection).access('V'),
            command(Command::StretchSelection).access('H'),
            command(Command::ArraySelection).access('Y'),
            design(DesignCommand::ReplaceInstance).access('S'),
            command(Command::SwapToRSpiceSymbols).access('W'),
            design(DesignCommand::SelectionBulkEdit).access('B'),
            Separator,
            design(DesignCommand::RenameSelection).access('M'),
            command(Command::ObjectProperties).access('O'),
            Separator,
            command(Command::FindInDesign).access('F'),
            command(Command::FindReplace).access('I'),
        ],
    ),
    // On macOS, Settings is the RSpice menu's.
    MenuEntry::Only(
        MenuContext::NoApplicationMenu,
        &[
            Separator,
            settings(SettingsCommand::Preferences).access('N'),
        ],
    ),
];

/// Every object of one kind on the active sheet, in place of the selection.
const SELECT_MENU: &[MenuEntry] = &[
    command_as(
        Command::SelectOfKind(SchematicObjectKind::Instances),
        "Instances",
    )
    .access('I'),
    command_as(Command::SelectOfKind(SchematicObjectKind::Wires), "Wires").access('W'),
    command_as(
        Command::SelectOfKind(SchematicObjectKind::NetLabels),
        "Net labels",
    )
    .access('N'),
    command_as(Command::SelectOfKind(SchematicObjectKind::Pins), "Pins").access('P'),
    command_as(Command::SelectOfKind(SchematicObjectKind::Text), "Text").access('T'),
    command_as(Command::SelectOfKind(SchematicObjectKind::Shapes), "Shapes").access('S'),
];

/// View: how near, what is drawn over the drawing, which panels stand
/// around it, and how much of the screen it takes.
const VIEW_MENU: &[MenuEntry] = &[
    command(Command::ZoomIn).access('I'),
    command(Command::ZoomOut).access('O'),
    command(Command::ZoomOneToOne).access('Z'),
    command(Command::ZoomFit).access('S'),
    command(Command::FitSchematicContent).access('F'),
    command(Command::ZoomToSelection).access('T'),
    command(Command::ZoomBox).access('B'),
    command(Command::ZoomPrevious).access('V'),
    Separator,
    MenuEntry::submenu("Grid", GRID_MENU).access('G'),
    design(DesignCommand::VisibilityOptions).access('D'),
    Separator,
    command_as(Command::ToggleNavigator, "Navigator").access('N'),
    command_as(Command::ToggleInspector, "Inspector").access('E'),
    command_as(Command::ToggleConsole, "Console").access('C'),
    results(ResultsCommand::ToggleResultsSplit).access('R'),
    command_as(Command::OpenProblems, "Problems").access('P'),
    Separator,
    // Folding, offered while a source editor is the document on screen.
    MenuEntry::Only(
        MenuContext::SourceEditor,
        &[MenuEntry::submenu("Folding", FOLDING_MENU).access('L')],
    ),
    command_as(Command::ToggleFocusMode, "Focus mode").access('M'),
    command_as(Command::ToggleFullScreen, "Full screen").access('U'),
];

/// The grid's styles, one of which is drawn; G steps through them.
const GRID_MENU: &[MenuEntry] = &[
    MenuEntry::radio(
        Command::SetGridStyle(crate::state::GridStyle::Dots).key(),
        "Dots",
    )
    .access('D'),
    MenuEntry::radio(
        Command::SetGridStyle(crate::state::GridStyle::Lines).key(),
        "Lines",
    )
    .access('L'),
    MenuEntry::radio(
        Command::SetGridStyle(crate::state::GridStyle::Off).key(),
        "Off",
    )
    .access('O'),
    Separator,
    MenuEntry::command_as(
        DesignCommand::GridSnapRouting.key(),
        "Grid, snap and routing\u{2026}",
    )
    .access('G'),
];

const FOLDING_MENU: &[MenuEntry] = &[
    code(CodeCommand::FoldSelection).access('F'),
    code(CodeCommand::UnfoldAtCaret).access('C'),
    code(CodeCommand::FoldAll).access('A'),
    code(CodeCommand::UnfoldAll).access('U'),
];

/// Design: move through the hierarchy, place, arrange and lay out sheets,
/// manage the design, then check it. A symbol view places its own shapes,
/// and a Verilog-A view compiles.
const DESIGN_MENU: &[MenuEntry] = &[
    command(Command::DescendHierarchyDirect).access('D'),
    command(Command::DescendHierarchy).access('W'),
    command(Command::AscendHierarchy).access('U'),
    // Inside a symbol edited in place, the way back out stands where the
    // way in did, on the same letter.
    MenuEntry::Only(
        MenuContext::NotSymbolView,
        &[command(Command::EditSymbolInPlace).access('E')],
    ),
    MenuEntry::Only(
        MenuContext::SymbolView,
        &[command(Command::ExitSymbolInPlace).access('E')],
    ),
    design(DesignCommand::CreateHierarchy).access('H'),
    project(ProjectCommand::CreateSymbol).access('F'),
    Separator,
    MenuEntry::Only(
        MenuContext::NotSymbolView,
        &[MenuEntry::submenu("Place", PLACE_MENU).access('P')],
    ),
    MenuEntry::Only(
        MenuContext::SymbolView,
        &[MenuEntry::submenu("Place", SYMBOL_PLACE_MENU).access('P')],
    ),
    MenuEntry::submenu("Arrange", ARRANGE_MENU).access('A'),
    // No letter is left on this menu that its name holds.
    MenuEntry::Only(
        MenuContext::NotSymbolView,
        &[MenuEntry::submenu("Standard values", STANDARD_VALUES_MENU)],
    ),
    MenuEntry::submenu("Sheet", SHEET_MENU).access('S'),
    Separator,
    project(ProjectCommand::Variants).access('N'),
    project(ProjectCommand::Annotate).access('O'),
    design(DesignCommand::ConnectivityManager).access('Y'),
    command(Command::HighlightNet).access('T'),
    design(DesignCommand::ReviewComments).access('R'),
    design(DesignCommand::EngineeringTableView).access('J'),
    Separator,
    // The design checks, narrowest first: the cellview in front of the
    // reader, the whole design, and the check that gates a save.
    command(Command::CheckCellview).access('L'),
    command(Command::CheckDesign).access('K'),
    MenuEntry::Only(
        MenuContext::NotSymbolView,
        &[project(ProjectCommand::CheckAndSave).access('C')],
    ),
    MenuEntry::Only(
        MenuContext::SymbolView,
        &[command(Command::SymbolSave).access('V')],
    ),
    MenuEntry::submenu("Violations", VIOLATIONS_MENU).access('I'),
    design(DesignCommand::RuleSetup).access('G'),
    MenuEntry::Only(
        MenuContext::VerilogAView,
        &[
            Separator,
            code(CodeCommand::CompileVerilogA).access('M'),
            code(CodeCommand::CancelVerilogACompile).access('V'),
            code(CodeCommand::VerilogABuildSettings).access('B'),
        ],
    ),
];

/// What a schematic places: the tools, then a part of each kind the
/// one-key placements put down.
const PLACE_MENU: &[MenuEntry] = &[
    command_as(Command::PlaceInstance, "Instance\u{2026}").access('I'),
    command_as(Command::PlaceWire, "Wire").access('W'),
    command_as(Command::PlaceBus, "Bus").access('B'),
    command_as(Command::PlaceBusTap, "Bus tap").access('T'),
    command_as(Command::PlaceLabel, "Net label").access('L'),
    command_as(Command::PlaceOffSheetConnector, "Off-sheet connector").access('O'),
    command_as(Command::PlacePin, "Pins\u{2026}").access('P'),
    command_as(Command::PlaceJunction, "Junction").access('J'),
    command_as(Command::PlaceText, "Text or note\u{2026}").access('N'),
    command_as(Command::PlaceShape, "Documentation shape\u{2026}").access('S'),
    Separator,
    command_as(Command::Place(ComponentType::Resistor), "Resistor").access('R'),
    command_as(Command::Place(ComponentType::Capacitor), "Capacitor").access('A'),
    command_as(Command::Place(ComponentType::Inductor), "Inductor").access('U'),
    command_as(Command::Place(ComponentType::Diode), "Diode").access('D'),
    command_as(
        Command::Place(ComponentType::VoltageSource),
        "Voltage source",
    )
    .access('V'),
    command_as(
        Command::Place(ComponentType::CurrentSource),
        "Current source",
    )
    .access('C'),
    command_as(Command::Place(ComponentType::Ground), "Ground").access('G'),
];

/// What a symbol view places: its drawing tools, then the edits to one pin.
const SYMBOL_PLACE_MENU: &[MenuEntry] = &[
    command_as(Command::SymbolPinTool, "Pin").access('P'),
    command_as(Command::SymbolPolylineTool, "Line").access('L'),
    command_as(Command::SymbolRectangleTool, "Rectangle").access('R'),
    command_as(Command::SymbolCircleTool, "Circle").access('C'),
    command_as(Command::SymbolArcTool, "Arc").access('A'),
    command_as(Command::SymbolPolygonTool, "Polygon").access('G'),
    command_as(Command::SymbolTextTool, "Text").access('T'),
    Separator,
    command(Command::SymbolRotatePin).access('N'),
    command(Command::SymbolMirrorPin).access('M'),
    command(Command::SymbolUpdatePinsFromContract).access('U'),
];

/// The sheets of the active drawing, in the order the strip shows them:
/// navigate, then add and remove, then the sheet's own setup in the order
/// it is done.
const SHEET_MENU: &[MenuEntry] = &[
    sheets(SheetsCommand::PreviousSheet).access('P'),
    sheets(SheetsCommand::NextSheet).access('N'),
    sheets(SheetsCommand::NewSheet).access('E'),
    MenuEntry::command_as(SheetsCommand::DeleteSheet.key(), "Delete\u{2026}").access('D'),
    Separator,
    sheets(SheetsCommand::SheetProperties).access('S'),
    sheets(SheetsCommand::SheetFormatManager).access('F'),
    sheets(SheetsCommand::SheetDefaults).access('H'),
];

/// The findings the checks left: step through them, or clear them.
const VIOLATIONS_MENU: &[MenuEntry] = &[
    command(Command::NextViolation).access('N'),
    command(Command::PreviousViolation).access('P'),
    Separator,
    command(Command::ClearChecks).access('C'),
];

/// The Design menu's Arrange submenu, group by group: turn and flip, line
/// up, space out, put back on the grid.
const ARRANGE_MENU: &[MenuEntry] = &[
    command(Command::RotateSelection).access('R'),
    command(Command::RotateSelectionCounterClockwise).access('C'),
    command(Command::MirrorSelectionHorizontal).access('V'),
    command(Command::MirrorSelectionVertical).access('M'),
    Separator,
    command(Command::AlignSelectionLeft).access('L'),
    command(Command::AlignSelectionCenter).access('H'),
    command(Command::AlignSelectionRight).access('A'),
    command(Command::AlignSelectionTop).access('T'),
    command(Command::AlignSelectionMiddle).access('G'),
    command(Command::AlignSelectionBottom).access('B'),
    Separator,
    command(Command::DistributeSelectionHorizontal).access('D'),
    command(Command::DistributeSelectionVertical).access('E'),
    Separator,
    command(Command::SnapSelectionToGrid).access('S'),
    command(Command::JoinCollinearWires).access('J'),
];

/// Snap the selected values to the project's preferred-number series, and
/// choose the series; the value fields' arrow keys step along the same one.
const STANDARD_VALUES_MENU: &[MenuEntry] = &[
    command(Command::SnapToStandardValue).access('S'),
    Separator,
    MenuEntry::radio(
        Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E6).key(),
        "E6",
    )
    .access('6'),
    MenuEntry::radio(
        Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E12).key(),
        "E12",
    )
    .access('1'),
    MenuEntry::radio(
        Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E24).key(),
        "E24",
    )
    .access('4'),
    MenuEntry::radio(
        Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E48).key(),
        "E48",
    )
    .access('8'),
    MenuEntry::radio(
        Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E96).key(),
        "E96",
    )
    .access('9'),
    MenuEntry::radio(
        Command::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E192).key(),
        "E192",
    )
    .access('2'),
];

/// Simulate: choose and fill the active plan, set it up, check and run it,
/// then read the operating point back onto the schematic.
const SIMULATE_MENU: &[MenuEntry] = &[
    // The manager owns which plan is active, and every row below it acts on
    // that plan. Filling the chosen plan reads directly under choosing it;
    // the navigator, the catalogue's other route, is a panel a reader can
    // collapse.
    simulate(SimulateCommand::ManagePlans).access('M'),
    simulate(SimulateCommand::AddAnalysis).access('A'),
    Separator,
    MenuEntry::submenu("Setup", SETUP_MENU).access('U'),
    Separator,
    simulate(SimulateCommand::Preflight).access('C'),
    MenuEntry::command_as(SimulateCommand::Run.key(), "Run").access('R'),
    MenuEntry::command_as(SimulateCommand::Stop.key(), "Stop").access('S'),
    code(CodeCommand::GenerateNetlist).access('G'),
    simulate(SimulateCommand::JobsManager).access('H'),
    Separator,
    command_as(Command::PlaceProbe, "Probe").access('P'),
    command(Command::AnnotateOperatingPoint).access('D'),
    command(Command::AnnotateTransientVoltages).access('T'),
    MenuEntry::command_as(
        ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Op).key(),
        "Operating-point table",
    )
    .access('O'),
];

/// The Simulation Studio's pages in the order a plan is authored, then what
/// the plan draws on from outside it.
const SETUP_MENU: &[MenuEntry] = &[
    simulate(SimulateCommand::Page(SimulationPage::Analyses)).access('A'),
    simulate(SimulateCommand::Page(SimulationPage::Excitations)).access('E'),
    simulate(SimulateCommand::Page(SimulationPage::Variables)).access('D'),
    simulate(SimulateCommand::Page(SimulationPage::Outputs)).access('O'),
    simulate(SimulateCommand::Page(SimulationPage::Specifications)).access('R'),
    simulate(SimulateCommand::Page(SimulationPage::RunSet)).access('V'),
    simulate(SimulateCommand::Page(SimulationPage::Models)).access('M'),
    simulate(SimulateCommand::Page(SimulationPage::Solver)).access('S'),
    simulate(SimulateCommand::Page(SimulationPage::Save)).access('P'),
    Separator,
    command(Command::ConfigurationSets).access('C'),
    command_as(
        Command::OpenWorkspace(Workspace::Stimulus),
        "Stimulus library",
    )
    .access('L'),
];

/// Results: where results are read, what is built from them, then what is
/// done on the plot in front.
const RESULTS_MENU: &[MenuEntry] = &[
    results(ResultsCommand::DatasetManifestBrowser).access('B'),
    MenuEntry::command_as(viewer(ResultViewer::Waves), "Waveforms").access('W'),
    MenuEntry::submenu("View as", VIEW_AS_MENU).access('V'),
    Separator,
    results(ResultsCommand::CreateResultDocument).access('N'),
    results(ResultsCommand::AddVisualizationPane).access('A'),
    results(ResultsCommand::CompareResultDatasets).access('D'),
    results(ResultsCommand::MeasurementLibrary).access('M'),
    results(ResultsCommand::FamilySlicing).access('F'),
    Separator,
    MenuEntry::command_as(
        ResultsCommand::ToggleLinkedCursors.key(),
        "Link A/B cursors",
    )
    .access('L'),
    results(ResultsCommand::VisualizationDocumentProperties).access('P'),
    results(ResultsCommand::WaveformCalculator).access('C'),
    results(ResultsCommand::ExpressionDiagnostics).access('E'),
    hardcopy(HardcopyCommand::ReportAuthoring).access('R'),
    results(ResultsCommand::ClearResults).access('H'),
];

/// The key of the command that opens `viewer`.
const fn viewer(viewer: ResultViewer) -> CommandKey {
    ResultsCommand::ResultViewer(viewer).key()
}

/// The other viewers the selected result can be read in, each offered only
/// while it fits that result. The operating-point table is Simulate's.
const VIEW_AS_MENU: &[MenuEntry] = &[
    MenuEntry::offered(viewer(ResultViewer::DcSweep), "DC sweep").access('D'),
    MenuEntry::offered(viewer(ResultViewer::Bode), "Bode").access('B'),
    MenuEntry::offered(viewer(ResultViewer::Nyquist), "Nyquist").access('Q'),
    MenuEntry::offered(viewer(ResultViewer::Smith), "Smith chart").access('S'),
    MenuEntry::offered(viewer(ResultViewer::Polar), "Polar response").access('R'),
    MenuEntry::offered(viewer(ResultViewer::PoleZero), "Pole-zero").access('Z'),
    MenuEntry::offered(viewer(ResultViewer::TransferFunction), "Transfer function").access('T'),
    MenuEntry::offered(viewer(ResultViewer::NetworkMatrix), "Network matrix").access('K'),
    MenuEntry::offered(viewer(ResultViewer::Fft), "FFT").access('F'),
    MenuEntry::offered(
        viewer(ResultViewer::HarmonicBalance),
        "Harmonic balance tones",
    )
    .access('H'),
    MenuEntry::offered(viewer(ResultViewer::PhaseNoise), "Phase noise").access('P'),
    MenuEntry::offered(viewer(ResultViewer::NoiseContrib), "Noise contribution").access('N'),
    MenuEntry::offered(
        viewer(ResultViewer::Contribution),
        "Sensitivity contribution",
    )
    .access('I'),
    MenuEntry::offered(viewer(ResultViewer::Eye), "Eye diagram").access('Y'),
    MenuEntry::offered(viewer(ResultViewer::Hist), "Histogram").access('G'),
    MenuEntry::offered(viewer(ResultViewer::Scatter), "Correlation scatter").access('C'),
    MenuEntry::offered(viewer(ResultViewer::BoxViolin), "Box and violin").access('V'),
    MenuEntry::offered(viewer(ResultViewer::Events), "Event history").access('E'),
    MenuEntry::offered(viewer(ResultViewer::Soa), "Safe operating area").access('A'),
    MenuEntry::offered(viewer(ResultViewer::Optimization), "Optimization").access('O'),
    MenuEntry::offered(viewer(ResultViewer::Specs), "Specification results").access('U'),
    MenuEntry::offered(viewer(ResultViewer::Table), "Sample table").access('L'),
    MenuEntry::offered(viewer(ResultViewer::Manifest), "Dataset manifest").access('M'),
];

/// Verify: the analyses run over a design to sign it off, each opening its
/// Verify page, with the specification matrix they check against.
const VERIFY_MENU: &[MenuEntry] = &[
    simulate(SimulateCommand::Verify(VerificationPage::Yield)).access('M'),
    simulate(SimulateCommand::Verify(VerificationPage::Corners)).access('C'),
    simulate(SimulateCommand::Verify(VerificationPage::Tuning)).access('P'),
    simulate(SimulateCommand::Verify(VerificationPage::Optimization)).access('O'),
    simulate(SimulateCommand::Verify(VerificationPage::DeviceSafety)).access('A'),
    results(ResultsCommand::EditSpecifications).access('S'),
    simulate(SimulateCommand::Verify(VerificationPage::Regression)).access('R'),
];

/// Tools: the libraries and models a design draws on, then the editors for
/// netlists and scripts.
const TOOLS_MENU: &[MenuEntry] = &[
    models(ModelsCommand::Page(ModelsPage::Models)).access('L'),
    models(ModelsCommand::ModelBrowser).access('B'),
    models(ModelsCommand::ModelEditor).access('E'),
    models(ModelsCommand::ModelCreateProjectCopy).access('C'),
    models(ModelsCommand::ModelCorrelation).access('M'),
    models(ModelsCommand::PdkSettings).access('P'),
    models(ModelsCommand::RescanModelLibraries).access('R'),
    Separator,
    command_as(
        Command::OpenWorkspace(Workspace::Netlist),
        "Netlist and script editor",
    )
    .access('N'),
    code(CodeCommand::AutomationConsole).access('S'),
];

/// Window: the application's windows and the document in front, the
/// workspaces and the documents in turn, then how the panels are laid out.
const WINDOW_MENU: &[MenuEntry] = &[
    // A macOS Window menu begins with the window itself.
    MenuEntry::Only(
        MenuContext::MacApplication,
        &[
            command(Command::MinimizeWindow),
            command(Command::ZoomWindow),
            Separator,
        ],
    ),
    command(Command::NewApplicationWindow).access('N'),
    MenuEntry::submenu("Move document to", MOVE_DOCUMENT_MENU).access('T'),
    command(Command::ReattachDocument).access('R'),
    // Gathering windows back and finding ones left off screen are a desktop
    // application's: the browser and a tablet show one window.
    MenuEntry::Only(
        MenuContext::DesktopApplication,
        &[
            command(Command::ConsolidateWindows).access('M'),
            command(Command::MonitorRecovery).access('O'),
        ],
    ),
    Separator,
    MenuEntry::submenu("Workspace", WORKSPACE_MENU).access('W'),
    command(Command::PreviousDocument).access('P'),
    command(Command::NextDocument).access('D'),
    command(Command::PreviousPane).access('V'),
    command(Command::NextPane).access('X'),
    Separator,
    MenuEntry::submenu("Layout", LAYOUT_MENU).access('L'),
    command_as(Command::WindowManager, "Windows\u{2026}").access('S'),
];

/// Where the document in front goes: a window of its own, or one of the
/// windows already open, chosen in the dialog.
const MOVE_DOCUMENT_MENU: &[MenuEntry] = &[
    command_as(Command::DetachDocument, "New window\u{2026}").access('N'),
    command_as(Command::MoveDocumentToWindow, "Another window\u{2026}").access('A'),
];

/// The eight workspaces in the order of their keys, Alt+1 to Alt+8: the
/// Stimulus library, fourth on the rail, kept the key it was given last.
const WORKSPACE_MENU: &[MenuEntry] = &[
    command_as(Command::OpenWorkspace(Workspace::Project), "Project").access('P'),
    command_as(Command::OpenWorkspace(Workspace::Design), "Design").access('D'),
    command_as(Command::OpenWorkspace(Workspace::Simulate), "Simulate").access('S'),
    command_as(Command::OpenWorkspace(Workspace::Results), "Results").access('R'),
    command_as(Command::OpenWorkspace(Workspace::Verify), "Verify").access('V'),
    command_as(Command::OpenWorkspace(Workspace::Models), "Models").access('M'),
    command_as(
        Command::OpenWorkspace(Workspace::Netlist),
        "Netlist and script editor",
    )
    .access('N'),
    command_as(
        Command::OpenWorkspace(Workspace::Stimulus),
        "Stimulus library",
    )
    .access('L'),
];

/// The saved panel layouts, chosen in their dialog, and the way back to
/// the workspace's own.
const LAYOUT_MENU: &[MenuEntry] = &[
    MenuEntry::command_as(
        SettingsCommand::WorkspaceLayouts.key(),
        "Saved layouts\u{2026}",
    )
    .access('S'),
    command(Command::ResetLayout).access('R'),
];

/// Help: reading about RSpice and learning it from examples, getting help
/// with a problem, the licence and updates, then the notices and the version.
const HELP_MENU: &[MenuEntry] = &[
    command(Command::HelpCenter).access('H'),
    command(Command::KeyboardShortcuts).access('K'),
    command(Command::Examples).access('E'),
    command(Command::ReleaseNotes).access('N'),
    Separator,
    command(Command::SystemDiagnostics).access('R'),
    cloud(CloudCommand::License).access('L'),
    // On macOS the RSpice menu checks for updates.
    MenuEntry::Only(
        MenuContext::UpdatesInHelpMenu,
        &[cloud(CloudCommand::CheckForUpdates).access('C')],
    ),
    Separator,
    command(Command::LegalPrivacy).access('T'),
    // On macOS, About is the RSpice menu's.
    MenuEntry::Only(
        MenuContext::NoApplicationMenu,
        &[command(Command::About).access('A')],
    ),
];

/// The RSpice menu the brand opens at the bar's left on the macOS desktop:
/// the application itself and its updates, its settings, and quitting it.
pub(in crate::workbench) const APPLICATION_MENU: &[MenuEntry] = &[
    command(Command::About),
    cloud(CloudCommand::CheckForUpdates),
    Separator,
    MenuEntry::command_as(SettingsCommand::Preferences.key(), "Settings\u{2026}"),
    Separator,
    // macOS answers ⌘Q itself, from the menu every application has.
    command_as(Command::Exit, "Quit RSpice").shows_key("\u{2318}Q"),
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
    MenuEntry::submenu(menu.label(), application_menu_entries(menu)).access(menu.access_key())
}

/// The source editor's right-click menu: the language group, shown only
/// where the document's language service can answer it, then the
/// clipboard, then the line edits. Mutating rows are greyed on a read-only
/// source.
pub(in crate::workbench) const SOURCE_EDITOR_MENU: &[MenuEntry] = &[
    MenuEntry::Only(
        MenuContext::NavigableSourceEditor,
        &[
            code(CodeCommand::GoToDefinition).access('G'),
            code(CodeCommand::FindReferences).access('F'),
            code(CodeCommand::RenameSymbol).access('R'),
            Separator,
        ],
    ),
    command(Command::Cut).access('T'),
    command(Command::Copy).access('C'),
    command(Command::Paste).access('P'),
    Separator,
    code(CodeCommand::ToggleLineComment).access('M'),
    code(CodeCommand::FormatDocument).access('O'),
];

/// The icon a menu row shows: the one the command's toolbar button wears
/// (`chrome/toolbar.rs`), for a command that has a button. A command with
/// no button shows none, so an icon in a menu always marks the toolbar's
/// twin. The column stays for the rows that carry a check mark, which is why
/// the check buttons' own glyph is the one no row wears: Check cellview,
/// Check design, Check and netlist and Validate would read as switched on.
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
        Command::SymbolSave => WorkbenchIcon::Save,
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
        Command::CycleGrid => WorkbenchIcon::Grid,
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
    if DesignCommand::from_key(key) == Some(DesignCommand::VisibilityOptions) {
        return Some(WorkbenchIcon::Visibility);
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
            SimulateCommand::Options => Some(WorkbenchIcon::Sliders),
            _ => None,
        };
    }
    if let Some(command) = CodeCommand::from_key(key) {
        return match command {
            CodeCommand::ManageSourceDocument => Some(WorkbenchIcon::Folder),
            // The Models toolbar's button; the Verilog-A view's wears Run.
            CodeCommand::CompileVerilogA => Some(WorkbenchIcon::Netlist),
            CodeCommand::CancelVerilogACompile => Some(WorkbenchIcon::Stop),
            CodeCommand::VerilogABuildSettings => Some(WorkbenchIcon::Sliders),
            CodeCommand::FindCodeDocument => Some(WorkbenchIcon::Search),
            CodeCommand::CompareGeneratedRevisions => Some(WorkbenchIcon::Compare),
            CodeCommand::ExportNetlist(crate::io::NetlistFormat::Spice) => {
                Some(WorkbenchIcon::Export)
            }
            CodeCommand::AutomationConsole => Some(WorkbenchIcon::Terminal),
            _ => None,
        };
    }
    if let Some(command) = ModelsCommand::from_key(key) {
        return match command {
            ModelsCommand::ModelSaveRevision => Some(WorkbenchIcon::Save),
            ModelsCommand::RescanModelLibraries => Some(WorkbenchIcon::Refresh),
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
