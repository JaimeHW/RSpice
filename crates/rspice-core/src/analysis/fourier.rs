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
//! Reads the last configured period(s) of the analysis window by the rule of
//! the dialect's own simulator ([`FourierMethod`]): the exact integral of the
//! samples' piecewise-linear interpolant (QSPICE, LTspice), or a DFT of a
//! 200-point-per-period grid interpolated linearly (ngspice) or by a natural
//! cubic spline (Xyce). The window ends at the authored `TO` or, when the card
//! states none, at the last accepted transient time; `FROM` is the earliest
//! time that window may reach, so a card can refuse rather than integrate over
//! a start-up transient it was meant to skip.

use super::measure_signals::impulse_observation::{self, ImpulseContribution};
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
    /// How the window's samples become coefficients.
    pub method: FourierMethod,
}

/// How a `.FOUR` card turns the samples of its window into harmonics.
///
/// The simulators disagree, and on a coarsely sampled waveform their spectra
/// differ by whole percent of THD, so each dialect reads its own simulator's
/// rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FourierMethod {
    /// The exact Fourier integral of the samples' piecewise-linear
    /// interpolant. QSPICE resamples that interpolant finely and takes an FFT
    /// (QPOST); LTspice prints the same spectrum. This is the limit both
    /// converge to.
    #[default]
    Interpolant,
    /// ngspice's: the samples interpolated linearly onto 200 equally spaced
    /// points per period from the window start, then a DFT of those points
    /// (frontend/fourier.c: fourgridsize 200, polydegree 1, CKTfour).
    LinearGrid,
    /// Xyce's: a natural cubic spline through the samples from the last one at
    /// or before the window start, evaluated at 200 equally spaced points per
    /// period, then a DFT of those points (N_IO_FourierMgr.C:722-905).
    SplineGrid,
}

impl FourierMethod {
    /// The rule each dialect's own simulator applies.
    pub fn for_dialect(dialect: crate::config::SpiceDialect) -> Self {
        match dialect {
            crate::config::SpiceDialect::Ngspice => Self::LinearGrid,
            crate::config::SpiceDialect::Xyce => Self::SplineGrid,
            crate::config::SpiceDialect::BestAvailable => Self::Interpolant,
        }
    }
}

/// Grid points per fundamental period of ngspice's and Xyce's Fourier
/// resampling (ngspice DEF_FOURGRIDSIZE; Xyce gridSize_).
const FOURIER_GRID_POINTS_PER_PERIOD: usize = 200;

impl FourierConfig {
    /// Create new Fourier config with fundamental frequency
    pub fn new(freq: Value) -> Self {
        Self {
            fundamental_freq: freq,
            num_harmonics: 9,
            num_periods: 1,
            window_stop: None,
            earliest_start: None,
            method: FourierMethod::Interpolant,
        }
    }

    /// Set how the window's samples become coefficients.
    pub fn with_method(mut self, method: FourierMethod) -> Self {
        self.method = method;
        self
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
            method: FourierMethod::Interpolant,
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
        let impulses = impulse_observation::resolve(netlist, result, spec, (start, stop), abort)?;
        self.analyze_observation(&result.time, values, &impulses, abort)
    }

    /// Analyze a finite signal and a real, time-independent linear combination
    /// of typed singular histories. For V(a,b), pass voltage traces with weights
    /// +1 and -1. Every supplied history must cover the full sampled extent.
    /// Voltage actions retain volt-seconds and higher derivative coefficients;
    /// they are never converted into current charge or sampled voltage spikes.
    pub fn analyze_impulses_with_abort(
        &self,
        time: &[Value],
        values: &[Value],
        traces: &[(crate::ImpulseTraceRef<'_>, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<FourierResult, FourierError> {
        if abort.is_aborted() {
            return Err(FourierError::Aborted);
        }
        self.validate_configuration()?;
        validate_waveform(time, values, abort)?;
        let mut terms = Vec::new();
        terms
            .try_reserve_exact(traces.len())
            .map_err(|_| FourierError::CurrentObservation {
                detail: "cannot allocate impulse contributions".into(),
            })?;
        for &(trace, weight) in traces {
            if abort.is_aborted() {
                return Err(FourierError::Aborted);
            }
            if !weight.is_finite() {
                return Err(FourierError::CurrentObservation {
                    detail: "impulse weight must be finite".into(),
                });
            }
            if !trace.complete() {
                return Err(FourierError::CurrentObservation {
                    detail: format!("signal '{trace}' has incomplete impulse history"),
                });
            }
            trace
                .validate(time[0], time[time.len() - 1])
                .map_err(|detail| FourierError::CurrentObservation { detail })?;
            terms.push(ImpulseContribution { trace, weight });
        }
        terms.sort_unstable_by_key(|term| term.trace.identity());
        let mut merged: Vec<ImpulseContribution<'_>> = Vec::new();
        merged
            .try_reserve_exact(terms.len())
            .map_err(|_| FourierError::CurrentObservation {
                detail: "cannot allocate merged impulse contributions".into(),
            })?;
        for term in terms {
            if abort.is_aborted() {
                return Err(FourierError::Aborted);
            }
            if let Some(last) = merged.last_mut()
                && last.trace.identity() == term.trace.identity()
            {
                last.weight += term.weight;
                if !last.weight.is_finite() {
                    return Err(FourierError::CurrentObservation {
                        detail: "combined impulse weight is not representable".into(),
                    });
                }
            } else {
                merged.push(term);
            }
        }
        merged.retain(|term| term.weight != 0.0);
        self.analyze_observation(time, values, &merged, abort)
    }

    fn analyze_observation(
        &self,
        time: &[Value],
        values: &[Value],
        impulses: &[ImpulseContribution<'_>],
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
        // A step longer than an eighth of the highest harmonic's period leaves
        // that harmonic to the interpolation rule rather than to the samples.
        // Every simulator computes it anyway, and so does this; Spectre warns.
        let maximum_interval = 0.125 / highest_frequency;
        let largest_interval = window_time
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .fold(0.0, Value::max);
        if largest_interval > maximum_interval * (1.0 + 16.0 * Value::EPSILON) {
            log::warn!(
                "[four.coarse] .FOUR {} Hz: a {largest_interval:e} s sample interval is longer than \
                 an eighth of harmonic {}'s period, so its harmonics read the interpolation \
                 between samples",
                self.config.fundamental_freq,
                self.config.num_harmonics
            );
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

        let duration = window_time[window_time.len() - 1] - window_time[0];
        let grid;
        let quadrature = match self.config.method {
            FourierMethod::Interpolant => {
                FourierQuadrature::new(&window_time, &window_values, duration, abort)?
                    .with_rule(QuadratureRule::Interpolant)
            }
            method => {
                let points = FOURIER_GRID_POINTS_PER_PERIOD
                    .checked_mul(self.config.num_periods)
                    .ok_or(FourierError::WindowCapacity {
                        samples: usize::MAX,
                    })?;
                grid = uniform_grid(
                    method,
                    (time, values),
                    (&window_time, &window_values),
                    points,
                    abort,
                )?;
                FourierQuadrature::new(&grid.0, &grid.1, duration, abort)?
                    .with_rule(QuadratureRule::Uniform)
                    .ending_at(window_time[window_time.len() - 1])
            }
        };
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
    rule: QuadratureRule,
    /// The window's end; an impulse at it is counted, one after it is not.
    end: Value,
}

/// How a [`FourierQuadrature`] weighs its samples.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum QuadratureRule {
    /// The trapezoidal rule on the samples.
    Trapezoid,
    /// The exact integral of the samples' piecewise-linear interpolant.
    Interpolant,
    /// Equal weights on a grid of equally spaced samples from the window
    /// start, its end excluded: a DFT.
    Uniform,
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
            rule: QuadratureRule::Trapezoid,
            end: time[time.len() - 1],
        })
    }

    fn with_rule(mut self, rule: QuadratureRule) -> Self {
        self.rule = rule;
        self
    }

    /// A grid's last sample lies a step short of the window's end.
    fn ending_at(mut self, end: Value) -> Self {
        self.end = end;
        self
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
        impulses: &[ImpulseContribution<'_>],
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
            for (index, point) in term.trace.points().enumerate() {
                if index.is_multiple_of(256) && abort.is_aborted() {
                    return Err(FourierError::Aborted);
                }
                if point.time <= self.time[0] || point.time > self.end {
                    continue;
                }
                let rate = crate::numerics::scaled_exp_product(
                    &[point.coefficient, term.weight],
                    &[self.duration],
                    0.0,
                );
                ensure_finite_coefficient(rate, harmonic, "impulse charge per period")?;
                if rate == 0.0 && point.coefficient != 0.0 && term.weight != 0.0 {
                    return Err(FourierError::CurrentObservation {
                        detail: "impulse charge per period is below the representable range".into(),
                    });
                }
                scale = scale.max(rate.abs().min(1.0));
            }
            for (index, point) in term.trace.derivatives().iter().enumerate() {
                if index.is_multiple_of(256) && abort.is_aborted() {
                    return Err(FourierError::Aborted);
                }
                if point.time <= self.time[0] || point.time > self.end {
                    continue;
                }
                let rate = point
                    .periodic_rate(frequency, term.weight, self.duration, 1.0)
                    .map_err(|detail| FourierError::CurrentObservation { detail })?;
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
        // The interpolant's DC term is the trapezoidal one; its harmonics are
        // integrated step by step.
        let samples = if self.rule == QuadratureRule::Interpolant && harmonic != 0 {
            self.add_interpolant(
                cycles,
                scale,
                (&mut cosine_integral, &mut cosine_correction),
                (&mut sine_integral, &mut sine_correction),
                abort,
            )?;
            0
        } else {
            self.time.len()
        };
        for index in 0..samples {
            if index.is_multiple_of(256) && abort.is_aborted() {
                return Err(FourierError::Aborted);
            }
            let value = self.values[index] / scale;
            let sample = if self.rule == QuadratureRule::Uniform {
                self.weighted_sample(value, self.duration / self.time.len() as Value)
            } else {
                let before = index.saturating_sub(1);
                let after = (index + 1).min(self.time.len() - 1);
                let span = self.time[after] - self.time[before];
                self.weighted_sample(0.5 * value, span)
            };
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
            for (index, point) in term.trace.points().enumerate() {
                if index.is_multiple_of(256) && abort.is_aborted() {
                    return Err(FourierError::Aborted);
                }
                if point.time <= self.time[0] || point.time > self.end {
                    continue;
                }
                let sample = crate::numerics::scaled_exp_product(
                    &[point.coefficient, term.weight],
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
            if harmonic != 0 {
                for (index, point) in term.trace.derivatives().iter().enumerate() {
                    if index.is_multiple_of(256) && abort.is_aborted() {
                        return Err(FourierError::Aborted);
                    }
                    if point.time <= self.time[0] || point.time > self.end {
                        continue;
                    }
                    let sample = point
                        .periodic_rate(frequency, term.weight, self.duration, scale)
                        .map_err(|detail| FourierError::CurrentObservation { detail })?;
                    let (sine, cosine) = phase(point.time).sin_cos();
                    // (-1)^n times the n-th derivative of the real Fourier
                    // basis. Integer quarter turns preserve exact axis zeros.
                    let (cosine, sine) = match point.order % 4 {
                        0 => (cosine, sine),
                        1 => (sine, -cosine),
                        2 => (-cosine, -sine),
                        _ => (-sine, cosine),
                    };
                    compensated_add(
                        &mut cosine_integral,
                        &mut cosine_correction,
                        sample * cosine,
                    );
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

    /// Add the exact integral of the piecewise-linear interpolant times the
    /// harmonic's cosine and sine over each step: on a step of `h` seconds
    /// and `phi` radians from `v0` to `v1`, `h e^{-j theta0} (v0 (I0 - I1) +
    /// v1 I1)` with `Ik = int_0^1 s^k e^{-j phi s} ds`.
    fn add_interpolant(
        &self,
        cycles: Value,
        scale: Value,
        cosine: (&mut Value, &mut Value),
        sine: (&mut Value, &mut Value),
        abort: &dyn AbortSignal,
    ) -> Result<(), FourierError> {
        let (cosine_integral, cosine_correction) = cosine;
        let (sine_integral, sine_correction) = sine;
        for index in 0..self.time.len() - 1 {
            if index.is_multiple_of(256) && abort.is_aborted() {
                return Err(FourierError::Aborted);
            }
            let start = self.time[index];
            let span = self.time[index + 1] - start;
            let turns = ((start - self.time[0]) / self.duration) * cycles;
            let theta = std::f64::consts::TAU * turns.fract();
            let phi = std::f64::consts::TAU * (span / self.duration) * cycles;
            let ((i0_re, i0_im), (i1_re, i1_im)) = linear_basis_moments(phi);
            let v0 = self.values[index] / scale;
            let v1 = self.values[index + 1] / scale;
            let weight_re = v0 * (i0_re - i1_re) + v1 * i1_re;
            let weight_im = v0 * (i0_im - i1_im) + v1 * i1_im;
            // The step's integral of v e^{-j omega t} is its weight turned by
            // e^{-j theta0}; the cosine integral is the real part, the sine
            // integral minus the imaginary part.
            let (sin_theta, cos_theta) = theta.sin_cos();
            let real = weight_re * cos_theta + weight_im * sin_theta;
            let imaginary = weight_im * cos_theta - weight_re * sin_theta;
            compensated_add(
                cosine_integral,
                cosine_correction,
                self.weighted_sample(real, span),
            );
            compensated_add(
                sine_integral,
                sine_correction,
                self.weighted_sample(-imaginary, span),
            );
        }
        Ok(())
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

/// `int_0^1 e^{-j phi s} ds` and `int_0^1 s e^{-j phi s} ds` as `(re, im)`
/// pairs: their power series below a radian, where the closed forms cancel,
/// and the closed forms above.
fn linear_basis_moments(phi: Value) -> ((Value, Value), (Value, Value)) {
    if phi.abs() < 1.0 {
        // sum_n (-j phi)^n / (n! (n + k + 1))
        let mut moments = [(0.0, 0.0); 2];
        let mut term = (1.0, 0.0);
        for n in 0..24 {
            for (k, moment) in moments.iter_mut().enumerate() {
                let denominator = (n + k + 1) as Value;
                moment.0 += term.0 / denominator;
                moment.1 += term.1 / denominator;
            }
            let factor = phi / (n + 1) as Value;
            term = (term.1 * factor, -term.0 * factor);
        }
        return (moments[0], moments[1]);
    }
    let (sin_phi, cos_phi) = phi.sin_cos();
    let half = (0.5 * phi).sin();
    let phi2 = phi * phi;
    (
        (sin_phi / phi, -2.0 * half * half / phi),
        (
            (cos_phi + phi * sin_phi - 1.0) / phi2,
            (phi * cos_phi - sin_phi) / phi2,
        ),
    )
}

/// The window resampled onto `points` equally spaced times from its start,
/// its end excluded, the way ngspice (linearly, from the window's own
/// samples) or Xyce (by a natural cubic spline from the last raw sample at or
/// before the start) does.
fn uniform_grid(
    method: FourierMethod,
    (time, values): (&[Value], &[Value]),
    (window_time, window_values): (&[Value], &[Value]),
    points: usize,
    abort: &dyn AbortSignal,
) -> Result<(Vec<Value>, Vec<Value>), FourierError> {
    let start = window_time[0];
    let stop = window_time[window_time.len() - 1];
    let step = (stop - start) / points as Value;
    let mut grid_time = Vec::new();
    grid_time
        .try_reserve_exact(points)
        .map_err(|_| FourierError::WindowCapacity { samples: points })?;
    grid_time.extend((0..points).map(|index| start + index as Value * step));
    let spline = match method {
        FourierMethod::SplineGrid => {
            let first = time
                .partition_point(|&sample| sample <= start)
                .saturating_sub(1);
            let last = time
                .partition_point(|&sample| sample < stop)
                .min(time.len() - 1);
            natural_spline_grid(
                &time[first..=last],
                &values[first..=last],
                start,
                step,
                points,
            )
        }
        FourierMethod::Interpolant | FourierMethod::LinearGrid => None,
    };
    if let Some(grid_values) = spline {
        return Ok((grid_time, grid_values));
    }
    // ngspice's linear interpolation, and a spline's through two samples.
    let mut grid_values = Vec::new();
    grid_values
        .try_reserve_exact(points)
        .map_err(|_| FourierError::WindowCapacity { samples: points })?;
    let mut segment = 0;
    for (index, &at) in grid_time.iter().enumerate() {
        if index.is_multiple_of(256) && abort.is_aborted() {
            return Err(FourierError::Aborted);
        }
        while segment + 2 < window_time.len() && window_time[segment + 1] <= at {
            segment += 1;
        }
        let (t0, t1) = (window_time[segment], window_time[segment + 1]);
        let (v0, v1) = (window_values[segment], window_values[segment + 1]);
        grid_values.push(v0 + (v1 - v0) * ((at - t0) / (t1 - t0)));
    }
    Ok((grid_time, grid_values))
}

/// Xyce's Fourier resampling (N_IO_FourierMgr.C interpolateData_, and the
/// `FOUR` measure's N_IO_MeasureFourier.C:244-308): a natural cubic spline
/// through `time`/`values`, whose first sample is the last one at or before
/// `start`, evaluated at `grid_size` points `step` apart from `start`. A grid
/// point at or past the last sample reads that sample's value. `None` for
/// fewer than three samples, which have no spline.
pub(crate) fn natural_spline_grid(
    time: &[Value],
    values: &[Value],
    start: Value,
    step: Value,
    grid_size: usize,
) -> Option<Vec<Value>> {
    let n = time.len();
    if n < 3 || values.len() != n {
        return None;
    }
    // The natural spline's second derivatives z.
    let h = time
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<_>>();
    let b = (0..n - 1)
        .map(|i| (6.0 / h[i]) * (values[i + 1] - values[i]))
        .collect::<Vec<_>>();
    let mut u = vec![0.0; n - 1];
    let mut v = vec![0.0; n - 1];
    u[1] = 2.0 * (h[0] + h[1]);
    v[1] = b[1] - b[0];
    for i in 2..n - 1 {
        u[i] = 2.0 * (h[i] + h[i - 1]) - h[i - 1] * h[i - 1] / u[i - 1];
        v[i] = b[i] - b[i - 1] - h[i - 1] * v[i - 1] / u[i - 1];
    }
    let mut z = vec![0.0; n];
    for i in (1..n - 1).rev() {
        z[i] = (v[i] - h[i] * z[i + 1]) / u[i];
    }

    let mut grid = Vec::with_capacity(grid_size);
    let mut grid_time = start;
    for i in 0..grid_size {
        if i > 0 {
            grid_time += step;
        }
        let mut j = n - 1;
        while grid_time - time[j] < 0.0 {
            j = j.checked_sub(1)?;
        }
        grid.push(if j == n - 1 {
            values[j]
        } else {
            let a = (z[j + 1] - z[j]) / (6.0 * h[j]);
            let b = z[j] / 2.0;
            let c =
                -(h[j] / 6.0) * z[j + 1] - (h[j] / 3.0) * z[j] + (values[j + 1] - values[j]) / h[j];
            let offset = grid_time - time[j];
            values[j] + offset * (c + offset * (b + offset * a))
        });
    }
    Some(grid)
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
    /// Phase (degrees) of the cosine-referenced phasor; see
    /// [`HarmonicComponent::sine_referenced_phase`] for the `.FOUR` table's.
    pub phase: Value,
}

impl HarmonicComponent {
    /// Phase (degrees) against a sine, the way a SPICE `.FOUR` table reports
    /// it (SPICE2; ngspice `frontend/fourier.c`, `atan2(cos_sum, sin_sum)`),
    /// so a `SIN` source at zero phase reads 0°. A cosine leads the sine of
    /// the same phase by a quarter turn; the DC term keeps its zero phase.
    #[must_use]
    pub fn sine_referenced_phase(&self) -> Value {
        if self.harmonic_number == 0 {
            return self.phase;
        }
        let phase = self.phase + 90.0;
        if phase > 180.0 { phase - 360.0 } else { phase }
    }

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
