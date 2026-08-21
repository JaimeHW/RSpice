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

use crate::diagnostics::ConsoleMessage;
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
use crate::workbench::state::JobsPlanScope;
use crate::workbench::{AppState, RSpiceApp};

use crate::workbench::{
    RouteTransitionSource, SurfaceId, SurfaceRoute, commands::vocabulary::Command,
};

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
    /// What the analysis is called. The stable identity is still what
    /// authenticates the task, so it stays on the row as hover text rather
    /// than being replaced — a person reads the name, a report reads the id.
    name: String,
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
    plan_id: Option<crate::product::SimulationPlanId>,
    campaign: Option<crate::state::SimulationCampaignMembership>,
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
        let plan_id = run
            .prepared_receipt()
            .and_then(crate::state::PreparedRunReceipt::simulation_plan_id);
        // Only the plan this run actually came from may name its tasks. The
        // active plan is not it whenever the run predates a plan switch, and
        // borrowing a name across that boundary would label a task after an
        // analysis that never produced it.
        let run_plan = plan_id.and_then(|plan_id| {
            state
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .filter(|plan| plan.id() == plan_id)
        });
        let (scope, evidence, source_revision, run_set_id, tasks) =
            run_authority_rows(run, executor_owned, current_analysis_progress, run_plan);
        let mut scope = plan_id.map_or(scope, |plan_id| {
            let name = state
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .filter(|plan| plan.id() == plan_id)
                .map(|_| state.sim_setup.active_plan_name().to_string())
                .or_else(|| {
                    state
                        .sim_setup
                        .inactive_plans()
                        .iter()
                        .find(|plan| plan.id() == plan_id)
                        .map(|plan| plan.name().to_string())
                })
                .unwrap_or_else(|| "Deleted plan".to_owned());
            format!("{name} · {plan_id}")
        });
        let campaign = run.campaign_membership().cloned();
        if let Some(membership) = &campaign {
            scope.push_str(&format!(
                "\n{} · member {}/{}",
                membership.name(),
                membership.member_index(),
                membership.member_count()
            ));
        }
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
            plan_id,
            campaign,
        }
    }
}

#[derive(Debug, Clone)]
struct CampaignRollup {
    id: crate::product::SimulationCampaignId,
    name: String,
    retained_members: usize,
    declared_members: u32,
    completed_members: usize,
    failed_members: usize,
    active_members: usize,
}

#[derive(Debug, Clone)]
struct JobsSnapshot {
    rows: Vec<RunRow>,
    campaigns: Vec<CampaignRollup>,
    selected_run_id: Option<RunId>,
    running: bool,
    cancelling: bool,
    current_target: ExecutionTarget,
}

impl JobsSnapshot {
    fn capture(state: &AppState, requested: Option<RunId>, scope: JobsPlanScope) -> Self {
        let active_run_id = state
            .simulation
            .active_execution
            .map(|execution| execution.run_id);
        let active_plan_id = state
            .sim_setup
            .stable_analysis_plan()
            .ok()
            .map(|plan| plan.id());
        let rows = state
            .simulation
            .runs
            .iter()
            .map(|run| RunRow::from_run(state, run, Some(run.run_id) == active_run_id))
            .filter(|row| match scope {
                JobsPlanScope::AllPlans => true,
                JobsPlanScope::ActivePlan => row.plan_id == active_plan_id,
                JobsPlanScope::ManualDeck => row.plan_id.is_none(),
            })
            .collect::<Vec<_>>();
        let selected_run_id = requested
            .filter(|id| rows.iter().any(|row| row.run_id == *id))
            .or_else(|| active_run_id.filter(|id| rows.iter().any(|row| row.run_id == *id)))
            .or_else(|| rows.first().map(|row| row.run_id));
        let mut campaigns = Vec::<CampaignRollup>::new();
        for row in &rows {
            let Some(membership) = &row.campaign else {
                continue;
            };
            let index = campaigns
                .iter()
                .position(|campaign| campaign.id == membership.campaign_id())
                .unwrap_or_else(|| {
                    campaigns.push(CampaignRollup {
                        id: membership.campaign_id(),
                        name: membership.name().to_owned(),
                        retained_members: 0,
                        declared_members: membership.member_count(),
                        completed_members: 0,
                        failed_members: 0,
                        active_members: 0,
                    });
                    campaigns.len() - 1
                });
            let campaign = &mut campaigns[index];
            campaign.retained_members += 1;
            match row.tone {
                RunTone::Success => campaign.completed_members += 1,
                RunTone::Error | RunTone::Warning => campaign.failed_members += 1,
                RunTone::Active => campaign.active_members += 1,
                RunTone::Neutral => {}
            }
        }
        Self {
            rows,
            campaigns,
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

pub(crate) fn open(state: &mut AppState) {
    let route = SurfaceRoute::surface(SurfaceId::JobsManager);
    if let Err(error) = state.workbench.navigate(route, RouteTransitionSource::User) {
        state.push_user_message(ConsoleMessage::warning(error.to_string()));
    }
}

pub(crate) fn show(ctx: &egui::Context, app: &mut RSpiceApp) {
    if app.state.workbench.current_route().surface_id() != SurfaceId::JobsManager {
        return;
    }

    let scope = app.state.workbench.jobs_manager.plan_scope;
    let snapshot = JobsSnapshot::capture(
        &app.state,
        app.state.workbench.jobs_manager.selected_run_id,
        scope,
    );
    app.state.workbench.jobs_manager.selected_run_id = snapshot.selected_run_id;
    let mut requested_selection = None;
    let mut requested_open = None;
    let mut requested_scope = scope;
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
        render_plan_scope(ui, scope, &mut requested_scope);
        render_status_strip(ui, &snapshot);
        let output = ScrollArea::vertical()
            .id_salt("workbench.jobs-manager.body")
            .vertical_scroll_offset(body_scroll_offset)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                render_content(ui, &snapshot, &mut requested_selection, &mut requested_open);
            });
        body_scroll_offset = output.state.offset.y;
    });

    app.state.workbench.jobs_manager.scroll_offset = body_scroll_offset;
    if requested_scope != scope {
        app.state.workbench.jobs_manager.plan_scope = requested_scope;
        app.state.workbench.jobs_manager.selected_run_id = None;
    }
    if let Some(run_id) = requested_selection {
        app.state.workbench.jobs_manager.selected_run_id = Some(run_id);
    }
    // The history is where a reader finds a run; opening one is what they came
    // to do next. Routed before the dialog's own choices so the manager closes
    // onto the dataset rather than back onto whatever was behind it.
    if let Some(run_id) = requested_open {
        open_run_in_results(app, run_id);
        return;
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
        DialogChoice::Ghost | DialogChoice::Cancelled => close_to_source(&mut app.state),
        DialogChoice::Secondary | DialogChoice::None => {}
    }
}

/// Leave the manager on the run the reader asked to see.
///
/// The stable run identity is what the history rows carry, so it is resolved
/// against the retained history here rather than assumed to still be there —
/// pruning runs while this dialog is open is legal.
fn open_run_in_results(app: &mut RSpiceApp, run_id: RunId) {
    let Some(index) = app
        .state
        .simulation
        .runs
        .iter()
        .position(|run| run.run_id == run_id)
    else {
        app.state.push_user_message(ConsoleMessage::warning(format!(
            "Run {run_id} is no longer retained, so it could not be opened in Results."
        )));
        return;
    };
    if !app.state.simulation.select_run(index) {
        app.state.push_user_message(ConsoleMessage::warning(format!(
            "Run {run_id} could not be selected, so Results was left unchanged."
        )));
        return;
    }
    close_to_source(&mut app.state);
    Command::OpenRunInResults.execute(app);
}

fn render_plan_scope(ui: &mut Ui, current: JobsPlanScope, requested: &mut JobsPlanScope) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new("History scope")
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text_dim),
                );
                for scope in JobsPlanScope::ALL {
                    if ui
                        .selectable_label(current == scope, scope.label())
                        .clicked()
                    {
                        *requested = scope;
                    }
                }
            });
        });
}

fn close_to_source(state: &mut AppState) {
    if state
        .workbench
        .navigate_back(RouteTransitionSource::User)
        .is_some()
    {
        return;
    }
    let fallback = SurfaceRoute::surface(SurfaceId::from_workspace(state.workbench.workspace));
    if let Err(error) = state
        .workbench
        .replace_route(fallback, RouteTransitionSource::User)
    {
        state.push_user_message(ConsoleMessage::warning(format!(
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

fn render_content(
    ui: &mut Ui,
    snapshot: &JobsSnapshot,
    requested_selection: &mut Option<RunId>,
    requested_open: &mut Option<RunId>,
) {
    let narrow = ui.available_width() <= 1_020.0;
    if narrow {
        render_queue_and_graph(ui, snapshot, requested_selection, requested_open);
        render_inspector(ui, snapshot, requested_open);
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
                    |ui| {
                        render_queue_and_graph(ui, snapshot, requested_selection, requested_open);
                    },
                );
                let (divider, _) = ui.allocate_exact_size(vec2(1.0, 0.0), Sense::hover());
                divider_x = divider.center().x;
                ui.allocate_ui_with_layout(
                    vec2((width - left_width - 1.0).max(280.0), 0.0),
                    Layout::top_down(Align::Min),
                    |ui| render_inspector(ui, snapshot, requested_open),
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
    requested_open: &mut Option<RunId>,
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
                match run_table_row(ui, width, row, snapshot.selected_run_id == Some(row.run_id)) {
                    Some(RunRowAction::Select) => *requested_selection = Some(row.run_id),
                    Some(RunRowAction::Open) => *requested_open = Some(row.run_id),
                    None => {}
                }
            }
        });

    if !snapshot.campaigns.is_empty() {
        section_head(
            ui,
            "Campaign roll-up",
            &format!("{} retained campaign(s)", snapshot.campaigns.len()),
            t.color.text_dim,
        );
        Frame::NONE
            .fill(t.color.bg_app)
            .inner_margin(Margin::symmetric(10, 7))
            .show(ui, |ui| {
                for campaign in &snapshot.campaigns {
                    let outcome = if campaign.active_members > 0 {
                        "running"
                    } else if campaign.retained_members < campaign.declared_members as usize {
                        "partial history"
                    } else if campaign.failed_members > 0 {
                        "completed with errors"
                    } else {
                        "complete"
                    };
                    ui.horizontal_wrapped(|ui| {
                        ui.label(
                            RichText::new(&campaign.name)
                                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                                .color(t.color.text),
                        );
                        ui.label(
                            RichText::new(campaign.id.to_string())
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_faint),
                        );
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.label(
                                RichText::new(format!(
                                    "{outcome} · {}/{} retained · {} complete · {} failed",
                                    campaign.retained_members,
                                    campaign.declared_members,
                                    campaign.completed_members,
                                    campaign.failed_members
                                ))
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(
                                    if campaign.failed_members > 0 {
                                        t.color.warn
                                    } else {
                                        t.color.ok
                                    },
                                ),
                            );
                        });
                    });
                }
            });
    }

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
                &["ANALYSIS", "EXPANSION", "ANALYSIS", "PROGRESS", "STATUS"],
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

fn render_inspector(ui: &mut Ui, snapshot: &JobsSnapshot, requested_open: &mut Option<RunId>) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::same(0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            crate::workbench::design_system::property_card(ui, "Execution target", |ui| {
                crate::workbench::design_system::property_row_toned(
                    ui,
                    "Readiness",
                    "Ready",
                    t.color.ok,
                );
                crate::workbench::design_system::property_row(
                    ui,
                    "Runtime",
                    snapshot.current_target.label(),
                );
                crate::workbench::design_system::property_row(
                    ui,
                    "Parallel slots",
                    "1 qualified analysis slot",
                );
                crate::workbench::design_system::property_row(
                    ui,
                    "Memory budget",
                    if cfg!(target_arch = "wasm32") {
                        "Browser managed"
                    } else {
                        "Process managed"
                    },
                );
                crate::workbench::design_system::property_row(
                    ui,
                    "Failure policy",
                    "Exact per-analysis outcome",
                );
            });
            crate::workbench::design_system::property_card(ui, "Selected run manifest", |ui| {
                if let Some(row) = snapshot.selected() {
                    crate::workbench::design_system::property_row(
                        ui,
                        "Job ID",
                        row.job_id.as_deref().unwrap_or("Not retained (legacy run)"),
                    );
                    crate::workbench::design_system::property_row(
                        ui,
                        "Stable run ID",
                        &row.run_id.to_string(),
                    );
                    crate::workbench::design_system::property_row(
                        ui,
                        "Input revision",
                        &row.source_revision,
                    );
                    crate::workbench::design_system::property_row(
                        ui,
                        "Analyses",
                        &row.analysis_count.to_string(),
                    );
                    crate::workbench::design_system::property_row(
                        ui,
                        "Run-set ID",
                        &row.run_set_id,
                    );
                    if let Some(campaign) = &row.campaign {
                        crate::workbench::design_system::property_row(
                            ui,
                            "Campaign",
                            campaign.name(),
                        );
                        crate::workbench::design_system::property_row(
                            ui,
                            "Campaign member",
                            &format!(
                                "{} / {} · {}",
                                campaign.member_index(),
                                campaign.member_count(),
                                campaign.campaign_id()
                            ),
                        );
                    }
                    crate::workbench::design_system::property_row(
                        ui,
                        "Tasks",
                        &row.task_count.to_string(),
                    );
                    crate::workbench::design_system::property_row_toned(
                        ui,
                        "Status",
                        &row.status,
                        row.tone.color(&t),
                    );
                    // The manifest states what the run produced; this is the
                    // one control that goes and looks at it.
                    let openable = row.analysis_count > 0;
                    let open = crate::ui::widgets::Button::new("Open in Results")
                        .enabled(openable)
                        .show(ui);
                    if openable {
                        if open
                            .on_hover_text("Activate this run's retained dataset in Results")
                            .clicked()
                        {
                            *requested_open = Some(row.run_id);
                        }
                    } else {
                        open.on_hover_text(
                            "This run retained no analysis result, so there is no dataset to open.",
                        );
                    }
                } else {
                    crate::workbench::design_system::property_row(
                        ui,
                        "Status",
                        "No retained run selected",
                    );
                }
            });
            crate::workbench::design_system::property_card(ui, "Execution targets", |ui| {
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
        crate::workbench::design_system::elide_text(ui, label, &label_font, label_rect.width());
    let visible_status =
        crate::workbench::design_system::elide_text(ui, status, &status_font, status_rect.width());
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
        crate::workbench::design_system::elide_text(ui, title, &title_font, title_rect.width());
    let visible_status =
        crate::workbench::design_system::elide_text(ui, status, &status_font, status_rect.width());
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

/// What a click on one history row asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RunRowAction {
    /// Make this the inspected run.
    Select,
    /// Leave the manager on this run's retained dataset.
    Open,
}

fn run_table_row(ui: &mut Ui, width: f32, row: &RunRow, selected: bool) -> Option<RunRowAction> {
    let t = Tokens::get(ui.ctx());
    let openable = row.analysis_count > 0;
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
    let response = response.on_hover_text(if openable {
        "Double-click to open this run's retained dataset in Results"
    } else {
        "This run retained no dataset, so there is nothing to open in Results"
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
    if openable && response.double_clicked() {
        return Some(RunRowAction::Open);
    }
    response.clicked().then_some(RunRowAction::Select)
}

fn task_table_row(ui: &mut Ui, width: f32, task: &TaskRow) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(width, 35.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_app);
    response.on_hover_text(format!("Analysis instance {}", task.identity));
    let progress = format!("{:.0}%", task.progress * 100.0);
    let labels = [
        task.code.as_str(),
        task.expansion.as_str(),
        task.name.as_str(),
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
    run_plan: Option<&crate::simulation::plan::SimulationPlan>,
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
                    // What a completed task was called is a fact the result
                    // recorded, and it already composes derivation and run-set
                    // point into the wording. A task still pending has no such
                    // record, so its name comes from the plan that queued it,
                    // and a task neither retained nor still in the plan falls
                    // back to the kind it will produce.
                    let name = run
                        .analyses
                        .get(index)
                        .map(|result| result.label.clone())
                        .or_else(|| {
                            run_plan
                                .and_then(|plan| plan.instance(task.instance_id()))
                                .map(|instance| instance.display_name().to_owned())
                        })
                        .unwrap_or_else(|| task.result_analysis_type().display_name().to_owned());
                    TaskRow {
                        code: task.result_analysis_type().short_label().to_owned(),
                        expansion: format!("{} dependencies", task.dependencies().len()),
                        name,
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
            name: analysis.label.clone(),
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
    let config = crate::workbench::workflows::export_workflow::SaveDialogConfig {
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
    crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, "json");
    let result = io
        .observe_destination(&path)
        .and_then(|destination| io.write_text_file_observed(&destination, &payload));
    match result {
        Ok(()) => app.state.push_user_message(ConsoleMessage::info(
            crate::workbench::workflows::export_workflow::export_completion_message(
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
    let campaign = run.campaign_membership().map(|membership| {
        json!({
            "campaign_id": membership.campaign_id().to_string(),
            "name": membership.name(),
            "member_index": membership.member_index(),
            "member_count": membership.member_count(),
        })
    });
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
        "exported_at_epoch_seconds": crate::time_compat::unix_epoch().as_secs_f64(),
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
            "campaign": campaign,
            "provenance": provenance,
            "analyses": analyses,
        }
    }))
    .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisResultSourceDomain, AnalysisType, PreparedRunReceipt,
        PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationState,
    };

    fn plan_receipt(plan_id: crate::product::SimulationPlanId, byte: u8) -> PreparedRunReceipt {
        PreparedRunReceipt::new(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(plan_id),
            crate::product::ObjectRevision::INITIAL,
            crate::product::ContentDigest::from_bytes([byte; 32]),
            crate::product::ContentDigest::from_bytes([byte.wrapping_add(1); 32]),
            PreparedSourceCheckReceipt::SchematicDrc(crate::product::ContentDigest::from_bytes(
                [byte.wrapping_add(2); 32],
            )),
            vec![
                PreparedRunTaskReceipt::new(
                    crate::product::AnalysisInstanceId::new(),
                    crate::product::ObjectRevision::INITIAL,
                    Vec::new(),
                    0,
                    crate::product::ContentDigest::from_bytes([byte.wrapping_add(3); 32]),
                )
                .expect("task receipt"),
            ],
        )
        .expect("plan receipt")
    }

    /// The history is where a run is found; opening one is what a reader came
    /// to do next. The stable identity is re-resolved at that moment, because
    /// history can be pruned while this dialog is open.
    #[test]
    fn opening_a_history_row_lands_on_that_run_and_refuses_a_pruned_one() {
        let mut app = RSpiceApp::test_instance();
        app.state.project_lifecycle.project_open = true;
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Simulate);
        let first = app.state.simulation.start_run().run_id;
        app.state.simulation.runs[0]
            .analyses
            .push(AnalysisResult::new(
                1,
                AnalysisType::Transient,
                "retained TRAN",
            ));
        let second = app.state.simulation.start_run().run_id;
        assert_eq!(
            app.state.simulation.active_run().map(|run| run.id),
            Some(2),
            "the newest run is selected before the hop"
        );

        open_run_in_results(&mut app, first);
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Results
        );
        assert_eq!(
            app.state.simulation.active_run().map(|run| run.id),
            Some(1),
            "the row's own run is what the hop carried, not the active one"
        );

        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Simulate);
        app.state.simulation.runs.retain(|run| run.run_id != second);
        let before = app.state.log_buffer.revision();
        open_run_in_results(&mut app, second);
        assert_eq!(
            app.state.workbench.workspace,
            crate::workbench::state::Workspace::Simulate,
            "a pruned run must not move the reader"
        );
        assert!(app.state.log_buffer.revision() > before);
    }

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
        let snapshot = JobsSnapshot::capture(&state, Some(first), JobsPlanScope::AllPlans);
        assert_eq!(snapshot.selected_run_id, Some(first));

        state.simulation.runs.retain(|run| run.run_id != first);
        let snapshot = JobsSnapshot::capture(&state, Some(first), JobsPlanScope::AllPlans);
        assert_eq!(snapshot.selected_run_id, Some(second));
    }

    #[test]
    fn history_scope_never_mixes_active_plan_inactive_plan_and_manual_decks() {
        let mut state = AppState::default();
        let inactive_plan_id = state.sim_setup.stable_analysis_plan().unwrap().id();
        let active_plan_id = state
            .sim_setup
            .create_plan("Campaign member")
            .expect("create second plan");

        let campaign_id = crate::product::SimulationCampaignId::new();
        let inactive_run = {
            let run = state
                .simulation
                .start_prepared_run(plan_receipt(inactive_plan_id, 1));
            run.set_campaign_membership(
                crate::state::SimulationCampaignMembership::new(
                    campaign_id,
                    "Two-plan campaign",
                    1,
                    2,
                )
                .unwrap(),
            )
            .unwrap();
            run.run_id
        };
        let active_run = {
            let run = state
                .simulation
                .start_prepared_run(plan_receipt(active_plan_id, 11));
            run.set_campaign_membership(
                crate::state::SimulationCampaignMembership::new(
                    campaign_id,
                    "Two-plan campaign",
                    2,
                    2,
                )
                .unwrap(),
            )
            .unwrap();
            run.run_id
        };
        let manual_run = state.simulation.start_run().run_id;

        let active = JobsSnapshot::capture(&state, None, JobsPlanScope::ActivePlan);
        assert_eq!(active.rows.len(), 1);
        assert_eq!(active.rows[0].run_id, active_run);
        assert_eq!(active.selected_run_id, Some(active_run));

        let manual = JobsSnapshot::capture(&state, None, JobsPlanScope::ManualDeck);
        assert_eq!(manual.rows.len(), 1);
        assert_eq!(manual.rows[0].run_id, manual_run);

        let all = JobsSnapshot::capture(&state, None, JobsPlanScope::AllPlans);
        assert_eq!(all.rows.len(), 3);
        assert!(all.rows.iter().any(|row| row.run_id == inactive_run));
        assert_eq!(all.campaigns.len(), 1);
        assert_eq!(all.campaigns[0].retained_members, 2);
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
        let campaign_id = crate::product::SimulationCampaignId::new();
        run.set_campaign_membership(
            crate::state::SimulationCampaignMembership::new(
                campaign_id,
                "Nightly characterization",
                1,
                2,
            )
            .unwrap(),
        )
        .unwrap();
        let manifest = serialize_manifest(&run).expect("manifest serializes");
        let value: serde_json::Value = serde_json::from_str(&manifest).unwrap();
        assert!(value["manifest"]["job_id"].is_null());
        assert!(value["manifest"]["execution_target"].is_null());
        assert_eq!(
            value["manifest"]["provenance"]["mode"],
            "legacy-unattributed"
        );
        assert_eq!(
            value["manifest"]["campaign"]["campaign_id"],
            campaign_id.to_string()
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
            name: "Startup transient".to_owned(),
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
