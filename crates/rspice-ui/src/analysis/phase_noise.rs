//! Phase Noise Plots
//!
//! Integrated phase noise visualization for oscillator analysis.
//! Matches Cadence SpectreRF's pnoise analysis output format.
//!
//! # Features
//!
//! - Phase noise vs. offset frequency plots (dBc/Hz)
//! - Jitter integration with RMS calculation
//! - Spot noise markers
//! - Multiple carrier support

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

// =============================================================================
// Phase Noise Point
// =============================================================================

/// A single phase noise data point
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PhaseNoisePoint {
    /// Offset frequency from carrier (Hz)
    pub offset_freq: f64,
    /// Phase noise power (dBc/Hz)
    pub phase_noise: f64,
}

impl PhaseNoisePoint {
    /// Create a new point
    pub fn new(offset_freq: f64, phase_noise: f64) -> Self {
        Self {
            offset_freq,
            phase_noise,
        }
    }
}

// =============================================================================
// Phase Noise Data
// =============================================================================

/// Complete phase noise dataset
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhaseNoiseData {
    /// Carrier frequency (Hz)
    pub carrier_freq: f64,
    /// Carrier power (dBm)
    pub carrier_power: f64,
    /// Phase noise points
    pub points: Vec<PhaseNoisePoint>,
    /// Name/label
    pub name: String,
}

impl PhaseNoiseData {
    /// Create new phase noise data
    pub fn new(carrier_freq: f64, carrier_power: f64) -> Self {
        Self {
            carrier_freq,
            carrier_power,
            ..Default::default()
        }
    }

    /// Add a data point
    pub fn add_point(&mut self, offset_freq: f64, phase_noise: f64) {
        self.points
            .push(PhaseNoisePoint::new(offset_freq, phase_noise));
    }

    /// Get phase noise at specific offset (linear interpolation)
    pub fn at_offset(&self, offset_freq: f64) -> Option<f64> {
        if self.points.is_empty() {
            return None;
        }

        // Find bracketing points
        for i in 0..self.points.len() - 1 {
            let p1 = &self.points[i];
            let p2 = &self.points[i + 1];

            if offset_freq >= p1.offset_freq && offset_freq <= p2.offset_freq {
                // Log-linear interpolation
                let log_f = offset_freq.log10();
                let log_f1 = p1.offset_freq.log10();
                let log_f2 = p2.offset_freq.log10();
                let t = (log_f - log_f1) / (log_f2 - log_f1);
                return Some(p1.phase_noise + t * (p2.phase_noise - p1.phase_noise));
            }
        }

        None
    }

    /// Calculate integrated jitter (RMS, in seconds)
    pub fn integrated_jitter(&self, f_low: f64, f_high: f64) -> f64 {
        if self.points.len() < 2 {
            return 0.0;
        }

        let mut integral = 0.0;

        for i in 0..self.points.len() - 1 {
            let p1 = &self.points[i];
            let p2 = &self.points[i + 1];

            // Skip points outside integration range
            if p2.offset_freq < f_low || p1.offset_freq > f_high {
                continue;
            }

            // Clamp to integration range
            let f1 = p1.offset_freq.max(f_low);
            let f2 = p2.offset_freq.min(f_high);

            if f2 <= f1 {
                continue;
            }

            // Average phase noise in this band (in dBc/Hz)
            let pn1 = self.at_offset(f1).unwrap_or(p1.phase_noise);
            let pn2 = self.at_offset(f2).unwrap_or(p2.phase_noise);
            let avg_pn_db = (pn1 + pn2) / 2.0;

            // Convert to linear and integrate
            let avg_pn_linear = 10.0f64.powf(avg_pn_db / 10.0);
            integral += avg_pn_linear * (f2 - f1);
        }

        // Convert to RMS jitter
        // jitter_rms = sqrt(2 * integral) / (2 * pi * f_carrier)
        let jitter_rms = (2.0 * integral).sqrt() / (2.0 * PI * self.carrier_freq);
        jitter_rms
    }

    /// Get minimum phase noise and its offset
    pub fn min_phase_noise(&self) -> Option<(f64, f64)> {
        self.points
            .iter()
            .filter(|point| point.offset_freq.is_finite() && point.phase_noise.is_finite())
            .min_by(|a, b| a.phase_noise.total_cmp(&b.phase_noise))
            .map(|p| (p.offset_freq, p.phase_noise))
    }

    /// Get phase noise at common offset frequencies
    pub fn spot_noise(&self) -> SpotNoiseValues {
        SpotNoiseValues {
            at_1khz: self.at_offset(1e3),
            at_10khz: self.at_offset(1e4),
            at_100khz: self.at_offset(1e5),
            at_1mhz: self.at_offset(1e6),
            at_10mhz: self.at_offset(1e7),
        }
    }
}

/// Common spot noise values
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpotNoiseValues {
    /// Phase noise at 1 kHz offset
    pub at_1khz: Option<f64>,
    /// Phase noise at 10 kHz offset
    pub at_10khz: Option<f64>,
    /// Phase noise at 100 kHz offset
    pub at_100khz: Option<f64>,
    /// Phase noise at 1 MHz offset
    pub at_1mhz: Option<f64>,
    /// Phase noise at 10 MHz offset
    pub at_10mhz: Option<f64>,
}

// =============================================================================
// Phase Noise Analysis State
// =============================================================================

/// State for phase noise viewer
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PhaseNoiseState {
    /// Phase noise datasets
    pub datasets: Vec<PhaseNoiseData>,
    /// Integration range low (Hz)
    pub integration_low: f64,
    /// Integration range high (Hz)
    pub integration_high: f64,
    /// Whether to show calculated jitter
    pub show_jitter: bool,
    /// Whether to show spot noise markers
    pub show_markers: bool,
    /// Selected dataset index
    pub selected_dataset: Option<usize>,
    /// X-axis in log scale
    pub log_x: bool,
    /// Y-axis range (min, max)
    pub y_range: (f64, f64),
}

impl PhaseNoiseState {
    /// Create a new state
    pub fn new() -> Self {
        Self {
            integration_low: 12e3,  // 12 kHz (GSM)
            integration_high: 20e6, // 20 MHz
            show_jitter: true,
            show_markers: true,
            log_x: true,
            y_range: (-160.0, -60.0),
            ..Default::default()
        }
    }

    /// Add a dataset
    pub fn add_dataset(&mut self, data: PhaseNoiseData) {
        self.datasets.push(data);
    }

    /// Get selected dataset
    pub fn current_dataset(&self) -> Option<&PhaseNoiseData> {
        self.selected_dataset.and_then(|idx| self.datasets.get(idx))
    }

    /// Calculate jitter for current dataset
    pub fn current_jitter(&self) -> Option<f64> {
        self.current_dataset()
            .map(|d| d.integrated_jitter(self.integration_low, self.integration_high))
    }

    /// Get jitter in picoseconds
    pub fn current_jitter_ps(&self) -> Option<f64> {
        self.current_jitter().map(|j| j * 1e12)
    }
}

// =============================================================================
// Noise Figure of Merit
// =============================================================================

/// Calculate oscillator figure of merit (FoM)
/// FoM = L(f_offset) - 20*log10(f_carrier/f_offset) + 10*log10(P_dc/1mW)
pub fn figure_of_merit(
    phase_noise: f64,  // dBc/Hz at offset
    carrier_freq: f64, // Hz
    offset_freq: f64,  // Hz
    power_mw: f64,     // DC power in mW
) -> f64 {
    phase_noise - 20.0 * (carrier_freq / offset_freq).log10() + 10.0 * power_mw.log10()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let pt = PhaseNoisePoint::new(1e6, -120.0);
        assert_eq!(pt.offset_freq, 1e6);
        assert_eq!(pt.phase_noise, -120.0);
    }

    #[test]
    fn test_data_creation() {
        let data = PhaseNoiseData::new(1e9, 0.0);
        assert_eq!(data.carrier_freq, 1e9);
        assert!(data.points.is_empty());
    }

    #[test]
    fn test_data_add_point() {
        let mut data = PhaseNoiseData::new(1e9, 0.0);
        data.add_point(1e3, -80.0);
        data.add_point(1e6, -120.0);

        assert_eq!(data.points.len(), 2);
    }

    #[test]
    fn test_data_at_offset() {
        let mut data = PhaseNoiseData::new(1e9, 0.0);
        data.add_point(1e3, -80.0);
        data.add_point(1e6, -120.0);

        // Exact match at boundaries
        assert!((data.at_offset(1e3).unwrap() - (-80.0)).abs() < 1e-10);
        assert!((data.at_offset(1e6).unwrap() - (-120.0)).abs() < 1e-10);

        // Interpolated value (geometric mean of 1e3 and 1e6 = 10^4.5 ≈ 31623)
        let mid = data.at_offset(10.0f64.powf(4.5)).unwrap();
        assert!(mid > -120.0 && mid < -80.0);
    }

    #[test]
    fn test_spot_noise() {
        let mut data = PhaseNoiseData::new(1e9, 0.0);
        data.add_point(1e3, -80.0);
        data.add_point(1e4, -100.0);
        data.add_point(1e5, -110.0);
        data.add_point(1e6, -120.0);

        let spots = data.spot_noise();
        assert!((spots.at_1khz.unwrap() - (-80.0)).abs() < 0.1);
        assert!((spots.at_1mhz.unwrap() - (-120.0)).abs() < 0.1);
    }

    #[test]
    fn test_integrated_jitter() {
        let mut data = PhaseNoiseData::new(1e9, 0.0);
        // Flat -100 dBc/Hz from 1 kHz to 10 MHz
        data.add_point(1e3, -100.0);
        data.add_point(1e7, -100.0);

        let jitter = data.integrated_jitter(1e3, 1e7);
        assert!(jitter > 0.0);
        // For -100 dBc/Hz flat, jitter should be in ps range
        assert!(jitter < 1e-9); // Less than 1 ns
    }

    #[test]
    fn test_min_phase_noise() {
        let mut data = PhaseNoiseData::new(1e9, 0.0);
        data.add_point(1e3, -80.0);
        data.add_point(1e6, -120.0);
        data.add_point(1e7, -110.0);

        let (offset, pn) = data.min_phase_noise().unwrap();
        assert_eq!(offset, 1e6);
        assert_eq!(pn, -120.0);
    }

    #[test]
    fn test_min_phase_noise_ignores_non_finite_values() {
        let mut data = PhaseNoiseData::new(1e9, 0.0);
        data.add_point(1e3, -80.0);
        data.add_point(f64::NAN, -140.0);
        data.add_point(1e6, f64::NAN);
        data.add_point(1e5, -110.0);

        let (offset, pn) = data.min_phase_noise().expect("finite points should exist");
        assert_eq!(offset, 1e5);
        assert_eq!(pn, -110.0);
    }

    #[test]
    fn test_state_creation() {
        let state = PhaseNoiseState::new();
        assert!(state.log_x);
        assert!(state.show_markers);
    }

    #[test]
    fn test_state_add_dataset() {
        let mut state = PhaseNoiseState::new();
        state.add_dataset(PhaseNoiseData::new(1e9, 0.0));

        assert_eq!(state.datasets.len(), 1);
    }

    #[test]
    fn test_figure_of_merit() {
        let fom = figure_of_merit(-120.0, 1e9, 1e6, 10.0);
        // FoM = -120 - 20*log10(1e9/1e6) + 10*log10(10)
        // FoM = -120 - 60 + 10 = -170
        assert!((fom - (-170.0)).abs() < 0.1);
    }

    #[test]
    fn test_jitter_ps() {
        let mut state = PhaseNoiseState::new();
        let mut data = PhaseNoiseData::new(1e9, 0.0);
        data.add_point(1e3, -100.0);
        data.add_point(1e7, -100.0);
        state.add_dataset(data);
        state.selected_dataset = Some(0);

        let jitter_ps = state.current_jitter_ps().unwrap();
        assert!(jitter_ps > 0.0);
    }
}
