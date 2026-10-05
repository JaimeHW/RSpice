//! Post-Processing Utilities Module
//!
//! Checked THD, SFDR, group-delay, SNR, and sample RMS measurements.
//!
//! Invalid inputs and unavailable measurements return errors. Waveform THD
//! uses the same whole-period integration as `.FOUR`; adjacent phase samples
//! must resolve changes within half a turn for group-delay unwrapping.

use super::fourier::{FourierAnalysis, FourierConfig, FourierError};
use crate::Value;
use num_complex::Complex64;

/// Invalid data, unavailable evidence, or an unrepresentable measurement.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum PostProcessingError {
    /// The measurement's input contract was not met.
    #[error("invalid measurement input: {0}")]
    InvalidInput(&'static str),
    /// The supplied data cannot define the requested measurement.
    #[error("measurement is undefined: {0}")]
    Undefined(&'static str),
    /// Finite input produced a quantity outside the representable range.
    #[error("measurement is not representable: {0}")]
    Unrepresentable(&'static str),
    /// Workspace allocation failed.
    #[error("cannot allocate measurement workspace: {0}")]
    Allocation(&'static str),
    /// Shared Fourier qualification or integration failed.
    #[error(transparent)]
    Fourier(#[from] FourierError),
}

fn validate_frequencies(frequencies: &[Value]) -> Result<(), PostProcessingError> {
    if frequencies.is_empty()
        || frequencies.iter().any(|f| !f.is_finite() || *f < 0.0)
        || frequencies.windows(2).any(|w| w[1] <= w[0])
    {
        return Err(PostProcessingError::InvalidInput(
            "frequencies must be finite, nonnegative, and strictly increasing",
        ));
    }
    Ok(())
}

fn workspace<T>(length: usize, name: &'static str) -> Result<Vec<T>, PostProcessingError> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(length)
        .map_err(|_| PostProcessingError::Allocation(name))?;
    Ok(values)
}

//=============================================================================
// THD (Total Harmonic Distortion)
//=============================================================================

/// Total Harmonic Distortion analysis result
#[derive(Debug, Clone)]
pub struct ThdResult {
    /// Fundamental frequency (Hz)
    pub fundamental_freq: Value,

    /// Fundamental amplitude (linear)
    pub fundamental_amplitude: Value,

    /// THD as a nonnegative ratio (may exceed one)
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
    pub fn from_harmonics(
        harmonics: &[Value],
        fundamental_freq: Value,
    ) -> Result<Self, PostProcessingError> {
        if !fundamental_freq.is_finite()
            || fundamental_freq <= 0.0
            || harmonics.is_empty()
            || harmonics.iter().any(|h| !h.is_finite() || *h < 0.0)
        {
            return Err(PostProcessingError::InvalidInput(
                "positive finite fundamental frequency and finite nonnegative harmonic amplitudes are required",
            ));
        }
        let fundamental = harmonics[0];
        if fundamental == 0.0 {
            return Err(PostProcessingError::Undefined("zero fundamental amplitude"));
        }
        // Scale before accumulation: neither squared amplitudes nor their
        // unnormalized Euclidean norm need fit into a floating-point value.
        let scale = harmonics.iter().skip(1).copied().fold(0.0, Value::max);
        let norm = if scale == 0.0 {
            0.0
        } else {
            harmonics
                .iter()
                .skip(1)
                .fold(0.0_f64, |norm, h| norm.hypot(h / scale))
        };
        let thd_ratio = (scale / fundamental) * norm;
        let thd_percent = thd_ratio * 100.0;
        if !thd_percent.is_finite() || (scale > 0.0 && thd_ratio == 0.0) {
            return Err(PostProcessingError::Unrepresentable(
                "total harmonic distortion",
            ));
        }
        let mut retained = workspace(harmonics.len(), "harmonic amplitudes")?;
        retained.extend_from_slice(harmonics);
        Ok(Self {
            fundamental_freq,
            fundamental_amplitude: fundamental,
            thd_ratio,
            thd_percent,
            thd_db: 20.0 * thd_ratio.log10(),
            harmonics: retained,
            num_harmonics: harmonics.len(),
            thd_plus_noise: None,
        })
    }

    /// Measure harmonics 1 through 10 over the last complete fundamental
    /// period, using the same integration and sampling checks as `.FOUR`.
    /// Short or under-resolved records are rejected, not zero-padded.
    pub fn from_waveform(
        samples: &[Value],
        sample_rate: Value,
        fundamental_freq: Value,
    ) -> Result<Self, PostProcessingError> {
        if !sample_rate.is_finite() || sample_rate <= 0.0 {
            return Err(PostProcessingError::InvalidInput(
                "sample rate must be positive and finite",
            ));
        }
        let mut times = workspace(samples.len(), "waveform time axis")?;
        times.extend((0..samples.len()).map(|i| i as Value / sample_rate));
        let result = FourierAnalysis::new(FourierConfig::new(fundamental_freq).with_harmonics(10))
            .analyze(&times, samples)?;
        let mut harmonics = workspace(10, "harmonic amplitudes")?;
        harmonics.extend(result.harmonics.iter().skip(1).map(|h| h.magnitude));
        Self::from_harmonics(&harmonics, fundamental_freq)
    }
}

//=============================================================================
// SFDR (Spurious-Free Dynamic Range)
//=============================================================================

/// SFDR analysis result
#[derive(Debug, Clone)]
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

    /// Full-scale dynamic range, present only for explicitly dBFS input.
    pub sfdr_dbfs: Option<Value>,

    /// Number of finite out-of-band bins (no noise-floor estimate is implied)
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
    ) -> Result<Self, PostProcessingError> {
        validate_frequencies(frequencies)?;
        if frequencies.len() != magnitudes_db.len()
            || magnitudes_db
                .iter()
                .any(|m| m.is_nan() || *m == Value::INFINITY)
            || !signal_freq.is_finite()
            || signal_freq <= 0.0
            || !signal_bw.is_finite()
            || signal_bw <= 0.0
        {
            return Err(PostProcessingError::InvalidInput(
                "matching spectrum lengths, finite levels (or -infinity for zero), and positive finite carrier frequency/bandwidth are required",
            ));
        }
        let in_carrier_band = |frequency: Value| (frequency - signal_freq).abs() <= signal_bw / 2.0;
        let signal_idx = frequencies
            .iter()
            .enumerate()
            .filter(|(i, f)| in_carrier_band(**f) && magnitudes_db[*i].is_finite())
            .max_by(|(a, _), (b, _)| magnitudes_db[*a].total_cmp(&magnitudes_db[*b]))
            .map(|(i, _)| i)
            .ok_or(PostProcessingError::Undefined(
                "no finite carrier in the requested band",
            ))?;
        let mut spurs = workspace(frequencies.len(), "spectrum spurs")?;
        for (&frequency, &level) in frequencies.iter().zip(magnitudes_db) {
            if frequency > 0.0 && !in_carrier_band(frequency) && level.is_finite() {
                spurs.push((frequency, level));
            }
        }
        spurs.sort_unstable_by(|a, b| b.1.total_cmp(&a.1));
        let (spur_freq, spur_level_db) = spurs
            .first()
            .copied()
            .ok_or(PostProcessingError::Undefined("no finite out-of-band spur"))?;
        let signal_level_db = magnitudes_db[signal_idx];
        let sfdr_db = signal_level_db - spur_level_db;
        if !sfdr_db.is_finite() {
            return Err(PostProcessingError::Unrepresentable("SFDR"));
        }
        Ok(Self {
            signal_freq: frequencies[signal_idx],
            signal_level_db,
            spur_freq,
            spur_level_db,
            sfdr_db,
            sfdr_dbfs: None,
            num_spurs: spurs.len(),
            spurs,
        })
    }

    /// Calculate from levels explicitly referenced to full scale (dBFS).
    pub fn from_spectrum_dbfs(
        frequencies: &[Value],
        magnitudes_dbfs: &[Value],
        signal_freq: Value,
        signal_bw: Value,
    ) -> Result<Self, PostProcessingError> {
        let mut result = Self::from_spectrum(frequencies, magnitudes_dbfs, signal_freq, signal_bw)?;
        result.sfdr_dbfs = Some(-result.spur_level_db);
        Ok(result)
    }
}

//=============================================================================
// IMD (Intermodulation Distortion)
//=============================================================================

/// Container for externally computed intermodulation distortion measurements.
/// This type does not implement an IMD or intercept-point estimator.
#[derive(Debug, Clone)]
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

//=============================================================================
// Group Delay
//=============================================================================

/// Group delay calculation from phase data
#[derive(Debug, Clone)]
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
    pub fn from_phase_data(
        frequencies: &[Value],
        phases: &[Value],
    ) -> Result<Self, PostProcessingError> {
        validate_frequencies(frequencies)?;
        if frequencies.len() < 2
            || frequencies.len() != phases.len()
            || phases.iter().any(|p| !p.is_finite())
        {
            return Err(PostProcessingError::InvalidInput(
                "at least two matching finite phase/frequency samples are required",
            ));
        }
        let mut result_freqs = workspace(frequencies.len() - 1, "group delay frequencies")?;
        let mut delays = workspace(frequencies.len() - 1, "group delays")?;
        for (f, p) in frequencies.windows(2).zip(phases.windows(2)) {
            let delay = super::phase::group_delay(f[0], p[0], f[1], p[1])
                .ok_or(PostProcessingError::Unrepresentable("group delay"))?;
            result_freqs.push(f[0] + (f[1] - f[0]) / 2.0);
            delays.push(delay);
        }
        let max_idx = (0..delays.len())
            .max_by(|&a, &b| delays[a].total_cmp(&delays[b]))
            .expect("nonempty delays");
        let max_delay = delays[max_idx];
        let min_delay = delays.iter().copied().fold(Value::INFINITY, Value::min);
        let scale = max_delay.abs().max(min_delay.abs());
        let average_delay = if scale == 0.0 {
            0.0
        } else {
            // Normalize before summing to avoid overflow in an otherwise finite mean.
            scale
                * (delays.iter().map(|delay| delay / scale).sum::<Value>() / delays.len() as Value)
        };
        let ripple = max_delay - min_delay;
        if !ripple.is_finite() || !average_delay.is_finite() {
            return Err(PostProcessingError::Unrepresentable(
                "group delay statistics",
            ));
        }
        let max_delay_freq = result_freqs[max_idx];
        Ok(Self {
            frequencies: result_freqs,
            delays,
            average_delay,
            ripple,
            max_delay_freq,
            max_delay,
        })
    }

    /// Calculate from finite, nonzero complex transfer-function samples.
    pub fn from_transfer_function(
        frequencies: &[Value],
        h: &[Complex64],
    ) -> Result<Self, PostProcessingError> {
        if h.iter()
            .any(|c| !c.re.is_finite() || !c.im.is_finite() || *c == Complex64::new(0.0, 0.0))
        {
            return Err(PostProcessingError::InvalidInput(
                "group delay requires finite nonzero transfer values",
            ));
        }
        let mut phases = workspace(h.len(), "transfer phases")?;
        phases.extend(h.iter().map(|c| c.arg()));
        Self::from_phase_data(frequencies, &phases)
    }
}

//=============================================================================
// SNR (Signal-to-Noise Ratio)
//=============================================================================

/// SNR analysis result
#[derive(Debug, Clone)]
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
    pub fn from_powers(
        signal_power: Value,
        noise_power: Value,
    ) -> Result<Self, PostProcessingError> {
        if !signal_power.is_finite()
            || !noise_power.is_finite()
            || signal_power < 0.0
            || noise_power < 0.0
        {
            return Err(PostProcessingError::InvalidInput(
                "powers must be finite and nonnegative",
            ));
        }
        if signal_power == 0.0 && noise_power == 0.0 {
            return Err(PostProcessingError::Undefined(
                "both signal and noise powers are zero",
            ));
        }
        // Subtract logarithms so a finite dB result survives ratio overflow.
        let snr_db = 10.0 * (signal_power.log10() - noise_power.log10());
        Ok(Self {
            signal_power,
            noise_power,
            snr_db,
            sinad_db: None,
            enob: None,
            noise_bandwidth: 0.0,
        })
    }
}

//=============================================================================
// RMS and Power Calculations
//=============================================================================

/// Equal-weight sample RMS. For irregularly sampled waveforms use the
/// time-weighted measurement API instead. Empty/non-finite input is rejected.
pub fn rms(samples: &[Value]) -> Result<Value, PostProcessingError> {
    if samples.is_empty() || samples.iter().any(|s| !s.is_finite()) {
        return Err(PostProcessingError::InvalidInput(
            "RMS requires nonempty finite samples",
        ));
    }
    let scale = samples.iter().map(|s| s.abs()).fold(0.0, Value::max);
    if scale == 0.0 {
        return Ok(0.0);
    }
    let mean_square =
        samples.iter().map(|s| (s / scale).powi(2)).sum::<Value>() / samples.len() as Value;
    Ok(scale * mean_square.sqrt())
}
