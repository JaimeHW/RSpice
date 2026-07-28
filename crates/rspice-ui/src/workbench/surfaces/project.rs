//! Project workspace surfaces from the workbench mockup.
//!
//! Every value is resolved from the live project, library, simulation, and
//! recovery state. The surface never substitutes fixture counts or synthetic
//! project history when a project has not produced that evidence yet.

mod recovery;
mod technology;

use recovery::*;
use technology::*;

use egui::{
    Align, Align2, Color32, Context, Layout, Margin, Rect, ScrollArea, Sense, Stroke, Ui, pos2,
    vec2,
};

use crate::diagnostics::ConsoleMessage;
use crate::state::{ProjectTechnologyBinding, model_library::ModelLibraryManager};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogSize, select};
use crate::workbench::RSpiceApp;

use super::super::commands::vocabulary::Command;
use super::super::design_system::{WorkbenchIcon, card, property_card, property_row, status_dot};
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
const WORKSPACE_TABLE_HEADER_HEIGHT: f32 = 37.0;
const WORKSPACE_TABLE_COLUMN_HEADER_HEIGHT: f32 = 27.0;
const WORKSPACE_TABLE_ROW_HEIGHT: f32 = 36.0;
const RECOVERY_TABLE_MIN_WIDTH: f32 = 820.0;
const TECHNOLOGY_TABLE_MIN_WIDTH: f32 = 760.0;
const RECOVERY_STATUS_HEIGHT: f32 = 68.0;
const TECHNOLOGY_METRIC_HEIGHT: f32 = 84.0;
const TECHNOLOGY_METRIC_COMPACT_HEIGHT: f32 = 70.0;
const TECHNOLOGY_SURFACE_TITLE: &str = "Model-library technology contract";
const TECHNOLOGY_SURFACE_ACTION: &str = "Attach model technology…";
const TECHNOLOGY_DIALOG_TITLE: &str = "Attach model-library technology";
const TECHNOLOGY_DIALOG_PRIMARY: &str = "Attach authenticated binding";
const TECHNOLOGY_DIALOG_CURRENT: &str = "Binding already attached";
const TECHNOLOGY_DIALOG_PENDING: &str = "Writing checkpoint…";

#[cfg(target_arch = "wasm32")]
struct BrowserTechnologyCheckpointCompletion {
    project_id: String,
    expected_revision: u64,
    binding: ProjectTechnologyBinding,
    result: Result<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointSummary, String>,
}

#[cfg(target_arch = "wasm32")]
struct BrowserRecoveryCatalogCompletion {
    project_id: String,
    result: Result<crate::workbench::lifecycle::project_checkpoint::ProjectCheckpointCatalog, String>,
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

#[derive(Debug, Clone, Copy, PartialEq)]
struct ProjectContractLayout {
    status_columns: usize,
    status_cell_width: f32,
    operation_columns: usize,
    operation_cell_width: f32,
}

impl ProjectContractLayout {
    fn resolve(content_width: f32) -> Self {
        let content_width = content_width.max(0.0);
        let status_columns = if content_width <= PROJECT_PHONE_BREAKPOINT {
            1
        } else if content_width <= PROJECT_RESPONSIVE_BREAKPOINT {
            2
        } else {
            4
        };
        let operation_columns = if content_width <= PROJECT_RESPONSIVE_BREAKPOINT {
            1
        } else {
            3
        };
        Self {
            status_columns,
            status_cell_width: content_width / status_columns as f32,
            operation_columns,
            operation_cell_width: content_width / operation_columns as f32,
        }
    }

    fn technology_metric_height(self) -> f32 {
        if self.status_columns == 4 {
            TECHNOLOGY_METRIC_HEIGHT
        } else {
            TECHNOLOGY_METRIC_COMPACT_HEIGHT
        }
    }
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
    let ctx = ui.ctx().clone();
    #[cfg(target_arch = "wasm32")]
    {
        poll_browser_technology_checkpoint(&ctx, app);
        poll_browser_recovery_completions(&ctx, app);
    }
    let t = Tokens::get(ui.ctx());
    egui::Frame::new().fill(t.color.bg_app).show(ui, |ui| {
        ScrollArea::vertical()
            .id_salt("workbench.project.surface")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let surface = ProjectSurfaceLayout::resolve(ui.available_width());
                ui.set_width(surface.content_width);
                match app.state.workbench.project_page {
                    ProjectPage::Dashboard => dashboard(ui, app),
                    ProjectPage::Configuration => configuration(ui, app),
                    ProjectPage::Technology => technology::show(ui, app),
                    ProjectPage::Dependencies => dependencies(ui, app),
                    ProjectPage::Recovery => recovery(ui, app),
                }
            });
    });
    show_technology_attachment_dialog(&ctx, app);
}

fn dashboard(ui: &mut Ui, app: &mut RSpiceApp) {
    let surface_width = ui.available_width();
    let layout = DashboardLayout::resolve(surface_width, surface_width);
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
    let responsive_width = ui.ctx().content_rect().width().min(width);
    let layout =
        ProjectEntryLayout::resolve(responsive_width, t.metrics.ctl_h >= TOUCH_TARGET_HEIGHT);
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

    let show_meta = !layout.status_below_title && responsive_width > 1120.0;
    let status_width = if responsive_width <= PROJECT_RESPONSIVE_BREAKPOINT {
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
    let resolution = app.state.workspace.resolve_hierarchy_with_active(
        &app.state.library_manager,
        &app.state.workspace.active_view,
        &app.state.schematic,
    );
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
    let resolved = resolution.resolved_instances;
    let total = resolution.total_instances;
    let resolution_label = if resolution.is_valid() {
        format!(
            "{resolved} instance{} resolved",
            if resolved == 1 { "" } else { "s" }
        )
    } else {
        format!("{resolved} / {total} instances resolved")
    };
    ui.painter().text(
        header.right_center() - vec2(11.0, 0.0),
        Align2::RIGHT_CENTER,
        resolution_label,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        if resolution.is_valid() {
            t.color.ok
        } else {
            t.color.warn
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
            if resolution.bindings.is_empty() {
                configuration_table_row(
                    ui,
                    table_width,
                    [
                        "No hierarchy root",
                        "testbench root",
                        "schematic → spice",
                        "—",
                        "inherit PVT",
                        "unresolved",
                    ],
                    false,
                    Some(t.color.warn),
                );
            } else {
                for binding in &resolution.bindings {
                    let mut reference =
                        format!("{}/{}", binding.reference.library, binding.reference.cell);
                    if binding.instance_count > 1 {
                        reference.push_str(&format!(" ×{}", binding.instance_count));
                    }
                    let search_order = binding.view_search_order.join(" → ");
                    let stop_view = binding.stop_view.as_deref().unwrap_or("—");
                    let status_color =
                        if !binding.status.is_resolved() || binding.status.is_modified() {
                            t.color.warn
                        } else {
                            t.color.ok
                        };
                    let response = configuration_table_row(
                        ui,
                        table_width,
                        [
                            &reference,
                            &binding.purpose,
                            &search_order,
                            stop_view,
                            &binding.model_section,
                            binding.status.label(),
                        ],
                        false,
                        Some(status_color),
                    );
                    if let Some(diagnostic) = binding.diagnostic.as_deref() {
                        response.on_hover_text(diagnostic);
                    }
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
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let height = if header {
        CONFIGURATION_TABLE_HEADER_HEIGHT
    } else {
        CONFIGURATION_TABLE_ROW_HEIGHT
    };
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let accessible_label = if header {
        format!("Hierarchy table columns: {}", cells.join(", "))
    } else {
        format!(
            "Hierarchy binding: cell {}; purpose {}; view search order {}; stop view {}; model section {}; status {}",
            cells[0], cells[1], cells[2], cells[3], cells[4], cells[5]
        )
    };
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            accessible_label.clone(),
        )
    });
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
    response
}

fn configuration_policy_cards(ui: &mut Ui, app: &RSpiceApp) {
    let add_netlisting = |ui: &mut Ui| {
        property_card(ui, "Netlisting policy", |ui| {
            property_row(ui, "Hierarchy", "Preserve names");
            property_row(ui, "Parameter evaluation", "Strict units");
            property_row(ui, "Unbound cell", "Block execution");
            property_row(ui, "Global nets", "Explicit; node 0 canonical");
        });
    };
    let add_environment = |ui: &mut Ui| {
        property_card(ui, "Environment", |ui| {
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

    if project_header_stacks(ui.available_width()) {
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

struct WorkspaceBandSpec {
    label: &'static str,
    value: String,
    detail: String,
    value_color: Color32,
}

fn workspace_status_band(
    ui: &mut Ui,
    id: &'static str,
    specs: &[WorkspaceBandSpec; 4],
    layout: ProjectContractLayout,
    cell_height: f32,
    metric: bool,
) {
    let t = Tokens::get(ui.ctx());
    let shown = egui::Grid::new(id)
        .num_columns(layout.status_columns)
        .spacing(vec2(0.0, 0.0))
        .show(ui, |ui| {
            for (index, spec) in specs.iter().enumerate() {
                ui.allocate_ui_with_layout(
                    vec2(layout.status_cell_width, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| {
                        ui.set_width(layout.status_cell_width);
                        workspace_status_cell(
                            ui,
                            spec,
                            cell_height,
                            metric,
                            (index + 1) % layout.status_columns != 0,
                            index + layout.status_columns < specs.len(),
                        );
                    },
                );
                if (index + 1) % layout.status_columns == 0 {
                    ui.end_row();
                }
            }
        });
    ui.painter().hline(
        shown.response.rect.x_range(),
        shown.response.rect.bottom(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

fn workspace_status_cell(
    ui: &mut Ui,
    spec: &WorkspaceBandSpec,
    height: f32,
    metric: bool,
    right_divider: bool,
    bottom_divider: bool,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    if right_divider {
        ui.painter().vline(
            rect.right(),
            rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
    }
    if bottom_divider {
        ui.painter().hline(
            rect.x_range(),
            rect.bottom(),
            Stroke::new(1.0, t.color.border),
        );
    }
    let content = rect.shrink2(vec2(12.0, 0.0));
    let painter = ui
        .painter()
        .with_clip_rect(content.intersect(ui.clip_rect()));
    let compact_metric = metric && height <= TECHNOLOGY_METRIC_COMPACT_HEIGHT;
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let value_font = theme::mono(
        if metric { 19.0 } else { 13.0 },
        if metric {
            FontWeight::Medium
        } else {
            FontWeight::SemiBold
        },
    );
    let detail_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let label = elide_text(ui, spec.label, &label_font, content.width());
    let value = elide_text(ui, &spec.value, &value_font, content.width());
    let detail = elide_text(ui, &spec.detail, &detail_font, content.width());
    painter.text(
        pos2(
            content.left(),
            rect.top()
                + if compact_metric {
                    7.0
                } else if metric {
                    11.0
                } else {
                    9.0
                },
        ),
        Align2::LEFT_TOP,
        label,
        label_font,
        t.color.text_dim,
    );
    painter.text(
        pos2(
            content.left(),
            rect.top()
                + if compact_metric {
                    25.0
                } else if metric {
                    32.0
                } else {
                    26.0
                },
        ),
        Align2::LEFT_TOP,
        value,
        value_font,
        spec.value_color,
    );
    painter.text(
        pos2(
            content.left(),
            rect.top()
                + if compact_metric {
                    51.0
                } else if metric {
                    60.0
                } else {
                    47.0
                },
        ),
        Align2::LEFT_TOP,
        detail,
        detail_font,
        t.color.text_faint,
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!("{}: {}. {}", spec.label, spec.value, spec.detail),
        )
    });
}

fn workspace_table_panel_header(ui: &mut Ui, title: &str, meta: &str, meta_color: Color32) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let (rect, _) =
        ui.allocate_exact_size(vec2(width, WORKSPACE_TABLE_HEADER_HEIGHT), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
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
        t.color.text,
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
    let t = Tokens::get(ui.ctx());
    let height = if header {
        WORKSPACE_TABLE_COLUMN_HEADER_HEIGHT
    } else {
        WORKSPACE_TABLE_ROW_HEIGHT
    };
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    let painter = ui.painter().with_clip_rect(rect.intersect(ui.clip_rect()));
    if header {
        painter.rect_filled(rect, 0.0, t.color.bg_panel_2);
    }
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
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
            painter.vline(left, rect.y_range(), Stroke::new(1.0, t.color.border));
        }
        let text_color = colored_cells
            .iter()
            .find_map(|(column, color)| (*column == index).then_some(*color))
            .unwrap_or(if header {
                t.color.text_faint
            } else {
                t.color.text_dim
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
    (response, cell_rects)
}

fn workspace_empty_table_row(ui: &mut Ui, width: f32, text: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(width, WORKSPACE_TABLE_ROW_HEIGHT), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let text_rect = rect.shrink2(vec2(10.0, 0.0));
    let visible_text = elide_text(ui, text, &font, text_rect.width());
    ui.painter().with_clip_rect(text_rect).text(
        rect.left_center() + vec2(10.0, 0.0),
        Align2::LEFT_CENTER,
        visible_text,
        font,
        t.color.text_faint,
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
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
    property_card(ui, "Reproducibility contract", |ui| {
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
            if project_header_stacks(ui.available_width()) {
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
    fn recovery_and_technology_use_local_four_two_one_status_geometry() {
        let wide = ProjectContractLayout::resolve(PROJECT_RESPONSIVE_BREAKPOINT + 180.0);
        assert_eq!(wide.status_columns, 4);
        assert_eq!(wide.operation_columns, 3);
        assert_close(wide.status_cell_width, 250.0);
        assert_close(wide.operation_cell_width, 1000.0 / 3.0);
        assert_close(wide.technology_metric_height(), TECHNOLOGY_METRIC_HEIGHT);

        let tablet = ProjectContractLayout::resolve(PROJECT_RESPONSIVE_BREAKPOINT);
        assert_eq!(tablet.status_columns, 2);
        assert_eq!(tablet.operation_columns, 1);
        assert_close(tablet.status_cell_width, 410.0);
        assert_close(tablet.operation_cell_width, 820.0);
        assert_close(
            tablet.technology_metric_height(),
            TECHNOLOGY_METRIC_COMPACT_HEIGHT,
        );

        let phone = ProjectContractLayout::resolve(PROJECT_PHONE_BREAKPOINT);
        assert_eq!(phone.status_columns, 1);
        assert_eq!(phone.operation_columns, 1);
        assert_close(phone.status_cell_width, 560.0);
        assert_close(phone.operation_cell_width, 560.0);

        let above_phone = ProjectContractLayout::resolve(PROJECT_PHONE_BREAKPOINT + 0.01);
        assert_eq!(above_phone.status_columns, 2);
    }

    #[test]
    fn recovery_and_technology_tables_preserve_complete_horizontal_geometry() {
        assert_close(WORKSPACE_TABLE_COLUMN_HEADER_HEIGHT, 27.0);
        assert_close(WORKSPACE_TABLE_ROW_HEIGHT, 36.0);
        assert!(RECOVERY_TABLE_MIN_WIDTH > PROJECT_PHONE_BREAKPOINT);
        assert!(TECHNOLOGY_TABLE_MIN_WIDTH > PROJECT_PHONE_BREAKPOINT);
        assert_close(
            [0.14_f32, 0.24, 0.12, 0.13, 0.13, 0.24].into_iter().sum(),
            1.0,
        );
        assert_close([0.24_f32, 0.25, 0.17, 0.21, 0.13].into_iter().sum(), 1.0);
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
    fn technology_attachment_copy_never_claims_pdk_qualification() {
        assert_eq!(
            TECHNOLOGY_SURFACE_TITLE,
            "Model-library technology contract"
        );
        assert_eq!(TECHNOLOGY_SURFACE_ACTION, "Attach model technology…");
        assert_eq!(TECHNOLOGY_DIALOG_TITLE, "Attach model-library technology");
        assert_eq!(TECHNOLOGY_DIALOG_PRIMARY, "Attach authenticated binding");
        for copy in [
            TECHNOLOGY_SURFACE_TITLE,
            TECHNOLOGY_SURFACE_ACTION,
            TECHNOLOGY_DIALOG_TITLE,
            TECHNOLOGY_DIALOG_PRIMARY,
            TECHNOLOGY_DIALOG_CURRENT,
            TECHNOLOGY_DIALOG_PENDING,
        ] {
            let copy = copy.to_ascii_lowercase();
            assert!(!copy.contains("pdk"));
            assert!(!copy.contains("qualified"));
        }
    }

    #[test]
    fn technology_resource_table_reports_absence_without_inventing_resources() {
        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager.clear();
        app.state.pdk_config.library_paths.clear();
        app.state.pdk_config.discovered_files.clear();
        app.state.pdk_config.scan_errors.clear();

        let rows = technology_resource_rows(&app);

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].resource, "Project technology binding");
        assert_eq!(rows[0].status, "not executable");
        assert!(!rows[0].healthy);
        assert_eq!(rows[1].resource, "Configured model search catalog");
        assert_eq!(rows[1].status, "empty");
        assert!(!rows[1].healthy);
        assert!(rows.iter().all(|row| {
            let copy = format!("{} {}", row.resource, row.status).to_ascii_lowercase();
            !copy.contains("qualified") && !copy.contains("rule deck")
        }));
    }

    #[test]
    fn technology_primary_reflects_selection_current_binding_and_checkpoint_state() {
        assert_eq!(
            technology_primary_state(false, false, false),
            TechnologyPrimaryState {
                label: TECHNOLOGY_DIALOG_PRIMARY,
                enabled: false,
            }
        );
        assert_eq!(
            technology_primary_state(true, false, false),
            TechnologyPrimaryState {
                label: TECHNOLOGY_DIALOG_PRIMARY,
                enabled: true,
            }
        );
        assert_eq!(
            technology_primary_state(true, false, true),
            TechnologyPrimaryState {
                label: TECHNOLOGY_DIALOG_CURRENT,
                enabled: false,
            }
        );
        assert_eq!(
            technology_primary_state(true, true, false),
            TechnologyPrimaryState {
                label: TECHNOLOGY_DIALOG_PENDING,
                enabled: false,
            }
        );
    }

    #[test]
    fn every_invalid_configured_model_library_has_an_actionable_diagnostic() {
        use sha2::{Digest as _, Sha256};

        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager.clear();
        app.state.model_library_manager.add_library(
            crate::state::model_library::ModelLibrary::new("missing-root"),
        );

        let missing_bytes_path = std::env::temp_dir().join("rspice-missing-retained-bytes.lib");
        let mut missing_bytes = crate::state::model_library::ModelLibrary::new("missing-bytes");
        missing_bytes.root_path = Some(missing_bytes_path.clone());
        missing_bytes.source_closure = vec![crate::state::model_library::ModelSourcePin {
            path: missing_bytes_path,
            digest: crate::product::ContentDigest::from_bytes([0x2a; 32]),
        }];
        app.state.model_library_manager.add_library(missing_bytes);

        let bytes = b".model nch nmos\n".to_vec();
        let valid_path = std::env::temp_dir().join("rspice-valid-catalog-model.lib");
        let digest = crate::product::ContentDigest::from_bytes(Sha256::digest(&bytes).into());
        let mut valid = crate::state::model_library::ModelLibrary::new("valid-models");
        valid.root_path = Some(valid_path.clone());
        valid.source_closure = vec![crate::state::model_library::ModelSourcePin {
            path: valid_path.clone(),
            digest,
        }];
        valid.source_contents = vec![crate::state::model_library::ModelSourceContent {
            path: valid_path,
            bytes,
        }];
        valid.add_model(crate::state::model_library::DeviceModel::new(
            "nch",
            crate::state::model_library::ModelType::Nmos,
        ));
        app.state.model_library_manager.add_library(valid);

        let catalog = technology_candidates(&app);

        assert_eq!(catalog.candidates.len(), 1);
        assert_eq!(
            catalog.candidates[0].binding.model_library(),
            "valid-models"
        );
        assert_eq!(catalog.diagnostics.len(), 2);
        assert_eq!(catalog.diagnostics[0].library_name, "missing-bytes");
        assert!(catalog.diagnostics[0].reason.starts_with(
            "technology binding does not retain exact bytes for every pinned model source"
        ));
        assert_eq!(catalog.diagnostics[1].library_name, "missing-root");
        assert!(
            catalog.diagnostics[1]
                .reason
                .starts_with("technology binding has no canonical root model source")
        );
        assert!(catalog.diagnostics.iter().all(|diagnostic| {
            diagnostic
                .reason
                .contains("Configure, refresh, or re-import")
        }));
    }

    #[test]
    fn technology_commit_rejects_model_bytes_changed_after_parse() {
        struct TempModelFile(std::path::PathBuf);
        impl Drop for TempModelFile {
            fn drop(&mut self) {
                let _ = std::fs::remove_file(&self.0);
            }
        }

        let path = std::env::temp_dir().join(format!(
            "rspice-technology-binding-{}.lib",
            uuid::Uuid::new_v4()
        ));
        std::fs::write(&path, ".model nch nmos\n").expect("write model fixture");
        let path = std::fs::canonicalize(path).expect("canonicalize model fixture");
        let _cleanup = TempModelFile(path.clone());
        let digest =
            ModelLibraryManager::calculate_source_digest(&path).expect("calculate accepted digest");
        let mut library = crate::state::model_library::ModelLibrary::new("qualified-models")
            .with_technology("Qualified analog models", "180 nm");
        library.root_path = Some(path.clone());
        library.source_closure = vec![crate::state::model_library::ModelSourcePin {
            path: path.clone(),
            digest,
        }];
        library.source_contents = vec![crate::state::model_library::ModelSourceContent {
            path: path.clone(),
            bytes: std::fs::read(&path).expect("read model fixture"),
        }];
        library.add_model(crate::state::model_library::DeviceModel::new(
            "nch",
            crate::state::model_library::ModelType::Nmos,
        ));
        let binding = ProjectTechnologyBinding::from_model_library(&library)
            .expect("library produces attachable binding");

        let mut manager = ModelLibraryManager::new();
        manager.add_library(library);
        verify_pinned_model_sources(&binding, &manager).expect("unchanged source verifies");
        std::fs::write(&path, ".model nch nmos level=1\n").expect("change model fixture");
        let error = verify_pinned_model_sources(&binding, &manager)
            .expect_err("changed source must fail closed");
        assert!(error.contains("changed after it was parsed"));
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
