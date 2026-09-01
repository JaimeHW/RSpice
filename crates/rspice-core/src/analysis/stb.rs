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
//! For stability (Bode criterion):
//! - Phase margin > 0° (typically > 45° for good damping)
//! - Gain margin > 0 dB (typically > 10 dB for robustness)

use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use num_complex::Complex64;
use std::f64::consts::PI;

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

    /// Gain margin minimum threshold (dB)
    pub min_gain_margin_db: Value,

    /// Phase margin minimum threshold (degrees)
    pub min_phase_margin_deg: Value,

    /// Whether to compute Nyquist data
    pub compute_nyquist: bool,

    /// Maximum loop gain to consider for crossover detection (dB)
    pub max_loop_gain_db: Value,
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

impl Default for StbConfig {
    fn default() -> Self {
        Self {
            freq_start: 1.0,  // 1 Hz
            freq_stop: 100e6, // 100 MHz
            num_points: 50,   // Points per decade
            sweep_type: StbSweepType::Decade,
            probe_node: None,
            ref_node: "0".to_string(),
            min_gain_margin_db: 10.0,   // 10 dB minimum
            min_phase_margin_deg: 45.0, // 45° minimum
            compute_nyquist: true,
            max_loop_gain_db: 200.0,
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

    /// Set stability thresholds
    pub fn with_thresholds(mut self, gain_margin_db: Value, phase_margin_deg: Value) -> Self {
        self.min_gain_margin_db = gain_margin_db;
        self.min_phase_margin_deg = phase_margin_deg;
        self
    }

    /// Enable/disable Nyquist computation
    pub fn with_nyquist(mut self, compute: bool) -> Self {
        self.compute_nyquist = compute;
        self
    }

    /// Generate frequency points
    pub fn frequency_points(&self) -> Vec<Value> {
        if self.validate().is_err() {
            return Vec::new();
        }

        match self.sweep_type {
            StbSweepType::Linear => self.linear_points(),
            StbSweepType::Decade => self.decade_points(),
            StbSweepType::Octave => self.octave_points(),
        }
    }

    /// Number of sweep points that will be generated, without allocating the
    /// frequency vector.
    pub fn frequency_point_count(&self) -> Result<usize, String> {
        self.validate()?;
        let count = match self.sweep_type {
            StbSweepType::Linear => self.num_points,
            StbSweepType::Decade => ((self.freq_stop.log10() - self.freq_start.log10())
                * self.num_points as Value)
                .ceil() as usize,
            StbSweepType::Octave => ((self.freq_stop.log2() - self.freq_start.log2())
                * self.num_points as Value)
                .ceil() as usize,
        };
        Ok(count.max(1))
    }

    fn linear_points(&self) -> Vec<Value> {
        if self.num_points <= 1 {
            return vec![self.freq_start];
        }
        let step = (self.freq_stop - self.freq_start) / (self.num_points - 1) as Value;
        (0..self.num_points)
            .map(|i| self.freq_start + i as Value * step)
            .collect()
    }

    fn decade_points(&self) -> Vec<Value> {
        let log_start = self.freq_start.log10();
        let log_stop = self.freq_stop.log10();
        let num_decades = log_stop - log_start;
        let total_points = (num_decades * self.num_points as f64).ceil() as usize;
        let total_points = total_points.max(1);

        (0..total_points)
            .map(|i| {
                let log_f = log_start
                    + (log_stop - log_start) * i as f64 / (total_points - 1).max(1) as f64;
                10.0_f64.powf(log_f)
            })
            .collect()
    }

    fn octave_points(&self) -> Vec<Value> {
        let log_start = self.freq_start.log2();
        let log_stop = self.freq_stop.log2();
        let num_octaves = log_stop - log_start;
        let total_points = (num_octaves * self.num_points as f64).ceil() as usize;
        let total_points = total_points.max(1);

        (0..total_points)
            .map(|i| {
                let log_f = log_start
                    + (log_stop - log_start) * i as f64 / (total_points - 1).max(1) as f64;
                2.0_f64.powf(log_f)
            })
            .collect()
    }

    /// Validate sweep configuration.
    pub fn validate(&self) -> Result<(), String> {
        if !self.freq_start.is_finite() || self.freq_start <= 0.0 {
            return Err("STB start frequency must be positive and finite".to_string());
        }
        if !self.freq_stop.is_finite() || self.freq_stop < self.freq_start {
            return Err("STB stop frequency must be finite and >= start".to_string());
        }
        if self.num_points == 0 {
            return Err("STB sweep must have at least one point".to_string());
        }
        if !self.min_gain_margin_db.is_finite()
            || !self.min_phase_margin_deg.is_finite()
            || !self.max_loop_gain_db.is_finite()
        {
            return Err("STB margin thresholds must be finite".to_string());
        }
        Ok(())
    }
}

//=============================================================================
// Stability Margins
//=============================================================================

/// Stability margins extracted from loop gain
#[derive(Debug, Clone, Default)]
pub struct StabilityMargins {
    /// Gain margin in dB (positive = stable)
    pub gain_margin_db: Value,

    /// Frequency at which phase = -180° (gain margin frequency)
    pub gain_margin_freq: Value,

    /// Phase margin in degrees (positive = stable)
    pub phase_margin_deg: Value,

    /// Frequency at which |L| = 0 dB (unity gain crossover)
    pub phase_margin_freq: Value,

    /// DC loop gain in dB
    pub dc_gain_db: Value,

    /// Unity gain bandwidth (Hz)
    pub unity_gain_bandwidth: Value,

    /// Whether the loop is conditionally stable (multiple crossovers)
    pub conditionally_stable: bool,

    /// Number of unity gain crossovers
    pub num_crossovers: usize,
}

impl StabilityMargins {
    /// Check if the system is stable (positive margins)
    pub fn is_stable(&self) -> bool {
        self.gain_margin_db > 0.0 && self.phase_margin_deg > 0.0
    }

    /// Get stability assessment string
    pub fn assessment(&self) -> String {
        if !self.is_stable() {
            "UNSTABLE".to_string()
        } else if self.conditionally_stable {
            "CONDITIONALLY STABLE".to_string()
        } else if self.phase_margin_deg < 30.0 {
            "MARGINALLY STABLE".to_string()
        } else if self.phase_margin_deg >= 60.0 && self.gain_margin_db >= 12.0 {
            "WELL DAMPED".to_string()
        } else {
            "STABLE".to_string()
        }
    }
}

//=============================================================================
// Bode Data Point
//=============================================================================

/// A single point on the Bode plot
#[derive(Debug, Clone)]
pub struct BodePoint {
    /// Frequency (Hz)
    pub frequency: Value,

    /// Loop gain magnitude (linear)
    pub magnitude: Value,

    /// Loop gain magnitude (dB)
    pub magnitude_db: Value,

    /// Phase (degrees)
    pub phase_deg: Value,

    /// Complex loop gain
    pub loop_gain: Complex64,
}

impl BodePoint {
    /// Create from complex loop gain
    pub fn from_loop_gain(frequency: Value, loop_gain: Complex64) -> Self {
        let magnitude = loop_gain.norm();
        let magnitude_db = 20.0 * magnitude.log10();
        let phase_deg = loop_gain.arg() * 180.0 / PI;

        Self {
            frequency,
            magnitude,
            magnitude_db,
            phase_deg,
            loop_gain,
        }
    }
}

//=============================================================================
// Nyquist Point
//=============================================================================

/// A point on the Nyquist contour
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct StbResult {
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
    /// Create new empty result
    pub fn new() -> Self {
        Self {
            bode_points: Vec::new(),
            nyquist_points: Vec::new(),
            margins: StabilityMargins::default(),
            success: true,
            warnings: Vec::new(),
        }
    }

    /// Get magnitude vs frequency data for Bode plot
    pub fn magnitude_curve(&self) -> Vec<(Value, Value)> {
        self.bode_points
            .iter()
            .map(|p| (p.frequency, p.magnitude_db))
            .collect()
    }

    /// Get phase vs frequency data for Bode plot
    pub fn phase_curve(&self) -> Vec<(Value, Value)> {
        self.bode_points
            .iter()
            .map(|p| (p.frequency, p.phase_deg))
            .collect()
    }

    /// Check if stable
    pub fn is_stable(&self) -> bool {
        self.margins.is_stable()
    }

    /// Get stability assessment
    pub fn assessment(&self) -> String {
        self.margins.assessment()
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum StbAnalysisError {
    /// The caller cancelled the projection.
    Aborted,
}

impl std::fmt::Display for StbAnalysisError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Aborted => formatter.write_str("STB result projection was aborted"),
        }
    }
}

impl std::error::Error for StbAnalysisError {}

impl StbAnalyzer {
    /// Create new STB analyzer
    pub fn new(config: StbConfig) -> Self {
        Self { config }
    }

    /// Analyze loop gain data and extract stability margins
    pub fn analyze(&self, frequencies: &[Value], loop_gains: &[Complex64]) -> StbResult {
        self.analyze_with_abort(frequencies, loop_gains, &NoAbort)
            .expect("NoAbort cannot cancel STB result projection")
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
        let mut result = StbResult::new();

        if frequencies.is_empty() || loop_gains.is_empty() {
            result.success = false;
            result.warnings.push("Empty input data".to_string());
            return Ok(result);
        }

        if frequencies.len() != loop_gains.len() {
            result.success = false;
            result
                .warnings
                .push("Frequency/gain length mismatch".to_string());
            return Ok(result);
        }

        // Build Bode points
        for (i, &freq) in frequencies.iter().enumerate() {
            poll_abort(abort, i)?;
            result
                .bode_points
                .push(BodePoint::from_loop_gain(freq, loop_gains[i]));
        }

        // Build Nyquist points if configured
        if self.config.compute_nyquist {
            for (i, &freq) in frequencies.iter().enumerate() {
                poll_abort(abort, i)?;
                result
                    .nyquist_points
                    .push(NyquistPoint::from_loop_gain(loop_gains[i], freq));
            }
        }

        // Extract margins
        result.margins = self.extract_margins(&result.bode_points, abort)?;

        // Check for conditional stability
        if result.margins.num_crossovers > 1 {
            result.warnings.push(format!(
                "Multiple unity-gain crossovers detected ({})",
                result.margins.num_crossovers
            ));
        }

        ensure_not_aborted(abort)?;
        Ok(result)
    }

    /// Extract stability margins from Bode data
    fn extract_margins(
        &self,
        points: &[BodePoint],
        abort: &dyn AbortSignal,
    ) -> Result<StabilityMargins, StbAnalysisError> {
        ensure_not_aborted(abort)?;
        let mut margins = StabilityMargins::default();

        if points.is_empty() {
            return Ok(margins);
        }

        // DC gain (lowest frequency point)
        margins.dc_gain_db = points[0].magnitude_db;

        // Find unity gain crossover(s) - where magnitude crosses 0 dB
        let crossovers = self.find_zero_crossings(points, |p| p.magnitude_db, abort)?;
        margins.num_crossovers = crossovers.len();
        margins.conditionally_stable = crossovers.len() > 1;

        // Phase margin: phase at first unity gain crossover
        if let Some(&(freq, _)) = crossovers.first() {
            margins.phase_margin_freq = freq;
            margins.unity_gain_bandwidth = freq;

            // Interpolate phase at crossover
            let phase = self.interpolate_at_frequency(points, freq, |p| p.phase_deg, abort)?;
            margins.phase_margin_deg = 180.0 + phase; // PM = 180° + phase
        } else {
            // No crossover found - either always > 0dB or always < 0dB
            let mut all_above_unity = true;
            let mut maximum = f64::NEG_INFINITY;
            for (index, point) in points.iter().enumerate() {
                poll_abort(abort, index)?;
                all_above_unity &= point.magnitude_db > 0.0;
                maximum = maximum.max(point.magnitude_db);
            }
            if all_above_unity {
                margins.phase_margin_deg = f64::NEG_INFINITY;
                margins.gain_margin_db = f64::NEG_INFINITY;
                return Ok(margins);
            } else {
                // High gain margin (loop never reaches 0 dB)
                margins.phase_margin_deg = f64::INFINITY;
                margins.gain_margin_db = -maximum;
            }
        }

        // Find phase crossover(s) - where phase crosses -180°
        let phase_crossings = self.find_phase_crossings(points, -180.0, abort)?;

        // Gain margin: magnitude at first phase crossover
        if let Some(&(freq, _)) = phase_crossings.first() {
            margins.gain_margin_freq = freq;

            // Interpolate magnitude at phase crossover
            let mag_db = self.interpolate_at_frequency(points, freq, |p| p.magnitude_db, abort)?;
            margins.gain_margin_db = -mag_db; // GM = -|L| at -180°
        } else {
            // Phase never crosses -180° - infinite gain margin
            margins.gain_margin_db = f64::INFINITY;
        }

        Ok(margins)
    }

    /// Find zero crossings in a curve
    fn find_zero_crossings<F>(
        &self,
        points: &[BodePoint],
        extractor: F,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, Value)>, StbAnalysisError>
    where
        F: Fn(&BodePoint) -> Value,
    {
        let mut crossings = Vec::new();

        for (index, window) in points.windows(2).enumerate() {
            poll_abort(abort, index)?;
            let v0 = extractor(&window[0]);
            let v1 = extractor(&window[1]);

            // Check for sign change
            if (v0 > 0.0 && v1 <= 0.0) || (v0 <= 0.0 && v1 > 0.0) {
                // Linear interpolation for crossing frequency
                let f0 = window[0].frequency;
                let f1 = window[1].frequency;

                // Use log interpolation for frequency
                let log_f0 = f0.log10();
                let log_f1 = f1.log10();
                let alpha = (0.0 - v0) / (v1 - v0);
                let log_f_cross = log_f0 + alpha * (log_f1 - log_f0);
                let f_cross = 10.0_f64.powf(log_f_cross);

                crossings.push((f_cross, 0.0));
            }
        }

        Ok(crossings)
    }

    /// Find phase crossings at specific phase value
    fn find_phase_crossings(
        &self,
        points: &[BodePoint],
        target_phase: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(Value, Value)>, StbAnalysisError> {
        let mut crossings = Vec::new();

        for (index, window) in points.windows(2).enumerate() {
            poll_abort(abort, index)?;
            let p0 = window[0].phase_deg;
            let p1 = window[1].phase_deg;

            // Unwrap phase for proper detection
            let p0_unwrap = self.unwrap_phase(p0, target_phase);
            let p1_unwrap = self.unwrap_phase(p1, target_phase);

            // Check for crossing
            if (p0_unwrap > target_phase && p1_unwrap <= target_phase)
                || (p0_unwrap <= target_phase && p1_unwrap > target_phase)
            {
                let f0 = window[0].frequency;
                let f1 = window[1].frequency;

                let log_f0 = f0.log10();
                let log_f1 = f1.log10();
                let alpha = (target_phase - p0_unwrap) / (p1_unwrap - p0_unwrap);
                let log_f_cross = log_f0 + alpha * (log_f1 - log_f0);
                let f_cross = 10.0_f64.powf(log_f_cross);

                crossings.push((f_cross, target_phase));
            }
        }

        Ok(crossings)
    }

    /// Unwrap phase for proper crossing detection
    fn unwrap_phase(&self, phase: Value, target: Value) -> Value {
        let mut p = phase;
        while p - target > 180.0 {
            p -= 360.0;
        }
        while p - target < -180.0 {
            p += 360.0;
        }
        p
    }

    /// Interpolate value at specific frequency
    fn interpolate_at_frequency<F>(
        &self,
        points: &[BodePoint],
        freq: Value,
        extractor: F,
        abort: &dyn AbortSignal,
    ) -> Result<Value, StbAnalysisError>
    where
        F: Fn(&BodePoint) -> Value,
    {
        // Find bracketing points
        for (index, window) in points.windows(2).enumerate() {
            poll_abort(abort, index)?;
            if window[0].frequency <= freq && window[1].frequency >= freq {
                let f0 = window[0].frequency.log10();
                let f1 = window[1].frequency.log10();
                let v0 = extractor(&window[0]);
                let v1 = extractor(&window[1]);

                let alpha = (freq.log10() - f0) / (f1 - f0);
                return Ok(v0 + alpha * (v1 - v0));
            }
        }

        // Extrapolate from nearest
        if freq < points[0].frequency {
            Ok(extractor(&points[0]))
        } else {
            Ok(extractor(points.last().expect("non-empty STB points")))
        }
    }
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
                config.frequency_points().is_empty(),
                "invalid STB sweep config produced points: {config:?}"
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
}
