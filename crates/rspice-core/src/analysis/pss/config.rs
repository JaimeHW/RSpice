//! PSS Configuration
//!
//! Defines configuration parameters for Periodic Steady-State analysis.

use crate::Value;

/// Configuration for Periodic Steady-State (PSS) analysis
///
/// PSS analysis finds the periodic operating point of circuits driven by
/// periodic large-signal excitations. This configuration controls all
/// aspects of the shooting Newton solver.
///
/// # Examples
///
/// ## Driven Circuit (known fundamental)
/// ```
/// use rspice_core::analysis::pss::PssConfig;
///
/// let config = PssConfig::new(1e9)  // 1 GHz fundamental
///     .with_harmonics(9)
///     .with_tstab(10e-9);
/// ```
///
/// ## Autonomous Oscillator (period detection)
/// ```
/// use rspice_core::analysis::pss::PssConfig;
///
/// let config = PssConfig::autonomous()
///     .with_period_guess(1e-9)  // ~1 GHz expected
///     .with_harmonics(15);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct PssConfig {
    /// Fundamental frequency (Hz).
    /// Set to 0.0 for autonomous circuits (oscillators) where period
    /// must be detected automatically.
    pub fundamental_freq: Value,

    /// Number of harmonics to compute in the result.
    /// Higher values provide more spectral detail but increase computation.
    /// Default: 9 (DC through 9th harmonic)
    pub num_harmonics: usize,

    /// Stabilization time before shooting iterations begin.
    /// Allows startup transients to decay before searching for periodicity.
    /// For oscillators, this should be several periods of expected oscillation.
    /// Default: 0.0 (no stabilization)
    pub tstab: Value,

    /// Maximum number of shooting Newton iterations.
    /// If not converged after this many iterations, analysis fails.
    /// Default: 100
    pub max_iterations: usize,

    /// Convergence tolerance for periodicity.
    /// The solver converges when ||x(T) - x(0)|| / ||x(0)|| < tolerance.
    /// Default: 1e-6 (relative)
    pub tolerance: Value,

    /// Absolute tolerance for small signals.
    /// Used when ||x(0)|| is very small to prevent division issues.
    /// Default: 1e-12
    pub abstol: Value,

    /// Enable automatic period detection for autonomous circuits.
    /// When true, the fundamental_freq is treated as an initial guess
    /// and refined during analysis.
    /// Default: false
    pub auto_period: bool,

    /// Optional node used for autonomous period detection. When omitted, the
    /// first solved voltage waveform is used for backward compatibility.
    pub oscillator_node: Option<String>,

    /// Initial guess for period when auto_period is true.
    /// Used to seed the period detection algorithm.
    /// Default: 1e-9 (1 ns, ~1 GHz)
    pub period_guess: Value,

    /// Number of transient periods to simulate during stabilization.
    /// Used when tstab is 0 but auto_period is true.
    /// Default: 10
    pub tstab_periods: usize,

    /// Damping factor for Newton updates.
    /// Values < 1.0 provide more conservative updates for difficult convergence.
    /// Default: 1.0 (no damping)
    pub damping_factor: Value,

    /// Maximum allowed period change per iteration (for autonomous).
    /// Prevents wild period oscillations during convergence.
    /// Default: 0.1 (10%)
    pub max_period_change: Value,

    /// Integration method override for transient shooting.
    /// None uses the engine default (typically TrapGear).
    pub integration_method: Option<crate::numerics::integration::IntegrationMethod>,

    /// Points per period for internal waveform storage.
    /// Higher values give better accuracy but use more memory.
    /// Default: 256
    pub points_per_period: usize,

    /// Enable verbose logging of convergence progress.
    /// Default: false
    pub verbose: bool,
}

impl PssConfig {
    /// Create a new PSS configuration for a driven circuit with known fundamental.
    ///
    /// # Arguments
    /// * `fundamental_freq` - Fundamental frequency in Hz
    ///
    /// # Example
    /// ```
    /// use rspice_core::analysis::pss::PssConfig;
    /// let config = PssConfig::new(1e9);  // 1 GHz
    /// assert_eq!(config.period(), 1e-9);
    /// ```
    pub fn new(fundamental_freq: Value) -> Self {
        Self {
            fundamental_freq,
            num_harmonics: 9,
            tstab: 0.0,
            max_iterations: 100,
            tolerance: 1e-6,
            abstol: 1e-12,
            auto_period: false,
            oscillator_node: None,
            period_guess: 1e-9,
            tstab_periods: 10,
            damping_factor: 1.0,
            max_period_change: 0.1,
            integration_method: None,
            points_per_period: 256,
            verbose: false,
        }
    }

    /// Create a configuration for autonomous circuits (oscillators).
    ///
    /// Enables automatic period detection. You should provide a period guess
    /// via `with_period_guess()` for faster convergence.
    ///
    /// # Example
    /// ```
    /// use rspice_core::analysis::pss::PssConfig;
    /// let config = PssConfig::autonomous()
    ///     .with_period_guess(1e-9);  // ~1 GHz expected
    /// ```
    pub fn autonomous() -> Self {
        Self {
            fundamental_freq: 0.0,
            auto_period: true,
            tstab_periods: 20, // Oscillators need more startup time
            ..Self::new(0.0)
        }
    }

    /// Get the period corresponding to the fundamental frequency.
    ///
    /// For autonomous circuits (fundamental_freq = 0), returns the period guess.
    #[inline]
    pub fn period(&self) -> Value {
        if self.fundamental_freq > 0.0 {
            1.0 / self.fundamental_freq
        } else {
            self.period_guess
        }
    }

    /// Set the number of harmonics to compute.
    pub fn with_harmonics(mut self, n: usize) -> Self {
        self.num_harmonics = n;
        self
    }

    /// Set the stabilization time before shooting.
    ///
    /// For oscillators, use `with_tstab_periods()` instead for automatic
    /// calculation based on detected period.
    pub fn with_tstab(mut self, tstab: Value) -> Self {
        self.tstab = tstab;
        self
    }

    /// Set stabilization time as number of periods.
    ///
    /// The actual tstab will be computed as `periods * detected_period`.
    pub fn with_tstab_periods(mut self, periods: usize) -> Self {
        self.tstab_periods = periods;
        self
    }

    /// Set convergence tolerance (relative).
    pub fn with_tolerance(mut self, tol: Value) -> Self {
        self.tolerance = tol;
        self
    }

    /// Set maximum Newton iterations.
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    /// Validate public configuration before the engine starts solving.
    pub fn validate(&self) -> Result<(), String> {
        if self.num_harmonics == 0 {
            return Err("num_harmonics must be > 0".to_string());
        }
        if !self.fundamental_freq.is_finite() || self.fundamental_freq < 0.0 {
            return Err("fundamental_freq must be finite and >= 0".to_string());
        }
        if !self.tstab.is_finite() || self.tstab < 0.0 {
            return Err("tstab must be finite and >= 0".to_string());
        }
        if self.max_iterations == 0 {
            return Err("max_iterations must be > 0".to_string());
        }
        if !self.tolerance.is_finite() || self.tolerance <= 0.0 {
            return Err("tolerance must be finite and > 0".to_string());
        }
        if !self.abstol.is_finite() || self.abstol <= 0.0 {
            return Err("abstol must be finite and > 0".to_string());
        }
        if !self.damping_factor.is_finite() || !(0.1..=1.0).contains(&self.damping_factor) {
            return Err("damping_factor must be finite and in [0.1, 1.0]".to_string());
        }
        if self.is_autonomous() {
            if !self.period_guess.is_finite() || self.period_guess <= 0.0 {
                return Err("period_guess must be finite and > 0".to_string());
            }
            if !self.max_period_change.is_finite() || self.max_period_change <= 0.0 {
                return Err("max_period_change must be finite and > 0".to_string());
            }
        }
        if self.points_per_period < 16 {
            return Err("points_per_period must be >= 16".to_string());
        }
        if self
            .num_harmonics
            .checked_mul(2)
            .is_none_or(|samples| samples > self.points_per_period)
        {
            return Err(
                "points_per_period must be at least twice num_harmonics to avoid aliasing"
                    .to_string(),
            );
        }
        if !self.effective_tstab().is_finite() {
            return Err("effective_tstab must be finite".to_string());
        }
        Ok(())
    }

    /// Set initial period guess for autonomous circuits.
    ///
    /// Also sets fundamental_freq to the corresponding frequency.
    pub fn with_period_guess(mut self, period: Value) -> Self {
        self.period_guess = period;
        if period > 0.0 {
            self.fundamental_freq = 1.0 / period;
        }
        self
    }

    /// Select the solved voltage waveform used for autonomous period detection.
    pub fn with_oscillator_node(mut self, node: impl Into<String>) -> Self {
        let node = node.into();
        self.oscillator_node = (!node.trim().is_empty()).then_some(node);
        self
    }

    /// Set Newton damping factor.
    ///
    /// Values < 1.0 provide more conservative updates.
    /// Useful for circuits with difficult convergence.
    pub fn with_damping(mut self, factor: Value) -> Self {
        self.damping_factor = factor.clamp(0.1, 1.0);
        self
    }

    /// Set points per period for waveform storage.
    pub fn with_points_per_period(mut self, points: usize) -> Self {
        self.points_per_period = points.max(16);
        self
    }

    /// Enable verbose logging.
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Check if this is an autonomous circuit configuration.
    #[inline]
    pub fn is_autonomous(&self) -> bool {
        self.auto_period || self.fundamental_freq <= 0.0
    }

    /// Get the effective stabilization time.
    ///
    /// If tstab is explicitly set, returns that value.
    /// Otherwise, computes from tstab_periods * period.
    pub fn effective_tstab(&self) -> Value {
        if self.tstab > 0.0 {
            self.tstab
        } else {
            self.tstab_periods as Value * self.period()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_rejects_unrepresentable_harmonic_bandwidth() {
        let mut zero = PssConfig::new(1.0e6);
        zero.num_harmonics = 0;
        assert!(zero.validate().is_err());

        let mut aliased = PssConfig::new(1.0e6);
        aliased.num_harmonics = 9;
        aliased.points_per_period = 16;
        assert!(aliased.validate().is_err());

        aliased.num_harmonics = 8;
        assert!(aliased.validate().is_ok());
    }
}

impl Default for PssConfig {
    fn default() -> Self {
        Self::new(1e9) // 1 GHz default
    }
}

impl From<&crate::netlist::PssCard> for PssConfig {
    /// Convert an authored `.PSS` card into the configuration the shooting
    /// solver takes.
    ///
    /// The conversion lives here rather than in the parser because a parsed
    /// deck sits below the analyses and may not name them. The card was
    /// validated where it was authored, so this is a total mapping; the
    /// exhaustive literal makes a new configuration field a compile error
    /// here instead of a silently defaulted one.
    fn from(card: &crate::netlist::PssCard) -> Self {
        Self {
            fundamental_freq: card.fundamental_freq,
            num_harmonics: card.num_harmonics,
            tstab: card.tstab,
            max_iterations: card.max_iterations,
            tolerance: card.tolerance,
            abstol: card.abstol,
            auto_period: card.auto_period,
            oscillator_node: card.oscillator_node.clone(),
            period_guess: card.period_guess,
            tstab_periods: card.tstab_periods,
            damping_factor: card.damping_factor,
            max_period_change: card.max_period_change,
            integration_method: card.integration_method,
            points_per_period: card.points_per_period,
            verbose: card.verbose,
        }
    }
}

#[cfg(test)]
mod card_conversion_tests {
    use super::*;
    use crate::netlist::{AnalysisCommand, Netlist, PssCard};

    const CIRCUIT: &str = "pss card conversion\n\
                           V1 in 0 SIN(0 1 1G)\n\
                           R1 in out 1k\n\
                           C1 out 0 1p\n";

    fn card(cards: &str) -> PssCard {
        let netlist =
            Netlist::parse(&format!("{CIRCUIT}{cards}\n.end\n")).expect("PSS deck parses");
        match netlist.analyses.into_iter().next() {
            Some(AnalysisCommand::Pss(card)) => *card,
            other => panic!("expected a .PSS card, got {other:?}"),
        }
    }

    #[test]
    fn the_card_defaults_are_the_configuration_defaults() {
        // The netlist layer cannot read these constants from here, so this is
        // what keeps the two copies from drifting apart.
        assert_eq!(
            PssConfig::from(&PssCard::driven(1.0e9)),
            PssConfig::new(1.0e9)
        );
        assert_eq!(
            PssConfig::from(&PssCard::autonomous()),
            PssConfig::autonomous()
        );
    }

    #[test]
    fn an_authored_driven_card_converts_to_the_direct_configuration() {
        assert_eq!(
            PssConfig::from(&card(".PSS FUND=1G")),
            PssConfig::new(1.0e9)
        );
    }

    #[test]
    fn an_authored_autonomous_card_converts_to_the_direct_configuration() {
        let mut expected = PssConfig::autonomous();
        expected.period_guess = 1e-9;
        expected.fundamental_freq = 1.0 / 1e-9;
        expected.oscillator_node = Some("OUT".to_string());
        expected.num_harmonics = 12;
        assert_eq!(
            PssConfig::from(&card(
                ".PSS AUTONOMOUS=TRUE PERIODGUESS=1n OSCNODE=OUT HARMS=12"
            )),
            expected
        );
    }

    #[test]
    fn every_authored_field_survives_the_conversion() {
        let converted = PssConfig::from(&card(
            ".pss fund=2.5g harms=15 tstab=3n tstabperiods=7 maxiter=250 tol=1e-8 \
             abstol=1e-15 damping=0.75 maxperiodchange=0.25 points=1024 method=trap verbose=true",
        ));
        assert_eq!(converted.fundamental_freq, 2.5e9);
        assert_eq!(converted.num_harmonics, 15);
        assert_eq!(converted.tstab_periods, 7);
        assert_eq!(converted.max_iterations, 250);
        assert_eq!(converted.tolerance, 1e-8);
        assert_eq!(converted.abstol, 1e-15);
        assert_eq!(converted.damping_factor, 0.75);
        assert_eq!(converted.max_period_change, 0.25);
        assert_eq!(converted.points_per_period, 1024);
        assert_eq!(
            converted.integration_method,
            Some(crate::numerics::integration::IntegrationMethod::Trapezoidal)
        );
        assert!(converted.verbose);
        assert!(converted.validate().is_ok());
    }

    #[test]
    fn the_ngspice_positional_card_converts_to_an_autonomous_configuration() {
        let converted = PssConfig::from(&card(".pss 3.1e6 500e-6 out 256 10 50"));
        assert!(converted.is_autonomous());
        assert_eq!(converted.fundamental_freq, 3.1e6);
        assert_eq!(converted.period_guess, 1.0 / 3.1e6);
        assert_eq!(converted.tstab, 500e-6);
        assert_eq!(converted.oscillator_node.as_deref(), Some("OUT"));
        assert_eq!(converted.points_per_period, 256);
        assert_eq!(converted.num_harmonics, 10);
        assert_eq!(converted.max_iterations, 50);
        assert!(converted.validate().is_ok());
    }

    #[test]
    fn a_card_the_parser_accepted_always_passes_the_configuration_validator() {
        for source in [
            ".PSS FUND=1G",
            ".PSS FUND=1G HARMS=128 POINTS=256",
            ".PSS AUTONOMOUS=TRUE",
            ".PSS AUTONOMOUS=TRUE PERIODGUESS=1n OSCNODE=out",
            ".PSS FUND=1G DAMPING=0.1 TOL=1e-12 ABSTOL=1e-18 MAXITER=1",
            ".pss 1e6 0 osc 32 16 40",
        ] {
            let converted = PssConfig::from(&card(source));
            assert!(
                converted.validate().is_ok(),
                "'{source}' converted to a configuration the validator rejects: {:?}",
                converted.validate()
            );
        }
    }
}
