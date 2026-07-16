//! Project workspace surfaces from the workbench mockup.
//!
//! Every value is resolved from the live project, library, simulation, and
//! recovery state. The surface never substitutes fixture counts or synthetic
//! project history when a project has not produced that evidence yet.

use egui::{Align, Align2, Color32, Layout, Rect, ScrollArea, Sense, Stroke, Ui, vec2};

use crate::common::RSpiceApp;
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;

use super::super::commands::Command;
use super::super::design_system::{WorkbenchIcon, card, property_row, status_dot};
use super::super::state::{ModelsPage, ProjectPage, Workspace};

const PROJECT_RESPONSIVE_BREAKPOINT: f32 = 820.0;
const PROJECT_PHONE_BREAKPOINT: f32 = 560.0;
const PROJECT_GRID_GAP: f32 = 1.0;
const PROJECT_HISTORY_MIN_WIDTH: f32 = 280.0;
const PROJECT_PRIMARY_SHARE: f32 = 0.65;
const STATUS_CARD_MIN_HEIGHT: f32 = 68.0;
const DASHBOARD_CARD_HEADER_HEIGHT: f32 = 37.0;
const TOUCH_TARGET_HEIGHT: f32 = 44.0;
const PROJECT_ENTRY_ROW_HEIGHT: f32 = 42.0;
const PROJECT_PHONE_ENTRY_ROW_HEIGHT: f32 = 52.0;
const RECENT_RUN_ROW_HEIGHT: f32 = 72.0;
const CONFIGURATION_TABLE_HEADER_HEIGHT: f32 = 27.0;
const CONFIGURATION_TABLE_ROW_HEIGHT: f32 = 28.0;
const CONFIGURATION_TABLE_MIN_WIDTH: f32 = 540.0;

#[derive(Debug, Clone, Copy, PartialEq)]
struct ProjectSurfaceLayout {
    content_width: f32,
    horizontal_inset: f32,
}

impl ProjectSurfaceLayout {
    fn resolve(surface_width: f32) -> Self {
        let surface_width = surface_width.max(0.0);
        Self {
            content_width: surface_width,
            horizontal_inset: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DashboardLayout {
    status_columns: usize,
    status_card_width: f32,
    project_columns: usize,
    project_primary_width: f32,
    project_history_width: f32,
}

impl DashboardLayout {
    fn resolve(viewport_width: f32, content_width: f32) -> Self {
        let content_width = content_width.max(0.0);
        let compact = viewport_width <= PROJECT_RESPONSIVE_BREAKPOINT;
        let status_columns = if compact { 2 } else { 4 };
        let project_columns = if compact { 1 } else { 2 };
        let status_card_width = content_width / status_columns as f32;

        let (project_primary_width, project_history_width) = if compact {
            (content_width, content_width)
        } else {
            let usable_width = (content_width - PROJECT_GRID_GAP).max(0.0);
            let history_width = (usable_width * (1.0 - PROJECT_PRIMARY_SHARE))
                .max(PROJECT_HISTORY_MIN_WIDTH.min(usable_width));
            (usable_width - history_width, history_width)
        };

        Self {
            status_columns,
            status_card_width,
            project_columns,
            project_primary_width,
            project_history_width,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ProjectEntryLayout {
    height: f32,
    status_below_title: bool,
    phone: bool,
}

impl ProjectEntryLayout {
    fn resolve(viewport_width: f32, touch_targets: bool) -> Self {
        let phone = viewport_width <= PROJECT_PHONE_BREAKPOINT;
        let status_below_title = viewport_width <= PROJECT_RESPONSIVE_BREAKPOINT;
        let height = if phone {
            PROJECT_PHONE_ENTRY_ROW_HEIGHT
        } else if status_below_title || touch_targets {
            TOUCH_TARGET_HEIGHT
        } else {
            PROJECT_ENTRY_ROW_HEIGHT
        };
        Self {
            height,
            status_below_title,
            phone,
        }
    }
}

fn dashboard_card_header_height(touch_targets: bool) -> f32 {
    if touch_targets {
        TOUCH_TARGET_HEIGHT
    } else {
        DASHBOARD_CARD_HEADER_HEIGHT
    }
}

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        let surface = ProjectSurfaceLayout::resolve(ui.available_width());
        ScrollArea::vertical()
            .id_salt("workbench.project.surface")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_width(surface.content_width);
                match app.state.workbench.project_page {
                    ProjectPage::Dashboard => dashboard(ui, app),
                    ProjectPage::Configuration => configuration(ui, app),
                    ProjectPage::Technology => technology(ui, app),
                    ProjectPage::Dependencies => dependencies(ui, app),
                    ProjectPage::Recovery => recovery(ui, app),
                }
            });
    });
}

fn dashboard(ui: &mut Ui, app: &mut RSpiceApp) {
    let layout = DashboardLayout::resolve(ui.ctx().content_rect().width(), ui.available_width());
    let revision = app.state.workspace.project.revision().get();
    let project_name = app.state.workspace.project.display_name().to_owned();
    let sheet_count = app.state.workspace.schematic_buffers.len();
    let instance_count = app.state.schematic.components.len();
    let dirty = app.state.schematic.is_dirty || app.state.workspace.any_dirty();

    header_with_actions(
        ui,
        &format!("PROJECT WORKSPACE · REVISION {revision}"),
        &project_name,
        &format!(
            "Hierarchical analog design · {sheet_count} sheet{} · {instance_count} instance{} · {}",
            plural(sheet_count),
            plural(instance_count),
            if dirty {
                "working changes"
            } else {
                "saved state current"
            }
        ),
        |ui, app| {
            if Button::new("Open project")
                .icon(Icon::Folder)
                .show(ui)
                .clicked()
            {
                Command::OpenProject.execute(app);
            }
            if Button::new("Open top schematic")
                .icon(Icon::Schematic)
                .accent()
                .show(ui)
                .clicked()
            {
                Command::OpenWorkspace(Workspace::Design).execute(app);
            }
        },
        app,
    );
    engineering_status(ui, app, layout);
    project_grid(ui, app, layout);
}

struct StatusCardSpec {
    label: &'static str,
    value: String,
    detail: String,
    ok: bool,
    command: Command,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StatusCardCopy {
    value: String,
    detail: String,
    ok: bool,
}

fn verification_card_copy(
    spec_count: usize,
    active_dataset: Option<&str>,
    covered_specs: usize,
    worst_yield_percent: Option<f64>,
    all_covered_specs_pass: bool,
) -> StatusCardCopy {
    let Some(dataset) = active_dataset else {
        return StatusCardCopy {
            value: format!("0/{spec_count} covered"),
            detail: "No active immutable dataset".to_owned(),
            ok: false,
        };
    };
    let dataset = short_identity(dataset);
    if spec_count == 0 {
        return StatusCardCopy {
            value: "No specifications".to_owned(),
            detail: format!("Dataset {dataset} · no mapped specifications"),
            ok: false,
        };
    }
    StatusCardCopy {
        value: worst_yield_percent.map_or_else(
            || format!("{covered_specs}/{spec_count} covered"),
            |yield_percent| format!("{yield_percent:.2}%"),
        ),
        detail: format!("{covered_specs}/{spec_count} specs · dataset {dataset}"),
        ok: covered_specs == spec_count && all_covered_specs_pass,
    }
}

fn model_dependency_card_copy(
    resolved_source_count: usize,
    scan_error_count: usize,
) -> StatusCardCopy {
    if resolved_source_count == 0 {
        return StatusCardCopy {
            value: "0 files".to_owned(),
            detail: if scan_error_count == 0 {
                "No resolved model dependency set".to_owned()
            } else {
                format!("No resolved set · {scan_error_count} scan diagnostics")
            },
            ok: false,
        };
    }
    StatusCardCopy {
        value: format!("{resolved_source_count} files"),
        detail: if scan_error_count == 0 {
            "Resolved source closure".to_owned()
        } else {
            format!("Resolved closure · {scan_error_count} scan diagnostics")
        },
        ok: scan_error_count == 0,
    }
}

fn short_identity(identity: &str) -> String {
    identity.chars().take(8).collect()
}

fn resolved_model_source_count(app: &RSpiceApp) -> usize {
    let mut resolved_sources = std::collections::BTreeSet::new();
    for library in app.state.model_library_manager.libraries_sorted() {
        for source in &library.source_closure {
            resolved_sources.insert(source.path.as_path());
        }
    }
    resolved_sources.len()
}

fn verification_status_for(app: &RSpiceApp) -> StatusCardCopy {
    let spec_count = app.state.workspace.specs.len();
    let active_run = app.state.simulation.active_run();
    let active_yield = active_run
        .and_then(|run| {
            app.state
                .simulation
                .yield_results_for_dataset(run.dataset_id)
        })
        .unwrap_or(&[]);
    let mut covered_specs = 0;
    let mut all_covered_specs_pass = true;
    for spec in &app.state.workspace.specs {
        let yield_result = active_yield.iter().find(|result| {
            result.spec.target.eq_ignore_ascii_case(&spec.measurement) && result.total_runs > 0
        });
        let measured_value = active_run.and_then(|run| {
            run.analyses
                .iter()
                .flat_map(|analysis| &analysis.measurements)
                .find(|measurement| measurement.name.eq_ignore_ascii_case(&spec.measurement))
                .and_then(|measurement| measurement.value)
                .filter(|value| value.is_finite())
        });
        if yield_result.is_some() || measured_value.is_some() {
            covered_specs += 1;
            let passes = yield_result
                .map(|result| result.fail_count == 0)
                .or_else(|| measured_value.map(|value| spec.passes(value)))
                .unwrap_or(false);
            all_covered_specs_pass &= passes;
        } else {
            all_covered_specs_pass = false;
        }
    }
    let worst_yield_percent = active_yield
        .iter()
        .map(|result| result.yield_percent)
        .filter(|value| value.is_finite())
        .min_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal));
    let active_dataset = active_run.map(|run| run.dataset_id.to_string());
    verification_card_copy(
        spec_count,
        active_dataset.as_deref(),
        covered_specs,
        worst_yield_percent,
        all_covered_specs_pass,
    )
}

fn engineering_status(ui: &mut Ui, app: &mut RSpiceApp, layout: DashboardLayout) {
    let t = Tokens::get(ui.ctx());
    let topology = app.state.schematic.topology_version();
    let checks_current = app.state.dialogs.drc_checked_version == topology;
    let check_summary = checks_current
        .then(|| {
            app.state
                .dialogs
                .drc_results
                .as_ref()
                .map(|result| result.summary())
        })
        .flatten();
    let enabled_analyses = app.state.sim_setup.enabled_analysis_instance_count();
    let valid_analyses = app
        .state
        .sim_setup
        .enabled_analysis_instances()
        .filter(|instance| {
            app.state
                .sim_setup
                .analysis_draft_validation_error(instance.draft())
                .is_none()
        })
        .count();
    let graph_valid = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .is_ok_and(|plan| plan.validation_issues().is_empty());
    let model_errors = app.state.pdk_config.scan_errors.len();
    let model_copy = model_dependency_card_copy(resolved_model_source_count(app), model_errors);

    let verification_copy = verification_status_for(app);
    let cards = [
        StatusCardSpec {
            label: "Schematic checks",
            value: check_summary
                .map(|summary| format!("{} errors", summary.errors + summary.critical))
                .unwrap_or_else(|| "stale".to_owned()),
            detail: check_summary
                .map(|summary| format!("{} advisories", summary.warnings + summary.info))
                .unwrap_or_else(|| "Run checks for this design revision".to_owned()),
            ok: check_summary.is_some_and(|summary| summary.passed),
            command: Command::RunChecks,
        },
        StatusCardSpec {
            label: "Simulation plan",
            value: format!("{enabled_analyses} analyses"),
            detail: format!("{valid_analyses} validated"),
            ok: enabled_analyses > 0 && valid_analyses == enabled_analyses && graph_valid,
            command: Command::OpenWorkspace(Workspace::Simulate),
        },
        StatusCardSpec {
            label: "Verification yield",
            value: verification_copy.value,
            detail: verification_copy.detail,
            ok: verification_copy.ok,
            command: Command::OpenWorkspace(Workspace::Verify),
        },
        StatusCardSpec {
            label: "Model dependencies",
            value: model_copy.value,
            detail: model_copy.detail,
            ok: model_copy.ok,
            command: Command::ModelsPage(ModelsPage::Models),
        },
    ];
    let mut requested_command = None;

    egui::Grid::new("workbench.project.engineering-status")
        .num_columns(layout.status_columns)
        .spacing(egui::Vec2::ZERO)
        .show(ui, |ui| {
            for (index, card) in cards.iter().enumerate() {
                let clicked = ui
                    .allocate_ui_with_layout(
                        egui::vec2(layout.status_card_width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| {
                            ui.set_width(layout.status_card_width);
                            status_card(
                                ui,
                                card.label,
                                &card.value,
                                &card.detail,
                                card.ok,
                                (index + 1) % layout.status_columns != 0,
                            )
                        },
                    )
                    .inner;
                if clicked {
                    requested_command = Some(card.command);
                }
                if (index + 1) % layout.status_columns == 0 {
                    ui.end_row();
                }
            }
        });
    if let Some(command) = requested_command {
        command.execute(app);
    }
    ui.painter().hline(
        ui.min_rect().x_range(),
        ui.cursor().top(),
        egui::Stroke::new(1.0, t.color.border_strong),
    );
}

fn status_card(
    ui: &mut Ui,
    label: &str,
    value: &str,
    detail: &str,
    ok: bool,
    right_divider: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), STATUS_CARD_MIN_HEIGHT),
        egui::Sense::click(),
    );
    if response.hovered() || response.has_focus() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    if right_divider {
        ui.painter().vline(
            rect.right(),
            rect.y_range(),
            egui::Stroke::new(1.0, t.color.border),
        );
    }
    let painter = ui
        .painter()
        .with_clip_rect(rect.shrink2(egui::vec2(12.0, 0.0)));
    painter.text(
        egui::pos2(rect.left() + 12.0, rect.top() + 9.0),
        egui::Align2::LEFT_TOP,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    painter.text(
        egui::pos2(rect.left() + 12.0, rect.top() + 26.0),
        egui::Align2::LEFT_TOP,
        value,
        theme::mono(17.0, FontWeight::Medium),
        if ok { t.color.ok } else { t.color.warn },
    );
    painter.text(
        egui::pos2(rect.left() + 12.0, rect.top() + 49.0),
        egui::Align2::LEFT_TOP,
        detail,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            format!("Open {label}: {value}. {detail}"),
        )
    });
    theme::paint_focus_ring_outset(ui, &response, response.rect);
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

fn project_grid(ui: &mut Ui, app: &mut RSpiceApp, layout: DashboardLayout) {
    if layout.project_columns == 1 {
        let primary = ui.allocate_ui_with_layout(
            egui::vec2(layout.project_primary_width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_width(layout.project_primary_width);
                design_entry_points(ui, app);
            },
        );
        ui.painter().hline(
            primary.response.rect.x_range(),
            primary.response.rect.bottom(),
            egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
        );
        ui.add_space(PROJECT_GRID_GAP);
        ui.allocate_ui_with_layout(
            egui::vec2(layout.project_history_width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_width(layout.project_history_width);
                recent_activity(ui, app);
            },
        );
        return;
    }

    let grid = ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = PROJECT_GRID_GAP;
        ui.allocate_ui_with_layout(
            egui::vec2(layout.project_primary_width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_width(layout.project_primary_width);
                design_entry_points(ui, app);
            },
        );
        ui.allocate_ui_with_layout(
            egui::vec2(layout.project_history_width, 0.0),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.set_width(layout.project_history_width);
                recent_activity(ui, app);
            },
        );
    });
    ui.painter().vline(
        grid.response.rect.left() + layout.project_primary_width + PROJECT_GRID_GAP * 0.5,
        grid.response.rect.y_range(),
        egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
}

fn design_entry_points(ui: &mut Ui, app: &mut RSpiceApp) {
    if dashboard_card_header(ui, "Design entry points", Some("Manage library")) {
        Command::ModelsPage(ModelsPage::Models).execute(app);
    }

    let active_view = app.state.workspace.active_view.clone();
    let open_views = app.state.workspace.open_views.clone();
    for document in open_views {
        let active = document.reference == active_view;
        let spec = ProjectListRow {
            icon: WorkbenchIcon::Design,
            title: format!("{} · {}", document.reference.cell, document.reference.view),
            detail: document.reference.display_path(),
            status: if document.dirty {
                "working changes".to_owned()
            } else if active {
                "active".to_owned()
            } else {
                "saved".to_owned()
            },
            meta: if active {
                format!("{} inst", app.state.schematic.components.len())
            } else {
                "open".to_owned()
            },
            tone: if document.dirty {
                RowTone::Warn
            } else if active {
                RowTone::Accent
            } else {
                RowTone::Neutral
            },
        };
        if project_list_row(ui, &spec) {
            app.state.open_workspace_view(document.reference);
            Command::OpenWorkspace(Workspace::Design).execute(app);
        }
    }

    let enabled_kinds = app
        .state
        .sim_setup
        .enabled_analysis_instances()
        .map(|instance| instance.kind().stable_id().to_uppercase())
        .collect::<Vec<_>>();
    let plan_valid = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .is_ok_and(|plan| plan.validation_issues().is_empty());
    let plan_summary = if enabled_kinds.is_empty() {
        "No enabled analyses".to_owned()
    } else {
        let shown = enabled_kinds.iter().take(6).cloned().collect::<Vec<_>>();
        let remainder = enabled_kinds.len().saturating_sub(shown.len());
        if remainder == 0 {
            shown.join(" · ")
        } else {
            format!("{} · +{remainder}", shown.join(" · "))
        }
    };
    if project_list_row(
        ui,
        &ProjectListRow {
            icon: WorkbenchIcon::Simulate,
            title: "Simulation plan".to_owned(),
            detail: plan_summary,
            status: if plan_valid {
                "validated".to_owned()
            } else {
                "review required".to_owned()
            },
            meta: format!("{} on", enabled_kinds.len()),
            tone: if plan_valid {
                RowTone::Accent
            } else {
                RowTone::Warn
            },
        },
    ) {
        Command::OpenWorkspace(Workspace::Simulate).execute(app);
    }

    let verification = verification_status_for(app);
    if project_list_row(
        ui,
        &ProjectListRow {
            icon: WorkbenchIcon::Verify,
            title: "Active verification evidence".to_owned(),
            detail: verification.detail.clone(),
            status: verification.value,
            meta: format!("{} specs", app.state.workspace.specs.len()),
            tone: if verification.ok {
                RowTone::Ok
            } else {
                RowTone::Warn
            },
        },
    ) {
        Command::OpenWorkspace(Workspace::Verify).execute(app);
    }
}

fn recent_activity(ui: &mut Ui, app: &mut RSpiceApp) {
    dashboard_card_header(ui, "Recent activity", None);
    let has_recovery = app.state.schematic.is_dirty || app.state.workspace.any_dirty();
    let run_limit = if has_recovery { 2 } else { 3 };
    let runs = app
        .state
        .simulation
        .runs
        .iter()
        .enumerate()
        .rev()
        .take(run_limit)
        .map(|(index, run)| {
            let measurement_count = run
                .analyses
                .iter()
                .map(|analysis| analysis.measurements.len())
                .sum::<usize>();
            let passed_count = run
                .analyses
                .iter()
                .flat_map(|analysis| &analysis.measurements)
                .filter(|measurement| measurement.passed)
                .count();
            let active_yield = app
                .state
                .simulation
                .yield_results_for_dataset(run.dataset_id)
                .unwrap_or(&[]);
            let worst_yield = active_yield
                .iter()
                .map(|result| result.yield_percent)
                .filter(|value| value.is_finite())
                .min_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal));
            let status = worst_yield.map_or_else(
                || {
                    if measurement_count == 0 {
                        if run.success {
                            "completed".to_owned()
                        } else {
                            "failed".to_owned()
                        }
                    } else {
                        format!("{passed_count}/{measurement_count} checks")
                    }
                },
                |yield_percent| format!("{yield_percent:.2}% yield"),
            );
            let pass = run.success
                && worst_yield.map_or(passed_count == measurement_count, |value| value >= 100.0);
            (
                index,
                RecentActivityRow {
                    title: run_title(run.id, &run.label),
                    status,
                    detail: format!(
                        "{} analyses · immutable dataset {}",
                        run.analyses.len(),
                        short_identity(&run.dataset_id.to_string())
                    ),
                    foot_left: format_duration(run.elapsed_time),
                    foot_right: format!("run {}", short_identity(&run.run_id.to_string())),
                    tone: if pass { RowTone::Ok } else { RowTone::Warn },
                    interactive: true,
                },
            )
        })
        .collect::<Vec<_>>();
    for (index, row) in runs {
        if recent_activity_row(ui, &row) {
            app.state.simulation.select_run(index);
            Command::OpenWorkspace(Workspace::Results).execute(app);
        }
    }

    if has_recovery {
        let dirty_count = app
            .state
            .workspace
            .open_views
            .iter()
            .filter(|document| document.dirty)
            .count()
            + usize::from(app.state.schematic.is_dirty)
            + usize::from(app.state.workspace.netlist_source_dirty);
        if recent_activity_row(
            ui,
            &RecentActivityRow {
                title: "Autosave checkpoint".to_owned(),
                status: "working changes".to_owned(),
                detail: format!(
                    "{dirty_count} modified document{} retained for recovery",
                    plural(dirty_count)
                ),
                foot_left: "Open Recovery".to_owned(),
                foot_right: format!("revision {}", app.state.workspace.project.revision().get()),
                tone: RowTone::Warn,
                interactive: true,
            },
        ) {
            app.state.workbench.project_page = ProjectPage::Recovery;
        }
    } else if app.state.simulation.runs.is_empty() {
        let row = if let Some(recent) = app.state.recent_files.first() {
            RecentActivityRow {
                title: "Recently opened project".to_owned(),
                status: "local".to_owned(),
                detail: recent.path.display().to_string(),
                foot_left: "No simulation runs yet".to_owned(),
                foot_right: String::new(),
                tone: RowTone::Neutral,
                interactive: false,
            }
        } else {
            RecentActivityRow {
                title: "No recent project activity".to_owned(),
                status: "idle".to_owned(),
                detail: "Run a validated simulation or edit a project document.".to_owned(),
                foot_left: "No immutable datasets".to_owned(),
                foot_right: String::new(),
                tone: RowTone::Neutral,
                interactive: false,
            }
        };
        recent_activity_row(ui, &row);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RowTone {
    Neutral,
    Accent,
    Ok,
    Warn,
}

impl RowTone {
    fn dot_color(self, t: &Tokens) -> egui::Color32 {
        match self {
            Self::Neutral => t.color.text_faint,
            Self::Accent => t.color.accent,
            Self::Ok => t.color.ok,
            Self::Warn => t.color.warn,
        }
    }

    fn text_color(self, t: &Tokens) -> egui::Color32 {
        match self {
            Self::Warn => t.color.warn,
            Self::Neutral | Self::Accent | Self::Ok => t.color.text_dim,
        }
    }
}

struct ProjectListRow {
    icon: WorkbenchIcon,
    title: String,
    detail: String,
    status: String,
    meta: String,
    tone: RowTone,
}

struct RecentActivityRow {
    title: String,
    status: String,
    detail: String,
    foot_left: String,
    foot_right: String,
    tone: RowTone,
    interactive: bool,
}

fn dashboard_card_header(ui: &mut Ui, title: &str, action: Option<&str>) -> bool {
    let t = Tokens::get(ui.ctx());
    let touch_targets = t.metrics.ctl_h >= TOUCH_TARGET_HEIGHT;
    let width = ui.available_width().max(1.0);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(width, dashboard_card_header_height(touch_targets)),
        egui::Sense::hover(),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.top(),
        egui::Stroke::new(1.0, t.color.border_strong),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        egui::pos2(rect.left() + 11.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        title,
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    let mut clicked = false;
    if let Some(action) = action {
        let vertical_inset = if touch_targets { 0.0 } else { 4.0 };
        let action_rect = rect.shrink2(egui::vec2(6.0, vertical_inset));
        ui.scope_builder(
            egui::UiBuilder::new()
                .max_rect(action_rect)
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
            |ui| {
                clicked = Button::new(action).ghost().show(ui).clicked();
            },
        );
    }
    clicked
}

fn project_list_row(ui: &mut Ui, row: &ProjectListRow) -> bool {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let viewport_width = ui.ctx().content_rect().width();
    let layout =
        ProjectEntryLayout::resolve(viewport_width, t.metrics.ctl_h >= TOUCH_TARGET_HEIGHT);
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(width, layout.height), egui::Sense::click());
    if response.hovered() || response.has_focus() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );

    let icon_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + 22.0, rect.center().y),
        egui::vec2(24.0, 24.0),
    );
    ui.painter().rect_filled(icon_rect, 3.0, t.color.accent_dim);
    row.icon
        .paint(ui.painter(), icon_rect.shrink(4.0), t.color.accent);

    let show_meta = !layout.status_below_title && viewport_width > 1120.0;
    let status_width = if viewport_width <= PROJECT_RESPONSIVE_BREAKPOINT {
        76.0
    } else {
        132.0
    };
    let meta_width = if show_meta { 66.0 } else { 0.0 };
    let meta_rect = egui::Rect::from_min_max(
        egui::pos2(rect.right() - 10.0 - meta_width, rect.top()),
        egui::pos2(rect.right() - 10.0, rect.bottom()),
    );
    let status_rect = egui::Rect::from_min_max(
        egui::pos2(meta_rect.left() - status_width, rect.top()),
        egui::pos2(meta_rect.left(), rect.bottom()),
    );
    let text_left = icon_rect.right() + 12.0;
    let text_rect = egui::Rect::from_min_max(
        egui::pos2(text_left, rect.top()),
        egui::pos2((status_rect.left() - 12.0).max(text_left), rect.bottom()),
    );
    let title_top = if layout.phone {
        6.0
    } else if layout.status_below_title {
        2.0
    } else {
        7.0
    };
    let detail_top = if layout.status_below_title {
        title_top + 14.0
    } else {
        23.0
    };
    paint_elided_text(
        ui,
        egui::pos2(text_rect.left(), rect.top() + title_top),
        &row.title,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
        text_rect.width(),
    );
    paint_elided_text(
        ui,
        egui::pos2(text_rect.left(), rect.top() + detail_top),
        &row.detail,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
        text_rect.width(),
    );
    if layout.status_below_title {
        let status_top = rect.bottom() - 14.0;
        ui.painter().circle_filled(
            egui::pos2(text_rect.left() + 3.0, status_top + 6.0),
            2.5,
            row.tone.dot_color(&t),
        );
        paint_elided_text(
            ui,
            egui::pos2(text_rect.left() + 11.0, status_top),
            &row.status,
            theme::mono(tokens::FS_0, FontWeight::Medium),
            row.tone.text_color(&t),
            (text_rect.width() - 14.0).max(0.0),
        );
    } else {
        ui.painter().circle_filled(
            egui::pos2(status_rect.left() + 3.0, status_rect.center().y),
            2.5,
            row.tone.dot_color(&t),
        );
        paint_elided_text(
            ui,
            egui::pos2(status_rect.left() + 11.0, status_rect.center().y - 6.0),
            &row.status,
            theme::mono(tokens::FS_0, FontWeight::Medium),
            row.tone.text_color(&t),
            (status_rect.width() - 14.0).max(0.0),
        );
    }
    if show_meta {
        let font = theme::mono(tokens::FS_0, FontWeight::Regular);
        let text = elide_text(ui, &row.meta, &font, meta_rect.width());
        ui.painter().text(
            meta_rect.right_center(),
            egui::Align2::RIGHT_CENTER,
            text,
            font,
            t.color.text_faint,
        );
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            format!("{}: {}. {}", row.title, row.status, row.detail),
        )
    });
    theme::paint_focus_ring_outset(ui, &response, rect);
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

fn recent_activity_row(ui: &mut Ui, row: &RecentActivityRow) -> bool {
    let t = Tokens::get(ui.ctx());
    let sense = if row.interactive {
        egui::Sense::click()
    } else {
        egui::Sense::hover()
    };
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width().max(1.0), RECENT_RUN_ROW_HEIGHT),
        sense,
    );
    if row.interactive && (response.hovered() || response.has_focus()) {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    let content = rect.shrink2(egui::vec2(10.0, 0.0));
    let status_width = (content.width() * 0.48).min(180.0);
    let title_width = (content.width() - status_width - 8.0).max(0.0);
    paint_elided_text(
        ui,
        egui::pos2(content.left(), rect.top() + 9.0),
        &row.title,
        theme::sans(tokens::FS_0, FontWeight::Medium),
        t.color.text,
        title_width,
    );
    let status_rect = egui::Rect::from_min_max(
        egui::pos2(content.right() - status_width, rect.top()),
        egui::pos2(content.right(), rect.top() + 28.0),
    );
    ui.painter().circle_filled(
        egui::pos2(status_rect.left() + 3.0, status_rect.center().y),
        2.5,
        row.tone.dot_color(&t),
    );
    let status_font = theme::mono(tokens::FS_0, FontWeight::Medium);
    let status = elide_text(ui, &row.status, &status_font, status_width - 11.0);
    ui.painter().text(
        egui::pos2(status_rect.right(), status_rect.center().y),
        egui::Align2::RIGHT_CENTER,
        status,
        status_font,
        row.tone.text_color(&t),
    );
    paint_elided_text(
        ui,
        egui::pos2(content.left(), rect.top() + 31.0),
        &row.detail,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
        content.width(),
    );
    paint_elided_text(
        ui,
        egui::pos2(content.left(), rect.top() + 53.0),
        &row.foot_left,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        if row.interactive {
            t.color.text_dim
        } else {
            t.color.text_faint
        },
        content.width() * 0.58,
    );
    let footer_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let footer_right = elide_text(ui, &row.foot_right, &footer_font, content.width() * 0.4);
    ui.painter().text(
        egui::pos2(content.right(), rect.top() + 53.0),
        egui::Align2::RIGHT_TOP,
        footer_right,
        footer_font,
        t.color.text_faint,
    );
    if row.interactive {
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Button,
                ui.is_enabled(),
                format!("{}: {}. {}", row.title, row.status, row.detail),
            )
        });
        theme::paint_focus_ring_outset(ui, &response, rect);
        response
            .on_hover_cursor(egui::CursorIcon::PointingHand)
            .clicked()
    } else {
        false
    }
}

fn paint_elided_text(
    ui: &Ui,
    position: egui::Pos2,
    text: &str,
    font: egui::FontId,
    color: egui::Color32,
    max_width: f32,
) {
    let text = elide_text(ui, text, &font, max_width);
    ui.painter()
        .text(position, egui::Align2::LEFT_TOP, text, font, color);
}

fn elide_text(ui: &Ui, text: &str, font: &egui::FontId, max_width: f32) -> String {
    if max_width <= 0.0 {
        return String::new();
    }
    if ui
        .painter()
        .layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::WHITE)
        .size()
        .x
        <= max_width
    {
        return text.to_owned();
    }
    let characters = text.chars().collect::<Vec<_>>();
    let mut low = 0;
    let mut high = characters.len();
    while low < high {
        let midpoint = (low + high).div_ceil(2);
        let candidate = characters[..midpoint]
            .iter()
            .copied()
            .chain(std::iter::once('…'))
            .collect::<String>();
        let width = ui
            .painter()
            .layout_no_wrap(candidate, font.clone(), egui::Color32::WHITE)
            .size()
            .x;
        if width <= max_width {
            low = midpoint;
        } else {
            high = midpoint - 1;
        }
    }
    characters[..low]
        .iter()
        .copied()
        .chain(std::iter::once('…'))
        .collect()
}

fn format_duration(seconds: f64) -> String {
    if !seconds.is_finite() || seconds < 0.0 {
        return "duration unavailable".to_owned();
    }
    if seconds >= 60.0 {
        let minutes = (seconds / 60.0).floor() as u64;
        let remaining = seconds - minutes as f64 * 60.0;
        format!("{minutes} min {remaining:.1} s")
    } else {
        format!("{seconds:.3} s")
    }
}

fn run_title(run_number: u64, label: &str) -> String {
    let sequence_prefix = format!("Run {run_number}");
    if label.starts_with(&sequence_prefix) {
        label.to_owned()
    } else {
        format!("{sequence_prefix} · {label}")
    }
}

fn configuration(ui: &mut Ui, app: &mut RSpiceApp) {
    let eyebrow = format!(
        "TESTBENCH CONFIGURATION · {}",
        app.state.workspace.project.display_name()
    );
    header_with_actions(
        ui,
        &eyebrow,
        "Hierarchy and view binding",
        "Define the exact design, view, model, environment and netlisting contracts used by every run.",
        |ui, app| {
            if Button::new("Validate configuration")
                .accent()
                .show(ui)
                .clicked()
            {
                Command::PreflightChecks.execute(app);
            }
            if Button::new("← Project").show(ui).clicked() {
                app.state.workbench.project_page = ProjectPage::Dashboard;
            }
        },
        app,
    );
    hierarchy_binding_table(ui, app);
    ui.add_space(PROJECT_GRID_GAP);
    configuration_policy_cards(ui, app);
}

fn hierarchy_binding_table(ui: &mut Ui, app: &RSpiceApp) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (header, _) = ui.allocate_exact_size(vec2(width, 37.0), Sense::hover());
    ui.painter().hline(
        header.x_range(),
        header.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
    ui.painter().hline(
        header.x_range(),
        header.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter().text(
        header.left_center() + vec2(11.0, 0.0),
        Align2::LEFT_CENTER,
        "Hierarchy binding table",
        theme::sans(tokens::FS_0, FontWeight::SemiBold),
        t.color.text,
    );
    let resolved = app.state.workspace.open_views.len();
    ui.painter().text(
        header.right_center() - vec2(11.0, 0.0),
        Align2::RIGHT_CENTER,
        format!(
            "{resolved} cellview{} resolved",
            if resolved == 1 { "" } else { "s" }
        ),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        if resolved == 0 {
            t.color.warn
        } else {
            t.color.ok
        },
    );

    ScrollArea::horizontal()
        .id_salt("workbench.project.configuration.hierarchy-table")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let table_width = width.max(CONFIGURATION_TABLE_MIN_WIDTH);
            ui.set_min_width(table_width);
            configuration_table_row(
                ui,
                table_width,
                [
                    "CELL / PATTERN",
                    "PURPOSE",
                    "VIEW SEARCH ORDER",
                    "STOP VIEW",
                    "MODEL SECTION",
                    "STATUS",
                ],
                true,
                None,
            );
            if app.state.workspace.open_views.is_empty() {
                configuration_table_row(
                    ui,
                    table_width,
                    ["No open cellviews", "—", "—", "—", "—", "unresolved"],
                    false,
                    Some(t.color.warn),
                );
            } else {
                for view in &app.state.workspace.open_views {
                    let active = view.reference == app.state.workspace.active_view;
                    let reference = view.reference.display_path();
                    let purpose = if active {
                        "testbench root"
                    } else {
                        "open cellview"
                    };
                    let search_order = format!("{} → spice", view.reference.view);
                    let stop_view = if view.reference.view.eq_ignore_ascii_case("spice") {
                        "spice"
                    } else {
                        "—"
                    };
                    let status = if view.dirty { "modified" } else { "resolved" };
                    configuration_table_row(
                        ui,
                        table_width,
                        [
                            &reference,
                            purpose,
                            &search_order,
                            stop_view,
                            "inherit PVT",
                            status,
                        ],
                        false,
                        Some(if view.dirty { t.color.warn } else { t.color.ok }),
                    );
                }
            }
        });
}

fn configuration_table_row(
    ui: &mut Ui,
    width: f32,
    cells: [&str; 6],
    header: bool,
    status_color: Option<Color32>,
) {
    let t = Tokens::get(ui.ctx());
    let height = if header {
        CONFIGURATION_TABLE_HEADER_HEIGHT
    } else {
        CONFIGURATION_TABLE_ROW_HEIGHT
    };
    let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    if header {
        painter.rect_filled(rect, 0.0, t.color.bg_panel_2);
    }
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let fractions = [0.22, 0.15, 0.22, 0.12, 0.16, 0.13];
    let mut left = rect.left();
    for (index, (text, fraction)) in cells.iter().zip(fractions).enumerate() {
        let right = if index + 1 == cells.len() {
            rect.right()
        } else {
            left + rect.width() * fraction
        };
        let cell = Rect::from_min_max(
            egui::pos2(left, rect.top()),
            egui::pos2(right, rect.bottom()),
        );
        if index > 0 {
            painter.vline(left, rect.y_range(), Stroke::new(1.0, t.color.border));
        }
        painter.with_clip_rect(cell.shrink2(vec2(8.0, 0.0))).text(
            cell.left_center() + vec2(8.0, 0.0),
            Align2::LEFT_CENTER,
            *text,
            if header {
                theme::sans(tokens::FS_0, FontWeight::Medium)
            } else {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            },
            if header {
                t.color.text_faint
            } else if index == cells.len() - 1 {
                status_color.unwrap_or(t.color.text_dim)
            } else {
                t.color.text_dim
            },
        );
        left = right;
    }
}

fn configuration_policy_cards(ui: &mut Ui, app: &RSpiceApp) {
    let add_netlisting = |ui: &mut Ui| {
        card(ui, "Netlisting policy", |ui| {
            property_row(ui, "Hierarchy", "Preserve names");
            property_row(ui, "Parameter evaluation", "Strict units");
            property_row(ui, "Unbound cell", "Block execution");
            property_row(ui, "Global nets", "Explicit; node 0 canonical");
        });
    };
    let add_environment = |ui: &mut Ui| {
        card(ui, "Environment", |ui| {
            property_row(
                ui,
                "Technology",
                app.state
                    .workspace
                    .project
                    .technology
                    .as_deref()
                    .unwrap_or("Not attached"),
            );
            property_row(ui, "Supply profile", "Not configured");
            property_row(
                ui,
                "Temperature source",
                &format!(
                    "Run set · {:.3} °C",
                    app.state.sim_setup.reference_pvt.temperature_celsius
                ),
            );
            property_row(ui, "Ground policy", "Node 0 canonical");
        });
    };

    if project_header_stacks(ui.ctx().content_rect().width()) {
        add_netlisting(ui);
        ui.add_space(PROJECT_GRID_GAP);
        add_environment(ui);
        return;
    }
    let width = ((ui.available_width() - PROJECT_GRID_GAP) / 2.0).max(1.0);
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = PROJECT_GRID_GAP;
        ui.allocate_ui_with_layout(
            vec2(width, 0.0),
            Layout::top_down(Align::Min),
            add_netlisting,
        );
        ui.allocate_ui_with_layout(
            vec2(width, 0.0),
            Layout::top_down(Align::Min),
            add_environment,
        );
    });
}

fn technology(ui: &mut Ui, app: &mut RSpiceApp) {
    let technology = app
        .state
        .workspace
        .project
        .technology
        .clone()
        .unwrap_or_else(|| "No technology attached".to_owned());
    header_with_actions(
        ui,
        "TECHNOLOGY ATTACHMENT",
        "Technology and PDK contract",
        "Versioned attachment for symbols, models, sections, and qualification evidence.",
        |ui, app| {
            if Button::new("Attach technology…")
                .accent()
                .show(ui)
                .clicked()
            {
                Command::PdkSettings.execute(app);
            }
        },
        app,
    );
    card(ui, "PDK resources", |ui| {
        property_row(ui, "Attachment", &technology);
        property_row(
            ui,
            "Configured search paths",
            &app.state.pdk_config.library_paths.len().to_string(),
        );
        property_row(
            ui,
            "Discovered model files",
            &app.state.pdk_config.discovered_files.len().to_string(),
        );
        property_row(
            ui,
            "Scan diagnostics",
            &app.state.pdk_config.scan_errors.len().to_string(),
        );
        for path in &app.state.pdk_config.library_paths {
            ui.label(
                egui::RichText::new(path.display_name())
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.text_dim),
            );
        }
    });
}

fn dependencies(ui: &mut Ui, app: &mut RSpiceApp) {
    header_with_actions(
        ui,
        "DEPENDENCY MANIFEST · CONTENT ADDRESSED",
        "Project dependency graph",
        "Libraries, model files, behavioral sources, and external contracts required to reproduce the project.",
        |_ui, _app| {},
        app,
    );
    let resolved_model_sources = resolved_model_source_count(app);
    card(ui, "Locked dependencies", |ui| {
        property_row(
            ui,
            "Design libraries",
            &app.state.library_manager.library_count().to_string(),
        );
        property_row(
            ui,
            "Cells",
            &app.state.library_manager.total_cell_count().to_string(),
        );
        property_row(
            ui,
            "Cellviews",
            &app.state.library_manager.total_view_count().to_string(),
        );
        property_row(
            ui,
            "Model libraries",
            &app.state.model_library_manager.library_count().to_string(),
        );
        property_row(
            ui,
            "Device models",
            &app.state
                .model_library_manager
                .total_model_count()
                .to_string(),
        );
        property_row(
            ui,
            "Resolved model source files",
            &resolved_model_sources.to_string(),
        );
        if resolved_model_sources == 0 {
            status_dot(
                ui,
                Tokens::get(ui.ctx()).color.warn,
                "No content-addressed model dependency set is resolved",
            );
        } else {
            status_dot(
                ui,
                Tokens::get(ui.ctx()).color.ok,
                "Model source closure is content addressed",
            );
        }
        for library in app.state.library_manager.libraries_sorted() {
            ui.separator();
            property_row(
                ui,
                &library.name,
                if library.read_only {
                    "Read only"
                } else {
                    "Project writable"
                },
            );
        }
        for library in app.state.model_library_manager.libraries_sorted() {
            ui.separator();
            property_row(
                ui,
                &format!("Model · {}", library.name),
                if library.source_closure.is_empty() {
                    "No resolved source closure".to_owned()
                } else {
                    format!("{} pinned source files", library.source_closure.len())
                }
                .as_str(),
            );
        }
    });
    ui.add_space(PROJECT_GRID_GAP);
    card(ui, "Reproducibility contract", |ui| {
        property_row(ui, "Project format", "RSpice project schema 1");
        property_row(ui, "Missing dependency", "Fail closed");
        property_row(ui, "Mutable external path", "Explicit project reference");
        property_row(
            ui,
            "Project identity",
            &app.state.workspace.project.id().to_string(),
        );
    });
}

fn recovery(ui: &mut Ui, app: &mut RSpiceApp) {
    header_with_actions(
        ui,
        "PROJECT RECOVERY · NON-DESTRUCTIVE",
        "Recovery Center",
        "Review live working changes and save a durable project copy without replacing recoverable state.",
        |ui, app| {
            if Button::new("Save current project")
                .accent()
                .show(ui)
                .clicked()
            {
                Command::Save.execute(app);
            }
            if Button::new("Save project copy…").show(ui).clicked() {
                Command::SaveAs.execute(app);
            }
        },
        app,
    );
    let dirty_documents: Vec<_> = app
        .state
        .workspace
        .open_views
        .iter()
        .filter(|view| view.dirty)
        .map(|view| view.reference.display_path())
        .collect();
    card(ui, "Recoverable working state", |ui| {
        property_row(
            ui,
            "Open documents",
            &app.state.workspace.open_views.len().to_string(),
        );
        property_row(ui, "Modified documents", &dirty_documents.len().to_string());
        property_row(
            ui,
            "Active schematic",
            if app.state.schematic.is_dirty {
                "Modified"
            } else {
                "Current"
            },
        );
        property_row(
            ui,
            "Manual netlist",
            if app.state.workspace.netlist_source_dirty {
                "Modified"
            } else {
                "Current"
            },
        );
        property_row(
            ui,
            "Project path",
            &app.state
                .workspace
                .project
                .path
                .as_ref()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "Not saved yet".to_owned()),
        );
        if dirty_documents.is_empty()
            && !app.state.schematic.is_dirty
            && !app.state.workspace.netlist_source_dirty
        {
            status_dot(
                ui,
                Tokens::get(ui.ctx()).color.ok,
                "No recovery action required",
            );
        } else {
            for document in dirty_documents {
                muted(ui, &document);
            }
        }
    });
}

fn header_with_actions(
    ui: &mut Ui,
    eyebrow: &str,
    title: &str,
    description: &str,
    actions: impl FnOnce(&mut Ui, &mut RSpiceApp),
    app: &mut RSpiceApp,
) {
    let t = Tokens::get(ui.ctx());
    let shown = egui::Frame::new()
        .inner_margin(egui::Margin {
            left: 16,
            right: 16,
            top: 14,
            bottom: 12,
        })
        .show(ui, |ui| {
            ui.set_width((ui.available_width()).max(1.0));
            if project_header_stacks(ui.ctx().content_rect().width()) {
                ui.vertical(|ui| {
                    project_heading(ui, eyebrow, title, description);
                    ui.add_space(8.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        actions(ui, app);
                    });
                });
            } else {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.spacing_mut().item_spacing.x = 16.0;
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        actions(ui, app);
                    });
                    let heading_width = ui.available_width().max(1.0);
                    ui.allocate_ui_with_layout(
                        egui::vec2(heading_width, 0.0),
                        egui::Layout::top_down(egui::Align::Min),
                        |ui| project_heading(ui, eyebrow, title, description),
                    );
                });
            }
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
}

fn project_heading(ui: &mut Ui, eyebrow: &str, title: &str, description: &str) {
    let t = Tokens::get(ui.ctx());
    ui.spacing_mut().item_spacing.y = 0.0;
    ui.add(
        egui::Label::new(
            egui::RichText::new(eyebrow.to_uppercase())
                .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                .color(t.color.text_faint),
        )
        .wrap(),
    );
    ui.add_space(2.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new(title)
                .font(theme::sans(21.0, FontWeight::SemiBold))
                .color(t.color.text),
        )
        .wrap(),
    );
    ui.add_space(5.0);
    ui.add(
        egui::Label::new(
            egui::RichText::new(description)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        )
        .wrap(),
    );
}

fn project_header_stacks(viewport_width: f32) -> bool {
    viewport_width <= PROJECT_RESPONSIVE_BREAKPOINT
}

fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

fn muted(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
            .color(t.color.text_faint),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() <= 0.001, "{actual} != {expected}");
    }

    #[test]
    fn dashboard_columns_switch_at_the_exact_mockup_breakpoint() {
        let compact = DashboardLayout::resolve(PROJECT_RESPONSIVE_BREAKPOINT, 772.0);
        let wide = DashboardLayout::resolve(PROJECT_RESPONSIVE_BREAKPOINT + 0.01, 772.0);

        assert_eq!(compact.status_columns, 2);
        assert_eq!(compact.project_columns, 1);
        assert_close(compact.status_card_width, 386.0);
        assert_close(compact.project_primary_width, 772.0);
        assert_close(compact.project_history_width, 772.0);

        assert_eq!(wide.status_columns, 4);
        assert_eq!(wide.project_columns, 2);
        assert_close(wide.status_card_width, 193.0);
        assert_close(
            wide.project_primary_width + PROJECT_GRID_GAP + wide.project_history_width,
            772.0,
        );
        assert_close(wide.project_history_width, PROJECT_HISTORY_MIN_WIDTH);
        assert!(project_header_stacks(PROJECT_RESPONSIVE_BREAKPOINT));
        assert!(!project_header_stacks(PROJECT_RESPONSIVE_BREAKPOINT + 0.01));
    }

    #[test]
    fn responsive_geometry_never_requests_width_outside_the_surface() {
        for surface_width in [0.0, 20.0, 820.0, 1600.0] {
            let surface = ProjectSurfaceLayout::resolve(surface_width);
            assert!(surface.content_width >= 0.0);
            assert!(surface.horizontal_inset >= 0.0);
            assert_close(
                surface.horizontal_inset * 2.0 + surface.content_width,
                surface_width,
            );
        }

        for (viewport_width, content_width) in [
            (390.0, 342.0),
            (820.0, 772.0),
            (821.0, 773.0),
            (1440.0, 1180.0),
        ] {
            let layout = DashboardLayout::resolve(viewport_width, content_width);
            assert_close(
                layout.status_card_width * layout.status_columns as f32,
                content_width,
            );
            if layout.project_columns == 1 {
                assert_close(layout.project_primary_width, content_width);
                assert_close(layout.project_history_width, content_width);
            } else {
                assert_close(
                    layout.project_primary_width + PROJECT_GRID_GAP + layout.project_history_width,
                    content_width,
                );
            }
        }
    }

    #[test]
    fn dashboard_body_uses_the_mockup_dense_row_contract() {
        assert_close(dashboard_card_header_height(false), 37.0);
        assert_close(
            ProjectEntryLayout::resolve(1440.0, false).height,
            PROJECT_ENTRY_ROW_HEIGHT,
        );
        assert_close(RECENT_RUN_ROW_HEIGHT, 72.0);
    }

    #[test]
    fn configuration_table_uses_the_mockup_dense_geometry() {
        assert_close(CONFIGURATION_TABLE_HEADER_HEIGHT, 27.0);
        assert_close(CONFIGURATION_TABLE_ROW_HEIGHT, 28.0);
        assert_close(CONFIGURATION_TABLE_MIN_WIDTH, 540.0);
        let column_fractions = [0.22_f32, 0.15, 0.22, 0.12, 0.16, 0.13];
        assert_close(column_fractions.into_iter().sum(), 1.0);
    }

    #[test]
    fn dashboard_rows_and_headers_expand_for_responsive_touch_contracts() {
        let tablet = ProjectEntryLayout::resolve(PROJECT_RESPONSIVE_BREAKPOINT, false);
        assert_close(tablet.height, TOUCH_TARGET_HEIGHT);
        assert!(tablet.status_below_title);
        assert!(!tablet.phone);

        let phone = ProjectEntryLayout::resolve(PROJECT_PHONE_BREAKPOINT, true);
        assert_close(phone.height, PROJECT_PHONE_ENTRY_ROW_HEIGHT);
        assert!(phone.status_below_title);
        assert!(phone.phone);

        let coarse_desktop = ProjectEntryLayout::resolve(1280.0, true);
        assert_close(coarse_desktop.height, TOUCH_TARGET_HEIGHT);
        assert!(!coarse_desktop.status_below_title);
        assert_close(dashboard_card_header_height(true), TOUCH_TARGET_HEIGHT);
    }

    #[test]
    fn verification_card_requires_current_dataset_coverage() {
        assert_eq!(
            verification_card_copy(3, None, 0, None, false),
            StatusCardCopy {
                value: "0/3 covered".to_owned(),
                detail: "No active immutable dataset".to_owned(),
                ok: false,
            }
        );
        assert_eq!(
            verification_card_copy(
                3,
                Some("12345678-1234-1234-1234-123456789abc"),
                2,
                Some(98.625),
                false,
            ),
            StatusCardCopy {
                value: "98.62%".to_owned(),
                detail: "2/3 specs · dataset 12345678".to_owned(),
                ok: false,
            }
        );
        assert!(verification_card_copy(3, Some("dataset-a"), 3, Some(100.0), true).ok);
        assert!(!verification_card_copy(0, Some("dataset-a"), 0, None, true).ok);
    }

    #[test]
    fn model_dependency_card_never_calls_an_empty_set_resolved() {
        let empty = model_dependency_card_copy(0, 0);
        assert_eq!(empty.value, "0 files");
        assert_eq!(empty.detail, "No resolved model dependency set");
        assert!(!empty.ok);

        let resolved = model_dependency_card_copy(7, 0);
        assert_eq!(resolved.value, "7 files");
        assert_eq!(resolved.detail, "Resolved source closure");
        assert!(resolved.ok);

        let advisory = model_dependency_card_copy(7, 1);
        assert_eq!(advisory.detail, "Resolved closure · 1 scan diagnostics");
        assert!(!advisory.ok);
    }

    #[test]
    fn recent_run_duration_copy_is_deterministic_and_honest() {
        assert_eq!(format_duration(0.125), "0.125 s");
        assert_eq!(format_duration(88.25), "1 min 28.2 s");
        assert_eq!(format_duration(f64::NAN), "duration unavailable");
        assert_eq!(short_identity("1234567890"), "12345678");
        assert_eq!(run_title(41, "Run 41 (08:26)"), "Run 41 (08:26)");
        assert_eq!(
            run_title(41, "PVT characterization"),
            "Run 41 · PVT characterization"
        );
    }
}
