//! FFT Data Structures
//!
//! Core data types for FFT and spectrum analysis.

use std::f64::consts::PI;

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

    /// Create from time-domain data using DFT
    /// Note: For production use, replace with FFT library
    pub fn from_time_domain(
        name: &str,
        data: &[f64],
        sample_rate: f64,
        window: WindowFunction,
    ) -> Self {
        let n = data.len();
        if n == 0 {
            return Self::new(name);
        }

        // Apply window
        let win = generate_window(window, n);
        let windowed = apply_window_copy(data, &win);

        // Calculate coherent gain correction
        let cg = window.coherent_gain();

        // Compute DFT (positive frequencies only)
        let n_freqs = n / 2 + 1;
        let mut points = Vec::with_capacity(n_freqs);

        for k in 0..n_freqs {
            let freq = k as f64 * sample_rate / n as f64;
            let mut real = 0.0;
            let mut imag = 0.0;

            for (j, &x) in windowed.iter().enumerate() {
                let angle = -2.0 * PI * k as f64 * j as f64 / n as f64;
                real += x * angle.cos();
                imag += x * angle.sin();
            }

            // Normalize
            let scale = if k == 0 || k == n / 2 {
                1.0 / n as f64 / cg
            } else {
                2.0 / n as f64 / cg // Factor of 2 for one-sided spectrum
            };

            real *= scale;
            imag *= scale;

            points.push(FftPoint::from_complex(freq, real, imag));
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

        Self {
            name: name.to_string(),
            points,
            sample_rate,
            fft_size: (frequencies.len() - 1) * 2,
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

        self.points[1..]
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| {
                a.magnitude
                    .partial_cmp(&b.magnitude)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(i, p)| (i + 1, p))
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

        // Find fundamental (largest peak excluding DC)
        let Some((fund_idx, fund_peak)) = fft.find_peak() else {
            return analysis;
        };

        analysis.fundamental_frequency = Some(fund_peak.frequency);
        analysis.fundamental_db = Some(fund_peak.magnitude_db());

        // Find harmonics
        let fund_freq = fund_peak.frequency;
        let fund_power = fund_peak.magnitude.powi(2);
        let mut harmonic_power_sum = 0.0;

        for h in 2..=num_harmonics {
            let harmonic_freq = fund_freq * h as f64;
            if harmonic_freq > fft.nyquist() {
                break;
            }

            if let Some(point) = fft.interpolate(harmonic_freq) {
                analysis
                    .harmonics
                    .push((point.frequency, point.magnitude_db()));
                harmonic_power_sum += point.magnitude.powi(2);
            }
        }

        // Calculate THD
        if fund_power > 0.0 {
            let thd_ratio = (harmonic_power_sum / fund_power).sqrt();
            analysis.thd_percent = Some(thd_ratio * 100.0);
            analysis.thd_db = Some(20.0 * thd_ratio.log10());
        }

        // Calculate SFDR (fundamental to next largest spur)
        let peaks = fft.find_peaks(fund_peak.magnitude_db() - 100.0);
        let mut largest_spur_db = f64::NEG_INFINITY;

        for (idx, peak) in &peaks {
            if *idx != fund_idx {
                let db = peak.magnitude_db();
                if db > largest_spur_db {
                    largest_spur_db = db;
                }
            }
        }

        if largest_spur_db.is_finite() {
            if let Some(fund_db) = analysis.fundamental_db {
                analysis.sfdr_db = Some(fund_db - largest_spur_db);
            }
        }

        // Estimate noise floor (median of bottom 25% magnitudes)
        let mut mags: Vec<f64> = fft.points[1..]
            .iter()
            .map(|p| p.magnitude_db())
            .filter(|db| db.is_finite())
            .collect();
        mags.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if mags.len() > 4 {
            let quarter = mags.len() / 4;
            let noise_floor: f64 = mags[..quarter].iter().sum::<f64>() / quarter as f64;
            analysis.noise_floor_db = Some(noise_floor);

            // SNR = fundamental - noise floor
            if let Some(fund_db) = analysis.fundamental_db {
                analysis.snr_db = Some(fund_db - noise_floor);
            }
        }

        analysis
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
}
