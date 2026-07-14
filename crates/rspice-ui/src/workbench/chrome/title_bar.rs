//! Application title bar and complete implemented menu taxonomy.

use egui::{Align, Context, Frame, Layout, Sense, TopBottomPanel, Ui, Vec2};

use crate::common::RSpiceApp;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::commands::Command;
use super::super::design_system::WorkbenchIcon;
use super::super::layout::LayoutSpec;
use super::super::state::{ModelsPage, VerificationPage, Workspace};

const DESCEND_MENU_LABEL: &str = "Descend into selected instance…";
const COMMAND_REFERENCE_MENU_LABEL: &str = "Command reference";

pub fn show(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    let viewport_width = ctx.content_rect().width();
    let menu_projection = MenuProjection::for_layout(viewport_width, layout.compact_shell);
    TopBottomPanel::top("workbench.title_bar")
        .exact_height(layout.title_bar_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(false)
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            ui.painter().hline(
                rect.x_range(),
                rect.bottom(),
                egui::Stroke::new(1.0, t.color.border),
            );
            ui.horizontal_centered(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0;
                brand(
                    ui,
                    app,
                    menu_projection.shows_title_context(),
                    layout.title_bar_height,
                );
                menus(ui, app, menu_projection);
                let context_left = ui.available_rect_before_wrap().left();
                let mut context_right = ui.max_rect().right();

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(6.0);
                    if viewport_width > 560.0
                        && icon_action(ui, WorkbenchIcon::Settings, "Open preferences")
                    {
                        Command::Preferences.execute(app);
                    }
                    if search_button(ui, viewport_width, app) {
                        Command::CommandPalette.execute(app);
                    }
                    context_right = ui.available_rect_before_wrap().right();
                });
                paint_title_context(
                    ui,
                    app,
                    egui::Rect::from_x_y_ranges(
                        context_left..=context_right,
                        ui.max_rect().y_range(),
                    ),
                    !menu_projection.shows_title_context(),
                );
            });
        });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ApplicationMenu {
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
    const fn label(self) -> &'static str {
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

    fn show(self, ui: &mut Ui, app: &mut RSpiceApp) {
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

const ALL_MENUS: [ApplicationMenu; 11] = [
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

const THROUGH_MODELS_MENUS: [ApplicationMenu; 8] = [
    ApplicationMenu::File,
    ApplicationMenu::Edit,
    ApplicationMenu::View,
    ApplicationMenu::Design,
    ApplicationMenu::Simulate,
    ApplicationMenu::Results,
    ApplicationMenu::Verify,
    ApplicationMenu::Models,
];

const THROUGH_SIMULATE_MENUS: [ApplicationMenu; 5] = [
    ApplicationMenu::File,
    ApplicationMenu::Edit,
    ApplicationMenu::View,
    ApplicationMenu::Design,
    ApplicationMenu::Simulate,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MenuProjection {
    Hidden,
    ThroughSimulate,
    ThroughModels,
    All,
}

impl MenuProjection {
    const fn for_layout(viewport_width: f32, compact_shell: bool) -> Self {
        if compact_shell {
            Self::Hidden
        } else {
            Self::for_width(viewport_width)
        }
    }

    const fn for_width(viewport_width: f32) -> Self {
        if viewport_width <= 820.0 {
            Self::Hidden
        } else if viewport_width <= 1020.0 {
            Self::ThroughSimulate
        } else if viewport_width <= 1360.0 {
            Self::ThroughModels
        } else {
            Self::All
        }
    }

    const fn visible_menus(self) -> &'static [ApplicationMenu] {
        match self {
            Self::Hidden => &[],
            Self::ThroughSimulate => &THROUGH_SIMULATE_MENUS,
            Self::ThroughModels => &THROUGH_MODELS_MENUS,
            Self::All => &ALL_MENUS,
        }
    }

    const fn has_overflow(self) -> bool {
        matches!(self, Self::ThroughSimulate | Self::ThroughModels)
    }

    const fn overflow_trigger_label(self) -> &'static str {
        match self {
            // The mockup adds a text label only in its 1021-1360 px range.
            Self::ThroughModels => "⋯  More",
            Self::ThroughSimulate => "⋯",
            Self::Hidden | Self::All => "",
        }
    }

    const fn shows_title_context(self) -> bool {
        !matches!(self, Self::Hidden)
    }
}

fn brand(ui: &mut Ui, app: &mut RSpiceApp, show_wordmark: bool, height: f32) {
    let t = Tokens::get(ui.ctx());
    let width = if show_wordmark { 86.0 } else { 38.0 };
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Open Project launcher",
        )
    });
    WorkbenchIcon::Brand.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.left() + 18.0, rect.center().y),
            Vec2::splat(22.0),
        ),
        t.color.accent,
    );
    if show_wordmark {
        ui.painter().text(
            egui::Pos2::new(rect.left() + 34.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            "RSpice",
            theme::sans(tokens::FS_2, FontWeight::SemiBold),
            t.color.text,
        );
    }
    if response.clicked() {
        Command::ProjectLauncher.execute(app);
    }
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text("Open Project launcher");
}

fn menus(ui: &mut Ui, app: &mut RSpiceApp, projection: MenuProjection) {
    for menu in projection.visible_menus() {
        top_menu(ui, menu.label(), |ui| menu.show(ui, app));
    }
    if projection.has_overflow() {
        overflow_menu_button(ui, app, projection);
    }
}

fn top_menu(ui: &mut Ui, label: &str, contents: impl FnOnce(&mut Ui)) {
    ui.menu_button(
        egui::RichText::new(label).font(theme::sans(tokens::FS_1, FontWeight::Regular)),
        |ui| {
            ui.set_min_width(286.0);
            contents(ui);
        },
    );
}

fn overflow_menu_button(ui: &mut Ui, app: &mut RSpiceApp, projection: MenuProjection) {
    let response = ui.menu_button(
        egui::RichText::new(projection.overflow_trigger_label())
            .font(theme::sans(tokens::FS_1, FontWeight::Regular)),
        |ui| {
            ui.set_min_width(286.0);
            overflow_menu(ui, app);
        },
    );
    response.response.on_hover_text("More application menus");
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
    let enabled = command.is_enabled(app);
    let shortcut = shortcut_for_occurrence(command, shortcut_override);
    let text = if shortcut.is_empty() {
        label.to_owned()
    } else {
        format!("{label}\t{shortcut}")
    };
    if ui
        .add_enabled(enabled, egui::Button::new(text).frame(false))
        .clicked()
    {
        command.execute(app);
        ui.close();
    }
}

fn shortcut_for_occurrence<'a>(command: Command, shortcut_override: Option<&'a str>) -> &'a str {
    shortcut_override.unwrap_or(command.spec().shortcut)
}

fn file_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::ProjectLauncher);
    ui.separator();
    command_item(ui, app, Command::OpenProject);
    command_item(ui, app, Command::NewProject);
    command_item(ui, app, Command::RecentProjects);
    ui.separator();
    command_item(ui, app, Command::Save);
    command_item(ui, app, Command::SaveAs);
    ui.separator();
    command_item(ui, app, Command::Exit);
}

fn edit_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    for command in [Command::Undo, Command::Redo] {
        command_item(ui, app, command);
    }
    ui.separator();
    for command in [
        Command::Cut,
        Command::Copy,
        Command::Paste,
        Command::Duplicate,
        Command::Delete,
        Command::SelectAll,
    ] {
        command_item(ui, app, command);
    }
    ui.separator();
    command_item(ui, app, Command::ObjectProperties);
    command_item(ui, app, Command::FindInDesign);
    ui.separator();
    command_item(ui, app, Command::Preferences);
}

fn view_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    for command in [
        Command::ZoomIn,
        Command::ZoomOut,
        Command::ZoomFit,
        Command::CycleGrid,
    ] {
        command_item(ui, app, command);
    }
    ui.separator();
    command_item_as(
        ui,
        app,
        Command::ToggleFullScreen,
        if app.state.workbench.full_screen {
            "Exit full screen"
        } else {
            "Enter full screen"
        },
        None,
    );
    command_item(ui, app, Command::ResetActiveView);
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
    command_item_as(ui, app, Command::DescendHierarchy, DESCEND_MENU_LABEL, None);
    ui.separator();
    for command in [
        Command::PlaceInstance,
        Command::PlaceWire,
        Command::PlaceLabel,
        Command::PlaceProbe,
    ] {
        command_item(ui, app, command);
    }
    ui.separator();
    command_item(ui, app, Command::CheckAndSave);
}

fn simulate_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::RunSimulation);
    command_item(ui, app, Command::StopSimulation);
    command_item(ui, app, Command::PreflightChecks);
    ui.separator();
    command_item_as(
        ui,
        app,
        Command::OpenWorkspace(Workspace::Simulate),
        "Simulation Studio",
        None,
    );
    command_item(ui, app, Command::SimulationOptions);
}

fn results_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item_as(
        ui,
        app,
        Command::OpenWorkspace(Workspace::Results),
        "Open results workspace",
        None,
    );
    ui.separator();
    command_item_as(ui, app, Command::WaveformCalculator, "Calculator…", None);
}

fn verify_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item_as(
        ui,
        app,
        Command::OpenWorkspace(Workspace::Verify),
        "Verification cockpit",
        None,
    );
    ui.separator();
    command_item_as(
        ui,
        app,
        Command::VerificationPage(VerificationPage::Specifications),
        "Specification matrix",
        None,
    );
}

fn models_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item_as(
        ui,
        app,
        Command::ModelsPage(ModelsPage::Catalog),
        "Model & library catalog",
        None,
    );
    ui.separator();
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

fn overflow_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    // The mockup's overflow taxonomy contains several future product
    // workflows. Project only the exact equivalent that has a real executor.
    command_item_as(
        ui,
        app,
        Command::AutomationConsole,
        "Automation workspace",
        Some(""),
    );
}

fn window_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    command_item(ui, app, Command::ToggleNavigator);
    command_item(ui, app, Command::ToggleInspector);
    command_item(ui, app, Command::ToggleConsole);
    command_item(ui, app, Command::ToggleFocusMode);
    ui.separator();
    command_item(ui, app, Command::ResetLayout);
}

fn help_menu(ui: &mut Ui, app: &mut RSpiceApp) {
    // F1 belongs to the mockup's RSpice Help command. Until that distinct
    // command exists, the command-reference occurrence must not claim it.
    command_item_as(
        ui,
        app,
        Command::KeyboardShortcuts,
        COMMAND_REFERENCE_MENU_LABEL,
        Some(""),
    );
    ui.separator();
    command_item(ui, app, Command::About);
}

fn paint_title_context(ui: &mut Ui, app: &RSpiceApp, bounds: egui::Rect, compact: bool) {
    if bounds.width() < 40.0 {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let dirty = app.state.schematic.is_dirty || app.state.workspace.any_dirty();
    let cell = active_title_cell(app);
    let full = if compact {
        cell
    } else {
        format!(
            "{}  /  {}",
            app.state.workspace.project.display_name(),
            cell
        )
    };
    let available_text_width = (bounds.width() - 26.0).max(12.0);
    let maximum_characters = (available_text_width / 6.3).floor().max(1.0) as usize;
    let text = ellipsize(&full, maximum_characters);
    let font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let painter = ui
        .painter()
        .with_clip_rect(bounds.shrink2(egui::vec2(5.0, 0.0)));
    let galley = painter.layout_no_wrap(text, font, t.color.text);
    let total_width = 13.0 + galley.size().x;
    let left = (bounds.center().x - total_width * 0.5).max(bounds.left() + 5.0);
    painter.circle_filled(
        egui::pos2(left + 3.0, bounds.center().y),
        3.0,
        if dirty { t.color.warn } else { t.color.ok },
    );
    painter.galley(
        egui::pos2(left + 13.0, bounds.center().y - galley.size().y * 0.5),
        galley,
        t.color.text,
    );
}

fn active_title_cell(app: &RSpiceApp) -> String {
    match app.state.workbench.workspace {
        Workspace::Project => "Project overview".to_owned(),
        Workspace::Design => format!(
            "{} · {}",
            app.state.workspace.active_view.cell, app.state.workspace.active_view.view
        ),
        Workspace::Simulate => "Simulation plan".to_owned(),
        Workspace::Results => "Results".to_owned(),
        Workspace::Verify => "Verification".to_owned(),
        Workspace::Models => "Model & Library Manager".to_owned(),
        Workspace::Netlist => "top.sp · generated".to_owned(),
    }
}

fn ellipsize(value: &str, maximum_characters: usize) -> String {
    let count = value.chars().count();
    if count <= maximum_characters {
        return value.to_owned();
    }
    if maximum_characters <= 1 {
        return "…".to_owned();
    }
    let mut shortened = value
        .chars()
        .take(maximum_characters - 1)
        .collect::<String>();
    shortened.push('…');
    shortened
}

fn search_button(ui: &mut Ui, viewport_width: f32, app: &RSpiceApp) -> bool {
    let t = Tokens::get(ui.ctx());
    let width = search_button_width(viewport_width);
    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, 26.0), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            "Search and run a command",
        )
    });
    ui.painter().rect_filled(rect, t.radius, t.color.bg_inset);
    ui.painter().rect_stroke(
        rect,
        t.radius,
        egui::Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    WorkbenchIcon::Search.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::Pos2::new(rect.left() + 15.0, rect.center().y),
            Vec2::splat(14.0),
        ),
        t.color.text_dim,
    );
    if width > 60.0 {
        ui.painter().text(
            egui::Pos2::new(rect.left() + 29.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            "Search or run a command",
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        ui.painter().text(
            egui::Pos2::new(rect.right() - 7.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            "Ctrl K",
            theme::mono(tokens::FS_0, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    let _ = app;
    theme::paint_focus_ring(ui, &response, rect);
    response
        .on_hover_text("Search and run a command (Ctrl+K)")
        .clicked()
}

fn search_button_width(viewport_width: f32) -> f32 {
    if viewport_width <= 1020.0 {
        31.0
    } else {
        (viewport_width * 0.18).min(230.0)
    }
}

fn icon_action(ui: &mut Ui, icon: WorkbenchIcon, label: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(28.0), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, t.radius, t.color.bg_hover);
    }
    icon.paint(ui.painter(), rect.shrink(6.0), t.color.text_dim);
    theme::paint_focus_ring(ui, &response, rect);
    response.on_hover_text(label).clicked()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels(projection: MenuProjection) -> Vec<&'static str> {
        projection
            .visible_menus()
            .iter()
            .map(|menu| menu.label())
            .collect()
    }

    #[test]
    fn menu_projection_matches_mockup_breakpoints() {
        assert_eq!(MenuProjection::for_width(820.0), MenuProjection::Hidden);
        assert_eq!(
            MenuProjection::for_width(821.0),
            MenuProjection::ThroughSimulate
        );
        assert_eq!(
            MenuProjection::for_width(1020.0),
            MenuProjection::ThroughSimulate
        );
        assert_eq!(
            MenuProjection::for_width(1021.0),
            MenuProjection::ThroughModels
        );
        assert_eq!(
            MenuProjection::for_width(1360.0),
            MenuProjection::ThroughModels
        );
        assert_eq!(MenuProjection::for_width(1361.0), MenuProjection::All);
        assert_eq!(
            MenuProjection::for_layout(844.0, true),
            MenuProjection::Hidden
        );
    }

    #[test]
    fn each_projection_exposes_only_the_mockup_menu_prefix() {
        assert!(labels(MenuProjection::Hidden).is_empty());
        assert_eq!(
            labels(MenuProjection::ThroughSimulate),
            ["File", "Edit", "View", "Design", "Simulate"]
        );
        assert_eq!(
            labels(MenuProjection::ThroughModels),
            [
                "File", "Edit", "View", "Design", "Simulate", "Results", "Verify", "Models"
            ]
        );
        assert_eq!(
            labels(MenuProjection::All),
            [
                "File",
                "Edit",
                "View",
                "Design",
                "Simulate",
                "Results",
                "Verify",
                "Models",
                "Automation",
                "Window",
                "Help"
            ]
        );
    }

    #[test]
    fn overflow_and_compact_title_labels_match_the_mockup() {
        assert_eq!(
            MenuProjection::ThroughModels.overflow_trigger_label(),
            "⋯  More"
        );
        assert_eq!(
            MenuProjection::ThroughSimulate.overflow_trigger_label(),
            "⋯"
        );
        assert!(!MenuProjection::Hidden.has_overflow());
        assert!(!MenuProjection::Hidden.shows_title_context());
        assert_eq!(search_button_width(1020.0), 31.0);
        assert!(search_button_width(1021.0) > 31.0);
    }

    #[test]
    fn occurrence_specific_shortcuts_can_be_suppressed_or_overridden() {
        assert_eq!(
            shortcut_for_occurrence(Command::KeyboardShortcuts, Some("")),
            ""
        );
        assert_eq!(
            shortcut_for_occurrence(Command::KeyboardShortcuts, Some("Ctrl+Alt+R")),
            "Ctrl+Alt+R"
        );
        assert_eq!(DESCEND_MENU_LABEL, "Descend into selected instance…");
        assert_eq!(COMMAND_REFERENCE_MENU_LABEL, "Command reference");
    }

    #[test]
    fn core_menu_commands_are_all_real_dispatch_commands() {
        let commands = [
            Command::OpenProject,
            Command::Save,
            Command::Undo,
            Command::PlaceWire,
            Command::RunSimulation,
            Command::WaveformCalculator,
            Command::PdkSettings,
            Command::AutomationConsole,
        ];
        assert!(commands.iter().all(|command| !command.spec().id.is_empty()));
    }
}
