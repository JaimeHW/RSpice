//! What each application menu holds, and the rows and rules it is drawn
//! with. The source editor's right-click menu shares the rows.

use super::*;

/// The View menu's viewport group, in the order it is drawn.
///
/// Named rather than written inline so the menu's rows are a value a test can
/// look at: `VisibilityOptions` was drawn once inside the loop and once again
/// straight after it, which no reader of a `for` body would see.
pub(super) const VIEW_VIEWPORT_COMMANDS: [Command; 11] = [
    Command::ZoomIn,
    Command::ZoomOut,
    Command::ZoomFit,
    Command::FitSchematicContent,
    Command::ZoomOneToOne,
    Command::ZoomToSelection,
    Command::ZoomBox,
    Command::CycleGrid,
    Command::GridSnapRouting,
    Command::DrawingSheetLayers,
    Command::VisibilityOptions,
];
pub(super) const DESIGN_PLACEMENT_COMMANDS: [Command; 10] = [
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
];
pub(super) const DESIGN_EDIT_COMMANDS: [Command; 4] = [
    Command::MoveSelection,
    Command::StretchSelection,
    Command::ArraySelection,
    Command::ReplaceInstance,
];
/// The sheets of the active drawing, in the order the strip shows them:
/// navigate first, then author.
pub(super) const DESIGN_SHEET_COMMANDS: [Command; 4] = [
    Command::PreviousSheet,
    Command::NextSheet,
    Command::NewSheet,
    Command::DeleteSheet,
];
pub(super) const DESIGN_MANAGEMENT_COMMANDS: [Command; 8] = [
    Command::CreateHierarchy,
    Command::DesignManagement,
    Command::SelectionBulkEdit,
    Command::ConnectivityManager,
    Command::ConfigurationSets,
    Command::RuleSetup,
    Command::ReviewComments,
    Command::RevisionHistory,
];
/// The design checks, narrowest first: the cellview in front of the reader,
/// the whole design, and the check that gates a save.
const DESIGN_CHECK_COMMANDS: [Command; 3] = [
    Command::CheckCellview,
    Command::CheckDesign,
    Command::CheckAndSave,
];
pub(super) const WINDOW_LAYOUT_COMMANDS: [Command; 5] = [
    Command::ToggleNavigator,
    Command::ToggleInspector,
    Command::ToggleConsole,
    Command::ToggleResultsSplit,
    Command::ToggleFocusMode,
];

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

    pub(super) fn show(self, ui: &mut Ui, app: &mut RSpiceApp) {
        match self {
            Self::File => file_menu(ui, app),
            Self::Edit => edit_menu(ui, app),
            Self::View => view_menu(ui, app),
            Self::Design => design_menu(ui, app),
            Self::Simulate => simulate_menu(ui, app),
            Self::Results => results_menu(ui, app),
            Self::Verify => verify_menu(ui, app),
            Self::Models => models_menu(ui, app),
            Self::Automation => automation_menu(ui, app),
            Self::Window => window_menu(ui, app),
            Self::Help => help_menu(ui, app),
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

fn command_item(ui: &mut Ui, app: &mut RSpiceApp, command: Command) {
    let spec = command.spec();
    command_item_as(ui, app, command, spec.label, None);
}

fn command_item_as(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    command: Command,
    label: &str,
    shortcut_override: Option<&str>,
) {
    command_item_as_impl(ui, app, command, label, shortcut_override, None);
}

fn scoped_palette_item(ui: &mut Ui, app: &mut RSpiceApp, label: &str, scope: &'static str) {
    command_item_as_impl(
        ui,
        app,
        Command::CommandPalette,
        label,
        Some("Ctrl+K"),
        Some(scope),
    );
}

fn command_item_as_impl(
    ui: &mut Ui,
    app: &mut RSpiceApp,
    command: Command,
    label: &str,
    shortcut_override: Option<&str>,
    palette_scope: Option<&'static str>,
) {
    let availability = command.availability(app);
    if availability == CommandAvailability::Hidden {
        return;
    }
    let enabled = availability.is_available();
    let shortcut = shortcut_for_occurrence(
        app.state.ui.preferences.shortcuts(),
        crate::workbench::app_state::runtime_command_platform(ui.ctx()),
        ui.ctx().os(),
        command,
        shortcut_override,
    );
    let row_height = ui.spacing().interact_size.y.max(MENU_ROW_HEIGHT);
    let (rect, response) = ui
        .add_enabled_ui(enabled, |ui| {
            ui.allocate_exact_size(Vec2::new(ui.available_width(), row_height), Sense::click())
        })
        .inner;
    let response = match availability {
        CommandAvailability::Disabled(reason) => {
            response.on_hover_text(format!("Unavailable: {reason}"))
        }
        CommandAvailability::Available | CommandAvailability::Hidden => response,
    };
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, label));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::MenuItem);
        node.set_label(label);
        if let CommandAvailability::Disabled(reason) = availability {
            node.set_description(reason);
        }
        if enabled && !shortcut.is_empty() {
            node.set_keyboard_shortcut(shortcut.as_str());
        }
    });
    record_menu_row(ui.ctx(), &response, label, enabled);

    let highlighted = enabled && (response.hovered() || response.has_focus());
    paint_menu_row(
        ui,
        rect,
        highlighted,
        enabled,
        command_icon(command),
        label,
        MenuRowTrailing::Shortcut(&shortcut),
    );
    theme::paint_focus_ring_outset(ui, &response, rect);

    let response = match availability {
        CommandAvailability::Disabled(reason) => response.on_disabled_hover_text(reason),
        CommandAvailability::Available | CommandAvailability::Hidden => response,
    };
    if response.clicked() {
        if let Some(scope) = palette_scope {
            app.state.dialogs.command_palette.open_in_scope(scope);
        } else {
            command.execute(app);
        }
        ui.close();
    }
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

fn menu_separator(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 9.0), Sense::hover());
    ui.painter().hline(
        (rect.left() + 5.0)..=(rect.right() - 5.0),
        rect.top() + 4.5,
        egui::Stroke::new(1.0, t.color.border),
    );
}

/// The source editor's right-click menu, drawn with the menu bar's own rows
/// and separators so its row height, text edge and rules match the menus
/// above it. A long press opens it on touch.
///
/// The language group shows only where the document's language service can
/// answer it; mutating rows are disabled on a read-only source.
pub(in crate::workbench) fn source_editor_context_menu(response: &Response, app: &mut RSpiceApp) {
    let ctx = response.ctx.clone();
    let large_targets = ctx.content_rect().width() <= 820.0
        || Tokens::get(&ctx).metrics.ctl_h >= MENU_TOUCH_ROW_HEIGHT;
    let popup_id = Popup::default_response_id(response);
    Popup::context_menu(response).show(|ui| {
        configure_menu_popup(ui, large_targets);
        ui.ctx().accesskit_node_builder(popup_id, |node| {
            node.set_role(egui::accesskit::Role::Menu);
            node.set_label("Source editor menu");
        });
        begin_menu_render(ui.ctx(), popup_id);
        let navigation = [
            Command::GoToDefinition,
            Command::FindReferences,
            Command::RenameSymbol,
        ];
        if navigation
            .iter()
            .any(|command| command.availability(app) != CommandAvailability::Hidden)
        {
            for command in navigation {
                command_item(ui, app, command);
            }
            menu_separator(ui);
        }
        for command in [Command::Cut, Command::Copy, Command::Paste] {
            command_item(ui, app, command);
        }
        menu_separator(ui);
        for command in [Command::ToggleLineComment, Command::FormatDocument] {
            command_item(ui, app, command);
        }
        let render_state = finish_menu_render(ui.ctx());
        handle_popup_keyboard(ui.ctx(), popup_id, response, &render_state.rows);
    });
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

fn file_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::ProjectLauncher);
    menu_separator(ui);
    command_item(ui, app, Command::OpenProject);
    command_item(ui, app, Command::OpenNetlist);
    command_item(ui, app, Command::NewProject);
    command_item(ui, app, Command::RecentProjects);
    menu_separator(ui);
    command_item(ui, app, Command::NewCell);
    command_item(ui, app, Command::OpenDocument);
    menu_separator(ui);
    command_item(ui, app, Command::Save);
    command_item(ui, app, Command::SaveAs);
    command_item(ui, app, Command::SaveAll);
    command_item(ui, app, Command::RevertActiveDocument);
    menu_separator(ui);
    command_item(ui, app, Command::ImportNetlist);
    command_item(ui, app, Command::ImportVerilogA);
    menu_separator(ui);
    command_item(ui, app, Command::ExportSchematicSvg);
    command_item(ui, app, Command::ExportWaveformsCsv);
    command_item(ui, app, Command::ExportPublicationSnapshot);
    command_item(
        ui,
        app,
        Command::ExportNetlist(crate::io::NetlistFormat::Spice),
    );
    menu_separator(ui);
    command_item(ui, app, Command::LiveSession);
    command_item(ui, app, Command::PublishToWeb);
    menu_separator(ui);
    command_item(ui, app, Command::CloseActiveDocument);
    command_item(ui, app, Command::CloseProject);
    menu_separator(ui);
    for command in DRAWING_SHEET_FILE_COMMANDS {
        command_item(ui, app, command);
    }
    command_item(ui, app, Command::PrintHardcopy);
    command_item(ui, app, Command::ExportActiveView);
    let platform = crate::workbench::app_state::runtime_command_platform(ui.ctx());
    if file_menu_shows_exit(platform) {
        menu_separator(ui);
        command_item(ui, app, Command::Exit);
    }
}

pub(super) const DRAWING_SHEET_FILE_COMMANDS: [Command; 4] = [
    Command::SheetProperties,
    Command::SheetFormatManager,
    Command::CustomSheetSizes,
    Command::SheetDefaults,
];

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

/// Edit acts on what the page shows: with a source editor as the document,
/// on its text; otherwise on the drawing's selection.
fn edit_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    let source = app.state.ui.active_source_editor.is_some();
    for command in [Command::Undo, Command::Redo] {
        command_item(ui, app, command);
    }
    menu_separator(ui);
    for command in [
        Command::Cut,
        Command::Copy,
        Command::Paste,
        if source {
            Command::DuplicateLine
        } else {
            Command::Duplicate
        },
        Command::Delete,
        Command::SelectAll,
    ] {
        command_item(ui, app, command);
    }
    menu_separator(ui);
    if source {
        for command in [
            Command::ToggleLineComment,
            Command::FormatDocument,
            Command::GoToLine,
        ] {
            command_item(ui, app, command);
        }
    } else {
        command_item(ui, app, Command::RenameSelection);
        command_item(ui, app, Command::ObjectProperties);
    }
    command_item(
        ui,
        app,
        if source {
            Command::FindCodeDocument
        } else {
            Command::FindInDesign
        },
    );
    if !source {
        command_item(ui, app, Command::FindReplace);
    }
    menu_separator(ui);
    command_item(ui, app, Command::Preferences);
}

/// Folding, offered while a source editor is the document on screen.
const VIEW_SOURCE_FOLD_COMMANDS: [Command; 4] = [
    Command::FoldSelection,
    Command::UnfoldAtCaret,
    Command::FoldAll,
    Command::UnfoldAll,
];

fn view_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    for command in VIEW_VIEWPORT_COMMANDS {
        command_item(ui, app, command);
    }
    if app.state.ui.active_source_editor.is_some() {
        menu_separator(ui);
        for command in VIEW_SOURCE_FOLD_COMMANDS {
            command_item(ui, app, command);
        }
    }
    menu_separator(ui);
    command_item_as(
        ui,
        app,
        Command::ToggleFullScreen,
        if app.state.workbench.full_screen_presentation {
            "Exit full screen"
        } else {
            "Enter full screen"
        },
        None,
    );
    command_item(ui, app, Command::ResetActiveView);
    menu_separator(ui);
    command_item(ui, app, Command::EngineeringTableView);
}

fn design_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item_as(
        ui,
        app,
        Command::OpenWorkspace(Workspace::Design),
        "Open active schematic",
        None,
    );
    command_item(ui, app, Command::AscendHierarchy);
    command_item(ui, app, Command::DescendHierarchy);
    menu_separator(ui);
    for command in DESIGN_PLACEMENT_COMMANDS {
        command_item(ui, app, command);
    }
    menu_separator(ui);
    for command in DESIGN_EDIT_COMMANDS {
        command_item(ui, app, command);
    }
    arrange_submenu(ui, |ui| {
        let width = arrange_submenu_width(ui, app);
        ui.set_max_width(width);
        ui.set_min_width(width);
        for (index, group) in ARRANGE_GROUPS.iter().enumerate() {
            if index > 0 {
                menu_separator(ui);
            }
            for &command in *group {
                command_item(ui, app, command);
            }
        }
    });
    command_item(ui, app, Command::JoinCollinearWires);
    menu_separator(ui);
    for command in DESIGN_SHEET_COMMANDS {
        command_item(ui, app, command);
    }
    menu_separator(ui);
    for command in DESIGN_MANAGEMENT_COMMANDS {
        command_item(ui, app, command);
    }
    menu_separator(ui);
    command_item_as(
        ui,
        app,
        Command::DesignSpecialistWorkspaces,
        "Specialist workspaces\u{2026}",
        None,
    );
    menu_separator(ui);
    for command in DESIGN_CHECK_COMMANDS {
        command_item(ui, app, command);
    }
    menu_separator(ui);
    scoped_palette_item(ui, app, "Browse all design commands\u{2026}", "Design");
}

pub(super) fn simulate_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    // The manager owns which plan is active, and every row below it acts on
    // that plan - "Run active plan", "Stop active run", its job history, its
    // preflight checks. Choosing the plan therefore reads above the actions
    // taken on it, in the same group they belong to.
    command_item(ui, app, Command::ManageSimulationPlans);
    // Filling the chosen plan reads directly under choosing it, and above the
    // actions that dispatch what it holds. It is here because the navigator is
    // the only other route to the catalogue, and the navigator is a panel a
    // reader can collapse.
    command_item(ui, app, Command::AddAnalysis);
    command_item(ui, app, Command::RunSimulation);
    command_item(ui, app, Command::StopSimulation);
    command_item(ui, app, Command::JobsManager);
    command_item(ui, app, Command::PreflightChecks);
    menu_separator(ui);
    command_item_as(
        ui,
        app,
        Command::OpenWorkspace(Workspace::Simulate),
        "Simulation Studio",
        None,
    );
    // Beside the studio, because a definition is what the studio's
    // excitations read: the mockup's Simulate menu lists the two together.
    command_item_as(
        ui,
        app,
        Command::OpenWorkspace(Workspace::Stimulus),
        "Stimulus Library",
        None,
    );
    command_item(ui, app, Command::SimulationOptions);
    command_item(ui, app, Command::GenerateNetlist);
    command_item(ui, app, Command::EditSpecifications);
}

pub(super) fn results_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(
        ui,
        app,
        Command::ResultViewer(crate::workbench::ResultViewer::Waves),
    );
    command_item(ui, app, Command::DatasetManifestBrowser);
    command_item(ui, app, Command::CreateResultDocument);
    command_item(ui, app, Command::CompareResultDatasets);
    command_item(ui, app, Command::ReportAuthoring);
    menu_separator(ui);
    command_item(ui, app, Command::ToggleLinkedCursors);
    command_item(ui, app, Command::WaveformCalculator);
    command_item(ui, app, Command::ExpressionDiagnostics);
    command_item(ui, app, Command::MeasurementLibrary);
    command_item(ui, app, Command::FamilySlicing);
    command_item(ui, app, Command::VisualizationDocumentProperties);
    menu_separator(ui);
    command_item(ui, app, Command::ImportResultDataset);
    command_item(ui, app, Command::ExportWaveformsCsv);
}

fn verify_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item_as(
        ui,
        app,
        Command::VerificationPage(VerificationPage::Yield),
        "Verification cockpit",
        None,
    );
    command_item_as(
        ui,
        app,
        Command::VerificationPage(VerificationPage::Corners),
        "Corner matrix",
        None,
    );
    command_item_as(
        ui,
        app,
        Command::VerificationPage(VerificationPage::Optimization),
        "Optimization",
        None,
    );
    command_item_as(
        ui,
        app,
        Command::VerificationPage(VerificationPage::DeviceSafety),
        "Safe operating area",
        None,
    );
    command_item_as(
        ui,
        app,
        Command::VerificationPage(VerificationPage::Regression),
        "Regression plan",
        None,
    );
}

fn models_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item_as(
        ui,
        app,
        Command::ModelsPage(ModelsPage::Models),
        "Model & library catalog",
        None,
    );
    command_item(ui, app, Command::ModelEditor);
    command_item(ui, app, Command::ModelCorrelation);
    menu_separator(ui);
    command_item_as(
        ui,
        app,
        Command::CompileVerilogA,
        "Verilog-A/AMS compiler",
        None,
    );
}

fn automation_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item_as(
        ui,
        app,
        Command::AutomationConsole,
        "Automation workspace",
        None,
    );
}

pub(super) fn overflow_menu(ui: &mut Ui, app: &mut RSpiceApp, projection: MenuProjection) {
    if projection == MenuProjection::ThroughSimulate {
        command_item_as(
            ui,
            app,
            Command::ResultViewer(crate::workbench::ResultViewer::Waves),
            "Results workspace",
            Some(""),
        );
        command_item_as(
            ui,
            app,
            Command::VerificationPage(VerificationPage::Yield),
            "Verification workspace",
            Some(""),
        );
        command_item_as(
            ui,
            app,
            Command::ModelsPage(ModelsPage::Models),
            "Models workspace",
            Some(""),
        );
    }
    command_item_as(
        ui,
        app,
        Command::AutomationConsole,
        "Automation workspace",
        Some(""),
    );
    command_item_as(
        ui,
        app,
        Command::SpecialistToolBrowser,
        "Specialist workspaces\u{2026}",
        Some(""),
    );
    menu_separator(ui);
    command_item(ui, app, Command::WorkspaceLayouts);
    command_item(ui, app, Command::WindowManager);
    command_item_as(ui, app, Command::JobsManager, "Jobs & targets", Some(""));
    command_item(ui, app, Command::ToggleFocusMode);
    command_item(ui, app, Command::ToggleConsole);
    menu_separator(ui);
    command_item(ui, app, Command::HelpCenter);
}

fn window_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    for command in WINDOW_LAYOUT_COMMANDS {
        command_item(ui, app, command);
    }
    menu_separator(ui);
    command_item(ui, app, Command::NewApplicationWindow);
    command_item(ui, app, Command::DetachDocument);
    command_item(ui, app, Command::MoveDocumentToWindow);
    command_item(ui, app, Command::ReattachDocument);
    command_item(ui, app, Command::ConsolidateWindows);
    command_item(ui, app, Command::MonitorRecovery);
    menu_separator(ui);
    command_item(ui, app, Command::PreviousDocument);
    command_item(ui, app, Command::NextDocument);
    command_item(ui, app, Command::CloseOtherDocuments);
    command_item(ui, app, Command::CloseAllDocuments);
    menu_separator(ui);
    command_item(ui, app, Command::WorkspaceLayouts);
    command_item(ui, app, Command::WindowManager);
    command_item(ui, app, Command::ResetLayout);
    menu_separator(ui);
    command_item(ui, app, Command::PreviousWorkspace);
    command_item(ui, app, Command::NextWorkspace);
}

fn help_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::HelpCenter);
    command_item(ui, app, Command::KeyboardShortcuts);
    command_item(ui, app, Command::ReleaseNotes);
    command_item(ui, app, Command::MigrationGuide);
    command_item(ui, app, Command::SystemDiagnostics);
    command_item(ui, app, Command::SupportBundle);
    command_item(ui, app, Command::LegalPrivacy);
    menu_separator(ui);
    command_item(ui, app, Command::About);
}
