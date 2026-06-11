//! Harmonic Balance Configuration
//!
//! Defines configuration parameters for HB analysis, including multi-tone
//! setup, convergence tolerances, and FFT sizing.

use crate::Value;

/// Configuration for a single tone in Harmonic Balance analysis
#[derive(Debug, Clone)]
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
/// use rspice_core::analysis::advanced::harmonic_balance::HbConfig;
///
/// let config = HbConfig::new(1e9)  // 1 GHz fundamental
///     .with_harmonics(9)           // DC through 9th harmonic
///     .with_tolerance(1e-6);
/// ```
///
/// ## Multi-tone analysis (mixer)
/// ```
/// use rspice_core::analysis::advanced::harmonic_balance::{HbConfig, HbTone};
///
/// let config = HbConfig::multi_tone(vec![
///     HbTone::new(900e6, 5).with_name("RF"),   // 900 MHz RF
///     HbTone::new(800e6, 5).with_name("LO"),   // 800 MHz LO
/// ])
/// .with_tolerance(1e-6);
/// ```
#[derive(Debug, Clone)]
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

    /// Maximum intermodulation order for multi-tone
    /// Limits the number of mixing products considered
    pub max_mixing_order: usize,

    /// Force the Krylov (GMRES + block-Jacobi) Newton-step solver.
    ///
    /// `false` is automatic: systems with ≥ 256 unknowns (nodes × spectral
    /// components) use Krylov, smaller systems use exact dense elimination.
    /// Krylov stagnation always falls back to the dense solve, so this
    /// switch affects speed only, never convergence.
    pub use_krylov: bool,

    /// GMRES restart parameter for the Krylov Newton-step solver.
    pub gmres_restart: usize,

    /// Enable source stepping for difficult convergence
    pub source_stepping: bool,

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
            max_mixing_order: 5,
            use_krylov: false,
            gmres_restart: 30,
            source_stepping: false,
            verbose: false,
        }
    }

    /// Create a multi-tone HB configuration
    ///
    /// # Arguments
    /// * `tones` - Vector of tone configurations
    pub fn multi_tone(tones: Vec<HbTone>) -> Self {
        let fundamental = tones.first().map(|t| t.frequency).unwrap_or(1e9);
        let max_harmonics = tones.iter().map(|t| t.num_harmonics).max().unwrap_or(9);

        Self {
            fundamental_freq: fundamental,
            num_harmonics: max_harmonics,
            tones,
            ..Self::new(fundamental)
        }
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
        self.oversample_factor = factor.max(1);
        self
    }

    /// Set maximum mixing order for multi-tone
    pub fn with_max_mixing_order(mut self, order: usize) -> Self {
        self.max_mixing_order = order;
        self
    }

    /// Enable Krylov subspace solver
    pub fn with_krylov(mut self, enable: bool) -> Self {
        self.use_krylov = enable;
        self
    }

    /// Enable source stepping
    pub fn with_source_stepping(mut self, enable: bool) -> Self {
        self.source_stepping = enable;
        self
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
        if self.is_multi_tone() {
            // For multi-tone, use box truncation
            // This is a simplified count; actual count depends on mixing order
            let mut count = 1; // DC
            for tone in &self.tones {
                count += 2 * tone.num_harmonics; // +/- harmonics
            }
            count
        } else {
            // Single-tone: DC + positive harmonics (negative are conjugates)
            self.num_harmonics + 1
        }
    }

    /// Get FFT size for time-domain evaluation
    pub fn fft_size(&self) -> usize {
        // Power of 2 for efficient FFT
        let min_size = self.num_spectral_components() * self.oversample_factor;
        min_size.next_power_of_two()
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
