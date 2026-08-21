//! What the working plan costs, per analysis and in total.
//!
//! Two numbers multiply per analysis instance, and both are per-instance rather
//! than per-plan. The *rate* is how many tasks one point of that analysis costs
//! — one for most kinds, plus a family assembly for the two that own their own
//! point declaration, plus a spectrum for a PSS that retains harmonics. The
//! *participation* is how many points the instance declared itself at, which is
//! where a nominal-only transient stops costing fifteen tasks.
//!
//! This module is the one owner of that arithmetic. The plan's scalar task
//! count is a projection of the table rather than a second derivation of it:
//! [`super::page_runset::exact_plan_task_count`] sums [`PlanWorkload::rows`],
//! so the per-analysis rows an operator reads and the queue a dispatch
//! authorizes cannot come apart. Before the table existed the scalar was
//! computed inline and there was nowhere to ask *which* analysis the tasks
//! belonged to; a per-analysis answer derived separately would have been a
//! second opinion about the same queue.
//!
//! Participation is not decided here either. Every row resolves through
//! [`super::participation::PlanParticipation`], which is itself a report of
//! what [`crate::simulation::run_set::participating_point_keys`] returned — the
//! same resolver the prepared expansion mints tasks from. So a row that says an
//! analysis visits four points names four points the queue actually contains.
//!
//! Duration is the same fact stated in time. It is the task count priced at the
//! run set's per-task budget through
//! [`crate::simulation::run_set::modelled_cost_ms`], and every surface that
//! promises a duration calls [`modelled_duration`] rather than multiplying
//! locally. That is what makes "tasks" and "how long" one claim instead of two.

use crate::product::AnalysisInstanceId;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::simulation::run_set::{self, AnalysisRunAt};
use crate::workbench::{AppState, RSpiceApp};

use super::page_kit::Tone;
use super::participation::PlanParticipation;

/// One enabled analysis instance, and what it contributes to the queue.
pub(super) struct AnalysisWorkloadRow {
    pub(super) id: AnalysisInstanceId,
    /// What the plan calls this instance. Two analyses of one kind narrowed to
    /// different points are exactly the pair a cost table has to tell apart, so
    /// the row is named rather than coded.
    pub(super) display_name: String,
    pub(super) kind: AnalysisKind,
    /// Tasks one point of this analysis costs.
    pub(super) tasks_per_point: usize,
    /// Why the rate is more than one task per point, when it is. `None` is the
    /// ordinary case and the table leaves the cell unannotated rather than
    /// writing "1 task per point" thirty times.
    pub(super) rate_note: Option<&'static str>,
    /// Points this instance is executed at.
    pub(super) points: usize,
    /// The participation as declared, which is what the "At" cell states. The
    /// resolved count alone cannot distinguish an instance scoped to every
    /// point from one whose selection happens to name them all.
    pub(super) run_at: AnalysisRunAt,
    /// Whether the participation refused to resolve against the declared space.
    /// A refused instance is still priced at the whole matrix — the rule
    /// [`PlanParticipation::point_count_for`] states — because pricing it at
    /// zero is how a budget silently shrinks.
    pub(super) unresolved: bool,
}

impl AnalysisWorkloadRow {
    /// Tasks this instance contributes: rate × participation.
    pub(super) const fn tasks(&self) -> usize {
        // Both factors are bounded by the run set's task budget, which has
        // already refused anything that could overflow here.
        self.tasks_per_point.saturating_mul(self.points)
    }

    /// The rate cell: `1` for the ordinary kind, `2 ×` where a kind assembles.
    pub(super) fn rate_cell(&self) -> String {
        if self.tasks_per_point == 1 {
            "1".to_owned()
        } else {
            format!("{} \u{d7}", self.tasks_per_point)
        }
    }

    /// The participation cell: `all`, `nominal`, `4/15`.
    ///
    /// Deliberately the spelling [`super::participation::InstanceParticipation::stat`]
    /// uses on the Run Set page's "At" column, because a reader moving between
    /// the two tables is reading one fact.
    pub(super) fn at_cell(&self, matrix: usize) -> String {
        if self.unresolved {
            return "unresolved".to_owned();
        }
        match self.run_at {
            AnalysisRunAt::AllPoints => format!("{} pts", self.points),
            AnalysisRunAt::NominalPoint => "nominal".to_owned(),
            AnalysisRunAt::SelectedPoints(_) => format!("{}/{matrix}", self.points),
        }
    }
}

/// The working plan's queue, analysis by analysis.
pub(super) struct PlanWorkload {
    pub(super) rows: Vec<AnalysisWorkloadRow>,
    /// Every point the declared space resolves to, which is what a partial
    /// participation is a fraction *of*.
    pub(super) matrix_points: usize,
    /// The run set's modelled cost of one task, in milliseconds.
    pub(super) cost_per_task_ms: u64,
}

impl PlanWorkload {
    /// Price the working plan over its declared space.
    ///
    /// Fails closed: a draft whose workload cannot be read is an error rather
    /// than a row priced at one, because a plan that silently under-counts is
    /// exactly the budget an operator would approve by mistake.
    pub(super) fn resolve(app: &RSpiceApp) -> Result<Self, String> {
        let global_axes_active = app
            .state
            .sim_setup
            .run_set
            .enabled_dimensions()
            .next()
            .is_some();
        let participation = PlanParticipation::resolve(&app.state);
        // Where the declaration does not expand exactly — an adaptive
        // composition proposes its points from feedback — there is nothing to
        // narrow against, so every analysis is priced at the whole declared
        // space, which is what the validator's own forecast counts.
        let all_points = if global_axes_active {
            run_set::validate(
                &app.state.sim_setup.run_set,
                app.state.sim_setup.enabled_analysis_instance_count(),
            )
            .forecast
            .point_count
        } else {
            1
        };
        let exact = global_axes_active && !participation.point_keys.is_empty();

        let mut rows = Vec::new();
        for instance in app.state.sim_setup.enabled_analysis_instances() {
            let (tasks_per_point, rate_note) =
                task_rate(instance.draft(), app, global_axes_active)?;
            let points = if exact {
                participation.point_count_for(instance.id())
            } else {
                all_points
            };
            rows.push(AnalysisWorkloadRow {
                id: instance.id(),
                display_name: instance.display_name().to_owned(),
                kind: instance.kind(),
                tasks_per_point,
                rate_note,
                points,
                run_at: if exact {
                    instance.run_at().clone()
                } else {
                    // With no exactly-expanded space to narrow against, every
                    // instance runs everywhere whatever it declared. Reporting
                    // the declaration here would let the "At" cell claim a
                    // narrowing the queue does not honour.
                    AnalysisRunAt::AllPoints
                },
                unresolved: exact
                    && participation
                        .for_instance(instance.id())
                        .is_some_and(|entry| entry.refusal.is_some()),
            });
        }
        Ok(Self {
            rows,
            matrix_points: all_points,
            cost_per_task_ms: app.state.sim_setup.run_set.budgets.cost_per_point_ms,
        })
    }

    /// The plan's exact queue cardinality.
    pub(super) fn total_tasks(&self) -> Result<usize, String> {
        self.rows
            .iter()
            .try_fold(0usize, |total, row| {
                row.tasks_per_point
                    .checked_mul(row.points)
                    .and_then(|tasks| total.checked_add(tasks))
            })
            .ok_or_else(|| "Plan workload exceeds task capacity".to_owned())
    }

    /// The plan's modelled duration, as the surfaces state it.
    pub(super) fn total_duration(&self) -> Result<String, String> {
        self.total_tasks().map(|tasks| {
            run_set::format_duration_ms(run_set::modelled_cost_ms(tasks, self.cost_per_task_ms))
        })
    }
}

/// The tasks one point of `draft` costs, and why it is more than one.
///
/// Lifted verbatim out of the scalar task count rather than re-derived: the two
/// numbers were always the same arithmetic, and the only thing the table adds
/// is somewhere to show the factors separately.
fn task_rate(
    draft: &AnalysisDraft,
    app: &RSpiceApp,
    global_axes_active: bool,
) -> Result<(usize, Option<&'static str>), String> {
    match draft {
        // A temperature or corner sweep declares its own points. With a global
        // run set enabled those axes are the run set's, so the instance is an
        // ordinary one-task analysis and its own declaration does not multiply.
        AnalysisDraft::Temperature(state) if !global_axes_active => {
            let mut state = state.clone();
            state.ensure_initialized();
            let config = state
                .to_config(
                    &app.state.sim_setup.run_set,
                    app.state.sim_setup.reference_pvt,
                )
                .map_err(|error| format!("Temperature workload is invalid: {error}"))?;
            let rate = config
                .num_temps()
                .checked_add(1)
                .ok_or_else(|| "Temperature workload exceeds task capacity".to_owned())?;
            Ok((rate, Some("own temperature points, plus one assembly")))
        }
        AnalysisDraft::Corner(state) if !global_axes_active => {
            let mut state = state.clone();
            state.ensure_initialized();
            let config = state
                .to_config(
                    &app.state.sim_setup.run_set,
                    app.state.sim_setup.reference_pvt,
                )
                .map_err(|error| format!("Corner workload is invalid: {error}"))?;
            config
                .validate()
                .map_err(|error| format!("Corner workload is invalid: {error}"))?;
            let points = if !config.points.is_empty() {
                config.points.len()
            } else if config.full_matrix {
                config
                    .process_corners
                    .len()
                    .checked_mul(config.voltages.len())
                    .and_then(|count| count.checked_mul(config.temperatures.len()))
                    .ok_or_else(|| "Corner workload exceeds task capacity".to_owned())?
            } else {
                config
                    .process_corners
                    .len()
                    .max(config.voltages.len())
                    .max(config.temperatures.len())
            };
            let rate = points
                .checked_add(1)
                .ok_or_else(|| "Corner workload exceeds task capacity".to_owned())?;
            Ok((rate, Some("own corner points, plus one assembly")))
        }
        AnalysisDraft::Pss(state) => {
            let mut state = state.clone();
            state.ensure_initialized();
            let config = state
                .to_config()
                .map_err(|error| format!("PSS workload is invalid: {error}"))?;
            if config.num_harmonics > 0 {
                Ok((2, Some("steady state, plus a retained spectrum")))
            } else {
                Ok((1, None))
            }
        }
        _ => Ok((1, None)),
    }
}

/// The duration a surface promising `task_count` tasks must state.
///
/// Takes the state rather than the application so a surface holding only the
/// state — the preflight strip does — can state the same duration as one that
/// holds the frame. Every caller passes a task count it already displayed, so
/// the duration is never an independent claim about the same queue.
pub(super) fn modelled_duration(app: &AppState, task_count: usize) -> String {
    run_set::format_duration_ms(run_set::modelled_cost_ms(
        task_count,
        app.sim_setup.run_set.budgets.cost_per_point_ms,
    ))
}

// ------------------------------------------------------------- task-rate card

/// `ANALYSIS · AT · TASKS/PT · TASKS · DURATION`.
///
/// Fractions rather than pixels: the Run Set page already carries an axis strip
/// at half the surface width and a point table with its own participation
/// column, and a task-rate table measured in pixels would be the one that
/// pushed the page into a horizontal scroll. Fractions cannot overflow.
const RATE_COLUMNS: [f32; 5] = [0.34, 0.15, 0.15, 0.13, 0.23];

/// Where the plan's tasks come from, analysis by analysis.
///
/// The forecast tile above states one number; this states the fold that
/// produced it. It exists because the tile could not answer the question an
/// operator actually has when the number surprises them — *which* analysis is
/// expensive, and whether it is expensive per point or expensive because it
/// runs everywhere. Those are different fixes, and the tile conflated them.
pub(super) fn task_rate_card(ui: &mut egui::Ui, app: &RSpiceApp) {
    let resolved = PlanWorkload::resolve(app);
    let (status, tone) = match &resolved {
        Ok(workload) => match workload.total_tasks() {
            Ok(tasks) => (
                format!(
                    "{tasks} task{} \u{b7} {}",
                    if tasks == 1 { "" } else { "s" },
                    workload
                        .total_duration()
                        .unwrap_or_else(|_| "over capacity".to_owned()),
                ),
                Tone::Neutral,
            ),
            Err(error) => (error, Tone::Error),
        },
        Err(error) => (error.clone(), Tone::Error),
    };

    super::page_kit::card(ui, "Task rate", Some((status.as_str(), tone)), |ui| {
        let Ok(workload) = &resolved else {
            // The refusal is already the card's status line. Repeating it in
            // the body would state one fault twice and imply two.
            return;
        };
        if workload.rows.is_empty() {
            super::page_kit::card_note(
                ui,
                "No analysis instance is enabled, so the plan expands to no tasks.",
            );
            return;
        }
        super::page_kit::ledger_head(
            ui,
            &RATE_COLUMNS,
            &["Analysis", "At", "Tasks/pt", "Tasks", "Duration"],
        );
        for row in &workload.rows {
            let name = format!("{} \u{b7} {}", row.kind.code(), row.display_name);
            let at = row.at_cell(workload.matrix_points);
            let rate = row.rate_cell();
            let tasks = row.tasks().to_string();
            let duration = run_set::format_duration_ms(run_set::modelled_cost_ms(
                row.tasks(),
                workload.cost_per_task_ms,
            ));
            let response = super::page_kit::ledger_row(
                ui,
                &RATE_COLUMNS,
                &[
                    (name.as_str(), Tone::Neutral),
                    (
                        at.as_str(),
                        if row.unresolved {
                            Tone::Error
                        } else {
                            Tone::Neutral
                        },
                    ),
                    (
                        rate.as_str(),
                        if row.tasks_per_point > 1 {
                            Tone::Accent
                        } else {
                            Tone::Neutral
                        },
                    ),
                    (tasks.as_str(), Tone::Neutral),
                    (duration.as_str(), Tone::Neutral),
                ],
                false,
            );
            // The identity first, as the plan stack spells it, so a row here
            // and a row there are visibly the same instance. The rate is the
            // one number a reader cannot account for from the other cells, so
            // it explains itself here rather than spending a column on prose.
            let mut hover = format!("{} \u{b7} {}", row.id, row.display_name);
            if let Some(note) = row.rate_note {
                hover.push_str(&format!(
                    "\n{} tasks per point: {note}.",
                    row.tasks_per_point
                ));
            }
            if row.unresolved {
                hover.push_str(
                    "\nThis participation does not resolve against the declared space, so it is \
                     priced at every point. Re-open its point selection.",
                );
            }
            response.on_hover_text(hover);
        }
        super::page_kit::card_note(
            ui,
            "Tasks are the rate times the participation. Duration prices them at the run set's \
             modelled cost per task, which the budgets card declares.",
        );
    });
}
