//! PNoise Result Types
//!
//! Comprehensive result structures for phase noise analysis including:
//! - Spectral density at offset frequencies (dBc/Hz)
//! - Individual noise contributor breakdown
//! - RMS phase jitter calculation
//! - Spot noise values

use crate::Value;
use std::collections::HashMap;

#[inline]
fn interpolate_over_frequency(
    query_freq: Value,
    f0: Value,
    y0: Value,
    f1: Value,
    y1: Value,
) -> Value {
    if (f1 - f0).abs() < 1e-18 {
        return y0;
    }

    let (x, x0, x1) = if query_freq > 0.0 && f0 > 0.0 && f1 > 0.0 {
        (query_freq.log10(), f0.log10(), f1.log10())
    } else {
        (query_freq, f0, f1)
    };

    if (x1 - x0).abs() < 1e-18 {
        return y0;
    }

    let t = (x - x0) / (x1 - x0);
    y0 + t * (y1 - y0)
}

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
        if self.spectral_points.is_empty() || !offset_freq.is_finite() {
            return None;
        }

        // Find bracketing points for interpolation
        let mut below: Option<&PhaseNoisePoint> = None;
        let mut above: Option<&PhaseNoisePoint> = None;

        for point in &self.spectral_points {
            if point.offset_freq <= offset_freq
                && below
                    .map(|b| point.offset_freq > b.offset_freq)
                    .unwrap_or(true)
            {
                below = Some(point);
            }
            if point.offset_freq >= offset_freq
                && above
                    .map(|a| point.offset_freq < a.offset_freq)
                    .unwrap_or(true)
            {
                above = Some(point);
            }
        }

        match (below, above) {
            (Some(b), Some(a)) if (a.offset_freq - b.offset_freq).abs() < 1e-10 => {
                Some(b.pn_dbc_hz)
            }
            (Some(b), Some(a)) => Some(interpolate_over_frequency(
                offset_freq,
                b.offset_freq,
                b.pn_dbc_hz,
                a.offset_freq,
                a.pn_dbc_hz,
            )),
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
        if self.contributions.is_empty() || !offset_freq.is_finite() {
            return None;
        }

        let mut below: Option<(Value, Value)> = None;
        let mut above: Option<(Value, Value)> = None;

        for &(freq, contrib) in &self.contributions {
            if freq <= offset_freq && below.map(|(f, _)| freq > f).unwrap_or(true) {
                below = Some((freq, contrib));
            }
            if freq >= offset_freq && above.map(|(f, _)| freq < f).unwrap_or(true) {
                above = Some((freq, contrib));
            }
        }

        match (below, above) {
            (Some((fb, cb)), Some((fa, _ca))) if (fa - fb).abs() < 1e-10 => Some(cb),
            (Some((fb, cb)), Some((fa, ca))) => {
                Some(interpolate_over_frequency(offset_freq, fb, cb, fa, ca))
            }
            (Some((_, cb)), None) => Some(cb),
            (None, Some((_, ca))) => Some(ca),
            (None, None) => None,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

