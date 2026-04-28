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

