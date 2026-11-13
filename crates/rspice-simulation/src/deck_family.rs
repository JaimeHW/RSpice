//! The members of a deck's Monte Carlo study or `.STEP` sweep.
//!
//! A deck that pairs its `.TRAN` card with a `.MC` study, or a swept analysis
//! (`.DC`, `.AC`, `.NOISE`, `.TRAN`) with a `.STEP` sweep, runs as one task
//! per trial or point, each naming its member ([`FamilyMember`]) and each
//! keeping its full result: one curve per member, the family LTspice and
//! QSPICE draw. This module reads the members off the deck without solving
//! anything, so every route that lowers such a deck — the application's
//! manual-deck run and the corpus runner's headless cases — lowers it to the
//! same members:
//!
//! - a trial is one index of the card's own numbering, from its first trial,
//!   varied as the deck states its variation (its own statistical functions,
//!   or the card's spread over the parameters it names);
//! - a point is one coordinate of the deck plan, in the plan's order (the
//!   first `.STEP` card varies fastest), with the value each axis takes there.
//!   Nested `.STEP` cards, `.STEP TEMP` among them, are the cross product of
//!   their points.
//!
//! [`member_parameters`] then reads what each member's circuit resolves its
//! top-level `.param` values to — the draw a trial makes, the coordinate a
//! point takes — through the engine's own sampler and sweep materializer.

use std::collections::BTreeMap;
use std::sync::Mutex;

use rspice_core::netlist::{AnalysisCommand, MonteCarloCommand, MonteCarloDistribution};
use rspice_core::{Engine, Netlist, NoAbort, ResourceLimits};
use rspice_simulation_contract::mc_draft::McVariationSource;

use crate::execution_options::FamilyMember;

/// The seed a `.MC` card that states none runs with on the prepared route,
/// the configured Monte Carlo study's own default.
pub(crate) const DEFAULT_MONTE_CARLO_SEED: u64 = 0x5EED_5EED;

/// Statistical functions a deck may state its Monte Carlo variation with.
const STATISTICAL_FUNCTIONS: [&str; 5] = ["agauss(", "aunif(", "gauss(", "unif(", "limit("];

/// One member of a deck's Monte Carlo study or `.STEP` sweep.
#[derive(Debug, Clone, PartialEq)]
pub struct DeckFamilyMember {
    /// The member a transient task names.
    pub member: FamilyMember,
    /// For a `.STEP` point, the value each swept axis takes there, in axis
    /// order: `("r", 2000.0)` for `.STEP PARAM r LIST 1k 2k`, every column
    /// of the row for a `.STEP DATA` table. Empty for a Monte Carlo trial.
    pub coordinate: Vec<(String, f64)>,
}

/// Whether `deck` states a statistical function outside comments.
///
/// `deck` must already hold every file it includes: a statistic stated in an
/// include the caller has not expanded is not seen.
#[must_use]
pub fn deck_states_statistics(deck: &str) -> bool {
    deck.lines().any(|line| {
        let code = line.split(['$', ';']).next().unwrap_or_default().trim();
        !code.starts_with('*') && {
            let lower = code.to_ascii_lowercase();
            STATISTICAL_FUNCTIONS
                .iter()
                .any(|function| lower.contains(function))
        }
    })
}

/// The deck's one `.MC` card.
fn monte_carlo_card(netlist: &Netlist) -> Result<&MonteCarloCommand, String> {
    netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::MonteCarlo(card) => Some(card),
            _ => None,
        })
        .ok_or_else(|| "the deck has no .MC card".to_owned())
}

/// How the trials of `card` are varied.
///
/// A deck that states its variation with statistical functions, in itself or
/// a file it includes, is redrawn from them. A deck that states none and
/// whose card names no parameter would have every `.param` perturbed by the
/// card's default spread, a supply included; that is refused, never run as if
/// it were the design's variation.
pub fn monte_carlo_variation_source(
    card: &MonteCarloCommand,
    states_statistics: bool,
) -> Result<McVariationSource, String> {
    if states_statistics {
        Ok(McVariationSource::DeckStatistics)
    } else if card.params.is_empty() {
        Err("the .mc card names no parameter and neither the deck nor any file it includes states a statistical function (agauss, aunif, gauss, unif, limit); refusing to perturb every .param".to_owned())
    } else {
        Ok(McVariationSource::ParameterTolerance)
    }
}

/// The study that draws trials `first_trial ..` of `card` as the engine's
/// Monte Carlo transient draws them: trial `k` of a study of one starting at
/// `k` draws what trial `k` of the whole study draws.
pub(crate) fn monte_carlo_study(
    card: &MonteCarloCommand,
    variation_source: McVariationSource,
    first_trial: usize,
    runs: usize,
) -> rspice_core::engine::MonteCarloStudyConfig {
    use rspice_core::analysis::Distribution;
    use rspice_core::engine::{MonteCarloStudyConfig, MonteCarloVariationSource};
    let mut study = MonteCarloStudyConfig::new(
        runs,
        card.seed.unwrap_or(DEFAULT_MONTE_CARLO_SEED),
        vec!["trial".to_owned()],
    );
    study.first_trial = first_trial;
    study.distribution = match card.distribution {
        MonteCarloDistribution::Gaussian => Distribution::Gaussian {
            sigma: card.relative_spread,
        },
        MonteCarloDistribution::Uniform => Distribution::Uniform {
            tolerance: card.relative_spread,
        },
        MonteCarloDistribution::WorstCase => Distribution::WorstCase {
            tolerance: card.relative_spread,
        },
    };
    study.variation_source = match variation_source {
        McVariationSource::ParameterTolerance => MonteCarloVariationSource::ParameterTolerance,
        McVariationSource::DeckStatistics => MonteCarloVariationSource::DeckStatistics,
    };
    study.parameter_filter = card.params.clone();
    study
}

/// The trials of the deck's `.MC` study, in the card's own numbering.
///
/// `states_statistics` is [`deck_states_statistics`] of the deck with its
/// includes expanded.
pub fn monte_carlo_members(
    netlist: &Netlist,
    states_statistics: bool,
) -> Result<Vec<DeckFamilyMember>, String> {
    let card = monte_carlo_card(netlist)?;
    if card.runs == 0 {
        return Err("the .mc card asks for no trials".to_owned());
    }
    let variation_source = monte_carlo_variation_source(card, states_statistics)?;
    let end = card
        .first_trial
        .checked_add(card.runs)
        .ok_or_else(|| "the .mc card's trial numbers overflow".to_owned())?;
    Ok((card.first_trial..end)
        .map(|index| DeckFamilyMember {
            member: FamilyMember::MonteCarloTrial {
                index,
                variation_source,
            },
            coordinate: Vec::new(),
        })
        .collect())
}

/// The points of the deck's `.STEP` sweep, in the deck plan's order, each
/// with the value every swept axis takes there.
///
/// A `.TEMP` card is an axis of the plan too. Listing several temperatures
/// it sweeps them, and its value names each point as `temp`, the way a
/// `.STEP TEMP` names it; one temperature sweeps nothing, so it names none.
pub fn step_members(
    netlist: &Netlist,
    limits: &ResourceLimits,
) -> Result<Vec<DeckFamilyMember>, String> {
    use rspice_core::execution::{AxisKind, DeckPlan, RunAxisValue};
    let plan = DeckPlan::from_netlist(netlist, limits)
        .map_err(|error| format!("the .step plan: {error}"))?;
    if !plan
        .axes()
        .iter()
        .any(|axis| axis.step_target().is_some() || axis.kind() == AxisKind::Data)
    {
        return Err("the deck has no .STEP card".to_owned());
    }
    let coordinates = plan
        .coordinates_with_abort(limits, &NoAbort)
        .map_err(|error| format!("the .step plan: {error}"))?;
    coordinates
        .iter()
        .enumerate()
        .map(|(index, coordinate)| {
            if coordinate.assignments().len() != plan.axes().len() {
                return Err(format!(
                    "point {index} assigns {} axes of the plan's {}",
                    coordinate.assignments().len(),
                    plan.axes().len()
                ));
            }
            let mut values = Vec::new();
            for (assignment, axis) in coordinate.assignments().iter().zip(plan.axes()) {
                let swept = axis.step_target().is_some() || axis.values().len() > 1;
                match assignment.value() {
                    RunAxisValue::Numeric(value) if swept => {
                        values.push((axis_label(assignment), *value));
                    }
                    RunAxisValue::Numeric(_) => {}
                    RunAxisValue::DataRow(bindings) => values.extend(
                        bindings
                            .iter()
                            .map(|binding| (binding.name().to_owned(), binding.value())),
                    ),
                    RunAxisValue::AlterVariant { label, .. } => {
                        return Err(format!(
                            "point {index} is the .ALTER variant {label}, which a family member cannot solve"
                        ));
                    }
                }
            }
            Ok(DeckFamilyMember {
                member: FamilyMember::StepPoint { index },
                coordinate: values,
            })
        })
        .collect()
}

/// A swept axis named the way the deck names what it sweeps: `r` for
/// `.STEP PARAM r`, `v1` for a source, `2n2222(bf)` for a model parameter.
fn axis_label(assignment: &rspice_core::execution::AxisAssignment) -> String {
    use rspice_core::execution::StepAxisTarget;
    match assignment.step_target() {
        Some(StepAxisTarget::Parameter { name })
        | Some(StepAxisTarget::Device {
            name,
            parameter: None,
        }) => name.clone(),
        Some(StepAxisTarget::Device {
            name,
            parameter: Some(parameter),
        })
        | Some(StepAxisTarget::Model { name, parameter }) => format!("{name}({parameter})"),
        Some(StepAxisTarget::Temperature) => "temp".to_owned(),
        None if assignment.kind() == rspice_core::execution::AxisKind::Temperature => {
            "temp".to_owned()
        }
        None => assignment.name().to_owned(),
    }
}

/// Every finite top-level `.param` value each member's circuit resolves,
/// by lowercase name, in member order: the values a trial draws or a point
/// takes, materialized by the engine's own sampler and sweep materializer
/// without solving.
pub fn member_parameters(
    engine: &Engine,
    netlist: &Netlist,
    members: &[DeckFamilyMember],
) -> Result<Vec<BTreeMap<String, f64>>, String> {
    let resolved = |circuit: &Netlist| {
        circuit
            .params
            .all_params()
            .into_iter()
            .filter(|(_, value)| value.is_finite())
            .map(|(name, value)| (name.to_ascii_lowercase(), value))
            .collect::<BTreeMap<_, _>>()
    };
    let trials = members
        .iter()
        .filter_map(|member| match member.member {
            FamilyMember::MonteCarloTrial {
                index,
                variation_source,
            } => Some((index, variation_source)),
            FamilyMember::StepPoint { .. } => None,
        })
        .collect::<Vec<_>>();
    let mut drawn = BTreeMap::new();
    if let (Some(&(first, variation_source)), Some(&(last, _))) = (trials.first(), trials.last()) {
        let card = monte_carlo_card(netlist)?;
        let study = monte_carlo_study(card, variation_source, first, last + 1 - first);
        let draws: Mutex<BTreeMap<usize, BTreeMap<String, f64>>> = Mutex::new(BTreeMap::new());
        engine
            .run_monte_carlo_measurements_with_abort(
                netlist,
                &study,
                &NoAbort,
                |_, trial, index, _| {
                    draws
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner())
                        .insert(index, resolved(trial));
                    Ok(vec![0.0])
                },
            )
            .map_err(|error| format!("the .mc study's draws: {error}"))?;
        drawn = draws
            .into_inner()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
    }
    let materializer = if members
        .iter()
        .any(|member| matches!(member.member, FamilyMember::StepPoint { .. }))
    {
        let limits = engine.config().resource_limits;
        let plan = rspice_core::execution::DeckPlan::from_netlist(netlist, &limits)
            .map_err(|error| format!("the .step plan: {error}"))?;
        Some(
            engine
                .prepare_deck_plan_materializer_with_abort(netlist, &plan, &NoAbort)
                .map_err(|error| format!("the .step plan: {error}"))?,
        )
    } else {
        None
    };
    members
        .iter()
        .map(|member| match member.member {
            FamilyMember::MonteCarloTrial { index, .. } => drawn
                .remove(&index)
                .ok_or_else(|| format!("trial {index} drew no parameters")),
            FamilyMember::StepPoint { index } => {
                let Some(materializer) = materializer.as_ref() else {
                    return Err(format!("point {index} has no .STEP plan"));
                };
                let run = materializer
                    .materialize_run_with_abort(index, &NoAbort)
                    .map_err(|error| format!("materialize point {index}: {error}"))?;
                let (_, point, _, _) = run.into_parts();
                Ok(resolved(&point))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests;
