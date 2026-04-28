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
/// use rspice_core::analysis::advanced::pss::PssConfig;
///
/// let config = PssConfig::new(1e9)  // 1 GHz fundamental
///     .with_harmonics(9)
///     .with_tstab(10e-9);
/// ```
///
/// ## Autonomous Oscillator (period detection)
/// ```
/// use rspice_core::analysis::advanced::pss::PssConfig;
///
/// let config = PssConfig::autonomous()
///     .with_period_guess(1e-9)  // ~1 GHz expected
///     .with_harmonics(15);
/// ```
#[derive(Debug, Clone)]
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
    pub integration_method: Option<crate::analysis::IntegrationMethod>,

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
    /// use rspice_core::analysis::advanced::pss::PssConfig;
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
    /// use rspice_core::analysis::advanced::pss::PssConfig;
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

    /// Set Newton damping factor.
    ///
    /// Values < 1.0 provide more conservative updates.
    /// Useful for circuits with difficult convergence.
    pub fn with_damping(mut self, factor: Value) -> Self {
        self.damping_factor = factor.clamp(0.1, 1.0);
        self
    }

    /// Set integration method for transient shooting.
    pub fn with_integration_method(mut self, method: crate::analysis::IntegrationMethod) -> Self {
        self.integration_method = Some(method);
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

impl Default for PssConfig {
    fn default() -> Self {
        Self::new(1e9) // 1 GHz default
    }
}

