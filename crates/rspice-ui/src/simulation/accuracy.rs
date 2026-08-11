//! One meaning for the analysis accuracy tier.
//!
//! Four analyses offered a "Fast / Balanced / Accurate / Robust" tier and each
//! resolved it differently. The operating point read the tier as a convergence
//! preset plus an iteration budget and left the reader's tolerances alone; the
//! transfer function hand-rolled a second policy in which `Fast` *replaced*
//! `RELTOL` and `VNTOL` with 1e-3 and 1e-5. A project whose solver page asked
//! for `RELTOL=1e-6` therefore kept it under one analysis and silently lost it
//! under the other, with nothing on screen to say so.
//!
//! [`AnalysisAccuracy`] is the one enum, and [`AccuracyPolicy`] the one
//! resolution, so a tier name means the same thing wherever it is offered. The
//! composition rule is stated once and holds for every tier:
//!
//! * The tier **owns the Newton iteration budget** outright. Trading iterations
//!   for wall-clock is the whole reason the tier exists, so it is not a floor
//!   the plan can raise or a ceiling the plan can lower.
//! * The tier **only tightens tolerances, never loosens them**. A reader who
//!   asked for a tighter tolerance than the tier's keeps it; asking for `Fast`
//!   buys fewer iterations and no continuation aids, not a coarser answer.
//! * The tier **owns the nonlinear continuation aids** — the whole set or none
//!   of it — except where an analysis offers an explicit homotopy control,
//!   which is more specific and therefore wins.

use serde::{Deserialize, Serialize};

/// The solver-effort tier an analysis resolves to.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnalysisAccuracy {
    /// Direct Newton with no continuation aids and a short iteration budget.
    Fast,
    /// The resolved plan and `.OPTIONS` policy, unmodified.
    #[default]
    Balanced,
    /// Tightened tolerances and a long iteration budget.
    Accurate,
    /// Every continuation aid enabled and the longest iteration budget.
    Robust,
}

impl AnalysisAccuracy {
    pub const ALL: [Self; 4] = [Self::Fast, Self::Balanced, Self::Accurate, Self::Robust];

    /// The tier's name, as the analysis form offers it and the solver page's
    /// resolved-policy ledger reports it.
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::Fast => "Fast",
            Self::Balanced => "Balanced",
            Self::Accurate => "Accurate",
            Self::Robust => "Robust",
        }
    }

    /// The numerical policy this tier resolves to.
    pub const fn solver_policy(self) -> AccuracyPolicy {
        match self {
            Self::Fast => AccuracyPolicy {
                continuation: ContinuationPolicy::Disabled,
                iteration_budget: 50,
                ceilings: ToleranceCeilings::NONE,
            },
            Self::Balanced => AccuracyPolicy {
                continuation: ContinuationPolicy::Inherit,
                iteration_budget: 150,
                ceilings: ToleranceCeilings::NONE,
            },
            Self::Accurate => AccuracyPolicy {
                continuation: ContinuationPolicy::Inherit,
                iteration_budget: 250,
                // The tightest ceiling either of the two former policies
                // applied, so converging them onto one contract never relaxed
                // a bound an analysis already honoured.
                ceilings: ToleranceCeilings {
                    tolerance: Some(1.0e-9),
                    voltage_reltol: Some(1.0e-8),
                    voltage_abstol: Some(1.0e-9),
                    current_abstol: Some(1.0e-14),
                    residual_reltol: Some(1.0e-5),
                },
            },
            Self::Robust => AccuracyPolicy {
                continuation: ContinuationPolicy::Full,
                iteration_budget: 500,
                ceilings: ToleranceCeilings::NONE,
            },
        }
    }
}

/// What a tier does to the nonlinear continuation aids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContinuationPolicy {
    /// Plain Newton: no gmin or source stepping, no pseudo-transient, no
    /// arc-length continuation, no damping.
    Disabled,
    /// Whatever the plan and the netlist's `.OPTIONS` resolved to.
    Inherit,
    /// Every aid, with combined damping.
    Full,
}

/// Upper bounds a tier places on the convergence tolerances.
///
/// `None` means the tier states no opinion, so the resolved value stands. A
/// bound is applied as a minimum against the resolved value and so can only
/// tighten it.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ToleranceCeilings {
    pub tolerance: Option<f64>,
    pub voltage_reltol: Option<f64>,
    pub voltage_abstol: Option<f64>,
    pub current_abstol: Option<f64>,
    pub residual_reltol: Option<f64>,
}

impl ToleranceCeilings {
    pub const NONE: Self = Self {
        tolerance: None,
        voltage_reltol: None,
        voltage_abstol: None,
        current_abstol: None,
        residual_reltol: None,
    };
}

/// The resolved numerical policy of one accuracy tier.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccuracyPolicy {
    pub continuation: ContinuationPolicy,
    pub iteration_budget: usize,
    pub ceilings: ToleranceCeilings,
}

impl AccuracyPolicy {
    /// Apply the tier to an already-resolved configuration.
    ///
    /// This runs *after* core defaults, the netlist's `.OPTIONS`, and the plan's
    /// solver page have all been resolved, which is what makes "only tightens"
    /// checkable: the value being tightened is the one the reader would have
    /// got without the tier.
    pub fn apply(&self, config: &mut rspice_core::engine::SimulationConfig) {
        use rspice_core::engine::DampingStrategy;

        config.max_iterations = self.iteration_budget;

        let convergence = &mut config.convergence_config;
        match self.continuation {
            ContinuationPolicy::Disabled => {
                convergence.gmin_stepping = false;
                convergence.source_stepping = false;
                convergence.pseudo_transient = false;
                convergence.arc_length = false;
                convergence.damping_strategy = DampingStrategy::None;
                convergence.nonlinear_continuation = None;
            }
            ContinuationPolicy::Inherit => {}
            ContinuationPolicy::Full => {
                convergence.gmin_stepping = true;
                convergence.source_stepping = true;
                convergence.pseudo_transient = true;
                convergence.arc_length = true;
                convergence.damping_strategy = DampingStrategy::Combined;
            }
        }

        tighten(&mut config.tolerance, self.ceilings.tolerance);
        tighten(&mut convergence.voltage_reltol, self.ceilings.voltage_reltol);
        tighten(&mut convergence.voltage_abstol, self.ceilings.voltage_abstol);
        tighten(&mut convergence.current_abstol, self.ceilings.current_abstol);
        tighten(
            &mut convergence.residual_reltol,
            self.ceilings.residual_reltol,
        );
    }
}

/// Lower `value` to `ceiling`, leaving a value that is already tighter alone.
///
/// A non-positive resolved tolerance would never be met, so it is treated as
/// unset and takes the ceiling rather than surviving as the minimum.
fn tighten(value: &mut f64, ceiling: Option<f64>) {
    let Some(ceiling) = ceiling else {
        return;
    };
    *value = if *value > 0.0 {
        value.min(ceiling)
    } else {
        ceiling
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::engine::{DampingStrategy, SimulationConfig};

    fn applied(accuracy: AnalysisAccuracy, base: SimulationConfig) -> SimulationConfig {
        let mut config = base;
        accuracy.solver_policy().apply(&mut config);
        config
    }

    #[test]
    fn every_tier_owns_the_iteration_budget() {
        let mut base = SimulationConfig::default();
        base.max_iterations = 63;
        for accuracy in AnalysisAccuracy::ALL {
            let policy = accuracy.solver_policy();
            assert_eq!(
                applied(accuracy, base.clone()).max_iterations,
                policy.iteration_budget,
                "{} must resolve to its own budget, not the plan's",
                accuracy.display_name()
            );
        }
    }

    #[test]
    fn no_tier_loosens_a_tolerance_the_reader_tightened() {
        let mut base = SimulationConfig::default();
        base.tolerance = 1.0e-12;
        base.convergence_config.voltage_reltol = 1.0e-12;
        base.convergence_config.voltage_abstol = 1.0e-15;
        base.convergence_config.current_abstol = 1.0e-18;
        base.convergence_config.residual_reltol = 1.0e-12;

        for accuracy in AnalysisAccuracy::ALL {
            let resolved = applied(accuracy, base.clone());
            let tier = accuracy.display_name();
            assert!(resolved.tolerance <= base.tolerance, "{tier} tolerance");
            let (got, want) = (&resolved.convergence_config, &base.convergence_config);
            assert!(got.voltage_reltol <= want.voltage_reltol, "{tier} reltol");
            assert!(got.voltage_abstol <= want.voltage_abstol, "{tier} vntol");
            assert!(got.current_abstol <= want.current_abstol, "{tier} abstol");
            assert!(
                got.residual_reltol <= want.residual_reltol,
                "{tier} residual"
            );
        }
    }

    #[test]
    fn balanced_changes_nothing_but_the_budget() {
        let mut base = SimulationConfig::default();
        base.convergence_config.gmin_stepping = true;
        base.convergence_config.arc_length = true;
        let resolved = applied(AnalysisAccuracy::Balanced, base.clone());

        assert_eq!(resolved.tolerance, base.tolerance);
        assert_eq!(
            format!("{:?}", resolved.convergence_config),
            format!("{:?}", base.convergence_config)
        );
    }

    #[test]
    fn fast_strips_the_continuation_aids_without_coarsening_the_answer() {
        let mut base = SimulationConfig::default();
        base.convergence_config.gmin_stepping = true;
        base.convergence_config.source_stepping = true;
        base.convergence_config.pseudo_transient = true;
        base.convergence_config.arc_length = true;
        base.convergence_config.damping_strategy = DampingStrategy::Combined;
        base.convergence_config.voltage_reltol = 1.0e-6;

        let resolved = applied(AnalysisAccuracy::Fast, base.clone());

        assert!(!resolved.convergence_config.gmin_stepping);
        assert!(!resolved.convergence_config.source_stepping);
        assert!(!resolved.convergence_config.pseudo_transient);
        assert!(!resolved.convergence_config.arc_length);
        assert_eq!(
            resolved.convergence_config.damping_strategy,
            DampingStrategy::None
        );
        // The defect this contract exists to close: `Fast` used to reset the
        // reader's 1e-6 relative tolerance to 1e-3 in one analysis and keep it
        // in another.
        assert_eq!(resolved.convergence_config.voltage_reltol, 1.0e-6);
    }

    #[test]
    fn accurate_tightens_a_loose_policy_to_the_documented_bounds() {
        let resolved = applied(AnalysisAccuracy::Accurate, SimulationConfig::default());
        assert!(resolved.tolerance <= 1.0e-9);
        assert!(resolved.convergence_config.voltage_reltol <= 1.0e-8);
        assert!(resolved.convergence_config.voltage_abstol <= 1.0e-9);
        assert!(resolved.convergence_config.current_abstol <= 1.0e-14);
        assert!(resolved.convergence_config.residual_reltol <= 1.0e-5);
    }

    #[test]
    fn robust_enables_every_continuation_aid() {
        let resolved = applied(AnalysisAccuracy::Robust, SimulationConfig::default());
        assert!(resolved.convergence_config.gmin_stepping);
        assert!(resolved.convergence_config.source_stepping);
        assert!(resolved.convergence_config.pseudo_transient);
        assert!(resolved.convergence_config.arc_length);
        assert_eq!(
            resolved.convergence_config.damping_strategy,
            DampingStrategy::Combined
        );
    }

    #[test]
    fn a_non_positive_resolved_tolerance_takes_the_ceiling() {
        let mut base = SimulationConfig::default();
        base.convergence_config.current_abstol = 0.0;
        let resolved = applied(AnalysisAccuracy::Accurate, base);
        assert_eq!(resolved.convergence_config.current_abstol, 1.0e-14);
    }

    #[test]
    fn the_tiers_are_ordered_by_effort() {
        let budgets: Vec<usize> = AnalysisAccuracy::ALL
            .iter()
            .map(|tier| tier.solver_policy().iteration_budget)
            .collect();
        assert!(
            budgets.windows(2).all(|pair| pair[0] < pair[1]),
            "a tier further down the list must never buy fewer iterations: {budgets:?}"
        );
    }

    #[test]
    fn serde_round_trips_every_tier_in_snake_case() {
        for accuracy in AnalysisAccuracy::ALL {
            let json = serde_json::to_string(&accuracy).expect("serialize");
            assert_eq!(json, format!("\"{}\"", accuracy.display_name().to_lowercase()));
            let back: AnalysisAccuracy = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(back, accuracy);
        }
    }
}
