//! FFT/IFFT Wrappers for Harmonic Balance
//!
//! Provides efficient FFT operations using rustfft for converting between
//! time-domain waveforms and frequency-domain spectral coefficients.

use num_complex::Complex64;
use rustfft::{Fft, FftDirection, FftPlanner};
use std::sync::Arc;

use crate::Value;

/// FFT/IFFT processor for Harmonic Balance
///
/// Encapsulates rustfft planner and provides convenient methods for
/// converting between time and frequency domains with proper normalization.
pub struct HbFft {
    /// FFT size (power of 2)
    fft_size: usize,

    /// Number of harmonics (not including DC)
    num_harmonics: usize,

    /// Forward FFT (time to frequency)
    fft: Arc<dyn Fft<f64>>,

    /// Inverse FFT (frequency to time)
    ifft: Arc<dyn Fft<f64>>,

    /// Scratch buffer for FFT operations
    scratch: Vec<Complex64>,
}

impl std::fmt::Debug for HbFft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HbFft")
            .field("fft_size", &self.fft_size)
            .field("num_harmonics", &self.num_harmonics)
            .finish_non_exhaustive()
    }
}

impl HbFft {
    /// Create a new FFT processor
    ///
    /// # Arguments
    /// * `num_harmonics` - Number of harmonics (not including DC)
    /// * `oversample` - Oversampling factor (typically 2 or 4)
    pub fn new(num_harmonics: usize, oversample: usize) -> Self {
        let min_size = (num_harmonics + 1) * oversample;
        let fft_size = min_size.next_power_of_two();

        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft(fft_size, FftDirection::Forward);
        let ifft = planner.plan_fft(fft_size, FftDirection::Inverse);

        let scratch_len = fft
            .get_inplace_scratch_len()
            .max(ifft.get_inplace_scratch_len());

        Self {
            fft_size,
            num_harmonics,
            fft,
            ifft,
            scratch: vec![Complex64::new(0.0, 0.0); scratch_len],
        }
    }

    /// Get FFT size
    pub fn size(&self) -> usize {
        self.fft_size
    }

    /// Get number of harmonics
    pub fn num_harmonics(&self) -> usize {
        self.num_harmonics
    }

    /// Convert spectral coefficients to time-domain waveform
    ///
    /// # Arguments
    /// * `spectrum` - Complex spectral coefficients [DC, H1, H2, ..., Hn]
    ///
    /// # Returns
    /// Time-domain waveform samples
    pub fn to_time_domain(&mut self, spectrum: &[Complex64]) -> Vec<Value> {
        let n = self.fft_size;

        // Build full spectrum with negative frequencies (conjugate symmetry for real signal)
        let mut full_spectrum = vec![Complex64::new(0.0, 0.0); n];

        // DC component
        if !spectrum.is_empty() {
            full_spectrum[0] = spectrum[0];
        }

        // Positive frequencies
        for (k, &coeff) in spectrum.iter().skip(1).enumerate() {
            if k + 1 < n / 2 {
                full_spectrum[k + 1] = coeff;
            }
        }

        // Negative frequencies (conjugate symmetry for real output)
        for k in 1..n / 2 {
            if k < spectrum.len() {
                full_spectrum[n - k] = spectrum[k].conj();
            }
        }

        // Perform IFFT
        self.ifft
            .process_with_scratch(&mut full_spectrum, &mut self.scratch);

        // Extract real part and normalize
        full_spectrum.iter().map(|c| c.re).collect()
    }

    /// Convert time-domain waveform to spectral coefficients
    ///
    /// # Arguments
    /// * `waveform` - Time-domain samples
    ///
    /// # Returns
    /// Complex spectral coefficients [DC, H1, H2, ..., Hn]
    pub fn to_frequency_domain(&mut self, waveform: &[Value]) -> Vec<Complex64> {
        let n = self.fft_size;

        // Prepare complex buffer
        let mut buffer: Vec<Complex64> = waveform
            .iter()
            .take(n)
            .map(|&x| Complex64::new(x, 0.0))
            .collect();

        // Zero-pad if necessary
        buffer.resize(n, Complex64::new(0.0, 0.0));

        // Perform FFT
        self.fft
            .process_with_scratch(&mut buffer, &mut self.scratch);

        // Extract harmonics and normalize
        let norm = 1.0 / n as f64;
        let mut spectrum = Vec::with_capacity(self.num_harmonics + 1);

        for k in 0..=self.num_harmonics {
            if k < n {
                spectrum.push(buffer[k] * norm);
            } else {
                spectrum.push(Complex64::new(0.0, 0.0));
            }
        }

        spectrum
    }

    /// Compute derivative spectrum (multiply by jω)
    ///
    /// For a signal x(t), if X(ω) is its spectrum, then dx/dt has spectrum jωX(ω)
    ///
    /// # Arguments
    /// * `spectrum` - Input spectral coefficients
    /// * `fundamental_freq` - Fundamental frequency in Hz
    pub fn derivative_spectrum(
        &self,
        spectrum: &[Complex64],
        fundamental_freq: Value,
    ) -> Vec<Complex64> {
        use std::f64::consts::PI;

        spectrum
            .iter()
            .enumerate()
            .map(|(k, &coeff)| {
                let omega = 2.0 * PI * (k as f64) * fundamental_freq;
                Complex64::new(0.0, omega) * coeff
            })
            .collect()
    }

    /// Compute integral spectrum (divide by jω)
    ///
    /// For a signal x(t), if X(ω) is its spectrum, then ∫x dt has spectrum X(ω)/(jω)
    /// Note: DC component remains unchanged
    ///
    /// # Arguments
    /// * `spectrum` - Input spectral coefficients
    /// * `fundamental_freq` - Fundamental frequency in Hz
    pub fn integral_spectrum(
        &self,
        spectrum: &[Complex64],
        fundamental_freq: Value,
    ) -> Vec<Complex64> {
        use std::f64::consts::PI;

        spectrum
            .iter()
            .enumerate()
            .map(|(k, &coeff)| {
                if k == 0 {
                    coeff // DC unchanged
                } else {
                    let omega = 2.0 * PI * (k as f64) * fundamental_freq;
                    coeff / Complex64::new(0.0, omega)
                }
            })
            .collect()
    }

    /// Compute spectral power (|X|²) for each harmonic
    pub fn spectral_power(&self, spectrum: &[Complex64]) -> Vec<Value> {
        spectrum.iter().map(|c| c.norm_sqr()).collect()
    }

    /// Compute total signal power via Parseval's theorem
    pub fn total_power(&self, spectrum: &[Complex64]) -> Value {
        // For real signal: P = |X₀|² + 2*Σ|Xₖ|² for k > 0
        let dc_power = spectrum.first().map(|c| c.norm_sqr()).unwrap_or(0.0);
        let harmonic_power: Value = spectrum.iter().skip(1).map(|c| c.norm_sqr()).sum();
        dc_power + 2.0 * harmonic_power
    }

    /// Generate time points for the FFT grid
    pub fn time_points(&self, period: Value) -> Vec<Value> {
        let n = self.fft_size;
        (0..n).map(|i| (i as f64) * period / (n as f64)).collect()
    }
}

impl Clone for HbFft {
    fn clone(&self) -> Self {
        Self::new(self.num_harmonics, self.fft_size / (self.num_harmonics + 1))
    }
}

#[cfg(test)]
mod fft_tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_fft_size_power_of_two() {
        let fft = HbFft::new(9, 2);
        assert!(fft.size().is_power_of_two());
        assert!(fft.size() >= 20);
    }

    #[test]
    fn test_roundtrip_dc() {
        let mut fft = HbFft::new(5, 2);

        // DC only
        let spectrum = vec![Complex64::new(1.5, 0.0)];
        let time = fft.to_time_domain(&spectrum);
        let recovered = fft.to_frequency_domain(&time);

        assert!(
            (recovered[0].re - 1.5).abs() < 1e-10,
            "DC roundtrip failed: got {}, expected 1.5",
            recovered[0].re
        );
    }

    #[test]
    fn test_roundtrip_sine() {
        let mut fft = HbFft::new(5, 4);
        let n = fft.size();

        // Generate a pure sine wave at fundamental frequency
        let amplitude = 2.0;
        let waveform: Vec<f64> = (0..n)
            .map(|i| amplitude * (2.0 * PI * (i as f64) / (n as f64)).sin())
            .collect();

        let spectrum = fft.to_frequency_domain(&waveform);

        // Fundamental should have amplitude/2 (one-sided)
        let h1_magnitude = spectrum[1].norm();
        assert!(
            (h1_magnitude - amplitude / 2.0).abs() < 0.1,
            "H1 magnitude: got {}, expected {}",
            h1_magnitude,
            amplitude / 2.0
        );

        // DC should be near zero
        assert!(
            spectrum[0].norm() < 0.01,
            "DC should be zero for sine: {}",
            spectrum[0].norm()
        );
    }

    #[test]
    fn test_roundtrip_cosine() {
        let mut fft = HbFft::new(5, 4);
        let n = fft.size();

        // Generate a pure cosine wave
        let amplitude = 3.0;
        let waveform: Vec<f64> = (0..n)
            .map(|i| amplitude * (2.0 * PI * (i as f64) / (n as f64)).cos())
            .collect();

        let spectrum = fft.to_frequency_domain(&waveform);
        let recovered = fft.to_time_domain(&spectrum);

        // Check waveform reconstruction
        for (i, (&orig, &rec)) in waveform.iter().zip(recovered.iter()).enumerate() {
            assert!(
                (orig - rec).abs() < 0.1,
                "Mismatch at {}: orig={}, rec={}",
                i,
                orig,
                rec
            );
        }
    }

    #[test]
    fn test_derivative_spectrum() {
        let fft = HbFft::new(3, 2);
        let f0 = 1e6; // 1 MHz

        // cos(2πf₀t) -> -2πf₀ sin(2πf₀t)
        // Spectrum of cos: [0, 0.5, 0, 0]
        // Derivative spectrum should be: [0, j*2πf₀*0.5, 0, 0]
        let spectrum = vec![
            Complex64::new(0.0, 0.0),
            Complex64::new(0.5, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
        ];

        let deriv = fft.derivative_spectrum(&spectrum, f0);

        // DC derivative should be 0
        assert!(deriv[0].norm() < 1e-10);

        // H1 should have phase shift of 90° (imaginary)
        let expected_h1 = 2.0 * PI * f0 * 0.5;
        assert!(
            (deriv[1].im - expected_h1).abs() < 1e-3,
            "H1 derivative imag: got {}, expected {}",
            deriv[1].im,
            expected_h1
        );
    }

    #[test]
    fn test_parseval_theorem() {
        let mut fft = HbFft::new(10, 4);
        let n = fft.size();

        // Generate a test waveform
        let waveform: Vec<f64> = (0..n)
            .map(|i| {
                let t = i as f64 / n as f64;
                1.0 + 2.0 * (2.0 * PI * t).cos() + 0.5 * (4.0 * PI * t).sin()
            })
            .collect();

        // Time domain power
        let time_power: f64 = waveform.iter().map(|x| x * x).sum::<f64>() / n as f64;

        // Frequency domain power
        let spectrum = fft.to_frequency_domain(&waveform);
        let freq_power = fft.total_power(&spectrum);

        assert!(
            (time_power - freq_power).abs() < 0.1,
            "Parseval: time_power={}, freq_power={}",
            time_power,
            freq_power
        );
    }

    #[test]
    fn test_time_points() {
        let fft = HbFft::new(5, 2);
        let period = 1e-9;
        let times = fft.time_points(period);

        assert_eq!(times.len(), fft.size());
        assert!((times[0] - 0.0).abs() < 1e-20);
        assert!(times.last().unwrap() < &period);
    }

    #[test]
    fn test_clone() {
        let fft1 = HbFft::new(7, 2);
        let fft2 = fft1.clone();
        assert_eq!(fft1.size(), fft2.size());
        assert_eq!(fft1.num_harmonics(), fft2.num_harmonics());
    }
}
