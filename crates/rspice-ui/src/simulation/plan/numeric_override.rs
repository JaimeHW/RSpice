//! Per-analysis departures from the plan's solver policy.
//!
//! The plan states one numerical policy for the whole deck, emitted as a single
//! `.OPTIONS` block. An analysis that must resolve differently records only the
//! keys it departs on; every key it leaves absent still resolves to the plan.
//!
//! The record reaches the engine as a second `.OPTIONS` block spliced into that
//! task's own deck. The netlist parser accumulates option cards into one set
//! with plain assignment, so the later card wins per key — which is what makes
//! "inherit unless stated" work without either block knowing about the other.
//!
//! Which options exist, and the engine site that reads each one, is
//! [`catalog`]. Nothing here decides what may be authored; this module decides
//! how an authored value is validated, stored, emitted and reported.

mod catalog;

use serde::{Deserialize, Serialize};

use crate::simulation::dialog::{
    DampingStrategy, IntegrationMethod, MatrixSolver, format_si_value, parse_si_value,
};

use super::AnalysisKind;

pub use catalog::{
    NumericOverrideOption, OptionPackage, OverrideSection, OverrideValue, OverrideValueKind,
};

/// One analysis's numerical departures. Absent means "inherit the plan".
///
/// Every field is `Option` and every one is serde-defaulted, so a project
/// written before an option existed still opens and simply states nothing for
/// it. The nine fields above `gmin` carry the names they were first persisted
/// under and must keep them.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AnalysisNumericOverride {
    reltol: Option<f64>,
    abstol: Option<f64>,
    vntol: Option<f64>,
    residual_reltol: Option<f64>,
    itl1: Option<usize>,
    itl4: Option<usize>,
    trtol: Option<f64>,
    integration_method: Option<IntegrationMethod>,
    max_timestep: Option<f64>,
    gmin: Option<f64>,
    gmin_stepping: Option<bool>,
    source_stepping: Option<bool>,
    pseudo_transient: Option<bool>,
    arc_length: Option<bool>,
    damping: Option<DampingStrategy>,
    chgtol: Option<f64>,
    lte_reltol: Option<f64>,
    lte_abstol: Option<f64>,
    min_timestep: Option<f64>,
    pivrel: Option<f64>,
    pivtol: Option<f64>,
    solver: Option<MatrixSolver>,
    bypass: Option<bool>,
    bypass_reltol: Option<f64>,
    bypass_abstol: Option<f64>,
}

impl AnalysisNumericOverride {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    /// The typed value this record states for one option, if it states one.
    ///
    /// One match, and every other reader — the emitter, the digest, the
    /// ledger — goes through it. A new option that forgets an arm here fails
    /// to compile rather than emitting nothing.
    #[must_use]
    pub fn stated(&self, option: NumericOverrideOption) -> Option<OverrideValue> {
        use NumericOverrideOption as O;
        Some(match option {
            O::Reltol => OverrideValue::Real(self.reltol?),
            O::Abstol => OverrideValue::Real(self.abstol?),
            O::Vntol => OverrideValue::Real(self.vntol?),
            O::ResidualReltol => OverrideValue::Real(self.residual_reltol?),
            O::Gmin => OverrideValue::Real(self.gmin?),
            O::Itl1 => OverrideValue::Count(self.itl1?),
            O::Itl4 => OverrideValue::Count(self.itl4?),
            O::GminStepping => OverrideValue::Flag(self.gmin_stepping?),
            O::SourceStepping => OverrideValue::Flag(self.source_stepping?),
            O::PseudoTransient => OverrideValue::Flag(self.pseudo_transient?),
            O::ArcLength => OverrideValue::Flag(self.arc_length?),
            O::Damping => OverrideValue::Damping(self.damping?),
            O::Chgtol => OverrideValue::Real(self.chgtol?),
            O::Trtol => OverrideValue::Real(self.trtol?),
            O::IntegrationMethod => OverrideValue::Method(self.integration_method?),
            O::LteReltol => OverrideValue::Real(self.lte_reltol?),
            O::LteAbstol => OverrideValue::Real(self.lte_abstol?),
            O::MinTimestep => OverrideValue::Real(self.min_timestep?),
            O::MaximumTimestep => OverrideValue::Real(self.max_timestep?),
            O::Pivrel => OverrideValue::Real(self.pivrel?),
            O::Pivtol => OverrideValue::Real(self.pivtol?),
            O::Solver => OverrideValue::Solver(self.solver?),
            O::Bypass => OverrideValue::Flag(self.bypass?),
            O::BypassReltol => OverrideValue::Real(self.bypass_reltol?),
            O::BypassAbstol => OverrideValue::Real(self.bypass_abstol?),
        })
    }

    /// Write one option's slot, or clear it.
    fn store(&mut self, option: NumericOverrideOption, value: Option<OverrideValue>) {
        use NumericOverrideOption as O;
        let real = || match value {
            Some(OverrideValue::Real(value)) => Some(value),
            _ => None,
        };
        let count = || match value {
            Some(OverrideValue::Count(value)) => Some(value),
            _ => None,
        };
        let flag = || match value {
            Some(OverrideValue::Flag(value)) => Some(value),
            _ => None,
        };
        match option {
            O::Reltol => self.reltol = real(),
            O::Abstol => self.abstol = real(),
            O::Vntol => self.vntol = real(),
            O::ResidualReltol => self.residual_reltol = real(),
            O::Gmin => self.gmin = real(),
            O::Itl1 => self.itl1 = count(),
            O::Itl4 => self.itl4 = count(),
            O::GminStepping => self.gmin_stepping = flag(),
            O::SourceStepping => self.source_stepping = flag(),
            O::PseudoTransient => self.pseudo_transient = flag(),
            O::ArcLength => self.arc_length = flag(),
            O::Damping => {
                self.damping = match value {
                    Some(OverrideValue::Damping(strategy)) => Some(strategy),
                    _ => None,
                };
            }
            O::Chgtol => self.chgtol = real(),
            O::Trtol => self.trtol = real(),
            O::IntegrationMethod => {
                self.integration_method = match value {
                    Some(OverrideValue::Method(method)) => Some(method),
                    _ => None,
                };
            }
            O::LteReltol => self.lte_reltol = real(),
            O::LteAbstol => self.lte_abstol = real(),
            O::MinTimestep => self.min_timestep = real(),
            O::MaximumTimestep => self.max_timestep = real(),
            O::Pivrel => self.pivrel = real(),
            O::Pivtol => self.pivtol = real(),
            O::Solver => {
                self.solver = match value {
                    Some(OverrideValue::Solver(solver)) => Some(solver),
                    _ => None,
                };
            }
            O::Bypass => self.bypass = flag(),
            O::BypassReltol => self.bypass_reltol = real(),
            O::BypassAbstol => self.bypass_abstol = real(),
        }
    }

    /// Every option this record states, in catalog order.
    #[must_use]
    pub fn entries(&self) -> Vec<(NumericOverrideOption, String)> {
        NumericOverrideOption::all()
            .filter_map(|option| self.value(option).map(|value| (option, value)))
            .collect()
    }

    /// The stated value of one option, formatted as the ledger reports it.
    #[must_use]
    pub fn value(&self, option: NumericOverrideOption) -> Option<String> {
        let stated = self.stated(option)?;
        Some(match stated {
            // A step bound is a physical quantity, so it reads in the same
            // engineering notation as the transient form's own Max step
            // rather than as a bare exponent.
            OverrideValue::Real(value)
                if matches!(
                    option,
                    NumericOverrideOption::MaximumTimestep | NumericOverrideOption::MinTimestep
                ) =>
            {
                format_si_value(value)
            }
            OverrideValue::Real(value) => format!("{value:e}"),
            OverrideValue::Count(value) => value.to_string(),
            OverrideValue::Flag(value) => if value { "on" } else { "off" }.to_owned(),
            OverrideValue::Method(method) => method.spice_name().to_owned(),
            OverrideValue::Damping(strategy) => strategy.display_name().to_owned(),
            OverrideValue::Solver(solver) => solver.display_name().to_owned(),
        })
    }

    /// Record one authored option, or refuse it with the reason.
    ///
    /// The kind is required because applicability is part of the value being
    /// stored: a record that accepted an inapplicable option would persist a
    /// bound no solve reads.
    pub fn set(
        &mut self,
        kind: AnalysisKind,
        option: NumericOverrideOption,
        authored: &str,
    ) -> Result<(), String> {
        if let Some(reason) = option.refusal_for(kind) {
            return Err(format!(
                "{} cannot carry {}: {reason}.",
                kind.label(),
                option.key()
            ));
        }
        let value = parse_authored(option, authored)?;
        self.store(option, Some(value));
        Ok(())
    }

    /// Stop stating one option, so it resolves to the plan again.
    pub fn clear(&mut self, option: NumericOverrideOption) {
        self.store(option, None);
    }

    /// The first option this record states that the kind cannot carry.
    ///
    /// Restored projects and cloned analyses reach the plan without passing
    /// through [`Self::set`], so the gate is re-checked wherever a record and a
    /// kind are bound together.
    #[must_use]
    pub fn first_refusal_for(
        &self,
        kind: AnalysisKind,
    ) -> Option<(NumericOverrideOption, &'static str)> {
        self.entries()
            .into_iter()
            .find_map(|(option, _)| option.refusal_for(kind).map(|reason| (option, reason)))
    }

    /// Emit the `.OPTIONS` cards this record adds to its analysis's deck.
    ///
    /// Empty when nothing is stated, so a caller can splice unconditionally and
    /// still leave an inheriting analysis's deck byte-identical.
    ///
    /// One card per package, because the parser's package selector stays in
    /// force for the rest of the command it appears on: a `TIMEINT` key placed
    /// among the global ones would re-scope every key after it.
    #[must_use]
    pub fn to_spice_options(&self) -> String {
        let mut cards = Vec::new();
        for package in [OptionPackage::Global, OptionPackage::Timeint] {
            let header = match package {
                OptionPackage::Global => ".OPTIONS",
                OptionPackage::Timeint => ".OPTIONS TIMEINT",
            };
            let mut lines = vec![header.to_owned()];
            for option in NumericOverrideOption::all() {
                let spec = option.spec();
                if spec.package != package {
                    continue;
                }
                let Some(text) = self.stated(option).and_then(OverrideValue::to_deck_text) else {
                    continue;
                };
                lines.push(format!("+ {}={text}", spec.key));
            }
            if lines.len() > 1 {
                cards.push(lines.join("\n"));
            }
        }
        cards.join("\n")
    }
}

fn parse_authored(option: NumericOverrideOption, authored: &str) -> Result<OverrideValue, String> {
    match option.value_kind() {
        OverrideValueKind::PositiveReal => positive_real(option, authored).map(OverrideValue::Real),
        OverrideValueKind::NonNegativeReal => {
            non_negative_real(option, authored).map(OverrideValue::Real)
        }
        OverrideValueKind::IterationCount => {
            iteration_budget(option, authored).map(OverrideValue::Count)
        }
        OverrideValueKind::Flag => flag(option, authored).map(OverrideValue::Flag),
        OverrideValueKind::Method => integration_method(authored).map(OverrideValue::Method),
        OverrideValueKind::Damping => damping_strategy(authored).map(OverrideValue::Damping),
        OverrideValueKind::Solver => matrix_solver(authored).map(OverrideValue::Solver),
    }
}

fn positive_real(option: NumericOverrideOption, authored: &str) -> Result<f64, String> {
    let value = parse_si_value(authored).map_err(|error| format!("{}: {error}", option.key()))?;
    if !value.is_finite() || value <= 0.0 {
        return Err(format!("{} must be a positive real value", option.key()));
    }
    Ok(value)
}

/// A floor of exactly zero is a real request: `GMIN=0` asks for no junction
/// conductance floor at all, which is what a deck checking for a genuinely
/// singular operating point wants.
fn non_negative_real(option: NumericOverrideOption, authored: &str) -> Result<f64, String> {
    let value = parse_si_value(authored).map_err(|error| format!("{}: {error}", option.key()))?;
    if !value.is_finite() || value < 0.0 {
        return Err(format!("{} cannot be negative", option.key()));
    }
    Ok(value)
}

fn iteration_budget(option: NumericOverrideOption, authored: &str) -> Result<usize, String> {
    let value: usize = authored
        .trim()
        .parse()
        .map_err(|_| format!("{} must be a whole iteration count", option.key()))?;
    if value == 0 {
        return Err(format!(
            "{} must allow at least one iteration",
            option.key()
        ));
    }
    Ok(value)
}

/// The spellings the deck carries are `1` and `0`, but a person authoring one
/// here types a word. Both are accepted; only the digit is emitted.
fn flag(option: NumericOverrideOption, authored: &str) -> Result<bool, String> {
    let authored = authored.trim();
    for (spelling, value) in [
        ("1", true),
        ("on", true),
        ("true", true),
        ("yes", true),
        ("0", false),
        ("off", false),
        ("false", false),
        ("no", false),
    ] {
        if authored.eq_ignore_ascii_case(spelling) {
            return Ok(value);
        }
    }
    Err(format!("{} must be on or off", option.key()))
}

/// Every method the plan-level chooser offers is one the netlist parser can
/// select, so the chooser's own list is the authority here. Keeping a second
/// list would let the two drift, and a method authored here that the parser
/// cannot read would leave the solve on whatever method it already had —
/// exactly the silently ignored bound this record exists to prevent.
fn integration_method(authored: &str) -> Result<IntegrationMethod, String> {
    let authored = authored.trim();
    IntegrationMethod::all()
        .iter()
        .copied()
        .find(|method| {
            method.spice_name().eq_ignore_ascii_case(authored)
                || method.display_name().eq_ignore_ascii_case(authored)
        })
        .ok_or_else(|| {
            format!(
                "METHOD must name an integration method the deck can select: {}",
                IntegrationMethod::all()
                    .iter()
                    .map(|method| method.spice_name())
                    .collect::<Vec<_>>()
                    .join(" · ")
            )
        })
}

fn damping_strategy(authored: &str) -> Result<DampingStrategy, String> {
    let authored = authored.trim();
    DampingStrategy::all()
        .iter()
        .copied()
        .find(|strategy| {
            strategy.spice_name().eq_ignore_ascii_case(authored)
                || strategy.display_name().eq_ignore_ascii_case(authored)
        })
        .ok_or_else(|| {
            format!(
                "DAMPING must name a strategy the deck can select: {}",
                DampingStrategy::all()
                    .iter()
                    .map(|strategy| strategy.spice_name())
                    .collect::<Vec<_>>()
                    .join(" · ")
            )
        })
}

/// Only a backend the deck can actually name.
///
/// [`MatrixSolver::Lu`] is the plan page's *automatic* setting and deliberately
/// emits no `SOLVER` key, so that the dialect and matrix profile keep choosing.
/// Storing it here would be a departure that departs from nothing — the record
/// would report an override the deck never carries — so it is refused, and
/// removing the option is how an analysis returns to automatic.
fn matrix_solver(authored: &str) -> Result<MatrixSolver, String> {
    let authored = authored.trim();
    MatrixSolver::all()
        .iter()
        .copied()
        .filter(|solver| solver.spice_name().is_some())
        .find(|solver| {
            solver
                .spice_name()
                .is_some_and(|name| name.eq_ignore_ascii_case(authored))
                || solver.display_name().eq_ignore_ascii_case(authored)
        })
        .ok_or_else(|| {
            format!(
                "SOLVER must name an explicit backend: {}. Remove the override to return this \
                 analysis to automatic backend selection.",
                MatrixSolver::all()
                    .iter()
                    .filter_map(|solver| solver.spice_name())
                    .collect::<Vec<_>>()
                    .join(" · ")
            )
        })
}

#[cfg(test)]
mod tests;
