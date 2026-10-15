//! Verification evidence, specifications, checks and history.
//!
//! Every Verify page has one grammar, the ADE results view's: a toolbar row
//! (title, the run it reads when there is a choice, one summary line, its
//! actions), one dense table of the run's own values, the detail of a row only
//! when that row is selected, and — when the run holds nothing for the page —
//! one line saying what produces data, with the action that produces it.

mod corners;
mod histogram;
mod optimization;
mod regression_contract;
mod regression_panel;
mod safety;
mod table;
mod toolbar;
mod tuning;
mod yield_page;

use egui::{ScrollArea, Ui};
use sha2::{Digest as _, Sha256};

use crate::state::SpecEntry;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Problem};
use crate::workbench::RSpiceApp;

use super::super::commands::vocabulary::Command;
use super::super::state::{VerificationPage, VerificationSessionState};
use tuning::{sync_tuning_session, tuning, tuning_review_dialog};

use corners::*;
use optimization::*;
use regression_contract::*;
use regression_panel::*;
use safety::*;
use yield_page::*;

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let page = app.state.workbench.verification_page;
    if page == VerificationPage::Tuning {
        sync_tuning_session(app);
    }
    // A refused action is stated under the toolbar of the page it was taken
    // on; another page has no business repeating it.
    let shown_page = egui::Id::new("workbench.verify.shown_page");
    if ui
        .ctx()
        .data(|data| data.get_temp::<VerificationPage>(shown_page))
        != Some(page)
    {
        app.state.workbench.verification.action_problem = None;
        ui.ctx().data_mut(|data| data.insert_temp(shown_page, page));
    }
    let ctx = ui.ctx().clone();
    let t = Tokens::get(ui.ctx());
    let surface = ui.available_rect_before_wrap();
    ui.painter().rect_filled(surface, 0.0, t.color.bg_app);
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
        match page {
            VerificationPage::Yield => yield_page(ui, app),
            VerificationPage::Corners => corners(ui, app),
            VerificationPage::Optimization => optimization(ui, app),
            VerificationPage::DeviceSafety => device_safety(ui, app),
            VerificationPage::Tuning => tuning(ui, app),
            VerificationPage::Regression => regression(ui, app),
        }
    });
    regression_baseline_picker(&ctx, app);
    tuning_review_dialog(&ctx, app);
}

/// The runs a page could read: the run it reads now, and every other run that
/// holds the page's evidence.
fn run_choice(
    app: &RSpiceApp,
    holds: impl Fn(&crate::state::SimulationRun) -> bool,
) -> toolbar::RunChoice {
    let selected = verification_run_index(app);
    toolbar::RunChoice {
        candidates: app
            .state
            .simulation
            .retained
            .runs
            .iter()
            .enumerate()
            .filter(|(index, run)| Some(*index) == selected || holds(run))
            .map(|(index, run)| (index, run.label.clone()))
            .collect(),
        selected,
    }
}

/// Make the run the reader picked on a toolbar the one Verify reads.
fn select_verification_run(simulation: &mut crate::state::SimulationState, index: usize) {
    let _ = simulation.select_run(index);
}

/// `summary` naming the run it describes, for a toolbar with no run selector
/// to name it.
fn append_run(summary: toolbar::Summary, run: Option<&str>) -> toolbar::Summary {
    match run {
        Some(run) => toolbar::Summary {
            text: format!("{} \u{00b7} {run}", summary.text),
            ..summary
        },
        None => summary,
    }
}

/// Record how the action the reader just took went: a refusal is stated under
/// the toolbar, and an action that worked clears it and says nothing.
fn record_action(session: &mut VerificationSessionState, result: Result<(), String>) {
    session.action_problem = result.err();
}

/// What this page could not do, under the toolbar it was asked on: the
/// refusal of the last action, and the reason the run it reads cannot be held
/// against the plan's limits.
fn action_problem_line(ui: &mut Ui, app: &RSpiceApp) {
    if let Some(problem) = &app.state.workbench.verification.action_problem {
        let _ = crate::ui::widgets::problem_line(ui, Problem::Failure, problem);
    }
    // `NoDataset` is the empty state, which already says what produces data;
    // repeating it under the same toolbar would be two lines for one fact.
    if verification_run(app).is_some()
        && let Some(refusal) = verification_evidence_domain(app).refusal()
    {
        let _ = crate::ui::widgets::problem_line(
            ui,
            Problem::Warning,
            &format!("Values are unjudged: the {refusal}."),
        );
    }
}

/// How the run Verify reads relates to the plan whose limits it would be
/// judged against.
///
/// Classified through the one owner of that question, so this surface and the
/// studio's requirements page cannot reach different conclusions about the
/// same run.
fn verification_evidence_domain(app: &RSpiceApp) -> crate::state::EvidenceDomain {
    app.state
        .simulation
        .evidence_domain(verification_plan_id(app))
}

/// The limits the run this page reads may be judged against.
///
/// A dataset another plan owns, or a manual deck no plan owns, is not this
/// plan's evidence: the studio's requirements page has always refused it, and
/// Verify used to judge it anyway, so one surface reported a margin and the
/// other reported nothing for the same run and the same limit. The run's own
/// values stay on the page — they are what it measured — with no verdict
/// column and the line under the toolbar naming why.
fn verification_specs(app: &RSpiceApp) -> Vec<SpecEntry> {
    if !verification_evidence_domain(app).answers_a_plan_limit() {
        return Vec::new();
    }
    verification_plan_id(app).map_or_else(
        || app.state.workspace.content.specs.clone(),
        |plan_id| app.state.workspace.content.active_specs(plan_id).to_vec(),
    )
}

fn regression_baseline_picker(ctx: &egui::Context, app: &mut RSpiceApp) {
    if !app
        .state
        .workbench
        .verification
        .regression_baseline_picker_open
    {
        return;
    }

    let active_run = app.state.simulation.active_run();
    let active_run_id = active_run.map(|run| run.run_id);
    let mut candidates = app
        .state
        .simulation
        .retained
        .runs
        .iter()
        .filter(|run| {
            Some(run.run_id) != active_run_id
                && validate_regression_run(run).is_ok()
                && active_run.is_some_and(|candidate| {
                    validate_regression_run(candidate).is_ok()
                        && (!derive_regression_checks(run, candidate).is_empty()
                            || !regression_waveform_pairs(run, candidate).is_empty())
                })
        })
        .map(|run| (run.run_id, run.id, run.label.clone()))
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| right.1.cmp(&left.1));

    let mut selected = app
        .state
        .workbench
        .verification
        .regression_baseline_picker_selection;
    if selected.is_none_or(|selected| !candidates.iter().any(|row| row.0 == selected)) {
        selected = candidates.first().map(|row| row.0);
    }

    let mut open = true;
    let mut cancel = false;
    let mut commit = false;
    let viewport = ctx.content_rect().size();
    let width = (viewport.x - 16.0).clamp(1.0, 620.0);
    let candidate_height = (viewport.y - 210.0).clamp(90.0, 420.0);
    egui::Window::new("Select regression baseline")
        .id(egui::Id::new("workbench.verify.regression_baseline_picker"))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_width(width)
        .min_width(width)
        .max_width(width)
        .show(ctx, |ui| {
            let t = Tokens::get(ui.ctx());
            if candidates.is_empty() {
                ui.label(
                    egui::RichText::new("No other runs to compare against.")
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(t.color.text),
                );
            } else {
                let list_width = ui.available_width();
                ScrollArea::vertical()
                    .id_salt("workbench.verify.regression_baseline_candidates")
                    .max_height(candidate_height)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        egui::Frame::new()
                            .fill(t.color.bg_panel)
                            .stroke(egui::Stroke::new(1.0, t.color.border))
                            .show(ui, |ui| {
                                ui.set_width((list_width - 2.0).max(1.0));
                                for (run_id, _, label) in &candidates {
                                    let is_selected = selected == Some(*run_id);
                                    let response = ui.selectable_label(
                                        is_selected,
                                        egui::RichText::new(label)
                                            .font(theme::sans(tokens::FS_1, FontWeight::Regular)),
                                    );
                                    if response.clicked() {
                                        selected = Some(*run_id);
                                    }
                                }
                            });
                    });
            }

            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let can_commit = selected.is_some();
                let use_baseline = Button::new("Use baseline").enabled(can_commit);
                let use_baseline = if can_commit {
                    use_baseline.accent()
                } else {
                    use_baseline
                };
                if use_baseline.show(ui).clicked() {
                    commit = true;
                }
                if Button::new("Cancel").show(ui).clicked() {
                    cancel = true;
                }
            });
        });

    if commit {
        if let Some(run_id) = selected {
            if let Err(error) = commit_regression_baseline(app, run_id) {
                app.state.workbench.verification.action_problem =
                    Some(format!("The baseline was not changed: {error}"));
                app.state
                    .workbench
                    .verification
                    .regression_baseline_picker_selection = selected;
                return;
            }
            app.state.workbench.verification.action_problem = None;
        }
        open = false;
    }
    if cancel {
        open = false;
    }
    app.state
        .workbench
        .verification
        .regression_baseline_picker_selection = selected;
    app.state
        .workbench
        .verification
        .regression_baseline_picker_open = open;
}

fn verification_run_index(app: &RSpiceApp) -> Option<usize> {
    app.state
        .simulation
        .view
        .active_run_idx
        .filter(|index| *index < app.state.simulation.retained.runs.len())
}

fn verification_run(app: &RSpiceApp) -> Option<&crate::state::SimulationRun> {
    verification_run_index(app).and_then(|index| app.state.simulation.retained.runs.get(index))
}

fn verification_yield_results(app: &RSpiceApp) -> &[rspice_results::yield_analysis::YieldResult] {
    verification_run(app)
        .and_then(|run| {
            app.state
                .simulation
                .retained
                .yield_results_for_dataset(run.dataset_id)
        })
        .unwrap_or(&[])
}

/// The plan whose limits this surface reads, when the plan is stable.
fn verification_plan_id(app: &RSpiceApp) -> Option<crate::product::SimulationPlanId> {
    app.state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JointSampleSummary {
    passing: usize,
    total: usize,
    specification_count: usize,
}

impl JointSampleSummary {
    fn yield_percent(self) -> f64 {
        self.passing as f64 / self.total as f64 * 100.0
    }
}

fn joint_sample_summary(
    results: &[rspice_results::yield_analysis::YieldResult],
) -> Option<JointSampleSummary> {
    let first = results.first()?;
    let total = first.total_runs;
    if total == 0
        || results
            .iter()
            .any(|result| result.total_runs != total || result.trail.len() != total)
    {
        return None;
    }
    let passing = (0..total)
        .filter(|index| results.iter().all(|result| result.trail[*index]))
        .count();
    Some(JointSampleSummary {
        passing,
        total,
        specification_count: results.len(),
    })
}

/// The worst measurement of a limit's name in a run, against that limit.
///
/// Verify used to take the first name-matching measurement it found in the
/// first attributed analysis. The studio's requirements page takes the worst of
/// every attributed measurement of that name, including the ones a result
/// family's own members retained, because a limit answered by one arbitrary
/// point of a sweep is not a verdict on the sweep. Two surfaces disagreeing
/// about which number answers one limit is not a display difference — one of
/// them is telling an engineer a specification passed while a retained point
/// failed it — so this reduces the same candidate set by the same rule.
fn worst_measurement_in_run(run: &crate::state::SimulationRun, spec: &SpecEntry) -> Option<f64> {
    run.analyses
        .iter()
        .filter(|analysis| verified_analysis(analysis))
        .filter(|analysis| {
            analysis
                .provenance()
                .is_some_and(|provenance| spec.scope.admits(provenance.pvt_point()))
        })
        .flat_map(|analysis| {
            let analysis_level = analysis
                .measurements
                .iter()
                .filter(|measurement| measurement.name.eq_ignore_ascii_case(&spec.measurement))
                .filter(|measurement| measurement.passed && measurement.error.is_none())
                .map(|measurement| measurement.value_in_unit(&spec.unit));
            let member_level = analysis
                .family_metadata
                .iter()
                .flat_map(|metadata| metadata.member_measurements())
                .filter_map(|member| member.evidence_for(&spec.measurement))
                .filter(|evidence| evidence.is_measured())
                .map(|evidence| evidence.value_in_unit(&spec.unit));
            analysis_level.chain(member_level)
        })
        .collect::<Result<Vec<_>, _>>()
        .ok()?
        .into_iter()
        .flatten()
        .filter(|value| value.is_finite())
        // `total_cmp` on a set already filtered finite, so a tie cannot panic.
        .min_by(|left, right| {
            verification_margin(spec, *left).total_cmp(&verification_margin(spec, *right))
        })
}

/// How much room a value has before it breaks the limit. Negative once broken,
/// and more negative the worse the break, so the minimum is the worst point.
fn verification_margin(spec: &SpecEntry, value: f64) -> f64 {
    match (spec.min, spec.max) {
        (Some(minimum), Some(maximum)) => (value - minimum).min(maximum - value),
        (Some(minimum), None) => value - minimum,
        (None, Some(maximum)) => maximum - value,
        (None, None) => 0.0,
    }
}

/// Dimensionless specification margin used for cross-specification ranking.
/// One-sided limits are normalized by the limit magnitude; ranges are
/// normalized by their span. Unbounded tracked values have no comparable
/// margin and are deliberately excluded.
fn normalized_margin(spec: &SpecEntry, value: f64) -> Option<f64> {
    const SCALE_FLOOR: f64 = 1.0e-30;
    match (spec.min, spec.max) {
        (Some(min), Some(max)) if max > min => {
            Some((value - min).min(max - value) / (max - min).max(SCALE_FLOOR))
        }
        (Some(min), None) => Some((value - min) / min.abs().max(SCALE_FLOOR)),
        (None, Some(max)) => Some((max - value) / max.abs().max(SCALE_FLOOR)),
        (Some(_), Some(_)) | (None, None) => None,
    }
}

/// A Spec column's cell: the bound, in the notation of the measured values
/// beside it, and nothing at all for a limit that bounds nothing.
fn limit_cell(spec: &SpecEntry) -> table::Cell {
    if spec.min.is_none() && spec.max.is_none() {
        table::Cell::empty()
    } else {
        table::Cell::number(spec.limit_text())
    }
}

/// A measured value in engineering notation with its unit: four significant
/// digits, an SI prefix, never a bare exponent.
fn format_value(value: f64, unit: &str) -> String {
    rspice_results::number_format::fmt_si_significant(value, unit, 4)
}

fn format_scalar(value: f64) -> String {
    format_value(value, "")
}

/// A difference in engineering notation, signed either way.
fn signed_scalar(value: f64) -> String {
    let sign = if value >= 0.0 { "+" } else { "" };
    format!("{sign}{}", format_scalar(value))
}

fn latest_analysis(
    app: &RSpiceApp,
    analysis_type: crate::state::AnalysisType,
) -> Option<&crate::state::AnalysisResult> {
    app.state.simulation.active_run().and_then(|run| {
        run.analyses
            .iter()
            .rev()
            .find(|analysis| analysis.analysis_type == analysis_type && verified_analysis(analysis))
    })
}

fn verified_analysis(analysis: &crate::state::AnalysisResult) -> bool {
    analysis.success && analysis.provenance.is_some()
}

/// The same analysis, kept only when its retained evidence validates.
///
/// The validator walks every retained sample of every waveform. Asked inside
/// a per-frame filter it re-walked the complete dataset on every frame, for a
/// verdict that is a property of an immutable dataset and so cannot change
/// until the datasets do. The Results workspace owns the memoized answer,
/// keyed by data version and analysis; this asks that owner rather than
/// keeping a second verdict of its own.
fn latest_validated_analysis(
    app: &RSpiceApp,
    analysis_type: crate::state::AnalysisType,
) -> Option<&crate::state::AnalysisResult> {
    let dataset_id = app.state.simulation.active_run()?.dataset_id;
    latest_analysis(app, analysis_type).filter(|analysis| {
        crate::workbench::documents::result_document::analysis_evidence_is_valid(
            &app.state, dataset_id, analysis,
        )
    })
}

fn request_analysis_run(
    app: &mut RSpiceApp,
    kinds: &[crate::simulation::plan::AnalysisKind],
) -> Result<(), String> {
    let mut selected = None;
    let mut plan_changed = false;
    let mut setup = app.state.sim_setup.clone();
    {
        let plan = setup.stable_analysis_plan_mut()?;
        for kind in kinds {
            let existing = plan
                .instances()
                .iter()
                .find(|instance| instance.kind() == *kind)
                .map(|instance| (instance.id(), instance.enabled()));
            let id = if let Some((id, enabled)) = existing {
                if !enabled {
                    plan.set_enabled(id, true)
                        .map_err(|error| error.to_string())?;
                    plan_changed = true;
                }
                id
            } else {
                plan_changed = true;
                plan.insert(*kind).map_err(|error| error.to_string())?.0
            };
            selected = Some((*kind, id));
        }
    }
    setup.refresh_legacy_analysis_projections();
    if plan_changed {
        app.state.sim_setup = setup;
        invalidate_plan_bound_preflight(app);
    }
    if let Some((kind, id)) = selected {
        app.state.workbench.active_analysis = kind.legacy_index();
        app.state.workbench.active_analysis_instance = Some(id);
    }
    crate::workbench::preflight::run_and_queue(app);
    if app.state.simulation.execution.trigger_simulation {
        Ok(())
    } else {
        Err(
            "Execution was not started; review the Simulation preflight report and use its prerequisite repair action."
                .to_owned(),
        )
    }
}

fn open_analysis_configuration(
    app: &mut RSpiceApp,
    kind: crate::simulation::plan::AnalysisKind,
) -> Result<(), String> {
    let mut setup = app.state.sim_setup.clone();
    let (id, plan_changed) = {
        let plan = setup.stable_analysis_plan_mut()?;
        if let Some(instance) = plan
            .instances()
            .iter()
            .find(|instance| instance.kind() == kind)
        {
            (instance.id(), false)
        } else {
            (
                plan.insert(kind).map_err(|error| error.to_string())?.0,
                true,
            )
        }
    };
    setup.refresh_legacy_analysis_projections();
    if plan_changed {
        app.state.sim_setup = setup;
        invalidate_plan_bound_preflight(app);
    }
    app.state.workbench.active_analysis = kind.legacy_index();
    app.state.workbench.active_analysis_instance = Some(id);
    app.state
        .workbench
        .activate(super::super::state::Workspace::Simulate);
    Ok(())
}

/// Invalidate both visible preflight evidence and the controller's retained
/// one-use permit at the same boundary as an authoritative plan mutation.
/// Historical results remain immutable and are intentionally unaffected.
pub(super) fn invalidate_plan_bound_preflight(app: &mut RSpiceApp) {
    app.invalidate_simulation_preflight();
}

#[derive(Debug, Clone, PartialEq)]
struct RegressionCheck {
    target: crate::state::RegressionTargetSelector,
    name: String,
    source_identity: String,
    baseline: f64,
    current: f64,
}

#[derive(Debug, Clone)]
struct RegressionTargetObservation {
    target: crate::state::RegressionTargetSelector,
    validation_error: Option<String>,
}

#[derive(Debug, Clone)]
struct RegressionCoverageIssue {
    target: Option<crate::state::RegressionTargetSelector>,
    label: String,
    detail: String,
}

#[cfg(test)]
mod raster;
#[cfg(test)]
mod tests;
