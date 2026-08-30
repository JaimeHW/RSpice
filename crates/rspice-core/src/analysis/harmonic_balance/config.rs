//! Harmonic Balance Configuration
//!
//! Defines configuration parameters for HB analysis, including multi-tone
//! setup, convergence tolerances, and FFT sizing.

use crate::Value;

/// Maximum collocation-grid size accepted by the standalone HB numerical
/// kernel.
///
/// Engine clients can impose a smaller resource limit.  This hard ceiling is
/// also enforced by the public solver constructor so an unauthenticated
/// configuration cannot ask `rustfft` to allocate an effectively unbounded
/// plan before the caller has a chance to receive an error.
pub(crate) const MAX_HB_COLLOCATION_POINTS: usize = 2_000_000;

/// Maximum requested Newton iterations accepted by one HB analysis.
pub(crate) const MAX_HB_ITERATIONS: usize = 1_000_000;

/// Maximum authored intermodulation order.
pub(crate) const MAX_HB_MIXING_ORDER: usize = 4096;

/// Maximum useful GMRES restart.  The shared Krylov implementation retains at
/// most this many Arnoldi vectors.
pub(crate) const MAX_HB_GMRES_RESTART: usize = 64;

/// A malformed harmonic-balance configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HbConfigError {
    field: &'static str,
    detail: String,
}

impl HbConfigError {
    pub(crate) fn new(field: &'static str, detail: impl Into<String>) -> Self {
        Self {
            field,
            detail: detail.into(),
        }
    }

    /// Configuration field that violated the numerical contract.
    pub fn field(&self) -> &'static str {
        self.field
    }

    /// Human-readable description of the violated invariant.
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl std::fmt::Display for HbConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.field, self.detail)
    }
}

impl std::error::Error for HbConfigError {}

/// Configuration for a single tone in Harmonic Balance analysis
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct HbTone {
    /// Tone frequency in Hz
    pub frequency: Value,
    /// Number of harmonics to include for this tone
    pub num_harmonics: usize,
    /// Tone name (for identification in results)
    pub name: String,
    /// Optional source name filter.
    ///
    /// When set, this tone only drives independent sources with a matching name.
    /// When omitted, the tone is broadcast to all AC-capable independent sources.
    pub source_name: Option<String>,
}

impl HbTone {
    /// Create a new tone configuration
    pub fn new(frequency: Value, num_harmonics: usize) -> Self {
        Self {
            frequency,
            num_harmonics,
            name: format!("f{:.3e}", frequency),
            source_name: None,
        }
    }

    /// Set the tone name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the independent source name this tone should drive.
    pub fn with_source(mut self, source_name: impl Into<String>) -> Self {
        let source_name = source_name.into();
        self.source_name = if source_name.trim().is_empty() {
            None
        } else {
            Some(source_name)
        };
        self
    }
}

/// Configuration for Harmonic Balance analysis
///
/// HB analysis finds the periodic steady-state solution by solving for
/// Fourier coefficients directly in the frequency domain.
///
/// # Examples
///
/// ## Single-tone analysis
/// ```
/// use rspice_core::analysis::harmonic_balance::HbConfig;
///
/// let config = HbConfig::new(1e9)  // 1 GHz fundamental
///     .with_harmonics(9)           // DC through 9th harmonic
///     .with_tolerance(1e-6);
/// ```
///
/// ## Multi-tone analysis (mixer)
/// ```
/// use rspice_core::analysis::harmonic_balance::{HbConfig, HbTone};
///
/// let config = HbConfig::multi_tone(vec![
///     HbTone::new(900e6, 5).with_name("RF"),   // 900 MHz RF
///     HbTone::new(800e6, 5).with_name("LO"),   // 800 MHz LO
/// ])
/// .with_tolerance(1e-6);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct HbConfig {
    /// Primary fundamental frequency (Hz)
    /// For single-tone analysis, this is the only frequency.
    pub fundamental_freq: Value,

    /// Number of harmonics for primary tone (including DC)
    /// Total spectral components = num_harmonics + 1 for single-tone
    pub num_harmonics: usize,

    /// Additional tones for multi-tone analysis
    /// Empty for single-tone analysis
    pub tones: Vec<HbTone>,

    /// Newton convergence tolerance (relative)
    /// Iteration stops when ||F(X)||/||X|| < tolerance
    pub tolerance: Value,

    /// Absolute tolerance for small signals
    pub abstol: Value,

    /// Maximum Newton iterations
    pub max_iterations: usize,

    /// Newton damping factor (0 < damping <= 1)
    /// Values < 1 provide more conservative updates
    pub damping: Value,

    /// Minimum damping factor for adaptive damping
    pub min_damping: Value,

    /// Oversampling factor for FFT (anti-aliasing)
    /// 2 = 2x oversampling, 4 = 4x, etc.
    /// Higher values reduce aliasing in nonlinear evaluation
    pub oversample_factor: usize,

    /// Optional exact number of time-domain collocation points.
    ///
    /// When unset, the solver chooses an oversampled power-of-two FFT grid.
    /// Set this for simulator-compatible minimal odd grids such as Xyce's
    /// `2 * NUMFREQ + 1` HB discretization. The engine validates that the
    /// grid is odd and can represent every configured harmonic.
    pub collocation_points: Option<usize>,

    /// Maximum intermodulation order for multi-tone
    /// Limits the number of mixing products considered
    pub max_mixing_order: usize,

    /// Force the Krylov solver.
    ///
    /// `false` is automatic: systems with ≥ 256 unknowns (nodes × spectral
    /// components) use Krylov, smaller systems use exact dense elimination.
    /// Shared Arnoldi storage is bounded independently of system dimension.
    /// Each consuming analysis owns its preconditioner, numerical qualification,
    /// and recovery policy.
    pub use_krylov: bool,

    /// Requested GMRES restart parameter for HB, PAC, and PNoise Krylov solves.
    ///
    /// The canonical configuration contract accepts 1 through 64. The shared
    /// solver further bounds this to the system dimension; requests below
    /// eight retain the historical minimum restart of eight whenever the
    /// dimension permits it. The default is 30.
    pub gmres_restart: usize,

    /// Enable source stepping for difficult convergence
    pub source_stepping: bool,

    /// Solve Newton steps with the exact real-split Jacobian (Toeplitz plus
    /// conjugate/Hankel coupling). `false` selects the legacy Toeplitz-only
    /// complex Jacobian, kept for A/B comparison and the large-system Krylov
    /// fast path; both converge to identical spectra, the exact path just
    /// gets there in fewer iterations.
    pub use_exact_jacobian: bool,

    /// Verbose logging
    pub verbose: bool,
}

impl HbConfig {
    /// Create a new single-tone HB configuration
    ///
    /// # Arguments
    /// * `fundamental_freq` - Fundamental frequency in Hz
    pub fn new(fundamental_freq: Value) -> Self {
        Self {
            fundamental_freq,
            num_harmonics: 9,
            tones: Vec::new(),
            tolerance: 1e-6,
            abstol: 1e-12,
            max_iterations: 100,
            damping: 1.0,
            min_damping: 0.1,
            oversample_factor: 2,
            collocation_points: None,
            max_mixing_order: 5,
            use_krylov: false,
            gmres_restart: 30,
            source_stepping: false,
            use_exact_jacobian: true,
            verbose: false,
        }
    }

    /// Create a multi-tone HB configuration
    ///
    /// The spectral basis is the common fundamental of all tones (their
    /// approximate greatest common divisor), so every tone lands on an
    /// integer harmonic: 900 MHz + 800 MHz resolves to a 100 MHz basis with
    /// the tones at harmonics 9 and 8. The harmonic count covers each tone's
    /// requested order against that basis. Taking the first tone's frequency
    /// as the basis (the previous behaviour) rejected every genuinely
    /// multi-tone configuration at run time.
    ///
    /// # Arguments
    /// * `tones` - Vector of tone configurations
    pub fn multi_tone(tones: Vec<HbTone>) -> Self {
        let basis = Self::common_basis(&tones);
        let num_harmonics = tones
            .iter()
            .map(|t| {
                let tone_harmonic = if basis > 0.0 {
                    (t.frequency / basis).round().max(1.0) as usize
                } else {
                    1
                };
                tone_harmonic.saturating_mul(t.num_harmonics.max(1))
            })
            .max()
            .unwrap_or(9)
            .min(4096);

        Self {
            fundamental_freq: basis,
            num_harmonics,
            tones,
            ..Self::new(basis)
        }
    }

    /// Approximate greatest common divisor of the tone frequencies.
    fn common_basis(tones: &[HbTone]) -> Value {
        fn float_gcd(a: Value, b: Value) -> Value {
            let (mut a, mut b) = (a.abs(), b.abs());
            let tol = 1e-9 * a.max(b).max(f64::MIN_POSITIVE);
            while b > tol {
                let r = a % b;
                a = b;
                b = r;
            }
            a
        }

        let mut basis = tones.first().map(|t| t.frequency).unwrap_or(1e9);
        for tone in tones.iter().skip(1) {
            basis = float_gcd(basis, tone.frequency);
        }
        basis
    }

    /// Set number of harmonics
    pub fn with_harmonics(mut self, n: usize) -> Self {
        self.num_harmonics = n.max(1);
        self
    }

    /// Set convergence tolerance
    pub fn with_tolerance(mut self, tol: Value) -> Self {
        self.tolerance = tol;
        self
    }

    /// Set maximum iterations
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    /// Set Newton damping factor
    pub fn with_damping(mut self, damping: Value) -> Self {
        self.damping = damping.clamp(0.1, 1.0);
        self
    }

    /// Set oversampling factor for FFT
    pub fn with_oversample(mut self, factor: usize) -> Self {
        self.oversample_factor = factor.max(2);
        self
    }

    /// Use an exact odd time-domain collocation grid.
    pub fn with_collocation_points(mut self, points: usize) -> Self {
        self.collocation_points = Some(points);
        self
    }

    /// Smallest odd collocation grid that can represent DC and every
    /// configured positive and negative harmonic.
    ///
    /// Returns `None` when the configured harmonic count cannot be
    /// represented by `usize`; callers should reject that configuration.
    pub fn minimum_collocation_points(&self) -> Option<usize> {
        self.num_harmonics.checked_mul(2)?.checked_add(1)
    }

    /// Validate every numerical invariant consumed by the HB engine and
    /// standalone solver.
    ///
    /// Callers may construct `HbConfig` with struct literals or mutate its
    /// public fields, so builder-method clamping is not an authentication
    /// boundary.  This method is the canonical contract used before FFT
    /// planning, solver construction, and retained-state reconstruction.
    pub(crate) fn validate(&self) -> Result<(), HbConfigError> {
        if !self.fundamental_freq.is_finite() || self.fundamental_freq <= 0.0 {
            return Err(HbConfigError::new(
                "fundamental_freq",
                "must be finite and positive",
            ));
        }
        if !self.fundamental_freq.recip().is_finite() {
            return Err(HbConfigError::new(
                "fundamental_freq",
                "must have a finite representable period",
            ));
        }
        if self.num_harmonics == 0 {
            return Err(HbConfigError::new("num_harmonics", "must be at least one"));
        }
        let minimum_points = self.minimum_collocation_points().ok_or_else(|| {
            HbConfigError::new(
                "num_harmonics",
                "overflows the addressable collocation grid",
            )
        })?;
        if minimum_points > MAX_HB_COLLOCATION_POINTS {
            return Err(HbConfigError::new(
                "num_harmonics",
                format!(
                    "requires {minimum_points} collocation points, above the supported limit {MAX_HB_COLLOCATION_POINTS}"
                ),
            ));
        }
        let highest_frequency = self.fundamental_freq * self.num_harmonics as Value;
        if !highest_frequency.is_finite()
            || !(std::f64::consts::TAU * highest_frequency).is_finite()
        {
            return Err(HbConfigError::new(
                "fundamental_freq",
                "and num_harmonics produce a non-finite angular frequency",
            ));
        }

        for (field, value) in [("tolerance", self.tolerance), ("abstol", self.abstol)] {
            if !value.is_finite() || value <= 0.0 {
                return Err(HbConfigError::new(
                    field,
                    "must be finite and greater than zero",
                ));
            }
        }
        if self.max_iterations == 0 || self.max_iterations > MAX_HB_ITERATIONS {
            return Err(HbConfigError::new(
                "max_iterations",
                format!("must be in 1..={MAX_HB_ITERATIONS}"),
            ));
        }
        if !self.damping.is_finite() || self.damping <= 0.0 || self.damping > 1.0 {
            return Err(HbConfigError::new(
                "damping",
                "must be finite and in (0, 1]",
            ));
        }
        if !self.min_damping.is_finite()
            || self.min_damping <= 0.0
            || self.min_damping > self.damping
        {
            return Err(HbConfigError::new(
                "min_damping",
                "must be finite, greater than zero, and no greater than damping",
            ));
        }
        if self.oversample_factor < 2 {
            return Err(HbConfigError::new(
                "oversample_factor",
                "must be at least two",
            ));
        }
        if self.max_mixing_order == 0 || self.max_mixing_order > MAX_HB_MIXING_ORDER {
            return Err(HbConfigError::new(
                "max_mixing_order",
                format!("must be in 1..={MAX_HB_MIXING_ORDER}"),
            ));
        }
        if self.gmres_restart == 0 || self.gmres_restart > MAX_HB_GMRES_RESTART {
            return Err(HbConfigError::new(
                "gmres_restart",
                format!("must be in 1..={MAX_HB_GMRES_RESTART}"),
            ));
        }

        for (index, tone) in self.tones.iter().enumerate() {
            if !tone.frequency.is_finite() || tone.frequency <= 0.0 {
                return Err(HbConfigError::new(
                    "tones",
                    format!("tone {index} frequency must be finite and greater than zero"),
                ));
            }
            if tone.num_harmonics == 0 {
                return Err(HbConfigError::new(
                    "tones",
                    format!("tone {index} must retain at least one harmonic"),
                ));
            }
            let ratio = tone.frequency / self.fundamental_freq;
            let harmonic = ratio.round();
            let relative_error = (ratio - harmonic).abs() / harmonic.abs().max(1.0);
            if !ratio.is_finite()
                || !harmonic.is_finite()
                || harmonic < 1.0
                || relative_error > 1.0e-9
                || harmonic > usize::MAX as Value
            {
                return Err(HbConfigError::new(
                    "tones",
                    format!(
                        "tone {index} frequency must be a positive integer harmonic of fundamental_freq"
                    ),
                ));
            }
            let harmonic = harmonic as usize;
            let required = harmonic.checked_mul(tone.num_harmonics).ok_or_else(|| {
                HbConfigError::new("tones", format!("tone {index} harmonic order overflows"))
            })?;
            if required > self.num_harmonics {
                return Err(HbConfigError::new(
                    "tones",
                    format!(
                        "tone {index} requires common-basis harmonic {required}, beyond num_harmonics {}",
                        self.num_harmonics
                    ),
                ));
            }
        }

        self.checked_fft_size().map(|_| ())
    }

    /// Enable verbose logging
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Check if this is a multi-tone analysis
    pub fn is_multi_tone(&self) -> bool {
        !self.tones.is_empty()
    }

    /// Get total number of spectral components per node
    pub fn num_spectral_components(&self) -> usize {
        self.checked_num_spectral_components().unwrap_or(usize::MAX)
    }

    fn checked_num_spectral_components(&self) -> Result<usize, HbConfigError> {
        if self.is_multi_tone() {
            self.tones.iter().try_fold(1usize, |count, tone| {
                tone.num_harmonics
                    .checked_mul(2)
                    .and_then(|components| count.checked_add(components))
                    .ok_or_else(|| {
                        HbConfigError::new(
                            "tones",
                            "spectral-component count exceeds this platform",
                        )
                    })
            })
        } else {
            self.num_harmonics.checked_add(1).ok_or_else(|| {
                HbConfigError::new(
                    "num_harmonics",
                    "spectral-component count exceeds this platform",
                )
            })
        }
    }

    /// Get FFT size for time-domain evaluation
    pub fn fft_size(&self) -> usize {
        self.checked_fft_size().unwrap_or(usize::MAX)
    }

    /// Return the exact FFT/collocation size without saturating arithmetic.
    pub(crate) fn checked_fft_size(&self) -> Result<usize, HbConfigError> {
        let minimum_points = self.minimum_collocation_points().ok_or_else(|| {
            HbConfigError::new(
                "num_harmonics",
                "overflows the addressable collocation grid",
            )
        })?;
        let fft_size = if let Some(points) = self.collocation_points {
            if points % 2 == 0 {
                return Err(HbConfigError::new(
                    "collocation_points",
                    "collocation grid must be odd",
                ));
            }
            if points < minimum_points {
                return Err(HbConfigError::new(
                    "collocation_points",
                    format!("collocation grid must contain at least {minimum_points} points"),
                ));
            }
            points
        } else {
            let spectral_components = self.checked_num_spectral_components()?;
            let oversampled = spectral_components
                .checked_mul(self.oversample_factor)
                .ok_or_else(|| {
                    HbConfigError::new(
                        "oversample_factor",
                        "overflows the addressable collocation grid",
                    )
                })?;
            oversampled
                .max(minimum_points)
                .checked_next_power_of_two()
                .ok_or_else(|| {
                    HbConfigError::new(
                        "oversample_factor",
                        "requires a collocation grid too large for this platform",
                    )
                })?
        };
        if fft_size > MAX_HB_COLLOCATION_POINTS {
            return Err(HbConfigError::new(
                if self.collocation_points.is_some() {
                    "collocation_points"
                } else {
                    "oversample_factor"
                },
                format!(
                    "requires {fft_size} points, above the supported limit {MAX_HB_COLLOCATION_POINTS}"
                ),
            ));
        }
        Ok(fft_size)
    }

    /// Get the fundamental period
    pub fn period(&self) -> Value {
        if self.fundamental_freq > 0.0 {
            1.0 / self.fundamental_freq
        } else {
            1.0
        }
    }

    /// Get all harmonic frequencies for single-tone
    pub fn harmonic_frequencies(&self) -> Vec<Value> {
        (0..=self.num_harmonics)
            .map(|k| k as Value * self.fundamental_freq)
            .collect()
    }
}

impl Default for HbConfig {
    fn default() -> Self {
        Self::new(1e9) // 1 GHz default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_tone_derives_the_common_basis() {
        // The mixer case from the type documentation: 900 + 800 MHz.
        let config = HbConfig::multi_tone(vec![
            HbTone::new(900e6, 5).with_name("RF"),
            HbTone::new(800e6, 5).with_name("LO"),
        ]);
        assert!(
            (config.fundamental_freq - 100e6).abs() < 1.0,
            "basis must be the 100 MHz common fundamental, got {}",
            config.fundamental_freq
        );
        // Both tones map to exact integer harmonics of the basis.
        for tone in &config.tones {
            let ratio = tone.frequency / config.fundamental_freq;
            assert!(
                (ratio - ratio.round()).abs() < 1e-9,
                "tone {} must land on an integer harmonic, got ratio {}",
                tone.name,
                ratio
            );
            let harmonic = ratio.round() as usize;
            assert!(
                harmonic * tone.num_harmonics <= config.num_harmonics,
                "harmonic budget must cover tone {} order {} at harmonic {}",
                tone.name,
                tone.num_harmonics,
                harmonic
            );
        }
    }

    #[test]
    fn multi_tone_with_harmonically_related_tones_keeps_the_lower_tone() {
        let config = HbConfig::multi_tone(vec![HbTone::new(1e9, 4), HbTone::new(2e9, 3)]);
        assert!(
            (config.fundamental_freq - 1e9).abs() < 1.0,
            "basis of harmonically related tones is the lower tone, got {}",
            config.fundamental_freq
        );
        assert!(config.num_harmonics >= 6);
    }

    #[test]
    fn multi_tone_fft_grid_represents_the_highest_common_basis_harmonic() {
        let config = HbConfig::multi_tone(vec![HbTone::new(900e6, 5), HbTone::new(800e6, 5)]);
        assert!(
            config.fft_size() > 2 * config.num_harmonics,
            "FFT grid {} cannot represent +/- harmonic {}",
            config.fft_size(),
            config.num_harmonics
        );
    }
}
