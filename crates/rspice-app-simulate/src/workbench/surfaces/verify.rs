//! Verification evidence, specifications, checks and history.
//!
//! Every Verify page has one grammar, the ADE results view's: a toolbar row
//! (title, the run it reads when there is a choice, one summary line, its
//! actions), one dense table of the run's own values, the detail of a row only
//! when that row is selected, and — when the run holds nothing for the page —
//! one line saying what produces data, with the action that produces it.

mod axes;
pub(crate) mod checks;
mod corners;
mod histogram;
mod monte_carlo;
pub(crate) mod reference;
mod regression_contract;
pub(crate) mod specification_edit;
pub(crate) mod specifications;
pub mod table;
pub mod toolbar;

use egui::Ui;
use sha2::{Digest as _, Sha256};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Problem;
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::{SimulateFeature, invalidate_preflight};
use crate::workbench::slots::SimulateState;

use super::super::state::VerificationSessionState;
use super::super::state::workbench_session::WidthClass;
use crate::state::VerificationPage;

use checks::*;
use corners::*;
use monte_carlo::*;
use reference::reference_view;
use regression_contract::*;
use specifications::specifications;

pub(crate) use corners::{FactSection, point_facts};
pub(crate) use monte_carlo::{campaign_facts, monte_carlo_gate, trial_facts, yields_below_gate};

pub fn show(ui: &mut Ui, cx: &mut Cx<'_, SimulateFeature>) {
    let page = cx.state.verification_page;
    // A refused action is stated under the toolbar of the page it was taken
    // on; another page has no business repeating it.
    let shown_page = egui::Id::new("workbench.verify.shown_page");
    if ui
        .ctx()
        .data(|data| data.get_temp::<VerificationPage>(shown_page))
        != Some(page)
    {
        cx.state.verification.action_problem = None;
        ui.ctx().data_mut(|data| data.insert_temp(shown_page, page));
        if page == VerificationPage::Corners && phone_width(ui.ctx()) {
            open_transposed(cx);
        }
    }
    let t = Tokens::get(ui.ctx());
    let surface = ui.available_rect_before_wrap();
    ui.painter().rect_filled(surface, 0.0, t.color.bg_app);
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
        match page {
            VerificationPage::Specifications => specifications(ui, cx),
            VerificationPage::Corners => corners(ui, cx),
            VerificationPage::MonteCarlo => monte_carlo(ui, cx),
            VerificationPage::Checks => checks_view(ui, cx),
            VerificationPage::Reference => reference_view(ui, cx),
        }
    });
}

/// Whether the window is phone-width, the width the app's own layout turns
/// to bottom navigation and drawers at.
fn phone_width(ctx: &egui::Context) -> bool {
    WidthClass::for_width(ctx.content_rect().width()) == WidthClass::Phone
}

/// Select in the Checks view the SOA check `check` names, as a result's
/// limit line asks; when the run Verify reads holds no such check, say so.
pub fn select_soa_check(
    model: &Model,
    simulate: &mut SimulateState,
    check: &crate::workbench::framework::requests::SoaCheckRef,
) -> Result<(), String> {
    if checks::select_soa_check(model, &mut simulate.verification, check) {
        Ok(())
    } else {
        Err(format!(
            "This run holds no {} check on {}.",
            soa_parameter_display(check.parameter),
            check.device
        ))
    }
}

/// The run the plan compares other runs against, by its identity.
pub fn reference_run(model: &Model, simulate: &SimulateState) -> Option<crate::product::RunId> {
    active_regression_baseline(model, simulate)
}

/// Whether `run` may become the plan's reference run, and why not when it
/// may not.
pub fn reference_eligibility(model: &Model, run: crate::product::RunId) -> Result<(), String> {
    regression_baseline_eligibility(model, run).map(|_| ())
}

/// Make `run` the plan's reference run, as one plan change.
pub fn set_reference_run(
    cx: &mut Cx<'_, SimulateFeature>,
    run: crate::product::RunId,
) -> Result<(), String> {
    commit_regression_baseline(cx, run)
}

/// Record how the action the reader just took went: a refusal is stated under
/// the toolbar, and an action that worked clears it and says nothing.
fn record_action(session: &mut VerificationSessionState, result: Result<(), String>) {
    session.action_problem = result.err();
}

/// What this page could not do, under the toolbar it was asked on: the
/// refusal of the last action, and the reason the run it reads cannot be held
/// against the plan's limits.
fn action_problem_line(ui: &mut Ui, model: &Model, simulate: &SimulateState) {
    if let Some(problem) = &simulate.verification.action_problem {
        let _ = crate::ui::widgets::problem_line(ui, Problem::Failure, problem);
    }
    // `NoDataset` is the empty state, which already says what produces data;
    // repeating it under the same toolbar would be two lines for one fact.
    if verification_run(model).is_some()
        && let Some(refusal) = verification_evidence_domain(model).refusal()
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
fn verification_evidence_domain(model: &Model) -> crate::state::EvidenceDomain {
    model
        .simulation
        .evidence_domain(verification_plan_id(model))
}

fn verification_run_index(model: &Model) -> Option<usize> {
    model
        .simulation
        .view
        .active_run_idx
        .filter(|index| *index < model.simulation.retained.runs.len())
}

fn verification_run(model: &Model) -> Option<&crate::state::SimulationRun> {
    verification_run_index(model).and_then(|index| model.simulation.retained.runs.get(index))
}

/// The plan whose limits this surface reads, when the plan is stable.
fn verification_plan_id(model: &Model) -> Option<crate::product::SimulationPlanId> {
    model
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
}

/// Whether `unit` takes an SI prefix. A level in decibels, a percentage and
/// an angle in degrees do not: half a decibel is `0.5 dB`, never `500 mdB`.
fn takes_si_prefix(unit: &str) -> bool {
    !(unit.starts_with("dB") || matches!(unit, "%" | "deg" | "\u{b0}"))
}

/// A measured value in engineering notation with its unit: four significant
/// digits, an SI prefix where the unit takes one, never a bare exponent.
fn format_value(value: f64, unit: &str) -> String {
    if takes_si_prefix(unit) {
        rspice_results::number_format::fmt_si_significant(value, unit, 4)
    } else {
        rspice_results::number_format::fmt_significant(value, 4, &format!(" {unit}"))
    }
}

/// One bound of a limit in the notation of the values held against it,
/// without the zeros nobody wrote it with: `40 dB`, `1.2 mA`.
fn format_bound(value: f64, unit: &str) -> String {
    if takes_si_prefix(unit) {
        return rspice_results::number_format::fmt_si_bound(value, unit);
    }
    let text = rspice_results::number_format::fmt_significant(value, 4, "");
    // Scientific notation carries its own exponent, and trimming its mantissa
    // would change the number.
    let number = if text.contains('.') && !text.contains(['e', 'E']) {
        text.trim_end_matches('0').trim_end_matches('.')
    } else {
        text.as_str()
    };
    format!("{number} {unit}")
}

fn format_scalar(value: f64) -> String {
    format_value(value, "")
}

/// `value` times ten to the `decades`, by an exact power of ten.
fn shifted(value: f64, decades: i32) -> f64 {
    let factor = 10f64.powi(decades.unsigned_abs().min(300) as i32);
    if decades >= 0 {
        value * factor
    } else {
        value / factor
    }
}

/// The decade of the last digit [`format_value`] prints of `value`: four
/// significant digits end three decades under the leading one, whatever
/// prefix the unit takes. Zero prints to three decimals.
fn last_digit_decade(value: f64) -> i32 {
    if value == 0.0 || !value.is_finite() {
        return -3;
    }
    value.abs().log10().floor() as i32 - 3
}

/// A difference taken from `of` — a margin to a limit, a change from a
/// reference — in the notation of the values beside it and at their
/// resolution: rounded to the last digit `of` is printed with, no more
/// significant digits than a value, and signed either way. A margin of
/// -0.6 dB from 39.40 dB reads `-0.60 dB`, and 40 µA from 1.160 mA `+40 µA`.
fn difference_text(difference: f64, of: f64, unit: &str) -> String {
    let decade = last_digit_decade(of);
    let steps = shifted(difference, -decade).round();
    if !steps.is_finite() {
        return format_value(difference, unit);
    }
    if steps == 0.0 {
        // Nothing at the value's resolution: zero spelled as the value is,
        // on the side the difference falls.
        let sign = if difference > 0.0 {
            "+"
        } else if difference < 0.0 {
            "-"
        } else {
            ""
        };
        let shown = format_value(of, unit);
        let (number, rest) = shown.split_once(' ').unwrap_or((shown.as_str(), ""));
        let (mantissa, exponent) = number
            .find(['e', 'E'])
            .map_or((number, ""), |at| number.split_at(at));
        let decimals = mantissa
            .split_once('.')
            .map_or(0, |(_, digits)| digits.len());
        let space = if rest.is_empty() { "" } else { " " };
        return format!("{sign}{:.decimals$}{exponent}{space}{rest}", 0.0);
    }
    let digits = (steps.abs().log10().floor() as usize + 1).min(4);
    let rounded = shifted(steps, decade);
    let sign = if rounded > 0.0 { "+" } else { "" };
    let text = if takes_si_prefix(unit) {
        rspice_results::number_format::fmt_si_significant(rounded, unit, digits)
    } else {
        rspice_results::number_format::fmt_significant(rounded, digits, &format!(" {unit}"))
    };
    format!("{sign}{}", text.trim_end())
}

fn verified_analysis(analysis: &crate::state::AnalysisResult) -> bool {
    analysis.success && analysis.provenance.is_some()
}

fn open_analysis_configuration(
    cx: &mut Cx<'_, SimulateFeature>,
    kind: crate::simulation::plan::AnalysisKind,
) -> Result<(), String> {
    let mut setup = cx.model.sim_setup.clone();
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
        cx.model.sim_setup = setup;
        invalidate_plan_bound_preflight(cx);
    }
    cx.state.active_analysis = kind.singleton_index();
    cx.state.active_analysis_instance = Some(id);
    cx.wb.activate(super::super::state::Workspace::Simulate);
    Ok(())
}

/// Invalidate both visible preflight evidence and the controller's retained
/// one-use permit at the same boundary as an authoritative plan mutation.
/// Historical results remain immutable and are intentionally unaffected.
pub(super) fn invalidate_plan_bound_preflight(cx: &mut Cx<'_, SimulateFeature>) {
    invalidate_preflight(cx);
}

#[derive(Debug, Clone, PartialEq)]
struct RegressionCheck {
    target: crate::state::RegressionTargetSelector,
    name: String,
    source_identity: String,
    baseline: f64,
    current: f64,
    /// The unit the producer states the measured value in; empty when it
    /// states none.
    unit: String,
}

#[derive(Debug, Clone)]
struct RegressionTargetObservation {
    target: crate::state::RegressionTargetSelector,
    validation_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct RegressionCoverageIssue {
    target: Option<crate::state::RegressionTargetSelector>,
    label: String,
    detail: String,
}

#[cfg(any(test, feature = "test-support"))]
pub mod raster;
#[cfg(test)]
pub(crate) mod tests;
