//! PNoise Result Types
//!
//! Comprehensive result structures for phase noise analysis including:
//! - Spectral density at offset frequencies (dBc/Hz)
//! - Individual noise contributor breakdown
//! - RMS phase jitter calculation
//! - Spot noise values

use crate::Value;
use std::collections::HashMap;

/// Complete phase noise analysis result
#[derive(Debug, Clone)]
pub struct PnoiseResult {
    /// Phase noise spectral density at each offset frequency
    pub spectral_points: Vec<PhaseNoisePoint>,

    /// Carrier/reference frequency [Hz]
    pub carrier_freq: Value,

    /// Individual noise contributors (device-by-device breakdown)
    pub contributors: Vec<NoiseContributor>,

    /// RMS phase jitter within integration bandwidth [seconds]
    pub rms_jitter: Option<Value>,

    /// RMS phase error within integration bandwidth [radians]
    pub rms_phase_error: Option<Value>,

    /// Integration bandwidth used for jitter calculation
    pub jitter_bandwidth: Option<(Value, Value)>,

    /// Analysis converged successfully
    pub converged: bool,

    /// Node names used in analysis
    pub output_node: String,
}

impl PnoiseResult {
    /// Create new phase noise result
    pub fn new(carrier_freq: Value, output_node: &str) -> Self {
        Self {
            spectral_points: Vec::new(),
            carrier_freq,
            contributors: Vec::new(),
            rms_jitter: None,
            rms_phase_error: None,
            jitter_bandwidth: None,
            converged: false,
            output_node: output_node.to_string(),
        }
    }

    /// Add a spectral point
    pub fn add_point(&mut self, point: PhaseNoisePoint) {
        self.spectral_points.push(point);
    }

    /// Add a noise contributor
    pub fn add_contributor(&mut self, contributor: NoiseContributor) {
        self.contributors.push(contributor);
    }

    /// Set RMS jitter result
    pub fn set_jitter(&mut self, jitter_seconds: Value, phase_radians: Value, bw: (Value, Value)) {
        self.rms_jitter = Some(jitter_seconds);
        self.rms_phase_error = Some(phase_radians);
        self.jitter_bandwidth = Some(bw);
    }

    /// Get phase noise at specific offset frequency (interpolated)
    pub fn phase_noise_at(&self, offset_freq: Value) -> Option<Value> {
        if self.spectral_points.is_empty() {
            return None;
        }

        // Find bracketing points for interpolation
        let mut below = None;
        let mut above = None;

        for point in &self.spectral_points {
            if point.offset_freq <= offset_freq {
                below = Some(point);
            }
            if point.offset_freq >= offset_freq && above.is_none() {
                above = Some(point);
            }
        }

        match (below, above) {
            (Some(b), Some(a)) if (a.offset_freq - b.offset_freq).abs() < 1e-10 => {
                Some(b.pn_dbc_hz)
            }
            (Some(b), Some(a)) => {
                // Log-linear interpolation (common for phase noise)
                let log_f = offset_freq.log10();
                let log_fb = b.offset_freq.log10();
                let log_fa = a.offset_freq.log10();
                let t = (log_f - log_fb) / (log_fa - log_fb);
                Some(b.pn_dbc_hz + t * (a.pn_dbc_hz - b.pn_dbc_hz))
            }
            (Some(b), None) => Some(b.pn_dbc_hz),
            (None, Some(a)) => Some(a.pn_dbc_hz),
            (None, None) => None,
        }
    }

    /// Get spot noise at standard offsets (1kHz, 10kHz, 100kHz, 1MHz)
    pub fn spot_noise_summary(&self) -> HashMap<String, Value> {
        let mut summary = HashMap::new();

        for offset in [1e3, 10e3, 100e3, 1e6] {
            if let Some(pn) = self.phase_noise_at(offset) {
                let label = if offset >= 1e6 {
                    format!("{}MHz", offset / 1e6)
                } else {
                    format!("{}kHz", offset / 1e3)
                };
                summary.insert(label, pn);
            }
        }

        summary
    }

    /// Get total integrated noise power [dBc] over frequency range
    pub fn integrated_noise_power(&self, f_start: Value, f_stop: Value) -> Option<Value> {
        if self.spectral_points.len() < 2 {
            return None;
        }

        // Trapezoidal integration in linear power
        let mut total_power = 0.0;

        for i in 1..self.spectral_points.len() {
            let p0 = &self.spectral_points[i - 1];
            let p1 = &self.spectral_points[i];

            // Check if segment overlaps integration range
            if p1.offset_freq < f_start || p0.offset_freq > f_stop {
                continue;
            }

            // Clamp to integration range
            let f0 = p0.offset_freq.max(f_start);
            let f1 = p1.offset_freq.min(f_stop);

            if f1 <= f0 {
                continue;
            }

            // Convert dBc/Hz to linear power spectral density
            let psd0 = 10.0_f64.powf(p0.pn_dbc_hz / 10.0);
            let psd1 = 10.0_f64.powf(p1.pn_dbc_hz / 10.0);

            // Trapezoidal rule
            total_power += (psd0 + psd1) / 2.0 * (f1 - f0);
        }

        if total_power > 0.0 {
            Some(10.0 * total_power.log10())
        } else {
            None
        }
    }

    /// Convert RMS jitter to picoseconds
    pub fn rms_jitter_ps(&self) -> Option<Value> {
        self.rms_jitter.map(|j| j * 1e12)
    }

    /// Number of spectral points
    pub fn num_points(&self) -> usize {
        self.spectral_points.len()
    }

    /// Check if result is valid
    pub fn is_valid(&self) -> bool {
        self.converged && !self.spectral_points.is_empty()
    }
}

/// Single phase noise measurement point
#[derive(Debug, Clone)]
pub struct PhaseNoisePoint {
    /// Offset frequency from carrier [Hz]
    pub offset_freq: Value,

    /// Phase noise spectral density [dBc/Hz]
    pub pn_dbc_hz: Value,

    /// Amplitude noise contribution [dBc/Hz] (if separated)
    pub am_noise: Option<Value>,

    /// Phase noise contribution from upper sideband [dBc/Hz]
    pub upper_sideband: Option<Value>,

    /// Phase noise contribution from lower sideband [dBc/Hz]
    pub lower_sideband: Option<Value>,
}

impl PhaseNoisePoint {
    /// Create new phase noise point
    pub fn new(offset_freq: Value, pn_dbc_hz: Value) -> Self {
        Self {
            offset_freq,
            pn_dbc_hz,
            am_noise: None,
            upper_sideband: None,
            lower_sideband: None,
        }
    }

    /// Create with sideband breakdown
    pub fn with_sidebands(offset_freq: Value, upper: Value, lower: Value) -> Self {
        // Combine sidebands: power adds linearly
        let upper_linear = 10.0_f64.powf(upper / 10.0);
        let lower_linear = 10.0_f64.powf(lower / 10.0);
        let combined = 10.0 * (upper_linear + lower_linear).log10();

        Self {
            offset_freq,
            pn_dbc_hz: combined,
            am_noise: None,
            upper_sideband: Some(upper),
            lower_sideband: Some(lower),
        }
    }

    /// Set AM noise component
    pub fn with_am_noise(mut self, am_dbc_hz: Value) -> Self {
        self.am_noise = Some(am_dbc_hz);
        self
    }
}

/// Individual noise contributor (for noise summary)
#[derive(Debug, Clone)]
pub struct NoiseContributor {
    /// Device/element name
    pub name: String,

    /// Device type (e.g., "resistor", "mosfet", "bjt")
    pub device_type: String,

    /// Noise contribution at each offset frequency [dBc/Hz]
    pub contributions: Vec<(Value, Value)>, // (offset_freq, contribution)

    /// Percentage of total noise at reference offset
    pub percentage: Option<Value>,
}

impl NoiseContributor {
    /// Create new noise contributor
    pub fn new(name: &str, device_type: &str) -> Self {
        Self {
            name: name.to_string(),
            device_type: device_type.to_string(),
            contributions: Vec::new(),
            percentage: None,
        }
    }

    /// Add contribution at offset frequency
    pub fn add_contribution(&mut self, offset_freq: Value, contribution_dbc: Value) {
        self.contributions.push((offset_freq, contribution_dbc));
    }

    /// Set percentage contribution
    pub fn with_percentage(mut self, pct: Value) -> Self {
        self.percentage = Some(pct);
        self
    }

    /// Get contribution at specific offset (interpolated)
    pub fn contribution_at(&self, offset_freq: Value) -> Option<Value> {
        if self.contributions.is_empty() {
            return None;
        }

        // Simple nearest-neighbor for now
        self.contributions
            .iter()
            .min_by(|a, b| {
                (a.0 - offset_freq)
                    .abs()
                    .partial_cmp(&(b.0 - offset_freq).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|&(_, c)| c)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod result_tests {
    use super::*;

    #[test]
    fn test_pnoise_result_new() {
        let result = PnoiseResult::new(1e9, "vco_out");
        assert_eq!(result.carrier_freq, 1e9);
        assert_eq!(result.output_node, "vco_out");
        assert!(!result.converged);
        assert!(result.spectral_points.is_empty());
    }

    #[test]
    fn test_pnoise_result_add_points() {
        let mut result = PnoiseResult::new(1e9, "out");
        result.add_point(PhaseNoisePoint::new(1e3, -80.0));
        result.add_point(PhaseNoisePoint::new(10e3, -100.0));
        result.add_point(PhaseNoisePoint::new(100e3, -120.0));

        assert_eq!(result.num_points(), 3);
    }

    #[test]
    fn test_pnoise_result_interpolation() {
        let mut result = PnoiseResult::new(1e9, "out");
        result.add_point(PhaseNoisePoint::new(1e3, -80.0));
        result.add_point(PhaseNoisePoint::new(10e3, -100.0));
        result.add_point(PhaseNoisePoint::new(100e3, -120.0));

        // Exact match
        assert!((result.phase_noise_at(1e3).unwrap() - (-80.0)).abs() < 0.1);

        // Interpolated
        let pn_5k = result.phase_noise_at(5e3).unwrap();
        assert!(pn_5k < -80.0 && pn_5k > -100.0);
    }

    #[test]
    fn test_pnoise_result_spot_summary() {
        let mut result = PnoiseResult::new(1e9, "out");
        result.add_point(PhaseNoisePoint::new(1e3, -80.0));
        result.add_point(PhaseNoisePoint::new(10e3, -100.0));
        result.add_point(PhaseNoisePoint::new(100e3, -120.0));
        result.add_point(PhaseNoisePoint::new(1e6, -140.0));

        let summary = result.spot_noise_summary();
        assert!(summary.contains_key("1kHz"));
        assert!(summary.contains_key("10kHz"));
        assert!(summary.contains_key("100kHz"));
        assert!(summary.contains_key("1MHz"));
    }

    #[test]
    fn test_pnoise_result_jitter() {
        let mut result = PnoiseResult::new(1e9, "out");
        result.set_jitter(1e-12, 6.28e-3, (1e3, 10e6));

        assert!(result.rms_jitter.is_some());
        assert_eq!(result.rms_jitter_ps().unwrap(), 1.0);
        assert!(result.jitter_bandwidth.is_some());
    }

    #[test]
    fn test_pnoise_result_validity() {
        let mut result = PnoiseResult::new(1e9, "out");
        assert!(!result.is_valid());

        result.add_point(PhaseNoisePoint::new(1e3, -80.0));
        assert!(!result.is_valid()); // Not converged

        result.converged = true;
        assert!(result.is_valid());
    }

    #[test]
    fn test_phase_noise_point_new() {
        let point = PhaseNoisePoint::new(10e3, -100.0);
        assert_eq!(point.offset_freq, 10e3);
        assert_eq!(point.pn_dbc_hz, -100.0);
        assert!(point.am_noise.is_none());
    }

    #[test]
    fn test_phase_noise_point_sidebands() {
        let point = PhaseNoisePoint::with_sidebands(10e3, -103.0, -103.0);

        // Equal sidebands at -103 dBc/Hz each combine to -100 dBc/Hz
        assert!((point.pn_dbc_hz - (-100.0)).abs() < 0.1);
        assert_eq!(point.upper_sideband, Some(-103.0));
        assert_eq!(point.lower_sideband, Some(-103.0));
    }

    #[test]
    fn test_phase_noise_point_am() {
        let point = PhaseNoisePoint::new(10e3, -100.0).with_am_noise(-110.0);

        assert_eq!(point.am_noise, Some(-110.0));
    }

    #[test]
    fn test_noise_contributor_new() {
        let contrib = NoiseContributor::new("R1", "resistor");
        assert_eq!(contrib.name, "R1");
        assert_eq!(contrib.device_type, "resistor");
        assert!(contrib.contributions.is_empty());
    }

    #[test]
    fn test_noise_contributor_add() {
        let mut contrib = NoiseContributor::new("M1", "mosfet");
        contrib.add_contribution(1e3, -90.0);
        contrib.add_contribution(10e3, -110.0);

        assert_eq!(contrib.contributions.len(), 2);
    }

    #[test]
    fn test_noise_contributor_percentage() {
        let contrib = NoiseContributor::new("R1", "resistor").with_percentage(45.5);

        assert_eq!(contrib.percentage, Some(45.5));
    }

    #[test]
    fn test_noise_contributor_interpolation() {
        let mut contrib = NoiseContributor::new("R1", "resistor");
        contrib.add_contribution(1e3, -90.0);
        contrib.add_contribution(10e3, -110.0);

        // Nearest neighbor
        let c = contrib.contribution_at(5e3).unwrap();
        assert!(c == -90.0 || c == -110.0);
    }

    #[test]
    fn test_integrated_noise_power() {
        let mut result = PnoiseResult::new(1e9, "out");
        // Flat phase noise of -100 dBc/Hz from 1kHz to 10kHz
        result.add_point(PhaseNoisePoint::new(1e3, -100.0));
        result.add_point(PhaseNoisePoint::new(10e3, -100.0));

        let integrated = result.integrated_noise_power(1e3, 10e3);
        assert!(integrated.is_some());

        // Integrated over 9kHz bandwidth: -100 + 10*log10(9000) ≈ -60.5 dBc
        let expected = -100.0 + 10.0 * 9000.0_f64.log10();
        assert!((integrated.unwrap() - expected).abs() < 1.0);
    }
}
