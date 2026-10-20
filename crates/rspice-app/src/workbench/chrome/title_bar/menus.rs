//! What each application menu holds, written down as entries the menu
//! renderer draws and a test can read. The source editor's right-click menu
//! is written the same way.

use super::*;
use menu_entries::MenuEntry::Separator;
use menu_entries::{
    MenuContext, MenuEntry, MenuRow, command, command_as, command_without_key, palette_scope,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum ApplicationMenu {
    File,
    Edit,
    View,
    Design,
    Simulate,
    Results,
    Verify,
    Models,
    Automation,
    Window,
    Help,
}

impl ApplicationMenu {
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::View => "View",
            Self::Design => "Design",
            Self::Simulate => "Simulate",
            Self::Results => "Results",
            Self::Verify => "Verify",
            Self::Models => "Models",
            Self::Automation => "Automation",
            Self::Window => "Window",
            Self::Help => "Help",
        }
    }

    pub(super) const fn entries(self) -> &'static [MenuEntry] {
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

pub(super) const ALL_MENUS: [ApplicationMenu; 11] = [
    ApplicationMenu::File,
    ApplicationMenu::Edit,
    ApplicationMenu::View,
    ApplicationMenu::Design,
    ApplicationMenu::Simulate,
    ApplicationMenu::Results,
    ApplicationMenu::Verify,
    ApplicationMenu::Models,
    ApplicationMenu::Automation,
    ApplicationMenu::Window,
    ApplicationMenu::Help,
];

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
    Separator,
    command(Command::PreviousWorkspace),
    command(Command::NextWorkspace),
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

/// What the bar's overflow menu holds where it hides menus: the three
/// workspaces of the hidden menus while only File through Simulate show,
/// then the rows every overflow keeps.
pub(super) const fn overflow_entries(projection: MenuProjection) -> &'static [MenuEntry] {
    if matches!(projection, MenuProjection::ThroughSimulate) {
        OVERFLOW_THROUGH_SIMULATE_MENU
    } else {
        OVERFLOW_MENU
    }
}

const OVERFLOW_THROUGH_SIMULATE_MENU: &[MenuEntry] = &[
    command_without_key(
        Command::ResultViewer(crate::workbench::ResultViewer::Waves),
        "Results workspace",
    ),
    command_without_key(
        Command::VerificationPage(VerificationPage::Yield),
        "Verification workspace",
    ),
    command_without_key(Command::ModelsPage(ModelsPage::Models), "Models workspace"),
    command_without_key(Command::AutomationConsole, "Automation workspace"),
    command_without_key(
        Command::SpecialistToolBrowser,
        "Specialist workspaces\u{2026}",
    ),
    Separator,
    command(Command::WorkspaceLayouts),
    command(Command::WindowManager),
    command_without_key(Command::JobsManager, "Jobs & targets"),
    command_as(Command::ToggleFocusMode, "Focus mode"),
    command_as(Command::ToggleConsole, "Console"),
    Separator,
    command(Command::HelpCenter),
];

const OVERFLOW_MENU: &[MenuEntry] = &[
    command_without_key(Command::AutomationConsole, "Automation workspace"),
    command_without_key(
        Command::SpecialistToolBrowser,
        "Specialist workspaces\u{2026}",
    ),
    Separator,
    command(Command::WorkspaceLayouts),
    command(Command::WindowManager),
    command_without_key(Command::JobsManager, "Jobs & targets"),
    command_as(Command::ToggleFocusMode, "Focus mode"),
    command_as(Command::ToggleConsole, "Console"),
    Separator,
    command(Command::HelpCenter),
];

/// The source editor's right-click menu: the language group, shown only
/// where the document's language service can answer it, then the
/// clipboard, then the line edits. Mutating rows are greyed on a read-only
/// source.
const SOURCE_EDITOR_MENU: &[MenuEntry] = &[
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

/// Size an open menu for `rows` (as wide as its longest row needs, within
/// the menu's bounds) and give its rows their height.
pub(super) fn configure_menu_popup(ui: &mut Ui, large_targets: bool, rows: &[MenuRow]) {
    let width = menu_entries::menu_rows_width(ui, rows);
    ui.set_min_width(width);
    ui.set_max_width(width);
    ui.spacing_mut().item_spacing.y = 0.0;
    ui.spacing_mut().interact_size.y = if large_targets {
        MENU_TOUCH_ROW_HEIGHT
    } else {
        MENU_ROW_HEIGHT
    };
}

/// The icon a menu row shows: the one the command's toolbar button wears
/// (`chrome/toolbar.rs`), for a command that has a button. A command with
/// no button shows none, so an icon in a menu always marks the toolbar's
/// twin. The column stays for the rows that carry a check mark.
pub(super) fn command_icon(command: Command) -> Option<WorkbenchIcon> {
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
        Command::ProjectLauncher
        | Command::ImportResultDataset
        | Command::ManageSourceDocument => WorkbenchIcon::Folder,
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

/// The source editor's right-click menu, drawn with the menu bar's own rows
/// and separators so its row height, text edge and rules match the menus
/// above it. A long press opens it on touch.
pub(in crate::workbench) fn source_editor_context_menu(response: &Response, app: &mut RSpiceApp) {
    let ctx = response.ctx.clone();
    let large_targets = ctx.content_rect().width() <= 820.0
        || Tokens::get(&ctx).metrics.ctl_h >= MENU_TOUCH_ROW_HEIGHT;
    let popup_id = Popup::default_response_id(response);
    let shown = Popup::context_menu(response).show(|ui| {
        let rows = menu_rows(SOURCE_EDITOR_MENU, app, ui.ctx());
        configure_menu_popup(ui, large_targets, &rows);
        ui.ctx().accesskit_node_builder(popup_id, |node| {
            node.set_role(egui::accesskit::Role::Menu);
            node.set_label("Source editor menu");
        });
        begin_menu_render(ui.ctx(), popup_id);
        let choice = show_menu_rows(ui, &rows);
        let render_state = finish_menu_render(ui.ctx());
        handle_popup_keyboard(ui.ctx(), popup_id, response, &render_state.rows);
        choice
    });
    if let Some(choice) = shown.and_then(|shown| shown.inner) {
        choice.run(app);
    }
}

pub(super) fn shortcut_for_occurrence(
    shortcuts: &crate::workbench::ShortcutPreferences,
    platform: crate::workbench::commands::vocabulary::CommandPlatform,
    operating_system: egui::os::OperatingSystem,
    command: Command,
    shortcut_override: Option<&str>,
) -> String {
    shortcut_override.map_or_else(
        || shortcuts.resolved_label(command, platform, operating_system),
        str::to_owned,
    )
}

pub(super) fn menu_popup_max_height(ui: &Ui) -> f32 {
    menu_popup_height_for_viewport(ui.ctx().content_rect().height())
}

pub(super) fn menu_popup_height_for_viewport(viewport_height: f32) -> f32 {
    // Preserve room for the title bar and a small bottom escape margin while
    // guaranteeing enough space to navigate at least three standard rows.
    (viewport_height - 64.0).clamp(MENU_ROW_HEIGHT * 3.0, 560.0)
}

pub(super) const fn file_menu_shows_exit(
    platform: crate::workbench::commands::vocabulary::CommandPlatform,
) -> bool {
    matches!(
        platform,
        crate::workbench::commands::vocabulary::CommandPlatform::Desktop
    )
}
