//! Stability (STB) Analysis Module
//!
//! STB analysis evaluates the stability of feedback loops by extracting loop gain
//! and computing gain/phase margins. This is essential for:
//!
//! - **Op-amp design**: Unity-gain stability, compensation verification
//! - **Voltage regulator**: Load and line transient stability
//! - **PLL design**: Loop filter optimization
//! - **Power supply**: Feedback loop stability under varying loads
//!
//! # Theory
//!
//! Loop stability is analyzed by breaking the feedback loop and measuring:
//! - **Loop gain L(s)**: The gain around the feedback loop
//! - **Phase margin (PM)**: 180° + ∠L(jω) at |L(jω)| = 1 (0 dB)
//! - **Gain margin (GM)**: 1/|L(jω)| at ∠L(jω) = -180°
//!
//! # Methods
//!
//! 1. **Middlebrook method**: Insert voltage/current probe at break point
//! 2. **Tian method**: Extract return ratio without physical break
//! 3. **Direct loop break**: Insert ideal transformer/gyrator
//!
//! # Stability Criteria
//!
//! Reported margins describe crossings resolved within the swept band.
//! Closed-loop stability additionally requires qualified pole or Nyquist
//! evidence; positive margins alone do not establish it.

mod circuit_poles;
pub use circuit_poles::{CircuitPoleEvidence, CircuitPoleFailure};

use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::frequency_grid::{
    FrequencyGridError, FrequencyGridScale, frequency_point_count, generate_frequency_grid,
};
use num_complex::Complex64;
use std::f64::consts::PI;
use std::fmt::Write as _;

//=============================================================================
// STB Configuration
//=============================================================================

/// Configuration for Stability (STB) analysis
#[derive(Debug, Clone)]
pub struct StbConfig {
    /// Start frequency for loop gain sweep (Hz)
    pub freq_start: Value,

    /// Stop frequency for loop gain sweep (Hz)
    pub freq_stop: Value,

    /// Number of frequency points
    pub num_points: usize,

    /// Sweep type
    pub sweep_type: StbSweepType,

    /// Break point node name (where loop is probed)
    pub probe_node: Option<String>,

    /// Reference node
    pub ref_node: String,

    /// Whether to compute Nyquist data
    pub compute_nyquist: bool,
}

/// Sweep type for STB analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StbSweepType {
    /// Linear frequency sweep (`num_points` total)
    Linear,
    /// Decade (logarithmic) sweep (`num_points` per decade)
    #[default]
    Decade,
    /// Octave (logarithmic) sweep (`num_points` per octave)
    Octave,
}

/// Invalid authored STB configuration.
///
/// The variants are structured so callers can classify configuration
/// failures without parsing display strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum StbConfigError {
    /// The sweep start was not finite and strictly positive.
    InvalidStartFrequency,
    /// The sweep stop was not finite or preceded the start.
    InvalidStopFrequency,
    /// No sweep points were requested.
    EmptySweep,
    /// A logarithmic sweep's implied point count exceeded `usize`.
    PointCountOverflow,
    /// The analysis card handed to the conversion was not a `.STB` card.
    NotAnStbCard,
}

impl std::fmt::Display for StbConfigError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidStartFrequency => {
                formatter.write_str("STB start frequency must be positive and finite")
            }
            Self::InvalidStopFrequency => {
                formatter.write_str("STB stop frequency must be finite and >= start")
            }
            Self::EmptySweep => formatter.write_str("STB sweep must have at least one point"),
            Self::PointCountOverflow => {
                formatter.write_str("STB logarithmic sweep point count exceeds addressable limits")
            }
            Self::NotAnStbCard => {
                formatter.write_str("the analysis card given to StbConfig is not .STB")
            }
        }
    }
}

impl std::error::Error for StbConfigError {}

/// The one place an authored `.STB` card becomes a configuration.
///
/// `rspice run`, the Python bindings, the WASM deck runner, the engine adapter
/// and the Studio all reach the analysis through this conversion, so a keyword
/// the card carries cannot be honoured on one route and dropped on another —
/// which is exactly what five hand-written builder chains used to do with the
/// Nyquist contour. The result is validated here, so a caller that gets a
/// configuration has one the engine will accept.
impl TryFrom<&crate::netlist::AnalysisCommand> for StbConfig {
    type Error = StbConfigError;

    fn try_from(card: &crate::netlist::AnalysisCommand) -> Result<Self, Self::Error> {
        let crate::netlist::AnalysisCommand::Stb {
            variation,
            points,
            start_freq,
            stop_freq,
            probe,
            compute_nyquist,
        } = card
        else {
            return Err(StbConfigError::NotAnStbCard);
        };
        let config = Self::new()
            .with_sweep(*start_freq, *stop_freq, *points)
            .with_sweep_type(match variation {
                crate::netlist::FreqVariation::Lin => StbSweepType::Linear,
                crate::netlist::FreqVariation::Dec => StbSweepType::Decade,
                crate::netlist::FreqVariation::Oct => StbSweepType::Octave,
            })
            .with_probe(probe)
            .with_nyquist(*compute_nyquist);
        config.validate()?;
        Ok(config)
    }
}

impl Default for StbConfig {
    fn default() -> Self {
        Self {
            freq_start: 1.0,  // 1 Hz
            freq_stop: 100e6, // 100 MHz
            num_points: 50,   // Points per decade
            sweep_type: StbSweepType::Decade,
            probe_node: None,
            ref_node: "0".to_string(),
            compute_nyquist: true,
        }
    }
}

impl StbConfig {
    /// Create new STB configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set frequency sweep range
    pub fn with_sweep(mut self, start: Value, stop: Value, points: usize) -> Self {
        self.freq_start = start;
        self.freq_stop = stop;
        self.num_points = points;
        self
    }

    /// Set sweep type
    pub fn with_sweep_type(mut self, sweep_type: StbSweepType) -> Self {
        self.sweep_type = sweep_type;
        self
    }

    /// Set probe node for loop break
    pub fn with_probe(mut self, node: &str) -> Self {
        self.probe_node = Some(node.to_uppercase());
        self
    }

    /// Enable/disable Nyquist computation
    pub fn with_nyquist(mut self, compute: bool) -> Self {
        self.compute_nyquist = compute;
        self
    }

    /// Generate frequency points while preserving configuration, capacity,
    /// and allocation failures.
    pub fn frequency_points(&self) -> Result<Vec<Value>, StbAnalysisError> {
        self.try_frequency_points()
    }

    /// Generate frequency points while preserving configuration and
    /// allocation failures.
    pub(crate) fn try_frequency_points(&self) -> Result<Vec<Value>, StbAnalysisError> {
        self.try_frequency_points_with_abort(&NoAbort)
    }

    /// Cancellable, fallible frequency-grid generation.
    pub(crate) fn try_frequency_points_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, StbAnalysisError> {
        ensure_not_aborted(abort)?;
        self.frequency_point_count()?;
        generate_frequency_grid(
            self.freq_start,
            self.freq_stop,
            self.num_points,
            self.grid_scale(),
            false,
            abort,
        )
        .map_err(|error| match error {
            FrequencyGridError::Aborted => StbAnalysisError::Aborted,
            other => StbAnalysisError::FrequencyGrid(other),
        })
    }

    /// Number of generated points, before allocation, using the same density
    /// and endpoint rule as the frequency vector passed to the solver.
    pub fn frequency_point_count(&self) -> Result<usize, StbConfigError> {
        self.validate()?;
        frequency_point_count(
            self.freq_start,
            self.freq_stop,
            self.num_points,
            self.grid_scale(),
        )
        .map_err(|_| StbConfigError::PointCountOverflow)
    }

    fn grid_scale(&self) -> FrequencyGridScale {
        match self.sweep_type {
            StbSweepType::Linear => FrequencyGridScale::Linear,
            StbSweepType::Decade => FrequencyGridScale::Decade,
            StbSweepType::Octave => FrequencyGridScale::Octave,
        }
    }

    /// Validate sweep configuration.
    pub fn validate(&self) -> Result<(), StbConfigError> {
        if !self.freq_start.is_finite() || self.freq_start <= 0.0 {
            return Err(StbConfigError::InvalidStartFrequency);
        }
        if !self.freq_stop.is_finite() || self.freq_stop < self.freq_start {
            return Err(StbConfigError::InvalidStopFrequency);
        }
        if self.num_points == 0 {
            return Err(StbConfigError::EmptySweep);
        }
        Ok(())
    }
}

//=============================================================================
// Stability Margins
//=============================================================================

/// One measured margin and the positive frequency of its resolved crossover.
/// Values are degrees for phase margins and dB for gain margins.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrossoverMargin {
    /// Signed margin in the unit stated by the owning field.
    pub value: Value,
    /// Measured or interpolated crossover frequency, in Hz.
    pub frequency: Value,
}

/// Margins observed in the authored sweep. Absence of a resolved crossover
/// does not establish an infinite margin or closed-loop stability.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StabilityMargins {
    /// Signed gain margin nearest zero among measured negative-real crossings.
    pub gain_margin: Option<CrossoverMargin>,
    /// Signed phase margin nearest zero among measured unity-gain crossings.
    pub phase_margin: Option<CrossoverMargin>,
    /// Independently measured zero-frequency return ratio. An engine failure
    /// to evaluate DC carries a warning; standalone AC projection leaves None.
    pub dc_loop_gain: Option<Complex64>,
    /// Number of resolved unity crossings; a connected plateau counts once.
    pub num_crossovers: usize,
}

impl StabilityMargins {
    /// DC magnitude in dB. Measured zero is negative infinity; None is unmeasured.
    pub fn dc_gain_db(&self) -> Option<Value> {
        self.dc_loop_gain.map(loop_gain_db)
    }

    /// Frequency of the unity crossing selected for the phase margin.
    pub fn unity_gain_bandwidth(&self) -> Option<Value> {
        self.phase_margin.map(|margin| margin.frequency)
    }

    /// Summary of measured margins, without a closed-loop stability claim.
    pub fn assessment(&self) -> &'static str {
        match (self.gain_margin, self.phase_margin) {
            (None, None) => "NO MARGINS MEASURED",
            (Some(gain), _) if gain.value <= 0.0 => "NONPOSITIVE MEASURED MARGIN",
            (_, Some(phase)) if phase.value <= 0.0 => "NONPOSITIVE MEASURED MARGIN",
            (Some(_), Some(_)) => "POSITIVE MEASURED MARGINS",
            _ => "PARTIAL MARGIN MEASUREMENT",
        }
    }
}

fn loop_gain_db(gain: Complex64) -> Value {
    let scale = gain.re.abs().max(gain.im.abs());
    if scale == 0.0 {
        f64::NEG_INFINITY
    } else {
        20.0 * (scale.log10() + (gain.re / scale).hypot(gain.im / scale).log10())
    }
}

//=============================================================================
// Bode Data Point
//=============================================================================

/// A single point on the Bode plot
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BodePoint {
    /// Frequency (Hz)
    pub frequency: Value,

    /// Linear magnitude; None if finite components exceed its representable range.
    pub magnitude: Option<Value>,

    /// Finite magnitude in dB; None at zero loop gain.
    pub magnitude_db: Option<Value>,

    /// Phase in degrees; None at zero loop gain. Each defined run is unwrapped.
    pub phase_deg: Option<Value>,

    /// Complex loop gain
    pub loop_gain: Complex64,
}

impl BodePoint {
    /// Create from complex loop gain
    pub fn from_loop_gain(frequency: Value, loop_gain: Complex64) -> Self {
        let magnitude = loop_gain.norm();
        let magnitude_db = loop_gain_db(loop_gain);
        let phase_deg =
            (loop_gain.re != 0.0 || loop_gain.im != 0.0).then(|| loop_gain.arg() * 180.0 / PI);

        Self {
            frequency,
            magnitude: magnitude.is_finite().then_some(magnitude),
            magnitude_db: magnitude_db.is_finite().then_some(magnitude_db),
            phase_deg,
            loop_gain,
        }
    }
}

//=============================================================================
// Nyquist Point
//=============================================================================

/// A point on the Nyquist contour
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NyquistPoint {
    /// Real part of L(jω)
    pub real: Value,

    /// Imaginary part of L(jω)
    pub imag: Value,

    /// Frequency (Hz)
    pub frequency: Value,
}

impl NyquistPoint {
    /// Create from complex loop gain
    pub fn from_loop_gain(loop_gain: Complex64, frequency: Value) -> Self {
        Self {
            real: loop_gain.re,
            imag: loop_gain.im,
            frequency,
        }
    }

    /// Distance from critical point (-1, 0)
    pub fn distance_from_critical(&self) -> Value {
        let dx = self.real + 1.0;
        let dy = self.imag;
        (dx * dx + dy * dy).sqrt()
    }
}

//=============================================================================
// STB Result
//=============================================================================

/// Result of Stability (STB) analysis
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StbResult {
    /// Complete circuit modes, distinct from the observed loop margins.
    #[serde(default, skip_serializing_if = "CircuitPoleEvidence::is_not_computed")]
    pub circuit_poles: CircuitPoleEvidence,

    /// Bode plot data points
    pub bode_points: Vec<BodePoint>,

    /// Nyquist contour points
    pub nyquist_points: Vec<NyquistPoint>,

    /// Extracted stability margins
    pub margins: StabilityMargins,

    /// Whether analysis converged/succeeded
    pub success: bool,

    /// Warning messages
    pub warnings: Vec<String>,
}

impl StbResult {
    pub fn retained_value_count(&self) -> usize {
        self.bode_points
            .len()
            .saturating_mul(6)
            .saturating_add(self.nyquist_points.len().saturating_mul(3))
            .saturating_add(6)
            .saturating_add(self.circuit_poles.retained_value_count())
    }

    /// Circuit stability requires complete qualified modes, regardless of margins.
    pub fn stability_verdict(&self) -> super::pole_zero::StabilityVerdict {
        if self.success {
            self.circuit_poles.stability_verdict()
        } else {
            super::pole_zero::StabilityVerdict::Indeterminate
        }
    }

    /// Validate retained samples and derived quantities before consuming a decoded result.
    /// Deserialization alone does not establish numerical consistency.
    pub fn validate_with_abort(
        &self,
        limits: &crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), StbAnalysisError> {
        ensure_not_aborted(abort)?;
        let invalid = |index, reason| StbAnalysisError::InvalidSample { index, reason };
        let n = self.bode_points.len();
        if !self.success || n == 0 {
            return Err(invalid(
                0,
                "retained STB result must be successful and nonempty",
            ));
        }
        if n > limits.max_analysis_points
            || self.retained_value_count() > limits.max_result_values
            || self.warnings.len() > limits.max_result_values
        {
            return Err(StbAnalysisError::CapacityOverflow {
                object: "retained STB result",
            });
        }
        self.circuit_poles.validate_with_abort(limits, abort)?;
        let mut warning_bytes = self.circuit_poles.diagnostic_bytes();
        for (index, warning) in self.warnings.iter().enumerate() {
            poll_abort(abort, index)?;
            warning_bytes = warning_bytes
                .checked_add(warning.len())
                .filter(|n| *n <= limits.max_external_data_bytes)
                .ok_or(StbAnalysisError::CapacityOverflow {
                    object: "retained STB warnings",
                })?;
        }
        if !self.nyquist_points.is_empty() && self.nyquist_points.len() != n {
            return Err(invalid(0, "Nyquist and Bode sample counts disagree"));
        }
        let close = |a: Option<f64>, b: Option<f64>, absolute_scale: f64| match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => {
                a.is_finite()
                    && b.is_finite()
                    && (a - b).abs()
                        <= 64.0 * f64::EPSILON * a.abs().max(b.abs()).max(absolute_scale)
            }
            _ => false,
        };
        let mut previous_phase = None;
        for (index, point) in self.bode_points.iter().enumerate() {
            poll_abort(abort, index)?;
            if !point.frequency.is_finite()
                || point.frequency <= 0.0
                || (index > 0 && point.frequency <= self.bode_points[index - 1].frequency)
                || !point.loop_gain.re.is_finite()
                || !point.loop_gain.im.is_finite()
            {
                return Err(invalid(index, "invalid frequency or complex loop gain"));
            }
            let mut expected = BodePoint::from_loop_gain(point.frequency, point.loop_gain);
            if let (Some(previous), Some(phase)) = (previous_phase, expected.phase_deg) {
                expected.phase_deg = Some(
                    previous
                        + super::phase::difference(previous, phase, 360.0).expect("finite phases"),
                );
            }
            previous_phase = expected.phase_deg;
            if !close(point.magnitude, expected.magnitude, 0.0)
                || !close(point.magnitude_db, expected.magnitude_db, 1.0)
                || !close(point.phase_deg, expected.phase_deg, 1.0)
            {
                return Err(invalid(
                    index,
                    "Bode quantities disagree with the complex loop gain",
                ));
            }
            if let Some(nyquist) = self.nyquist_points.get(index)
                && (nyquist.frequency != point.frequency
                    || nyquist.real != point.loop_gain.re
                    || nyquist.imag != point.loop_gain.im)
            {
                return Err(invalid(
                    index,
                    "Nyquist sample disagrees with the complex loop gain",
                ));
            }
        }
        let expected =
            StbAnalyzer::new(StbConfig::new()).extract_margins(&self.bode_points, abort)?;
        for (actual, expected) in [
            (self.margins.gain_margin, expected.gain_margin),
            (self.margins.phase_margin, expected.phase_margin),
        ] {
            if !close(actual.map(|m| m.value), expected.map(|m| m.value), 1.0)
                || !close(
                    actual.map(|m| m.frequency),
                    expected.map(|m| m.frequency),
                    0.0,
                )
            {
                return Err(invalid(
                    0,
                    "margin disagrees with the measured Bode crossings",
                ));
            }
        }
        if self.margins.num_crossovers != expected.num_crossovers
            || self
                .margins
                .dc_loop_gain
                .is_some_and(|v| !v.re.is_finite() || !v.im.is_finite())
        {
            return Err(invalid(0, "invalid crossover count or DC loop gain"));
        }
        ensure_not_aborted(abort)
    }

    /// Create new empty result
    pub fn new() -> Self {
        Self {
            circuit_poles: CircuitPoleEvidence::NotComputed,
            bode_points: Vec::new(),
            nyquist_points: Vec::new(),
            margins: StabilityMargins::default(),
            success: true,
            warnings: Vec::new(),
        }
    }

    /// Allocate the retained per-point result storage before analysis work
    /// begins. The returned vectors have zero length and exact requested
    /// capacity, so projection itself cannot trigger a user-sized growth.
    pub(crate) fn try_with_capacity(
        point_count: usize,
        compute_nyquist: bool,
    ) -> Result<Self, StbAnalysisError> {
        let mut result = Self::new();
        try_reserve_exact(&mut result.bode_points, point_count, "STB Bode result")?;
        if compute_nyquist {
            try_reserve_exact(
                &mut result.nyquist_points,
                point_count,
                "STB Nyquist result",
            )?;
        }
        // Reserve for a multiple-crossover warning and an unavailable DC solve.
        try_reserve_exact(&mut result.warnings, 2, "STB warning list")?;
        Ok(result)
    }

    /// Get magnitude vs frequency data for a Bode plot.
    pub fn magnitude_curve(&self) -> Result<Vec<(Value, Option<Value>)>, StbAnalysisError> {
        self.magnitude_curve_with_abort(&NoAbort)
    }

    /// Cancellable, fallible magnitude-curve projection.
    pub(crate) fn magnitude_curve_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, Option<Value>)>, StbAnalysisError> {
        self.project_bode_curve_with_abort(|point| point.magnitude_db, abort)
    }

    /// Get phase vs frequency data for a Bode plot.
    pub fn phase_curve(&self) -> Result<Vec<(Value, Option<Value>)>, StbAnalysisError> {
        self.phase_curve_with_abort(&NoAbort)
    }

    /// Cancellable, fallible phase-curve projection.
    pub(crate) fn phase_curve_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, Option<Value>)>, StbAnalysisError> {
        self.project_bode_curve_with_abort(|point| point.phase_deg, abort)
    }

    fn project_bode_curve_with_abort(
        &self,
        ordinate: impl Fn(&BodePoint) -> Option<Value>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, Option<Value>)>, StbAnalysisError> {
        ensure_not_aborted(abort)?;
        let mut curve = Vec::new();
        try_reserve_exact(&mut curve, self.bode_points.len(), "STB Bode curve")?;
        for (index, point) in self.bode_points.iter().enumerate() {
            poll_abort(abort, index)?;
            curve.push((point.frequency, ordinate(point)));
        }
        ensure_not_aborted(abort)?;
        Ok(curve)
    }

    /// Summarize the measured margins. This is not a stability verdict.
    pub fn margin_assessment(&self) -> &'static str {
        if self.success {
            self.margins.assessment()
        } else {
            "ANALYSIS FAILED"
        }
    }
}

impl Default for StbResult {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// STB Analyzer
//=============================================================================

/// Stability Analyzer for feedback loop analysis
#[derive(Debug)]
pub struct StbAnalyzer {
    /// Configuration
    config: StbConfig,
}

/// Failure while projecting an already-computed loop-gain sweep into Bode,
/// Nyquist, and stability-margin results.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StbAnalysisError {
    /// The authored sweep or margin configuration is invalid.
    InvalidConfiguration(StbConfigError),
    /// The requested frequencies cannot be represented as a finite grid.
    FrequencyGrid(FrequencyGridError),
    /// A sample cannot support a finite, ordered Bode sweep.
    InvalidSample {
        /// Zero-based position in the supplied sweep.
        index: usize,
        /// Invalid quantity or shape.
        reason: &'static str,
    },
    /// A checked retained shape exceeded the platform address space.
    CapacityOverflow {
        /// Result or workspace whose shape overflowed.
        object: &'static str,
    },
    /// A fallible reservation failed before projection wrote any values.
    Allocation {
        /// Result or workspace that could not be reserved.
        object: &'static str,
        /// Number of elements requested from the allocator.
        requested: usize,
        /// Original allocator refusal, including capacity overflow.
        source: std::collections::TryReserveError,
    },
    /// The caller cancelled the projection.
    Aborted,
}

impl std::fmt::Display for StbAnalysisError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConfiguration(error) => {
                write!(formatter, "invalid STB configuration: {error}")
            }
            Self::FrequencyGrid(error) => write!(formatter, "STB frequency grid: {error}"),
            Self::InvalidSample { index, reason } => {
                write!(formatter, "invalid STB sample {index}: {reason}")
            }
            Self::CapacityOverflow { object } => {
                write!(formatter, "{object} exceeds addressable capacity")
            }
            Self::Allocation {
                object,
                requested,
                source,
            } => {
                write!(
                    formatter,
                    "unable to allocate {requested} elements for {object}: {source}"
                )
            }
            Self::Aborted => formatter.write_str("STB result projection was aborted"),
        }
    }
}

impl std::error::Error for StbAnalysisError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FrequencyGrid(source) => Some(source),
            Self::InvalidConfiguration(source) => Some(source),
            Self::Allocation { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<StbConfigError> for StbAnalysisError {
    fn from(error: StbConfigError) -> Self {
        Self::InvalidConfiguration(error)
    }
}

impl StbAnalyzer {
    /// Create new STB analyzer
    pub fn new(config: StbConfig) -> Self {
        Self { config }
    }

    /// Analyze loop gain data and extract stability margins
    pub fn analyze(
        &self,
        frequencies: &[Value],
        loop_gains: &[Complex64],
    ) -> Result<StbResult, StbAnalysisError> {
        self.analyze_with_abort(frequencies, loop_gains, &NoAbort)
    }

    /// Analyze loop-gain data with cooperative cancellation during every
    /// linear scan of the potentially large sweep.
    pub fn analyze_with_abort(
        &self,
        frequencies: &[Value],
        loop_gains: &[Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<StbResult, StbAnalysisError> {
        ensure_not_aborted(abort)?;

        if frequencies.is_empty() || loop_gains.is_empty() {
            let mut result = StbResult::try_with_capacity(0, false)?;
            result.success = false;
            result
                .warnings
                .push(try_owned_message("Empty input data", "STB warning")?);
            return Ok(result);
        }

        if frequencies.len() != loop_gains.len() {
            let mut result = StbResult::try_with_capacity(0, false)?;
            result.success = false;
            result.warnings.push(try_owned_message(
                "Frequency/gain length mismatch",
                "STB warning",
            )?);
            return Ok(result);
        }

        let result = StbResult::try_with_capacity(frequencies.len(), self.config.compute_nyquist)?;
        self.analyze_preallocated_with_abort(frequencies, loop_gains, result, abort)
    }

    /// Project into storage reserved before circuit work. This is used by the
    /// engine so a result allocation failure cannot occur after an expensive
    /// operating-point and frequency solve.
    pub(crate) fn analyze_preallocated_with_abort(
        &self,
        frequencies: &[Value],
        loop_gains: &[Complex64],
        mut result: StbResult,
        abort: &dyn AbortSignal,
    ) -> Result<StbResult, StbAnalysisError> {
        ensure_not_aborted(abort)?;
        if frequencies.is_empty()
            || frequencies.len() != loop_gains.len()
            || !result.bode_points.is_empty()
            || !result.nyquist_points.is_empty()
            || !result.warnings.is_empty()
            || result.bode_points.capacity() < frequencies.len()
            || (self.config.compute_nyquist && result.nyquist_points.capacity() < frequencies.len())
        {
            return Err(StbAnalysisError::CapacityOverflow {
                object: "preallocated STB result",
            });
        }

        self.config.validate()?;
        // Unwrap one continuous path. Independently wrapping each endpoint
        // around -180 degrees invents crossings near zero phase.
        let mut previous_phase = None;
        // Build Bode points
        for (index, (&frequency, &loop_gain)) in frequencies.iter().zip(loop_gains).enumerate() {
            poll_abort(abort, index)?;
            if !frequency.is_finite()
                || frequency <= 0.0
                || (index > 0 && frequency <= frequencies[index - 1])
            {
                return Err(StbAnalysisError::InvalidSample {
                    index,
                    reason: "frequencies must be finite, positive, and strictly increasing",
                });
            }
            let mut point = BodePoint::from_loop_gain(frequency, loop_gain);
            if !loop_gain.re.is_finite() || !loop_gain.im.is_finite() {
                return Err(StbAnalysisError::InvalidSample {
                    index,
                    reason: "loop gain must have finite components",
                });
            }
            if let (Some(previous), Some(phase)) = (previous_phase, point.phase_deg) {
                point.phase_deg = Some(
                    previous
                        + super::phase::difference(previous, phase, 360.0)
                            .expect("validated finite phases"),
                );
            }
            previous_phase = point.phase_deg;
            result.bode_points.push(point);
        }

        // Build Nyquist points if configured
        if self.config.compute_nyquist {
            for (index, (&frequency, &loop_gain)) in frequencies.iter().zip(loop_gains).enumerate()
            {
                poll_abort(abort, index)?;
                result
                    .nyquist_points
                    .push(NyquistPoint::from_loop_gain(loop_gain, frequency));
            }
        }

        // Extract margins
        result.margins = self.extract_margins(&result.bode_points, abort)?;

        // Report multiple observed unity crossings without a stability claim
        if result.margins.num_crossovers > 1 {
            result
                .warnings
                .push(multiple_crossover_warning(result.margins.num_crossovers)?);
        }

        ensure_not_aborted(abort)?;
        Ok(result)
    }

    /// Extract measured margins over every resolved crossing. Frequency and
    /// ordinate are interpolated together on the same log-frequency segment.
    fn extract_margins(
        &self,
        points: &[BodePoint],
        abort: &dyn AbortSignal,
    ) -> Result<StabilityMargins, StbAnalysisError> {
        ensure_not_aborted(abort)?;
        let mut phase_margin = None;
        let mut gain_margin = None;
        let mut count = 0;
        for (index, point) in points.iter().enumerate() {
            poll_abort(abort, index)?;
            if let (Some(m1), Some(p1)) = (point.magnitude_db, point.phase_deg) {
                if m1 == 0.0 {
                    // A sampled unity plateau is one connected crossing, but
                    // its least phase margin may occur anywhere along it.
                    if index == 0 || points[index - 1].magnitude_db != Some(0.0) {
                        count += 1;
                    }
                    retain_binding_margin(
                        &mut phase_margin,
                        phase_margin_degrees(p1),
                        point.frequency,
                    );
                }
                if p1.rem_euclid(360.0) == 180.0 {
                    retain_binding_margin(&mut gain_margin, -m1, point.frequency);
                }
            }
            let Some(previous) = index.checked_sub(1).map(|i| &points[i]) else {
                continue;
            };
            let (Some(m0), Some(p0), Some(m1), Some(p1)) = (
                previous.magnitude_db,
                previous.phase_deg,
                point.magnitude_db,
                point.phase_deg,
            ) else {
                // At a measured zero, log magnitude and phase are undefined.
                // Interpolate this segment in complex gain instead: a straight
                // segment to zero has the nonzero endpoint's phase throughout.
                let (nonzero, zero_at_end) = if previous.magnitude_db.is_some() {
                    (previous, true)
                } else {
                    (point, false)
                };
                if let (Some(db), Some(phase)) = (nonzero.magnitude_db, nonzero.phase_deg)
                    && db > 0.0
                {
                    count += 1;
                    let inverse_magnitude = 10.0_f64.powf(-db / 20.0);
                    let alpha = if zero_at_end {
                        1.0 - inverse_magnitude
                    } else {
                        inverse_magnitude
                    };
                    let frequency = segment_frequency(previous.frequency, point.frequency, alpha);
                    retain_binding_margin(
                        &mut phase_margin,
                        phase_margin_degrees(phase),
                        frequency,
                    );
                    if phase.rem_euclid(360.0) == 180.0 {
                        retain_binding_margin(&mut gain_margin, 0.0, frequency);
                    }
                }
                continue;
            };
            if (m0 < 0.0 && m1 > 0.0) || (m0 > 0.0 && m1 < 0.0) {
                count += 1;
                let alpha = -m0 / (m1 - m0);
                let frequency = segment_frequency(previous.frequency, point.frequency, alpha);
                retain_binding_margin(
                    &mut phase_margin,
                    phase_margin_degrees(p0 + alpha * (p1 - p0)),
                    frequency,
                );
                // A segment lying on the negative real axis reaches zero
                // gain margin exactly where its magnitude crosses unity.
                if p0 == p1 && p0.rem_euclid(360.0) == 180.0 {
                    retain_binding_margin(&mut gain_margin, 0.0, frequency);
                }
            }
            // Unwrapped adjacent phases differ by at most half a turn, so
            // at most one odd half-turn lies strictly inside this segment.
            // Include positive turns too: the first sampled phase cannot
            // establish how many turns occurred before the sweep began.
            let target = 180.0 + 360.0 * ((p0.min(p1) - 180.0) / 360.0).ceil();
            if target > p0.min(p1) && target < p0.max(p1) {
                let alpha = (target - p0) / (p1 - p0);
                let frequency = segment_frequency(previous.frequency, point.frequency, alpha);
                retain_binding_margin(&mut gain_margin, -(m0 + alpha * (m1 - m0)), frequency);
                if m0 == 0.0 && m1 == 0.0 {
                    retain_binding_margin(&mut phase_margin, 0.0, frequency);
                }
            }
        }
        let measured = |(value, frequency)| CrossoverMargin { value, frequency };
        Ok(StabilityMargins {
            num_crossovers: count,
            gain_margin: gain_margin.map(measured),
            phase_margin: phase_margin.map(measured),
            dc_loop_gain: None,
        })
    }
}

/// Retain the signed margin nearest zero; exact ties keep the lowest
/// frequency because candidates arrive in ascending frequency order except
/// for sampled right endpoints, which are inspected before their segment.
fn retain_binding_margin(best: &mut Option<(Value, Value)>, value: Value, frequency: Value) {
    if best.is_none_or(|(old_value, old_frequency)| {
        value.abs() < old_value.abs()
            || (value.abs() == old_value.abs() && frequency < old_frequency)
    }) {
        *best = Some((value, frequency));
    }
}

/// Signed distance from the negative real axis in (-180, 180] degrees.
/// Keep the full unwrapped phase in BodePoint; folding a reported margin
/// does not establish a Nyquist winding count or closed-loop stability.
fn phase_margin_degrees(phase: Value) -> Value {
    let margin = super::phase::difference(-180.0, phase, 360.0).expect("validated finite phase");
    if margin == -180.0 { 180.0 } else { margin }
}

fn segment_frequency(start: Value, stop: Value, alpha: Value) -> Value {
    // Interpolating log values avoids overflow/underflow in stop/start.
    10.0_f64
        .powf(start.log10() + alpha * (stop.log10() - start.log10()))
        .clamp(start, stop)
}

fn try_reserve_exact<T>(
    values: &mut Vec<T>,
    requested: usize,
    object: &'static str,
) -> Result<(), StbAnalysisError> {
    values
        .try_reserve_exact(requested)
        .map_err(|source| StbAnalysisError::Allocation {
            object,
            requested,
            source,
        })
}

fn try_owned_message(message: &str, object: &'static str) -> Result<String, StbAnalysisError> {
    let mut owned = String::new();
    owned
        .try_reserve_exact(message.len())
        .map_err(|source| StbAnalysisError::Allocation {
            object,
            requested: message.len(),
            source,
        })?;
    owned.push_str(message);
    Ok(owned)
}

fn multiple_crossover_warning(count: usize) -> Result<String, StbAnalysisError> {
    const PREFIX: &str = "Multiple unity-gain crossovers detected (";
    const MAX_USIZE_DECIMAL_DIGITS: usize = usize::BITS as usize;
    let capacity = PREFIX
        .len()
        .checked_add(MAX_USIZE_DECIMAL_DIGITS)
        .and_then(|value| value.checked_add(1))
        .ok_or(StbAnalysisError::CapacityOverflow {
            object: "STB warning",
        })?;
    let mut warning = String::new();
    warning
        .try_reserve_exact(capacity)
        .map_err(|source| StbAnalysisError::Allocation {
            object: "STB warning",
            requested: capacity,
            source,
        })?;
    write!(&mut warning, "{PREFIX}{count})").map_err(|_| StbAnalysisError::CapacityOverflow {
        object: "STB warning",
    })?;
    Ok(warning)
}

const STB_ABORT_POLL_STRIDE: usize = 256;

#[inline]
fn ensure_not_aborted(abort: &dyn AbortSignal) -> Result<(), StbAnalysisError> {
    if abort.is_aborted() {
        Err(StbAnalysisError::Aborted)
    } else {
        Ok(())
    }
}

#[inline]
fn poll_abort(abort: &dyn AbortSignal, index: usize) -> Result<(), StbAnalysisError> {
    if index.is_multiple_of(STB_ABORT_POLL_STRIDE) {
        ensure_not_aborted(abort)?;
    }
    Ok(())
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::CountingAbort;

    #[test]
    fn log_sweeps_reject_invalid_frequency_grids() {
        for config in [
            StbConfig::new()
                .with_sweep(0.0, 1.0e3, 10)
                .with_sweep_type(StbSweepType::Decade),
            StbConfig::new()
                .with_sweep(f64::NAN, 1.0e3, 10)
                .with_sweep_type(StbSweepType::Decade),
            StbConfig::new()
                .with_sweep(1.0, f64::INFINITY, 10)
                .with_sweep_type(StbSweepType::Octave),
            StbConfig::new()
                .with_sweep(1.0e3, 1.0, 10)
                .with_sweep_type(StbSweepType::Decade),
            StbConfig::new()
                .with_sweep(1.0, 1.0e3, 0)
                .with_sweep_type(StbSweepType::Decade),
        ] {
            assert!(
                config.frequency_points().is_err(),
                "invalid STB sweep config did not return an error: {config:?}"
            );
        }
    }

    #[test]
    fn result_projection_observes_abort_within_one_poll_stride() {
        let count = STB_ABORT_POLL_STRIDE * 4;
        let frequencies = (0..count)
            .map(|index| 1.0 + index as Value)
            .collect::<Vec<_>>();
        let loop_gains = vec![Complex64::new(2.0, 0.0); count];
        let abort = CountingAbort::new(2);

        let error = StbAnalyzer::new(StbConfig::new())
            .analyze_with_abort(&frequencies, &loop_gains, &abort)
            .expect_err("counted cancellation must stop STB projection");

        assert_eq!(error, StbAnalysisError::Aborted);
        assert_eq!(
            abort.count(),
            3,
            "projection must stop on the first true poll"
        );
    }

    #[test]
    fn logarithmic_point_count_overflow_is_typed_and_allocation_free() {
        let config = StbConfig::new()
            .with_sweep(f64::MIN_POSITIVE, f64::MAX, usize::MAX)
            .with_sweep_type(StbSweepType::Decade);

        assert_eq!(
            config.frequency_point_count(),
            Err(StbConfigError::PointCountOverflow)
        );
        assert!(matches!(
            config.try_frequency_points(),
            Err(StbAnalysisError::InvalidConfiguration(
                StbConfigError::PointCountOverflow
            ))
        ));
        assert!(matches!(
            config.frequency_points(),
            Err(StbAnalysisError::InvalidConfiguration(
                StbConfigError::PointCountOverflow
            ))
        ));
    }

    #[test]
    fn linear_frequency_grid_reports_unallocatable_capacity() {
        let config = StbConfig::new()
            .with_sweep(1.0, 2.0, usize::MAX)
            .with_sweep_type(StbSweepType::Linear);

        assert!(matches!(
            config.try_frequency_points(),
            Err(StbAnalysisError::FrequencyGrid(
                FrequencyGridError::Allocation {
                    requested: usize::MAX,
                    ..
                }
            ))
        ));
        assert!(matches!(
            config.frequency_points(),
            Err(StbAnalysisError::FrequencyGrid(
                FrequencyGridError::Allocation {
                    requested: usize::MAX,
                    ..
                }
            ))
        ));
    }

    #[test]
    fn frequency_grid_observes_abort_within_one_poll_stride() {
        let count = STB_ABORT_POLL_STRIDE * 4;
        let config = StbConfig::new()
            .with_sweep(1.0, count as Value, count)
            .with_sweep_type(StbSweepType::Linear);
        let abort = CountingAbort::new(2);

        let error = config
            .try_frequency_points_with_abort(&abort)
            .expect_err("counted cancellation must stop frequency generation");

        assert_eq!(error, StbAnalysisError::Aborted);
        assert_eq!(abort.count(), 3);
    }

    #[test]
    fn result_projection_preallocation_reports_unallocatable_capacity() {
        use std::error::Error;
        let error = StbResult::try_with_capacity(usize::MAX, true).unwrap_err();
        let expected = Vec::<BodePoint>::new()
            .try_reserve_exact(usize::MAX)
            .unwrap_err();
        assert_eq!(
            error
                .source()
                .unwrap()
                .downcast_ref::<std::collections::TryReserveError>(),
            Some(&expected)
        );
        assert!(error.to_string().contains(&expected.to_string()));
        let StbAnalysisError::Allocation {
            object,
            requested,
            source,
        } = error
        else {
            panic!("projection allocation failure lost its cause");
        };
        assert_eq!((object, requested), ("STB Bode result", usize::MAX));
        assert_eq!(source, expected);
    }

    #[test]
    fn bode_curve_projection_is_fallible_and_cancellable() {
        let count = STB_ABORT_POLL_STRIDE * 4;
        let mut result =
            StbResult::try_with_capacity(count, false).expect("small test Bode result allocation");
        for index in 0..count {
            result.bode_points.push(BodePoint::from_loop_gain(
                index as Value + 1.0,
                Complex64::new(2.0, 0.0),
            ));
        }
        let magnitude = result
            .magnitude_curve()
            .expect("small magnitude projection");
        assert_eq!(magnitude.len(), count);
        assert_eq!(magnitude[0], (1.0, Some(20.0 * 2.0_f64.log10())));

        let abort = CountingAbort::new(2);
        assert_eq!(
            result.phase_curve_with_abort(&abort),
            Err(StbAnalysisError::Aborted)
        );
        assert_eq!(abort.count(), 3);
    }
}
