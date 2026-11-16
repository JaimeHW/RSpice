//! The one way Verify changes the plan's specifications and their policy.
//!
//! Every edit — a name or a limit typed into the table, a specification added
//! or deleted, a property set in the inspector, an acceptance setting —
//! replaces what the active plan holds whole, is validated with the rest of the
//! plan, and is committed as one plan change: one revision, one receipt, one
//! step of the setup's history. The plan-bound preflight is invalidated with
//! it, and the run is judged against the new limits at once, so an edit made
//! after a run changes the verdict without running again.

use rspice_project::ProjectWorkspace;
use rspice_results::number_format::fmt_si_bound;
use rspice_vocabulary::quantity::{
    QuantityInputKind, QuantityPresentationPolicy, UiNumberLocale, parse_ui_quantity,
};

use rspice_results::specification::SpecificationComparison;
use rspice_vocabulary::product::{SimulationPlanId, SpecificationId};

use crate::state::{
    MissingMeasurementPolicy, MonteCarloSpecificationGate, NominalFailurePolicy,
    RegressionSpecificationPolicy, SpecificationDefinition, SpecificationPolicy, VerificationPage,
};
use crate::workbench::app_state::Model;
use crate::workbench::app_state::design_history::SetupParts;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::{SimulateFeature, invalidate_preflight};

/// Commit `change` to the active plan as one plan change, and return the line
/// its receipt states.
///
/// `change` edits a copy of the project and says what it did; the copy is
/// validated and adopted only when the plan takes the change, so a refused
/// edit leaves the plan, its revision and the sheet exactly as they were. A
/// refusal is recorded with the plan's outcomes and returned for the view the
/// edit came from to state.
fn commit_plan(
    cx: &mut Cx<'_, SimulateFeature>,
    change: impl FnOnce(&mut ProjectWorkspace, SimulationPlanId) -> Result<String, String>,
) -> Result<String, String> {
    let outcome: Result<_, String> = (|| {
        let plan_id = cx
            .model
            .sim_setup
            .stable_analysis_plan()
            .map(|plan| plan.id())
            .map_err(|_| "the simulation plan is not ready".to_owned())?;
        let mut workspace = cx.model.workspace.clone();
        let mut setup = cx.model.sim_setup.clone();
        let detail = change(&mut workspace.content, plan_id)?;
        workspace
            .content
            .validate_simulation_configuration()
            .map_err(|error| error.to_string())?;
        let receipt = setup
            .commit_active_plan_configuration_change(detail)
            .map_err(|error| error.to_string())?;
        Ok((workspace, setup, receipt))
    })();
    let (workspace, setup, receipt) = match outcome {
        Ok(committed) => committed,
        Err(error) => {
            let _ = cx.model.record_plan_refusal(error.clone());
            return Err(error);
        }
    };
    let step = cx.model.setup_snapshot(SetupParts::PLAN_DATA);
    cx.model.workspace = workspace;
    cx.model.sim_setup = setup;
    // One step of the setup's history, named for the view that took it.
    cx.model
        .record_setup_change(VerificationPage::Specifications.label(), step);
    invalidate_preflight(cx);
    let line = receipt.status_line();
    cx.model.record_plan_receipt(line.clone());
    let model = &mut *cx.model;
    model
        .simulation
        .refresh_verification(&model.sim_setup, &model.workspace.content);
    Ok(line)
}

/// Replace the active plan's specification definitions with what `change`
/// makes of them, as one plan change.
///
/// `change` is handed the plan's definitions — migrated from its legacy limits
/// when it had only those — and returns the sentence the receipt states.
pub fn edit_specifications(
    cx: &mut Cx<'_, SimulateFeature>,
    change: impl FnOnce(&mut Vec<SpecificationDefinition>) -> Result<String, String>,
) -> Result<String, String> {
    commit_plan(cx, |workspace, plan_id| {
        let mut definitions = workspace
            .ensure_active_plan_data(plan_id)
            .specification_definitions
            .clone();
        let detail = change(&mut definitions)?;
        for definition in &definitions {
            definition
                .validate()
                .map_err(|error| format!("{}: {error}", definition.measurement))?;
        }
        workspace.replace_active_specification_definitions(plan_id, definitions);
        Ok(detail)
    })
}

/// Make `policy` the active plan's acceptance policy, as one plan change.
pub fn edit_policy(
    cx: &mut Cx<'_, SimulateFeature>,
    policy: SpecificationPolicy,
) -> Result<String, String> {
    let detail = format!(
        "Updated specification evaluation policy: nominal {}, missing {}, Monte Carlo {}, regression {}.",
        match policy.nominal_failure {
            NominalFailurePolicy::Block => "blocks acceptance",
            NominalFailurePolicy::RecordDisposition => "records a review disposition",
        },
        match policy.missing_measurement {
            MissingMeasurementPolicy::FailClosed => "fails closed",
            MissingMeasurementPolicy::ReportUnmapped => "reports unmapped for review",
        },
        monte_carlo_gate_label(&policy.monte_carlo),
        match policy.regression {
            RegressionSpecificationPolicy::LimitAndWaveform => "checks limits and waveforms",
            RegressionSpecificationPolicy::LimitOnly => "checks limits only",
        }
    );
    commit_plan(cx, move |workspace, plan_id| {
        policy.validate()?;
        workspace
            .ensure_active_plan_data(plan_id)
            .specification_policy = policy;
        Ok(detail)
    })
}

/// The Monte Carlo gate in words, as the acceptance settings offer it.
pub fn monte_carlo_gate_label(gate: &MonteCarloSpecificationGate) -> String {
    match gate {
        MonteCarloSpecificationGate::NotGated => "Not gated".to_owned(),
        MonteCarloSpecificationGate::YieldAtLeast { percent } => {
            format!("Yield at least {percent}%")
        }
    }
}

/// The active plan's definition of `measurement`, as an edit would find it:
/// the one the plan holds, or the one its legacy limits migrate to, which is
/// the same definition with the same identity.
pub fn plan_definition(model: &Model, measurement: &str) -> Option<SpecificationDefinition> {
    let plan_id = model.sim_setup.stable_analysis_plan().ok()?.id();
    let content = &model.workspace.content;
    match content.plan_data(plan_id) {
        Some(payload) if !payload.specification_definitions.is_empty() => payload
            .specification_definitions
            .iter()
            .find(|definition| definition.measurement.eq_ignore_ascii_case(measurement))
            .cloned(),
        _ => content
            .active_specs(plan_id)
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.measurement.eq_ignore_ascii_case(measurement))
            .map(|(index, entry)| SpecificationDefinition::from_legacy(plan_id, index, entry)),
    }
}

/// The definition a sheet row states: by its identity when the row carries
/// one, otherwise by its measurement, which is unique within a plan.
pub fn definition_index(
    definitions: &[SpecificationDefinition],
    specification: Option<SpecificationId>,
    measurement: &str,
) -> Option<usize> {
    specification
        .and_then(|id| {
            definitions
                .iter()
                .position(|definition| definition.id == id)
        })
        .or_else(|| {
            definitions
                .iter()
                .position(|definition| definition.measurement.eq_ignore_ascii_case(measurement))
        })
}

/// A limit as the table states it: `≥ 40 dB`, `≤ 1.2 mA`, `10 MHz … 20 MHz`,
/// `50 Ω ± 1 Ω`; `None` for a tracked specification, which bounds nothing.
pub fn limit_text(comparison: &SpecificationComparison, unit: &str) -> Option<String> {
    let bound = |value: f64| fmt_si_bound(value, unit);
    match *comparison {
        SpecificationComparison::Tracked => None,
        SpecificationComparison::Minimum { limit } => Some(format!("\u{2265} {}", bound(limit))),
        SpecificationComparison::Maximum { limit } => Some(format!("\u{2264} {}", bound(limit))),
        SpecificationComparison::Range { minimum, maximum } => {
            Some(format!("{} \u{2026} {}", bound(minimum), bound(maximum)))
        }
        SpecificationComparison::EqualWithin { target, tolerance } => {
            Some(format!("{} \u{00b1} {}", bound(target), bound(tolerance)))
        }
    }
}

/// The words a limit cell opens with: what it shows, when typing that back
/// gives the same limit, otherwise every digit of it, which always does.
pub fn limit_spelling(
    comparison: &SpecificationComparison,
    unit: &str,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> String {
    let Some(shown) = limit_text(comparison, unit) else {
        return String::new();
    };
    if parse_limit(&shown, unit, comparison, policy, locale)
        .is_ok_and(|parsed| parsed.bitwise_eq(comparison))
    {
        return shown;
    }
    let value = |value: f64| {
        if unit.is_empty() {
            format!("{value}")
        } else {
            format!("{value} {unit}")
        }
    };
    match *comparison {
        SpecificationComparison::Tracked => String::new(),
        SpecificationComparison::Minimum { limit } => format!(">= {}", value(limit)),
        SpecificationComparison::Maximum { limit } => format!("<= {}", value(limit)),
        SpecificationComparison::Range { minimum, maximum } => {
            format!("{} .. {}", value(minimum), value(maximum))
        }
        SpecificationComparison::EqualWithin { target, tolerance } => {
            format!("{} +/- {}", value(target), value(tolerance))
        }
    }
}

/// What a typed limit says in `unit`.
///
/// `≥ 40 dB` and `>= 40` set a minimum, `≤` and `<=` a maximum, `10M..20M` or
/// `10 MHz … 20 MHz` a range, and `50 ± 1` or `50 +/- 1` a target within a
/// tolerance. A bare value keeps the direction of a one-sided limit it
/// replaces; anything else needs the reader to say which way it bounds. An
/// empty cell, or a dash, leaves the specification tracked. Each value goes
/// through the interactive quantity parser under the reader's own policy, so
/// `10 MHz`, `900m` and `40 dB` read the way every other field reads them.
pub fn parse_limit(
    text: &str,
    unit: &str,
    current: &SpecificationComparison,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> Result<SpecificationComparison, String> {
    let text = text.trim();
    if text.is_empty() || text == "\u{2014}" || text == "-" {
        return Ok(SpecificationComparison::Tracked);
    }
    let value = |text: &str| parse_quantity(text, unit, policy, locale);
    for (prefix, minimum) in [
        ("\u{2265}", true),
        (">=", true),
        (">", true),
        ("\u{2264}", false),
        ("<=", false),
        ("<", false),
    ] {
        if let Some(rest) = text.strip_prefix(prefix) {
            let limit = value(rest)?;
            return Ok(if minimum {
                SpecificationComparison::Minimum { limit }
            } else {
                SpecificationComparison::Maximum { limit }
            });
        }
    }
    for separator in ["\u{2026}", ".."] {
        if let Some((low, high)) = text.split_once(separator) {
            return Ok(SpecificationComparison::Range {
                minimum: value(low)?,
                maximum: value(high)?,
            });
        }
    }
    for separator in ["\u{00b1}", "+/-"] {
        if let Some((target, tolerance)) = text.split_once(separator) {
            return Ok(SpecificationComparison::EqualWithin {
                target: value(target)?,
                tolerance: value(tolerance)?,
            });
        }
    }
    let limit = value(text)?;
    match current {
        SpecificationComparison::Minimum { .. } => Ok(SpecificationComparison::Minimum { limit }),
        SpecificationComparison::Maximum { .. } => Ok(SpecificationComparison::Maximum { limit }),
        _ => Err(format!(
            "say which way {text} bounds: \u{2265} {text} or \u{2264} {text}"
        )),
    }
}

/// One typed value in `unit`, through the interactive quantity parser.
fn parse_quantity(
    text: &str,
    unit: &str,
    policy: QuantityPresentationPolicy,
    locale: UiNumberLocale,
) -> Result<f64, String> {
    let text = text.trim();
    let kind = match unit {
        "s" => QuantityInputKind::Time,
        "Hz" => QuantityInputKind::Frequency,
        _ => QuantityInputKind::EngineeringScalar,
    };
    if kind == QuantityInputKind::EngineeringScalar {
        let source = text.strip_suffix(unit).unwrap_or(text).trim_end();
        return parse_ui_quantity(source, kind, policy, locale).map_err(|error| error.to_string());
    }
    // A time or a frequency is typed in the specification's own unit, so a
    // bare `10M` is ten megahertz the way `10 MHz` is.
    parse_ui_quantity(text, kind, policy, locale)
        .or_else(|error| {
            parse_ui_quantity(&format!("{text}{unit}"), kind, policy, locale).map_err(|_| error)
        })
        .map_err(|error| error.to_string())
}

/// Set `definition`'s limit to `comparison`, and say what changed.
///
/// A tracked specification bounds nothing, so it keeps no guard band.
pub fn set_limit(
    definition: &mut SpecificationDefinition,
    comparison: SpecificationComparison,
) -> String {
    if comparison == SpecificationComparison::Tracked {
        definition.guard_band = None;
    }
    let detail = match limit_text(&comparison, &definition.unit) {
        Some(limit) => format!(
            "Set the limit of {} to {limit}.",
            definition.requirement_name
        ),
        None => format!(
            "Removed the limit of {}; it is tracked.",
            definition.requirement_name
        ),
    };
    definition.comparison = comparison;
    detail
}
