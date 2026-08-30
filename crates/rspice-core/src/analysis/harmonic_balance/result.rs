//! Harmonic Balance Result Types
//!
//! Defines the output data structures for HB analysis, including spectral
//! voltages, harmonic data, and power calculations.

use crate::Value;
use num_complex::Complex64;
use std::collections::BTreeSet;

/// One HB branch-current spectrum retained from an actual MNA branch unknown.
///
/// The coefficients use the same public convention as [`SpectralVoltage`]:
/// DC is the real harmonic-zero value and each positive-frequency coefficient
/// is the physical peak-amplitude phasor.
#[derive(Debug, Clone)]
pub struct SpectralBranchCurrent {
    /// Authored device name, when the engine could retain it.
    pub device_name: String,
    /// Complex current coefficients `[DC, H1, H2, ..., Hn]`, positive from the
    /// device's positive terminal into its negative terminal.
    pub coefficients: Vec<Complex64>,
    /// Harmonic frequencies corresponding to `coefficients`.
    pub frequencies: Vec<Value>,
}

/// Reactive element represented in a retained HB periodic state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HbReactiveKind {
    Capacitor,
    Inductor,
}

/// Voltage and current spectra for one reactive element.
#[derive(Debug, Clone)]
pub struct HbReactiveSpectrum {
    pub device_name: String,
    pub kind: HbReactiveKind,
    /// Positive-to-negative terminal-voltage phasors.
    pub voltage_coefficients: Vec<Complex64>,
    /// Positive-to-negative branch-current phasors.
    pub current_coefficients: Vec<Complex64>,
    /// Whether harmonic zero is a physically exact branch current. The
    /// production HB engine reports `true` for capacitor constitutive current
    /// and exact inductor MNA branch current. `false` is retained only for
    /// compatibility with legacy or manually constructed results.
    pub dc_current_is_exact: bool,
}

/// A known reason an HB phase projection is not a complete physical state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HbContinuationLimitation {
    /// Legacy compatibility marker for HB states produced with a stiff Norton
    /// voltage-source equivalent. The production engine no longer emits it.
    NonlinearVoltageSourcesUseNortonEquivalent,
    /// Legacy compatibility marker for an inductor DC current reconstructed
    /// from a finite-conductance short. The production engine no longer emits
    /// it.
    InductorDcCurrentUsesShortSurrogate,
    /// A Verilog-A device may own dynamic state that node-voltage spectra do
    /// not describe. Consumers must not assume that state can be restored.
    VerilogAInternalStateNotRetained,
}

/// One reactive element evaluated at a carrier phase.
#[derive(Debug, Clone)]
pub struct HbReactivePhaseState {
    pub device_name: String,
    pub kind: HbReactiveKind,
    pub voltage: Value,
    pub current: Value,
    pub current_is_exact: bool,
}

/// A generic periodic-state projection at one carrier phase.
///
/// This is deliberately not an authenticated Envelope initializer. Envelope
/// execution must separately prove that its selected modulation sources were
/// frozen for the carrier solve and are reactivated at slow-time origin zero.
#[derive(Debug, Clone)]
pub struct HbPhaseState {
    pub phase_radians: Value,
    pub node_voltages: Vec<(String, Value)>,
    pub mna_branch_currents: Vec<(String, Value)>,
    pub reactive_states: Vec<HbReactivePhaseState>,
    pub limitations: Vec<HbContinuationLimitation>,
}

/// Error returned when a carrier-phase projection cannot be evaluated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HbPhaseProjectionError {
    NonFinitePhase,
    InvalidResult,
}

impl std::fmt::Display for HbPhaseProjectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonFinitePhase => write!(formatter, "HB projection phase must be finite"),
            Self::InvalidResult => write!(formatter, "HB result is not a valid periodic state"),
        }
    }
}

impl std::error::Error for HbPhaseProjectionError {}

impl HbPhaseState {
    /// True only when the HB runtime retained every state represented by the
    /// solved circuit. Envelope source-freeze authentication remains a
    /// separate caller responsibility even when this returns true.
    pub fn is_complete(&self) -> bool {
        self.limitations.is_empty()
    }
}

fn evaluate_one_sided_spectrum(coefficients: &[Complex64], phase_radians: Value) -> Value {
    let dc = coefficients
        .first()
        .map(|coefficient| coefficient.re)
        .unwrap_or(0.0);
    coefficients
        .iter()
        .enumerate()
        .skip(1)
        .fold(dc, |value, (harmonic, coefficient)| {
            let rotation = Complex64::from_polar(1.0, harmonic as Value * phase_radians);
            value + (*coefficient * rotation).re
        })
}

/// Spectral voltage data for a single node
#[derive(Debug, Clone)]
pub struct SpectralVoltage {
    /// Node name/identifier
    pub node_name: String,

    /// Complex Fourier coefficients [DC, H1, H2, ..., Hn]
    /// These are one-sided (positive frequencies only)
    pub coefficients: Vec<Complex64>,

    /// Harmonic frequencies corresponding to coefficients
    pub frequencies: Vec<Value>,
}

impl SpectralVoltage {
    /// Create new spectral voltage data
    pub fn new(node_name: impl Into<String>, num_harmonics: usize) -> Self {
        Self {
            node_name: node_name.into(),
            coefficients: vec![Complex64::new(0.0, 0.0); num_harmonics + 1],
            frequencies: Vec::new(),
        }
    }

    /// Get DC component (real value)
    pub fn dc(&self) -> Value {
        self.coefficients.first().map(|c| c.re).unwrap_or(0.0)
    }

    /// Get magnitude at harmonic k
    pub fn magnitude(&self, k: usize) -> Value {
        self.coefficients.get(k).map(|c| c.norm()).unwrap_or(0.0)
    }

    /// Get phase at harmonic k (radians)
    pub fn phase(&self, k: usize) -> Value {
        self.coefficients.get(k).map(|c| c.arg()).unwrap_or(0.0)
    }

    /// Get magnitude in dBV at harmonic k
    pub fn magnitude_dbv(&self, k: usize) -> Value {
        let mag = self.magnitude(k);
        if mag > 1e-15 {
            20.0 * mag.log10()
        } else {
            -300.0 // Floor
        }
    }

    /// Get peak-to-peak voltage (approximation from harmonics)
    pub fn peak_to_peak(&self) -> Value {
        // Sum of all harmonic amplitudes * 2 (approximate)
        let ac_sum: Value = self.coefficients.iter().skip(1).map(|c| c.norm()).sum();
        2.0 * ac_sum // Peak-to-peak of AC component
    }

    /// Get RMS voltage
    pub fn rms(&self) -> Value {
        // RMS = sqrt(DC² + Σ|Hk|²/2)
        let dc_sq = self.dc().powi(2);
        let ac_power: Value = self
            .coefficients
            .iter()
            .skip(1)
            .map(|c| c.norm_sqr() / 2.0)
            .sum();
        (dc_sq + ac_power).sqrt()
    }

    /// Get total harmonic distortion (THD) as ratio
    pub fn thd(&self) -> Value {
        let h1 = self.magnitude(1);
        if h1 < 1e-15 {
            return 0.0;
        }

        let harmonic_power: Value = self.coefficients.iter().skip(2).map(|c| c.norm_sqr()).sum();

        harmonic_power.sqrt() / h1
    }

    /// Get total harmonic distortion in percent
    pub fn thd_percent(&self) -> Value {
        self.thd() * 100.0
    }
}

/// Detailed data for a single harmonic
#[derive(Debug, Clone)]
pub struct HarmonicData {
    /// Harmonic index (0 = DC, 1 = fundamental, etc.)
    pub index: usize,

    /// Frequency in Hz
    pub frequency: Value,

    /// Complex coefficient
    pub coefficient: Complex64,

    /// Magnitude
    pub magnitude: Value,

    /// Phase in radians
    pub phase: Value,

    /// Magnitude in dBV
    pub magnitude_dbv: Value,

    /// Phase in degrees
    pub phase_degrees: Value,
}

impl HarmonicData {
    /// Create from coefficient
    pub fn from_coefficient(index: usize, frequency: Value, coeff: Complex64) -> Self {
        let magnitude = coeff.norm();
        let phase = coeff.arg();
        let magnitude_dbv = if magnitude > 1e-15 {
            20.0 * magnitude.log10()
        } else {
            -300.0
        };

        Self {
            index,
            frequency,
            coefficient: coeff,
            magnitude,
            phase,
            magnitude_dbv,
            phase_degrees: phase.to_degrees(),
        }
    }
}

/// Complete result of Harmonic Balance analysis
#[derive(Debug, Clone)]
pub struct HbResult {
    /// Convergence status
    pub converged: bool,

    /// Number of Newton iterations
    pub iterations: usize,

    /// Final residual norm
    pub residual_norm: Value,

    /// Fundamental frequency
    pub fundamental_freq: Value,

    /// Spectral voltages for each node
    pub spectral_voltages: Vec<SpectralVoltage>,

    /// Node names (for indexing)
    pub node_names: Vec<String>,

    /// Number of harmonics analyzed
    pub num_harmonics: usize,

    /// Harmonic frequencies
    pub harmonic_frequencies: Vec<Value>,

    /// Analysis time in seconds
    pub solve_time_seconds: Value,

    /// Multi-tone info (if applicable)
    pub tones: Vec<String>,

    /// Current spectra retained from the canonical ideal voltage-source and
    /// inductor MNA branch unknowns solved by linear or nonlinear HB.
    pub mna_branch_currents: Vec<SpectralBranchCurrent>,

    /// Named linear reactive spectra. Capacitor current is evaluated from its
    /// constitutive relation; inductor current is retained from its exact MNA
    /// branch unknown.
    pub reactive_spectra: Vec<HbReactiveSpectrum>,

    /// Reasons a projected state must not be treated as a complete physical
    /// continuation state.
    pub continuation_limitations: Vec<HbContinuationLimitation>,
}

impl HbResult {
    /// Create a new empty result
    pub fn new(fundamental_freq: Value, num_nodes: usize, num_harmonics: usize) -> Self {
        Self {
            converged: false,
            iterations: 0,
            residual_norm: f64::INFINITY,
            fundamental_freq,
            spectral_voltages: Vec::with_capacity(num_nodes),
            node_names: Vec::new(),
            num_harmonics,
            harmonic_frequencies: (0..=num_harmonics)
                .map(|k| k as f64 * fundamental_freq)
                .collect(),
            solve_time_seconds: 0.0,
            tones: Vec::new(),
            mna_branch_currents: Vec::new(),
            reactive_spectra: Vec::new(),
            continuation_limitations: Vec::new(),
        }
    }

    /// Evaluate retained node, MNA-current, and reactive state at a carrier
    /// phase. `phase_radians` is normalized modulo `2*pi` by the trigonometric
    /// evaluation and may be outside the principal interval.
    pub fn project_phase(
        &self,
        phase_radians: Value,
    ) -> Result<HbPhaseState, HbPhaseProjectionError> {
        if !phase_radians.is_finite() {
            return Err(HbPhaseProjectionError::NonFinitePhase);
        }
        if !self.is_valid() {
            return Err(HbPhaseProjectionError::InvalidResult);
        }
        let node_voltages = self
            .spectral_voltages
            .iter()
            .map(|voltage| {
                (
                    voltage.node_name.clone(),
                    evaluate_one_sided_spectrum(&voltage.coefficients, phase_radians),
                )
            })
            .collect();
        let mna_branch_currents = self
            .mna_branch_currents
            .iter()
            .map(|current| {
                (
                    current.device_name.clone(),
                    evaluate_one_sided_spectrum(&current.coefficients, phase_radians),
                )
            })
            .collect();
        let reactive_states = self
            .reactive_spectra
            .iter()
            .map(|reactive| HbReactivePhaseState {
                device_name: reactive.device_name.clone(),
                kind: reactive.kind,
                voltage: evaluate_one_sided_spectrum(&reactive.voltage_coefficients, phase_radians),
                current: evaluate_one_sided_spectrum(&reactive.current_coefficients, phase_radians),
                current_is_exact: reactive.dc_current_is_exact,
            })
            .collect();

        Ok(HbPhaseState {
            phase_radians,
            node_voltages,
            mna_branch_currents,
            reactive_states,
            limitations: self.continuation_limitations.clone(),
        })
    }

    /// Get number of nodes
    pub fn num_nodes(&self) -> usize {
        self.spectral_voltages.len()
    }

    /// Get DC operating point for all nodes
    pub fn dc_operating_point(&self) -> Vec<(String, Value)> {
        self.spectral_voltages
            .iter()
            .zip(self.node_names.iter())
            .map(|(sv, name)| (name.clone(), sv.dc()))
            .collect()
    }

    /// Check whether this is a complete, structurally consistent, finite HB
    /// solution that is safe to consume as a periodic state.
    pub fn is_valid(&self) -> bool {
        let Some(harmonic_count) = self.num_harmonics.checked_add(1) else {
            return false;
        };
        if !self.converged
            || self.num_harmonics == 0
            || !self.residual_norm.is_finite()
            || self.residual_norm < 0.0
            || !self.fundamental_freq.is_finite()
            || self.fundamental_freq <= 0.0
            || !self.solve_time_seconds.is_finite()
            || self.solve_time_seconds < 0.0
            || self.harmonic_frequencies.len() != harmonic_count
            || self.node_names.is_empty()
            || self.spectral_voltages.len() != self.node_names.len()
        {
            return false;
        }
        if !self
            .harmonic_frequencies
            .iter()
            .enumerate()
            .all(|(harmonic, &frequency)| {
                frequency.is_finite() && frequency == harmonic as Value * self.fundamental_freq
            })
        {
            return false;
        }

        let mut unique_nodes = BTreeSet::new();
        if !self
            .spectral_voltages
            .iter()
            .enumerate()
            .all(|(node, spectrum)| {
                let Some(name) = self.node_names.get(node) else {
                    return false;
                };
                !name.is_empty()
                    && name.trim() == name
                    && name == &spectrum.node_name
                    && unique_nodes.insert(name.to_ascii_lowercase())
                    && spectrum.coefficients.len() == harmonic_count
                    && spectrum.frequencies == self.harmonic_frequencies
                    && spectrum
                        .coefficients
                        .iter()
                        .all(|coefficient| coefficient.re.is_finite() && coefficient.im.is_finite())
                    && spectrum
                        .coefficients
                        .first()
                        .is_some_and(|coefficient| coefficient.im == 0.0)
            })
        {
            return false;
        }

        let mut unique_branches = BTreeSet::new();
        if !self.mna_branch_currents.iter().all(|branch| {
            !branch.device_name.is_empty()
                && branch.device_name.trim() == branch.device_name
                && unique_branches.insert(branch.device_name.to_ascii_lowercase())
                && branch.coefficients.len() == harmonic_count
                && branch.frequencies == self.harmonic_frequencies
                && branch
                    .coefficients
                    .iter()
                    .all(|coefficient| coefficient.re.is_finite() && coefficient.im.is_finite())
                && branch
                    .coefficients
                    .first()
                    .is_some_and(|coefficient| coefficient.im == 0.0)
        }) {
            return false;
        }

        let mut unique_reactive = BTreeSet::new();
        self.reactive_spectra.iter().all(|reactive| {
            !reactive.device_name.is_empty()
                && reactive.device_name.trim() == reactive.device_name
                && unique_reactive.insert(reactive.device_name.to_ascii_lowercase())
                && reactive.voltage_coefficients.len() == harmonic_count
                && reactive.current_coefficients.len() == harmonic_count
                && reactive
                    .voltage_coefficients
                    .iter()
                    .chain(reactive.current_coefficients.iter())
                    .all(|coefficient| coefficient.re.is_finite() && coefficient.im.is_finite())
                && reactive
                    .voltage_coefficients
                    .first()
                    .is_some_and(|coefficient| coefficient.im == 0.0)
                && reactive
                    .current_coefficients
                    .first()
                    .is_some_and(|coefficient| coefficient.im == 0.0)
        })
    }
}
