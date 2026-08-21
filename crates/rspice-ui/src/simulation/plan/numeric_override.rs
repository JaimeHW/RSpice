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

use serde::{Deserialize, Serialize};

use crate::simulation::dialog::{IntegrationMethod, format_si_value, parse_si_value};

use super::AnalysisKind;

/// One solver option an analysis may depart from the plan on.
///
/// The set is deliberately the intersection of what
/// `rspice_core::SimulationConfigOverrides` can resolve and what the Solver
/// page already states at plan level: an option nobody can see at plan level
/// has no policy to depart from, and one the resolver ignores would be an
/// override that never reaches a solve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericOverrideOption {
    Reltol,
    Abstol,
    Vntol,
    ResidualReltol,
    Itl1,
    Itl4,
    Trtol,
    IntegrationMethod,
    MaximumTimestep,
}

/// Why an analysis of some kind cannot carry an option.
///
/// Two things make an option inapplicable: the solve never reads it, or some
/// other record already owns it. Both are worded once here so a reader meets
/// the same sentence wherever the refusal surfaces.
const NOT_TIME_STEPPED: &str =
    "this analysis never advances time, so a time-integration bound would not reach its solve";
const TRANSIENT_OWNS_STEP_CEILING: &str =
    "the transient's own Max step field owns this, and one bound cannot have two copies";

impl NumericOverrideOption {
    /// Why an analysis carrying an accuracy tier cannot be given ITL1, and
    /// equally why the Solver ledger does not report the plan's ITL1 as that
    /// analysis's effective Newton budget.
    ///
    /// [`crate::simulation::accuracy::AccuracyPolicy::apply`] assigns
    /// `max_iterations` from the tier *after* the deck's `.OPTIONS` have been
    /// resolved, so an ITL1 written into either options block — the plan's or
    /// this record's — is overwritten before the first Newton step. One
    /// sentence states it, and both the refusal and the ledger's origin cell
    /// use this one.
    pub const ACCURACY_TIER_OWNS_ITERATIONS: &'static str =
        "its accuracy tier owns the Newton budget and is applied after the deck's options";

    /// Every option, in the order the ledger reports them.
    pub const ALL: [Self; 9] = [
        Self::Reltol,
        Self::Abstol,
        Self::Vntol,
        Self::ResidualReltol,
        Self::Itl1,
        Self::Itl4,
        Self::Trtol,
        Self::IntegrationMethod,
        Self::MaximumTimestep,
    ];

    /// How the option is named on the ledger and in refusals.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Reltol => "Update bound · RELTOL",
            Self::Abstol => "Current floor · ABSTOL",
            Self::Vntol => "Voltage floor · VNTOL",
            Self::ResidualReltol => "Residual bound · RESIDUAL_RELTOL",
            Self::Itl1 => "Newton budget · ITL1",
            Self::Itl4 => "Iterations per step · ITL4",
            Self::Trtol => "Truncation bound · TRTOL",
            Self::IntegrationMethod => "Integration method",
            Self::MaximumTimestep => "Step ceiling",
        }
    }

    /// Short identity used by the option picker and by refusal messages.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Reltol => "RELTOL",
            Self::Abstol => "ABSTOL",
            Self::Vntol => "VNTOL",
            Self::ResidualReltol => "RESIDUAL_RELTOL",
            Self::Itl1 => "ITL1",
            Self::Itl4 => "ITL4",
            Self::Trtol => "TRTOL",
            Self::IntegrationMethod => "METHOD",
            Self::MaximumTimestep => "DELMAX",
        }
    }

    /// What an authored value must look like, for the input's placeholder.
    pub const fn value_hint(self) -> &'static str {
        match self {
            Self::Itl1 | Self::Itl4 => "iteration count",
            Self::IntegrationMethod => "TRAP · EULER · GEAR2 · TRAPGEAR",
            Self::MaximumTimestep => "time, SI suffixes accepted",
            _ => "positive real",
        }
    }

    /// Why this kind cannot carry this option, if it cannot.
    ///
    /// A refusal is the whole point of the gate: an override that is accepted,
    /// persisted and then ignored by the solve is indistinguishable from one
    /// that works, which is the failure this record exists to avoid.
    pub const fn refusal_for(self, kind: AnalysisKind) -> Option<&'static str> {
        match self {
            Self::Itl4 | Self::Trtol | Self::IntegrationMethod | Self::MaximumTimestep
                if !kind.advances_time() =>
            {
                Some(NOT_TIME_STEPPED)
            }
            Self::MaximumTimestep if matches!(kind, AnalysisKind::Transient) => {
                Some(TRANSIENT_OWNS_STEP_CEILING)
            }
            // Both kinds that offer an accuracy tier, not just the operating
            // point: the transfer function resolves its tier through the same
            // `AccuracyPolicy::apply`, so an ITL1 recorded against it would be
            // accepted, persisted, reported — and then overwritten.
            Self::Itl1
                if matches!(
                    kind,
                    AnalysisKind::OperatingPoint | AnalysisKind::TransferFunction
                ) =>
            {
                Some(Self::ACCURACY_TIER_OWNS_ITERATIONS)
            }
            _ => None,
        }
    }

    /// The options this kind may carry, for the authoring picker.
    pub fn applicable_to(kind: AnalysisKind) -> Vec<Self> {
        Self::ALL
            .into_iter()
            .filter(|option| option.refusal_for(kind).is_none())
            .collect()
    }
}

/// One analysis's numerical departures. Absent means "inherit the plan".
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
}

impl AnalysisNumericOverride {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    #[must_use]
    pub const fn reltol(&self) -> Option<f64> {
        self.reltol
    }

    #[must_use]
    pub const fn abstol(&self) -> Option<f64> {
        self.abstol
    }

    #[must_use]
    pub const fn vntol(&self) -> Option<f64> {
        self.vntol
    }

    #[must_use]
    pub const fn residual_reltol(&self) -> Option<f64> {
        self.residual_reltol
    }

    #[must_use]
    pub const fn itl1(&self) -> Option<usize> {
        self.itl1
    }

    #[must_use]
    pub const fn itl4(&self) -> Option<usize> {
        self.itl4
    }

    #[must_use]
    pub const fn trtol(&self) -> Option<f64> {
        self.trtol
    }

    #[must_use]
    pub const fn integration_method(&self) -> Option<IntegrationMethod> {
        self.integration_method
    }

    #[must_use]
    pub const fn max_timestep(&self) -> Option<f64> {
        self.max_timestep
    }

    /// Every option this record states, in [`NumericOverrideOption::ALL`] order.
    #[must_use]
    pub fn entries(&self) -> Vec<(NumericOverrideOption, String)> {
        NumericOverrideOption::ALL
            .into_iter()
            .filter_map(|option| self.value(option).map(|value| (option, value)))
            .collect()
    }

    /// The stated value of one option, formatted as the ledger reports it.
    #[must_use]
    pub fn value(&self, option: NumericOverrideOption) -> Option<String> {
        match option {
            NumericOverrideOption::Reltol => self.reltol.map(format_real),
            NumericOverrideOption::Abstol => self.abstol.map(format_real),
            NumericOverrideOption::Vntol => self.vntol.map(format_real),
            NumericOverrideOption::ResidualReltol => self.residual_reltol.map(format_real),
            NumericOverrideOption::Itl1 => self.itl1.map(|value| value.to_string()),
            NumericOverrideOption::Itl4 => self.itl4.map(|value| value.to_string()),
            NumericOverrideOption::Trtol => self.trtol.map(format_real),
            NumericOverrideOption::IntegrationMethod => self
                .integration_method
                .map(|method| method.spice_name().to_owned()),
            // A step ceiling is a physical quantity, so it reads in the same
            // engineering notation as the transient form's own Max step rather
            // than as a bare exponent.
            NumericOverrideOption::MaximumTimestep => self.max_timestep.map(format_si_value),
        }
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
        match option {
            NumericOverrideOption::Reltol => self.reltol = Some(positive_real(option, authored)?),
            NumericOverrideOption::Abstol => self.abstol = Some(positive_real(option, authored)?),
            NumericOverrideOption::Vntol => self.vntol = Some(positive_real(option, authored)?),
            NumericOverrideOption::ResidualReltol => {
                self.residual_reltol = Some(positive_real(option, authored)?);
            }
            NumericOverrideOption::Itl1 => self.itl1 = Some(iteration_budget(option, authored)?),
            NumericOverrideOption::Itl4 => self.itl4 = Some(iteration_budget(option, authored)?),
            NumericOverrideOption::Trtol => self.trtol = Some(positive_real(option, authored)?),
            NumericOverrideOption::IntegrationMethod => {
                self.integration_method = Some(integration_method(authored)?);
            }
            NumericOverrideOption::MaximumTimestep => {
                self.max_timestep = Some(positive_real(option, authored)?);
            }
        }
        Ok(())
    }

    /// Stop stating one option, so it resolves to the plan again.
    pub fn clear(&mut self, option: NumericOverrideOption) {
        match option {
            NumericOverrideOption::Reltol => self.reltol = None,
            NumericOverrideOption::Abstol => self.abstol = None,
            NumericOverrideOption::Vntol => self.vntol = None,
            NumericOverrideOption::ResidualReltol => self.residual_reltol = None,
            NumericOverrideOption::Itl1 => self.itl1 = None,
            NumericOverrideOption::Itl4 => self.itl4 = None,
            NumericOverrideOption::Trtol => self.trtol = None,
            NumericOverrideOption::IntegrationMethod => self.integration_method = None,
            NumericOverrideOption::MaximumTimestep => self.max_timestep = None,
        }
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
    /// Reals are written at full precision rather than the two-digit form the
    /// plan-level emitter uses: a plan value is a preset the reader recognizes,
    /// whereas these are exactly what someone typed, and a deck that rounded
    /// them would resolve to a bound the ledger does not report.
    #[must_use]
    pub fn to_spice_options(&self) -> String {
        let mut cards = Vec::new();
        let mut lines = vec![".OPTIONS".to_owned()];
        if let Some(value) = self.reltol {
            lines.push(format!("+ RELTOL={value:e}"));
        }
        if let Some(value) = self.abstol {
            lines.push(format!("+ ABSTOL={value:e}"));
        }
        if let Some(value) = self.vntol {
            lines.push(format!("+ VNTOL={value:e}"));
        }
        if let Some(value) = self.residual_reltol {
            lines.push(format!("+ RESIDUAL_RELTOL={value:e}"));
        }
        if let Some(value) = self.itl1 {
            lines.push(format!("+ ITL1={value}"));
        }
        if let Some(value) = self.itl4 {
            lines.push(format!("+ ITL4={value}"));
        }
        if let Some(value) = self.trtol {
            lines.push(format!("+ TRTOL={value:e}"));
        }
        if let Some(method) = self.integration_method {
            lines.push(format!("+ METHOD={}", method.spice_name()));
        }
        if lines.len() > 1 {
            cards.push(lines.join("\n"));
        }
        // The step ceiling has no bare option key: the parser reaches
        // `transient_timeint_max_timestep` only through the TIMEINT package,
        // and the package latches for the rest of its card, so it gets one.
        if let Some(value) = self.max_timestep {
            cards.push(format!(".OPTIONS TIMEINT DELMAX={value:e}"));
        }
        cards.join("\n")
    }
}

/// Full-precision exponent form, matching what the emitted deck carries.
fn format_real(value: f64) -> String {
    format!("{value:e}")
}

fn positive_real(option: NumericOverrideOption, authored: &str) -> Result<f64, String> {
    let value = parse_si_value(authored).map_err(|error| format!("{}: {error}", option.key()))?;
    if !value.is_finite() || value <= 0.0 {
        return Err(format!("{} must be a positive real value", option.key()));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_record_adds_nothing_to_a_deck() {
        let record = AnalysisNumericOverride::default();
        assert!(record.is_empty());
        assert!(record.to_spice_options().is_empty());
        assert!(record.entries().is_empty());
    }

    #[test]
    fn every_stated_option_reaches_the_emitted_options_cards() {
        let mut record = AnalysisNumericOverride::default();
        for (option, authored) in [
            (NumericOverrideOption::Reltol, "2e-4"),
            (NumericOverrideOption::Abstol, "5e-13"),
            (NumericOverrideOption::Vntol, "1u"),
            (NumericOverrideOption::ResidualReltol, "3e-4"),
            (NumericOverrideOption::Itl1, "220"),
            (NumericOverrideOption::Itl4, "12"),
            (NumericOverrideOption::Trtol, "3"),
            (NumericOverrideOption::IntegrationMethod, "gear2"),
        ] {
            record
                .set(AnalysisKind::Transient, option, authored)
                .unwrap_or_else(|error| panic!("{option:?} is authorable on a transient: {error}"));
        }

        let emitted = record.to_spice_options();
        for expected in [
            "RELTOL=2e-4",
            "ABSTOL=5e-13",
            "VNTOL=1e-6",
            "RESIDUAL_RELTOL=3e-4",
            "ITL1=220",
            "ITL4=12",
            "TRTOL=3e0",
            "METHOD=GEAR2",
        ] {
            assert!(
                emitted.contains(expected),
                "the emitted options must carry {expected}:\n{emitted}"
            );
        }

        // Emitting text the engine's own parser rejects, or reads as another
        // number, would leave the run resolving to something the ledger never
        // reported. The parser is the only authority on that, so it is asked.
        let parsed = rspice_core::netlist::parse_netlist(&format!("emitted\n{emitted}\n.end\n"))
            .expect("the emitted cards parse");
        assert_eq!(parsed.options.reltol, Some(2.0e-4));
        assert_eq!(parsed.options.abstol, Some(5.0e-13));
        assert_eq!(parsed.options.vntol, Some(1.0e-6));
        assert_eq!(parsed.options.residual_reltol, Some(3.0e-4));
        assert_eq!(parsed.options.itl1, Some(220));
        assert_eq!(parsed.options.itl4, Some(12));
        assert_eq!(parsed.options.trtol, Some(3.0));
        assert_eq!(parsed.options.method.as_deref(), Some("GEAR2"));
    }

    #[test]
    fn a_step_ceiling_is_emitted_through_the_timeint_package() {
        let mut record = AnalysisNumericOverride::default();
        record
            .set(
                AnalysisKind::Fourier,
                NumericOverrideOption::MaximumTimestep,
                "500p",
            )
            .expect("a Fourier measurement runs a transient");
        let emitted = record.to_spice_options();
        assert_eq!(emitted, ".OPTIONS TIMEINT DELMAX=5e-10");
        let parsed = rspice_core::netlist::parse_netlist(&format!("emitted\n{emitted}\n.end\n"))
            .expect("the TIMEINT card parses");
        assert_eq!(parsed.options.timeint_delmax, Some(5.0e-10));
        assert_eq!(
            record
                .value(NumericOverrideOption::MaximumTimestep)
                .unwrap(),
            format_si_value(5.0e-10)
        );
    }

    #[test]
    fn an_option_the_kind_cannot_use_is_refused_and_stores_nothing() {
        let mut record = AnalysisNumericOverride::default();
        let error = record
            .set(AnalysisKind::Ac, NumericOverrideOption::Itl4, "12")
            .expect_err("an AC sweep never takes a timestep");
        assert!(error.contains("ITL4"), "{error}");
        assert!(record.is_empty());

        let error = record
            .set(
                AnalysisKind::Transient,
                NumericOverrideOption::MaximumTimestep,
                "1n",
            )
            .expect_err("the transient form owns its step ceiling");
        assert!(error.contains("Max step"), "{error}");

        let error = record
            .set(
                AnalysisKind::OperatingPoint,
                NumericOverrideOption::Itl1,
                "200",
            )
            .expect_err("the operating point's accuracy tier owns its Newton budget");
        assert!(error.contains("accuracy tier"), "{error}");
        assert!(record.is_empty());
    }

    #[test]
    fn a_value_that_cannot_bound_a_solve_is_refused() {
        let mut record = AnalysisNumericOverride::default();
        for (kind, option, authored) in [
            (AnalysisKind::Ac, NumericOverrideOption::Reltol, "0"),
            (AnalysisKind::Ac, NumericOverrideOption::Reltol, "-1e-3"),
            (AnalysisKind::Ac, NumericOverrideOption::Reltol, "wide"),
            (AnalysisKind::Ac, NumericOverrideOption::Itl1, "0"),
            (AnalysisKind::Ac, NumericOverrideOption::Itl1, "2.5"),
            (
                AnalysisKind::Transient,
                NumericOverrideOption::IntegrationMethod,
                "simpson",
            ),
            // Retired chooser spellings. A saved project decodes them onto
            // the surviving method, but authoring one here is a typo.
            (
                AnalysisKind::Transient,
                NumericOverrideOption::IntegrationMethod,
                "GEAR2ONLY",
            ),
            (
                AnalysisKind::Transient,
                NumericOverrideOption::IntegrationMethod,
                "GEAR",
            ),
        ] {
            assert!(
                record.set(kind, option, authored).is_err(),
                "{option:?} must refuse {authored:?}"
            );
        }
        assert!(record.is_empty());
    }

    #[test]
    fn a_restored_record_is_re_checked_against_its_kind() {
        let mut record = AnalysisNumericOverride::default();
        record
            .set(AnalysisKind::Transient, NumericOverrideOption::Itl4, "12")
            .expect("a transient takes timesteps");
        assert!(record.first_refusal_for(AnalysisKind::Transient).is_none());
        let (option, _) = record
            .first_refusal_for(AnalysisKind::Ac)
            .expect("the same record cannot be carried by an AC sweep");
        assert_eq!(option, NumericOverrideOption::Itl4);
    }

    #[test]
    fn a_record_naming_a_retired_method_decodes_onto_the_survivor() {
        // The chooser stopped offering the plain `Gear` and `Gear2Only`
        // spellings, but an analysis authored under either still has to open
        // and still has to emit a card the parser reads.
        for retired in ["Gear", "Gear2Only"] {
            let record: AnalysisNumericOverride =
                serde_json::from_str(&format!(r#"{{"integration_method":"{retired}"}}"#))
                    .unwrap_or_else(|error| panic!("a record naming {retired} decodes: {error}"));

            assert_eq!(record.integration_method(), Some(IntegrationMethod::Gear2));
            assert_eq!(record.to_spice_options(), ".OPTIONS\n+ METHOD=GEAR2");
        }
    }
}
