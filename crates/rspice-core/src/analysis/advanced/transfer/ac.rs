use super::*;

//=============================================================================
// AC Transfer Function (XF Analysis)
//=============================================================================

use num_complex::Complex64;
use std::f64::consts::PI;

/// Single frequency point in AC transfer function
#[derive(Debug, Clone)]
pub struct AcTransferPoint {
    /// Frequency (Hz)
    pub frequency: Value,

    /// Complex transfer function H(jω)
    pub transfer: Complex64,

    /// Magnitude (linear)
    pub magnitude: Value,

    /// Magnitude in dB
    pub magnitude_db: Value,

    /// Phase in radians
    pub phase_rad: Value,

    /// Phase in degrees
    pub phase_deg: Value,
}

impl AcTransferPoint {
    /// Create from frequency and complex transfer function
    pub fn new(frequency: Value, transfer: Complex64) -> Self {
        let magnitude = transfer.norm();
        Self {
            frequency,
            transfer,
            magnitude,
            magnitude_db: 20.0 * magnitude.log10(),
            phase_rad: transfer.arg(),
            phase_deg: transfer.arg() * 180.0 / PI,
        }
    }

    /// Group delay contribution between this point and next
    pub fn group_delay(&self, next: &AcTransferPoint) -> Value {
        let df = next.frequency - self.frequency;
        if df.abs() < 1e-15 {
            return 0.0;
        }

        let mut dphi = next.phase_rad - self.phase_rad;
        // Unwrap phase
        while dphi > PI {
            dphi -= 2.0 * PI;
        }
        while dphi < -PI {
            dphi += 2.0 * PI;
        }

        -dphi / (2.0 * PI * df)
    }
}

/// AC Transfer Function analysis result
#[derive(Debug, Clone)]
pub struct AcTransferResult {
    /// Output node/variable name
    pub output: String,

    /// Input source name
    pub input: String,

    /// Transfer function at each frequency
    pub points: Vec<AcTransferPoint>,

    /// DC gain (if available)
    pub dc_gain: Option<Value>,

    /// DC gain in dB
    pub dc_gain_db: Option<Value>,

    /// Peak gain frequency (Hz)
    pub peak_frequency: Option<Value>,

    /// Peak gain (dB)
    pub peak_gain_db: Option<Value>,

    /// -3dB cutoff frequency (low) for bandpass
    pub cutoff_low: Option<Value>,

    /// -3dB cutoff frequency (high)
    pub cutoff_high: Option<Value>,

    /// Bandwidth (Hz)
    pub bandwidth: Option<Value>,

    /// Quality factor Q (for bandpass/resonant)
    pub q_factor: Option<Value>,

    /// Unity gain frequency (Hz)
    pub unity_gain_frequency: Option<Value>,

    /// Phase margin at unity gain (degrees)
    pub phase_margin: Option<Value>,
}

impl Default for AcTransferResult {
    fn default() -> Self {
        Self::new("", "")
    }
}

impl AcTransferResult {
    /// Create new result
    pub fn new(output: &str, input: &str) -> Self {
        Self {
            output: output.to_string(),
            input: input.to_string(),
            points: Vec::new(),
            dc_gain: None,
            dc_gain_db: None,
            peak_frequency: None,
            peak_gain_db: None,
            cutoff_low: None,
            cutoff_high: None,
            bandwidth: None,
            q_factor: None,
            unity_gain_frequency: None,
            phase_margin: None,
        }
    }

    /// Add a frequency point
    pub fn add_point(&mut self, point: AcTransferPoint) {
        self.points.push(point);
    }

    /// Get frequency vector
    pub fn frequencies(&self) -> Vec<Value> {
        self.points.iter().map(|p| p.frequency).collect()
    }

    /// Get magnitude curve (freq, dB)
    pub fn magnitude_curve(&self) -> Vec<(Value, Value)> {
        self.points
            .iter()
            .map(|p| (p.frequency, p.magnitude_db))
            .collect()
    }

    /// Get phase curve (freq, degrees)
    pub fn phase_curve(&self) -> Vec<(Value, Value)> {
        self.points
            .iter()
            .map(|p| (p.frequency, p.phase_deg))
            .collect()
    }

    /// Get group delay curve
    pub fn group_delay_curve(&self) -> Vec<(Value, Value)> {
        if self.points.len() < 2 {
            return Vec::new();
        }

        self.points
            .windows(2)
            .map(|w| {
                let gd = w[0].group_delay(&w[1]);
                ((w[0].frequency + w[1].frequency) / 2.0, gd)
            })
            .collect()
    }

    /// Compute filter characteristics from data
    pub fn compute_characteristics(&mut self) {
        if self.points.is_empty() {
            return;
        }

        // Find peak
        let Some((peak_idx, peak_db)) = self
            .points
            .iter()
            .enumerate()
            .filter_map(|(idx, point)| {
                point
                    .magnitude_db
                    .is_finite()
                    .then_some((idx, point.magnitude_db))
            })
            .max_by(|a, b| a.1.total_cmp(&b.1))
        else {
            return;
        };

        self.peak_gain_db = Some(peak_db);
        self.peak_frequency = Some(self.points[peak_idx].frequency);

        // DC gain (from lowest frequency if < 100 Hz)
        if let Some(first) = self.points.first()
            && first.frequency < 100.0
        {
            self.dc_gain = Some(first.magnitude);
            self.dc_gain_db = Some(first.magnitude_db);
        }

        // Find -3dB cutoffs
        let threshold = peak_db - 3.0;

        // Low cutoff (before peak)
        self.cutoff_low = self.find_crossing_before(peak_idx, threshold);

        // High cutoff (after peak)
        self.cutoff_high = self.find_crossing_after(peak_idx, threshold);

        // Bandwidth
        if let (Some(fl), Some(fh)) = (self.cutoff_low, self.cutoff_high) {
            self.bandwidth = Some(fh - fl);

            // Q factor = f_center / bandwidth
            if let Some(fc) = self.peak_frequency {
                let bw = fh - fl;
                if bw > 0.0 {
                    self.q_factor = Some(fc / bw);
                }
            }
        }

        // Unity gain frequency and phase margin
        self.unity_gain_frequency = self.find_unity_gain_crossing();
        if let Some(ugf) = self.unity_gain_frequency {
            self.phase_margin = self.phase_at_frequency(ugf).map(|p| 180.0 + p);
        }
    }

    /// Find frequency where magnitude crosses threshold before index
    fn find_crossing_before(&self, before_idx: usize, threshold: Value) -> Option<Value> {
        for i in (1..before_idx).rev() {
            let db0 = self.points[i - 1].magnitude_db;
            let db1 = self.points[i].magnitude_db;
            let f0_raw = self.points[i - 1].frequency;
            let f1_raw = self.points[i].frequency;
            if !db0.is_finite()
                || !db1.is_finite()
                || !f0_raw.is_finite()
                || !f1_raw.is_finite()
                || f0_raw <= 0.0
                || f1_raw <= 0.0
            {
                continue;
            }

            if (db0 <= threshold && db1 > threshold) || (db0 >= threshold && db1 < threshold) {
                let denom = db1 - db0;
                if denom.abs() < 1e-15 {
                    continue;
                }
                // Interpolate
                let f0 = f0_raw.log10();
                let f1 = f1_raw.log10();
                let alpha = (threshold - db0) / denom;
                let crossing = 10.0_f64.powf(f0 + alpha * (f1 - f0));
                if crossing.is_finite() {
                    return Some(crossing);
                }
            }
        }
        None
    }

    /// Find frequency where magnitude crosses threshold after index
    fn find_crossing_after(&self, after_idx: usize, threshold: Value) -> Option<Value> {
        for i in after_idx..self.points.len() - 1 {
            let db0 = self.points[i].magnitude_db;
            let db1 = self.points[i + 1].magnitude_db;
            let f0_raw = self.points[i].frequency;
            let f1_raw = self.points[i + 1].frequency;
            if !db0.is_finite()
                || !db1.is_finite()
                || !f0_raw.is_finite()
                || !f1_raw.is_finite()
                || f0_raw <= 0.0
                || f1_raw <= 0.0
            {
                continue;
            }

            if (db0 >= threshold && db1 < threshold) || (db0 <= threshold && db1 > threshold) {
                let denom = db1 - db0;
                if denom.abs() < 1e-15 {
                    continue;
                }
                // Interpolate
                let f0 = f0_raw.log10();
                let f1 = f1_raw.log10();
                let alpha = (threshold - db0) / denom;
                let crossing = 10.0_f64.powf(f0 + alpha * (f1 - f0));
                if crossing.is_finite() {
                    return Some(crossing);
                }
            }
        }
        None
    }

    /// Find unity gain (0 dB) crossing frequency
    fn find_unity_gain_crossing(&self) -> Option<Value> {
        for i in 0..self.points.len() - 1 {
            let db0 = self.points[i].magnitude_db;
            let db1 = self.points[i + 1].magnitude_db;
            let f0_raw = self.points[i].frequency;
            let f1_raw = self.points[i + 1].frequency;
            if !db0.is_finite()
                || !db1.is_finite()
                || !f0_raw.is_finite()
                || !f1_raw.is_finite()
                || f0_raw <= 0.0
                || f1_raw <= 0.0
            {
                continue;
            }

            if (db0 >= 0.0 && db1 < 0.0) || (db0 <= 0.0 && db1 > 0.0) {
                let denom = db1 - db0;
                if denom.abs() < 1e-15 {
                    continue;
                }
                let f0 = f0_raw.log10();
                let f1 = f1_raw.log10();
                let alpha = (0.0 - db0) / denom;
                let crossing = 10.0_f64.powf(f0 + alpha * (f1 - f0));
                if crossing.is_finite() {
                    return Some(crossing);
                }
            }
        }
        None
    }

    /// Get phase at specific frequency (interpolated)
    fn phase_at_frequency(&self, freq: Value) -> Option<Value> {
        if !freq.is_finite() || freq <= 0.0 {
            return None;
        }
        for i in 0..self.points.len() - 1 {
            if self.points[i].frequency <= freq && self.points[i + 1].frequency >= freq {
                let f0_raw = self.points[i].frequency;
                let f1_raw = self.points[i + 1].frequency;
                let p0 = self.points[i].phase_deg;
                let p1 = self.points[i + 1].phase_deg;
                if !f0_raw.is_finite()
                    || !f1_raw.is_finite()
                    || !p0.is_finite()
                    || !p1.is_finite()
                    || f0_raw <= 0.0
                    || f1_raw <= 0.0
                {
                    continue;
                }
                let f0 = f0_raw.log10();
                let f1 = f1_raw.log10();
                let denom = f1 - f0;
                if denom.abs() < 1e-15 {
                    continue;
                }
                let alpha = (freq.log10() - f0) / denom;
                let phase = p0 + alpha * (p1 - p0);
                if phase.is_finite() {
                    return Some(phase);
                }
            }
        }
        None
    }
}

/// Configuration for AC transfer function analysis
#[derive(Debug, Clone)]
pub struct AcTransferConfig {
    /// Output node name
    pub output_node: String,

    /// Reference node (ground if None)
    pub ref_node: Option<String>,

    /// Input source name
    pub input_source: String,

    /// Start frequency (Hz)
    pub freq_start: Value,

    /// Stop frequency (Hz)
    pub freq_stop: Value,

    /// Number of points per decade (for decade sweep)
    pub points_per_decade: usize,

    /// Sweep type
    pub sweep_type: AcSweepType,
}

/// AC frequency sweep type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AcSweepType {
    /// Linear sweep
    Linear,
    /// Decade (logarithmic) sweep
    #[default]
    Decade,
    /// Octave sweep
    Octave,
}

impl AcTransferConfig {
    /// Create decade sweep configuration
    pub fn decade(
        output_node: &str,
        input_source: &str,
        freq_start: Value,
        freq_stop: Value,
        points_per_decade: usize,
    ) -> Self {
        Self {
            output_node: output_node.to_string(),
            ref_node: None,
            input_source: input_source.to_string(),
            freq_start,
            freq_stop,
            points_per_decade,
            sweep_type: AcSweepType::Decade,
        }
    }

    /// Create linear sweep configuration
    pub fn linear(
        output_node: &str,
        input_source: &str,
        freq_start: Value,
        freq_stop: Value,
        num_points: usize,
    ) -> Self {
        Self {
            output_node: output_node.to_string(),
            ref_node: None,
            input_source: input_source.to_string(),
            freq_start,
            freq_stop,
            points_per_decade: num_points, // Repurpose for total points
            sweep_type: AcSweepType::Linear,
        }
    }

    /// Set reference node
    pub fn with_ref(mut self, ref_node: &str) -> Self {
        self.ref_node = Some(ref_node.to_string());
        self
    }

    /// Validate sweep configuration.
    pub fn validate(&self) -> Result<(), String> {
        if self.points_per_decade == 0 {
            return Err("AC transfer sweep must have at least one point".to_string());
        }
        if !self.freq_start.is_finite() {
            return Err("AC transfer start frequency must be finite".to_string());
        }
        if !self.freq_stop.is_finite() || self.freq_stop < self.freq_start {
            return Err("AC transfer stop frequency must be finite and >= start".to_string());
        }

        match self.sweep_type {
            AcSweepType::Linear if self.freq_start < 0.0 => {
                Err("AC transfer start frequency must be non-negative".to_string())
            }
            AcSweepType::Decade | AcSweepType::Octave if self.freq_start <= 0.0 => Err(
                "AC transfer start frequency must be positive for logarithmic sweeps".to_string(),
            ),
            _ => Ok(()),
        }
    }

    /// Generate frequency points
    pub fn frequency_points(&self) -> Vec<Value> {
        self.try_frequency_points().unwrap_or_default()
    }

    /// Generate frequency points, preserving validation failures for callers
    /// that need to distinguish invalid input from an empty result.
    pub fn try_frequency_points(&self) -> Result<Vec<Value>, String> {
        self.validate()?;

        Ok(match self.sweep_type {
            AcSweepType::Linear => {
                let n = self.points_per_decade;
                if n <= 1 {
                    vec![self.freq_start]
                } else {
                    let step = (self.freq_stop - self.freq_start) / (n - 1) as Value;
                    (0..n)
                        .map(|i| self.freq_start + i as Value * step)
                        .collect()
                }
            }
            AcSweepType::Decade => {
                let log_start = self.freq_start.log10();
                let log_stop = self.freq_stop.log10();
                let num_decades = log_stop - log_start;
                let total_points = (num_decades * self.points_per_decade as f64).ceil() as usize;
                let total_points = total_points.max(1);

                (0..total_points)
                    .map(|i| {
                        let log_f = log_start
                            + (log_stop - log_start) * i as f64 / (total_points - 1).max(1) as f64;
                        10.0_f64.powf(log_f)
                    })
                    .collect()
            }
            AcSweepType::Octave => {
                let log2_start = self.freq_start.log2();
                let log2_stop = self.freq_stop.log2();
                let num_octaves = log2_stop - log2_start;
                let total_points = (num_octaves * self.points_per_decade as f64).ceil() as usize;
                let total_points = total_points.max(1);

                (0..total_points)
                    .map(|i| {
                        let log2_f = log2_start
                            + (log2_stop - log2_start) * i as f64
                                / (total_points - 1).max(1) as f64;
                        2.0_f64.powf(log2_f)
                    })
                    .collect()
            }
        })
    }
}

/// AC Transfer Function Analyzer
pub struct AcTransferAnalyzer {
    config: AcTransferConfig,
}

impl AcTransferAnalyzer {
    /// Create new analyzer
    pub fn new(config: AcTransferConfig) -> Self {
        Self { config }
    }

    /// Analyze using a transfer function evaluator
    ///
    /// The evaluator should return H(jω) for given frequency
    pub fn analyze<F>(&self, mut evaluator: F) -> AcTransferResult
    where
        F: FnMut(Value) -> Complex64,
    {
        let mut result = AcTransferResult::new(&self.config.output_node, &self.config.input_source);

        for freq in self.config.frequency_points() {
            let h = evaluator(freq);
            result.add_point(AcTransferPoint::new(freq, h));
        }

        result.compute_characteristics();
        result
    }

    /// Create a test lowpass filter transfer function
    ///
    /// H(s) = ω₀ / (s + ω₀) = 1 / (1 + s/ω₀)
    pub fn test_lowpass(&self, cutoff_freq: Value) -> AcTransferResult {
        let omega_0 = 2.0 * PI * cutoff_freq;

        self.analyze(|freq| {
            let s = Complex64::new(0.0, 2.0 * PI * freq);
            Complex64::new(omega_0, 0.0) / (s + Complex64::new(omega_0, 0.0))
        })
    }

    /// Create a test highpass filter transfer function
    ///
    /// H(s) = s / (s + ω₀)
    pub fn test_highpass(&self, cutoff_freq: Value) -> AcTransferResult {
        let omega_0 = 2.0 * PI * cutoff_freq;

        self.analyze(|freq| {
            let s = Complex64::new(0.0, 2.0 * PI * freq);
            s / (s + Complex64::new(omega_0, 0.0))
        })
    }

    /// Create a test bandpass filter transfer function
    ///
    /// H(s) = ωₒ/Q · s / (s² + ωₒ/Q · s + ωₒ²)
    pub fn test_bandpass(&self, center_freq: Value, q_factor: Value) -> AcTransferResult {
        let omega_0 = 2.0 * PI * center_freq;
        let omega_q = omega_0 / q_factor;

        self.analyze(|freq| {
            let s = Complex64::new(0.0, 2.0 * PI * freq);
            let num = Complex64::new(omega_q, 0.0) * s;
            let denom =
                s * s + s * Complex64::new(omega_q, 0.0) + Complex64::new(omega_0 * omega_0, 0.0);
            num / denom
        })
    }

    /// Create a test two-pole lowpass (Butterworth-like)
    ///
    /// H(s) = ω₀² / (s² + √2·ω₀·s + ω₀²)
    pub fn test_butterworth_lowpass(&self, cutoff_freq: Value) -> AcTransferResult {
        let omega_0 = 2.0 * PI * cutoff_freq;
        let omega_0_sq = omega_0 * omega_0;
        let sqrt2_omega_0 = 2.0_f64.sqrt() * omega_0;

        self.analyze(|freq| {
            let s = Complex64::new(0.0, 2.0 * PI * freq);
            Complex64::new(omega_0_sq, 0.0)
                / (s * s + s * Complex64::new(sqrt2_omega_0, 0.0) + Complex64::new(omega_0_sq, 0.0))
        })
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_frequency_points_preserves_invalid_grid_errors() {
        let config = AcTransferConfig::decade("out", "vin", 0.0, 1.0e6, 10);

        let err = config
            .try_frequency_points()
            .expect_err("invalid AC transfer sweep should return a validation error");

        assert!(
            err.contains("start frequency"),
            "unexpected AC transfer error: {err}"
        );
        assert!(
            config.frequency_points().is_empty(),
            "legacy frequency_points should expose invalid grids as empty"
        );
    }

    #[test]
    fn frequency_points_rejects_non_finite_sweeps() {
        for config in [
            AcTransferConfig::linear("out", "vin", f64::NAN, 1.0e6, 10),
            AcTransferConfig::decade("out", "vin", 1.0, f64::INFINITY, 10),
            AcTransferConfig::decade("out", "vin", 1.0e6, 1.0, 10),
            AcTransferConfig::linear("out", "vin", 1.0, 1.0e6, 0),
        ] {
            assert!(
                config.frequency_points().is_empty(),
                "invalid AC transfer sweep config produced points: {config:?}"
            );
        }
    }
}
