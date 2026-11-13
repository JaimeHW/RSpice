//! Production Project workspace.
//!
//! The persistent section chrome hosts five focused projections of the live
//! project. Every page reads authoritative project state and dispatches real
//! workbench workflows; no mockup fixture data is admitted here.

pub(crate) mod configuration;
mod dependencies;
pub(crate) mod landing;
pub(crate) mod library;
pub(crate) mod library_publication;
pub(crate) mod overview;
mod page;
pub(crate) mod recovery;
mod technology;

use egui::{
    Align, Align2, Color32, Context, Key, Layout, Margin, Modifiers, Rect, Response, ScrollArea,
    Sense, Stroke, Ui, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::command_catalog::CommandKey;
use crate::state::{ProjectTechnologyBinding, model_library::ModelLibraryManager};
use crate::ui::accessibility::plural_suffix;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogSize, Problem, paint_problem_mark, problem_line, select,
};
use crate::workbench::RSpiceApp;
use crate::workbench::app::dialogs::project::configuration_dialogs::show_configuration_page_in;
use crate::workbench::app_state::DesignCheckStatus;
use crate::workbench::features;
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;

use super::super::commands::vocabulary::Command;
use super::super::design_system::{WorkbenchIcon, elide_text, property_card, property_row};
use super::super::framework::command_registry::ids;
use super::super::state::Workspace;
use crate::workbench::project::workspace_state::ProjectPage;
use recovery::recovery;
use technology::open_technology_attachment_dialog;
pub(in crate::workbench) use technology::show_technology_attachment_dialog;
#[cfg(target_arch = "wasm32")]
use technology::{poll_browser_recovery_completions, poll_browser_technology_checkpoint};

const PROJECT_TAB_HEIGHT: f32 = 24.0;
/// The unsaved mark a tab or row carries, the document bar's dot, and the
/// room it takes after the label.
const DIRTY_MARK_SIZE: f32 = 6.0;
const DIRTY_MARK_ADVANCE: f32 = 12.0;
const WORKSPACE_TABLE_HEADER_HEIGHT: f32 = 37.0;

/// Height of one ledger row.
///
/// The density token is the same one the navigator and inspector lists read,
/// so a ledger row, a tree row and a property row line up at every density
/// instead of the ledger holding a desktop-only 36 px that reads loose beside
/// them and still misses a touch target.
fn workspace_table_row_height(ui: &Ui) -> f32 {
    Tokens::get(ui.ctx()).metrics.row_h
}

/// Height of a ledger's column header, drawn tighter than its rows.
fn workspace_table_column_header_height(ui: &Ui) -> f32 {
    (workspace_table_row_height(ui) - 4.0).max(22.0)
}

/// Width that is both allocated to this UI and actually visible through the
/// current clip. Split-pane children can otherwise inherit the unconstrained
/// central-panel width and paint beneath the Inspector dock.
fn visible_workspace_width(ui: &Ui) -> f32 {
    ui.available_rect_before_wrap()
        .intersect(ui.clip_rect())
        .width()
        .max(1.0)
}
/// Height that is both allocated to this UI and actually visible through the
/// current clip.
///
/// A page rendered inside the surface's vertical `ScrollArea` is offered an
/// unbounded `available_height`, so a panel that wants to reach the bottom of
/// the workspace has to ask the clip how far that is.
fn visible_workspace_height(ui: &Ui) -> f32 {
    ui.available_rect_before_wrap()
        .intersect(ui.clip_rect())
        .height()
        .max(1.0)
}

#[cfg(test)]
const TECHNOLOGY_SURFACE_TITLE: &str = "Model-library technology contract";
const TECHNOLOGY_SURFACE_ACTION: &str = "Attach model technology\u{2026}";
const TECHNOLOGY_DIALOG_TITLE: &str = "Attach model-library technology";
const TECHNOLOGY_DIALOG_PRIMARY: &str = "Attach binding";
const TECHNOLOGY_DIALOG_CURRENT: &str = "Binding already attached";
const TECHNOLOGY_DIALOG_PENDING: &str = "Writing checkpoint\u{2026}";

#[cfg(target_arch = "wasm32")]
struct BrowserTechnologyCheckpointCompletion {
    project_id: String,
    expected_revision: u64,
    binding: ProjectTechnologyBinding,
    authority: crate::state::ProjectTechnologyChangeAuthority,
    migration_evidence: Option<crate::state::pdk_config::PdkTechnologyMigrationEvidence>,
    result:
        Result<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary, String>,
}

#[cfg(target_arch = "wasm32")]
struct BrowserRecoveryCatalogCompletion {
    project_id: String,
    result:
        Result<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointCatalog, String>,
}

#[cfg(target_arch = "wasm32")]
struct BrowserRecoveryCopyCompletion {
    project_id: String,
    filename: String,
    result: Result<Vec<u8>, String>,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_TECHNOLOGY_CHECKPOINT_COMPLETIONS: std::cell::RefCell<std::collections::VecDeque<BrowserTechnologyCheckpointCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
    static BROWSER_RECOVERY_CATALOG_COMPLETIONS: std::cell::RefCell<std::collections::VecDeque<BrowserRecoveryCatalogCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
    static BROWSER_RECOVERY_COPY_COMPLETIONS: std::cell::RefCell<std::collections::VecDeque<BrowserRecoveryCopyCompletion>> =
        const { std::cell::RefCell::new(std::collections::VecDeque::new()) };
}

/// Take what the browser finished writing or reading for this workspace
/// before it draws.
#[cfg(target_arch = "wasm32")]
pub(in crate::workbench) fn poll_browser_completions(
    ctx: &Context,
    cx: &mut Cx<'_, ProjectFeature>,
) {
    poll_browser_technology_checkpoint(ctx, cx);
    poll_browser_recovery_completions(ctx, cx);
}

/// The workspace with no project behind it, which it is whenever a project
/// is closed, the launcher is dismissed with nothing open, a recovery review
/// declines to reopen, or a live session is waiting for the host's snapshot.
/// Every section of the open workspace projects an open project's state, so
/// showing them here would dress the bootstrap placeholder up as the reader's
/// own work.
pub(in crate::workbench) fn show_landing(ui: &mut Ui, app: &mut RSpiceApp) {
    landing::show(ui, app);
}

/// The open project's page in front, as the panel the chrome's tabs control.
pub(in crate::workbench) fn page(ui: &mut Ui, app: &mut RSpiceApp) {
    let page = app.state.project.project_page;
    let panel = ui.scope(|ui| match page {
        // The Overview is the page in front, so its context holds the
        // dispatcher's answers for the commands its rows offer.
        ProjectPage::Overview => {
            overview::overview(ui, &mut features::cx_with_foreign_answers(app, |_| true))
        }
        // The Library page carries out what the reader asked of it between
        // its own calls, which the shell still does over the application.
        ProjectPage::Library => library::library(ui, app),
        ProjectPage::Recovery => recovery(ui, &mut features::cx(app)),
        ProjectPage::Configuration => configuration::configuration(ui, &mut features::cx(app)),
        page => {
            ScrollArea::vertical()
                .id_salt(("workbench.project.surface", page))
                .auto_shrink([false, false])
                .show(ui, |ui| match page {
                    ProjectPage::Dependencies => {
                        dependencies::dependencies(ui, &mut features::cx(app))
                    }
                    ProjectPage::Overview
                    | ProjectPage::Library
                    | ProjectPage::Configuration
                    | ProjectPage::Recovery => {
                        unreachable!("page owns its scroll contract")
                    }
                });
        }
    });
    ui.ctx().accesskit_node_builder(panel.response.id, |node| {
        node.set_role(egui::accesskit::Role::TabPanel);
        node.set_label(format!("Project {}", page.label()));
    });
}

/// Reuse the exact project-owned Library/Cell/View browser from a specialist
/// route without introducing a second selection or mutation owner.
pub(super) fn show_library_browser(ui: &mut Ui, app: &mut RSpiceApp) {
    library::library(ui, app);
}

/// What the reader asked of the chrome, raised once it has drawn: a command,
/// or what only the shell carries out.
enum ChromeRaise {
    Execute(CommandKey),
    Shell(ProjectRequest),
}

/// The open project's chrome: the section tabs, the revision, and the health
/// chips and actions of the page in front, in the project feature's context.
/// The tabs and the configuration chip change the page where they are drawn;
/// what else the reader asked of it is raised once it has drawn and answered
/// before the page below it draws, in the same frame.
pub(in crate::workbench) fn chrome(cx: &mut Cx<'_, ProjectFeature>, ui: &mut Ui) {
    let tokens = Tokens::get(ui.ctx());
    let mut raised = None;
    let shown = egui::Frame::new()
        .fill(tokens.color.bg_panel)
        .inner_margin(Margin {
            left: 8,
            right: 8,
            top: 0,
            bottom: 0,
        })
        .show(ui, |ui| {
            let inline = ui.available_width()
                >= project_tabs_desired_width(ui) + project_context_desired_width(cx) + 10.0;
            if inline {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    project_tabs(ui, cx);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        project_context_cluster(ui, cx, &mut raised);
                    });
                });
            } else {
                ScrollArea::horizontal()
                    .id_salt("workbench.project.tabs.compact")
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        project_tabs(ui, cx);
                    });
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        project_context_cluster(ui, cx, &mut raised);
                    });
                });
            }
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
    match raised {
        Some(ChromeRaise::Execute(key)) => cx.execute(key),
        Some(ChromeRaise::Shell(request)) => cx.wb.outbox.push(request),
        None => {}
    }
}

fn project_context_cluster(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    raised: &mut Option<ChromeRaise>,
) {
    let tokens = Tokens::get(ui.ctx());
    let desired = project_context_desired_width(cx);
    let width = ui.available_width().min(desired).max(1.0);
    ui.allocate_ui_with_layout(
        vec2(width, PROJECT_TAB_HEIGHT),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            project_revision_label(ui, cx, &tokens);
            if cx.state.project_page != ProjectPage::Overview {
                project_health_chips(ui, cx, raised);
                project_page_actions(ui, cx, raised);
            }
        },
    );
}

fn project_context_desired_width(cx: &Cx<'_, ProjectFeature>) -> f32 {
    if cx.state.project_page == ProjectPage::Overview {
        160.0
    } else {
        610.0
    }
}

fn project_tabs_desired_width(ui: &Ui) -> f32 {
    let tokens = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Medium);
    let labels = ProjectPage::ALL
        .iter()
        .map(|page| {
            ui.painter()
                .layout_no_wrap(page.label().to_owned(), font.clone(), tokens.color.text)
                .size()
                .x
                + 20.0
        })
        .sum::<f32>();
    labels + 2.0 * (ProjectPage::ALL.len().saturating_sub(1) as f32)
}

fn project_tabs(ui: &mut Ui, cx: &mut Cx<'_, ProjectFeature>) {
    let tokens = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Medium);
    let tab_ids = ProjectPage::ALL.map(|page| ui.id().with(("project-section-tab", page)));
    // Edits to a configuration survive leaving its page, so its tab carries
    // the unsaved mark a document tab does until they are applied.
    let configuration_dirty = cx
        .state
        .configuration
        .is_dirty(&cx.model.workspace.content.configuration_sets);
    let shown = ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        for (index, page) in ProjectPage::ALL.into_iter().enumerate() {
            let text_width = ui
                .painter()
                .layout_no_wrap(page.label().to_owned(), font.clone(), tokens.color.text)
                .size()
                .x;
            let dirty = page == ProjectPage::Configuration && configuration_dirty;
            let mark_width = if dirty { DIRTY_MARK_ADVANCE } else { 0.0 };
            let (_, rect) =
                ui.allocate_space(vec2(text_width + 20.0 + mark_width, PROJECT_TAB_HEIGHT));
            let response = ui.interact(rect, tab_ids[index], Sense::click());
            let selected = cx.state.project_page == page;
            response.widget_info(|| {
                WidgetInfo::selected(
                    WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    selected,
                    page.label(),
                )
            });
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_role(egui::accesskit::Role::Tab);
                node.set_label(page.label());
                node.set_selected(selected);
                if dirty {
                    node.set_description("Unsaved changes.");
                }
            });
            if response.hovered() {
                ui.painter().rect_filled(rect, 0.0, tokens.color.bg_hover);
            }
            ui.painter().text(
                rect.center() - vec2(mark_width * 0.5, 0.0),
                Align2::CENTER_CENTER,
                page.label(),
                font.clone(),
                if selected {
                    tokens.color.text
                } else {
                    tokens.color.text_dim
                },
            );
            if dirty {
                // A live state, not a warning: the secondary text tone.
                ui.painter().circle_filled(
                    pos2(
                        rect.center().x + text_width * 0.5 + DIRTY_MARK_ADVANCE * 0.5,
                        rect.center().y,
                    ),
                    DIRTY_MARK_SIZE * 0.5,
                    tokens.color.text_dim,
                );
            }
            if selected {
                ui.painter().rect_filled(
                    Rect::from_min_max(
                        pos2(rect.left(), rect.bottom() - 2.0),
                        pos2(rect.right(), rect.bottom()),
                    ),
                    0.0,
                    tokens.color.accent,
                );
            }
            theme::paint_focus_ring(ui, &response, rect);
            if response.clicked() {
                cx.state.project_page = page;
            }
            if response.has_focus() {
                let target = ui.input_mut(|input| {
                    if input.consume_key(Modifiers::NONE, Key::ArrowRight) {
                        Some((index + 1) % ProjectPage::ALL.len())
                    } else if input.consume_key(Modifiers::NONE, Key::ArrowLeft) {
                        Some((index + ProjectPage::ALL.len() - 1) % ProjectPage::ALL.len())
                    } else if input.consume_key(Modifiers::NONE, Key::Home) {
                        Some(0)
                    } else if input.consume_key(Modifiers::NONE, Key::End) {
                        Some(ProjectPage::ALL.len() - 1)
                    } else {
                        None
                    }
                });
                if let Some(target) = target {
                    cx.state.project_page = ProjectPage::ALL[target];
                    ui.memory_mut(|memory| memory.request_focus(tab_ids[target]));
                }
            }
            response.on_hover_text(format!("Open project {}", page.label()));
        }
    });
    ui.ctx().accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::TabList);
        node.set_label("Project sections");
    });
}

fn project_revision_label(ui: &mut Ui, cx: &Cx<'_, ProjectFeature>, tokens: &Tokens) {
    ui.label(
        egui::RichText::new(format!(
            "main @ r{}",
            cx.model.workspace.content.project.revision().get()
        ))
        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
        .color(tokens.color.text_faint),
    )
    .on_hover_text("Current mutable project input revision");
}

fn project_health_chips(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    raised: &mut Option<ChromeRaise>,
) {
    if cx.model.is_netlist_first_without_schematic() {
        let (label, description, problem) = cx
            .model.workspace
            .content.netlist_document
            .as_ref()
            .map_or_else(
                || {
                    (
                        "source unavailable".to_owned(),
                        "The project has no SPICE source document".to_owned(),
                        Some(Problem::Failure),
                    )
                },
                |document| match document.validation() {
                    None => (
                        "source not validated".to_owned(),
                        "Open and validate the retained SPICE source deck".to_owned(),
                        None,
                    ),
                    Some(report) if report.content_digest() != document.content_digest() => (
                        "source stale".to_owned(),
                        "The source changed after the retained validation".to_owned(),
                        None,
                    ),
                    Some(report) => {
                        let errors = report.error_count();
                        let warnings = report.warning_count();
                        (
                            format!("source {errors} err · {warnings} adv"),
                            format!(
                                "Retained SPICE source validation: {errors} errors and {warnings} advisories"
                            ),
                            (errors > 0).then_some(Problem::Failure),
                        )
                    }
                },
            );
        if project_status_chip(ui, "project-source", &label, &description, problem, true).clicked()
        {
            *raised = Some(ChromeRaise::Execute(
                Command::OpenWorkspace(Workspace::Netlist).key(),
            ));
        }
    } else {
        let root = cx.model.workspace.content.simulation_root_reference();
        let (checks_label, checks_description, checks_problem) = match cx
            .model
            .simulation_root_design_check_status(
                cx.runs.slot_facts(cx.peers.design().review_markers()),
            ) {
            DesignCheckStatus::Current(receipt) => {
                let summary = receipt.result.summary();
                let errors = summary.critical + summary.errors;
                let advisories = summary.warnings + summary.info;
                (
                    format!("checks {errors} err · {advisories} adv"),
                    format!(
                        "Project-root schematic checks: {errors} blocking errors and {advisories} advisories"
                    ),
                    (errors > 0).then_some(Problem::Failure),
                )
            }
            DesignCheckStatus::NotRun => (
                "checks not run".to_owned(),
                "Run schematic checks for the project root".to_owned(),
                None,
            ),
            DesignCheckStatus::Stale(_) => (
                "checks stale".to_owned(),
                "Project-root inputs changed after the retained check".to_owned(),
                None,
            ),
            DesignCheckStatus::Unavailable { reason, .. } => (
                "checks unavailable".to_owned(),
                reason,
                Some(Problem::Failure),
            ),
        };
        if project_status_chip(
            ui,
            "project-checks",
            &checks_label,
            &checks_description,
            checks_problem,
            true,
        )
        .clicked()
        {
            *raised = Some(ChromeRaise::Shell(ProjectRequest::OpenRootAndCheck(root)));
        }
    }

    let active_configuration = cx.model.workspace.content.configuration_sets.active();
    let configuration =
        active_configuration.map_or("Default", |configuration| configuration.name());
    // A configuration's name is a name, not a state, and Default is what a
    // project that uses no configuration runs with rather than a fault.
    if project_status_chip(
        ui,
        "project-configuration",
        &format!("config {configuration}"),
        &format!("Configuration used for runs: {configuration}"),
        None,
        true,
    )
    .clicked()
    {
        // The Configurations command, carried out where the chip is drawn so
        // the band's action after it reads the page it opens. Of the steps
        // every command passes through, only these two guards act on it.
        let command = Command::ConfigurationSets;
        if !command.held_by_project_operation(cx.platform)
            && !command.lacks_open_project(cx.platform)
        {
            show_configuration_page_in(cx.platform, cx.wb, &mut cx.state.project_page);
        }
    }

    if let Some((index, run)) = cx.model.simulation.retained.runs.iter().enumerate().next() {
        let has_dataset = !run.analyses.is_empty();
        // A run that completed with data is the ordinary run, so the chip
        // names only the two cases that are not.
        let run_label = if run.success && has_dataset {
            format!("run {}", run.id)
        } else if has_dataset {
            format!("run {} · review", run.id)
        } else {
            format!("run {} · no data", run.id)
        };
        let description = format!(
            "Latest retained simulation run {}: {}",
            run.id,
            if has_dataset {
                format!("{} retained analyses", run.analyses.len())
            } else {
                "no retained result dataset".to_owned()
            }
        );
        if project_status_chip(
            ui,
            "project-latest-run",
            &run_label,
            &description,
            None,
            true,
        )
        .clicked()
        {
            // A run with no retained dataset sends the reader to wherever the
            // project can actually be run again, which for a netlist-first
            // project is the deck, not Simulate.
            let destination = overview::run_destination(
                has_dataset,
                cx.model.is_netlist_first_without_schematic(),
            );
            if cx.model.simulation.select_run(index) {
                *raised = Some(ChromeRaise::Execute(
                    Command::OpenWorkspace(destination).key(),
                ));
            }
        }
    } else {
        project_status_chip(
            ui,
            "project-latest-run",
            "run none",
            "No retained simulation run",
            None,
            false,
        );
    }
}

/// Side of a problem mark painted inside a chip, and the advance from the
/// mark's left edge to the text after it: read from the kit, so a marked chip
/// and an unmarked one beside it keep the same text rhythm.
use crate::ui::widgets::{
    PROBLEM_MARK_ADVANCE as CHIP_MARK_ADVANCE, PROBLEM_MARK_SIDE as CHIP_MARK_SIDE,
};

/// One of the title row's health chips: a link to the page that owns the
/// fact, so its words stay neutral. A failure the reader has to act on puts
/// the kit's cross in front of them; nothing else is coloured.
fn project_status_chip(
    ui: &mut Ui,
    id_salt: &'static str,
    label: &str,
    accessible_label: &str,
    problem: Option<Problem>,
    enabled: bool,
) -> Response {
    let tokens = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let text_width = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font.clone(), tokens.color.text_dim)
        .size()
        .x;
    let mark_advance = if problem.is_some() {
        CHIP_MARK_ADVANCE
    } else {
        0.0
    };
    let (_, rect) = ui.allocate_space(vec2(text_width + mark_advance + 14.0, 22.0));
    let response = ui.interact(
        rect,
        ui.id().with(("project-status-chip", id_salt)),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    if enabled && (response.hovered() || response.has_focus()) {
        ui.painter().rect_filled(rect, 3.0, tokens.color.bg_hover);
    }
    let text_left = rect.left() + 7.0 + mark_advance;
    if let Some(problem) = problem {
        paint_problem_mark(
            ui.painter(),
            Rect::from_center_size(
                pos2(
                    rect.left() + 7.0 + CHIP_MARK_SIDE * 0.5,
                    rect.center().y + 0.5,
                ),
                vec2(CHIP_MARK_SIDE, CHIP_MARK_SIDE),
            ),
            problem,
        );
    }
    ui.painter().text(
        pos2(text_left, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        font,
        if enabled {
            tokens.color.text_dim
        } else {
            tokens.color.text_faint
        },
    );
    let announcement = problem.map_or_else(
        || accessible_label.to_owned(),
        |problem| problem.announce(accessible_label),
    );
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, enabled, announcement.clone()));
    if enabled {
        theme::paint_focus_ring(ui, &response, rect);
    }
    response.on_hover_text(accessible_label)
}

fn project_page_actions(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    raised: &mut Option<ChromeRaise>,
) {
    match cx.state.project_page {
        ProjectPage::Overview => {}
        ProjectPage::Library => {
            if Button::new("Attach library\u{2026}").show(ui).clicked() {
                *raised = Some(ChromeRaise::Execute(ids::PDK_SETTINGS));
            }
        }
        // The Configuration page's own band carries its actions.
        ProjectPage::Configuration | ProjectPage::Dependencies | ProjectPage::Recovery => {}
    }
}

/// A ledger panel's own header: its name on the left and one fact about the
/// list on the right. The fact is never painted as a verdict; a panel the
/// reader has to look at carries the kit's mark in front of it instead.
fn workspace_table_panel_header(ui: &mut Ui, title: &str, meta: &str, problem: Option<Problem>) {
    let tokens = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, _) =
        ui.allocate_exact_size(vec2(width, WORKSPACE_TABLE_HEADER_HEIGHT), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, tokens.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.top(),
        Stroke::new(1.0, tokens.color.border_strong),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
    let content = rect.shrink2(vec2(11.0, 0.0));
    let gap = 8.0;
    let title_width = ((content.width() - gap) * 0.62).max(0.0);
    let title_rect = Rect::from_min_max(
        content.min,
        pos2(content.left() + title_width, content.bottom()),
    );
    let meta_rect = Rect::from_min_max(
        pos2(
            (title_rect.right() + gap).min(content.right()),
            content.top(),
        ),
        content.max,
    );
    let title_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let meta_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let title = elide_text(ui, title, &title_font, title_rect.width());
    let meta = elide_text(ui, meta, &meta_font, meta_rect.width());
    ui.painter().with_clip_rect(title_rect).text(
        title_rect.left_center(),
        Align2::LEFT_CENTER,
        title,
        title_font,
        tokens.color.text,
    );
    let meta_left = ui
        .painter()
        .with_clip_rect(meta_rect)
        .text(
            meta_rect.right_center(),
            Align2::RIGHT_CENTER,
            meta,
            meta_font,
            tokens.color.text_faint,
        )
        .left();
    if let Some(problem) = problem {
        paint_problem_mark(
            ui.painter(),
            Rect::from_center_size(
                pos2(meta_left - 6.0 - CHIP_MARK_SIDE * 0.5, meta_rect.center().y),
                vec2(CHIP_MARK_SIDE, CHIP_MARK_SIDE),
            ),
            problem,
        );
    }
}

/// How one ledger row is painted.
#[derive(Clone, Copy, PartialEq, Eq)]
enum WorkspaceRowKind {
    /// The column header: tighter, filled, and unclickable.
    Header,
    /// An ordinary body row.
    Body,
    /// The body row the page's selection points at.
    Selected,
}

impl WorkspaceRowKind {
    const fn is_header(self) -> bool {
        matches!(self, Self::Header)
    }

    /// The selected row for a boolean the caller already holds.
    const fn body(selected: bool) -> Self {
        if selected { Self::Selected } else { Self::Body }
    }
}

/// One ledger row.
///
/// `faint_columns` are the cells holding a placeholder rather than a value,
/// which take the faint tone so an absent value never competes with a real
/// one. `problem` marks one cell — a resource the reader has to look at —
/// with the kit's mark in front of its words; no cell's text is ever
/// coloured.
fn workspace_table_row<const N: usize>(
    ui: &mut Ui,
    width: f32,
    cells: [&str; N],
    fractions: [f32; N],
    kind: WorkspaceRowKind,
    mono_columns: &[usize],
    faint_columns: &[usize],
    problem: Option<(usize, Problem)>,
) -> (egui::Response, Vec<Rect>) {
    let tokens = Tokens::get(ui.ctx());
    let header = kind.is_header();
    let height = if header {
        workspace_table_column_header_height(ui)
    } else {
        workspace_table_row_height(ui)
    };
    let (rect, response) = ui.allocate_exact_size(
        vec2(width, height),
        if header {
            Sense::hover()
        } else {
            Sense::click()
        },
    );
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    // A row the reader can select has to answer the pointer. Marking the
    // selection with a two-pixel rule alone left the table looking painted on.
    if header {
        painter.rect_filled(rect, 0.0, tokens.color.bg_panel_2);
    } else if kind == WorkspaceRowKind::Selected {
        painter.rect_filled(rect, 0.0, tokens.color.bg_active);
    } else if response.hovered() {
        painter.rect_filled(rect, 0.0, tokens.color.bg_hover);
    }
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
    if kind == WorkspaceRowKind::Selected {
        painter.rect_filled(
            Rect::from_min_size(rect.left_top(), vec2(2.0, rect.height())),
            0.0,
            tokens.color.accent,
        );
    }
    let mut left = rect.left();
    let mut cell_rects = Vec::with_capacity(N);
    for (index, (text, fraction)) in cells.iter().zip(fractions.iter()).enumerate() {
        let right = if index + 1 == N {
            rect.right()
        } else {
            left + rect.width() * fraction
        };
        let cell = Rect::from_min_max(pos2(left, rect.top()), pos2(right, rect.bottom()));
        if index > 0 {
            painter.vline(left, rect.y_range(), Stroke::new(1.0, tokens.color.border));
        }
        let text_color = if header || faint_columns.contains(&index) {
            tokens.color.text_faint
        } else {
            tokens.color.text_dim
        };
        let font = if header {
            theme::sans(tokens::FS_0, FontWeight::Medium)
        } else if mono_columns.contains(&index) {
            theme::mono(tokens::FS_0, FontWeight::Regular)
        } else {
            theme::sans(tokens::FS_0, FontWeight::Regular)
        };
        let mark = problem.and_then(|(column, problem)| (column == index).then_some(problem));
        if let Some(problem) = mark {
            paint_problem_mark(
                &painter,
                Rect::from_center_size(
                    pos2(cell.left() + 8.0 + CHIP_MARK_SIDE * 0.5, cell.center().y),
                    vec2(CHIP_MARK_SIDE, CHIP_MARK_SIDE),
                ),
                problem,
            );
        }
        let text_left = 8.0
            + if mark.is_some() {
                CHIP_MARK_ADVANCE
            } else {
                0.0
            };
        let text_rect = cell.shrink2(vec2(text_left, 0.0));
        let text = elide_text(ui, text, &font, text_rect.width());
        painter.with_clip_rect(text_rect).text(
            cell.left_center() + vec2(text_left, 0.0),
            Align2::LEFT_CENTER,
            text,
            font,
            text_color,
        );
        cell_rects.push(cell);
        left = right;
    }
    let announcement = {
        let joined = cells.join(", ");
        problem.map_or(joined.clone(), |(_, problem)| problem.announce(&joined))
    };
    response.widget_info(|| {
        if header {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
        } else {
            egui::WidgetInfo::selected(
                egui::WidgetType::SelectableLabel,
                ui.is_enabled(),
                kind == WorkspaceRowKind::Selected,
                &announcement,
            )
        }
    });
    // No-ops for a header, which senses hover only and can never hold focus.
    theme::paint_focus_ring(ui, &response, rect);
    (response, cell_rects)
}

fn workspace_empty_table_row(ui: &mut Ui, width: f32, text: &str) {
    let tokens = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(width, workspace_table_row_height(ui)), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let text_rect = rect.shrink2(vec2(10.0, 0.0));
    let visible_text = elide_text(ui, text, &font, text_rect.width());
    ui.painter().with_clip_rect(text_rect).text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        visible_text,
        font,
        tokens.color.text_faint,
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
}

fn short_identity(identity: &str) -> String {
    identity.chars().take(8).collect()
}

fn muted(ui: &mut Ui, text: &str) {
    let tokens = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(tokens.color.text_faint),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() <= 0.001, "{actual} != {expected}");
    }

    #[test]
    fn workspace_table_fractions_are_complete() {
        assert_close(
            [0.12_f32, 0.15, 0.19, 0.10, 0.11, 0.10, 0.23]
                .into_iter()
                .sum(),
            1.0,
        );
        assert_close([0.24_f32, 0.25, 0.17, 0.21, 0.13].into_iter().sum(), 1.0);
    }

    #[test]
    fn technology_copy_does_not_claim_physical_pdk_qualification() {
        for copy in [
            TECHNOLOGY_SURFACE_TITLE,
            TECHNOLOGY_SURFACE_ACTION,
            TECHNOLOGY_DIALOG_TITLE,
            TECHNOLOGY_DIALOG_PRIMARY,
            TECHNOLOGY_DIALOG_CURRENT,
            TECHNOLOGY_DIALOG_PENDING,
        ] {
            let copy = copy.to_ascii_lowercase();
            assert!(!copy.contains("qualified"));
            assert!(!copy.contains("rule deck"));
        }
    }
}
