//! Per-instance run-set participation, as the Simulation Studio's surfaces
//! read it.
//!
//! Four surfaces have to agree about which points an analysis visits: the Run
//! Set page's forecast, its resolved point table, the specification registry's
//! coverage advisory, and the Analyses page's own control. Each used to answer
//! the question by multiplying the whole plan by the whole matrix, which is
//! exactly the arithmetic participation exists to stop.
//!
//! [`PlanParticipation`] resolves it once per frame from the working plan and
//! the working run set, through the same
//! [`crate::simulation::run_set::participating_point_keys`] the prepared
//! expansion uses. Nothing here decides participation; it only reports what
//! that resolver returned, so a surface cannot disagree with the queue.

use std::collections::HashSet;

use egui::Ui;

use crate::product::AnalysisInstanceId;
use crate::simulation::plan::{AnalysisInstance, AnalysisKind};
use crate::simulation::run_set::{self, AnalysisRunAt, RunSetPoint};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogSize};
use crate::workbench::RSpiceApp;
use crate::workbench::state::AnalysisRunPointsDraft;

/// One enabled analysis instance and the points it visits.
pub(super) struct InstanceParticipation {
    pub(super) id: AnalysisInstanceId,
    pub(super) kind: AnalysisKind,
    pub(super) run_at: AnalysisRunAt,
    /// The resolved point identities, in declared order. Empty only when the
    /// participation refused, in which case `refusal` says why.
    pub(super) keys: Vec<String>,
    /// Why this participation does not resolve against the declared space.
    pub(super) refusal: Option<String>,
}

impl InstanceParticipation {
    /// The compact stat a row or a card carries: `nominal`, `15 pts`, `4/15`.
    pub(super) fn stat(&self, matrix: usize) -> String {
        if self.refusal.is_some() {
            return "unresolved".to_owned();
        }
        match self.run_at {
            AnalysisRunAt::AllPoints => format!("{} pts", self.keys.len()),
            AnalysisRunAt::NominalPoint => "nominal".to_owned(),
            AnalysisRunAt::SelectedPoints(_) => format!("{}/{matrix}", self.keys.len()),
        }
    }
}

/// The whole plan's participation against the working run set.
pub(super) struct PlanParticipation {
    /// Every point the run set resolves, in declared order. Empty when the
    /// declaration does not expand exactly, which is a refusal the Run Set page
    /// already reports on its own terms.
    pub(super) point_keys: Vec<String>,
    /// The reference point's identity, when the declared space contains one.
    pub(super) nominal_key: Option<String>,
    pub(super) instances: Vec<InstanceParticipation>,
    /// Whether the run set declares any axis at all. With none, every analysis
    /// runs once and participation is not a question the page can ask.
    pub(super) has_axes: bool,
}

impl PlanParticipation {
    /// Resolve the working plan against the working run set.
    pub(super) fn resolve(app: &RSpiceApp) -> Self {
        let state = &app.state.sim_setup.run_set;
        let reference = app.state.sim_setup.reference_pvt;
        let has_axes = state.enabled_dimensions().next().is_some();
        let points: Vec<RunSetPoint<'_>> = run_set::resolve(state).unwrap_or_default();
        let point_keys: Vec<String> = points.iter().map(RunSetPoint::point_key).collect();
        let nominal_key = run_set::nominal_point_key(&points, reference);
        let instances = app
            .state
            .sim_setup
            .enabled_analysis_instances()
            .map(|instance| {
                let run_at = instance.run_at().clone();
                match run_set::participating_point_keys(&run_at, &points, reference) {
                    Ok(keys) => InstanceParticipation {
                        id: instance.id(),
                        kind: instance.kind(),
                        run_at,
                        keys,
                        refusal: None,
                    },
                    Err(refusal) => InstanceParticipation {
                        id: instance.id(),
                        kind: instance.kind(),
                        run_at,
                        keys: Vec::new(),
                        refusal: Some(refusal.message),
                    },
                }
            })
            .collect();
        Self {
            point_keys,
            nominal_key,
            instances,
            has_axes,
        }
    }

    /// How many points one analysis contributes a task at.
    ///
    /// A run set with no axes runs everything once, and a participation that
    /// refuses counts as the whole space — the same rule
    /// [`crate::simulation::run_set::participating_point_count`] states, because
    /// pricing a refused plan at zero is how a budget silently shrinks.
    pub(super) fn point_count_for(&self, id: AnalysisInstanceId) -> usize {
        if !self.has_axes {
            return 1;
        }
        self.instances
            .iter()
            .find(|entry| entry.id == id)
            .map_or(self.point_keys.len(), |entry| {
                if entry.refusal.is_some() {
                    self.point_keys.len()
                } else {
                    entry.keys.len()
                }
            })
    }

    pub(super) fn for_instance(&self, id: AnalysisInstanceId) -> Option<&InstanceParticipation> {
        self.instances.iter().find(|entry| entry.id == id)
    }

    /// The enabled analyses that run at one point, by kind code.
    ///
    /// Used by the point table, where the count is the cell and the names are
    /// the hover: a row that says "4/6" without naming the two that are absent
    /// tells the reader there is a problem and not where.
    pub(super) fn analyses_at(&self, key: &str) -> Vec<AnalysisKind> {
        self.instances
            .iter()
            .filter(|entry| entry.refusal.is_none() && entry.keys.iter().any(|held| held == key))
            .map(|entry| entry.kind)
            .collect()
    }

    /// The enabled analyses that do *not* run at one point.
    pub(super) fn analyses_absent_at(&self, key: &str) -> Vec<AnalysisKind> {
        self.instances
            .iter()
            .filter(|entry| entry.refusal.is_none() && !entry.keys.iter().any(|held| held == key))
            .map(|entry| entry.kind)
            .collect()
    }

    /// How many of `demanded` points at least one enabled analysis visits.
    ///
    /// This is what a specification's coverage is bounded by: a bound scoped to
    /// a point no analysis runs at can never be judged there, however the run
    /// goes.
    pub(super) fn covered_of(&self, demanded: &[String]) -> usize {
        let visited: HashSet<&str> = self
            .instances
            .iter()
            .filter(|entry| entry.refusal.is_none())
            .flat_map(|entry| entry.keys.iter().map(String::as_str))
            .collect();
        demanded
            .iter()
            .filter(|key| visited.contains(key.as_str()))
            .count()
    }
}

/// The instance a participation edit applies to, resolved from the plan.
pub(super) fn instance_run_at(app: &RSpiceApp, id: AnalysisInstanceId) -> AnalysisRunAt {
    app.state
        .sim_setup
        .analysis_plan
        .as_ref()
        .and_then(|plan| plan.instance(id))
        .map_or_else(AnalysisRunAt::default, |instance| {
            AnalysisInstance::run_at(instance).clone()
        })
}

// ------------------------------------------------------------ point picker

/// The largest matrix the picker presents point by point.
///
/// The fused dialog does not scroll, so the matrix is drawn as a dense
/// multi-column list — three columns of fifteen at the minimum supported dialog
/// size. Past that the picker refuses and names the alternative rather than
/// hiding rows behind a scrollbar or truncating the matrix silently: a
/// selection made against a partial list is a selection against a run set the
/// operator was not shown.
pub(super) const POINT_PICKER_LIMIT: usize = 45;

/// Columns the picker lays the matrix out in at a given width.
pub(super) const fn picker_columns(points: usize) -> usize {
    if points > 30 {
        3
    } else if points > 15 {
        2
    } else {
        1
    }
}

/// Open the picker on one instance, pre-ticked with the points it visits.
pub(super) fn open_point_picker(app: &mut RSpiceApp, id: AnalysisInstanceId) {
    let resolved = PlanParticipation::resolve(app);
    let selected = resolved
        .for_instance(id)
        .filter(|entry| entry.refusal.is_none())
        .map_or_else(|| resolved.point_keys.clone(), |entry| entry.keys.clone());
    app.state.workbench.simulation_workflow = Some(
        crate::workbench::state::SimulationWorkflowDialog::AnalysisRunPoints(
            crate::workbench::state::AnalysisRunPointsDraft::new(id, selected),
        ),
    );
}

/// Commit one analysis's run-set participation.
///
/// Deliberately the same shape as [`super::lifecycle::commit_numeric_override`]:
/// participation is a plan mutation like any other, so it goes through
/// `set_run_at`'s transaction, leaves a receipt, refreshes the projections and
/// invalidates preflight through the one path every editor uses. That is what
/// satisfies the staleness contract by construction — an authorized preflight
/// cannot be followed by a narrowing and then a dispatch of a queue nothing
/// checked.
pub(super) fn commit_run_at(
    app: &mut RSpiceApp,
    id: AnalysisInstanceId,
    run_at: AnalysisRunAt,
) -> Result<(), String> {
    let result = match app.state.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => plan
            .set_run_at(id, run_at)
            .map_err(|error| error.to_string()),
        Err(error) => Err(error),
    };
    match result {
        Ok(receipt) => {
            super::lifecycle::refresh_analysis_projections(app);
            super::lifecycle::record_receipt(app, &receipt);
            Ok(())
        }
        Err(error) => {
            super::lifecycle::record_failure(app, "Run-set participation", &error);
            Err(error)
        }
    }
}

/// Adopt a point selection from the picker.
pub(super) fn commit_point_selection(
    app: &mut RSpiceApp,
    draft: &crate::workbench::state::AnalysisRunPointsDraft,
) -> Result<String, String> {
    // Re-resolved at commit rather than trusted from the draft: the run set is
    // editable on another page while this dialog is open, and a key that has
    // stopped existing must be refused here rather than stored and refused at
    // every later preflight.
    let declared: HashSet<String> = PlanParticipation::resolve(app)
        .point_keys
        .into_iter()
        .collect();
    if let Some(stale) = draft.selected.iter().find(|key| !declared.contains(*key)) {
        return Err(format!(
            "Run-set point {stale} no longer exists; the run set changed while this selection was \
             open. Re-open the point picker."
        ));
    }
    let count = draft.selected.len();
    let total = declared.len();
    let run_at = if count == total && total > 0 {
        // A selection of everything *is* "all points". Storing it as a list
        // would freeze today's matrix onto the instance, so a later axis value
        // would orphan an instance that never narrowed anything.
        AnalysisRunAt::AllPoints
    } else {
        AnalysisRunAt::SelectedPoints(draft.selected.clone())
    };
    commit_run_at(app, draft.instance, run_at)?;
    Ok(format!(
        "This analysis now runs at {count} of {total} run-set points. The run set keeps every \
         point it declares, and every other instance keeps its own scope."
    ))
}

/// The point picker: which resolved run-set points one instance is scoped to.
///
/// The picker renders the run set's own resolved point table rather than a
/// second list — the same rows, the same identities, the same working revision
/// the Run Set page previewed. Participation is a property of this instance
/// alone: the run set keeps every point it declares, every other instance keeps
/// its own scope, and every retained dataset keeps the points it already froze.
pub(super) fn analysis_run_points_dialog(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    mut draft: AnalysisRunPointsDraft,
) {
    let resolved = PlanParticipation::resolve(app);
    let points = resolved.point_keys.clone();
    let labels = point_labels(app);
    let over_limit = points.len() > POINT_PICKER_LIMIT;
    let name = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .and_then(|plan| plan.instance(draft.instance))
        .map_or_else(
            || "no active instance".to_owned(),
            |instance| instance.kind().label().to_owned(),
        );

    // Points the run set has dropped since the dialog opened are struck from
    // the draft as they are drawn, so a selection can never be committed
    // against a matrix the reader was not shown.
    draft.selected.retain(|key| points.contains(key));
    draft.validation_error = picker_refusal(&points, &draft, over_limit);
    let enabled = draft.validation_error.is_none();
    let chosen = draft.selected.len();

    let choice = Dialog::new(
        "SIMULATION PLAN · INSTANCE PARTICIPATION",
        "Run points",
        "Apply point selection",
    )
    .description("Choose which resolved run-set points this analysis instance is executed at.")
    .size(DialogSize::SimulationWorkflow)
    .flush_body()
    .ghost("Cancel")
    .primary_enabled(enabled)
    .primary_on_enter(false)
    .show(ctx, |ui| {
        super::workflows::workflow_setting_row(
            ui,
            "Instance",
            "Restricting an instance never removes a run-set point.",
            |ui| {
                ui.label(
                    egui::RichText::new(&name).font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                );
            },
        );
        super::workflows::workflow_section_heading(
            ui,
            &format!("Resolved run-set points · {}", points.len()),
        );
        if over_limit {
            picker_note(
                ui,
                &format!(
                    "This run set resolves {} points. Point-by-point scoping is offered up to \
                     {POINT_PICKER_LIMIT}; above that, disable a run-set axis or scope this \
                     instance to the nominal point instead. No point is hidden and no selection \
                     was changed.",
                    points.len()
                ),
            );
        }
        point_grid(ui, &points, &labels, &mut draft.selected, !over_limit);
        picker_note(
            ui,
            &format!(
                "{chosen} of {} points selected \u{00b7} {chosen} task{} from this instance. The \
                 run set keeps every point it declares and every other instance keeps its own \
                 scope.",
                points.len(),
                if chosen == 1 { "" } else { "s" }
            ),
        );
        super::workflows::workflow_validation_message(ui, draft.validation_error.as_deref());
    });
    super::workflows::finish_workflow_choice(ctx, app, choice, draft, commit_point_selection);
}

/// Why the picker cannot apply what it is showing.
fn picker_refusal(
    points: &[String],
    draft: &AnalysisRunPointsDraft,
    over_limit: bool,
) -> Option<String> {
    if points.is_empty() {
        return Some(
            "This composition has no materialized point table yet. Validate and preview the run \
             set before scoping an instance to part of it."
                .to_owned(),
        );
    }
    if over_limit {
        return Some(format!(
            "Point-by-point scoping is offered up to {POINT_PICKER_LIMIT} points."
        ));
    }
    if draft.selected.is_empty() {
        return Some(
            "Select at least one run-set point; an instance that runs nowhere produces no \
             evidence. Disable the instance instead."
                .to_owned(),
        );
    }
    None
}

/// The matrix, as a dense multi-column list of one checkbox per point.
///
/// Adding a temperature widens the list instead of lengthening it, which is
/// what keeps a fused dialog that does not scroll able to show the whole
/// matrix up to its stated limit.
fn point_grid(
    ui: &mut Ui,
    points: &[String],
    labels: &[String],
    selected: &mut Vec<String>,
    enabled: bool,
) {
    let columns = picker_columns(points.len());
    let rows = points.len().div_ceil(columns.max(1));
    ui.columns(columns.max(1), |panes| {
        for (column, pane) in panes.iter_mut().enumerate() {
            for row in 0..rows {
                let index = column * rows + row;
                let Some(key) = points.get(index) else {
                    break;
                };
                let mut ticked = selected.iter().any(|held| held == key);
                pane.add_enabled_ui(enabled, |pane| {
                    let label = labels.get(index).map_or("", String::as_str);
                    if pane
                        .checkbox(&mut ticked, format!("{:02}  {label}", index + 1))
                        .changed()
                    {
                        if ticked {
                            selected.push(key.clone());
                        } else {
                            selected.retain(|held| held != key);
                        }
                    }
                });
            }
        }
    });
}

/// The authored label of every resolved point, in declared order.
fn point_labels(app: &RSpiceApp) -> Vec<String> {
    run_set::resolve(&app.state.sim_setup.run_set)
        .unwrap_or_default()
        .iter()
        .map(RunSetPoint::label)
        .collect()
}

fn picker_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .inner_margin(egui::Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.add(
                egui::Label::new(
                    egui::RichText::new(text)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
}

// -------------------------------------------------- the instance's control

/// What a participation edit on the Analyses page asks for.
pub(super) enum ParticipationAction {
    /// Commit one of the two modes that need no point list.
    Set(AnalysisRunAt),
    /// Open the picker; the selection itself is committed from there.
    ChoosePoints,
}

/// The Analyses page's own participation control, on the instance contract.
///
/// It sits with the contract rather than in the analysis form because it is not
/// a property of the analysis kind: every kind has one, none of them spell it
/// differently, and a copy per form is thirty-odd places for the three options
/// to drift apart.
///
/// The control is only offered where it means something. With no run-set axis
/// enabled every analysis runs exactly once, and a three-way choice about which
/// of one point to run at would be a control whose every setting is the same
/// run.
pub(super) fn participation_row(
    ui: &mut Ui,
    resolved: &PlanParticipation,
    id: AnalysisInstanceId,
) -> Option<ParticipationAction> {
    if !resolved.has_axes {
        crate::workbench::design_system::property_row(
            ui,
            "Run-set points",
            "1 · no global axis is enabled",
        );
        return None;
    }
    let matrix = resolved.point_keys.len();
    let entry = resolved.for_instance(id)?;
    let options: Vec<String> = [
        AnalysisRunAt::AllPoints,
        AnalysisRunAt::NominalPoint,
        AnalysisRunAt::SelectedPoints(Vec::new()),
    ]
    .iter()
    .map(|mode| mode.label().to_owned())
    .collect();
    let mut action = None;
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("Run-set points")
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        let width = (ui.available_width() - 118.0).clamp(150.0, 260.0);
        if let Some(picked) = crate::ui::widgets::select(
            ui,
            "analysis-run-at",
            "Run-set points this analysis is executed at",
            entry.run_at.label(),
            &options,
            width,
        ) {
            action = match picked {
                0 => Some(ParticipationAction::Set(AnalysisRunAt::AllPoints)),
                1 => Some(ParticipationAction::Set(AnalysisRunAt::NominalPoint)),
                // "Selected points" is not a value the select can commit: it
                // needs the points themselves, and committing an empty list
                // here would be an instance that runs nowhere.
                _ => Some(ParticipationAction::ChoosePoints),
            };
        }
        if crate::ui::widgets::Button::new("Choose\u{2026}")
            .show(ui)
            .on_hover_text("Pick the exact resolved run-set points this instance is executed at")
            .clicked()
        {
            action = Some(ParticipationAction::ChoosePoints);
        }
        ui.label(
            egui::RichText::new(entry.stat(matrix))
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(if entry.refusal.is_some() {
                    t.color.err
                } else {
                    t.color.text_faint
                }),
        );
    });
    if let Some(refusal) = entry.refusal.as_deref() {
        picker_note(ui, refusal);
    }
    action
}

/// Apply whatever the control asked for.
pub(super) fn apply_participation_action(
    app: &mut RSpiceApp,
    id: AnalysisInstanceId,
    action: ParticipationAction,
) {
    match action {
        ParticipationAction::Set(run_at) => {
            let _ = commit_run_at(app, id, run_at);
        }
        ParticipationAction::ChoosePoints => open_point_picker(app, id),
    }
}

// ------------------------------------------- reconciling a run-set edit

/// What a run-set edit did to the point selections that named its points.
pub(super) struct Reconciliation {
    /// One line per instance whose selection lost points, naming them.
    pub(super) pruned: Vec<String>,
}

/// Bring every point selection back into agreement with an edited run set.
///
/// A run-set edit can remove a point an instance was scoped to. Leaving the
/// stale identity behind would make the instance refuse at every later
/// preflight with a message about a point nobody can find, so the edit settles
/// it at the moment the points that are disappearing can still be named.
///
/// Two outcomes, and which one applies is decided by what is left rather than
/// by how much was removed. A selection that keeps at least one point is
/// *pruned*, and the receipt names the dropped points — the operator's
/// declaration survives the edit as closely as the new space allows. A
/// selection that would keep nothing is a *refusal*: applying it would leave an
/// analysis that runs nowhere, which is a disabled analysis, and no run-set
/// edit should silently disable one.
///
/// Nothing here is silent. Every path either names what it dropped or refuses.
pub(super) fn reconcile_selections(app: &mut RSpiceApp) -> Result<Reconciliation, String> {
    let Some(points) = run_set::resolve(&app.state.sim_setup.run_set) else {
        // A space that does not expand exactly names no points, so there is
        // nothing to judge a selection against. The Run Set page already
        // refuses such a declaration on its own terms.
        return Ok(Reconciliation { pruned: Vec::new() });
    };
    let declared: HashSet<String> = points.iter().map(RunSetPoint::point_key).collect();

    // Decided for every instance before any of them is changed: a refusal found
    // on the fourth analysis must not leave the first three already pruned.
    let mut prunes: Vec<(AnalysisInstanceId, AnalysisKind, Vec<String>, Vec<String>)> = Vec::new();
    for instance in app.state.sim_setup.enabled_analysis_instances() {
        let AnalysisRunAt::SelectedPoints(keys) = instance.run_at() else {
            continue;
        };
        let (kept, dropped): (Vec<String>, Vec<String>) =
            keys.iter().cloned().partition(|key| declared.contains(key));
        if dropped.is_empty() {
            continue;
        }
        if kept.is_empty() {
            return Err(format!(
                "This edit removes every run-set point the {} analysis is scoped to ({}). It \
                 would leave that analysis running nowhere. Widen its participation first, or \
                 disable it.",
                instance.kind().label(),
                dropped.join(", ")
            ));
        }
        prunes.push((instance.id(), instance.kind(), kept, dropped));
    }

    let mut pruned = Vec::with_capacity(prunes.len());
    for (id, kind, kept, dropped) in prunes {
        let kept_count = kept.len();
        commit_run_at(app, id, AnalysisRunAt::SelectedPoints(kept))?;
        pruned.push(format!(
            "{} dropped {} point{} it was scoped to ({}) and now runs at {kept_count}",
            kind.label(),
            dropped.len(),
            if dropped.len() == 1 { "" } else { "s" },
            dropped.join(", "),
        ));
    }
    Ok(Reconciliation { pruned })
}
