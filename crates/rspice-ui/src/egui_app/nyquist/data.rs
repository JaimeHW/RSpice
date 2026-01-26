//! Nyquist Plot Data Structures
//!
//! Core data types for Nyquist/Polar plot analysis.

use std::f64::consts::PI;

// =============================================================================
// Nyquist Point
// =============================================================================

/// Single point on Nyquist plot
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NyquistPoint {
    /// Frequency in Hz
    pub frequency: f64,
    /// Real part of loop gain
    pub real: f64,
    /// Imaginary part of loop gain
    pub imag: f64,
}

impl NyquistPoint {
    /// Create new point
    pub fn new(frequency: f64, real: f64, imag: f64) -> Self {
        Self {
            frequency,
            real,
            imag,
        }
    }

    /// Create from magnitude and phase
    pub fn from_polar(frequency: f64, magnitude: f64, phase_rad: f64) -> Self {
        Self {
            frequency,
            real: magnitude * phase_rad.cos(),
            imag: magnitude * phase_rad.sin(),
        }
    }

    /// Magnitude
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    /// Phase in radians
    pub fn phase(&self) -> f64 {
        self.imag.atan2(self.real)
    }

    /// Phase in degrees
    pub fn phase_deg(&self) -> f64 {
        self.phase() * 180.0 / PI
    }

    /// Distance from critical point (-1, 0)
    pub fn distance_from_critical(&self) -> f64 {
        ((self.real + 1.0).powi(2) + self.imag.powi(2)).sqrt()
    }
}

// =============================================================================
// Nyquist Data
// =============================================================================

/// Complete Nyquist plot data
#[derive(Debug, Clone)]
pub struct NyquistData {
    /// Name/label
    pub name: String,
    /// Data points (sorted by frequency)
    pub points: Vec<NyquistPoint>,
}

impl Default for NyquistData {
    fn default() -> Self {
        Self {
            name: String::new(),
            points: Vec::new(),
        }
    }
}

impl NyquistData {
    /// Create new empty data
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            points: Vec::new(),
        }
    }

    /// Create from frequency, real, imag arrays
    pub fn from_arrays(name: &str, freq: &[f64], real: &[f64], imag: &[f64]) -> Self {
        let n = freq.len().min(real.len()).min(imag.len());
        let points: Vec<NyquistPoint> = (0..n)
            .map(|i| NyquistPoint::new(freq[i], real[i], imag[i]))
            .collect();

        Self {
            name: name.to_string(),
            points,
        }
    }

    /// Create from Bode data (frequency, magnitude, phase_rad)
    pub fn from_bode(name: &str, freq: &[f64], mag: &[f64], phase: &[f64]) -> Self {
        let n = freq.len().min(mag.len()).min(phase.len());
        let points: Vec<NyquistPoint> = (0..n)
            .map(|i| NyquistPoint::from_polar(freq[i], mag[i], phase[i]))
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
    pub fn add_point(&mut self, point: NyquistPoint) {
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

    /// Real axis range
    pub fn real_range(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() {
            return None;
        }
        let min = self.points.iter().map(|p| p.real).fold(f64::MAX, f64::min);
        let max = self.points.iter().map(|p| p.real).fold(f64::MIN, f64::max);
        Some((min, max))
    }

    /// Imaginary axis range
    pub fn imag_range(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() {
            return None;
        }
        let min = self.points.iter().map(|p| p.imag).fold(f64::MAX, f64::min);
        let max = self.points.iter().map(|p| p.imag).fold(f64::MIN, f64::max);
        Some((min, max))
    }

    /// Minimum distance from critical point (-1, 0)
    pub fn min_distance_from_critical(&self) -> Option<f64> {
        self.points
            .iter()
            .map(|p| p.distance_from_critical())
            .fold(None, |acc, d| Some(acc.map_or(d, |a: f64| a.min(d))))
    }

    /// Count encirclements of critical point
    /// Positive = clockwise, Negative = counter-clockwise
    pub fn count_encirclements(&self) -> i32 {
        if self.points.len() < 2 {
            return 0;
        }

        let mut total_angle = 0.0;

        for window in self.points.windows(2) {
            // Vector from critical point (-1, 0) to each point
            let dx0 = window[0].real + 1.0;
            let dy0 = window[0].imag;
            let dx1 = window[1].real + 1.0;
            let dy1 = window[1].imag;

            let angle0 = dy0.atan2(dx0);
            let angle1 = dy1.atan2(dx1);

            let mut delta = angle1 - angle0;

            // Handle wraparound
            while delta > PI {
                delta -= 2.0 * PI;
            }
            while delta < -PI {
                delta += 2.0 * PI;
            }

            total_angle += delta;
        }

        (total_angle / (2.0 * PI)).round() as i32
    }

    /// Check if system is stable (no encirclement of -1,0 for open-loop stable)
    pub fn is_stable_open_loop(&self) -> bool {
        self.count_encirclements() == 0
    }

    /// Gain margin (at phase = -180°)
    pub fn gain_margin(&self) -> Option<f64> {
        // Find where curve crosses negative real axis (imag ≈ 0, real < 0)
        for point in &self.points {
            if point.imag.abs() < 0.01 && point.real < 0.0 {
                // Gain margin = 1 / |H(jω)| at this frequency
                let mag = point.magnitude();
                if mag > 0.0 {
                    return Some(1.0 / mag);
                }
            }
        }
        None
    }

    /// Phase margin (at |H| = 1 crossover)
    pub fn phase_margin(&self) -> Option<f64> {
        // Find where magnitude ≈ 1
        for point in &self.points {
            let mag = point.magnitude();
            if (mag - 1.0).abs() < 0.05 {
                // Phase margin = 180° + phase at this point
                return Some(180.0 + point.phase_deg());
            }
        }
        None
    }
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
    // NyquistPoint Tests
    // =========================================================================

    #[test]
    fn test_point_new() {
        let p = NyquistPoint::new(100.0, 0.5, 0.3);
        assert_eq!(p.frequency, 100.0);
        assert_eq!(p.real, 0.5);
        assert_eq!(p.imag, 0.3);
    }

    #[test]
    fn test_point_from_polar() {
        let p = NyquistPoint::from_polar(100.0, 1.0, 0.0);
        assert!(approx_eq(p.real, 1.0));
        assert!(approx_eq(p.imag, 0.0));

        let p2 = NyquistPoint::from_polar(100.0, 1.0, PI / 2.0);
        assert!(approx_eq(p2.real, 0.0));
        assert!(approx_eq(p2.imag, 1.0));
    }

    #[test]
    fn test_magnitude() {
        let p = NyquistPoint::new(100.0, 3.0, 4.0);
        assert!(approx_eq(p.magnitude(), 5.0));
    }

    #[test]
    fn test_phase() {
        let p = NyquistPoint::new(100.0, 1.0, 1.0);
        assert!(approx_eq(p.phase(), PI / 4.0));
    }

    #[test]
    fn test_phase_deg() {
        let p = NyquistPoint::new(100.0, 0.0, 1.0);
        assert!(approx_eq(p.phase_deg(), 90.0));
    }

    #[test]
    fn test_distance_from_critical() {
        // At critical point
        let p = NyquistPoint::new(100.0, -1.0, 0.0);
        assert!(approx_eq(p.distance_from_critical(), 0.0));

        // At origin
        let p2 = NyquistPoint::new(100.0, 0.0, 0.0);
        assert!(approx_eq(p2.distance_from_critical(), 1.0));
    }

    // =========================================================================
    // NyquistData Tests
    // =========================================================================

    #[test]
    fn test_data_new() {
        let data = NyquistData::new("Test");
        assert!(data.is_empty());
        assert_eq!(data.name, "Test");
    }

    #[test]
    fn test_data_from_arrays() {
        let freq = vec![100.0, 1000.0, 10000.0];
        let real = vec![1.0, 0.0, -0.5];
        let imag = vec![0.0, -1.0, -0.5];

        let data = NyquistData::from_arrays("Test", &freq, &real, &imag);
        assert_eq!(data.len(), 3);
    }

    #[test]
    fn test_data_from_bode() {
        let freq = vec![100.0, 1000.0];
        let mag = vec![1.0, 0.5];
        let phase = vec![0.0, -PI / 2.0];

        let data = NyquistData::from_bode("Test", &freq, &mag, &phase);
        assert_eq!(data.len(), 2);
        assert!(approx_eq(data.points[0].real, 1.0));
        assert!(approx_eq(data.points[1].imag, -0.5));
    }

    #[test]
    fn test_frequency_range() {
        let data = NyquistData::from_arrays(
            "Test",
            &[100.0, 1000.0, 10000.0],
            &[0.0, 0.0, 0.0],
            &[0.0, 0.0, 0.0],
        );

        let (min, max) = data.frequency_range().unwrap();
        assert_eq!(min, 100.0);
        assert_eq!(max, 10000.0);
    }

    #[test]
    fn test_real_range() {
        let data = NyquistData::from_arrays(
            "Test",
            &[100.0, 1000.0, 10000.0],
            &[-2.0, 0.0, 1.0],
            &[0.0, 0.0, 0.0],
        );

        let (min, max) = data.real_range().unwrap();
        assert_eq!(min, -2.0);
        assert_eq!(max, 1.0);
    }

    #[test]
    fn test_min_distance() {
        let data = NyquistData::from_arrays("Test", &[100.0, 1000.0], &[-0.5, 0.0], &[0.0, 0.0]);

        let min_dist = data.min_distance_from_critical().unwrap();
        assert!(approx_eq(min_dist, 0.5)); // Point at (-0.5, 0) is 0.5 from (-1, 0)
    }

    #[test]
    fn test_no_encirclement() {
        // Curve stays in right half plane
        let data = NyquistData::from_arrays(
            "Test",
            &[100.0, 1000.0, 10000.0],
            &[0.5, 0.3, 0.1],
            &[-0.2, -0.1, 0.0],
        );

        assert_eq!(data.count_encirclements(), 0);
        assert!(data.is_stable_open_loop());
    }

    #[test]
    fn test_encirclement_detection() {
        // Create a circle around (-1, 0)
        let n = 12;
        let mut points = Vec::new();
        for i in 0..n {
            let angle = 2.0 * PI * i as f64 / n as f64;
            let real = -1.0 + 0.5 * angle.cos();
            let imag = 0.5 * angle.sin();
            points.push(NyquistPoint::new(i as f64 * 100.0, real, imag));
        }
        // Close the loop
        points.push(points[0]);

        let mut data = NyquistData::new("Circle");
        for p in points {
            data.add_point(p);
        }

        // Should detect 1 encirclement
        let enc = data.count_encirclements();
        assert!(enc != 0);
    }
}
