//! FFT Data Structures
//!
//! Core data types for FFT and spectrum analysis.

use std::f64::consts::PI;

use rustfft::{num_complex::Complex, FftPlanner};

use super::window::{apply_window_copy, generate_window, WindowFunction};

// =============================================================================
// FFT Point
// =============================================================================

/// Single point in FFT spectrum
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FftPoint {
    /// Frequency in Hz
    pub frequency: f64,
    /// Magnitude (linear)
    pub magnitude: f64,
    /// Phase in radians
    pub phase: f64,
}

impl FftPoint {
    /// Create new point
    pub fn new(frequency: f64, magnitude: f64, phase: f64) -> Self {
        Self {
            frequency,
            magnitude,
            phase,
        }
    }

    /// Create from complex components
    pub fn from_complex(frequency: f64, real: f64, imag: f64) -> Self {
        Self {
            frequency,
            magnitude: (real * real + imag * imag).sqrt(),
            phase: imag.atan2(real),
        }
    }

    /// Magnitude in dB (20 * log10)
    pub fn magnitude_db(&self) -> f64 {
        if self.magnitude <= 0.0 {
            f64::NEG_INFINITY
        } else {
            20.0 * self.magnitude.log10()
        }
    }

    /// Magnitude in dBV (reference 1V)
    pub fn magnitude_dbv(&self) -> f64 {
        self.magnitude_db()
    }

    /// Magnitude in dBm (reference 1mW into 50Ω)
    pub fn magnitude_dbm(&self, z0: f64) -> f64 {
        if self.magnitude <= 0.0 {
            f64::NEG_INFINITY
        } else {
            let z0 = if z0.is_finite() && z0 > 0.0 { z0 } else { 50.0 };
            let power_mw = (self.magnitude * self.magnitude) / z0 * 1000.0;
            10.0 * power_mw.log10()
        }
    }

    /// Phase in degrees
    pub fn phase_deg(&self) -> f64 {
        self.phase * 180.0 / PI
    }
}

// =============================================================================
// FFT Data
// =============================================================================

/// Complete FFT/spectrum data
#[derive(Debug, Clone)]
pub struct FftData {
    /// Name/label
    pub name: String,
    /// Spectrum points (positive frequencies only, DC to Nyquist)
    pub points: Vec<FftPoint>,
    /// Sample rate of original data
    pub sample_rate: f64,
    /// Number of points in original FFT
    pub fft_size: usize,
    /// Window function used
    pub window: WindowFunction,
}

impl Default for FftData {
    fn default() -> Self {
        Self {
            name: String::new(),
            points: Vec::new(),
            sample_rate: 1.0,
            fft_size: 0,
            window: WindowFunction::Hanning,
        }
    }
}

impl FftData {
    /// Create new empty FFT data
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Create from uniformly sampled time-domain data using FFT.
    pub fn from_time_domain(
        name: &str,
        data: &[f64],
        sample_rate: f64,
        window: WindowFunction,
    ) -> Self {
        let n = data.len();
        if n == 0 || !sample_rate.is_finite() || sample_rate <= 0.0 {
            return Self::new(name);
        }

        // Apply window
        let win = generate_window(window, n);
        let windowed = apply_window_copy(data, &win);

        // Coherent gain from actual generated coefficients, not a table constant.
        let cg = win.iter().sum::<f64>() / n as f64;
        if !cg.is_finite() || cg.abs() < 1e-15 {
            return Self::new(name);
        }

        let mut buffer: Vec<Complex<f64>> =
            windowed.into_iter().map(|x| Complex::new(x, 0.0)).collect();
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(n);
        fft.process(&mut buffer);

        // One-sided spectrum: include DC..Nyquist (Nyquist exists only for even n).
        let n_freqs = n / 2 + 1;
        let mut points = Vec::with_capacity(n_freqs);
        let base_scale = 1.0 / (n as f64 * cg);
        let has_nyquist = n % 2 == 0;

        for (k, bin) in buffer.iter().take(n_freqs).enumerate() {
            let freq = k as f64 * sample_rate / n as f64;
            let mut scale = base_scale;
            if k != 0 && !(has_nyquist && k == n / 2) {
                scale *= 2.0;
            }

            points.push(FftPoint::from_complex(freq, bin.re * scale, bin.im * scale));
        }

        Self {
            name: name.to_string(),
            points,
            sample_rate,
            fft_size: n,
            window,
        }
    }

    /// Create from magnitude/phase arrays
    pub fn from_spectrum(
        name: &str,
        frequencies: &[f64],
        magnitudes: &[f64],
        phases: &[f64],
        sample_rate: f64,
    ) -> Self {
        let n = frequencies.len().min(magnitudes.len()).min(phases.len());
        let points: Vec<FftPoint> = (0..n)
            .map(|i| FftPoint::new(frequencies[i], magnitudes[i], phases[i]))
            .collect();
        let fft_size = n.saturating_sub(1).saturating_mul(2);

        Self {
            name: name.to_string(),
            points,
            sample_rate,
            fft_size,
            window: WindowFunction::Rectangular,
        }
    }

    /// Number of frequency bins
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Is empty
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Frequency resolution (bin width)
    pub fn frequency_resolution(&self) -> f64 {
        if self.fft_size == 0 {
            return 0.0;
        }
        self.sample_rate / self.fft_size as f64
    }

    /// Nyquist frequency
    pub fn nyquist(&self) -> f64 {
        self.sample_rate / 2.0
    }

    /// Frequency range
    pub fn frequency_range(&self) -> Option<(f64, f64)> {
        if self.points.is_empty() {
            return None;
        }
        Some((
            self.points.first()?.frequency,
            self.points.last()?.frequency,
        ))
    }

    /// Magnitude range in dB
    pub fn magnitude_range_db(&self) -> Option<(f64, f64)> {
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

    /// DC component (bin 0)
    pub fn dc(&self) -> Option<&FftPoint> {
        self.points.first()
    }

    /// Find peak magnitude bin (excluding DC)
    pub fn find_peak(&self) -> Option<(usize, &FftPoint)> {
        if self.points.len() < 2 {
            return None;
        }

        self.points
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(_, point)| point.magnitude.is_finite())
            .max_by(|(_, a), (_, b)| a.magnitude.total_cmp(&b.magnitude))
    }

    /// Find all peaks above threshold
    pub fn find_peaks(&self, threshold_db: f64) -> Vec<(usize, &FftPoint)> {
        let mut peaks = Vec::new();

        for i in 1..self.points.len().saturating_sub(1) {
            let prev = self.points[i - 1].magnitude;
            let curr = self.points[i].magnitude;
            let next = self.points[i + 1].magnitude;

            if curr > prev && curr > next && self.points[i].magnitude_db() > threshold_db {
                peaks.push((i, &self.points[i]));
            }
        }

        peaks
    }

    /// Interpolate magnitude at specific frequency
    pub fn interpolate(&self, frequency: f64) -> Option<FftPoint> {
        if self.points.is_empty() {
            return None;
        }

        for window in self.points.windows(2) {
            if frequency >= window[0].frequency && frequency <= window[1].frequency {
                let t =
                    (frequency - window[0].frequency) / (window[1].frequency - window[0].frequency);

                return Some(FftPoint {
                    frequency,
                    magnitude: window[0].magnitude
                        + t * (window[1].magnitude - window[0].magnitude),
                    phase: window[0].phase + t * (window[1].phase - window[0].phase),
                });
            }
        }

        None
    }
}

// =============================================================================
// Spectrum Analysis
// =============================================================================

/// Analysis results from FFT spectrum
#[derive(Debug, Clone, Default)]
pub struct SpectrumAnalysis {
    /// Fundamental frequency (Hz)
    pub fundamental_frequency: Option<f64>,
    /// Fundamental magnitude (dB)
    pub fundamental_db: Option<f64>,
    /// Total Harmonic Distortion (%)
    pub thd_percent: Option<f64>,
    /// THD in dB
    pub thd_db: Option<f64>,
    /// Spurious-Free Dynamic Range (dB)
    pub sfdr_db: Option<f64>,
    /// Signal-to-Noise Ratio (dB)
    pub snr_db: Option<f64>,
    /// Signal-to-Noise-and-Distortion Ratio (dB)
    pub sinad_db: Option<f64>,
    /// Noise floor (dB)
    pub noise_floor_db: Option<f64>,
    /// Harmonic frequencies and magnitudes
    pub harmonics: Vec<(f64, f64)>,
}

impl SpectrumAnalysis {
    /// Analyze FFT data
    pub fn analyze(fft: &FftData, num_harmonics: usize) -> Self {
        let mut analysis = Self::default();
        let Some((fund_idx, _fund_peak)) = fft.find_peak() else {
            return analysis;
        };

        let fundamental = Self::interpolate_peak(fft, fund_idx).unwrap_or_else(|| {
            let p = fft.points[fund_idx];
            (fund_idx, p.frequency, p.magnitude, p.magnitude_db())
        });
        let (fund_bin, fund_freq, fund_mag, fund_db) = fundamental;
        let fund_power = fund_mag * fund_mag;
        if !(fund_power.is_finite() && fund_power > 0.0) {
            return analysis;
        }

        analysis.fundamental_frequency = Some(fund_freq);
        analysis.fundamental_db = Some(fund_db);

        let guard_bins = Self::guard_bins(fft.window);
        let mut excluded_for_noise = vec![false; fft.points.len()];
        let mut excluded_for_spur = vec![false; fft.points.len()];
        Self::exclude_bin_region(&mut excluded_for_noise, 0, 0);
        Self::exclude_bin_region(&mut excluded_for_noise, fund_bin, guard_bins);
        Self::exclude_bin_region(&mut excluded_for_spur, 0, 0);
        Self::exclude_bin_region(&mut excluded_for_spur, fund_bin, guard_bins);

        // Harmonic extraction
        let mut harmonic_power_sum = 0.0;
        let harmonic_count = num_harmonics.max(1);
        for h in 2..=harmonic_count {
            let target = fund_freq * h as f64;
            if target > fft.nyquist() {
                break;
            }
            let Some((harm_idx, harm_freq, harm_mag, harm_db)) =
                Self::find_harmonic_peak(fft, target)
            else {
                continue;
            };
            if !harm_mag.is_finite() || harm_mag <= 0.0 {
                continue;
            }

            analysis.harmonics.push((harm_freq, harm_db));
            harmonic_power_sum += harm_mag * harm_mag;
            Self::exclude_bin_region(&mut excluded_for_noise, harm_idx, guard_bins);
        }

        let thd_ratio = (harmonic_power_sum / fund_power).sqrt();
        if thd_ratio.is_finite() {
            analysis.thd_percent = Some(thd_ratio * 100.0);
            if thd_ratio > 0.0 {
                analysis.thd_db = Some(20.0 * thd_ratio.log10());
            }
        }

        // Largest spur outside guarded signal/harmonic bins.
        let mut largest_spur_db = f64::NEG_INFINITY;
        let mut noise_power_sum = 0.0;
        let mut noise_db_bins = Vec::new();
        for (idx, point) in fft.points.iter().enumerate().skip(1) {
            let mag = point.magnitude;
            if !mag.is_finite() || mag <= 0.0 {
                continue;
            }
            let db = point.magnitude_db();
            if !db.is_finite() {
                continue;
            }

            if !excluded_for_spur.get(idx).copied().unwrap_or(false) {
                largest_spur_db = largest_spur_db.max(db);
            }

            if excluded_for_noise.get(idx).copied().unwrap_or(false) {
                continue;
            }
            noise_power_sum += mag * mag;
            noise_db_bins.push(db);
        }

        if largest_spur_db.is_finite() {
            analysis.sfdr_db = Some(fund_db - largest_spur_db);
        }

        if !noise_db_bins.is_empty() {
            noise_db_bins.sort_by(|a, b| a.total_cmp(b));
            let mid = noise_db_bins.len() / 2;
            let noise_floor = if noise_db_bins.len() % 2 == 0 {
                0.5 * (noise_db_bins[mid - 1] + noise_db_bins[mid])
            } else {
                noise_db_bins[mid]
            };
            analysis.noise_floor_db = Some(noise_floor);
        }

        if noise_power_sum > 0.0 {
            let snr = 10.0 * (fund_power / noise_power_sum).log10();
            if snr.is_finite() {
                analysis.snr_db = Some(snr);
            }
        }

        let noise_and_distortion = noise_power_sum + harmonic_power_sum;
        if noise_and_distortion > 0.0 {
            let sinad = 10.0 * (fund_power / noise_and_distortion).log10();
            if sinad.is_finite() {
                analysis.sinad_db = Some(sinad);
            }
        }

        analysis
    }

    fn interpolate_peak(fft: &FftData, idx: usize) -> Option<(usize, f64, f64, f64)> {
        let point = fft.points.get(idx)?;
        let mut freq = point.frequency;
        let mut db = point.magnitude_db();
        if !db.is_finite() {
            return None;
        }

        if idx > 0 && idx + 1 < fft.points.len() {
            let left = fft.points[idx - 1].magnitude_db();
            let center = fft.points[idx].magnitude_db();
            let right = fft.points[idx + 1].magnitude_db();
            if left.is_finite() && center.is_finite() && right.is_finite() {
                let denom = left - 2.0 * center + right;
                if denom.abs() > 1e-12 {
                    let delta = (0.5 * (left - right) / denom).clamp(-0.5, 0.5);
                    let df = fft.frequency_resolution();
                    freq = point.frequency + delta * df;
                    db = center - 0.25 * (left - right) * delta;
                }
            }
        }

        let mag = if db.is_finite() {
            10.0_f64.powf(db / 20.0)
        } else {
            point.magnitude
        };
        if !mag.is_finite() || mag <= 0.0 {
            return None;
        }

        Some((idx, freq, mag, db))
    }

    fn find_harmonic_peak(fft: &FftData, target_freq: f64) -> Option<(usize, f64, f64, f64)> {
        if fft.points.len() < 3 {
            return None;
        }
        let df = fft.frequency_resolution();
        if df <= 0.0 || !df.is_finite() {
            return None;
        }

        let center = (target_freq / df).round() as isize;
        let search = 2isize;
        let min_idx = (center - search).max(1) as usize;
        let max_idx = (center + search).min((fft.points.len() - 1) as isize) as usize;
        if min_idx > max_idx {
            return None;
        }

        let mut best_idx = None;
        let mut best_mag = f64::NEG_INFINITY;
        for idx in min_idx..=max_idx {
            let mag = fft.points[idx].magnitude;
            if mag.is_finite() && mag > best_mag {
                best_mag = mag;
                best_idx = Some(idx);
            }
        }
        let idx = best_idx?;
        Self::interpolate_peak(fft, idx)
    }

    fn guard_bins(window: WindowFunction) -> usize {
        match window {
            WindowFunction::Rectangular => 1,
            WindowFunction::Hanning | WindowFunction::Hamming => 2,
            WindowFunction::Blackman | WindowFunction::Kaiser | WindowFunction::Gaussian => 3,
            WindowFunction::BlackmanHarris | WindowFunction::FlatTop => 4,
        }
    }

    fn exclude_bin_region(mask: &mut [bool], center: usize, radius: usize) {
        if mask.is_empty() {
            return;
        }
        let start = center.saturating_sub(radius);
        let end = (center + radius).min(mask.len() - 1);
        for idx in start..=end {
            mask[idx] = true;
        }
    }

    /// Format THD for display
    pub fn format_thd(&self) -> String {
        match self.thd_percent {
            Some(thd) if thd.is_finite() => format!("{:.3}%", thd),
            _ => "N/A".to_string(),
        }
    }

    /// Format SFDR for display
    pub fn format_sfdr(&self) -> String {
        match self.sfdr_db {
            Some(sfdr) if sfdr.is_finite() => format!("{:.1} dB", sfdr),
            _ => "N/A".to_string(),
        }
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

    // Generate test sine wave
    fn generate_sine(freq: f64, sample_rate: f64, n_samples: usize) -> Vec<f64> {
        (0..n_samples)
            .map(|i| (2.0 * PI * freq * i as f64 / sample_rate).sin())
            .collect()
    }

    // Generate sine with harmonics
    fn generate_distorted_sine(
        freq: f64,
        sample_rate: f64,
        n_samples: usize,
        harmonics: &[(usize, f64)],
    ) -> Vec<f64> {
        (0..n_samples)
            .map(|i| {
                let t = i as f64 / sample_rate;
                let mut val = (2.0 * PI * freq * t).sin();
                for &(h, amp) in harmonics {
                    val += amp * (2.0 * PI * freq * h as f64 * t).sin();
                }
                val
            })
            .collect()
    }

    // =========================================================================
    // FftPoint Tests
    // =========================================================================

    #[test]
    fn test_fft_point_new() {
        let p = FftPoint::new(1000.0, 0.5, 0.0);
        assert_eq!(p.frequency, 1000.0);
        assert_eq!(p.magnitude, 0.5);
    }

    #[test]
    fn test_fft_point_from_complex() {
        let p = FftPoint::from_complex(100.0, 3.0, 4.0);
        assert!(approx_eq(p.magnitude, 5.0));
    }

    #[test]
    fn test_magnitude_db() {
        let p = FftPoint::new(100.0, 1.0, 0.0);
        assert!(approx_eq(p.magnitude_db(), 0.0));

        let p2 = FftPoint::new(100.0, 10.0, 0.0);
        assert!(approx_eq(p2.magnitude_db(), 20.0));

        let p3 = FftPoint::new(100.0, 0.1, 0.0);
        assert!(approx_eq(p3.magnitude_db(), -20.0));
    }

    #[test]
    fn test_magnitude_db_zero() {
        let p = FftPoint::new(100.0, 0.0, 0.0);
        assert!(p.magnitude_db().is_infinite());
    }

    #[test]
    fn test_magnitude_dbm() {
        // 1Vrms into 50Ω = 20mW = 10*log10(20) = 13dBm
        let p = FftPoint::new(100.0, 1.0, 0.0);
        let dbm = p.magnitude_dbm(50.0);
        assert!(approx_eq_rel(dbm, 13.01, 0.1));
    }

    #[test]
    fn test_magnitude_dbm_invalid_impedance_falls_back_to_nominal() {
        let p = FftPoint::new(100.0, 1.0, 0.0);
        let nominal = p.magnitude_dbm(50.0);
        let invalid = p.magnitude_dbm(0.0);
        assert!(nominal.is_finite());
        assert!(invalid.is_finite());
        assert!(approx_eq_rel(invalid, nominal, 1e-9));
    }

    #[test]
    fn test_phase_deg() {
        let p = FftPoint::new(100.0, 1.0, PI / 2.0);
        assert!(approx_eq(p.phase_deg(), 90.0));
    }

    // =========================================================================
    // FftData Tests
    // =========================================================================

    #[test]
    fn test_fft_data_new() {
        let fft = FftData::new("Test");
        assert!(fft.is_empty());
        assert_eq!(fft.name, "Test");
    }

    #[test]
    fn test_fft_data_default() {
        let fft = FftData::default();
        assert!(fft.is_empty());
        assert_eq!(fft.window, WindowFunction::Hanning);
    }

    #[test]
    fn test_fft_from_sine() {
        let fs = 1000.0;
        let f_sig = 100.0;
        let data = generate_sine(f_sig, fs, 256);

        let fft = FftData::from_time_domain("Sine", &data, fs, WindowFunction::Rectangular);

        assert!(!fft.is_empty());
        assert_eq!(fft.sample_rate, fs);
        assert_eq!(fft.fft_size, 256);

        // Should find peak near 100 Hz
        let (_, peak) = fft.find_peak().unwrap();
        assert!((peak.frequency - f_sig).abs() < fft.frequency_resolution() * 2.0);
    }

    #[test]
    fn test_fft_frequency_resolution() {
        let fft = FftData {
            sample_rate: 1000.0,
            fft_size: 1024,
            ..Default::default()
        };
        assert!(approx_eq_rel(
            fft.frequency_resolution(),
            1000.0 / 1024.0,
            0.01
        ));
    }

    #[test]
    fn test_fft_nyquist() {
        let fft = FftData {
            sample_rate: 44100.0,
            ..Default::default()
        };
        assert!(approx_eq(fft.nyquist(), 22050.0));
    }

    #[test]
    fn test_fft_find_peak() {
        let fs = 1000.0;
        let data = generate_sine(250.0, fs, 128);
        let fft = FftData::from_time_domain("Test", &data, fs, WindowFunction::Hanning);

        let peak = fft.find_peak();
        assert!(peak.is_some());
        let (idx, p) = peak.unwrap();
        assert!(idx > 0); // Not DC
        assert!(p.magnitude > 0.1);
    }

    #[test]
    fn test_fft_find_peak_ignores_non_finite_bins() {
        let fft = FftData {
            name: "Test".to_string(),
            points: vec![
                FftPoint::new(0.0, 0.1, 0.0), // DC
                FftPoint::new(100.0, f64::NAN, 0.0),
                FftPoint::new(200.0, f64::INFINITY, 0.0),
                FftPoint::new(300.0, 2.0, 0.0),
                FftPoint::new(400.0, 1.5, 0.0),
            ],
            sample_rate: 1000.0,
            fft_size: 8,
            window: WindowFunction::Rectangular,
        };

        let peak = fft.find_peak().expect("peak should be found");
        assert_eq!(peak.0, 3);
        assert!(approx_eq(peak.1.frequency, 300.0));
    }

    #[test]
    fn test_fft_find_peak_all_non_finite_returns_none() {
        let fft = FftData {
            name: "Test".to_string(),
            points: vec![
                FftPoint::new(0.0, 0.1, 0.0), // DC
                FftPoint::new(100.0, f64::NAN, 0.0),
                FftPoint::new(200.0, f64::INFINITY, 0.0),
                FftPoint::new(300.0, f64::NEG_INFINITY, 0.0),
            ],
            sample_rate: 1000.0,
            fft_size: 8,
            window: WindowFunction::Rectangular,
        };

        assert!(fft.find_peak().is_none());
    }

    #[test]
    fn test_fft_find_peaks() {
        let fs = 1000.0;
        let data: Vec<f64> = (0..256)
            .map(|i| {
                let t = i as f64 / fs;
                (2.0 * PI * 100.0 * t).sin() + 0.5 * (2.0 * PI * 200.0 * t).sin()
            })
            .collect();

        let fft = FftData::from_time_domain("Test", &data, fs, WindowFunction::Rectangular);
        let peaks = fft.find_peaks(-40.0);

        // Should find at least 2 peaks
        assert!(peaks.len() >= 2);
    }

    #[test]
    fn test_fft_interpolate() {
        let fs = 1000.0;
        let data = generate_sine(100.0, fs, 256);
        let fft = FftData::from_time_domain("Test", &data, fs, WindowFunction::Hanning);

        // Interpolate at exact bin frequency
        let p = fft.interpolate(100.0);
        assert!(p.is_some());
    }

    #[test]
    fn test_fft_dc_component() {
        let data = vec![1.0, 1.0, 1.0, 1.0]; // DC only
        let fft = FftData::from_time_domain("DC", &data, 1.0, WindowFunction::Rectangular);

        let dc = fft.dc().unwrap();
        assert!(dc.magnitude > 0.5); // Should have significant DC
    }

    #[test]
    fn test_fft_from_spectrum() {
        let freqs = vec![0.0, 100.0, 200.0, 300.0];
        let mags = vec![0.0, 1.0, 0.5, 0.1];
        let phases = vec![0.0, 0.0, 0.0, 0.0];

        let fft = FftData::from_spectrum("Test", &freqs, &mags, &phases, 1000.0);

        assert_eq!(fft.len(), 4);
        assert_eq!(fft.points[1].magnitude, 1.0);
    }

    #[test]
    fn test_fft_from_spectrum_empty_inputs() {
        let fft = FftData::from_spectrum("Empty", &[], &[], &[], 1000.0);
        assert!(fft.is_empty());
        assert_eq!(fft.fft_size, 0);
    }

    #[test]
    fn test_fft_from_spectrum_mismatched_lengths_uses_shortest_input() {
        let freqs = vec![0.0, 100.0, 200.0, 300.0];
        let mags = vec![0.0, 1.0];
        let phases = vec![0.0, 0.25];

        let fft = FftData::from_spectrum("Short", &freqs, &mags, &phases, 1000.0);

        assert_eq!(fft.len(), 2);
        assert_eq!(fft.fft_size, 2);
        assert_eq!(fft.points[0].frequency, 0.0);
        assert_eq!(fft.points[1].frequency, 100.0);
    }

    // =========================================================================
    // SpectrumAnalysis Tests
    // =========================================================================

    #[test]
    fn test_analysis_pure_sine() {
        let fs = 8000.0;
        let data = generate_sine(1000.0, fs, 1024);
        let fft = FftData::from_time_domain("Sine", &data, fs, WindowFunction::Hanning);

        let analysis = SpectrumAnalysis::analyze(&fft, 10);

        assert!(analysis.fundamental_frequency.is_some());
        // Pure sine should have very low THD
        if let Some(thd) = analysis.thd_percent {
            assert!(thd < 1.0); // Less than 1% THD
        }
    }

    #[test]
    fn test_analysis_distorted_sine() {
        let fs = 8000.0;
        let harmonics = vec![(2, 0.1), (3, 0.05)]; // 10% 2nd, 5% 3rd harmonic
        let data = generate_distorted_sine(1000.0, fs, 1024, &harmonics);
        let fft = FftData::from_time_domain("Distorted", &data, fs, WindowFunction::Hanning);

        let analysis = SpectrumAnalysis::analyze(&fft, 10);

        // Should detect harmonics
        assert!(!analysis.harmonics.is_empty());

        // THD should be roughly sqrt(0.1^2 + 0.05^2) ≈ 11.2%
        if let Some(thd) = analysis.thd_percent {
            assert!(thd > 5.0 && thd < 20.0);
        }
    }

    #[test]
    fn test_analysis_sfdr() {
        let fs = 8000.0;
        let data: Vec<f64> = (0..1024)
            .map(|i| {
                let t = i as f64 / fs;
                (2.0 * PI * 1000.0 * t).sin() + 0.01 * (2.0 * PI * 1500.0 * t).sin()
            })
            .collect();

        let fft = FftData::from_time_domain("Test", &data, fs, WindowFunction::Hanning);
        let analysis = SpectrumAnalysis::analyze(&fft, 10);

        // SFDR should be roughly 40dB (1.0 vs 0.01)
        if let Some(sfdr) = analysis.sfdr_db {
            assert!(sfdr > 30.0 && sfdr < 50.0);
        }
    }

    #[test]
    fn test_analysis_sfdr_includes_harmonic_spur() {
        let fs = 20_000.0;
        let data: Vec<f64> = (0..4096)
            .map(|i| {
                let t = i as f64 / fs;
                (2.0 * PI * 1_000.0 * t).sin()
                    + 0.2 * (2.0 * PI * 2_000.0 * t).sin()
                    + 0.01 * (2.0 * PI * 3_700.0 * t).sin()
            })
            .collect();

        let fft = FftData::from_time_domain("SFDR-Harmonic", &data, fs, WindowFunction::Hanning);
        let analysis = SpectrumAnalysis::analyze(&fft, 10);

        // Largest spur is the 2nd harmonic at about -14 dBc.
        let sfdr = analysis.sfdr_db.expect("expected SFDR");
        assert!(sfdr > 10.0 && sfdr < 20.0);
    }

    #[test]
    fn test_analysis_format() {
        let mut analysis = SpectrumAnalysis::default();
        analysis.thd_percent = Some(1.234);
        analysis.sfdr_db = Some(60.5);

        assert!(analysis.format_thd().contains("1.234"));
        assert!(analysis.format_sfdr().contains("60.5"));
    }

    #[test]
    fn test_analysis_empty() {
        let fft = FftData::new("Empty");
        let analysis = SpectrumAnalysis::analyze(&fft, 10);

        assert!(analysis.fundamental_frequency.is_none());
        assert!(analysis.thd_percent.is_none());
    }

    #[test]
    fn test_analysis_harmonics_beyond_nyquist() {
        // Low sample rate so harmonics exceed Nyquist
        let fs = 1000.0;
        let data = generate_sine(400.0, fs, 256);
        let fft = FftData::from_time_domain("Test", &data, fs, WindowFunction::Hanning);

        let analysis = SpectrumAnalysis::analyze(&fft, 10);

        // Only 1 harmonic (2nd at 800Hz) should be below Nyquist (500Hz)
        // Actually 2*400=800 > 500, so no harmonics should be detected
        assert!(analysis.harmonics.is_empty() || analysis.harmonics.len() <= 1);
    }

    #[test]
    fn test_analysis_ignores_non_finite_bins_for_noise_floor() {
        let fft = FftData {
            name: "Test".to_string(),
            points: vec![
                FftPoint::new(0.0, 0.0, 0.0), // DC
                FftPoint::new(100.0, 1.0, 0.0),
                FftPoint::new(200.0, 0.01, 0.0),
                FftPoint::new(300.0, 0.02, 0.0),
                FftPoint::new(400.0, f64::NAN, 0.0),
                FftPoint::new(500.0, f64::INFINITY, 0.0),
                FftPoint::new(600.0, 0.03, 0.0),
                FftPoint::new(700.0, 0.015, 0.0),
                FftPoint::new(800.0, 0.005, 0.0),
            ],
            sample_rate: 2000.0,
            fft_size: 16,
            window: WindowFunction::Rectangular,
        };

        let analysis = SpectrumAnalysis::analyze(&fft, 5);
        assert_eq!(analysis.fundamental_frequency, Some(100.0));
        assert!(analysis
            .noise_floor_db
            .map(|db| db.is_finite())
            .unwrap_or(false));
        assert!(analysis.snr_db.map(|db| db.is_finite()).unwrap_or(false));
    }

    #[test]
    fn test_analysis_reports_sinad_for_distorted_signal() {
        let fs = 20_000.0;
        let data: Vec<f64> = (0..4096)
            .map(|i| {
                let t = i as f64 / fs;
                (2.0 * PI * 1000.0 * t).sin()
                    + 0.1 * (2.0 * PI * 2000.0 * t).sin()
                    + 0.01 * (2.0 * PI * 3500.0 * t).sin()
            })
            .collect();
        let fft = FftData::from_time_domain("SINAD", &data, fs, WindowFunction::Blackman);
        let analysis = SpectrumAnalysis::analyze(&fft, 10);
        assert!(analysis.sinad_db.map(|v| v.is_finite()).unwrap_or(false));
    }

    #[test]
    fn test_fft_from_time_domain_handles_odd_length() {
        let fs = 1000.0;
        let n = 255usize;
        let f_sig = 117.0;
        let data: Vec<f64> = (0..n)
            .map(|i| {
                let t = i as f64 / fs;
                (2.0 * PI * f_sig * t).sin()
            })
            .collect();
        let fft = FftData::from_time_domain("Odd", &data, fs, WindowFunction::Hanning);
        assert_eq!(fft.len(), n / 2 + 1);
        let (_, peak) = fft.find_peak().expect("peak exists");
        assert!((peak.frequency - f_sig).abs() < 2.0 * fft.frequency_resolution());
    }
}
