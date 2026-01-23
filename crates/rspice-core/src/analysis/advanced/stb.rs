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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StbSweepType {
    /// Linear frequency sweep
    Linear,
    /// Decade (logarithmic) sweep
    Decade,
}

impl Default for StbSweepType {
    fn default() -> Self {
        Self::Decade
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
        match self.sweep_type {
            StbSweepType::Linear => self.linear_points(),
            StbSweepType::Decade => self.decade_points(),
        }
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
        if self.freq_start <= 0.0 || self.freq_stop <= 0.0 {
            return vec![self.freq_start.max(1e-15)];
        }
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
    /// Check if margins meet minimum thresholds
    pub fn meets_thresholds(&self, min_gm_db: Value, min_pm_deg: Value) -> bool {
        self.gain_margin_db >= min_gm_db && self.phase_margin_deg >= min_pm_deg
    }

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

    /// Get Nyquist contour
    pub fn nyquist_contour(&self) -> Vec<(Value, Value)> {
        self.nyquist_points
            .iter()
            .map(|p| (p.real, p.imag))
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

impl StbAnalyzer {
    /// Create new STB analyzer
    pub fn new(config: StbConfig) -> Self {
        Self { config }
    }

    /// Analyze loop gain data and extract stability margins
    pub fn analyze(&self, frequencies: &[Value], loop_gains: &[Complex64]) -> StbResult {
        let mut result = StbResult::new();

        if frequencies.is_empty() || loop_gains.is_empty() {
            result.success = false;
            result.warnings.push("Empty input data".to_string());
            return result;
        }

        if frequencies.len() != loop_gains.len() {
            result.success = false;
            result
                .warnings
                .push("Frequency/gain length mismatch".to_string());
            return result;
        }

        // Build Bode points
        for (i, &freq) in frequencies.iter().enumerate() {
            result
                .bode_points
                .push(BodePoint::from_loop_gain(freq, loop_gains[i]));
        }

        // Build Nyquist points if configured
        if self.config.compute_nyquist {
            for (i, &freq) in frequencies.iter().enumerate() {
                result
                    .nyquist_points
                    .push(NyquistPoint::from_loop_gain(loop_gains[i], freq));
            }
        }

        // Extract margins
        result.margins = self.extract_margins(&result.bode_points);

        // Check for conditional stability
        if result.margins.num_crossovers > 1 {
            result.warnings.push(format!(
                "Multiple unity-gain crossovers detected ({})",
                result.margins.num_crossovers
            ));
        }

        result
    }

    /// Extract stability margins from Bode data
    fn extract_margins(&self, points: &[BodePoint]) -> StabilityMargins {
        let mut margins = StabilityMargins::default();

        if points.is_empty() {
            return margins;
        }

        // DC gain (lowest frequency point)
        margins.dc_gain_db = points[0].magnitude_db;

        // Find unity gain crossover(s) - where magnitude crosses 0 dB
        let crossovers = self.find_zero_crossings(points, |p| p.magnitude_db);
        margins.num_crossovers = crossovers.len();
        margins.conditionally_stable = crossovers.len() > 1;

        // Phase margin: phase at first unity gain crossover
        if let Some(&(freq, _)) = crossovers.first() {
            margins.phase_margin_freq = freq;
            margins.unity_gain_bandwidth = freq;

            // Interpolate phase at crossover
            let phase = self.interpolate_at_frequency(points, freq, |p| p.phase_deg);
            margins.phase_margin_deg = 180.0 + phase; // PM = 180° + phase
        } else {
            // No crossover found - either always > 0dB or always < 0dB
            if points.iter().all(|p| p.magnitude_db > 0.0) {
                margins.phase_margin_deg = f64::NEG_INFINITY;
                margins.gain_margin_db = f64::NEG_INFINITY;
                return margins;
            } else {
                // High gain margin (loop never reaches 0 dB)
                margins.phase_margin_deg = f64::INFINITY;
                margins.gain_margin_db = -points
                    .iter()
                    .map(|p| p.magnitude_db)
                    .fold(f64::NEG_INFINITY, f64::max);
            }
        }

        // Find phase crossover(s) - where phase crosses -180°
        let phase_crossings = self.find_phase_crossings(points, -180.0);

        // Gain margin: magnitude at first phase crossover
        if let Some(&(freq, _)) = phase_crossings.first() {
            margins.gain_margin_freq = freq;

            // Interpolate magnitude at phase crossover
            let mag_db = self.interpolate_at_frequency(points, freq, |p| p.magnitude_db);
            margins.gain_margin_db = -mag_db; // GM = -|L| at -180°
        } else {
            // Phase never crosses -180° - infinite gain margin
            margins.gain_margin_db = f64::INFINITY;
        }

        margins
    }

    /// Find zero crossings in a curve
    fn find_zero_crossings<F>(&self, points: &[BodePoint], extractor: F) -> Vec<(Value, Value)>
    where
        F: Fn(&BodePoint) -> Value,
    {
        let mut crossings = Vec::new();

        for window in points.windows(2) {
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

        crossings
    }

    /// Find phase crossings at specific phase value
    fn find_phase_crossings(
        &self,
        points: &[BodePoint],
        target_phase: Value,
    ) -> Vec<(Value, Value)> {
        let mut crossings = Vec::new();

        for window in points.windows(2) {
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

        crossings
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
    fn interpolate_at_frequency<F>(&self, points: &[BodePoint], freq: Value, extractor: F) -> Value
    where
        F: Fn(&BodePoint) -> Value,
    {
        // Find bracketing points
        for window in points.windows(2) {
            if window[0].frequency <= freq && window[1].frequency >= freq {
                let f0 = window[0].frequency.log10();
                let f1 = window[1].frequency.log10();
                let v0 = extractor(&window[0]);
                let v1 = extractor(&window[1]);

                let alpha = (freq.log10() - f0) / (f1 - f0);
                return v0 + alpha * (v1 - v0);
            }
        }

        // Extrapolate from nearest
        if freq < points[0].frequency {
            extractor(&points[0])
        } else {
            extractor(points.last().unwrap())
        }
    }

    /// Create a test loop gain (single-pole system with gain)
    pub fn create_test_loop_gain(&self, dc_gain_db: Value, pole_freq: Value) -> StbResult {
        let frequencies = self.config.frequency_points();
        let dc_gain = 10.0_f64.powf(dc_gain_db / 20.0);

        let loop_gains: Vec<Complex64> = frequencies
            .iter()
            .map(|&f| {
                let s = Complex64::new(0.0, 2.0 * PI * f);
                let pole = 2.0 * PI * pole_freq;
                Complex64::new(dc_gain, 0.0) / (1.0 + s / pole)
            })
            .collect();

        self.analyze(&frequencies, &loop_gains)
    }

    /// Create a two-pole system (typical op-amp feedback)
    pub fn create_two_pole_loop_gain(
        &self,
        dc_gain_db: Value,
        pole1_freq: Value,
        pole2_freq: Value,
    ) -> StbResult {
        let frequencies = self.config.frequency_points();
        let dc_gain = 10.0_f64.powf(dc_gain_db / 20.0);

        let loop_gains: Vec<Complex64> = frequencies
            .iter()
            .map(|&f| {
                let s = Complex64::new(0.0, 2.0 * PI * f);
                let p1 = 2.0 * PI * pole1_freq;
                let p2 = 2.0 * PI * pole2_freq;
                Complex64::new(dc_gain, 0.0) / ((1.0 + s / p1) * (1.0 + s / p2))
            })
            .collect();

        self.analyze(&frequencies, &loop_gains)
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Configuration Tests
    // =========================================================================

    #[test]
    fn test_stb_config_default() {
        let config = StbConfig::default();
        assert!(config.freq_start > 0.0);
        assert!(config.freq_stop > config.freq_start);
        assert!(config.num_points > 0);
        assert_eq!(config.min_phase_margin_deg, 45.0);
        assert_eq!(config.min_gain_margin_db, 10.0);
    }

    #[test]
    fn test_stb_config_builder() {
        let config = StbConfig::new()
            .with_sweep(10.0, 1e6, 100)
            .with_sweep_type(StbSweepType::Linear)
            .with_probe("VOUT")
            .with_thresholds(15.0, 60.0)
            .with_nyquist(false);

        assert_eq!(config.freq_start, 10.0);
        assert_eq!(config.freq_stop, 1e6);
        assert_eq!(config.num_points, 100);
        assert_eq!(config.sweep_type, StbSweepType::Linear);
        assert_eq!(config.probe_node, Some("VOUT".to_string()));
        assert_eq!(config.min_gain_margin_db, 15.0);
        assert_eq!(config.min_phase_margin_deg, 60.0);
        assert!(!config.compute_nyquist);
    }

    #[test]
    fn test_frequency_points_linear() {
        let config = StbConfig::new()
            .with_sweep(100.0, 200.0, 11)
            .with_sweep_type(StbSweepType::Linear);

        let points = config.frequency_points();
        assert_eq!(points.len(), 11);
        assert!((points[0] - 100.0).abs() < 1e-10);
        assert!((points[10] - 200.0).abs() < 1e-10);
    }

    #[test]
    fn test_frequency_points_decade() {
        let config = StbConfig::new()
            .with_sweep(1.0, 1e6, 10)
            .with_sweep_type(StbSweepType::Decade);

        let points = config.frequency_points();
        assert!(points.len() >= 50); // 6 decades * ~10 pts

        // Check log spacing
        for i in 1..points.len() {
            assert!(points[i] > points[i - 1]);
        }
    }

    // =========================================================================
    // Bode Point Tests
    // =========================================================================

    #[test]
    fn test_bode_point_from_loop_gain() {
        let loop_gain = Complex64::new(10.0, 0.0); // 20 dB, 0°
        let point = BodePoint::from_loop_gain(1e3, loop_gain);

        assert!((point.magnitude - 10.0).abs() < 1e-10);
        assert!((point.magnitude_db - 20.0).abs() < 0.01);
        assert!(point.phase_deg.abs() < 0.01);
    }

    #[test]
    fn test_bode_point_negative_phase() {
        // -90° phase (pure imaginary negative)
        let loop_gain = Complex64::new(0.0, -10.0);
        let point = BodePoint::from_loop_gain(1e3, loop_gain);

        assert!((point.phase_deg - (-90.0)).abs() < 0.01);
    }

    // =========================================================================
    // Nyquist Point Tests
    // =========================================================================

    #[test]
    fn test_nyquist_point_creation() {
        let lg = Complex64::new(-0.5, 0.5);
        let np = NyquistPoint::from_loop_gain(lg, 1e3);

        assert!((np.real - (-0.5)).abs() < 1e-10);
        assert!((np.imag - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_nyquist_distance_from_critical() {
        // At critical point
        let np1 = NyquistPoint {
            real: -1.0,
            imag: 0.0,
            frequency: 1e3,
        };
        assert!(np1.distance_from_critical() < 1e-10);

        // Away from critical point
        let np2 = NyquistPoint {
            real: 0.0,
            imag: 0.0,
            frequency: 1e3,
        };
        assert!((np2.distance_from_critical() - 1.0).abs() < 1e-10);
    }

    // =========================================================================
    // Stability Margins Tests
    // =========================================================================

    #[test]
    fn test_stability_margins_stable() {
        let margins = StabilityMargins {
            gain_margin_db: 12.0,
            phase_margin_deg: 60.0,
            ..Default::default()
        };

        assert!(margins.is_stable());
        assert!(margins.meets_thresholds(10.0, 45.0));
        assert!(!margins.meets_thresholds(15.0, 45.0));
    }

    #[test]
    fn test_stability_margins_unstable() {
        let margins = StabilityMargins {
            gain_margin_db: -5.0,
            phase_margin_deg: -10.0,
            ..Default::default()
        };

        assert!(!margins.is_stable());
        assert_eq!(margins.assessment(), "UNSTABLE");
    }

    #[test]
    fn test_stability_margins_marginal() {
        let margins = StabilityMargins {
            gain_margin_db: 6.0,
            phase_margin_deg: 25.0,
            ..Default::default()
        };

        assert!(margins.is_stable());
        assert_eq!(margins.assessment(), "MARGINALLY STABLE");
    }

    #[test]
    fn test_stability_margins_well_damped() {
        let margins = StabilityMargins {
            gain_margin_db: 15.0,
            phase_margin_deg: 65.0,
            ..Default::default()
        };

        assert_eq!(margins.assessment(), "WELL DAMPED");
    }

    // =========================================================================
    // Analyzer Tests
    // =========================================================================

    #[test]
    fn test_analyzer_empty_input() {
        let config = StbConfig::default();
        let analyzer = StbAnalyzer::new(config);

        let result = analyzer.analyze(&[], &[]);
        assert!(!result.success);
    }

    #[test]
    fn test_analyzer_single_pole_stable() {
        let config = StbConfig::new().with_sweep(1.0, 1e6, 20);
        let analyzer = StbAnalyzer::new(config);

        // Single pole system: 40dB gain, 10kHz pole
        // UGB = 10kHz * 100 = 1MHz, PM = 90° (single pole)
        let result = analyzer.create_test_loop_gain(40.0, 10e3);

        assert!(result.success);
        assert!(result.margins.is_stable());
        assert!(result.margins.phase_margin_deg > 80.0); // Single pole → ~90° PM
    }

    #[test]
    fn test_analyzer_two_pole_stable() {
        let config = StbConfig::new().with_sweep(1.0, 1e8, 30);
        let analyzer = StbAnalyzer::new(config);

        // Two poles well separated: PM should be good
        let result = analyzer.create_two_pole_loop_gain(60.0, 100.0, 1e6);

        assert!(result.success);
        assert!(result.margins.is_stable());
    }

    #[test]
    fn test_analyzer_two_pole_marginal() {
        let config = StbConfig::new().with_sweep(1.0, 1e8, 50);
        let analyzer = StbAnalyzer::new(config);

        // Two poles close together: reduced PM
        let result = analyzer.create_two_pole_loop_gain(40.0, 10e3, 30e3);

        assert!(result.success);
        // PM should be less than 90°
        assert!(result.margins.phase_margin_deg < 90.0);
    }

    #[test]
    fn test_analyzer_bode_curves() {
        let config = StbConfig::new().with_sweep(100.0, 1e6, 20);
        let analyzer = StbAnalyzer::new(config);

        let result = analyzer.create_test_loop_gain(40.0, 1e3);

        let mag_curve = result.magnitude_curve();
        let phase_curve = result.phase_curve();

        assert!(!mag_curve.is_empty());
        assert_eq!(mag_curve.len(), phase_curve.len());

        // Magnitude should decrease with frequency (after pole)
        // Phase should become more negative
    }

    #[test]
    fn test_analyzer_nyquist_contour() {
        let config = StbConfig::new().with_sweep(1.0, 1e6, 20).with_nyquist(true);
        let analyzer = StbAnalyzer::new(config);

        let result = analyzer.create_test_loop_gain(20.0, 1e3);

        assert!(!result.nyquist_contour().is_empty());

        // Nyquist should not encircle -1 for stable system
        let min_dist: Value = result
            .nyquist_points
            .iter()
            .map(|p| p.distance_from_critical())
            .fold(f64::INFINITY, f64::min);

        assert!(min_dist > 0.0); // Doesn't pass through -1
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_analyzer_high_gain() {
        let config = StbConfig::new().with_sweep(0.1, 1e6, 20);
        let analyzer = StbAnalyzer::new(config);

        // Very high DC gain
        let result = analyzer.create_test_loop_gain(100.0, 1.0);

        assert!(result.success);
        assert!(result.margins.dc_gain_db > 90.0);
    }

    #[test]
    fn test_analyzer_low_bandwidth() {
        let config = StbConfig::new().with_sweep(0.01, 1e3, 20);
        let analyzer = StbAnalyzer::new(config);

        // Very low pole frequency
        let result = analyzer.create_test_loop_gain(40.0, 0.1);

        assert!(result.success);
    }

    #[test]
    fn test_sweep_type_default() {
        assert_eq!(StbSweepType::default(), StbSweepType::Decade);
    }

    #[test]
    fn test_result_assessment() {
        let config = StbConfig::new().with_sweep(1.0, 1e6, 30);
        let analyzer = StbAnalyzer::new(config);

        let result = analyzer.create_test_loop_gain(40.0, 10e3);

        let assessment = result.assessment();
        assert!(!assessment.is_empty());
    }
}
