//! PNoise Result Types
//!
//! Comprehensive result structures for phase noise analysis including:
//! - Spectral density at offset frequencies (dBc/Hz)
//! - Individual noise contributor breakdown
//! - RMS phase jitter calculation
//! - Spot noise values

use crate::Value;

#[inline]
fn interpolate_over_frequency(
    query_freq: Value,
    f0: Value,
    y0: Value,
    f1: Value,
    y1: Value,
) -> Value {
    if query_freq == f0 || y0 == y1 {
        return y0;
    }
    if query_freq == f1 {
        return y1;
    }
    // Interior interpolation in dB is geometric in linear power: a zero
    // endpoint dominates, including a segment that is noiseless throughout.
    if y0 == Value::NEG_INFINITY || y1 == Value::NEG_INFINITY {
        return Value::NEG_INFINITY;
    }

    let (left_weight, right_weight, width) = if f0 > 0.0 {
        fn log_ratio(upper: Value, lower: Value) -> Value {
            let relative = (upper - lower) / lower;
            if relative.is_finite() {
                relative.ln_1p()
            } else {
                upper.ln() - lower.ln()
            }
        }
        // Separate distances preserve close positive frequencies and both
        // endpoint weights; subtracting rounded logarithms can collapse them.
        let left = log_ratio(f1, query_freq);
        let right = log_ratio(query_freq, f0);
        (left, right, left + right)
    } else {
        (f1 - query_freq, query_freq - f0, f1 - f0)
    };
    let left = crate::numerics::scaled_exp_product(&[y0, left_weight], &[width], 0.0);
    let right = crate::numerics::scaled_exp_product(&[y1, right_weight], &[width], 0.0);
    // A convex combination cannot leave its endpoints, even if rounding an
    // extreme finite endpoint sum overflows by an ulp.
    (left + right).clamp(y0.min(y1), y0.max(y1))
}

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

    /// Interpolate dB noise against log frequency, using linear frequency for
    /// a segment starting at zero. Queries outside the sampled range clamp to
    /// its nearest endpoint. Invalid frequencies/densities return `None`;
    /// negative infinity is accepted as exactly noiseless. Samples may be unordered.
    pub fn phase_noise_at(&self, offset_freq: Value) -> Option<Value> {
        if self.spectral_points.is_empty() || !offset_freq.is_finite() || offset_freq < 0.0 {
            return None;
        }

        // Find bracketing points for interpolation
        let mut below: Option<&PhaseNoisePoint> = None;
        let mut above: Option<&PhaseNoisePoint> = None;

        for point in &self.spectral_points {
            if !point.offset_freq.is_finite()
                || point.offset_freq < 0.0
                || (!point.pn_dbc_hz.is_finite() && point.pn_dbc_hz != Value::NEG_INFINITY)
            {
                return None;
            }
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
            (Some(b), Some(a)) if a.offset_freq == b.offset_freq => Some(b.pn_dbc_hz),
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

    /// Integrated single-sideband noise power in dBc, using linear-PSD
    /// trapezoids clipped to the requested frequency range. This integration
    /// policy is distinct from the log-frequency spot interpolation in
    /// [`Self::phase_noise_at`]. No extrapolation is performed.
    ///
    /// Returns `None` for invalid bounds, an invalid/unordered spectrum, or
    /// no positive-width overlap. An exactly noiseless band returns negative
    /// infinity. Finite dB densities need not have representable linear powers.
    pub fn integrated_noise_power(&self, f_start: Value, f_stop: Value) -> Option<Value> {
        crate::analysis::noise::integrate_decibel_noise(
            self.spectral_points
                .iter()
                .map(|point| (point.offset_freq, point.pn_dbc_hz)),
            Some((f_start, f_stop)),
        )
        .ok()?
        .map(|total| total.decibels())
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

    /// Add contribution at offset frequency
    pub fn add_contribution(&mut self, offset_freq: Value, contribution_dbc: Value) {
        self.contributions.push((offset_freq, contribution_dbc));
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn spectrum(points: &[(Value, Value)]) -> PnoiseResult {
        let mut result = PnoiseResult::new(1e6, "out");
        for &(frequency, density) in points {
            result.add_point(PhaseNoisePoint::new(frequency, density));
        }
        result
    }

    #[test]
    fn spot_noise_interpolation_preserves_frequency_and_density_range() {
        for (f0, fm, f1) in [
            (1e-14, 1e-13, 1e-12),
            (1e-300, 1.0, 1e300),
            (
                Value::from_bits(1),
                Value::from_bits(2),
                Value::from_bits(4),
            ),
        ] {
            let result = spectrum(&[(f1, -160.0), (f0, -100.0)]);
            assert!((result.phase_noise_at(fm).unwrap() + 130.0).abs() < 3e-14);
        }
        let f0 = 1.0_f64;
        let fm = Value::from_bits(f0.to_bits() + 1);
        let f1 = Value::from_bits(f0.to_bits() + 2);
        assert!(
            (spectrum(&[(f0, -100.0), (f1, -160.0)])
                .phase_noise_at(fm)
                .unwrap()
                + 130.0)
                .abs()
                < 3e-14
        );
        let result = spectrum(&[(1.0, Value::MAX), (100.0, -Value::MAX)]);
        assert_eq!(result.phase_noise_at(10.0), Some(0.0));
        let result = spectrum(&[(0.0, 0.0), (1e300, 1e300)]);
        assert_eq!(result.phase_noise_at(1e-300), Some(1e-300));
    }

    #[test]
    fn spot_noise_interpolation_handles_zero_power_and_invalid_samples() {
        let result = spectrum(&[(1.0, Value::NEG_INFINITY), (100.0, Value::NEG_INFINITY)]);
        assert_eq!(result.phase_noise_at(10.0), Some(Value::NEG_INFINITY));
        let result = spectrum(&[(1.0, Value::NEG_INFINITY), (100.0, -100.0)]);
        assert_eq!(result.phase_noise_at(10.0), Some(Value::NEG_INFINITY));
        assert_eq!(result.phase_noise_at(100.0), Some(-100.0));
        assert_eq!(result.phase_noise_at(1000.0), Some(-100.0));
        assert_eq!(result.phase_noise_at(-1.0), None);
        for invalid in [Value::NAN, Value::INFINITY] {
            assert_eq!(
                spectrum(&[(1.0, invalid), (2.0, -100.0)]).phase_noise_at(2.0),
                None
            );
        }
        assert_eq!(
            spectrum(&[(-1.0, -100.0), (2.0, -100.0)]).phase_noise_at(2.0),
            None
        );
    }

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

    #[test]
    fn integrated_noise_clips_linear_psd_at_both_band_edges() {
        // PSD = 4*f - 3 on [1,3]; its primitive is 2*f^2 - 3*f.
        let result = spectrum(&[(1.0, 0.0), (3.0, 10.0 * 9.0_f64.log10())]);
        for (start, stop, expected_power) in [
            (1.0, 2.0, 3.0_f64),
            (2.0, 3.0, 7.0),
            (1.5, 2.5, 5.0),
            (0.0, 4.0, 10.0),
        ] {
            let actual = result.integrated_noise_power(start, stop).unwrap();
            assert!((actual - 10.0 * expected_power.log10()).abs() < 2e-14);
        }
    }

    #[test]
    fn integrated_noise_keeps_extreme_density_bandwidth_and_clipping_factors() {
        for (density, start, stop, expected) in [
            (4000.0, 1e-300, 2e-300, 1000.0),
            (-4000.0, 1e100, 2e100, -3000.0),
            (Value::MAX, 1.0, 2.0, Value::MAX),
            (-Value::MAX, 1.0, 2.0, -Value::MAX),
        ] {
            let actual = spectrum(&[(start, density), (stop, density)])
                .integrated_noise_power(start, stop)
                .unwrap();
            assert!((actual - expected).abs() <= expected.abs() * 2e-15);
        }
        // A rising triangle clipped near zero: power = 10^400 * b^2/(2*10^300).
        // b/span underflows; the integrated power and its dB value do not.
        let result = spectrum(&[(0.0, Value::NEG_INFINITY), (1e300, 4000.0)]);
        let actual = result.integrated_noise_power(0.0, 1e-100).unwrap();
        assert!((actual - (-1000.0 - 10.0 * 2.0_f64.log10())).abs() < 1e-12);
        // A relative density that underflows before multiplication by width
        // still supplies half of the total area.
        let result = spectrum(&[
            (0.0, 0.0),
            (1e-100, Value::NEG_INFINITY),
            (2e-100, -4000.0),
            (1e300, -4000.0),
        ]);
        let actual = result.integrated_noise_power(0.0, 1e300).unwrap();
        assert!((actual - (-1000.0 + 10.0 * 1.5_f64.log10())).abs() < 2e-12);
    }

    #[test]
    fn integrated_noise_rejects_invalid_series_and_distinguishes_noiseless_overlap() {
        let zero = spectrum(&[(1.0, Value::NEG_INFINITY), (2.0, Value::NEG_INFINITY)]);
        assert_eq!(
            zero.integrated_noise_power(1.0, 2.0),
            Some(Value::NEG_INFINITY)
        );
        for (start, stop) in [
            (0.0, 1.0),
            (2.0, 3.0),
            (2.0, 1.0),
            (-1.0, 2.0),
            (Value::NAN, 2.0),
            (1.0, Value::INFINITY),
        ] {
            assert_eq!(zero.integrated_noise_power(start, stop), None);
        }
        for points in [
            [(2.0, 0.0), (1.0, 0.0)],
            [(1.0, 0.0), (1.0, 0.0)],
            [(1.0, Value::NAN), (2.0, 0.0)],
            [(1.0, Value::INFINITY), (2.0, 0.0)],
            [(Value::NEG_INFINITY, 0.0), (2.0, 0.0)],
        ] {
            assert_eq!(spectrum(&points).integrated_noise_power(1.0, 2.0), None);
        }
    }
}
