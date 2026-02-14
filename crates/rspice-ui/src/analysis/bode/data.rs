//! Bode Plot Data Structures
//!
//! Core data types for frequency response analysis and Bode plots.

use std::f64::consts::PI;

// =============================================================================
// Frequency Response Data
// =============================================================================

/// Single frequency response data point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrequencyPoint {
    /// Frequency in Hz
    pub frequency: f64,
    /// Magnitude (linear)
    pub magnitude: f64,
    /// Phase in radians
    pub phase: f64,
}

impl FrequencyPoint {
    /// Create new frequency point
    pub fn new(frequency: f64, magnitude: f64, phase: f64) -> Self {
        Self {
            frequency,
            magnitude,
            phase,
        }
    }

    /// Create from complex value
    pub fn from_complex(frequency: f64, real: f64, imag: f64) -> Self {
        let magnitude = (real * real + imag * imag).sqrt();
        let phase = imag.atan2(real);
        Self {
            frequency,
            magnitude,
            phase,
        }
    }

    /// Magnitude in dB
    pub fn magnitude_db(&self) -> f64 {
        if self.magnitude <= 0.0 {
            f64::NEG_INFINITY
        } else {
            20.0 * self.magnitude.log10()
        }
    }

    /// Phase in degrees
    pub fn phase_deg(&self) -> f64 {
        self.phase * 180.0 / PI
    }

    /// Angular frequency (rad/s)
    pub fn omega(&self) -> f64 {
        2.0 * PI * self.frequency
    }
}

// =============================================================================
// Frequency Response
// =============================================================================

/// Complete frequency response data
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct FrequencyResponse {
    /// Name/label
    pub name: String,
    /// Data points (sorted by frequency)
    pub points: Vec<FrequencyPoint>,
}


impl FrequencyResponse {
    /// Create new empty response
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            points: Vec::new(),
        }
    }

    /// Create from frequency, magnitude, phase arrays
    pub fn from_arrays(name: &str, freq: &[f64], mag: &[f64], phase: &[f64]) -> Self {
        let n = freq.len().min(mag.len()).min(phase.len());
        let points: Vec<FrequencyPoint> = (0..n)
            .map(|i| FrequencyPoint::new(freq[i], mag[i], phase[i]))
            .collect();

        Self {
            name: name.to_string(),
            points,
        }
    }

    /// Create from frequency, real, imaginary arrays
    pub fn from_complex_arrays(name: &str, freq: &[f64], real: &[f64], imag: &[f64]) -> Self {
        let n = freq.len().min(real.len()).min(imag.len());
        let points: Vec<FrequencyPoint> = (0..n)
            .map(|i| FrequencyPoint::from_complex(freq[i], real[i], imag[i]))
            .collect();

        Self {
            name: name.to_string(),
            points,
        }
    }

    /// Number of points
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Is empty
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Add a point
    pub fn add_point(&mut self, point: FrequencyPoint) {
        self.points.push(point);
    }

    /// Frequency range
    pub fn frequency_range(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() {
            return None;
        }
        let min = self.points.first()?.frequency;
        let max = self.points.last()?.frequency;
        Some((min, max))
    }

    /// Magnitude range in dB
    pub fn magnitude_range_db(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() {
            return None;
        }
        let dbs: Vec<f64> = self
            .points
            .iter()
            .map(|p| p.magnitude_db())
            .filter(|db| db.is_finite())
            .collect();

        if dbs.is_empty() {
            return None;
        }

        let min = dbs.iter().copied().fold(f64::MAX, f64::min);
        let max = dbs.iter().copied().fold(f64::MIN, f64::max);
        Some((min, max))
    }

    /// Phase range in degrees
    pub fn phase_range_deg(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() {
            return None;
        }
        let degs: Vec<f64> = self.points.iter().map(|p| p.phase_deg()).collect();

        let min = degs.iter().copied().fold(f64::MAX, f64::min);
        let max = degs.iter().copied().fold(f64::MIN, f64::max);
        Some((min, max))
    }

    /// Get DC gain (magnitude at lowest frequency)
    pub fn dc_gain_db(&self) -> Option<f64> {
        self.points.first().map(|p| p.magnitude_db())
    }

    /// Get bandwidth (-3dB point)
    pub fn bandwidth_3db(&self) -> Option<f64> {
        let dc_gain = self.dc_gain_db()?;
        let target = dc_gain - 3.0;

        for window in self.points.windows(2) {
            let db0 = window[0].magnitude_db();
            let db1 = window[1].magnitude_db();

            if db0 >= target && db1 < target {
                // Interpolate
                let t = (target - db0) / (db1 - db0);
                let f0 = window[0].frequency.log10();
                let f1 = window[1].frequency.log10();
                return Some(10.0_f64.powf(f0 + t * (f1 - f0)));
            }
        }
        None
    }

    /// Interpolate at a specific frequency
    pub fn interpolate(&self, frequency: f64) -> Option<FrequencyPoint> {
        if self.points.is_empty() {
            return None;
        }

        // Find surrounding points (log interpolation)
        let log_f = frequency.log10();

        for window in self.points.windows(2) {
            let log_f0 = window[0].frequency.log10();
            let log_f1 = window[1].frequency.log10();

            if log_f >= log_f0 && log_f <= log_f1 {
                let t = (log_f - log_f0) / (log_f1 - log_f0);

                // Linear interpolation in dB/degrees
                let db = window[0].magnitude_db()
                    + t * (window[1].magnitude_db() - window[0].magnitude_db());
                let phase_deg =
                    window[0].phase_deg() + t * (window[1].phase_deg() - window[0].phase_deg());

                return Some(FrequencyPoint {
                    frequency,
                    magnitude: 10.0_f64.powf(db / 20.0),
                    phase: phase_deg * PI / 180.0,
                });
            }
        }

        None
    }
}

// =============================================================================
// Bode Data (Multiple Responses)
// =============================================================================

/// Collection of frequency responses for Bode plot
#[derive(Debug, Clone, Default)]
pub struct BodeData {
    /// All frequency responses
    pub responses: Vec<FrequencyResponse>,
    /// Calculated margins
    pub margins: Option<StabilityMargins>,
}

impl BodeData {
    /// Create new empty Bode data
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a frequency response
    pub fn add_response(&mut self, response: FrequencyResponse) {
        self.responses.push(response);
    }

    /// Number of responses
    pub fn response_count(&self) -> usize {
        self.responses.len()
    }

    /// Get first response
    pub fn primary(&self) -> Option<&FrequencyResponse> {
        self.responses.first()
    }

    /// Combined frequency range
    pub fn frequency_range(&self) -> Option<(f64, f64)> {
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for resp in &self.responses {
            if let Some((lo, hi)) = resp.frequency_range() {
                min = min.min(lo);
                max = max.max(hi);
            }
        }

        if min <= max {
            Some((min, max))
        } else {
            None
        }
    }

    /// Combined magnitude range in dB
    pub fn magnitude_range_db(&self) -> Option<(f64, f64)> {
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for resp in &self.responses {
            if let Some((lo, hi)) = resp.magnitude_range_db() {
                min = min.min(lo);
                max = max.max(hi);
            }
        }

        if min <= max {
            Some((min, max))
        } else {
            None
        }
    }

    /// Calculate stability margins for primary response
    pub fn calculate_margins(&mut self) {
        if let Some(resp) = self.primary() {
            self.margins = Some(StabilityMargins::calculate(resp));
        }
    }
}

// =============================================================================
// Stability Margins
// =============================================================================

/// Gain and phase margins for stability analysis
#[derive(Debug, Clone, Default)]
pub struct StabilityMargins {
    /// Gain margin in dB (at phase = -180°)
    pub gain_margin_db: Option<f64>,
    /// Frequency where gain margin is measured
    pub gain_margin_frequency: Option<f64>,
    /// Phase margin in degrees (at magnitude = 0dB)
    pub phase_margin_deg: Option<f64>,
    /// Frequency where phase margin is measured (crossover frequency)
    pub phase_margin_frequency: Option<f64>,
    /// Is system stable (both margins positive)?
    pub is_stable: bool,
}

impl StabilityMargins {
    /// Calculate margins from frequency response
    pub fn calculate(response: &FrequencyResponse) -> Self {
        let mut margins = Self::default();

        // Find gain crossover (0dB crossover)
        let crossover = find_crossover_frequency(response, 0.0);
        if let Some((freq, phase)) = crossover {
            margins.phase_margin_frequency = Some(freq);
            margins.phase_margin_deg = Some(180.0 + phase);
        }

        // Find phase crossover (-180° crossing)
        let phase_cross = find_phase_crossover(response, -180.0);
        if let Some((freq, mag_db)) = phase_cross {
            margins.gain_margin_frequency = Some(freq);
            margins.gain_margin_db = Some(-mag_db);
        }

        // Determine stability
        margins.is_stable = margins.gain_margin_db.is_none_or(|gm| gm > 0.0)
            && margins.phase_margin_deg.is_none_or(|pm| pm > 0.0);

        margins
    }

    /// Format gain margin
    pub fn format_gain_margin(&self) -> String {
        match self.gain_margin_db {
            Some(gm) if gm.is_finite() => format!("{:.1} dB", gm),
            _ => "∞".to_string(),
        }
    }

    /// Format phase margin
    pub fn format_phase_margin(&self) -> String {
        match self.phase_margin_deg {
            Some(pm) if pm.is_finite() => format!("{:.1}°", pm),
            _ => "N/A".to_string(),
        }
    }
}

/// Find frequency where magnitude crosses target dB
fn find_crossover_frequency(response: &FrequencyResponse, target_db: f64) -> Option<(f64, f64)> {
    for window in response.points.windows(2) {
        let db0 = window[0].magnitude_db();
        let db1 = window[1].magnitude_db();

        if (db0 >= target_db && db1 < target_db) || (db0 < target_db && db1 >= target_db) {
            let t = (target_db - db0) / (db1 - db0);
            let log_f0 = window[0].frequency.log10();
            let log_f1 = window[1].frequency.log10();
            let freq = 10.0_f64.powf(log_f0 + t * (log_f1 - log_f0));
            let phase = window[0].phase_deg() + t * (window[1].phase_deg() - window[0].phase_deg());
            return Some((freq, phase));
        }
    }
    None
}

/// Find frequency where phase crosses target degrees
fn find_phase_crossover(response: &FrequencyResponse, target_deg: f64) -> Option<(f64, f64)> {
    for window in response.points.windows(2) {
        let deg0 = window[0].phase_deg();
        let deg1 = window[1].phase_deg();

        if (deg0 >= target_deg && deg1 < target_deg) || (deg0 < target_deg && deg1 >= target_deg) {
            let t = (target_deg - deg0) / (deg1 - deg0);
            let log_f0 = window[0].frequency.log10();
            let log_f1 = window[1].frequency.log10();
            let freq = 10.0_f64.powf(log_f0 + t * (log_f1 - log_f0));
            let mag_db = window[0].magnitude_db()
                + t * (window[1].magnitude_db() - window[0].magnitude_db());
            return Some((freq, mag_db));
        }
    }
    None
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    fn approx_eq_rel(a: f64, b: f64, rel_tol: f64) -> bool {
        if b.abs() < EPSILON {
            a.abs() < EPSILON
        } else {
            ((a - b) / b).abs() < rel_tol
        }
    }

    // =========================================================================
    // FrequencyPoint Tests
    // =========================================================================

    #[test]
    fn test_frequency_point_new() {
        let p = FrequencyPoint::new(1000.0, 2.0, PI / 4.0);
        assert_eq!(p.frequency, 1000.0);
        assert_eq!(p.magnitude, 2.0);
    }

    #[test]
    fn test_frequency_point_from_complex() {
        let p = FrequencyPoint::from_complex(100.0, 1.0, 1.0);
        assert!(approx_eq_rel(p.magnitude, 2.0_f64.sqrt(), 0.01));
        assert!(approx_eq_rel(p.phase, PI / 4.0, 0.01));
    }

    #[test]
    fn test_magnitude_db() {
        let p = FrequencyPoint::new(100.0, 10.0, 0.0);
        assert!(approx_eq(p.magnitude_db(), 20.0));

        let p2 = FrequencyPoint::new(100.0, 1.0, 0.0);
        assert!(approx_eq(p2.magnitude_db(), 0.0));

        let p3 = FrequencyPoint::new(100.0, 0.1, 0.0);
        assert!(approx_eq(p3.magnitude_db(), -20.0));
    }

    #[test]
    fn test_magnitude_db_zero() {
        let p = FrequencyPoint::new(100.0, 0.0, 0.0);
        assert!(p.magnitude_db().is_infinite());
    }

    #[test]
    fn test_phase_deg() {
        let p = FrequencyPoint::new(100.0, 1.0, PI / 2.0);
        assert!(approx_eq(p.phase_deg(), 90.0));

        let p2 = FrequencyPoint::new(100.0, 1.0, -PI);
        assert!(approx_eq(p2.phase_deg(), -180.0));
    }

    #[test]
    fn test_omega() {
        let p = FrequencyPoint::new(1.0, 1.0, 0.0);
        assert!(approx_eq(p.omega(), 2.0 * PI));
    }

    // =========================================================================
    // FrequencyResponse Tests
    // =========================================================================

    #[test]
    fn test_frequency_response_new() {
        let resp = FrequencyResponse::new("Test");
        assert_eq!(resp.name, "Test");
        assert!(resp.is_empty());
    }

    #[test]
    fn test_frequency_response_from_arrays() {
        let freq = vec![100.0, 1000.0, 10000.0];
        let mag = vec![1.0, 0.707, 0.1];
        let phase = vec![0.0, -PI / 4.0, -PI / 2.0];

        let resp = FrequencyResponse::from_arrays("Test", &freq, &mag, &phase);
        assert_eq!(resp.len(), 3);
    }

    #[test]
    fn test_frequency_range() {
        let resp = FrequencyResponse::from_arrays(
            "Test",
            &[100.0, 1000.0, 10000.0],
            &[1.0, 1.0, 1.0],
            &[0.0, 0.0, 0.0],
        );

        let (min, max) = resp.frequency_range().unwrap();
        assert_eq!(min, 100.0);
        assert_eq!(max, 10000.0);
    }

    #[test]
    fn test_magnitude_range_db() {
        let resp = FrequencyResponse::from_arrays(
            "Test",
            &[100.0, 1000.0, 10000.0],
            &[10.0, 1.0, 0.1],
            &[0.0, 0.0, 0.0],
        );

        let (min, max) = resp.magnitude_range_db().unwrap();
        assert!(approx_eq(min, -20.0));
        assert!(approx_eq(max, 20.0));
    }

    #[test]
    fn test_dc_gain() {
        let resp = FrequencyResponse::from_arrays(
            "Test",
            &[10.0, 100.0, 1000.0],
            &[100.0, 10.0, 1.0],
            &[0.0, 0.0, 0.0],
        );

        let dc_gain = resp.dc_gain_db().unwrap();
        assert!(approx_eq(dc_gain, 40.0));
    }

    #[test]
    fn test_bandwidth_3db() {
        // Create a simple RC lowpass response
        let mut resp = FrequencyResponse::new("RC Lowpass");
        let fc = 1000.0; // 1kHz cutoff

        for i in 0..31 {
            let f = 10.0_f64.powf(i as f64 / 10.0 + 1.0); // 10Hz to 10kHz
            let omega = 2.0 * PI * f;
            let omega_c = 2.0 * PI * fc;
            let mag = 1.0 / (1.0 + (omega / omega_c).powi(2)).sqrt();
            let phase = -(omega / omega_c).atan();
            resp.add_point(FrequencyPoint::new(f, mag, phase));
        }

        let bw = resp.bandwidth_3db().unwrap();
        // Should be close to 1kHz
        assert!((bw - 1000.0).abs() / 1000.0 < 0.1);
    }

    #[test]
    fn test_interpolate() {
        let resp = FrequencyResponse::from_arrays(
            "Test",
            &[100.0, 1000.0],
            &[10.0, 1.0], // 20dB at 100Hz, 0dB at 1kHz
            &[0.0, -PI / 2.0],
        );

        // Interpolate at geometric mean = 316.2 Hz
        let mid = resp.interpolate(316.227766);
        assert!(mid.is_some());
        let mid = mid.unwrap();
        // Should be roughly 10dB (midpoint in dB scale)
        assert!((mid.magnitude_db() - 10.0).abs() < 1.0);
    }

    // =========================================================================
    // BodeData Tests
    // =========================================================================

    #[test]
    fn test_bode_data_new() {
        let data = BodeData::new();
        assert_eq!(data.response_count(), 0);
    }

    #[test]
    fn test_bode_data_add_response() {
        let mut data = BodeData::new();
        data.add_response(FrequencyResponse::new("Test"));
        assert_eq!(data.response_count(), 1);
    }

    #[test]
    fn test_bode_data_primary() {
        let mut data = BodeData::new();
        data.add_response(FrequencyResponse::new("First"));
        data.add_response(FrequencyResponse::new("Second"));

        assert_eq!(data.primary().unwrap().name, "First");
    }

    // =========================================================================
    // StabilityMargins Tests
    // =========================================================================

    #[test]
    fn test_margins_format() {
        let mut margins = StabilityMargins::default();
        margins.gain_margin_db = Some(10.0);
        margins.phase_margin_deg = Some(45.0);

        assert!(margins.format_gain_margin().contains("10"));
        assert!(margins.format_phase_margin().contains("45"));
    }

    #[test]
    fn test_margins_infinite() {
        let margins = StabilityMargins::default();
        assert!(margins.format_gain_margin().contains("∞"));
    }

    #[test]
    fn test_calculate_margins_stable() {
        // Create a stable system (phase margin at 0dB crossover)
        let mut resp = FrequencyResponse::new("Stable");

        // Magnitude decreasing, phase decreasing
        for i in 0..20 {
            let f = 10.0_f64.powf(i as f64 / 5.0); // 1 to 10kHz
            let mag = 100.0 / f.powf(0.5); // -10dB/dec slope
            let phase = -(f / 10.0).atan() * 2.0; // Slowly decreasing phase
            resp.add_point(FrequencyPoint::new(f, mag, phase));
        }

        let margins = StabilityMargins::calculate(&resp);
        // System should be stable
        assert!(margins.is_stable);
    }
}
