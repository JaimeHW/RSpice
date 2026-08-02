//! Production Project workspace.
//!
//! The persistent section chrome hosts five focused projections of the live
//! project. Every page reads authoritative project state and dispatches real
//! workbench workflows; no mockup fixture data is admitted here.

mod configuration;
mod dependencies;
mod library;
mod library_publication;
mod overview;
mod recovery;
mod technology;

use egui::{
    Align, Align2, Color32, Context, Key, Layout, Margin, Modifiers, Rect, Response, ScrollArea,
    Sense, Stroke, Ui, WidgetInfo, WidgetType, pos2, vec2,
};

use crate::diagnostics::ConsoleMessage;
use crate::state::{ProjectTechnologyBinding, model_library::ModelLibraryManager};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, select};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::DesignCheckStatus;

use super::super::commands::vocabulary::Command;
use super::super::design_system::{WorkbenchIcon, property_card, property_row, status_dot};
use super::super::state::{ProjectPage, Workspace};
use recovery::recovery;
use technology::{open_technology_attachment_dialog, show_technology_attachment_dialog};
#[cfg(target_arch = "wasm32")]
use technology::{poll_browser_recovery_completions, poll_browser_technology_checkpoint};

const PROJECT_TAB_HEIGHT: f32 = 24.0;
const WORKSPACE_TABLE_HEADER_HEIGHT: f32 = 37.0;
const WORKSPACE_TABLE_COLUMN_HEADER_HEIGHT: f32 = 27.0;
const WORKSPACE_TABLE_ROW_HEIGHT: f32 = 36.0;

/// Width that is both allocated to this UI and actually visible through the
/// current clip. Split-pane children can otherwise inherit the unconstrained
/// central-panel width and paint beneath the Inspector dock.
fn visible_workspace_width(ui: &Ui) -> f32 {
    ui.available_rect_before_wrap()
        .intersect(ui.clip_rect())
        .width()
        .max(1.0)
}
#[cfg(test)]
const TECHNOLOGY_SURFACE_TITLE: &str = "Model-library technology contract";
const TECHNOLOGY_SURFACE_ACTION: &str = "Attach model technology\u{2026}";
const TECHNOLOGY_DIALOG_TITLE: &str = "Attach model-library technology";
const TECHNOLOGY_DIALOG_PRIMARY: &str = "Attach authenticated binding";
const TECHNOLOGY_DIALOG_CURRENT: &str = "Binding already attached";
const TECHNOLOGY_DIALOG_PENDING: &str = "Writing checkpoint\u{2026}";

#[cfg(target_arch = "wasm32")]
struct BrowserTechnologyCheckpointCompletion {
    project_id: String,
    expected_revision: u64,
    binding: ProjectTechnologyBinding,
    authority: crate::state::ProjectTechnologyChangeAuthority,
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

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let ctx = ui.ctx().clone();
    #[cfg(target_arch = "wasm32")]
    {
        poll_browser_technology_checkpoint(&ctx, app);
        poll_browser_recovery_completions(&ctx, app);
    }
    let tokens = Tokens::get(ui.ctx());
    egui::Frame::new().fill(tokens.color.bg_app).show(ui, |ui| {
        project_chrome(ui, app);
        let page = app.state.workbench.project_page;
        let panel = ui.scope(|ui| match page {
            ProjectPage::Overview => overview::overview(ui, app),
            ProjectPage::Library => library::library(ui, app),
            page => {
                ScrollArea::vertical()
                    .id_salt(("workbench.project.surface", page))
                    .auto_shrink([false, false])
                    .show(ui, |ui| match page {
                        ProjectPage::Configuration => {
                            configuration::configuration(ui, app);
                        }
                        ProjectPage::Dependencies => dependencies::dependencies(ui, app),
                        ProjectPage::Recovery => recovery(ui, app),
                        ProjectPage::Overview | ProjectPage::Library => {
                            unreachable!("page owns its scroll contract")
                        }
                    });
            }
        });
        ui.ctx().accesskit_node_builder(panel.response.id, |node| {
            node.set_role(egui::accesskit::Role::TabPanel);
            node.set_label(format!("Project {}", page.label()));
        });
    });
    show_technology_attachment_dialog(&ctx, app);
}

/// Reuse the exact project-owned Library/Cell/View browser from a specialist
/// route without introducing a second selection or mutation owner.
pub(super) fn show_library_browser(ui: &mut Ui, app: &mut RSpiceApp) {
    library::library(ui, app);
}

fn project_chrome(ui: &mut Ui, app: &mut RSpiceApp) {
    let tokens = Tokens::get(ui.ctx());
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
                >= project_tabs_desired_width(ui) + project_context_desired_width(app) + 10.0;
            if inline {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    project_tabs(ui, app);
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        project_context_cluster(ui, app);
                    });
                });
            } else {
                ScrollArea::horizontal()
                    .id_salt("workbench.project.tabs.compact")
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        project_tabs(ui, app);
                    });
                ui.horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        project_context_cluster(ui, app);
                    });
                });
            }
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
}

fn project_context_cluster(ui: &mut Ui, app: &mut RSpiceApp) {
    let tokens = Tokens::get(ui.ctx());
    let desired = project_context_desired_width(app);
    let width = ui.available_width().min(desired).max(1.0);
    ui.allocate_ui_with_layout(
        vec2(width, PROJECT_TAB_HEIGHT),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            project_revision_label(ui, app, &tokens);
            if app.state.workbench.project_page != ProjectPage::Overview {
                project_health_chips(ui, app);
                project_page_actions(ui, app);
            }
        },
    );
}

fn project_context_desired_width(app: &RSpiceApp) -> f32 {
    if app.state.workbench.project_page == ProjectPage::Overview {
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

fn project_tabs(ui: &mut Ui, app: &mut RSpiceApp) {
    let tokens = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Medium);
    let tab_ids = ProjectPage::ALL.map(|page| ui.id().with(("project-section-tab", page)));
    let shown = ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        for (index, page) in ProjectPage::ALL.into_iter().enumerate() {
            let text_width = ui
                .painter()
                .layout_no_wrap(page.label().to_owned(), font.clone(), tokens.color.text)
                .size()
                .x;
            let (_, rect) = ui.allocate_space(vec2(text_width + 20.0, PROJECT_TAB_HEIGHT));
            let response = ui.interact(rect, tab_ids[index], Sense::click());
            let selected = app.state.workbench.project_page == page;
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
            });
            if response.hovered() {
                ui.painter().rect_filled(rect, 0.0, tokens.color.bg_hover);
            }
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                page.label(),
                font.clone(),
                if selected {
                    tokens.color.text
                } else {
                    tokens.color.text_dim
                },
            );
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
                app.state.workbench.project_page = page;
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
                    app.state.workbench.project_page = ProjectPage::ALL[target];
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

fn project_revision_label(ui: &mut Ui, app: &RSpiceApp, tokens: &Tokens) {
    ui.label(
        egui::RichText::new(format!(
            "main @ r{}",
            app.state.workspace.project.revision().get()
        ))
        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
        .color(tokens.color.text_faint),
    )
    .on_hover_text("Current mutable project input revision");
}

fn project_health_chips(ui: &mut Ui, app: &mut RSpiceApp) {
    let tokens = Tokens::get(ui.ctx());
    if app.state.is_netlist_first_without_schematic() {
        let (label, description, tone) = app
            .state
            .workspace
            .netlist_document
            .as_ref()
            .map_or_else(
                || {
                    (
                        "source unavailable".to_owned(),
                        "The project has no canonical SPICE source document".to_owned(),
                        tokens.color.err,
                    )
                },
                |document| match document.validation() {
                    None => (
                        "source not validated".to_owned(),
                        "Open and validate the retained SPICE source deck".to_owned(),
                        tokens.color.warn,
                    ),
                    Some(report) if report.content_digest() != document.content_digest() => (
                        "source stale".to_owned(),
                        "The source changed after the retained validation".to_owned(),
                        tokens.color.warn,
                    ),
                    Some(report) => {
                        let errors = report.error_count();
                        let warnings = report.warning_count();
                        (
                            format!("source {errors} err · {warnings} adv"),
                            format!(
                                "Retained SPICE source validation: {errors} errors and {warnings} advisories"
                            ),
                            if errors > 0 {
                                tokens.color.err
                            } else if warnings > 0 {
                                tokens.color.warn
                            } else {
                                tokens.color.ok
                            },
                        )
                    }
                },
            );
        if project_status_chip(ui, "project-source", &label, &description, tone, true).clicked() {
            Command::OpenWorkspace(Workspace::Netlist).execute(app);
        }
    } else {
        let root = crate::state::CellViewRef::new(
            &app.state.workspace.project.root_library,
            &app.state.workspace.project.top_cell,
            crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
        );
        let (checks_label, checks_description, checks_tone) = match app
            .state
            .project_root_design_check_status()
        {
            DesignCheckStatus::Current(receipt) => {
                let summary = receipt.result.summary();
                let errors = summary.critical + summary.errors;
                let advisories = summary.warnings + summary.info;
                (
                    format!("checks {errors} err · {advisories} adv"),
                    format!(
                        "Project-root schematic checks: {errors} blocking errors and {advisories} advisories"
                    ),
                    if errors == 0 {
                        tokens.color.ok
                    } else {
                        tokens.color.err
                    },
                )
            }
            DesignCheckStatus::NotRun => (
                "checks not run".to_owned(),
                "Run schematic checks for the project root".to_owned(),
                tokens.color.warn,
            ),
            DesignCheckStatus::Stale(_) => (
                "checks stale".to_owned(),
                "Project-root inputs changed after the retained check".to_owned(),
                tokens.color.warn,
            ),
            DesignCheckStatus::Unavailable { reason, .. } => {
                ("checks unavailable".to_owned(), reason, tokens.color.err)
            }
        };
        if project_status_chip(
            ui,
            "project-checks",
            &checks_label,
            &checks_description,
            checks_tone,
            true,
        )
        .clicked()
        {
            app.state.open_workspace_view(root);
            Command::OpenWorkspace(Workspace::Design).execute(app);
            if Command::RunChecks.is_enabled(app) {
                Command::RunChecks.execute(app);
            }
        }
    }

    let active_configuration = app.state.workspace.configuration_sets.active();
    let configuration = active_configuration.map_or("none", |configuration| configuration.name());
    if project_status_chip(
        ui,
        "project-configuration",
        &format!("config {configuration}"),
        &format!("Active project configuration: {configuration}"),
        if active_configuration.is_none() {
            tokens.color.warn
        } else {
            tokens.color.ok
        },
        true,
    )
    .clicked()
    {
        Command::ConfigurationSets.execute(app);
    }

    if let Some((index, run)) = app.state.simulation.runs.iter().enumerate().next() {
        let has_dataset = !run.analyses.is_empty();
        let run_label = format!(
            "run {} · {}",
            run.id,
            if run.success && has_dataset {
                "complete"
            } else if has_dataset {
                "review"
            } else {
                "no data"
            }
        );
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
            if run.success && has_dataset {
                tokens.color.ok
            } else {
                tokens.color.warn
            },
            true,
        )
        .clicked()
            && app.state.simulation.select_run(index)
        {
            Command::OpenWorkspace(if has_dataset {
                Workspace::Results
            } else {
                Workspace::Simulate
            })
            .execute(app);
        }
    } else {
        project_status_chip(
            ui,
            "project-latest-run",
            "run none",
            "No retained simulation run",
            tokens.color.text_faint,
            false,
        );
    }
}

fn project_status_chip(
    ui: &mut Ui,
    id_salt: &'static str,
    label: &str,
    accessible_label: &str,
    color: Color32,
    enabled: bool,
) -> Response {
    let tokens = Tokens::get(ui.ctx());
    let font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let width = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font.clone(), color)
        .size()
        .x
        + 14.0;
    let (_, rect) = ui.allocate_space(vec2(width, 22.0));
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
    ui.painter().text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        font,
        if enabled {
            color
        } else {
            tokens.color.text_faint
        },
    );
    response.widget_info(|| {
        WidgetInfo::labeled(WidgetType::Button, enabled, accessible_label.to_owned())
    });
    if enabled {
        theme::paint_focus_ring(ui, &response, rect);
    }
    response.on_hover_text(accessible_label)
}

fn project_page_actions(ui: &mut Ui, app: &mut RSpiceApp) {
    match app.state.workbench.project_page {
        ProjectPage::Overview => {}
        ProjectPage::Library => {
            if Button::new("Attach library\u{2026}").show(ui).clicked() {
                Command::PdkSettings.execute(app);
            }
        }
        ProjectPage::Configuration => {
            if Button::new("New override\u{2026}").show(ui).clicked() {
                crate::workbench::app::open_configuration_binding_dialog(&mut app.state);
            }
            if Button::new("Validate").show(ui).clicked() {
                Command::PreflightChecks.execute(app);
            }
        }
        ProjectPage::Dependencies | ProjectPage::Recovery => {}
    }
}

fn workspace_table_panel_header(ui: &mut Ui, title: &str, meta: &str, meta_color: Color32) {
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
    ui.painter().with_clip_rect(meta_rect).text(
        meta_rect.right_center(),
        Align2::RIGHT_CENTER,
        meta,
        meta_font,
        meta_color,
    );
}

fn workspace_table_row<const N: usize>(
    ui: &mut Ui,
    width: f32,
    cells: [&str; N],
    fractions: [f32; N],
    header: bool,
    mono_columns: &[usize],
    colored_cells: &[(usize, Color32)],
) -> (egui::Response, Vec<Rect>) {
    let tokens = Tokens::get(ui.ctx());
    let height = if header {
        WORKSPACE_TABLE_COLUMN_HEADER_HEIGHT
    } else {
        WORKSPACE_TABLE_ROW_HEIGHT
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
    if header {
        painter.rect_filled(rect, 0.0, tokens.color.bg_panel_2);
    }
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, tokens.color.border),
    );
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
        let text_color = colored_cells
            .iter()
            .find_map(|(column, color)| (*column == index).then_some(*color))
            .unwrap_or(if header {
                tokens.color.text_faint
            } else {
                tokens.color.text_dim
            });
        let font = if header {
            theme::sans(tokens::FS_0, FontWeight::Medium)
        } else if mono_columns.contains(&index) {
            theme::mono(tokens::FS_0, FontWeight::Regular)
        } else {
            theme::sans(tokens::FS_0, FontWeight::Regular)
        };
        let text_rect = cell.shrink2(vec2(8.0, 0.0));
        let text = elide_text(ui, text, &font, text_rect.width());
        painter.with_clip_rect(text_rect).text(
            cell.left_center() + vec2(8.0, 0.0),
            Align2::LEFT_CENTER,
            text,
            font,
            text_color,
        );
        cell_rects.push(cell);
        left = right;
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), cells.join(", "))
    });
    // No-ops for a header, which senses hover only and can never hold focus.
    theme::paint_focus_ring(ui, &response, rect);
    (response, cell_rects)
}

fn workspace_empty_table_row(ui: &mut Ui, width: f32, text: &str) {
    let tokens = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(width, WORKSPACE_TABLE_ROW_HEIGHT), Sense::hover());
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

fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

fn muted(ui: &mut Ui, text: &str) {
    let tokens = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(tokens.color.text_faint),
    );
}

fn elide_text(ui: &Ui, text: &str, font: &egui::FontId, max_width: f32) -> String {
    if max_width <= 0.0 {
        return String::new();
    }
    if ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), Color32::WHITE)
        .size()
        .x
        <= max_width
    {
        return text.to_owned();
    }
    let ellipsis = "\u{2026}";
    if ui
        .painter()
        .layout_no_wrap(ellipsis.to_owned(), font.clone(), Color32::WHITE)
        .size()
        .x
        > max_width
    {
        return String::new();
    }
    let mut output = text.to_owned();
    while !output.is_empty() {
        output.pop();
        let candidate = format!("{output}{ellipsis}");
        if ui
            .painter()
            .layout_no_wrap(candidate.clone(), font.clone(), Color32::WHITE)
            .size()
            .x
            <= max_width
        {
            return candidate;
        }
    }
    ellipsis.to_owned()
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

    #[test]
    fn every_project_page_renders_at_desktop_tablet_and_phone_widths() {
        for page in ProjectPage::ALL {
            for size in [
                egui::vec2(1440.0, 900.0),
                egui::vec2(820.0, 1180.0),
                egui::vec2(390.0, 844.0),
            ] {
                let ctx = egui::Context::default();
                crate::ui::Theme::default().apply(&ctx);
                let mut app = RSpiceApp::test_instance();
                app.state.workbench.project_page = page;

                let output = ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
                    },
                );

                assert!(
                    !output.shapes.is_empty(),
                    "{} did not render at {}x{}",
                    page.label(),
                    size.x,
                    size.y
                );
            }
        }
    }
}
