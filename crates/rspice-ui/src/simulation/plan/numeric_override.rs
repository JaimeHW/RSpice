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
    DampingStrategy, HbTimeDomainMode, IntegrationMethod, MatrixSolver, parse_si_value,
};

use super::AnalysisKind;

// `OptionReach` is deliberately absent: which solves read an option is the
// catalog's own business, and every caller asks the question it actually has —
// `refusal_for`, which answers with the sentence a reader needs.
pub use catalog::{
    NumericOverrideOption, OptionPackage, OverrideSection, OverrideValue, OverrideValueKind,
    SolverOwnership,
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
    transient_newton_reltol: Option<f64>,
    transient_newton_abstol: Option<f64>,
    transient_newton_update_bound: Option<f64>,
    transient_newton_residual_bound: Option<f64>,
    transient_newton_budget: Option<usize>,
    transient_device_convergence: Option<bool>,
    transient_nox_solver: Option<bool>,
    strobe_interval: Option<f64>,
    /// Absent and empty are the same state, so the emitter and [`Self::stated`]
    /// both read an empty list as "not stated": a record holding `Some(vec![])`
    /// would emit a key with no times after it, which the parser reads as a
    /// syntax error rather than as a schedule.
    output_time_points: Option<Vec<f64>>,
    retain_every_signal: Option<bool>,
    hb_initial_state: Option<HbTimeDomainMode>,
}

impl AnalysisNumericOverride {
    /// Apply a base analysis's options after the enclosing study's defaults.
    pub(crate) fn with_base_options(mut self, base: &Self) -> Self {
        for option in NumericOverrideOption::all() {
            if let Some(value) = base.stated(option) {
                if option == NumericOverrideOption::OutputTimePoints {
                    // The parser accumulates exact output times across cards.
                    if let OverrideValue::TimeList(ref points) = value {
                        let times = self.output_time_points.get_or_insert_with(Vec::new);
                        times.extend(points.iter().copied());
                        continue;
                    }
                }
                self.store(option, Some(value));
            }
        }
        self
    }

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
            O::TransientNewtonReltol => OverrideValue::Real(self.transient_newton_reltol?),
            O::TransientNewtonAbstol => OverrideValue::Real(self.transient_newton_abstol?),
            O::TransientNewtonUpdateBound => {
                OverrideValue::Real(self.transient_newton_update_bound?)
            }
            O::TransientNewtonResidualBound => {
                OverrideValue::Real(self.transient_newton_residual_bound?)
            }
            O::TransientNewtonBudget => OverrideValue::Count(self.transient_newton_budget?),
            O::TransientDeviceConvergence => {
                OverrideValue::Flag(self.transient_device_convergence?)
            }
            O::TransientNoxSolver => OverrideValue::Flag(self.transient_nox_solver?),
            O::StrobeInterval => OverrideValue::Real(self.strobe_interval?),
            O::OutputTimePoints => {
                // The strobe interval wins when a record somehow holds both.
                // `set_for_instance` refuses the pair, but a hand-edited
                // project file does not pass through it, and a record that
                // reported both would emit a card the engine's parser refuses
                // — turning an over-specified schedule into a broken netlist.
                // One of the two has to lose, deterministically, and it is
                // stated here rather than in each of the four readers.
                if self.strobe_interval.is_some() {
                    return None;
                }
                let times = self.output_time_points.as_ref()?;
                if times.is_empty() {
                    return None;
                }
                OverrideValue::TimeList(times.clone())
            }
            O::RetainEverySignal => OverrideValue::Flag(self.retain_every_signal?),
            O::HbInitialState => OverrideValue::TimeDomainMode(self.hb_initial_state?),
        })
    }

    /// Write one option's slot, or clear it.
    fn store(&mut self, option: NumericOverrideOption, value: Option<OverrideValue>) {
        use NumericOverrideOption as O;
        // Borrowed rather than copied: one value kind carries a list, so the
        // enum is `Clone` and not `Copy`, and three readers of one `value`
        // cannot each take it.
        let real = || match &value {
            Some(OverrideValue::Real(value)) => Some(*value),
            _ => None,
        };
        let count = || match &value {
            Some(OverrideValue::Count(value)) => Some(*value),
            _ => None,
        };
        let flag = || match &value {
            Some(OverrideValue::Flag(value)) => Some(*value),
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
                self.solver = match &value {
                    Some(OverrideValue::Solver(solver)) => Some(*solver),
                    _ => None,
                };
            }
            O::Bypass => self.bypass = flag(),
            O::BypassReltol => self.bypass_reltol = real(),
            O::BypassAbstol => self.bypass_abstol = real(),
            O::TransientNewtonReltol => self.transient_newton_reltol = real(),
            O::TransientNewtonAbstol => self.transient_newton_abstol = real(),
            O::TransientNewtonUpdateBound => self.transient_newton_update_bound = real(),
            O::TransientNewtonResidualBound => self.transient_newton_residual_bound = real(),
            O::TransientNewtonBudget => self.transient_newton_budget = count(),
            O::TransientDeviceConvergence => self.transient_device_convergence = flag(),
            O::TransientNoxSolver => self.transient_nox_solver = flag(),
            O::StrobeInterval => self.strobe_interval = real(),
            O::OutputTimePoints => {
                self.output_time_points = match &value {
                    Some(OverrideValue::TimeList(times)) => Some(times.clone()),
                    _ => None,
                };
            }
            O::RetainEverySignal => self.retain_every_signal = flag(),
            O::HbInitialState => {
                self.hb_initial_state = match &value {
                    Some(OverrideValue::TimeDomainMode(mode)) => Some(*mode),
                    _ => None,
                };
            }
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
    ///
    /// The formatting itself is [`OverrideValue::to_report_text`], beside the
    /// deck spelling it has to stay consistent with: engineering notation for
    /// every real, the same spelling the preset it is reported beside is
    /// written in. Two options were spelled that way and the rest fell back to
    /// a bare exponent, so the solver ledger set an authored RELTOL of `1e-4`
    /// against a plan preset of `1m` and asked the reader to convert one of
    /// them before they could tell which was tighter.
    #[must_use]
    pub fn value(&self, option: NumericOverrideOption) -> Option<String> {
        Some(self.stated(option)?.to_report_text())
    }

    /// Record one authored option, or refuse it with the reason.
    ///
    /// The kind is required because applicability is part of the value being
    /// stored: a record that accepted an inapplicable option would persist a
    /// bound no solve reads.
    ///
    /// Takes the instance's [`SolverOwnership`] as well as its kind, because
    /// the instance's own tier and homotopy assign five of these options after
    /// the deck is resolved: storing one of those under an owning instance
    /// would persist a value the solve overwrites before its first Newton
    /// step. An instance whose controls assign nothing passes
    /// [`SolverOwnership::NONE`], and then only the kind decides.
    pub fn set_for_instance(
        &mut self,
        kind: AnalysisKind,
        ownership: SolverOwnership,
        option: NumericOverrideOption,
        authored: &str,
    ) -> Result<(), String> {
        if let Some(reason) = option.refusal_for_instance(kind, ownership) {
            return Err(format!(
                "{} cannot carry {}: {reason}.",
                kind.label(),
                option.key()
            ));
        }
        if let Some(reason) = self.refusal_against_what_is_already_stated(option) {
            return Err(reason);
        }
        let value = parse_authored(option, authored)?;
        self.store(option, Some(value));
        Ok(())
    }

    /// Why this record cannot take one more option, given what it already
    /// states.
    ///
    /// One pair of options is mutually exclusive in the engine's own parser:
    /// an `.OPTIONS OUTPUT` card carrying both `INITIAL_INTERVAL` and
    /// `OUTPUTTIMEPOINTS` is a syntax error, because a strobe lattice and an
    /// exact stop list are two answers to one question. Refused here rather
    /// than left to the run, where the emitted deck would fail to parse and
    /// the analysis would be reported as a broken netlist instead of as an
    /// over-specified schedule.
    fn refusal_against_what_is_already_stated(
        &self,
        option: NumericOverrideOption,
    ) -> Option<String> {
        use NumericOverrideOption as O;
        let conflict = match option {
            O::StrobeInterval => O::OutputTimePoints,
            O::OutputTimePoints => O::StrobeInterval,
            _ => return None,
        };
        self.stated(conflict).map(|_| {
            format!(
                "{} cannot be stated beside {}: an output schedule is either a strobe interval or \
                 an exact list of stops, and the engine's parser refuses a card carrying both. \
                 Clear {} first.",
                option.key(),
                conflict.key(),
                conflict.key()
            )
        })
    }

    /// Stop stating one option, so it resolves to the plan again.
    pub fn clear(&mut self, option: NumericOverrideOption) {
        self.store(option, None);
    }

    /// The first option this record states that the instance cannot carry.
    ///
    /// Restored projects and cloned analyses reach the plan without passing
    /// through [`Self::set_for_instance`], so the gate is re-checked wherever
    /// a record and an instance are bound together — which includes an edit to
    /// the instance's *tier or homotopy*. A record that was authorable under
    /// `Balanced` stops being so under `Robust`, and the plan transaction has
    /// to refuse the change rather than leave behind a stored value the solve
    /// discards.
    #[must_use]
    pub fn first_refusal_for_instance(
        &self,
        kind: AnalysisKind,
        ownership: SolverOwnership,
    ) -> Option<(NumericOverrideOption, &'static str)> {
        self.entries().into_iter().find_map(|(option, _)| {
            option
                .refusal_for_instance(kind, ownership)
                .map(|reason| (option, reason))
        })
    }

    /// Emit the `.OPTIONS` cards this record adds to its analysis's deck.
    ///
    /// Empty when nothing is stated, so a caller can splice unconditionally and
    /// still leave an inheriting analysis's deck byte-identical.
    ///
    /// One card per package, because the parser's package selector stays in
    /// force for the rest of the command it appears on: a `TIMEINT` key placed
    /// among the global ones would re-scope every key after it.
    ///
    /// Driven by [`OptionPackage::ALL`] rather than by a list written here. A
    /// package added to the catalog and forgotten by a hand-written array
    /// would have its keys silently dropped from every deck, and the analysis
    /// would run under the plan's policy while the ledger reported a
    /// departure.
    #[must_use]
    pub fn to_spice_options(&self) -> String {
        let mut cards = Vec::new();
        for package in OptionPackage::ALL {
            let mut lines = vec![package.header().to_owned()];
            for option in NumericOverrideOption::all() {
                let spec = option.spec();
                if spec.package != package {
                    continue;
                }
                let Some(text) = self
                    .stated(option)
                    .as_ref()
                    .and_then(OverrideValue::to_deck_text)
                else {
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
        OverrideValueKind::TimeDomainMode => {
            hb_time_domain_mode(authored).map(OverrideValue::TimeDomainMode)
        }
        OverrideValueKind::TimeList => time_list(option, authored).map(OverrideValue::TimeList),
    }
}

/// The chooser's own list is the authority, the same way it is for the
/// integration method: a mode authored here that the parser cannot read would
/// leave the HB solve on whatever initial state it already had.
fn hb_time_domain_mode(authored: &str) -> Result<HbTimeDomainMode, String> {
    let authored = authored.trim();
    HbTimeDomainMode::all()
        .iter()
        .copied()
        .find(|mode| {
            mode.spice_name().eq_ignore_ascii_case(authored)
                || mode.display_name().eq_ignore_ascii_case(authored)
        })
        .ok_or_else(|| {
            format!(
                "TAHB must name an initial state the deck can select: {}",
                HbTimeDomainMode::all()
                    .iter()
                    .map(|mode| mode.display_name())
                    .collect::<Vec<_>>()
                    .join(" · ")
            )
        })
}

/// An increasing list of output stops.
///
/// Both separators are accepted because both are read back: the deck carries
/// commas and the well reports spaces. Every rule here is one the engine's own
/// breakpoint schedule enforces — finite, non-negative, strictly increasing —
/// so a list this accepts is a list that run can use, and a list it refuses is
/// refused with the reason rather than at the netlist parser three surfaces
/// later.
fn time_list(option: NumericOverrideOption, authored: &str) -> Result<Vec<f64>, String> {
    let mut times = Vec::new();
    for field in authored
        .split([',', ' ', '\t'])
        .map(str::trim)
        .filter(|field| !field.is_empty())
    {
        let time = parse_si_value(field).map_err(|error| format!("{}: {error}", option.key()))?;
        if !time.is_finite() || time < 0.0 {
            return Err(format!(
                "{} stops must be finite times at or after zero, not {field}",
                option.key()
            ));
        }
        if let Some(previous) = times.last().copied()
            && time <= previous
        {
            return Err(format!(
                "{} stops must increase: {field} does not come after the stop before it",
                option.key()
            ));
        }
        times.push(time);
    }
    if times.is_empty() {
        return Err(format!("{} needs at least one stop", option.key()));
    }
    Ok(times)
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
