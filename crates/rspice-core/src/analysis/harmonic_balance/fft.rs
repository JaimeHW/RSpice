//! FFT/IFFT Wrappers for Harmonic Balance
//!
//! Provides efficient FFT operations using rustfft for converting between
//! time-domain waveforms and frequency-domain spectral coefficients.

#![allow(clippy::needless_range_loop)]
use num_complex::Complex64;
use rustfft::{Fft, FftDirection, FftPlanner};
use std::sync::Arc;

use crate::Value;

/// FFT/IFFT processor for Harmonic Balance
///
/// Encapsulates rustfft planner and provides convenient methods for
/// converting between time and frequency domains with proper normalization.
pub struct HbFft {
    /// FFT/collocation-grid size
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
    ///
    /// The oversampling factor is floored at 2: below that the top
    /// harmonics collide with the conjugate half of the spectrum and would
    /// be silently dropped from the solution representation.
    pub fn new(num_harmonics: usize, oversample: usize) -> Self {
        let min_size = (num_harmonics + 1) * oversample.max(2);
        let fft_size = min_size.next_power_of_two();

        Self::with_size(num_harmonics, fft_size)
    }

    /// Create a processor with an exact collocation-grid size.
    pub fn with_size(num_harmonics: usize, fft_size: usize) -> Self {
        let minimum_size = num_harmonics
            .checked_mul(2)
            .and_then(|value| value.checked_add(1))
            .expect("HB harmonic count exceeds the addressable collocation grid");
        assert!(
            fft_size >= minimum_size,
            "HB collocation grid must represent DC and ± every configured harmonic"
        );

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

    #[inline]
    fn positive_harmonic_limit(&self) -> usize {
        (self.fft_size - 1) / 2
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
        let positive_limit = self.positive_harmonic_limit();
        for (k, &coeff) in spectrum.iter().enumerate().skip(1).take(positive_limit) {
            full_spectrum[k] = coeff;
        }

        // Negative frequencies (conjugate symmetry for real output)
        for k in 1..=positive_limit {
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

    /// Convert time-domain waveform to spectral coefficients, keeping
    /// `count + 1` entries (DC through harmonic `count`, capped at the
    /// alias-free half of the grid).
    ///
    /// The Newton Jacobian's conjugate (Hankel) coupling needs conductance
    /// harmonics up to 2H, beyond the solution representation's H.
    pub fn to_frequency_domain_n(&mut self, waveform: &[Value], count: usize) -> Vec<Complex64> {
        let n = self.fft_size;
        let mut buffer: Vec<Complex64> = waveform
            .iter()
            .take(n)
            .map(|&x| Complex64::new(x, 0.0))
            .collect();
        buffer.resize(n, Complex64::new(0.0, 0.0));
        self.fft
            .process_with_scratch(&mut buffer, &mut self.scratch);

        let norm = 1.0 / n as f64;
        let max_kept = self.positive_harmonic_limit();
        (0..=count.min(max_kept))
            .map(|k| buffer[k] * norm)
            .collect()
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
        Self::with_size(self.num_harmonics, self.fft_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_minimal_grid_round_trips_the_highest_harmonic() {
        let mut fft = HbFft::with_size(5, 11);
        let mut spectrum = vec![Complex64::new(0.0, 0.0); 6];
        spectrum[0] = Complex64::new(0.75, 0.0);
        spectrum[5] = Complex64::new(-0.2, 0.35);

        let waveform = fft.to_time_domain(&spectrum);
        let round_trip = fft.to_frequency_domain(&waveform);
        assert_eq!(waveform.len(), 11);
        for (expected, actual) in spectrum.iter().zip(&round_trip) {
            assert!(
                (*expected - *actual).norm() < 1.0e-12,
                "expected {expected:?}, got {actual:?}"
            );
        }
    }
}
