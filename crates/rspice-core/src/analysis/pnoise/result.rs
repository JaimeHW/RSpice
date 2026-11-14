//! PNoise Result Types
//!
//! Comprehensive result structures for phase noise analysis including:
//! - Spectral density at offset frequencies (dBc/Hz)
//! - Individual noise contributor breakdown
//! - RMS phase jitter calculation
//! - Spot noise values

use crate::Value;

/// Complete phase noise analysis result
#[derive(Debug, Clone)]
pub struct PnoiseResult {
    /// Phase noise spectral density at each offset frequency
    pub spectral_points: Vec<PhaseNoisePoint>,

    /// Carrier/reference frequency `Hz`
    pub carrier_freq: Value,

    /// Individual noise contributors (device-by-device breakdown)
    pub contributors: Vec<NoiseContributor>,

    /// RMS phase jitter within integration bandwidth `seconds`
    pub rms_jitter: Option<Value>,

    /// RMS phase error within integration bandwidth `radians`
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
    /// Offset frequency from carrier `Hz`
    pub offset_freq: Value,

    /// Phase noise spectral density `dBc/Hz`
    pub pn_dbc_hz: Value,

    /// Amplitude noise contribution `dBc/Hz` (if separated)
    pub am_noise: Option<Value>,

    /// Phase noise contribution from upper sideband `dBc/Hz`
    pub upper_sideband: Option<Value>,

    /// Phase noise contribution from lower sideband `dBc/Hz`
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
        // Add powers relative to the larger dB value. Neither absolute
        // linear power needs to fit in f64, and ln_1p retains a weak sideband.
        let largest = upper.max(lower);
        let combined = if upper.is_nan() || lower.is_nan() {
            Value::NAN
        } else if !largest.is_finite() {
            largest
        } else {
            largest
                + (10.0 / std::f64::consts::LN_10)
                    * 10.0_f64.powf((upper.min(lower) - largest) / 10.0).ln_1p()
        };

        Self {
            offset_freq,
            pn_dbc_hz: combined,
            am_noise: None,
            upper_sideband: Some(upper),
            lower_sideband: Some(lower),
        }
    }
}

/// Individual noise contributor (for noise summary)
#[derive(Debug, Clone)]
pub struct NoiseContributor {
    /// Device/element name
    pub name: String,

    /// Device type (e.g., "resistor", "mosfet", "bjt")
    pub device_type: String,

    /// Noise contribution at each offset frequency `dBc/Hz`
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
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sideband_power_sum_preserves_extreme_decibels_and_weak_terms() {
        for db in [4000.0, -4000.0, 0.0, Value::MAX, -Value::MAX] {
            let actual = PhaseNoisePoint::with_sidebands(1.0, db, db).pn_dbc_hz;
            let expected = db + 10.0 * 2.0_f64.log10();
            assert!((actual - expected).abs() <= 2.0 * Value::EPSILON * expected.abs());
            assert_eq!(
                PhaseNoisePoint::with_sidebands(1.0, db, Value::NEG_INFINITY).pn_dbc_hz,
                db
            );
        }
        let weak = PhaseNoisePoint::with_sidebands(1.0, 0.0, -200.0).pn_dbc_hz;
        assert!((weak / (1e-20 * 10.0 / std::f64::consts::LN_10) - 1.0).abs() < 2e-15);
        assert_eq!(
            PhaseNoisePoint::with_sidebands(1.0, Value::NEG_INFINITY, Value::NEG_INFINITY)
                .pn_dbc_hz,
            Value::NEG_INFINITY
        );
        assert!(
            PhaseNoisePoint::with_sidebands(1.0, Value::NAN, 0.0)
                .pn_dbc_hz
                .is_nan()
        );
    }
}
