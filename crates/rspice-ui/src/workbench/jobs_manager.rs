//! Jobs, targets, and retained run history manager.
//!
//! This is a truthful projection of the execution state that the Rust
//! simulator actually owns. Mockup fixtures for remote schedulers, quotas,
//! or checkpoints are never presented as live records; unavailable target
//! classes are identified explicitly instead.

use egui::{
    Align, Align2, Color32, Frame, Layout, Margin, Rect, RichText, ScrollArea, Sense, Stroke, Ui,
    WidgetInfo, WidgetType, pos2, vec2,
};
use serde_json::json;

use crate::common::{AppState, RSpiceApp, app::ConsoleMessage};
use crate::product::RunId;
use crate::state::{
    AnalysisResultSourceDomain, ExecutionTarget, SimulationRun, SimulationRunLifecycle,
    SimulationRunProvenance,
};
use crate::ui::{
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{Dialog, DialogChoice, DialogSize},
};

use super::{RouteTransitionSource, SurfaceId, SurfaceRoute, commands::Command};

const DESCRIPTION: &str = "Review active execution, retained run history, immutable run authority, qualified targets, and exact run-manifest exports.";
const TABLE_MIN_WIDTH: f32 = 800.0;
const QUEUE_COLUMN_FRACTIONS: [f32; 6] = [0.08, 0.24, 0.18, 0.18, 0.14, 0.18];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RunTone {
    Active,
    Success,
    Warning,
    Error,
    Neutral,
}

impl RunTone {
    fn color(self, t: &Tokens) -> Color32 {
        match self {
            Self::Active => t.color.accent,
            Self::Success => t.color.ok,
            Self::Warning => t.color.warn,
            Self::Error => t.color.err,
            Self::Neutral => t.color.text_dim,
        }
    }
}

#[derive(Debug, Clone)]
struct TaskRow {
    code: String,
    expansion: String,
    identity: String,
    progress: f32,
    status: String,
    tone: RunTone,
}

#[derive(Debug, Clone)]
struct RunRow {
    run_id: RunId,
    job_id: Option<String>,
    sequence: u64,
    label: String,
    scope: String,
    target: String,
    progress: f32,
    elapsed: String,
    status: String,
    tone: RunTone,
    analysis_count: usize,
    task_count: usize,
    evidence: String,
    source_revision: String,
    run_set_id: String,
    tasks: Vec<TaskRow>,
}

impl RunRow {
    fn from_run(state: &AppState, run: &SimulationRun, active: bool) -> Self {
        let executor_owned = active && !run.lifecycle.is_terminal();
        let current_analysis_progress = state.simulation.progress.clamp(0.0, 1.0) as f32;
        let (status, tone) = match run.lifecycle {
            SimulationRunLifecycle::LegacyUnknown => {
                ("Legacy status not retained".to_owned(), RunTone::Neutral)
            }
            SimulationRunLifecycle::Preparing => ("Preparing".to_owned(), RunTone::Active),
            SimulationRunLifecycle::Running => {
                let detail = state.simulation.status.trim();
                (
                    if active && !detail.is_empty() {
                        detail.to_owned()
                    } else {
                        "Running".to_owned()
                    },
                    RunTone::Active,
                )
            }
            SimulationRunLifecycle::Cancelling => ("Cancelling".to_owned(), RunTone::Warning),
            SimulationRunLifecycle::Completed => ("Complete".to_owned(), RunTone::Success),
            SimulationRunLifecycle::Failed => ("Failed".to_owned(), RunTone::Error),
            SimulationRunLifecycle::Aborted => ("Aborted".to_owned(), RunTone::Warning),
            SimulationRunLifecycle::Interrupted => ("Interrupted".to_owned(), RunTone::Warning),
        };
        let target = run
            .execution_target
            .map(|target| target.label().to_owned())
            .unwrap_or_else(|| "Target not retained (legacy run)".to_owned());
        let (scope, evidence, source_revision, run_set_id, tasks) =
            run_authority_rows(run, executor_owned, current_analysis_progress);
        // The runner reports progress for the analysis currently executing.
        // The manager reports progress for the immutable prepared batch, so
        // completed task prefixes and the active task must be folded together.
        let progress = batch_progress(&tasks);
        let task_count = tasks.len().max(run.analyses.len());
        Self {
            run_id: run.run_id,
            job_id: run.job_id.map(|id| id.to_string()),
            sequence: run.id,
            label: run.label.clone(),
            scope,
            target,
            progress,
            elapsed: if active && !run.lifecycle.is_terminal() {
                "in progress".to_owned()
            } else if run.lifecycle == SimulationRunLifecycle::LegacyUnknown {
                "not retained".to_owned()
            } else {
                format_duration(run.elapsed_time)
            },
            status,
            tone,
            analysis_count: run.analyses.len(),
            task_count,
            evidence,
            source_revision,
            run_set_id,
            tasks,
        }
    }
}

#[derive(Debug, Clone)]
struct JobsSnapshot {
    rows: Vec<RunRow>,
    selected_run_id: Option<RunId>,
    running: bool,
    cancelling: bool,
    current_target: ExecutionTarget,
}

impl JobsSnapshot {
    fn capture(state: &AppState, requested: Option<RunId>) -> Self {
        let active_run_id = state
            .simulation
            .active_execution
            .map(|execution| execution.run_id);
        let rows = state
            .simulation
            .runs
            .iter()
            .map(|run| RunRow::from_run(state, run, Some(run.run_id) == active_run_id))
            .collect::<Vec<_>>();
        let selected_run_id = requested
            .filter(|id| rows.iter().any(|row| row.run_id == *id))
            .or(active_run_id)
            .or_else(|| rows.first().map(|row| row.run_id));
        Self {
            rows,
            selected_run_id,
            running: active_run_id.is_some(),
            cancelling: active_run_id.is_some_and(|run_id| {
                state
                    .simulation
                    .run_by_stable_id(run_id)
                    .is_some_and(|run| run.lifecycle == SimulationRunLifecycle::Cancelling)
            }),
            current_target: ExecutionTarget::current(),
        }
    }

    fn selected(&self) -> Option<&RunRow> {
        self.selected_run_id
            .and_then(|id| self.rows.iter().find(|row| row.run_id == id))
    }
}

pub(crate) fn open(app: &mut RSpiceApp) {
    let route = SurfaceRoute::surface(SurfaceId::JobsManager);
    if let Err(error) = app
        .state
        .workbench
        .navigate(route, RouteTransitionSource::User)
    {
        app.state
            .push_user_message(ConsoleMessage::warning(error.to_string()));
    }
}

pub(crate) fn show(ctx: &egui::Context, app: &mut RSpiceApp) {
    if app.state.workbench.current_route().surface_id() != SurfaceId::JobsManager {
        return;
    }

    let snapshot =
        JobsSnapshot::capture(&app.state, app.state.workbench.jobs_manager.selected_run_id);
    app.state.workbench.jobs_manager.selected_run_id = snapshot.selected_run_id;
    let mut requested_selection = None;
    let mut body_scroll_offset = app.state.workbench.jobs_manager.scroll_offset;
    let run_label = if snapshot.cancelling {
        "Cancelling…"
    } else if snapshot.running {
        "Stop active run"
    } else {
        "Run active plan"
    };
    let export_enabled = snapshot.selected().is_some();
    let run_enabled = if snapshot.cancelling {
        false
    } else if snapshot.running {
        Command::StopSimulation.is_enabled(app)
    } else {
        Command::RunSimulation.is_enabled(app)
    };
    let dialog = Dialog::new("EXECUTION", "Jobs, targets & run history", run_label)
        .description(DESCRIPTION)
        .size(DialogSize::JobsManager)
        .primary_enabled(run_enabled)
        .primary_on_enter(false)
        .ghost("Close")
        .flush_body()
        .manual_body_scroll();
    let dialog = if export_enabled {
        dialog.secondary("Export run manifest")
    } else {
        dialog
    };
    let choice = dialog.show(ctx, |ui| {
        render_status_strip(ui, &snapshot);
        let output = ScrollArea::vertical()
            .id_salt("workbench.jobs-manager.body")
            .vertical_scroll_offset(body_scroll_offset)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                render_content(ui, &snapshot, &mut requested_selection);
            });
        body_scroll_offset = output.state.offset.y;
    });

    app.state.workbench.jobs_manager.scroll_offset = body_scroll_offset;
    if let Some(run_id) = requested_selection {
        app.state.workbench.jobs_manager.selected_run_id = Some(run_id);
    }
    match choice {
        DialogChoice::Primary => {
            if snapshot.running {
                Command::StopSimulation.execute(app);
            } else {
                Command::RunSimulation.execute(app);
            }
        }
        DialogChoice::Secondary if export_enabled => export_selected_manifest(app),
        DialogChoice::Ghost | DialogChoice::Cancelled => close_to_source(app),
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

fn close_to_source(app: &mut RSpiceApp) {
    if app
        .state
        .workbench
        .navigate_back(RouteTransitionSource::User)
        .is_some()
    {
        return;
    }
    let fallback = SurfaceRoute::surface(SurfaceId::from_workspace(app.state.workbench.workspace));
    if let Err(error) = app
        .state
        .workbench
        .replace_route(fallback, RouteTransitionSource::User)
    {
        app.state.push_user_message(ConsoleMessage::warning(format!(
            "Could not close Jobs, targets & run history: {error}"
        )));
    }
}

fn render_status_strip(ui: &mut Ui, snapshot: &JobsSnapshot) {
    let t = Tokens::get(ui.ctx());
    let selected = snapshot.selected();
    let entries = [
        (
            "Selected run",
            selected
                .map(|row| format!("Run {}", row.sequence))
                .unwrap_or_else(|| "No retained run".to_owned()),
            selected
                .map(|row| format!("{} tasks", row.task_count))
                .unwrap_or_else(|| "Run the active plan to create one".to_owned()),
            selected.map_or(RunTone::Neutral, |row| row.tone),
        ),
        (
            "Execution",
            selected
                .map(|row| row.status.clone())
                .unwrap_or_else(|| "Idle".to_owned()),
            selected
                .map(|row| row.elapsed.clone())
                .unwrap_or_else(|| "No active engine job".to_owned()),
            selected.map_or(RunTone::Neutral, |row| row.tone),
        ),
        (
            "Target",
            selected
                .map(|row| row.target.clone())
                .unwrap_or_else(|| snapshot.current_target.label().to_owned()),
            selected
                .map(|_| "Retained with selected run".to_owned())
                .unwrap_or_else(|| format!("{} runtime", snapshot.current_target.runtime())),
            RunTone::Success,
        ),
        (
            "Evidence",
            selected
                .map(|row| row.evidence.clone())
                .unwrap_or_else(|| "No run evidence".to_owned()),
            selected
                .map(|row| format!("source {}", row.source_revision))
                .unwrap_or_else(|| "Nothing selected".to_owned()),
            selected.map_or(RunTone::Neutral, |row| {
                if row.evidence.starts_with("Prepared") {
                    RunTone::Success
                } else {
                    RunTone::Warning
                }
            }),
        ),
    ];
    let width = ui.available_width().max(1.0);
    let columns = jobs_status_columns(width);
    let cell_width = width / columns as f32;
    for row in entries.chunks(columns) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
            for (index, (label, value, detail, tone)) in row.iter().enumerate() {
                let (rect, response) =
                    ui.allocate_exact_size(vec2(cell_width, 68.0), Sense::hover());
                response.widget_info(|| {
                    WidgetInfo::labeled(
                        WidgetType::Label,
                        true,
                        format!("{label}: {value}. {detail}"),
                    )
                });
                ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
                let content = rect.shrink2(vec2(12.0, 0.0));
                let painter = ui.painter().with_clip_rect(content);
                painter.text(
                    pos2(content.left(), rect.top() + 9.0),
                    Align2::LEFT_TOP,
                    *label,
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.text_dim,
                );
                painter.text(
                    pos2(content.left(), rect.top() + 28.0),
                    Align2::LEFT_TOP,
                    value,
                    theme::mono(17.0, FontWeight::Medium),
                    tone.color(&t),
                );
                painter.text(
                    pos2(content.left(), rect.top() + 50.0),
                    Align2::LEFT_TOP,
                    detail,
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.text_faint,
                );
                if index + 1 < row.len() {
                    ui.painter().vline(
                        rect.right(),
                        rect.y_range(),
                        Stroke::new(1.0, t.color.border),
                    );
                }
            }
        });
        ui.painter().hline(
            egui::Rangef::new(ui.min_rect().left(), ui.min_rect().left() + width),
            ui.cursor().top(),
            Stroke::new(1.0, t.color.border_strong),
        );
    }
}

fn jobs_status_columns(width: f32) -> usize {
    if width <= 820.0 { 2 } else { 4 }
}

fn batch_progress(tasks: &[TaskRow]) -> f32 {
    if tasks.is_empty() {
        0.0
    } else {
        tasks.iter().map(|task| task.progress).sum::<f32>() / tasks.len() as f32
    }
}

fn render_content(ui: &mut Ui, snapshot: &JobsSnapshot, requested_selection: &mut Option<RunId>) {
    let narrow = ui.available_width() <= 1_020.0;
    if narrow {
        render_queue_and_graph(ui, snapshot, requested_selection);
        render_inspector(ui, snapshot);
    } else {
        let width = ui.available_width();
        let left_width = ((width - 1.0) * 0.695).max(560.0);
        let mut divider_x = 0.0;
        let response = ui
            .horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.allocate_ui_with_layout(
                    vec2(left_width, 0.0),
                    Layout::top_down(Align::Min),
                    |ui| render_queue_and_graph(ui, snapshot, requested_selection),
                );
                let (divider, _) = ui.allocate_exact_size(vec2(1.0, 0.0), Sense::hover());
                divider_x = divider.center().x;
                ui.allocate_ui_with_layout(
                    vec2((width - left_width - 1.0).max(280.0), 0.0),
                    Layout::top_down(Align::Min),
                    |ui| render_inspector(ui, snapshot),
                );
            })
            .response;
        let t = Tokens::get(ui.ctx());
        ui.painter().vline(
            divider_x,
            response.rect.y_range(),
            Stroke::new(1.0, t.color.border),
        );
    }
    render_recovery(ui, snapshot);
}

fn render_queue_and_graph(
    ui: &mut Ui,
    snapshot: &JobsSnapshot,
    requested_selection: &mut Option<RunId>,
) {
    let t = Tokens::get(ui.ctx());
    ui.set_min_width(1.0);
    section_head(
        ui,
        "Execution queue",
        if snapshot.running { "1 active" } else { "idle" },
        if snapshot.running {
            t.color.accent
        } else {
            t.color.text_dim
        },
    );
    ScrollArea::horizontal()
        .id_salt("workbench.jobs-manager.queue-table")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let width = ui.available_width().max(TABLE_MIN_WIDTH);
            ui.set_min_width(width);
            table_header(
                ui,
                width,
                &[
                    "RUN",
                    "PLAN / SCOPE",
                    "TARGET",
                    "PROGRESS",
                    "DURATION",
                    "STATUS",
                ],
                &QUEUE_COLUMN_FRACTIONS,
            );
            if snapshot.rows.is_empty() {
                empty_row(
                    ui,
                    width,
                    "No retained runs. Run the active plan to create immutable execution history.",
                );
            }
            for row in &snapshot.rows {
                if run_table_row(ui, width, row, snapshot.selected_run_id == Some(row.run_id)) {
                    *requested_selection = Some(row.run_id);
                }
            }
        });

    let selected = snapshot.selected();
    section_head(
        ui,
        "Frozen execution graph",
        selected
            .map(|row| format!("{} analysis nodes", row.tasks.len()))
            .as_deref()
            .unwrap_or("no selected run"),
        t.color.text_dim,
    );
    ScrollArea::horizontal()
        .id_salt("workbench.jobs-manager.graph-table")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let width = ui.available_width().max(650.0);
            ui.set_min_width(width);
            table_header(
                ui,
                width,
                &[
                    "ANALYSIS",
                    "EXPANSION",
                    "TASK IDENTITY",
                    "PROGRESS",
                    "STATUS",
                ],
                &[0.15, 0.21, 0.34, 0.14, 0.16],
            );
            match selected {
                Some(row) if !row.tasks.is_empty() => {
                    for task in &row.tasks {
                        task_table_row(ui, width, task);
                    }
                }
                _ => empty_row(
                    ui,
                    width,
                    "No frozen task graph is available for the selected run.",
                ),
            }
        });
}

fn render_inspector(ui: &mut Ui, snapshot: &JobsSnapshot) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::same(0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            super::design_system::property_card(ui, "Execution target", |ui| {
                super::design_system::property_row_toned(ui, "Readiness", "Ready", t.color.ok);
                super::design_system::property_row(ui, "Runtime", snapshot.current_target.label());
                super::design_system::property_row(
                    ui,
                    "Parallel slots",
                    "1 qualified analysis slot",
                );
                super::design_system::property_row(
                    ui,
                    "Memory budget",
                    if cfg!(target_arch = "wasm32") {
                        "Browser managed"
                    } else {
                        "Process managed"
                    },
                );
                super::design_system::property_row(
                    ui,
                    "Failure policy",
                    "Exact per-analysis outcome",
                );
            });
            super::design_system::property_card(ui, "Selected run manifest", |ui| {
                if let Some(row) = snapshot.selected() {
                    super::design_system::property_row(
                        ui,
                        "Job ID",
                        row.job_id.as_deref().unwrap_or("Not retained (legacy run)"),
                    );
                    super::design_system::property_row(
                        ui,
                        "Stable run ID",
                        &row.run_id.to_string(),
                    );
                    super::design_system::property_row(ui, "Input revision", &row.source_revision);
                    super::design_system::property_row(
                        ui,
                        "Analyses",
                        &row.analysis_count.to_string(),
                    );
                    super::design_system::property_row(ui, "Run-set ID", &row.run_set_id);
                    super::design_system::property_row(ui, "Tasks", &row.task_count.to_string());
                    super::design_system::property_row_toned(
                        ui,
                        "Status",
                        &row.status,
                        row.tone.color(&t),
                    );
                } else {
                    super::design_system::property_row(ui, "Status", "No retained run selected");
                }
            });
            super::design_system::property_card(ui, "Execution targets", |ui| {
                target_row(
                    ui,
                    snapshot.current_target.label(),
                    "qualified for this build",
                    true,
                );
                let unavailable = match snapshot.current_target {
                    ExecutionTarget::LocalDesktop => "Browser worker",
                    ExecutionTarget::NativeMobile => "Browser worker",
                    ExecutionTarget::BrowserWorker => "Native application engine",
                };
                target_row(ui, unavailable, "not executable in this build", false);
                target_row(ui, "Remote scheduler", "not configured", false);
            });
        });
}

fn target_row(ui: &mut Ui, label: &str, status: &str, selected: bool) {
    let t = Tokens::get(ui.ctx());
    let height = if ui.ctx().input(|input| input.has_touch_screen()) {
        44.0
    } else {
        31.0
    };
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, format!("{label}: {status}")));
    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_active);
        ui.painter().rect_filled(
            Rect::from_min_max(rect.min, pos2(rect.left() + 2.0, rect.bottom())),
            0.0,
            t.color.accent,
        );
    }
    ui.painter().circle_filled(
        pos2(rect.left() + 12.0, rect.center().y),
        3.0,
        if selected {
            t.color.ok
        } else {
            t.color.text_faint
        },
    );
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let status_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let status_desired = ui
        .painter()
        .layout_no_wrap(status.to_owned(), status_font.clone(), t.color.text)
        .size()
        .x;
    let (label_rect, status_rect) = trailing_text_regions(rect, 22.0, 9.0, status_desired);
    let visible_label =
        super::design_system::elide_text(ui, label, &label_font, label_rect.width());
    let visible_status =
        super::design_system::elide_text(ui, status, &status_font, status_rect.width());
    ui.painter().with_clip_rect(label_rect).text(
        label_rect.left_center(),
        Align2::LEFT_CENTER,
        visible_label,
        label_font,
        if selected {
            t.color.text
        } else {
            t.color.text_dim
        },
    );
    ui.painter().with_clip_rect(status_rect).text(
        status_rect.right_center(),
        Align2::RIGHT_CENTER,
        visible_status,
        status_font,
        if selected {
            t.color.ok
        } else {
            t.color.text_faint
        },
    );
}

fn render_recovery(ui: &mut Ui, snapshot: &JobsSnapshot) {
    let t = Tokens::get(ui.ctx());
    section_head(
        ui,
        "Failure, interruption and recovery",
        if snapshot.cancelling {
            "cancellation requested"
        } else if snapshot.running {
            "active job is cancellable"
        } else if snapshot.rows.iter().any(|row| row.tone == RunTone::Error) {
            "failed evidence retained"
        } else if snapshot.rows.iter().any(|row| row.tone == RunTone::Warning) {
            "interrupted evidence retained"
        } else {
            "no recoverable condition"
        },
        if snapshot
            .rows
            .iter()
            .any(|row| matches!(row.tone, RunTone::Error | RunTone::Warning))
        {
            t.color.warn
        } else {
            t.color.text_dim
        },
    );
    Frame::NONE
        .fill(t.color.bg_app)
        .inner_margin(Margin::symmetric(10, 9))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            if snapshot.cancelling {
                ui.label(
                    RichText::new("Cancellation is bound to the active job and run identity. The controller is acknowledging the request; completed analysis results remain retained exactly as produced.")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.warn),
                );
            } else if snapshot.running {
                ui.label(
                    RichText::new("The active local job can be cancelled cooperatively. Completed analysis results remain attached to the run; no completed result is relabeled or discarded.")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            } else if snapshot.rows.iter().any(|row| row.tone == RunTone::Error) {
                ui.label(
                    RichText::new("Failed run evidence is retained exactly as reported. This build has no resumable checkpoint for the selected job; a new dispatch receives a new job and run identity.")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.warn),
                );
            } else if snapshot
                .rows
                .iter()
                .any(|row| row.tone == RunTone::Warning)
            {
                ui.label(
                    RichText::new("Aborted or interrupted run evidence is retained exactly as reported. This build has no resumable checkpoint for that execution; a new dispatch receives a new job and run identity.")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.warn),
                );
            } else {
                ui.label(
                    RichText::new("No disconnected scheduler, preempted job, quota rejection, partial result, or failed convergence condition is currently reported by the execution controller.")
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                );
            }
        });
}

fn section_head(ui: &mut Ui, title: &str, status: &str, tone: Color32) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 37.0), Sense::hover());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, title));
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let title_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
    let status_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let status_desired = ui
        .painter()
        .layout_no_wrap(status.to_owned(), status_font.clone(), tone)
        .size()
        .x;
    let (title_rect, status_rect) = trailing_text_regions(rect, 10.0, 10.0, status_desired);
    let visible_title =
        super::design_system::elide_text(ui, title, &title_font, title_rect.width());
    let visible_status =
        super::design_system::elide_text(ui, status, &status_font, status_rect.width());
    ui.painter().with_clip_rect(title_rect).text(
        title_rect.left_center(),
        Align2::LEFT_CENTER,
        visible_title,
        title_font,
        t.color.text,
    );
    ui.painter().with_clip_rect(status_rect).text(
        status_rect.right_center(),
        Align2::RIGHT_CENTER,
        visible_status,
        status_font,
        tone,
    );
}

fn trailing_text_regions(
    rect: Rect,
    left_inset: f32,
    right_inset: f32,
    desired_trailing_width: f32,
) -> (Rect, Rect) {
    let gap = 8.0;
    let content = Rect::from_min_max(
        pos2(rect.left() + left_inset, rect.top()),
        pos2(
            (rect.right() - right_inset).max(rect.left() + left_inset),
            rect.bottom(),
        ),
    );
    let columns_width = (content.width() - gap).max(0.0);
    let trailing_width = desired_trailing_width.max(0.0).min(columns_width * 0.48);
    let leading_width = (columns_width - trailing_width).max(0.0);
    let leading = Rect::from_min_max(
        content.min,
        pos2(content.left() + leading_width, content.bottom()),
    );
    let trailing = Rect::from_min_max(
        pos2((leading.right() + gap).min(content.right()), content.top()),
        content.max,
    );
    (leading, trailing)
}

fn table_header(ui: &mut Ui, width: f32, labels: &[&str], fractions: &[f32]) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(width, 28.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    paint_cells(ui, rect, labels, fractions, t.color.text_dim, true);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn run_table_row(ui: &mut Ui, width: f32, row: &RunRow, selected: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(width, 40.0), Sense::click());
    response.widget_info(|| {
        WidgetInfo::labeled(
            WidgetType::Button,
            true,
            format!(
                "Run {} ({}): {}, {}",
                row.sequence, row.label, row.scope, row.status
            ),
        )
    });
    let fill = if selected {
        t.color.bg_active
    } else if response.hovered() || response.has_focus() {
        t.color.bg_hover
    } else {
        t.color.bg_app
    };
    ui.painter().rect_filled(rect, 0.0, fill);
    if selected {
        ui.painter().rect_filled(
            Rect::from_min_max(rect.min, pos2(rect.left() + 2.0, rect.bottom())),
            0.0,
            t.color.accent,
        );
    }
    let progress = format!("{:.0}%", row.progress * 100.0);
    let labels = [
        format!("{}", row.sequence),
        row.scope.clone(),
        row.target.clone(),
        progress,
        row.elapsed.clone(),
        row.status.clone(),
    ];
    let refs = labels.iter().map(String::as_str).collect::<Vec<_>>();
    paint_cells(
        ui,
        rect,
        &refs,
        &QUEUE_COLUMN_FRACTIONS,
        t.color.text,
        false,
    );
    let progress_left = rect.left() + rect.width() * (0.08 + 0.24 + 0.18) + 8.0;
    let progress_width = (rect.width() * 0.18 - 16.0).max(10.0);
    paint_progress(
        ui,
        Rect::from_min_size(
            pos2(progress_left, rect.bottom() - 8.0),
            vec2(progress_width, 3.0),
        ),
        row.progress,
        row.tone.color(&t),
    );
    theme::paint_focus_ring_outset(ui, &response, rect);
    response.clicked()
}

fn task_table_row(ui: &mut Ui, width: f32, task: &TaskRow) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(width, 35.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    let progress = format!("{:.0}%", task.progress * 100.0);
    let labels = [
        task.code.as_str(),
        task.expansion.as_str(),
        task.identity.as_str(),
        progress.as_str(),
        task.status.as_str(),
    ];
    paint_cells(
        ui,
        rect,
        &labels,
        &[0.15, 0.21, 0.34, 0.14, 0.16],
        t.color.text,
        false,
    );
    let left = rect.left() + rect.width() * (0.15 + 0.21 + 0.34) + 8.0;
    paint_progress(
        ui,
        Rect::from_min_size(
            pos2(left, rect.bottom() - 7.0),
            vec2((rect.width() * 0.14 - 16.0).max(10.0), 3.0),
        ),
        task.progress,
        task.tone.color(&t),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

fn empty_row(ui: &mut Ui, width: f32, message: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(width, 42.0), Sense::hover());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, true, message));
    ui.painter().text(
        pos2(rect.left() + 10.0, rect.center().y),
        Align2::LEFT_CENTER,
        message,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}

fn paint_cells(
    ui: &Ui,
    rect: Rect,
    labels: &[&str],
    fractions: &[f32],
    color: Color32,
    header: bool,
) {
    let t = Tokens::get(ui.ctx());
    let mut x = rect.left();
    for (index, (label, fraction)) in labels.iter().zip(fractions).enumerate() {
        let cell_width = rect.width() * *fraction;
        let cell = Rect::from_min_max(pos2(x, rect.top()), pos2(x + cell_width, rect.bottom()));
        let clipped = cell.shrink2(vec2(8.0, 0.0));
        ui.painter().with_clip_rect(clipped).text(
            clipped.left_center(),
            Align2::LEFT_CENTER,
            *label,
            if header {
                theme::sans(tokens::FS_0, FontWeight::Medium)
            } else if index == 0 {
                theme::mono(tokens::FS_0, FontWeight::Medium)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Regular)
            },
            if !header && index + 1 == labels.len() {
                color
            } else if header {
                t.color.text_dim
            } else {
                color
            },
        );
        if index + 1 < labels.len() {
            ui.painter().vline(
                x + cell_width,
                rect.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        x += cell_width;
    }
}

fn paint_progress(ui: &Ui, rect: Rect, progress: f32, tone: Color32) {
    let t = Tokens::get(ui.ctx());
    ui.painter().rect_filled(rect, 0.0, t.color.border_strong);
    ui.painter().rect_filled(
        Rect::from_min_size(
            rect.min,
            vec2(rect.width() * progress.clamp(0.0, 1.0), rect.height()),
        ),
        0.0,
        tone,
    );
}

fn run_authority_rows(
    run: &SimulationRun,
    executor_owned: bool,
    run_progress: f32,
) -> (String, String, String, String, Vec<TaskRow>) {
    match run.provenance() {
        Some(SimulationRunProvenance::Prepared(receipt)) => {
            let completed = run.analyses.len();
            let tasks = receipt
                .tasks()
                .iter()
                .enumerate()
                .map(|(index, task)| {
                    let (status, progress, tone) = if let Some(result) = run.analyses.get(index) {
                        if result.success {
                            ("complete", 1.0, RunTone::Success)
                        } else {
                            ("failed", 1.0, RunTone::Error)
                        }
                    } else {
                        pending_task_outcome(
                            run.lifecycle,
                            executor_owned,
                            index == completed,
                            run_progress,
                        )
                    };
                    TaskRow {
                        code: task.result_analysis_type().short_label().to_owned(),
                        expansion: format!("{} dependencies", task.dependencies().len()),
                        identity: task.instance_id().to_string(),
                        progress,
                        status: status.to_owned(),
                        tone,
                    }
                })
                .collect();
            let scope = match receipt.source_domain() {
                AnalysisResultSourceDomain::SimulationPlan => "Simulation plan",
                AnalysisResultSourceDomain::ManualDeck => "Manual source deck",
                AnalysisResultSourceDomain::LegacyUnclassified => "Legacy source",
            }
            .to_owned();
            (
                scope,
                "Prepared receipt valid".to_owned(),
                receipt.project_revision().get().to_string(),
                receipt
                    .simulation_plan_id()
                    .map(|id| id.to_string())
                    .unwrap_or_else(|| "manual-deck".to_owned()),
                tasks,
            )
        }
        Some(SimulationRunProvenance::LegacyUnattributed) => (
            "Legacy retained run".to_owned(),
            "Legacy · no prepared receipt".to_owned(),
            "not retained".to_owned(),
            "legacy".to_owned(),
            legacy_task_rows(run),
        ),
        Some(SimulationRunProvenance::LegacyPreparedUnclassified) => (
            "Legacy prepared run".to_owned(),
            "Legacy · source domain unknown".to_owned(),
            "not retained".to_owned(),
            "legacy-prepared".to_owned(),
            legacy_task_rows(run),
        ),
        None => (
            "Transient run preparation".to_owned(),
            "Unsealed runtime state".to_owned(),
            "not sealed".to_owned(),
            "unsealed".to_owned(),
            legacy_task_rows(run),
        ),
    }
}

fn pending_task_outcome(
    lifecycle: SimulationRunLifecycle,
    executor_owned: bool,
    first_unfinished: bool,
    run_progress: f32,
) -> (&'static str, f32, RunTone) {
    use SimulationRunLifecycle as Lifecycle;

    if executor_owned && first_unfinished {
        return match lifecycle {
            Lifecycle::Preparing => ("preparing", 0.0, RunTone::Active),
            Lifecycle::Running => ("running", run_progress, RunTone::Active),
            Lifecycle::Cancelling => ("cancelling", run_progress, RunTone::Warning),
            Lifecycle::LegacyUnknown
            | Lifecycle::Completed
            | Lifecycle::Failed
            | Lifecycle::Aborted
            | Lifecycle::Interrupted => ("not run", 0.0, RunTone::Neutral),
        };
    }

    if !executor_owned
        && matches!(
            lifecycle,
            Lifecycle::Preparing | Lifecycle::Running | Lifecycle::Cancelling
        )
    {
        return if first_unfinished {
            ("detached", 0.0, RunTone::Warning)
        } else {
            ("not run", 0.0, RunTone::Neutral)
        };
    }

    match lifecycle {
        Lifecycle::Preparing | Lifecycle::Running => ("queued", 0.0, RunTone::Neutral),
        Lifecycle::Cancelling | Lifecycle::Aborted => {
            if first_unfinished {
                ("aborted", 0.0, RunTone::Warning)
            } else {
                ("not run", 0.0, RunTone::Neutral)
            }
        }
        Lifecycle::Interrupted => {
            if first_unfinished {
                ("interrupted", 0.0, RunTone::Warning)
            } else {
                ("not run", 0.0, RunTone::Neutral)
            }
        }
        Lifecycle::Failed => ("not run", 0.0, RunTone::Neutral),
        Lifecycle::Completed | Lifecycle::LegacyUnknown => ("not retained", 0.0, RunTone::Neutral),
    }
}

fn legacy_task_rows(run: &SimulationRun) -> Vec<TaskRow> {
    run.analyses
        .iter()
        .map(|analysis| TaskRow {
            code: analysis.analysis_type.short_label().to_owned(),
            expansion: "retained result".to_owned(),
            identity: format!("analysis-sequence-{}", analysis.id),
            progress: 1.0,
            status: if analysis.success {
                "complete"
            } else {
                "failed"
            }
            .to_owned(),
            tone: if analysis.success {
                RunTone::Success
            } else {
                RunTone::Error
            },
        })
        .collect()
}

fn format_duration(seconds: f64) -> String {
    if !seconds.is_finite() || seconds < 0.0 {
        return "not available".to_owned();
    }
    if seconds < 1.0 {
        return format!("{:.3} s", seconds);
    }
    if seconds < 60.0 {
        return format!("{:.2} s", seconds);
    }
    let minutes = (seconds / 60.0).floor();
    format!("{minutes:.0} min {:.1} s", seconds - minutes * 60.0)
}

fn export_selected_manifest(app: &mut RSpiceApp) {
    let selected_id = app.state.workbench.jobs_manager.selected_run_id;
    let Some(run) = selected_id.and_then(|id| app.state.simulation.run_by_stable_id(id)) else {
        app.state.push_user_message(ConsoleMessage::warning(
            "No retained run is selected for manifest export.",
        ));
        return;
    };
    let sequence = run.id;
    let payload = match serialize_manifest(run) {
        Ok(payload) => payload,
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::error(format!(
                "Run manifest export failed: {error}"
            )));
            return;
        }
    };
    let default_name = format!("rspice-run-{sequence}-manifest.json");
    let config = crate::common::export_workflow::SaveDialogConfig {
        title: "Export Run Manifest",
        default_name: &default_name,
        filter_name: "RSpice Run Manifest",
        filter_extensions: &["json"],
    };
    let io = app.export_workflow_io.as_ref();
    let Some(mut path) = (match io.show_save_dialog(config) {
        Ok(path) => path,
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::error(format!(
                "Run manifest export picker failed: {error}"
            )));
            return;
        }
    }) else {
        return;
    };
    crate::common::file_actions::ensure_file_extension(&mut path, "json");
    let result = io
        .observe_destination(&path)
        .and_then(|destination| io.write_text_file_observed(&destination, &payload));
    match result {
        Ok(()) => app.state.push_user_message(ConsoleMessage::info(
            crate::common::export_workflow::export_completion_message(
                "Run manifest",
                &path,
                None,
                io,
            ),
        )),
        Err(error) => app.state.push_user_message(ConsoleMessage::error(format!(
            "Run manifest export failed for '{}': {error}",
            path.display()
        ))),
    }
}

fn serialize_manifest(run: &SimulationRun) -> Result<String, String> {
    run.validate_provenance()?;
    let success = if matches!(
        run.lifecycle,
        SimulationRunLifecycle::Preparing
            | SimulationRunLifecycle::Running
            | SimulationRunLifecycle::Cancelling
    ) {
        serde_json::Value::Null
    } else {
        json!(run.success)
    };
    let provenance = match run.provenance() {
        Some(SimulationRunProvenance::Prepared(receipt)) => json!({
            "mode": "prepared-task-bound",
            "source_domain": match receipt.source_domain() {
                AnalysisResultSourceDomain::SimulationPlan => "simulation-plan",
                AnalysisResultSourceDomain::ManualDeck => "manual-deck",
                AnalysisResultSourceDomain::LegacyUnclassified => "legacy-unclassified",
            },
            "simulation_plan_id": receipt.simulation_plan_id().map(|id| id.to_string()),
            "project_revision": receipt.project_revision().get(),
            "prepared_snapshot_digest": receipt.prepared_snapshot_digest().to_string(),
            "source_content_digest": receipt.source_content_digest().to_string(),
            "source_check": {
                "kind": if receipt.source_check_receipt().is_schematic_drc() { "schematic-drc" } else { "manual-source-check" },
                "digest": receipt.source_check_receipt().digest().to_string(),
            },
            "tasks": receipt.tasks().iter().map(|task| json!({
                "analysis_instance_id": task.instance_id().to_string(),
                "analysis": task.result_analysis_type().display_name(),
                "analysis_kind_tag": task.analysis_kind_tag(),
                "source_revision": task.source_revision().get(),
                "dependencies": task.dependencies().iter().map(ToString::to_string).collect::<Vec<_>>(),
                "configuration_digest": task.config_digest().to_string(),
            })).collect::<Vec<_>>(),
        }),
        Some(SimulationRunProvenance::LegacyUnattributed) => json!({
            "mode": "legacy-unattributed",
            "warning": "This result predates prepared-run authority. Missing identity is not inferred."
        }),
        Some(SimulationRunProvenance::LegacyPreparedUnclassified) => json!({
            "mode": "legacy-prepared-unclassified",
            "warning": "This result predates authoritative prepared source-domain retention."
        }),
        None => json!({
            "mode": "unsealed-runtime",
            "warning": "The run has no authoritative prepared receipt and is not sign-off evidence."
        }),
    };
    let analyses = run
        .analyses
        .iter()
        .map(|analysis| {
            json!({
                "sequence": analysis.id,
                "analysis": analysis.analysis_type.display_name(),
                "label": analysis.label,
                "timestamp_epoch_seconds": analysis.timestamp,
                "success": analysis.success,
                "error": analysis.error_message,
                "waveform_count": analysis.waveforms.len(),
                "measurement_count": analysis.measurements.len(),
                "measurements": analysis.measurements.iter().map(|measurement| json!({
                    "name": measurement.name,
                    "value": measurement.value.as_ref().map(ToString::to_string),
                    "passed": measurement.passed,
                    "expected": measurement.expected.as_ref().map(ToString::to_string),
                    "tolerance": measurement.tolerance.as_ref().map(ToString::to_string),
                    "error": measurement.error,
                })).collect::<Vec<_>>(),
                "saved_output_receipt_count": analysis.saved_output_receipts.len(),
                "provenance": analysis.provenance.as_ref().map(|record| json!({
                    "source_domain": format!("{:?}", record.source_domain()),
                    "source_instance_id": record.source_instance_id().to_string(),
                    "source_revision": record.source_revision().get(),
                    "prepared_snapshot_digest": record.prepared_snapshot_digest().to_string(),
                    "dependencies": record.dependency_ids().iter().map(ToString::to_string).collect::<Vec<_>>(),
                })),
            })
        })
        .collect::<Vec<_>>();
    serde_json::to_string_pretty(&json!({
        "product": "RSpice Workbench",
        "schema": "rspice.run-manifest/1.0",
        "exported_at_epoch_seconds": crate::common::time_compat::unix_epoch().as_secs_f64(),
        "manifest": {
            "job_id": run.job_id.map(|id| id.to_string()),
            "run_id": run.run_id.to_string(),
            "dataset_id": run.dataset_id.to_string(),
            "display_sequence": run.id,
            "label": run.label,
            "started_at_epoch_seconds": run.timestamp,
            "elapsed_seconds": run.elapsed_time,
            "lifecycle": run.lifecycle,
            "success": success,
            "execution_target": run.execution_target.map(|target| json!({
                "id": target.runtime(),
                "label": target.label(),
            })),
            "provenance": provenance,
            "analyses": analyses,
        }
    }))
    .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, SimulationState};

    #[test]
    fn stale_selection_reanchors_to_the_active_stable_run() {
        let mut state = AppState::default();
        let first = state.simulation.start_run().run_id;
        state
            .simulation
            .active_run_idx
            .expect("first run is selected");
        let second = state.simulation.start_run().run_id;
        state.simulation.active_execution = state
            .simulation
            .run_by_stable_id(second)
            .and_then(SimulationRun::execution_identity);
        let snapshot = JobsSnapshot::capture(&state, Some(first));
        assert_eq!(snapshot.selected_run_id, Some(first));

        state.simulation.runs.retain(|run| run.run_id != first);
        let snapshot = JobsSnapshot::capture(&state, Some(first));
        assert_eq!(snapshot.selected_run_id, Some(second));
    }

    #[test]
    fn manifest_never_infers_missing_legacy_job_or_target_identity() {
        let mut run = SimulationRun::new(7);
        run.job_id = None;
        run.execution_target = None;
        run.lifecycle = SimulationRunLifecycle::LegacyUnknown;
        run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .expect("legacy fixture is valid");
        let manifest = serialize_manifest(&run).expect("manifest serializes");
        let value: serde_json::Value = serde_json::from_str(&manifest).unwrap();
        assert!(value["manifest"]["job_id"].is_null());
        assert!(value["manifest"]["execution_target"].is_null());
        assert_eq!(
            value["manifest"]["provenance"]["mode"],
            "legacy-unattributed"
        );
    }

    #[test]
    fn current_runs_capture_stable_job_and_execution_target() {
        let mut simulation = SimulationState::default();
        let run = simulation.start_run();
        assert!(run.job_id.is_some());
        assert_eq!(run.execution_target, Some(ExecutionTarget::current()));
    }

    #[test]
    fn in_flight_manifest_never_claims_a_successful_terminal_outcome() {
        let mut run = SimulationRun::new(8);
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .unwrap();
        run.mark_running().unwrap();

        let manifest = serialize_manifest(&run).unwrap();
        let value: serde_json::Value = serde_json::from_str(&manifest).unwrap();

        assert!(value["manifest"]["success"].is_null());
        assert_eq!(value["manifest"]["lifecycle"], "running");
    }

    #[test]
    fn status_strip_uses_the_mockup_four_to_two_column_breakpoint() {
        assert_eq!(jobs_status_columns(1_120.0), 4);
        assert_eq!(jobs_status_columns(820.0), 2);
        assert_eq!(jobs_status_columns(390.0), 2);
    }

    #[test]
    fn trailing_row_copy_has_disjoint_measured_regions() {
        let row = Rect::from_min_size(pos2(10.0, 20.0), vec2(280.0, 31.0));
        let (leading, trailing) = trailing_text_regions(row, 22.0, 9.0, 300.0);

        assert!(leading.right() <= trailing.left());
        assert!(trailing.width() <= (row.width() - 22.0 - 9.0 - 8.0) * 0.48 + f32::EPSILON);
        assert_eq!(leading.left(), row.left() + 22.0);
        assert_eq!(trailing.right(), row.right() - 9.0);
    }

    #[test]
    fn batch_progress_combines_completed_prefix_and_current_task_fraction() {
        let task = |progress| TaskRow {
            code: "TRAN".to_owned(),
            expansion: "0 dependencies".to_owned(),
            identity: "task".to_owned(),
            progress,
            status: "running".to_owned(),
            tone: RunTone::Active,
        };
        let tasks = [task(1.0), task(0.25), task(0.0), task(0.0)];
        assert!((batch_progress(&tasks) - 0.3125).abs() < f32::EPSILON);
        assert_eq!(batch_progress(&[]), 0.0);
    }

    #[test]
    fn terminal_task_suffixes_never_claim_to_be_queued() {
        assert_eq!(
            pending_task_outcome(SimulationRunLifecycle::Aborted, false, true, 0.7).0,
            "aborted"
        );
        assert_eq!(
            pending_task_outcome(SimulationRunLifecycle::Interrupted, false, true, 0.7).0,
            "interrupted"
        );
        assert_eq!(
            pending_task_outcome(SimulationRunLifecycle::Failed, false, true, 0.7).0,
            "not run"
        );
        assert_eq!(
            pending_task_outcome(SimulationRunLifecycle::Running, true, false, 0.7).0,
            "queued"
        );
    }
}
