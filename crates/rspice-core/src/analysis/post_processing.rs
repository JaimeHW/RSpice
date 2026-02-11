//! Post-Processing Utilities Module
//!
//! Provides commercial-grade post-processing functions for simulation results:
//!
//! - **THD**: Total Harmonic Distortion calculation
//! - **SFDR**: Spurious-Free Dynamic Range
//! - **IMD**: Intermodulation Distortion (IP2, IP3)
//! - **Group Delay**: Phase derivative vs frequency
//! - **Bode Analysis**: Gain/phase extraction with margin markers
//! - **SNR**: Signal-to-Noise Ratio calculations
//!
//! These utilities process raw waveform or frequency-domain data to extract
//! standardized performance metrics used in analog/RF design.

use crate::Value;
use num_complex::Complex64;
use std::f64::consts::PI;

//=============================================================================
// THD (Total Harmonic Distortion)
//=============================================================================

/// Total Harmonic Distortion analysis result
#[derive(Debug, Clone, Default)]
pub struct ThdResult {
    /// Fundamental frequency (Hz)
    pub fundamental_freq: Value,

    /// Fundamental amplitude (linear)
    pub fundamental_amplitude: Value,

    /// THD as ratio (0-1)
    pub thd_ratio: Value,

    /// THD in percent
    pub thd_percent: Value,

    /// THD in dB
    pub thd_db: Value,

    /// Individual harmonic amplitudes (index 0 = fundamental, 1 = 2nd, etc.)
    pub harmonics: Vec<Value>,

    /// Number of harmonics included
    pub num_harmonics: usize,

    /// THD+N (distortion plus noise)
    pub thd_plus_noise: Option<Value>,
}

impl ThdResult {
    /// Calculate THD from harmonic amplitudes
    ///
    /// # Arguments
    /// * `harmonics` - Vector of harmonic amplitudes [fundamental, 2nd, 3rd, ...]
    /// * `fundamental_freq` - Frequency of the fundamental
    pub fn from_harmonics(harmonics: &[Value], fundamental_freq: Value) -> Self {
        if harmonics.is_empty() {
            return Self::default();
        }

        let fundamental = harmonics[0];
        if fundamental <= 0.0 {
            return Self::default();
        }

        // Sum of squares of harmonics (excluding fundamental)
        let harmonic_power: Value = harmonics.iter().skip(1).map(|h| h * h).sum();
        let thd_ratio = (harmonic_power / (fundamental * fundamental)).sqrt();

        Self {
            fundamental_freq,
            fundamental_amplitude: fundamental,
            thd_ratio,
            thd_percent: thd_ratio * 100.0,
            thd_db: 20.0 * thd_ratio.log10(),
            harmonics: harmonics.to_vec(),
            num_harmonics: harmonics.len(),
            thd_plus_noise: None,
        }
    }

    /// Calculate THD from time-domain waveform using FFT
    pub fn from_waveform(samples: &[Value], sample_rate: Value, fundamental_freq: Value) -> Self {
        let n = samples.len();
        if n < 4 {
            return Self::default();
        }

        // Simple DFT for harmonic extraction
        let harmonics = Self::extract_harmonics(samples, sample_rate, fundamental_freq, 10);
        Self::from_harmonics(&harmonics, fundamental_freq)
    }

    /// Extract harmonic amplitudes using DFT
    fn extract_harmonics(
        samples: &[Value],
        sample_rate: Value,
        fundamental: Value,
        max_harmonics: usize,
    ) -> Vec<Value> {
        let n = samples.len();
        let mut harmonics = Vec::with_capacity(max_harmonics);

        for h in 1..=max_harmonics {
            let freq = h as Value * fundamental;
            let bin = (freq * n as Value / sample_rate).round() as usize;

            if bin >= n / 2 {
                break;
            }

            // DFT at specific bin
            let mut re = 0.0;
            let mut im = 0.0;
            for (i, &sample) in samples.iter().enumerate() {
                let angle = 2.0 * PI * bin as f64 * i as f64 / n as f64;
                re += sample * angle.cos();
                im += sample * angle.sin();
            }

            let amplitude = 2.0 * (re * re + im * im).sqrt() / n as f64;
            harmonics.push(amplitude);
        }

        harmonics
    }
}

//=============================================================================
// SFDR (Spurious-Free Dynamic Range)
//=============================================================================

/// SFDR analysis result
#[derive(Debug, Clone, Default)]
pub struct SfdrResult {
    /// Carrier (signal) frequency
    pub signal_freq: Value,

    /// Carrier amplitude (dBFS or dBm)
    pub signal_level_db: Value,

    /// Largest spur frequency
    pub spur_freq: Value,

    /// Largest spur level (dB)
    pub spur_level_db: Value,

    /// SFDR in dB (signal - largest_spur)
    pub sfdr_db: Value,

    /// SFDR referenced to full-scale (dBFS)
    pub sfdr_dbfs: Value,

    /// Number of spurs found above noise floor
    pub num_spurs: usize,

    /// All spur frequencies and levels
    pub spurs: Vec<(Value, Value)>, // (freq, level_db)
}

impl SfdrResult {
    /// Calculate SFDR from spectrum data
    ///
    /// # Arguments
    /// * `frequencies` - Frequency bins
    /// * `magnitudes_db` - Magnitude spectrum in dB
    /// * `signal_freq` - Expected signal frequency (carrier)
    /// * `signal_bw` - Bandwidth to exclude around signal
    pub fn from_spectrum(
        frequencies: &[Value],
        magnitudes_db: &[Value],
        signal_freq: Value,
        signal_bw: Value,
    ) -> Self {
        if frequencies.len() != magnitudes_db.len() || frequencies.is_empty() {
            return Self::default();
        }

        // Find signal peak (within bandwidth of expected)
        let signal_indices: Vec<usize> = frequencies
            .iter()
            .enumerate()
            .filter(|(_, f)| (*f - signal_freq).abs() < signal_bw / 2.0)
            .map(|(i, _)| i)
            .collect();

        let signal_idx = signal_indices
            .iter()
            .max_by(|a, b| {
                let lhs = magnitudes_db[**a];
                let rhs = magnitudes_db[**b];
                let lhs = if lhs.is_finite() {
                    lhs
                } else {
                    f64::NEG_INFINITY
                };
                let rhs = if rhs.is_finite() {
                    rhs
                } else {
                    f64::NEG_INFINITY
                };
                lhs.total_cmp(&rhs)
            })
            .copied()
            .unwrap_or(0);

        let signal_level = magnitudes_db
            .get(signal_idx)
            .copied()
            .filter(|level| level.is_finite())
            .unwrap_or(0.0);
        let actual_signal_freq = frequencies
            .get(signal_idx)
            .copied()
            .filter(|freq| freq.is_finite())
            .unwrap_or(signal_freq);

        // Find all spurs (exclude DC and signal region)
        let mut spurs: Vec<(Value, Value)> = Vec::new();
        let exclude_start = actual_signal_freq - signal_bw;
        let exclude_end = actual_signal_freq + signal_bw;

        for (i, mag) in magnitudes_db.iter().enumerate() {
            let freq = frequencies[i];
            if !freq.is_finite() || !mag.is_finite() {
                continue;
            }
            // Exclude DC bin and signal region
            if freq.abs() < frequencies.get(1).copied().unwrap_or(1.0) {
                continue;
            }
            if freq >= exclude_start && freq <= exclude_end {
                continue;
            }
            spurs.push((freq, *mag));
        }

        // Sort by level (descending)
        spurs.sort_by(|a, b| b.1.total_cmp(&a.1));

        let (spur_freq, spur_level) = spurs.first().copied().unwrap_or((0.0, -200.0));

        Self {
            signal_freq: actual_signal_freq,
            signal_level_db: signal_level,
            spur_freq,
            spur_level_db: spur_level,
            sfdr_db: signal_level - spur_level,
            sfdr_dbfs: -spur_level, // Assuming 0 dBFS reference
            num_spurs: spurs.len(),
            spurs,
        }
    }
}

//=============================================================================
// IMD (Intermodulation Distortion)
//=============================================================================

/// Intermodulation distortion result
#[derive(Debug, Clone, Default)]
pub struct ImdResult {
    /// First input frequency
    pub f1: Value,

    /// Second input frequency
    pub f2: Value,

    /// Fundamental amplitudes (f1, f2)
    pub fundamental_levels: (Value, Value),

    /// Second-order products (f1+f2, f1-f2, 2*f1, 2*f2)
    pub imd2_products: Vec<(Value, Value)>, // (freq, level_db)

    /// Third-order products (2*f1-f2, 2*f2-f1)
    pub imd3_products: Vec<(Value, Value)>,

    /// OIP2 (output-referred IP2) in dBm
    pub oip2_dbm: Value,

    /// IIP2 (input-referred IP2) in dBm
    pub iip2_dbm: Value,

    /// OIP3 (output-referred IP3) in dBm
    pub oip3_dbm: Value,

    /// IIP3 (input-referred IP3) in dBm
    pub iip3_dbm: Value,

    /// Gain from input to output (dB)
    pub gain_db: Value,
}

impl ImdResult {
    /// Calculate intercept points from two-tone test data
    ///
    /// # Arguments
    /// * `f1`, `f2` - Input frequencies
    /// * `input_level_dbm` - Input power per tone
    /// * `fundamental_output_dbm` - Output power at fundamentals
    /// * `imd2_output_dbm` - Output power at 2nd-order IMD products
    /// * `imd3_output_dbm` - Output power at 3rd-order IMD products
    pub fn from_two_tone_test(
        f1: Value,
        f2: Value,
        input_level_dbm: Value,
        fundamental_output_dbm: Value,
        imd2_output_dbm: Value,
        imd3_output_dbm: Value,
    ) -> Self {
        let gain_db = fundamental_output_dbm - input_level_dbm;

        // OIP2 = Pout + (Pout - P_IMD2)
        // The IMD2 product grows at 2x rate, so slope is 2:1
        let oip2 = fundamental_output_dbm + (fundamental_output_dbm - imd2_output_dbm);
        let iip2 = oip2 - gain_db;

        // OIP3 = Pout + (Pout - P_IMD3) / 2
        // The IMD3 product grows at 3x rate, so slope is 3:1
        // OIP3 = Pout + (Pout - P_IMD3) / 2
        let oip3 = fundamental_output_dbm + (fundamental_output_dbm - imd3_output_dbm) / 2.0;
        let iip3 = oip3 - gain_db;

        Self {
            f1,
            f2,
            fundamental_levels: (fundamental_output_dbm, fundamental_output_dbm),
            imd2_products: vec![
                (f1 + f2, imd2_output_dbm),
                ((f1 - f2).abs(), imd2_output_dbm),
            ],
            imd3_products: vec![
                (2.0 * f1 - f2, imd3_output_dbm),
                (2.0 * f2 - f1, imd3_output_dbm),
            ],
            oip2_dbm: oip2,
            iip2_dbm: iip2,
            oip3_dbm: oip3,
            iip3_dbm: iip3,
            gain_db,
        }
    }

    /// Calculate 1dB compression point estimate from IP3
    pub fn p1db_estimate(&self) -> Value {
        // P1dB ≈ OIP3 - 9.6 dB (theoretical)
        self.oip3_dbm - 9.6
    }
}

//=============================================================================
// Group Delay
//=============================================================================

/// Group delay calculation from phase data
#[derive(Debug, Clone, Default)]
pub struct GroupDelayResult {
    /// Frequency points
    pub frequencies: Vec<Value>,

    /// Group delay values (seconds)
    pub delays: Vec<Value>,

    /// Average group delay
    pub average_delay: Value,

    /// Peak-to-peak group delay variation
    pub ripple: Value,

    /// Frequency of maximum delay
    pub max_delay_freq: Value,

    /// Maximum delay value
    pub max_delay: Value,
}

impl GroupDelayResult {
    /// Calculate group delay from phase vs frequency data
    ///
    /// Group delay τ = -dφ/dω = -dφ/(2π·df)
    ///
    /// # Arguments
    /// * `frequencies` - Frequency points (Hz)
    /// * `phases` - Phase values (radians)
    pub fn from_phase_data(frequencies: &[Value], phases: &[Value]) -> Self {
        if frequencies.len() < 2 || frequencies.len() != phases.len() {
            return Self::default();
        }

        let n = frequencies.len();
        let mut result_freqs = Vec::with_capacity(n - 1);
        let mut delays = Vec::with_capacity(n - 1);

        for i in 0..n - 1 {
            if !frequencies[i].is_finite()
                || !frequencies[i + 1].is_finite()
                || !phases[i].is_finite()
                || !phases[i + 1].is_finite()
            {
                continue;
            }

            let df = frequencies[i + 1] - frequencies[i];
            if df.abs() < 1e-15 {
                continue;
            }

            // Unwrap phase difference
            let mut dphi = phases[i + 1] - phases[i];
            while dphi > PI {
                dphi -= 2.0 * PI;
            }
            while dphi < -PI {
                dphi += 2.0 * PI;
            }

            // τ = -dφ/(2π·df)
            let tau = -dphi / (2.0 * PI * df);
            if !tau.is_finite() {
                continue;
            }

            result_freqs.push((frequencies[i] + frequencies[i + 1]) / 2.0);
            delays.push(tau);
        }

        if delays.is_empty() {
            return Self::default();
        }

        let average = delays.iter().sum::<Value>() / delays.len() as Value;

        let max_delay = delays.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_delay = delays.iter().cloned().fold(f64::INFINITY, f64::min);
        let ripple = max_delay - min_delay;

        let max_idx = delays
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(i, _)| i)
            .unwrap_or(0);

        Self {
            frequencies: result_freqs.clone(),
            delays,
            average_delay: average,
            ripple,
            max_delay_freq: result_freqs.get(max_idx).copied().unwrap_or(0.0),
            max_delay,
        }
    }

    /// Calculate from complex transfer function data
    pub fn from_transfer_function(frequencies: &[Value], h: &[Complex64]) -> Self {
        let phases: Vec<Value> = h.iter().map(|c| c.arg()).collect();
        Self::from_phase_data(frequencies, &phases)
    }
}

//=============================================================================
// SNR (Signal-to-Noise Ratio)
//=============================================================================

/// SNR analysis result
#[derive(Debug, Clone, Default)]
pub struct SnrResult {
    /// Signal power (linear, watts or V²)
    pub signal_power: Value,

    /// Noise power (linear)
    pub noise_power: Value,

    /// SNR in dB
    pub snr_db: Value,

    /// SINAD (Signal + Noise + Distortion) in dB
    pub sinad_db: Option<Value>,

    /// ENOB (Effective Number of Bits) - for ADC characterization
    pub enob: Option<Value>,

    /// Noise bandwidth used (Hz)
    pub noise_bandwidth: Value,
}

impl SnrResult {
    /// Calculate SNR from signal and noise powers
    pub fn from_powers(signal_power: Value, noise_power: Value) -> Self {
        let snr_db = if noise_power > 0.0 {
            10.0 * (signal_power / noise_power).log10()
        } else {
            f64::INFINITY
        };

        Self {
            signal_power,
            noise_power,
            snr_db,
            sinad_db: None,
            enob: None,
            noise_bandwidth: 0.0,
        }
    }

    /// Calculate SINAD and ENOB from signal, noise, and distortion
    pub fn with_distortion(
        signal_power: Value,
        noise_power: Value,
        distortion_power: Value,
    ) -> Self {
        let nad = noise_power + distortion_power;
        let sinad_db = if nad > 0.0 {
            10.0 * (signal_power / nad).log10()
        } else {
            f64::INFINITY
        };

        // ENOB = (SINAD - 1.76) / 6.02
        let enob = (sinad_db - 1.76) / 6.02;

        let mut result = Self::from_powers(signal_power, noise_power);
        result.sinad_db = Some(sinad_db);
        result.enob = Some(enob);
        result
    }
}

//=============================================================================
// RMS and Power Calculations
//=============================================================================

/// Calculate RMS value of a waveform
pub fn rms(samples: &[Value]) -> Value {
    if samples.is_empty() {
        return 0.0;
    }
    let sum_sq: Value = samples.iter().map(|s| s * s).sum();
    (sum_sq / samples.len() as Value).sqrt()
}

/// Calculate power spectrum (magnitude squared) from complex spectrum
pub fn power_spectrum(spectrum: &[Complex64]) -> Vec<Value> {
    spectrum.iter().map(|c| c.norm_sqr()).collect()
}

/// Calculate power in dB from linear power
pub fn power_db(power: Value, reference: Value) -> Value {
    if power <= 0.0 || reference <= 0.0 {
        return f64::NEG_INFINITY;
    }
    10.0 * (power / reference).log10()
}

/// Calculate voltage/amplitude in dB from linear value
pub fn amplitude_db(amplitude: Value, reference: Value) -> Value {
    if amplitude <= 0.0 || reference <= 0.0 {
        return f64::NEG_INFINITY;
    }
    20.0 * (amplitude / reference).log10()
}

/// Convert dB to linear (power)
pub fn db_to_linear_power(db: Value) -> Value {
    10.0_f64.powf(db / 10.0)
}

/// Convert dB to linear (voltage/amplitude)
pub fn db_to_linear_amplitude(db: Value) -> Value {
    10.0_f64.powf(db / 20.0)
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // THD Tests
    // =========================================================================

    #[test]
    fn test_thd_from_harmonics_pure_sine() {
        // Pure sine: only fundamental, no harmonics
        let harmonics = vec![1.0, 0.0, 0.0, 0.0, 0.0];
        let result = ThdResult::from_harmonics(&harmonics, 1e3);

        assert!((result.thd_ratio).abs() < 1e-10);
        assert!(result.thd_percent < 1e-8);
    }

    #[test]
    fn test_thd_from_harmonics_distorted() {
        // Fundamental + 10% 2nd harmonic + 5% 3rd harmonic
        let harmonics = vec![1.0, 0.1, 0.05];
        let result = ThdResult::from_harmonics(&harmonics, 1e3);

        // THD = sqrt(0.1² + 0.05²) / 1 = sqrt(0.01 + 0.0025) = sqrt(0.0125) ≈ 0.1118
        assert!((result.thd_ratio - 0.1118).abs() < 0.001);
        assert!((result.thd_percent - 11.18).abs() < 0.1);
    }

    #[test]
    fn test_thd_from_harmonics_empty() {
        let result = ThdResult::from_harmonics(&[], 1e3);
        assert_eq!(result.fundamental_amplitude, 0.0);
    }

    #[test]
    fn test_thd_from_waveform() {
        // Generate sine wave with 3rd harmonic
        let n = 1024;
        let f1 = 100.0; // 100 Hz fundamental
        let sample_rate = 10000.0;

        let samples: Vec<Value> = (0..n)
            .map(|i| {
                let t = i as Value / sample_rate;
                (2.0 * PI * f1 * t).sin() + 0.1 * (2.0 * PI * 3.0 * f1 * t).sin()
            })
            .collect();

        let result = ThdResult::from_waveform(&samples, sample_rate, f1);

        // Should detect ~10% THD from 3rd harmonic
        assert!(result.thd_percent > 5.0);
        assert!(result.thd_percent < 15.0);
    }

    // =========================================================================
    // SFDR Tests
    // =========================================================================

    #[test]
    fn test_sfdr_calculation() {
        // Create spectrum with signal at 1 kHz and spur at 2 kHz
        let frequencies: Vec<Value> = (0..100).map(|i| i as Value * 100.0).collect();
        let mut magnitudes_db = vec![-80.0; 100];

        magnitudes_db[10] = 0.0; // Signal at 1 kHz = 0 dBFS
        magnitudes_db[20] = -60.0; // Spur at 2 kHz = -60 dBFS

        let result = SfdrResult::from_spectrum(&frequencies, &magnitudes_db, 1000.0, 200.0);

        assert!((result.signal_level_db - 0.0).abs() < 1.0);
        assert!(result.sfdr_db > 55.0 && result.sfdr_db < 65.0);
    }

    #[test]
    fn test_sfdr_empty() {
        let result = SfdrResult::from_spectrum(&[], &[], 1000.0, 100.0);
        assert_eq!(result.sfdr_db, 0.0);
    }

    #[test]
    fn test_sfdr_ignores_non_finite_bins() {
        let frequencies = vec![0.0, 1_000.0, 2_000.0, f64::NAN, 3_000.0];
        let magnitudes = vec![-90.0, 0.0, -65.0, -10.0, f64::INFINITY];

        let result = SfdrResult::from_spectrum(&frequencies, &magnitudes, 1_000.0, 200.0);

        assert!(result.signal_level_db.is_finite());
        assert!(result.spur_level_db.is_finite());
        assert!(result.sfdr_db.is_finite());
        assert!(result
            .spurs
            .iter()
            .all(|(f, l)| f.is_finite() && l.is_finite()));
    }

    // =========================================================================
    // IMD Tests
    // =========================================================================

    #[test]
    fn test_imd_two_tone() {
        // Standard two-tone test
        let result = ImdResult::from_two_tone_test(
            1e6,   // f1 = 1 MHz
            1.1e6, // f2 = 1.1 MHz
            -10.0, // Input = -10 dBm per tone
            0.0,   // Output fundamental = 0 dBm (10 dB gain)
            -50.0, // IMD2 = -50 dBm
            -60.0, // IMD3 = -60 dBm
        );

        assert!((result.gain_db - 10.0).abs() < 0.1);
        assert!(result.oip3_dbm > 25.0); // OIP3 should be high
        assert!(result.iip3_dbm > 15.0); // IIP3 = OIP3 - gain
    }

    #[test]
    fn test_imd_p1db_estimate() {
        let result = ImdResult::from_two_tone_test(1e6, 1.1e6, -10.0, 0.0, -40.0, -50.0);

        let p1db = result.p1db_estimate();
        // P1dB should be about 9.6 dB below OIP3
        assert!((p1db - (result.oip3_dbm - 9.6)).abs() < 0.1);
    }

    // =========================================================================
    // Group Delay Tests
    // =========================================================================

    #[test]
    fn test_group_delay_constant() {
        // Linear phase = constant group delay
        let frequencies: Vec<Value> = (1..=10).map(|i| i as Value * 1e3).collect();
        let delay = 1e-6; // 1 μs delay
        let phases: Vec<Value> = frequencies.iter().map(|&f| -2.0 * PI * f * delay).collect();

        let result = GroupDelayResult::from_phase_data(&frequencies, &phases);

        // All delays should be ≈ 1 μs
        for d in &result.delays {
            assert!((d - delay).abs() < 1e-8);
        }
        assert!((result.average_delay - delay).abs() < 1e-8);
        assert!(result.ripple < 1e-9); // Minimal ripple
    }

    #[test]
    fn test_group_delay_from_transfer() {
        let frequencies: Vec<Value> = (1..=10).map(|i| i as Value * 1e3).collect();

        // Single pole at 1 kHz
        let pole = 2.0 * PI * 1e3;
        let h: Vec<Complex64> = frequencies
            .iter()
            .map(|&f| {
                let s = Complex64::new(0.0, 2.0 * PI * f);
                Complex64::new(1.0, 0.0) / (1.0 + s / pole)
            })
            .collect();

        let result = GroupDelayResult::from_transfer_function(&frequencies, &h);

        assert!(!result.delays.is_empty());
        assert!(result.average_delay > 0.0);
    }

    #[test]
    fn test_group_delay_empty() {
        let result = GroupDelayResult::from_phase_data(&[], &[]);
        assert!(result.frequencies.is_empty());
    }

    #[test]
    fn test_group_delay_skips_non_finite_phase_points() {
        let frequencies = vec![1e3, 2e3, 3e3, 4e3, 5e3];
        let phases = vec![0.0, -0.2, f64::NAN, -0.8, -1.0];

        let result = GroupDelayResult::from_phase_data(&frequencies, &phases);
        assert!(!result.delays.is_empty());
        assert!(result.delays.iter().all(|delay| delay.is_finite()));
        assert!(result.max_delay.is_finite());
        assert!(result.max_delay_freq.is_finite());
    }

    // =========================================================================
    // SNR Tests
    // =========================================================================

    #[test]
    fn test_snr_calculation() {
        // Signal = 1W, Noise = 0.001W → SNR = 30 dB
        let result = SnrResult::from_powers(1.0, 0.001);

        assert!((result.snr_db - 30.0).abs() < 0.01);
    }

    #[test]
    fn test_snr_with_distortion() {
        // Signal = 1W, Noise = 0.0001W, Distortion = 0.0001W
        let result = SnrResult::with_distortion(1.0, 0.0001, 0.0001);

        // SINAD = 10*log10(1 / 0.0002) = 10*log10(5000) ≈ 37 dB
        assert!(result.sinad_db.unwrap() > 36.0 && result.sinad_db.unwrap() < 38.0);

        // ENOB ≈ (37 - 1.76) / 6.02 ≈ 5.85 bits
        assert!(result.enob.unwrap() > 5.5 && result.enob.unwrap() < 6.5);
    }

    #[test]
    fn test_snr_zero_noise() {
        let result = SnrResult::from_powers(1.0, 0.0);
        assert!(result.snr_db.is_infinite());
    }

    // =========================================================================
    // Utility Function Tests
    // =========================================================================

    #[test]
    fn test_rms_sine() {
        // RMS of sine wave = peak / √2
        let n = 1000;
        let samples: Vec<Value> = (0..n)
            .map(|i| (2.0 * PI * i as f64 / n as f64).sin())
            .collect();

        let rms_val = rms(&samples);
        assert!((rms_val - 1.0 / 2.0_f64.sqrt()).abs() < 0.01);
    }

    #[test]
    fn test_rms_empty() {
        assert_eq!(rms(&[]), 0.0);
    }

    #[test]
    fn test_power_spectrum() {
        let spectrum = vec![
            Complex64::new(1.0, 0.0),
            Complex64::new(0.0, 2.0),
            Complex64::new(1.0, 1.0),
        ];

        let ps = power_spectrum(&spectrum);

        assert!((ps[0] - 1.0).abs() < 1e-10);
        assert!((ps[1] - 4.0).abs() < 1e-10);
        assert!((ps[2] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_db_conversions() {
        // 0 dB = 1x
        assert!((db_to_linear_power(0.0) - 1.0).abs() < 1e-10);
        assert!((db_to_linear_amplitude(0.0) - 1.0).abs() < 1e-10);

        // 20 dB voltage = 10x, 20 dB power = 100x
        assert!((db_to_linear_amplitude(20.0) - 10.0).abs() < 1e-10);
        assert!((db_to_linear_power(20.0) - 100.0).abs() < 1e-10);

        // Round trip
        let amp = 5.0;
        let db_val = amplitude_db(amp, 1.0);
        let recovered = db_to_linear_amplitude(db_val);
        assert!((recovered - amp).abs() < 1e-10);
    }

    #[test]
    fn test_amplitude_db() {
        // 10x = 20 dB
        assert!((amplitude_db(10.0, 1.0) - 20.0).abs() < 0.01);

        // 0 value
        assert!(amplitude_db(0.0, 1.0) == f64::NEG_INFINITY);
    }

    #[test]
    fn test_power_db() {
        // 10x = 10 dB
        assert!((power_db(10.0, 1.0) - 10.0).abs() < 0.01);

        // 100x = 20 dB
        assert!((power_db(100.0, 1.0) - 20.0).abs() < 0.01);
    }
}
