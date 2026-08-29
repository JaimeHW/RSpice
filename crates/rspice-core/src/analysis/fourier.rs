//! Fourier Analysis (.FOUR directive)
//!
//! Computes Fourier components and Total Harmonic Distortion (THD) from
//! transient simulation results.
//!
//! # SPICE Syntax
//! ```text
//! .FOUR <freq> [nharms] <output> [output...]
//! .FOUR 1kHz V(out)
//! .FOUR 60Hz 15 V(load) I(Rsense)
//! ```
//!
//! # Output
//! Computes magnitude and phase of DC, fundamental, and harmonics up to nharms.
//! Also computes THD (Total Harmonic Distortion).
//!
//! # Algorithm
//! Uses trapezoidal Fourier integration over the last configured period(s) of
//! the waveform.

use crate::Value;
use std::f64::consts::PI;

//=============================================================================
// Fourier Analysis Settings
//=============================================================================

/// Configuration for Fourier analysis
#[derive(Debug, Clone)]
pub struct FourierConfig {
    /// Fundamental frequency (Hz)
    pub fundamental_freq: Value,
    /// Number of harmonics to compute (default 9)
    pub num_harmonics: usize,
    /// Number of periods to analyze (default 1)
    pub num_periods: usize,
}

impl FourierConfig {
    /// Create new Fourier config with fundamental frequency
    pub fn new(freq: Value) -> Self {
        Self {
            fundamental_freq: freq,
            num_harmonics: 9,
            num_periods: 1,
        }
    }

    /// Set number of harmonics
    pub fn with_harmonics(mut self, n: usize) -> Self {
        self.num_harmonics = n;
        self
    }

    /// Get period of fundamental
    pub(crate) fn period(&self) -> Value {
        1.0 / self.fundamental_freq
    }

    /// Get analysis window duration
    pub(crate) fn window_duration(&self) -> Value {
        self.period() * self.num_periods as f64
    }
}

/// A failure to qualify a Fourier configuration, waveform, or computed
/// spectrum.
///
/// Fourier failures are explicit because a zero-filled spectrum is a valid
/// numerical result for some waveforms and therefore cannot also represent
/// missing or invalid evidence.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
#[non_exhaustive]
pub enum FourierError {
    /// The requested fundamental cannot define a physical period.
    #[error("fundamental frequency must be positive and finite, got {frequency}")]
    InvalidFundamentalFrequency { frequency: Value },
    /// A result without a fundamental component cannot satisfy `.FOUR`.
    #[error("number of harmonics must be at least one")]
    NoHarmonics,
    /// The analysis window must contain at least one period.
    #[error("number of analysis periods must be at least one")]
    NoPeriods,
    /// The requested harmonic vector cannot be represented or allocated.
    #[error("cannot allocate a Fourier result for {num_harmonics} harmonics")]
    HarmonicCapacity { num_harmonics: usize },
    /// The selected waveform window cannot be retained safely.
    #[error("cannot allocate a Fourier analysis window for {samples} samples")]
    WindowCapacity { samples: usize },
    /// No waveform samples were supplied.
    #[error("waveform is empty")]
    EmptyWaveform,
    /// Time and value evidence must describe the same samples.
    #[error("time/value length mismatch: {time_points} time points and {values} waveform values")]
    LengthMismatch { time_points: usize, values: usize },
    /// Numerical integration requires at least two samples.
    #[error("waveform has {samples} sample(s); Fourier integration requires at least two")]
    InsufficientSamples { samples: usize },
    /// A time coordinate is not usable numerical evidence.
    #[error("time sample {index} is non-finite ({value})")]
    NonFiniteTime { index: usize, value: Value },
    /// A waveform coordinate is not usable numerical evidence.
    #[error("waveform sample {index} is non-finite ({value})")]
    NonFiniteValue { index: usize, value: Value },
    /// Integration requires a strictly increasing time axis.
    #[error(
        "time sample {index} ({current}) is not strictly greater than the preceding sample ({previous})"
    )]
    NonIncreasingTime {
        index: usize,
        previous: Value,
        current: Value,
    },
    /// Finite endpoints can still have an unrepresentable span.
    #[error("waveform time span from {start} to {end} is not finite and positive")]
    InvalidTimeSpan { start: Value, end: Value },
    /// The frequency/period request overflowed or underflowed.
    #[error("Fourier analysis window duration is not finite and positive ({duration})")]
    InvalidWindowDuration { duration: Value },
    /// The waveform does not cover the configured number of periods.
    #[error(
        "waveform duration {available} s is shorter than the required Fourier window {required} s"
    )]
    InsufficientDuration { available: Value, required: Value },
    /// The covered window does not contain enough integration points.
    #[error("Fourier analysis window contains only {samples} sample(s)")]
    InsufficientWindowSamples { samples: usize },
    /// The retained window cannot resolve the highest requested harmonic.
    #[error(
        "waveform sample interval {interval} s does not resolve harmonic {harmonic}; it must be no greater than {maximum} s"
    )]
    InsufficientSampleRate {
        harmonic: usize,
        interval: Value,
        maximum: Value,
    },
    /// A requested harmonic frequency overflowed.
    #[error("frequency for harmonic {harmonic} is non-finite ({frequency})")]
    NonFiniteHarmonicFrequency { harmonic: usize, frequency: Value },
    /// Numerical integration produced an unauthenticatable coefficient.
    #[error("computed {quantity} for harmonic {harmonic} is non-finite")]
    NonFiniteCoefficient {
        harmonic: usize,
        quantity: &'static str,
    },
    /// The aggregate distortion metric overflowed.
    #[error("computed total harmonic distortion is non-finite ({value})")]
    NonFiniteThd { value: Value },
    /// A public magnitude field or reference is not a valid magnitude.
    #[error("{role} must be finite and non-negative, got {value}")]
    InvalidMagnitude { role: &'static str, value: Value },
    /// A retained public THD field is malformed.
    #[error("total harmonic distortion must be finite and non-negative, got {value}")]
    InvalidThd { value: Value },
    /// A mathematically nonzero relative result cannot be represented by
    /// [`Value`].
    #[error("{quantity} is outside the representable floating-point range")]
    UnrepresentableRelativeSpectrum { quantity: &'static str },
}

//=============================================================================
// Fourier Analysis Engine
//=============================================================================

/// Fourier Analysis Engine
#[derive(Debug, Clone)]
pub struct FourierAnalysis {
    /// Configuration
    config: FourierConfig,
}

impl FourierAnalysis {
    /// Create new Fourier analysis
    pub fn new(config: FourierConfig) -> Self {
        Self { config }
    }

    /// Perform Fourier analysis on a waveform
    ///
    /// # Arguments
    /// * `time` - Time points
    /// * `values` - Waveform values at each time point
    ///
    /// # Returns
    /// A qualified Fourier result, or a typed error when the configuration,
    /// waveform evidence, or computed coefficients are invalid.
    pub fn analyze(&self, time: &[Value], values: &[Value]) -> Result<FourierResult, FourierError> {
        self.validate_configuration()?;
        validate_waveform(time, values)?;

        // Find analysis window (last periods of waveform)
        let window_duration = self.config.window_duration();
        if !window_duration.is_finite() || window_duration <= 0.0 {
            return Err(FourierError::InvalidWindowDuration {
                duration: window_duration,
            });
        }
        let t_end = time[time.len() - 1];
        let available_duration = t_end - time[0];
        if !available_duration.is_finite() || available_duration <= 0.0 {
            return Err(FourierError::InvalidTimeSpan {
                start: time[0],
                end: t_end,
            });
        }
        let duration_tolerance = 64.0
            * Value::EPSILON
            * available_duration
                .max(window_duration)
                .max(Value::MIN_POSITIVE);
        if available_duration + duration_tolerance < window_duration {
            return Err(FourierError::InsufficientDuration {
                available: available_duration,
                required: window_duration,
            });
        }
        let t_start = if available_duration <= window_duration {
            time[0]
        } else {
            t_end - window_duration
        };

        // Retain an exact-period window. When its leading edge lies between
        // samples, interpolate the boundary instead of silently shortening
        // the integration interval and biasing every coefficient.
        let (window_time, window_values) = exact_window(time, values, t_start)?;
        if window_time.len() < 3 {
            return Err(FourierError::InsufficientWindowSamples {
                samples: window_time.len(),
            });
        }

        let highest_frequency = self.config.num_harmonics as Value * self.config.fundamental_freq;
        if !highest_frequency.is_finite() {
            return Err(FourierError::NonFiniteHarmonicFrequency {
                harmonic: self.config.num_harmonics,
                frequency: highest_frequency,
            });
        }
        let maximum_interval = 1.0 / (8.0 * highest_frequency);
        let largest_interval = window_time
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .fold(0.0, Value::max);
        let interval_tolerance = maximum_interval * (1.0 + 16.0 * Value::EPSILON);
        if !maximum_interval.is_finite()
            || maximum_interval <= 0.0
            || largest_interval > interval_tolerance
        {
            return Err(FourierError::InsufficientSampleRate {
                harmonic: self.config.num_harmonics,
                interval: largest_interval,
                maximum: maximum_interval,
            });
        }

        // Compute Fourier coefficients using DFT
        let harmonic_count =
            self.config
                .num_harmonics
                .checked_add(1)
                .ok_or(FourierError::HarmonicCapacity {
                    num_harmonics: self.config.num_harmonics,
                })?;
        let mut harmonics = Vec::new();
        harmonics.try_reserve_exact(harmonic_count).map_err(|_| {
            FourierError::HarmonicCapacity {
                num_harmonics: self.config.num_harmonics,
            }
        })?;

        for n in 0..=self.config.num_harmonics {
            let freq = n as f64 * self.config.fundamental_freq;
            if !freq.is_finite() {
                return Err(FourierError::NonFiniteHarmonicFrequency {
                    harmonic: n,
                    frequency: freq,
                });
            }
            let (mag, phase) = self.compute_harmonic(&window_time, &window_values, freq, n)?;

            harmonics.push(HarmonicComponent {
                harmonic_number: n,
                frequency: freq,
                magnitude: mag,
                phase,
            });
        }

        // Calculate THD
        let dc = harmonics[0].magnitude;
        let fundamental = harmonics.get(1).map(|h| h.magnitude).unwrap_or(0.0);

        let harmonic_norm: Value = harmonics
            .iter()
            .skip(2) // Skip DC and fundamental
            .fold(0.0, |norm, harmonic| norm.hypot(harmonic.magnitude));

        let thd = if fundamental == 0.0 {
            None
        } else {
            let value = harmonic_norm / fundamental * 100.0;
            if !value.is_finite() {
                return Err(FourierError::NonFiniteThd { value });
            }
            Some(value)
        };

        Ok(FourierResult {
            fundamental_freq: self.config.fundamental_freq,
            dc_component: dc,
            harmonics,
            thd,
        })
    }

    fn validate_configuration(&self) -> Result<(), FourierError> {
        let fundamental = self.config.fundamental_freq;
        if !fundamental.is_finite() || fundamental <= 0.0 {
            return Err(FourierError::InvalidFundamentalFrequency {
                frequency: fundamental,
            });
        }
        if self.config.num_harmonics == 0 {
            return Err(FourierError::NoHarmonics);
        }
        if self.config.num_harmonics.checked_add(1).is_none() {
            return Err(FourierError::HarmonicCapacity {
                num_harmonics: self.config.num_harmonics,
            });
        }
        if self.config.num_periods == 0 {
            return Err(FourierError::NoPeriods);
        }
        Ok(())
    }

    /// Compute single harmonic component using numerical integration
    fn compute_harmonic(
        &self,
        time: &[Value],
        values: &[Value],
        freq: Value,
        harmonic: usize,
    ) -> Result<(Value, Value), FourierError> {
        let t_start = time[0];
        let t_end = time[time.len() - 1];
        let duration = t_end - t_start;

        if !duration.is_finite() || duration <= 0.0 {
            return Err(FourierError::InvalidTimeSpan {
                start: t_start,
                end: t_end,
            });
        }

        // For DC (n=0), just compute average
        if harmonic == 0 {
            let mut integral = 0.0;
            for index in 1..time.len() {
                let dt = time[index] - time[index - 1];
                let normalized_dt = dt / duration;
                let average = 0.5 * values[index - 1] + 0.5 * values[index];
                integral += average * normalized_dt;
            }
            let avg = integral;
            ensure_finite_coefficient(avg, harmonic, "DC component")?;
            return Ok((avg, 0.0));
        }

        // Compute a_n and b_n using trapezoidal integration
        // a_n = (2/T) * integral(f(t) * cos(2*pi*n*f*t) dt)
        // b_n = (2/T) * integral(f(t) * sin(2*pi*n*f*t) dt)

        let omega = 2.0 * PI * freq;
        if !omega.is_finite() {
            return Err(FourierError::NonFiniteHarmonicFrequency {
                harmonic,
                frequency: freq,
            });
        }

        let mut cosine_integral = 0.0;
        let mut sine_integral = 0.0;
        for index in 1..time.len() {
            let dt = time[index] - time[index - 1];
            let normalized_dt = dt / duration;
            let phase0 = omega * (time[index - 1] - t_start);
            let phase1 = omega * (time[index] - t_start);
            if !phase0.is_finite() || !phase1.is_finite() {
                return Err(FourierError::NonFiniteCoefficient {
                    harmonic,
                    quantity: "phase argument",
                });
            }
            let cosine_average =
                0.5 * values[index - 1] * phase0.cos() + 0.5 * values[index] * phase1.cos();
            let sine_average =
                0.5 * values[index - 1] * phase0.sin() + 0.5 * values[index] * phase1.sin();
            cosine_integral += cosine_average * normalized_dt;
            sine_integral += sine_average * normalized_dt;
        }

        let a_n = 2.0 * cosine_integral;
        let b_n = 2.0 * sine_integral;
        ensure_finite_coefficient(a_n, harmonic, "cosine coefficient")?;
        ensure_finite_coefficient(b_n, harmonic, "sine coefficient")?;

        let magnitude = a_n.hypot(b_n);
        let phase = (-b_n).atan2(a_n) * 180.0 / PI; // Convert to degrees
        ensure_finite_coefficient(magnitude, harmonic, "magnitude")?;
        ensure_finite_coefficient(phase, harmonic, "phase")?;

        Ok((magnitude, phase))
    }
}

fn validate_waveform(time: &[Value], values: &[Value]) -> Result<(), FourierError> {
    if time.len() != values.len() {
        return Err(FourierError::LengthMismatch {
            time_points: time.len(),
            values: values.len(),
        });
    }
    if time.is_empty() {
        return Err(FourierError::EmptyWaveform);
    }
    if time.len() < 2 {
        return Err(FourierError::InsufficientSamples {
            samples: time.len(),
        });
    }
    for (index, (&sample_time, &sample_value)) in time.iter().zip(values).enumerate() {
        if !sample_time.is_finite() {
            return Err(FourierError::NonFiniteTime {
                index,
                value: sample_time,
            });
        }
        if !sample_value.is_finite() {
            return Err(FourierError::NonFiniteValue {
                index,
                value: sample_value,
            });
        }
        if index > 0 && sample_time <= time[index - 1] {
            return Err(FourierError::NonIncreasingTime {
                index,
                previous: time[index - 1],
                current: sample_time,
            });
        }
    }
    Ok(())
}

fn ensure_finite_coefficient(
    value: Value,
    harmonic: usize,
    quantity: &'static str,
) -> Result<(), FourierError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(FourierError::NonFiniteCoefficient { harmonic, quantity })
    }
}

/// Select the trailing analysis window and interpolate its leading boundary
/// when it falls between authored transient samples.
fn exact_window(
    time: &[Value],
    values: &[Value],
    t_start: Value,
) -> Result<(Vec<Value>, Vec<Value>), FourierError> {
    let first_retained = time.partition_point(|&sample| sample < t_start);
    let interpolate_boundary = first_retained < time.len() && time[first_retained] != t_start;
    let retained_samples = time.len().saturating_sub(first_retained);
    let window_samples = retained_samples
        .checked_add(usize::from(interpolate_boundary))
        .ok_or(FourierError::WindowCapacity {
            samples: retained_samples,
        })?;
    let mut window_time = Vec::new();
    let mut window_values = Vec::new();
    window_time
        .try_reserve_exact(window_samples)
        .map_err(|_| FourierError::WindowCapacity {
            samples: window_samples,
        })?;
    window_values
        .try_reserve_exact(window_samples)
        .map_err(|_| FourierError::WindowCapacity {
            samples: window_samples,
        })?;

    if interpolate_boundary {
        let lower =
            first_retained
                .checked_sub(1)
                .ok_or(FourierError::InsufficientWindowSamples {
                    samples: retained_samples,
                })?;
        let fraction = (t_start - time[lower]) / (time[first_retained] - time[lower]);
        let interpolated =
            (1.0 - fraction).mul_add(values[lower], fraction * values[first_retained]);
        ensure_finite_coefficient(interpolated, 0, "interpolated window boundary")?;
        window_time.push(t_start);
        window_values.push(interpolated);
    }
    window_time.extend_from_slice(&time[first_retained..]);
    window_values.extend_from_slice(&values[first_retained..]);

    Ok((window_time, window_values))
}

//=============================================================================
// Results
//=============================================================================

/// Single harmonic component
#[derive(Debug, Clone)]
pub struct HarmonicComponent {
    /// Harmonic number (0 = DC, 1 = fundamental, 2 = 2nd harmonic, etc.)
    pub harmonic_number: usize,
    /// Frequency (Hz)
    pub frequency: Value,
    /// Magnitude
    pub magnitude: Value,
    /// Phase (degrees)
    pub phase: Value,
}

impl HarmonicComponent {
    /// Get normalized magnitude (percent of the fundamental).
    ///
    /// `Ok(None)` means the reference magnitude is exactly zero. Invalid
    /// public magnitude fields are rejected instead of being interpreted as
    /// zero.
    pub fn normalized(&self, fundamental_mag: Value) -> Result<Option<Value>, FourierError> {
        validate_relative_magnitude("harmonic magnitude", self.magnitude)?;
        validate_relative_magnitude("fundamental reference magnitude", fundamental_mag)?;
        if fundamental_mag == 0.0 {
            return Ok(None);
        }
        if self.magnitude == 0.0 {
            return Ok(Some(0.0));
        }

        let ratio = self.magnitude / fundamental_mag;
        let mut percent = ratio * 100.0;
        if ratio == 0.0 {
            // Division can underflow even when the percentage remains
            // representable. Scaling the necessarily-small numerator first
            // recovers that range without risking overflow.
            percent = (self.magnitude * 100.0) / fundamental_mag;
        }
        if !percent.is_finite() || percent == 0.0 {
            return Err(FourierError::UnrepresentableRelativeSpectrum {
                quantity: "normalized harmonic magnitude",
            });
        }
        Ok(Some(percent))
    }

    /// Get magnitude in dB relative to the fundamental.
    ///
    /// `Ok(None)` means the reference is exactly zero. An exactly zero
    /// numerator with a nonzero reference is represented by negative
    /// infinity, its exact logarithmic value.
    pub fn db(&self, fundamental_mag: Value) -> Result<Option<Value>, FourierError> {
        validate_relative_magnitude("harmonic magnitude", self.magnitude)?;
        validate_relative_magnitude("fundamental reference magnitude", fundamental_mag)?;
        if fundamental_mag == 0.0 {
            return Ok(None);
        }
        if self.magnitude == 0.0 {
            return Ok(Some(Value::NEG_INFINITY));
        }

        // Subtract logarithms rather than taking the ratio first so valid
        // extreme finite magnitudes cannot overflow or underflow.
        let value = 20.0 * (self.magnitude.log10() - fundamental_mag.log10());
        if !value.is_finite() {
            return Err(FourierError::UnrepresentableRelativeSpectrum {
                quantity: "relative harmonic magnitude in dB",
            });
        }
        Ok(Some(value))
    }
}

fn validate_relative_magnitude(role: &'static str, value: Value) -> Result<(), FourierError> {
    if value.is_finite() && value >= 0.0 {
        Ok(())
    } else {
        Err(FourierError::InvalidMagnitude { role, value })
    }
}

/// Fourier analysis result
#[derive(Debug, Clone)]
pub struct FourierResult {
    /// Fundamental frequency analyzed
    pub fundamental_freq: Value,
    /// DC component
    pub dc_component: Value,
    /// All harmonic components (including DC and fundamental)
    pub harmonics: Vec<HarmonicComponent>,
    /// Total Harmonic Distortion (%), or `None` when the fundamental is
    /// exactly zero and the ratio is mathematically undefined.
    pub thd: Option<Value>,
}

impl FourierResult {
    /// Get fundamental component
    pub fn fundamental(&self) -> Option<&HarmonicComponent> {
        self.harmonics.get(1)
    }

    /// Get specific harmonic
    pub fn harmonic(&self, n: usize) -> Option<&HarmonicComponent> {
        self.harmonics.get(n)
    }

    /// Get THD in dB.
    ///
    /// `Ok(None)` preserves undefined THD from an exactly zero fundamental.
    /// A defined zero-percent THD is exactly negative infinity in dB.
    pub fn thd_db(&self) -> Result<Option<Value>, FourierError> {
        let Some(thd) = self.thd else {
            return Ok(None);
        };
        if !thd.is_finite() || thd < 0.0 {
            return Err(FourierError::InvalidThd { value: thd });
        }
        if thd == 0.0 {
            return Ok(Some(Value::NEG_INFINITY));
        }

        // THD is stored in percent; subtract log10(100) without first
        // dividing a potentially subnormal value.
        let value = 20.0 * (thd.log10() - 2.0);
        if !value.is_finite() {
            return Err(FourierError::UnrepresentableRelativeSpectrum {
                quantity: "total harmonic distortion in dB",
            });
        }
        Ok(Some(value))
    }
}

//=============================================================================
// Tests
//=============================================================================
