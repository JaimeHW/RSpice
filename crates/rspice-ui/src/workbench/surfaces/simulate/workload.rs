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
//! [`crate::simulation::run_set::participating_point_keys`] — the same
//! resolver the prepared expansion mints tasks from, and the same one the Run
//! Set page's [`super::participation::PlanParticipation`] reports. So a row
//! that says an analysis visits four points names four points the queue
//! actually contains. Two reporters of one resolver is not two derivations;
//! two derivations is what the Plan Manager used to have.
//!
//! Duration is the same fact stated in time. It is the task count priced at the
//! run set's per-task budget through
//! [`crate::simulation::run_set::modelled_cost_ms`], and every surface that
//! promises a duration calls [`modelled_duration`] rather than multiplying
//! locally. That is what makes "tasks" and "how long" one claim instead of two.

use crate::product::AnalysisInstanceId;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::simulation::run_set::{self, AnalysisRunAt, RunSetPoint};
use crate::workbench::{AppState, RSpiceApp};

use super::page_kit::Tone;
use super::participation::PlanParticipation;

/// Where the pricing body learns each instance's participation from.
///
/// Two entry points, one arithmetic. Both arms answer from
/// [`crate::simulation::run_set::participating_point_keys`] against the same
/// expansion of the same declaration: `Expansion` calls it here, and `Reported`
/// reads back the call [`PlanParticipation`] already made for the surface it
/// belongs to. Reporting a resolver twice is not deciding twice.
enum ResolvedParticipation<'a> {
    /// A point expansion this pricing owns, for a plan that is not on screen
    /// and therefore has no participation report — a stored plan, or a campaign
    /// row built from one.
    Expansion(Vec<RunSetPoint<'a>>),
    /// A report the caller already resolved, so the declared space is expanded
    /// once for the surface rather than once per reader of it.
    Reported(&'a PlanParticipation),
}

impl ResolvedParticipation<'_> {
    /// Whether the declaration expanded exactly.
    ///
    /// An adaptive composition proposes its points from feedback, so there is
    /// nothing to narrow a participation against and every analysis is priced
    /// at the whole declared space.
    fn expands_exactly(&self) -> bool {
        match self {
            Self::Expansion(points) => !points.is_empty(),
            Self::Reported(participation) => !participation.point_keys.is_empty(),
        }
    }

    /// How many points one instance visits, or `None` where its participation
    /// does not resolve against the declared space.
    ///
    /// An instance the report does not carry is answered `None` too. Both
    /// callers walk the plan's enabled instances, so a miss is an
    /// inconsistency rather than a narrowing — and `None` prices it at the
    /// whole matrix, which is the direction that cannot silently shrink a
    /// budget.
    fn points_for(
        &self,
        instance: &crate::simulation::plan::AnalysisInstance,
        reference: run_set::ReferencePoint,
    ) -> Option<usize> {
        match self {
            Self::Expansion(points) => {
                run_set::participating_point_keys(instance.run_at(), points, reference)
                    .ok()
                    .map(|keys| keys.len())
            }
            Self::Reported(participation) => participation
                .for_instance(instance.id())
                .filter(|entry| entry.refusal.is_none())
                .map(|entry| entry.keys.len()),
        }
    }
}

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
    /// A refused instance is still priced at the whole matrix, because pricing
    /// it at zero is how a budget silently shrinks; the prepared expansion
    /// refuses such an instance by name rather than dispatching a narrowed one.
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
        Self::resolve_for(
            app.state.sim_setup.enabled_analysis_instances(),
            &app.state.sim_setup.run_set,
            app.state.sim_setup.reference_pvt,
        )
    }

    /// Price any plan over any run set.
    ///
    /// Takes the three things the arithmetic actually reads rather than the
    /// application, so a surface holding a *stored* plan — the Plan Manager's
    /// records, and the campaign rows built from them — prices it through this
    /// projection instead of multiplying points by analyses. That second
    /// derivation knew nothing about participation, about a PSS that retains a
    /// spectrum, or about a Temperature or Corner analysis's own point
    /// declaration, so a stored plan's task count could disagree with the queue
    /// the same plan dispatches.
    ///
    /// Expands the declared space itself: a stored plan is not on screen and
    /// has no participation report to read one from.
    pub(super) fn resolve_for<'a>(
        instances: impl Iterator<Item = &'a crate::simulation::plan::AnalysisInstance>,
        run_set: &run_set::RunSetState,
        reference: run_set::ReferencePoint,
    ) -> Result<Self, String> {
        // The same resolver the prepared expansion mints its tasks from, and
        // the one `super::participation::PlanParticipation` reports to the Run
        // Set page. Reporting it twice is fine; deciding it twice would not be.
        let expansion = run_set::resolve(run_set).unwrap_or_default();
        Self::priced(
            instances,
            run_set,
            reference,
            &ResolvedParticipation::Expansion(expansion),
        )
    }

    /// Price the working plan against a participation the caller already has.
    ///
    /// Resolving participation expands the whole declared space, so a surface
    /// that has done it once — the Run Set page's point table does, to fill its
    /// "At" column — must not pay for it again to price the same rows. Reading
    /// that report back is not a second derivation: it is
    /// [`crate::simulation::run_set::participating_point_keys`] applied to the
    /// same expansion, which is the one call [`Self::resolve_for`] makes too.
    pub(super) fn resolve_with(
        app: &RSpiceApp,
        participation: &PlanParticipation,
    ) -> Result<Self, String> {
        Self::priced(
            app.state.sim_setup.enabled_analysis_instances(),
            &app.state.sim_setup.run_set,
            app.state.sim_setup.reference_pvt,
            &ResolvedParticipation::Reported(participation),
        )
    }

    /// The one pricing body. Both entry points differ only in where they learn
    /// each instance's participation from, never in what they do with it.
    fn priced<'a>(
        instances: impl Iterator<Item = &'a crate::simulation::plan::AnalysisInstance>,
        run_set: &run_set::RunSetState,
        reference: run_set::ReferencePoint,
        resolved: &ResolvedParticipation<'_>,
    ) -> Result<Self, String> {
        let instances: Vec<&crate::simulation::plan::AnalysisInstance> = instances.collect();
        let global_axes_active = run_set.enabled_dimensions().next().is_some();
        // Where the declaration does not expand exactly — an adaptive
        // composition proposes its points from feedback — there is nothing to
        // narrow against, so every analysis is priced at the whole declared
        // space, which is what the validator's own forecast counts.
        // The forecast's own point count, not a validation run to read it: the
        // findings a whole validation allocates are thrown away here, and this
        // card is drawn beside five other things that price against the same
        // space.
        let all_points = if global_axes_active {
            run_set::forecast_point_count(run_set)
        } else {
            1
        };
        let exact = global_axes_active && resolved.expands_exactly();

        let mut rows = Vec::new();
        for instance in instances {
            let (tasks_per_point, rate_note) =
                task_rate(instance.draft(), run_set, reference, global_axes_active)?;
            // A participation that does not resolve against the declared space
            // is priced at the whole matrix rather than at zero: pricing it at
            // zero is how a budget silently shrinks. The prepared expansion
            // refuses such an instance by name a few steps later.
            let participating = resolved.points_for(instance, reference);
            let points = if exact {
                participating.unwrap_or(all_points)
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
                unresolved: exact && participating.is_none(),
            });
        }
        Ok(Self {
            rows,
            matrix_points: all_points,
            cost_per_task_ms: run_set.budgets.cost_per_point_ms,
        })
    }

    /// One instance's contribution, as a rail row states it: `15 pts · 15
    /// tasks · 3.55 s`.
    ///
    /// A projection of the row this table already holds, not a second pricing
    /// of the same instance: the points come from the same participation the
    /// prepared expansion mints tasks from, and the duration from the same
    /// per-task budget the plan total is priced at. `None` for an instance the
    /// table does not carry, which is every disabled one — a disabled instance
    /// costs the queue nothing, and a row claiming otherwise would be pricing
    /// work that is never dispatched.
    pub(super) fn row_cost_for(&self, id: AnalysisInstanceId) -> Option<String> {
        let row = self.rows.iter().find(|row| row.id == id)?;
        let tasks = row.tasks();
        Some(format!(
            "{} \u{b7} {tasks} tasks \u{b7} {}",
            row.at_cell(self.matrix_points),
            run_set::format_duration_ms(run_set::modelled_cost_ms(tasks, self.cost_per_task_ms)),
        ))
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

    /// Tasks one composed point costs: every instance that visits it, at its
    /// own rate.
    ///
    /// This is the Run Set page's per-point cell, and it is a fold of the same
    /// rows [`Self::total_tasks`] folds — over one point instead of over all of
    /// them. Before it existed the cell printed the enabled-instance count on
    /// every row, which is neither: it ignored participation, so a nominal-only
    /// transient was charged to all fifteen points, and it ignored the rate, so
    /// a PSS retaining a spectrum was charged one task instead of two.
    pub(super) fn tasks_at_point(&self, participation: &PlanParticipation, key: &str) -> usize {
        self.rows
            .iter()
            .filter(|row| participation.instance_visits(row.id, key))
            .fold(0usize, |total, row| {
                total.saturating_add(row.tasks_per_point)
            })
    }

    /// The plan's modelled duration, as the surfaces state it.
    pub(super) fn total_duration(&self) -> Result<String, String> {
        self.total_tasks().map(|tasks| {
            run_set::format_duration_ms(run_set::modelled_cost_ms(tasks, self.cost_per_task_ms))
        })
    }
}

/// The tasks one point of one instance costs, for a surface pricing a single
/// instance rather than the whole queue.
///
/// The point picker states how many tasks a selection buys, and a point is not
/// a task: a PSS that retains harmonics mints two. Reading the rate through the
/// one owner keeps the picker's number and this table's from disagreeing about
/// the same instance. `None` where the draft's workload cannot be read at all,
/// which is a plan with no queue to price.
pub(super) fn instance_task_rate(app: &RSpiceApp, draft: &AnalysisDraft) -> Option<usize> {
    let global_axes_active = app
        .state
        .sim_setup
        .run_set
        .enabled_dimensions()
        .next()
        .is_some();
    task_rate(
        draft,
        &app.state.sim_setup.run_set,
        app.state.sim_setup.reference_pvt,
        global_axes_active,
    )
    .ok()
    .map(|(rate, _)| rate)
}

/// The tasks one point of `draft` costs, and why it is more than one.
///
/// Lifted verbatim out of the scalar task count rather than re-derived: the two
/// numbers were always the same arithmetic, and the only thing the table adds
/// is somewhere to show the factors separately.
fn task_rate(
    draft: &AnalysisDraft,
    run_set: &run_set::RunSetState,
    reference: run_set::ReferencePoint,
    global_axes_active: bool,
) -> Result<(usize, Option<&'static str>), String> {
    match draft {
        // A temperature or corner sweep owns its own point expansion, in both
        // of the temperature form's axis modes — inheriting the run-set axis
        // authors the numbers once, but this instance still walks them.
        //
        // So these arms are the *only* ones that describe such an instance
        // truthfully, and they are guarded on the global axes being off. With
        // them on, two declarations claim one run and the plan does not compose
        // at all: `validate_for_plan` refuses it and the prepared expansion
        // refuses it again before minting a task. The rate below the guard is
        // therefore a placeholder for a plan that has no queue, never a price —
        // which is why `task_rate_card` states the refusal instead of calling
        // this at all.
        AnalysisDraft::Temperature(state) if !global_axes_active => {
            let mut state = state.clone();
            state.ensure_initialized();
            let config = state
                .to_config(run_set, reference)
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
                .to_config(run_set, reference)
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
/// The refusal that means this plan mints no queue at all.
///
/// A Temperature or Corner instance owns its own point expansion, and so does
/// an enabled global Run Set. Two expansions over one run is an ambiguity the
/// prepared snapshot refuses by name at the analysis-plan stage, before a
/// single task exists — and `validate_for_plan` refuses it here under this
/// stable identity.
///
/// Both of the temperature sweep's axis modes own an expansion. Inheriting the
/// run-set axis removes *drift*, because the temperatures are then authored
/// once; it does not move who walks them. So neither mode makes such an
/// instance composable, and neither may be priced against the global matrix.
const COMPOSITION_REFUSAL: &str = "RUNSET-ANALYSIS-COMPOSITION";

/// The run set's own composition refusals, if it raised any.
///
/// Read as the run set's verdict, by the stable identity refusals carry, so no
/// surface decides for itself that a plan composes. Pricing a refused
/// composition at rate-times-matrix would promise a queue the expansion mints
/// nothing for — the most expensive kind of wrong number, because it is
/// arithmetically consistent with everything around it.
pub(super) fn composition_refusals(
    validation: &run_set::RunSetValidation,
) -> impl Iterator<Item = &str> {
    validation
        .errors
        .iter()
        .filter(|error| error.id == COMPOSITION_REFUSAL)
        .map(|error| error.message.as_str())
}

/// The plan's queue, priced per analysis.
///
/// `resolved` is the workload the route already priced, handed in rather than
/// asked for. Pricing one expands the declared space, and the Run Set route
/// draws this card beside five other things that want the same workload — six
/// expansions a frame, each costing the size of the space rather than the size
/// of the declaration.
pub(super) fn task_rate_card(
    ui: &mut egui::Ui,
    app: &RSpiceApp,
    validation: &run_set::RunSetValidation,
    resolved: Result<&PlanWorkload, &String>,
) {
    let refusals: Vec<&str> = composition_refusals(validation).collect();
    if !refusals.is_empty() {
        super::page_kit::card(
            ui,
            "Task rate",
            Some(("no queue \u{b7} composition refused", Tone::Error)),
            |ui| {
                for refusal in refusals {
                    super::page_kit::card_note(ui, refusal);
                }
                super::page_kit::card_note(
                    ui,
                    "Nothing is priced while two declarations claim the same run: the prepared \
                     expansion refuses before it mints a task, so any task count here would be \
                     for a queue that never exists.",
                );
            },
        );
        return;
    }

    let (status, tone) = match resolved {
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
        let Ok(workload) = resolved else {
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
        // Named by the plan, not by the row's own copy of the display name.
        // Two unnamed transients answer to the same display name, so the
        // Analysis column read "TRAN · Transient" on both rows and priced two
        // different queues under one heading. `instance_list_label` is the
        // plan's answer to what an instance is called beside its siblings, and
        // the tab strip above this card now asks the same question of it.
        let plan = app.state.sim_setup.stable_analysis_plan().ok();
        for row in &workload.rows {
            let listed = plan
                .and_then(|plan| {
                    let index = plan
                        .instances()
                        .iter()
                        .position(|instance| instance.id() == row.id)?;
                    plan.instance_list_label(index)
                })
                .unwrap_or_else(|| row.display_name.clone());
            let name = format!("{} \u{b7} {listed}", row.kind.code());
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
                super::page_kit::RowPress::Ignored,
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
