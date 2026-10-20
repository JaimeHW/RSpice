//! What each application menu holds, written down as entries the menu
//! renderer draws and a test can read. The source editor's right-click menu
//! is written the same way.

use super::*;
use menu_entries::MenuEntry::Separator;
use menu_entries::{
    MenuContext, MenuEntry, command, command_as, command_without_key, palette_scope,
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
        icon: WorkbenchIcon::Grid,
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

pub(super) fn configure_menu_popup(ui: &mut Ui, large_targets: bool) {
    ui.set_min_width(MENU_CONTENT_WIDTH);
    ui.set_max_width(MENU_CONTENT_WIDTH);
    ui.spacing_mut().item_spacing.y = 0.0;
    ui.spacing_mut().interact_size.y = if large_targets {
        MENU_TOUCH_ROW_HEIGHT
    } else {
        MENU_ROW_HEIGHT
    };
}

pub(super) fn command_icon(command: Command) -> WorkbenchIcon {
    match command {
        Command::OpenWorkspace(Workspace::Project) | Command::ProjectLauncher => {
            WorkbenchIcon::Project
        }
        Command::OpenWorkspace(Workspace::Design) => WorkbenchIcon::Design,
        Command::OpenWorkspace(Workspace::Simulate) => WorkbenchIcon::Simulate,
        Command::OpenWorkspace(Workspace::Results)
        | Command::ResultViewer(_)
        | Command::WaveformCalculator
        | Command::CompareResultDatasets
        | Command::MeasurementLibrary
        | Command::FamilySlicing => WorkbenchIcon::Results,
        // The diagnostics tool exists to surface broken expressions, so it
        // wears the mark of the thing it reports rather than the workspace.
        Command::ExpressionDiagnostics => WorkbenchIcon::Warning,
        Command::OpenWorkspace(Workspace::Verify)
        | Command::VerificationPage(_)
        | Command::EditSpecifications
        | Command::PublishToWeb => WorkbenchIcon::Verify,
        Command::LiveSession => WorkbenchIcon::User,
        Command::OpenWorkspace(Workspace::Models)
        | Command::ModelsPage(_)
        | Command::ModelBrowser
        | Command::ModelEditor
        | Command::PdkSettings => WorkbenchIcon::Models,
        Command::OpenWorkspace(Workspace::Stimulus) => WorkbenchIcon::Source,
        Command::OpenWorkspace(Workspace::Netlist) => WorkbenchIcon::Netlist,
        Command::AutomationConsole => WorkbenchIcon::Terminal,
        Command::NewProject => WorkbenchIcon::File,
        Command::NewCell
        | Command::NewView
        | Command::NewVerilogADevice
        | Command::CreateResultDocument => WorkbenchIcon::Add,
        Command::OpenProject
        | Command::OpenDocument
        | Command::ImportResultDataset
        | Command::DatasetManifestBrowser => WorkbenchIcon::Folder,
        Command::OpenNetlist => WorkbenchIcon::Netlist,
        Command::RecentProjects => WorkbenchIcon::History,
        Command::Save | Command::SaveAs | Command::SaveAll | Command::ModelSaveRevision => {
            WorkbenchIcon::Save
        }
        Command::RevertActiveDocument
        | Command::ResetActiveView
        | Command::ResetLayout
        | Command::WorkspaceLayouts => WorkbenchIcon::Refresh,
        Command::ImportNetlist
        | Command::ImportVerilogA
        | Command::ExportSchematicSvg
        | Command::ExportWaveformsCsv
        | Command::ExportPublicationSnapshot
        | Command::ExportNetlist(_)
        | Command::PrintHardcopy
        | Command::ExportActiveView => WorkbenchIcon::Export,
        Command::SheetProperties => WorkbenchIcon::File,
        Command::SheetFormatManager => WorkbenchIcon::Grid,
        Command::CustomSheetSizes => WorkbenchIcon::Models,
        Command::SheetDefaults => WorkbenchIcon::File,
        Command::Undo => WorkbenchIcon::Undo,
        Command::Redo => WorkbenchIcon::Redo,
        Command::Cut | Command::Copy | Command::Paste | Command::Duplicate => WorkbenchIcon::Copy,
        Command::Delete | Command::ClearConsole => WorkbenchIcon::Trash,
        Command::SelectAll => WorkbenchIcon::Grid,
        Command::RenameSelection => WorkbenchIcon::Label,
        Command::ObjectProperties | Command::SimulationOptions => WorkbenchIcon::Sliders,
        Command::FindInDesign
        | Command::FindReplace
        | Command::FindCodeDocument
        | Command::CommandPalette => WorkbenchIcon::Search,
        Command::Preferences => WorkbenchIcon::Settings,
        Command::ZoomIn => WorkbenchIcon::ZoomIn,
        Command::ZoomOut => WorkbenchIcon::ZoomOut,
        Command::ZoomFit => WorkbenchIcon::ZoomFit,
        Command::FitSchematicContent => WorkbenchIcon::ZoomFit,
        Command::CycleGrid => WorkbenchIcon::Grid,
        Command::GridSnapRouting => WorkbenchIcon::Sliders,
        Command::DrawingSheetLayers => WorkbenchIcon::Layers,
        Command::VisibilityOptions => WorkbenchIcon::Layers,
        Command::ToggleNavigator => WorkbenchIcon::Navigator,
        Command::ToggleInspector => WorkbenchIcon::Inspector,
        Command::ToggleConsole => WorkbenchIcon::Console,
        Command::ToggleResultsSplit => WorkbenchIcon::Compare,
        Command::ToggleFocusMode | Command::ToggleFullScreen => WorkbenchIcon::Focus,
        Command::PlaceInstance => WorkbenchIcon::Component,
        Command::Place(_) => WorkbenchIcon::Add,
        Command::PlaceWire | Command::JoinCollinearWires => WorkbenchIcon::Wire,
        Command::PlaceBus => WorkbenchIcon::Bus,
        Command::PlaceBusTap => WorkbenchIcon::Bus,
        Command::PlaceJunction => WorkbenchIcon::Grid,
        Command::PlaceLabel => WorkbenchIcon::Label,
        Command::PlaceProbe => WorkbenchIcon::Probe,
        Command::PlacePin => WorkbenchIcon::Component,
        Command::PlaceText => WorkbenchIcon::Label,
        Command::PlaceShape => WorkbenchIcon::Grid,
        Command::MoveSelection | Command::StretchSelection => WorkbenchIcon::Select,
        Command::ArraySelection => WorkbenchIcon::Grid,
        Command::RotateSelection | Command::RotateSelectionCounterClockwise => {
            WorkbenchIcon::Rotate
        }
        Command::MirrorSelectionHorizontal => WorkbenchIcon::Mirror,
        Command::MirrorSelectionVertical => WorkbenchIcon::MirrorVertical,
        Command::AlignSelectionLeft
        | Command::AlignSelectionCenter
        | Command::AlignSelectionRight
        | Command::AlignSelectionTop
        | Command::AlignSelectionMiddle
        | Command::AlignSelectionBottom
        | Command::DistributeSelectionHorizontal
        | Command::DistributeSelectionVertical
        | Command::SnapSelectionToGrid => WorkbenchIcon::Grid,
        Command::ReplaceInstance => WorkbenchIcon::Refresh,
        Command::CreateHierarchy | Command::DesignManagement => WorkbenchIcon::Folder,
        Command::ConnectivityManager => WorkbenchIcon::Bus,
        Command::SelectionBulkEdit | Command::RuleSetup => WorkbenchIcon::Sliders,
        Command::ReviewComments => WorkbenchIcon::Info,
        Command::RevisionHistory => WorkbenchIcon::History,
        Command::AscendHierarchy => WorkbenchIcon::ArrowLeft,
        Command::DescendHierarchy | Command::DescendHierarchyDirect => WorkbenchIcon::Folder,
        Command::RunSimulation | Command::ModelRunQualificationTests => WorkbenchIcon::Run,
        Command::StopSimulation => WorkbenchIcon::Stop,
        Command::PreflightChecks
        | Command::CheckCellview
        | Command::CheckDesign
        | Command::CheckAndSave
        | Command::ModelValidate => WorkbenchIcon::Check,
        Command::CompileVerilogA => WorkbenchIcon::Code,
        Command::VerilogABuildSettings => WorkbenchIcon::Sliders,
        Command::AccountOrganization | Command::License => WorkbenchIcon::User,
        Command::KeyboardShortcuts => WorkbenchIcon::Search,
        Command::HelpCenter => WorkbenchIcon::Info,
        Command::ReleaseNotes => WorkbenchIcon::History,
        Command::MigrationGuide => WorkbenchIcon::Compare,
        Command::SystemDiagnostics | Command::SupportBundle => WorkbenchIcon::Terminal,
        Command::LegalPrivacy => WorkbenchIcon::Check,
        Command::ModelCompareRelease => WorkbenchIcon::Compare,
        Command::DesignSpecialistWorkspaces | Command::SpecialistToolBrowser => WorkbenchIcon::Grid,
        Command::AddVisualizationPane => WorkbenchIcon::Add,
        Command::ToggleLinkedCursors => WorkbenchIcon::Target,
        Command::VisualizationDocumentProperties => WorkbenchIcon::Settings,
        Command::About => WorkbenchIcon::Info,
        Command::PreviousDocument | Command::PreviousWorkspace => WorkbenchIcon::ArrowLeft,
        Command::NextDocument | Command::NextWorkspace => WorkbenchIcon::ArrowRight,
        Command::CloseOtherDocuments | Command::CloseAllDocuments => WorkbenchIcon::File,
        Command::NewApplicationWindow => WorkbenchIcon::Add,
        Command::DetachDocument => WorkbenchIcon::File,
        Command::MoveDocumentToWindow => WorkbenchIcon::ArrowRight,
        Command::ReattachDocument => WorkbenchIcon::ArrowLeft,
        Command::ConsolidateWindows => WorkbenchIcon::Grid,
        Command::MonitorRecovery => WorkbenchIcon::Refresh,
        Command::WindowManager => WorkbenchIcon::Folder,
        Command::Exit => WorkbenchIcon::Stop,
        Command::CloseActiveDocument => WorkbenchIcon::File,
        Command::CloseProject => WorkbenchIcon::Folder,
        Command::Cancel => WorkbenchIcon::Close,
        Command::ToggleLineComment | Command::GoToMatchingBracket => WorkbenchIcon::Code,
        Command::IndentLines => WorkbenchIcon::ArrowRight,
        Command::OutdentLines => WorkbenchIcon::ArrowLeft,
        Command::DuplicateLine => WorkbenchIcon::Copy,
        Command::DeleteLine => WorkbenchIcon::Trash,
        Command::MoveLineUp => WorkbenchIcon::ArrowUp,
        Command::MoveLineDown => WorkbenchIcon::ChevronDown,
        Command::GoToLine => WorkbenchIcon::Target,
        Command::FormatDocument => WorkbenchIcon::Format,
        Command::FoldSelection | Command::FoldAll => WorkbenchIcon::ChevronRight,
        Command::UnfoldAtCaret | Command::UnfoldAll => WorkbenchIcon::ChevronDown,
        Command::GoToDefinition => WorkbenchIcon::ArrowRight,
        Command::FindReferences => WorkbenchIcon::Search,
        Command::RenameSymbol => WorkbenchIcon::Label,
        _ => WorkbenchIcon::File,
    }
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
        configure_menu_popup(ui, large_targets);
        ui.ctx().accesskit_node_builder(popup_id, |node| {
            node.set_role(egui::accesskit::Role::Menu);
            node.set_label("Source editor menu");
        });
        begin_menu_render(ui.ctx(), popup_id);
        let rows = menu_rows(SOURCE_EDITOR_MENU, app, ui.ctx());
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
