//! What each application menu holds, written down as entries the menu
//! renderer draws and a test can read. The source editor's right-click menu
//! is the code feature's (`code::SOURCE_EDITOR_MENU`).

use crate::workbench::commands::vocabulary::ShellCommand;
use rspice_app_cloud::workbench::cloud::commands::CloudCommand;
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_design::schematic::view::sheet_visibility::SchematicObjectKind;
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_hardcopy::workbench::hardcopy::commands::HardcopyCommand;
use rspice_app_models::workbench::models::commands::ModelsCommand;
use rspice_app_project::workbench::project::commands::ProjectCommand;
use rspice_app_results::workbench::documents::result_document::ResultViewer;
use rspice_app_results::workbench::results::commands::ResultsCommand;
use rspice_app_settings::workbench::settings::commands::SettingsCommand;
use rspice_app_sheets::workbench::sheets::commands::SheetsCommand;
use rspice_app_simulate::workbench::simulate::commands::SimulateCommand;
use rspice_app_state::state::ModelsPage;
use rspice_app_state::state::SimulationPage;
use rspice_app_state::state::VerificationPage;
use rspice_app_state::state::command_catalog::CommandKey;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::chrome::title_bar::application_menus::ApplicationMenu;
use rspice_app_workbench::workbench::chrome::title_bar::menu_entries::MenuEntry::Separator;
use rspice_app_workbench::workbench::chrome::title_bar::menu_entries::{MenuContext, MenuEntry};
use rspice_app_workbench::workbench::framework::command_registry::WorkbenchCommand;
use rspice_ui_kit::panels::WorkbenchIcon;

/// The command's row under the vocabulary's label.
const fn command(command: ShellCommand) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// The command's row under a label of the menu's own.
const fn command_as(command: ShellCommand, label: &'static str) -> MenuEntry {
    MenuEntry::command_as(command.key(), label)
}

/// The routed command's row under the label the workbench declares.
const fn workbench(command: WorkbenchCommand) -> MenuEntry {
    MenuEntry::command(command.key())
}

/// The routed command's row under a label of the menu's own.
const fn workbench_as(command: WorkbenchCommand, label: &'static str) -> MenuEntry {
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

/// The design feature's command's row under a label of the menu's own.
const fn design_as(command: DesignCommand, label: &'static str) -> MenuEntry {
    MenuEntry::command_as(command.key(), label)
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
    command(ShellCommand::CloseActiveDocument).access('C'),
    command(ShellCommand::CloseOtherDocuments).access('H'),
    command(ShellCommand::CloseAllDocuments).access('L'),
    project(ProjectCommand::CloseProject).access('J'),
    Separator,
    project(ProjectCommand::Save).access('S'),
    project(ProjectCommand::SaveAs).access('A'),
    project(ProjectCommand::SaveAll),
    command(ShellCommand::RevertActiveDocument).access('D'),
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
        &[Separator, command(ShellCommand::Exit).access('X')],
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
        CodeCommand::ExportNetlist(rspice_app_services::io::NetlistFormat::Spice).key(),
        "SPICE netlist\u{2026}",
    )
    .access('S'),
    MenuEntry::command_as(
        CodeCommand::ExportNetlist(rspice_app_services::io::NetlistFormat::Spectre).key(),
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
    workbench(WorkbenchCommand::Undo).access('U'),
    workbench(WorkbenchCommand::Redo).access('R'),
    MenuEntry::Only(
        MenuContext::Drawing,
        &[command(ShellCommand::RepeatLastCommand).access('L')],
    ),
    Separator,
    workbench(WorkbenchCommand::Cut).access('T'),
    workbench(WorkbenchCommand::Copy).access('C'),
    // A picture of the selection or the view, for a document or a message.
    MenuEntry::Only(
        MenuContext::Drawing,
        &[hardcopy(HardcopyCommand::CopyAsImage).access('G')],
    ),
    workbench(WorkbenchCommand::Paste).access('P'),
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
            workbench(WorkbenchCommand::Duplicate).access('E'),
            workbench(WorkbenchCommand::Delete).access('D'),
        ],
    ),
    Separator,
    workbench(WorkbenchCommand::SelectAll).access('A'),
    MenuEntry::Only(
        MenuContext::Drawing,
        &[
            workbench(WorkbenchCommand::DeselectAll),
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
            design(DesignCommand::MoveSelection).access('V'),
            design(DesignCommand::StretchSelection).access('H'),
            design(DesignCommand::ArraySelection).access('Y'),
            design(DesignCommand::ReplaceInstance).access('S'),
            design(DesignCommand::SwapToRSpiceSymbols).access('W'),
            design(DesignCommand::SelectionBulkEdit).access('B'),
            Separator,
            design(DesignCommand::RenameSelection).access('M'),
            design(DesignCommand::ObjectProperties).access('O'),
            Separator,
            workbench(WorkbenchCommand::FindInDesign).access('F'),
            design(DesignCommand::FindReplace).access('I'),
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
    design_as(
        DesignCommand::SelectOfKind(SchematicObjectKind::Instances),
        "Instances",
    )
    .access('I'),
    design_as(
        DesignCommand::SelectOfKind(SchematicObjectKind::Wires),
        "Wires",
    )
    .access('W'),
    design_as(
        DesignCommand::SelectOfKind(SchematicObjectKind::NetLabels),
        "Net labels",
    )
    .access('N'),
    design_as(
        DesignCommand::SelectOfKind(SchematicObjectKind::Pins),
        "Pins",
    )
    .access('P'),
    design_as(
        DesignCommand::SelectOfKind(SchematicObjectKind::Text),
        "Text",
    )
    .access('T'),
    design_as(
        DesignCommand::SelectOfKind(SchematicObjectKind::Shapes),
        "Shapes",
    )
    .access('S'),
];

/// View: how near, what is drawn over the drawing, which panels stand
/// around it, and how much of the screen it takes.
const VIEW_MENU: &[MenuEntry] = &[
    workbench(WorkbenchCommand::ZoomIn).access('I'),
    workbench(WorkbenchCommand::ZoomOut).access('O'),
    workbench(WorkbenchCommand::ZoomOneToOne).access('Z'),
    // A symbol has no sheet: its Fit frames the symbol.
    MenuEntry::Only(
        MenuContext::NotSymbolView,
        &[
            workbench(WorkbenchCommand::ZoomFit).access('S'),
            design(DesignCommand::FitSchematicContent).access('F'),
        ],
    ),
    MenuEntry::Only(
        MenuContext::SymbolView,
        &[workbench_as(WorkbenchCommand::ZoomFit, "Fit").access('F')],
    ),
    workbench(WorkbenchCommand::ZoomToSelection).access('T'),
    workbench(WorkbenchCommand::ZoomBox).access('B'),
    workbench(WorkbenchCommand::ZoomPrevious).access('V'),
    Separator,
    MenuEntry::submenu("Grid", GRID_MENU).access('G'),
    design(DesignCommand::VisibilityOptions).access('D'),
    MenuEntry::Only(
        MenuContext::SymbolView,
        &[design(DesignCommand::SymbolPlacedPreview).access('A')],
    ),
    Separator,
    command_as(ShellCommand::ToggleNavigator, "Navigator").access('N'),
    command_as(ShellCommand::ToggleInspector, "Inspector").access('E'),
    command_as(ShellCommand::ToggleConsole, "Console").access('C'),
    results(ResultsCommand::ToggleResultsSplit).access('R'),
    command_as(ShellCommand::OpenProblems, "Problems").access('P'),
    Separator,
    // Folding, offered while a source editor is the document on screen.
    MenuEntry::Only(
        MenuContext::SourceEditor,
        &[MenuEntry::submenu("Folding", FOLDING_MENU).access('L')],
    ),
    command_as(ShellCommand::ToggleFocusMode, "Focus mode").access('M'),
    command_as(ShellCommand::ToggleFullScreen, "Full screen").access('U'),
];

/// The grid's styles, one of which is drawn; G steps through them.
const GRID_MENU: &[MenuEntry] = &[
    MenuEntry::radio(DesignCommand::GridDots.key(), "Dots").access('D'),
    MenuEntry::radio(DesignCommand::GridLines.key(), "Lines").access('L'),
    MenuEntry::radio(DesignCommand::GridOff.key(), "Off").access('O'),
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
    design(DesignCommand::DescendHierarchyDirect).access('D'),
    design(DesignCommand::DescendHierarchy).access('W'),
    design(DesignCommand::AscendHierarchy).access('U'),
    // Inside a symbol edited in place, the way back out stands where the
    // way in did, on the same letter.
    MenuEntry::Only(
        MenuContext::NotSymbolView,
        &[design(DesignCommand::EditSymbolInPlace).access('E')],
    ),
    MenuEntry::Only(
        MenuContext::SymbolView,
        &[design(DesignCommand::ExitSymbolInPlace).access('E')],
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
    // A symbol has no sheets and no standard values. No letter is left on
    // this menu that "Standard values" holds.
    MenuEntry::Only(
        MenuContext::NotSymbolView,
        &[
            MenuEntry::submenu("Standard values", STANDARD_VALUES_MENU),
            MenuEntry::submenu("Sheet", SHEET_MENU).access('S'),
        ],
    ),
    Separator,
    project(ProjectCommand::Variants).access('N'),
    project(ProjectCommand::Annotate).access('O'),
    design(DesignCommand::ConnectivityManager).access('Y'),
    design(DesignCommand::HighlightNet).access('T'),
    design(DesignCommand::ReviewComments).access('R'),
    design(DesignCommand::EngineeringTableView).access('J'),
    Separator,
    // The design checks, narrowest first: the cellview in front of the
    // reader, the whole design, and the check that gates a save.
    design(DesignCommand::CheckCellview).access('L'),
    design(DesignCommand::CheckDesign).access('K'),
    project(ProjectCommand::CheckAndSave).access('C'),
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
    MenuEntry::command_as(DesignCommand::PlaceInstance.key(), "Instance\u{2026}").access('I'),
    MenuEntry::command_as(DesignCommand::PlaceWire.key(), "Wire").access('W'),
    MenuEntry::command_as(DesignCommand::PlaceBus.key(), "Bus").access('B'),
    MenuEntry::command_as(DesignCommand::PlaceBusTap.key(), "Bus tap").access('T'),
    MenuEntry::command_as(DesignCommand::PlaceLabel.key(), "Net label").access('L'),
    MenuEntry::command_as(
        DesignCommand::PlaceOffSheetConnector.key(),
        "Off-sheet connector",
    )
    .access('O'),
    MenuEntry::command_as(DesignCommand::PlacePin.key(), "Pins\u{2026}").access('P'),
    MenuEntry::command_as(DesignCommand::PlaceJunction.key(), "Junction").access('J'),
    MenuEntry::command_as(DesignCommand::PlaceText.key(), "Text or note\u{2026}").access('N'),
    MenuEntry::command_as(
        DesignCommand::PlaceShape.key(),
        "Documentation shape\u{2026}",
    )
    .access('S'),
    Separator,
    MenuEntry::command_as(DesignCommand::PlaceResistor.key(), "Resistor").access('R'),
    MenuEntry::command_as(DesignCommand::PlaceCapacitor.key(), "Capacitor").access('A'),
    MenuEntry::command_as(DesignCommand::PlaceInductor.key(), "Inductor").access('U'),
    MenuEntry::command_as(DesignCommand::PlaceDiode.key(), "Diode").access('D'),
    MenuEntry::command_as(DesignCommand::PlaceVoltageSource.key(), "Voltage source").access('V'),
    MenuEntry::command_as(DesignCommand::PlaceCurrentSource.key(), "Current source").access('C'),
    MenuEntry::command_as(DesignCommand::PlaceGround.key(), "Ground").access('G'),
];

/// What a symbol view places: its drawing tools, then its pins from the
/// interface contract, and the contract's pins from it.
const SYMBOL_PLACE_MENU: &[MenuEntry] = &[
    design_as(DesignCommand::SymbolPinTool, "Pin").access('P'),
    design_as(DesignCommand::SymbolPolylineTool, "Line").access('L'),
    design_as(DesignCommand::SymbolRectangleTool, "Rectangle").access('R'),
    design_as(DesignCommand::SymbolCircleTool, "Circle").access('C'),
    design_as(DesignCommand::SymbolArcTool, "Arc").access('A'),
    design_as(DesignCommand::SymbolPolygonTool, "Polygon").access('G'),
    design_as(DesignCommand::SymbolTextTool, "Text").access('T'),
    Separator,
    design(DesignCommand::SymbolUpdatePinsFromContract).access('U'),
    design(DesignCommand::SymbolCreateSchematicPins).access('S'),
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
    design(DesignCommand::NextViolation).access('N'),
    design(DesignCommand::PreviousViolation).access('P'),
    Separator,
    design(DesignCommand::ClearChecks).access('C'),
];

/// The Design menu's Arrange submenu, group by group: turn and flip, line
/// up, space out, put back on the grid.
const ARRANGE_MENU: &[MenuEntry] = &[
    design(DesignCommand::RotateSelection).access('R'),
    design(DesignCommand::RotateSelectionCounterClockwise).access('C'),
    design(DesignCommand::MirrorSelectionHorizontal).access('V'),
    design(DesignCommand::MirrorSelectionVertical).access('M'),
    Separator,
    design(DesignCommand::AlignSelectionLeft).access('L'),
    design(DesignCommand::AlignSelectionCenter).access('H'),
    design(DesignCommand::AlignSelectionRight).access('A'),
    design(DesignCommand::AlignSelectionTop).access('T'),
    design(DesignCommand::AlignSelectionMiddle).access('G'),
    design(DesignCommand::AlignSelectionBottom).access('B'),
    Separator,
    design(DesignCommand::DistributeSelectionHorizontal).access('D'),
    design(DesignCommand::DistributeSelectionVertical).access('E'),
    Separator,
    design(DesignCommand::SnapSelectionToGrid).access('S'),
    design(DesignCommand::JoinCollinearWires).access('J'),
];

/// Snap the selected values to the project's preferred-number series, and
/// choose the series; the value fields' arrow keys step along the same one.
const STANDARD_VALUES_MENU: &[MenuEntry] = &[
    design(DesignCommand::SnapToStandardValue).access('S'),
    Separator,
    MenuEntry::radio(
        DesignCommand::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E6).key(),
        "E6",
    )
    .access('6'),
    MenuEntry::radio(
        DesignCommand::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E12).key(),
        "E12",
    )
    .access('1'),
    MenuEntry::radio(
        DesignCommand::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E24).key(),
        "E24",
    )
    .access('4'),
    MenuEntry::radio(
        DesignCommand::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E48).key(),
        "E48",
    )
    .access('8'),
    MenuEntry::radio(
        DesignCommand::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E96).key(),
        "E96",
    )
    .access('9'),
    MenuEntry::radio(
        DesignCommand::SetStandardValueSeries(rspice_vocabulary::quantity::ESeries::E192).key(),
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
    MenuEntry::command_as(DesignCommand::PlaceProbe.key(), "Probe").access('P'),
    design(DesignCommand::AnnotateOperatingPoint).access('D'),
    design(DesignCommand::AnnotateTransientVoltages).access('T'),
    MenuEntry::command_as(
        ResultsCommand::ResultViewer(
            rspice_app_results::workbench::documents::result_document::ResultViewer::Op,
        )
        .key(),
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
    command(ShellCommand::ConfigurationSets).access('C'),
    command_as(
        ShellCommand::OpenWorkspace(Workspace::Stimulus),
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
        ShellCommand::OpenWorkspace(Workspace::Netlist),
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
            command(ShellCommand::MinimizeWindow),
            command(ShellCommand::ZoomWindow),
            Separator,
        ],
    ),
    command(ShellCommand::NewApplicationWindow).access('N'),
    MenuEntry::submenu("Move document to", MOVE_DOCUMENT_MENU).access('T'),
    command(ShellCommand::ReattachDocument).access('R'),
    // Gathering windows back and finding ones left off screen are a desktop
    // application's: the browser and a tablet show one window.
    MenuEntry::Only(
        MenuContext::DesktopApplication,
        &[
            command(ShellCommand::ConsolidateWindows).access('M'),
            command(ShellCommand::MonitorRecovery).access('O'),
        ],
    ),
    Separator,
    MenuEntry::submenu("Workspace", WORKSPACE_MENU).access('W'),
    command(ShellCommand::PreviousDocument).access('P'),
    command(ShellCommand::NextDocument).access('D'),
    command(ShellCommand::PreviousPane).access('V'),
    command(ShellCommand::NextPane).access('X'),
    Separator,
    MenuEntry::submenu("Layout", LAYOUT_MENU).access('L'),
    command_as(ShellCommand::WindowManager, "Windows\u{2026}").access('S'),
];

/// Where the document in front goes: a window of its own, or one of the
/// windows already open, chosen in the dialog.
const MOVE_DOCUMENT_MENU: &[MenuEntry] = &[
    command_as(ShellCommand::DetachDocument, "New window\u{2026}").access('N'),
    command_as(ShellCommand::MoveDocumentToWindow, "Another window\u{2026}").access('A'),
];

/// The eight workspaces in the order of their keys, Alt+1 to Alt+8: the
/// Stimulus library, fourth on the rail, kept the key it was given last.
const WORKSPACE_MENU: &[MenuEntry] = &[
    command_as(ShellCommand::OpenWorkspace(Workspace::Project), "Project").access('P'),
    command_as(ShellCommand::OpenWorkspace(Workspace::Design), "Design").access('D'),
    command_as(ShellCommand::OpenWorkspace(Workspace::Simulate), "Simulate").access('S'),
    command_as(ShellCommand::OpenWorkspace(Workspace::Results), "Results").access('R'),
    command_as(ShellCommand::OpenWorkspace(Workspace::Verify), "Verify").access('V'),
    command_as(ShellCommand::OpenWorkspace(Workspace::Models), "Models").access('M'),
    command_as(
        ShellCommand::OpenWorkspace(Workspace::Netlist),
        "Netlist and script editor",
    )
    .access('N'),
    command_as(
        ShellCommand::OpenWorkspace(Workspace::Stimulus),
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
    command(ShellCommand::ResetLayout).access('R'),
];

/// Help: reading about RSpice and learning it from examples, getting help
/// with a problem, the licence and updates, then the notices and the version.
const HELP_MENU: &[MenuEntry] = &[
    command(ShellCommand::HelpCenter).access('H'),
    command(ShellCommand::KeyboardShortcuts).access('K'),
    command(ShellCommand::Examples).access('E'),
    command(ShellCommand::ReleaseNotes).access('N'),
    Separator,
    command(ShellCommand::SystemDiagnostics).access('R'),
    cloud(CloudCommand::License).access('L'),
    // On macOS the RSpice menu checks for updates.
    MenuEntry::Only(
        MenuContext::UpdatesInHelpMenu,
        &[cloud(CloudCommand::CheckForUpdates).access('C')],
    ),
    Separator,
    command(ShellCommand::LegalPrivacy).access('T'),
    // On macOS, About is the RSpice menu's.
    MenuEntry::Only(
        MenuContext::NoApplicationMenu,
        &[command(ShellCommand::About).access('A')],
    ),
];

/// The RSpice menu the brand opens at the bar's left on the macOS desktop:
/// the application itself and its updates, its settings, and quitting it.
pub(in crate::workbench) const APPLICATION_MENU: &[MenuEntry] = &[
    command(ShellCommand::About),
    cloud(CloudCommand::CheckForUpdates),
    Separator,
    MenuEntry::command_as(SettingsCommand::Preferences.key(), "Settings\u{2026}"),
    Separator,
    // macOS answers ⌘Q itself, from the menu every application has.
    command_as(ShellCommand::Exit, "Quit RSpice").shows_key("\u{2318}Q"),
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

/// The icon a menu row shows: the one the command's toolbar button wears
/// (`chrome/toolbar.rs`), for a command that has a button. A command with
/// no button shows none, so an icon in a menu always marks the toolbar's
/// twin. The column stays for the rows that carry a check mark, which is why
/// the check buttons' own glyph is the one no row wears: Check cellview,
/// Check design, Check and netlist and Validate would read as switched on.
pub(in crate::workbench) fn command_icon(command: WorkbenchCommand) -> Option<WorkbenchIcon> {
    let icon = match command {
        WorkbenchCommand::Duplicate => WorkbenchIcon::Copy,
        WorkbenchCommand::Delete => WorkbenchIcon::Trash,
        WorkbenchCommand::Undo => WorkbenchIcon::Undo,
        WorkbenchCommand::Redo => WorkbenchIcon::Redo,
        WorkbenchCommand::ZoomIn => WorkbenchIcon::ZoomIn,
        WorkbenchCommand::ZoomOut => WorkbenchIcon::ZoomOut,
        WorkbenchCommand::ZoomFit => WorkbenchIcon::ZoomFit,
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
    if let Some(command) = DesignCommand::from_key(key) {
        return match command {
            DesignCommand::VisibilityOptions => Some(WorkbenchIcon::Visibility),
            DesignCommand::SelectTool => Some(WorkbenchIcon::Select),
            DesignCommand::CycleGrid => Some(WorkbenchIcon::Grid),
            DesignCommand::PlaceInstance => Some(WorkbenchIcon::Instance),
            DesignCommand::PlaceWire => Some(WorkbenchIcon::Wire),
            DesignCommand::PlaceBus => Some(WorkbenchIcon::Bus),
            DesignCommand::PlaceBusTap => Some(WorkbenchIcon::BusTap),
            DesignCommand::PlaceJunction => Some(WorkbenchIcon::Junction),
            DesignCommand::PlaceLabel => Some(WorkbenchIcon::Label),
            DesignCommand::PlaceOffSheetConnector => Some(WorkbenchIcon::ArrowRight),
            DesignCommand::PlaceProbe => Some(WorkbenchIcon::Probe),
            DesignCommand::PlaceText => Some(WorkbenchIcon::Text),
            DesignCommand::PlacePin => Some(WorkbenchIcon::Pin),
            DesignCommand::DescendHierarchyDirect => Some(WorkbenchIcon::Layers),
            DesignCommand::AscendHierarchy => Some(WorkbenchIcon::ArrowUp),
            DesignCommand::RotateSelection => Some(WorkbenchIcon::Rotate),
            DesignCommand::MirrorSelectionHorizontal => Some(WorkbenchIcon::Mirror),
            DesignCommand::MirrorSelectionVertical => Some(WorkbenchIcon::MirrorVertical),
            DesignCommand::ExitSymbolInPlace => Some(WorkbenchIcon::ArrowUp),
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
            CodeCommand::ExportNetlist(rspice_app_services::io::NetlistFormat::Spice) => {
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
