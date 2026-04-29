use rustfft::num_complex::Complex;

use super::{
    FftPoint, SpectrumNormalization,
    cache::{cached_fft_plan, cached_window},
};
use crate::analysis::fft::window::WindowFunction;
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
    /// Amplitude normalization used for `points`.
    pub normalization: SpectrumNormalization,
}

impl Default for FftData {
    fn default() -> Self {
        Self {
            name: String::new(),
            points: Vec::new(),
            sample_rate: 1.0,
            fft_size: 0,
            window: WindowFunction::Hanning,
            normalization: SpectrumNormalization::Peak,
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
        Self::from_time_domain_with_normalization(
            name,
            data,
            sample_rate,
            window,
            SpectrumNormalization::Peak,
        )
    }

    /// Create from uniformly sampled time-domain data using FFT.
    pub fn from_time_domain_with_normalization(
        name: &str,
        data: &[f64],
        sample_rate: f64,
        window: WindowFunction,
        normalization: SpectrumNormalization,
    ) -> Self {
        let n = data.len();
        if n == 0 || !sample_rate.is_finite() || sample_rate <= 0.0 {
            return Self::new(name);
        }

        let window_entry = cached_window(window, n);

        // Coherent gain from actual generated coefficients, not a table constant.
        let cg = window_entry.coherent_gain;
        if !cg.is_finite() || cg.abs() < 1e-15 {
            return Self::new(name);
        }

        let mut buffer: Vec<Complex<f64>> = data
            .iter()
            .zip(window_entry.coefficients.iter())
            .map(|(&sample, &coeff)| Complex::new(sample * coeff, 0.0))
            .collect();
        let fft = cached_fft_plan(n);
        fft.process(&mut buffer);

        // One-sided spectrum: include DC..Nyquist (Nyquist exists only for even n).
        // AC bins are amplitude-calibrated to peak values for FFT plot compatibility.
        let n_freqs = n / 2 + 1;
        let mut points = Vec::with_capacity(n_freqs);
        let base_scale = (1.0 / (n as f64 * cg)) * normalization.scale_from_peak();
        let has_nyquist = crate::utils::numeric::is_multiple_of(n, 2);

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
            normalization,
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
        Self::from_spectrum_with_normalization(
            name,
            frequencies,
            magnitudes,
            phases,
            sample_rate,
            SpectrumNormalization::Peak,
        )
    }

    /// Create from magnitude/phase arrays with explicit normalization metadata.
    pub fn from_spectrum_with_normalization(
        name: &str,
        frequencies: &[f64],
        magnitudes: &[f64],
        phases: &[f64],
        sample_rate: f64,
        normalization: SpectrumNormalization,
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
            normalization,
        }
    }

    /// Convert all spectrum magnitudes to a different normalization mode.
    pub fn convert_normalization(&mut self, target: SpectrumNormalization) {
        if self.normalization == target {
            return;
        }
        let scale = SpectrumNormalization::relative_scale(self.normalization, target);
        for point in &mut self.points {
            point.magnitude *= scale;
        }
        self.normalization = target;
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
        let mut min_db = f64::INFINITY;
        let mut max_db = f64::NEG_INFINITY;
        let mut has_finite = false;

        for point in &self.points {
            let db = point.magnitude_db();
            if !db.is_finite() {
                continue;
            }
            has_finite = true;
            min_db = min_db.min(db);
            max_db = max_db.max(db);
        }

        if !has_finite {
            return None;
        }
        Some((min_db, max_db))
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

    /// Find peak bin indices above threshold.
    pub fn find_peak_indices(&self, threshold_db: f64) -> Vec<usize> {
        let mut peaks = Vec::new();

        for i in 1..self.points.len().saturating_sub(1) {
            let prev = self.points[i - 1].magnitude;
            let curr = self.points[i].magnitude;
            let next = self.points[i + 1].magnitude;

            if curr > prev && curr > next && self.points[i].magnitude_db() > threshold_db {
                peaks.push(i);
            }
        }

        peaks
    }

    /// Find all peaks above threshold
    pub fn find_peaks(&self, threshold_db: f64) -> Vec<(usize, &FftPoint)> {
        self.find_peak_indices(threshold_db)
            .into_iter()
            .map(|index| (index, &self.points[index]))
            .collect()
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
