//! Fourier Analysis (.FOUR directive)
//!
//! Computes Fourier components and Total Harmonic Distortion (THD) from
//! transient simulation results.
//!
//! # SPICE Syntax
//! ```text
//! .FOUR <freq> [nharms] <output> [output...] [PERIODS=k] [FROM=t] [TO=t]
//! .FOUR 1kHz V(out)
//! .FOUR 60Hz 15 V(load) I(Rsense)
//! .FOUR 1k 9 V(out) PERIODS=4 FROM=2m TO=6m
//! ```
//!
//! # Output
//! Computes magnitude and phase of DC, fundamental, and harmonics up to nharms.
//! Also computes THD (Total Harmonic Distortion).
//!
//! # Algorithm
//! Uses trapezoidal Fourier integration over the last configured period(s) of
//! the analysis window. The window ends at the authored `TO` or, when the card
//! states none, at the last accepted transient time; `FROM` is the earliest
//! time that window may reach, so a card can refuse rather than integrate over
//! a start-up transient it was meant to skip.

use super::measure_signals::current_observation::{self, CurrentImpulseContribution};
use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::netlist::AnalysisCommand;
use crate::numerics::compensated_add;
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
    /// Time the analysis window ends at, or `None` for the last accepted
    /// transient time (`.FOUR`'s `TO=`).
    ///
    /// The window is the whole `num_periods` fundamental periods that END
    /// here, so moving this end moves the whole window rather than trimming
    /// it: a card can read the spectrum of a settled interval in the middle of
    /// a long record.
    pub window_stop: Option<Value>,
    /// Earliest time that window may reach, or `None` when the card states no
    /// settling guard (`.FOUR`'s `FROM=`).
    ///
    /// This is a refusal, not a clamp. Shortening the window to honour it
    /// would integrate over a fraction of a period and bias every coefficient,
    /// so a window that reaches back past this time is refused by name.
    pub earliest_start: Option<Value>,
}

impl FourierConfig {
    /// Create new Fourier config with fundamental frequency
    pub fn new(freq: Value) -> Self {
        Self {
            fundamental_freq: freq,
            num_harmonics: 9,
            num_periods: 1,
            window_stop: None,
            earliest_start: None,
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

    /// Qualify the request itself, before any waveform evidence is read.
    fn validate(&self) -> Result<(), FourierError> {
        if !self.fundamental_freq.is_finite() || self.fundamental_freq <= 0.0 {
            return Err(FourierError::InvalidFundamentalFrequency {
                frequency: self.fundamental_freq,
            });
        }
        if self.num_harmonics == 0 {
            return Err(FourierError::NoHarmonics);
        }
        if self.num_harmonics.checked_add(1).is_none() {
            return Err(FourierError::HarmonicCapacity {
                num_harmonics: self.num_harmonics,
            });
        }
        if self.num_periods == 0 {
            return Err(FourierError::NoPeriods);
        }
        Ok(())
    }
}

/// The one reading of a `.FOUR` card.
///
/// Every surface that runs an authored Fourier card converts the parsed card
/// here rather than destructuring it, so the window a deck asks for cannot be
/// honoured on one route and dropped on another. A card that authors no
/// keyword yields exactly `FourierConfig::new(f).with_harmonics(n)`.
impl TryFrom<&AnalysisCommand> for FourierConfig {
    type Error = FourierError;

    fn try_from(command: &AnalysisCommand) -> Result<Self, Self::Error> {
        let AnalysisCommand::Four {
            fundamental,
            num_harmonics,
            periods,
            window_from,
            window_to,
            ..
        } = command
        else {
            return Err(FourierError::NotAFourierCard);
        };
        let config = Self {
            fundamental_freq: *fundamental,
            num_harmonics: *num_harmonics,
            num_periods: *periods,
            window_stop: *window_to,
            earliest_start: *window_from,
        };
        config.validate()?;
        Ok(config)
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
    /// A requested current lacks coverage or has undefined impulse algebra.
    #[error("Fourier current observation: {detail}")]
    CurrentObservation { detail: String },
    /// Cooperative cancellation was requested while qualifying or
    /// integrating the retained waveform.
    #[error("Fourier analysis aborted")]
    Aborted,
    /// The requested fundamental cannot define a physical period.
    #[error("fundamental frequency must be positive and finite, got {frequency}")]
    InvalidFundamentalFrequency { frequency: Value },
    /// A result without a fundamental component cannot satisfy `.FOUR`.
    #[error("number of harmonics must be at least one")]
    NoHarmonics,
    /// The analysis window must contain at least one period.
    #[error("number of analysis periods must be at least one")]
    NoPeriods,
    /// A Fourier configuration was asked for from a card that is not `.FOUR`.
    #[error("a Fourier configuration can only be read from a .FOUR card")]
    NotAFourierCard,
    /// The authored window end does not lie within the transient record.
    ///
    /// Integrating up to a time the run never reached would silently end the
    /// window at the last accepted point instead, which is a different
    /// spectrum from the one the card asked for.
    #[error(
        "the Fourier window ends at {stop} s; it must lie after the first transient sample at {record_start} s and no later than the last at {record_end} s"
    )]
    WindowStopOutsideRecord {
        stop: Value,
        record_start: Value,
        record_end: Value,
    },
    /// The whole-period window reaches back past the authored earliest start.
    #[error(
        "the Fourier window begins at {start} s, before FROM = {earliest} s; lower PERIODS or FROM, or raise TO"
    )]
    WindowStartsBeforeEarliestStart { start: Value, earliest: Value },
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

    /// Perform Fourier analysis on a finite sampled waveform.
    /// For a retained result with charge impulses, use
    /// [`Self::analyze_transient_output_with_abort`] to include its singular current.
    ///
    /// # Arguments
    /// * `time` - Time points
    /// * `values` - Waveform values at each time point
    ///
    /// # Returns
    /// A qualified Fourier result, or a typed error when the configuration,
    /// waveform evidence, or computed coefficients are invalid.
    pub fn analyze(&self, time: &[Value], values: &[Value]) -> Result<FourierResult, FourierError> {
        self.analyze_with_abort(time, values, &NoAbort)
    }

    /// Perform Fourier analysis with cooperative cancellation.
    pub fn analyze_with_abort(
        &self,
        time: &[Value],
        values: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<FourierResult, FourierError> {
        self.analyze_observation(time, values, &[], abort)
    }

    /// Transform an output column already resolved from `result` and `spec`.
    ///
    /// `values` must be the finite column produced by the shared transient
    /// output resolver for this specification. Complete current observations
    /// additionally contribute their charge impulses analytically. The window
    /// excludes impulses at its start and includes its end (`(start, end]`),
    /// so adjacent and resumed intervals count each accepted impulse once. Affine current
    /// expressions accept time-independent real coefficients; undefined
    /// impulse products and nonlinear projections return explicit errors.
    ///
    /// Results without an impulse section retain the historical sampled-only
    /// interpretation; they do not certify complete physical-current history.
    /// With an impulse section, missing/incomplete current coverage is an error.
    pub fn analyze_transient_output_with_abort(
        &self,
        netlist: Option<&crate::Netlist>,
        result: &super::transient::TransientResult,
        spec: &str,
        values: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<FourierResult, FourierError> {
        if abort.is_aborted() {
            return Err(FourierError::Aborted);
        }
        self.validate_configuration()?;
        // The impulse interval is the analysis window itself, `(start, stop]`,
        // so a charge step the card's window excludes is not counted and one
        // it includes is counted exactly once.
        let (start, stop) = self.window_bounds(&result.time)?;
        let impulses = current_observation::resolve(netlist, result, spec, (start, stop), abort)?;
        self.analyze_observation(&result.time, values, &impulses, abort)
    }

    /// Analyze sampled current together with its complete signed charge history.
    /// The caller identifies the trace; impulses are integrated on `(start, stop]`
    /// by the same physical transform used for authored `.FOUR` cards.
    pub fn analyze_current_with_abort(
        &self,
        time: &[Value],
        values: &[Value],
        trace: &crate::CurrentImpulseTrace,
        abort: &dyn AbortSignal,
    ) -> Result<FourierResult, FourierError> {
        if abort.is_aborted() {
            return Err(FourierError::Aborted);
        }
        self.validate_configuration()?;
        validate_waveform(time, values, abort)?;
        if !trace.complete {
            return Err(FourierError::CurrentObservation {
                detail: format!("current '{}' has incomplete impulse history", trace.owner),
            });
        }
        trace
            .validate(time[0], time[time.len() - 1])
            .map_err(|detail| FourierError::CurrentObservation { detail })?;
        self.analyze_observation(
            time,
            values,
            &[CurrentImpulseContribution { trace, weight: 1.0 }],
            abort,
        )
    }

    fn analyze_observation(
        &self,
        time: &[Value],
        values: &[Value],
        impulses: &[CurrentImpulseContribution<'_>],
        abort: &dyn AbortSignal,
    ) -> Result<FourierResult, FourierError> {
        if abort.is_aborted() {
            return Err(FourierError::Aborted);
        }
        self.validate_configuration()?;
        validate_waveform(time, values, abort)?;

        let (t_start, t_stop) = self.window_bounds(time)?;

        // Retain an exact-period window. When either edge lies between
        // samples, interpolate the boundary instead of silently lengthening
        // or shortening the integration interval and biasing every
        // coefficient.
        let (window_time, window_values) = exact_window(time, values, t_start, t_stop, abort)?;
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
        let maximum_interval = 0.125 / highest_frequency;
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

        let quadrature = FourierQuadrature::new(
            &window_time,
            &window_values,
            window_time[window_time.len() - 1] - window_time[0],
            abort,
        )?;
        for n in 0..=self.config.num_harmonics {
            if abort.is_aborted() {
                return Err(FourierError::Aborted);
            }
            let freq = n as f64 * self.config.fundamental_freq;
            if !freq.is_finite() {
                return Err(FourierError::NonFiniteHarmonicFrequency {
                    harmonic: n,
                    frequency: freq,
                });
            }
            let (mag, phase) = quadrature.component_with_impulses(freq, n, impulses, abort)?;

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

        let mut harmonic_norm: Value = 0.0;
        for (index, harmonic) in harmonics.iter().enumerate().skip(2) {
            if index.is_multiple_of(256) && abort.is_aborted() {
                return Err(FourierError::Aborted);
            }
            harmonic_norm = harmonic_norm.hypot(harmonic.magnitude);
        }

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

    /// The `[t0, t1]` interval this configuration asks to integrate over.
    ///
    /// `t1` is the authored window end or, when the card states none, the last
    /// accepted time; `t0` is `t1` less the configured whole number of
    /// fundamental periods. Both ends are qualified against the record and
    /// against an authored earliest start before a single sample is read, so a
    /// window the deck cannot have meant is refused by name rather than
    /// quietly replaced by a shorter one.
    fn window_bounds(&self, time: &[Value]) -> Result<(Value, Value), FourierError> {
        // Find analysis window (last periods of waveform)
        let window_duration = self.config.window_duration();
        if !window_duration.is_finite() || window_duration <= 0.0 {
            return Err(FourierError::InvalidWindowDuration {
                duration: window_duration,
            });
        }
        let first = time.first().copied().ok_or(FourierError::EmptyWaveform)?;
        let record_end = time.last().copied().ok_or(FourierError::EmptyWaveform)?;
        let t_end = match self.config.window_stop {
            None => record_end,
            Some(stop) => {
                // An authored end one rounding step past the record's own last
                // time is that time: a deck writing `TO=5m` beside `.TRAN … 5m`
                // asks for the record it produced, not for a refusal.
                let tolerance = 64.0
                    * Value::EPSILON
                    * stop.abs().max(record_end.abs()).max(Value::MIN_POSITIVE);
                if !stop.is_finite() || stop <= first || stop > record_end + tolerance {
                    return Err(FourierError::WindowStopOutsideRecord {
                        stop,
                        record_start: first,
                        record_end,
                    });
                }
                stop.min(record_end)
            }
        };
        let available_duration = t_end - first;
        if !available_duration.is_finite() || available_duration <= 0.0 {
            return Err(FourierError::InvalidTimeSpan {
                start: first,
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
            first
        } else {
            t_end - window_duration
        };
        if let Some(earliest) = self.config.earliest_start {
            let tolerance =
                64.0 * Value::EPSILON * t_start.abs().max(earliest.abs()).max(Value::MIN_POSITIVE);
            if !earliest.is_finite() || t_start + tolerance < earliest {
                return Err(FourierError::WindowStartsBeforeEarliestStart {
                    start: t_start,
                    earliest,
                });
            }
        }
        Ok((t_start, t_end))
    }

    fn validate_configuration(&self) -> Result<(), FourierError> {
        self.config.validate()
    }
}

/// One qualified waveform and normalization window shared by transient FOUR
/// and retained PSS spectra. Scaling precedes integration, so finite results
/// do not depend on the physical units of time or voltage.
pub(crate) struct FourierQuadrature<'a> {
    time: &'a [Value],
    values: &'a [Value],
    duration: Value,
    scale: Value,
}

impl<'a> FourierQuadrature<'a> {
    pub(crate) fn new(
        time: &'a [Value],
        values: &'a [Value],
        duration: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Self, FourierError> {
        let scale = validate_waveform(time, values, abort)?;
        if !duration.is_finite() || duration <= 0.0 {
            return Err(FourierError::InvalidWindowDuration { duration });
        }
        let span = time[time.len() - 1] - time[0];
        if !span.is_finite() || span <= 0.0 {
            return Err(FourierError::InvalidTimeSpan {
                start: time[0],
                end: time[time.len() - 1],
            });
        }
        Ok(Self {
            time,
            values,
            duration,
            // Normalized trapezoidal weights already bound the sum of
            // physical contributions over a complete period. Scale small
            // signals up to preserve subnormal coefficients, but do not
            // divide large signals down: that could erase a tiny DC term
            // between exactly canceling large samples.
            scale: scale.min(1.0),
        })
    }

    /// Signed DC or physical peak magnitude and cosine-referenced phase.
    pub(crate) fn component(
        &self,
        frequency: Value,
        harmonic: usize,
        abort: &dyn AbortSignal,
    ) -> Result<(Value, Value), FourierError> {
        self.component_with_impulses(frequency, harmonic, &[], abort)
    }

    fn component_with_impulses(
        &self,
        frequency: Value,
        harmonic: usize,
        impulses: &[CurrentImpulseContribution<'_>],
        abort: &dyn AbortSignal,
    ) -> Result<(Value, Value), FourierError> {
        if abort.is_aborted() {
            return Err(FourierError::Aborted);
        }
        let cycles = frequency * self.duration;
        if !frequency.is_finite() || !cycles.is_finite() {
            return Err(FourierError::NonFiniteHarmonicFrequency {
                harmonic,
                frequency,
            });
        }
        // Normalize finite and singular contributions together. Choosing a
        // scale from the finite waveform alone can overflow a valid charge
        // contribution when that waveform is tiny or identically zero.
        let mut scale = self.scale;
        for term in impulses {
            for (index, point) in term.trace.points.iter().enumerate() {
                if index.is_multiple_of(256) && abort.is_aborted() {
                    return Err(FourierError::Aborted);
                }
                if point.time <= self.time[0] || point.time > self.time[self.time.len() - 1] {
                    continue;
                }
                let rate = crate::numerics::scaled_exp_product(
                    &[point.charge_coulombs, term.weight],
                    &[self.duration],
                    0.0,
                );
                ensure_finite_coefficient(rate, harmonic, "impulse charge per period")?;
                if rate == 0.0 && point.charge_coulombs != 0.0 && term.weight != 0.0 {
                    return Err(FourierError::CurrentObservation {
                        detail: "impulse charge per period is below the representable range".into(),
                    });
                }
                scale = scale.max(rate.abs().min(1.0));
            }
        }
        if scale == 0.0 {
            return Ok((0.0, 0.0));
        }

        let mut cosine_integral = 0.0;
        let mut cosine_correction = 0.0;
        let mut sine_integral = 0.0;
        let mut sine_correction = 0.0;
        let phase = |time: Value| {
            // Form dimensionless cycles before radians. `TAU * frequency`
            // can overflow even when the frequency and phase are finite.
            let turns = ((time - self.time[0]) / self.duration) * cycles;
            std::f64::consts::TAU * turns.fract()
        };
        for index in 0..self.time.len() {
            if index.is_multiple_of(256) && abort.is_aborted() {
                return Err(FourierError::Aborted);
            }
            let before = index.saturating_sub(1);
            let after = (index + 1).min(self.time.len() - 1);
            let span = self.time[after] - self.time[before];
            let value = self.values[index] / scale;
            let sample = self.weighted_sample(0.5 * value, span);
            let (cosine, sine) = if harmonic == 0 {
                (1.0, 0.0)
            } else {
                let (sine, cosine) = phase(self.time[index]).sin_cos();
                (cosine, sine)
            };
            // Accumulate each knot's contribution separately. Averaging a
            // tiny sample with its large neighbor would discard it before
            // compensation ever sees the term.
            compensated_add(
                &mut cosine_integral,
                &mut cosine_correction,
                sample * cosine,
            );
            if harmonic != 0 {
                compensated_add(&mut sine_integral, &mut sine_correction, sample * sine);
            }
        }
        for term in impulses {
            for (index, point) in term.trace.points.iter().enumerate() {
                if index.is_multiple_of(256) && abort.is_aborted() {
                    return Err(FourierError::Aborted);
                }
                if point.time <= self.time[0] || point.time > self.time[self.time.len() - 1] {
                    continue;
                }
                let sample = crate::numerics::scaled_exp_product(
                    &[point.charge_coulombs, term.weight],
                    &[self.duration, scale],
                    0.0,
                );
                let (cosine, sine) = if harmonic == 0 {
                    (1.0, 0.0)
                } else {
                    let (sine, cosine) = phase(point.time).sin_cos();
                    (cosine, sine)
                };
                compensated_add(
                    &mut cosine_integral,
                    &mut cosine_correction,
                    sample * cosine,
                );
                if harmonic != 0 {
                    compensated_add(&mut sine_integral, &mut sine_correction, sample * sine);
                }
            }
        }
        if harmonic == 0 {
            let dc = (cosine_integral + cosine_correction) * scale;
            ensure_finite_coefficient(dc, harmonic, "DC component")?;
            return Ok((dc, 0.0));
        }
        let a_n = 2.0 * (cosine_integral + cosine_correction);
        let b_n = 2.0 * (sine_integral + sine_correction);
        let magnitude = a_n.hypot(b_n) * scale;
        let phase = (-b_n).atan2(a_n) * 180.0 / PI;
        ensure_finite_coefficient(magnitude, harmonic, "magnitude")?;
        ensure_finite_coefficient(phase, harmonic, "phase")?;

        Ok((magnitude, phase))
    }

    fn weighted_sample(&self, value: Value, span: Value) -> Value {
        let weight = span / self.duration;
        if weight.is_normal() {
            return value * weight;
        }
        // A subnormal (or zero) duration ratio may lose significant bits
        // even when the final physical contribution is normal. Reorder the
        // same product/quotient using a normal intermediate when available.
        let area = value * span;
        if area.is_normal() {
            return area / self.duration;
        }
        let normalized = value / self.duration;
        if normalized.is_normal() {
            return normalized * span;
        }
        value * weight
    }
}

fn validate_waveform(
    time: &[Value],
    values: &[Value],
    abort: &dyn AbortSignal,
) -> Result<Value, FourierError> {
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
    let mut scale: Value = 0.0;
    for (index, (&sample_time, &sample_value)) in time.iter().zip(values).enumerate() {
        if index.is_multiple_of(256) && abort.is_aborted() {
            return Err(FourierError::Aborted);
        }
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
        scale = scale.max(sample_value.abs());
        if index > 0 && sample_time <= time[index - 1] {
            return Err(FourierError::NonIncreasingTime {
                index,
                previous: time[index - 1],
                current: sample_time,
            });
        }
    }
    Ok(scale)
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

/// Select the analysis window `[t_start, t_stop]` and interpolate whichever of
/// its boundaries falls between authored transient samples.
///
/// Snapping a boundary to the nearest sample instead would change the
/// integration interval by up to one time step, which scales every coefficient
/// by the wrong window length; the interpolated edge keeps the window exactly
/// the whole number of periods the card asked for.
fn exact_window(
    time: &[Value],
    values: &[Value],
    t_start: Value,
    t_stop: Value,
    abort: &dyn AbortSignal,
) -> Result<(Vec<Value>, Vec<Value>), FourierError> {
    if abort.is_aborted() {
        return Err(FourierError::Aborted);
    }
    let first_retained = time.partition_point(|&sample| sample < t_start);
    let past_last_retained = time.partition_point(|&sample| sample <= t_stop);
    let interpolate_start = first_retained < time.len() && time[first_retained] != t_start;
    let interpolate_stop = past_last_retained < time.len()
        && past_last_retained > 0
        && time[past_last_retained - 1] != t_stop;
    let retained_samples = past_last_retained.saturating_sub(first_retained);
    let window_samples = retained_samples
        .checked_add(usize::from(interpolate_start) + usize::from(interpolate_stop))
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

    if interpolate_start {
        let lower =
            first_retained
                .checked_sub(1)
                .ok_or(FourierError::InsufficientWindowSamples {
                    samples: retained_samples,
                })?;
        window_values.push(interpolated_boundary(
            time,
            values,
            lower,
            first_retained,
            t_start,
        )?);
        window_time.push(t_start);
    }
    for (index, (&sample_time, &sample_value)) in time[first_retained..past_last_retained]
        .iter()
        .zip(&values[first_retained..past_last_retained])
        .enumerate()
    {
        if index.is_multiple_of(256) && abort.is_aborted() {
            return Err(FourierError::Aborted);
        }
        window_time.push(sample_time);
        window_values.push(sample_value);
    }
    if interpolate_stop {
        window_values.push(interpolated_boundary(
            time,
            values,
            past_last_retained - 1,
            past_last_retained,
            t_stop,
        )?);
        window_time.push(t_stop);
    }

    Ok((window_time, window_values))
}

/// Linear value at `boundary`, which lies strictly between `time[lower]` and
/// `time[upper]`.
fn interpolated_boundary(
    time: &[Value],
    values: &[Value],
    lower: usize,
    upper: usize,
    boundary: Value,
) -> Result<Value, FourierError> {
    let fraction = (boundary - time[lower]) / (time[upper] - time[lower]);
    let interpolated = (1.0 - fraction).mul_add(values[lower], fraction * values[upper]);
    ensure_finite_coefficient(interpolated, 0, "interpolated window boundary")?;
    Ok(interpolated)
}

//=============================================================================
// Results
//=============================================================================

/// Single harmonic component
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, PartialEq)]
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
